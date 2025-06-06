use std::error::Error;
use std::sync::{Arc, RwLock};
use tokio::net::{UnixListener, UnixStream};
use tokio::io::{AsyncWriteExt, AsyncBufReadExt, BufReader};
use tokio::sync::broadcast;
use crate::{Theme, config};

pub struct IpcServer {
    socket_path: String,
    sender: broadcast::Sender<Theme>,
    current_theme: Arc<RwLock<Theme>>,
}

impl IpcServer {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let socket_dir = dirs::runtime_dir()
            .or_else(|| dirs::home_dir().map(|h| h.join(".local/run")))
            .unwrap_or_else(|| std::env::temp_dir());
        
        // Create directory if it doesn't exist
        std::fs::create_dir_all(&socket_dir)?;
        
        let socket_path = socket_dir
            .join("theme-switcher.sock")
            .to_string_lossy()
            .to_string();

        // Remove existing socket if it exists
        let _ = std::fs::remove_file(&socket_path);

        let (sender, _) = broadcast::channel(16);

        Ok(Self {
            socket_path,
            sender,
            current_theme: Arc::new(RwLock::new(Theme::Dark)), // Default, will be updated
        })
    }

    pub fn get_broadcaster(&self) -> broadcast::Sender<Theme> {
        self.sender.clone()
    }
    
    pub fn set_current_theme(&self, theme: Theme) {
        if let Ok(mut current) = self.current_theme.write() {
            *current = theme;
        }
    }
    
    pub fn get_current_theme_state(&self) -> Arc<RwLock<Theme>> {
        self.current_theme.clone()
    }

    pub async fn start(&self) -> Result<(), Box<dyn Error>> {
        let listener = UnixListener::bind(&self.socket_path)?;
        config::log_info(&format!("IPC server listening on: {}", self.socket_path));

        // Set permissions to allow user access
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(&self.socket_path, std::fs::Permissions::from_mode(0o600))?;
        }

        let sender = self.sender.clone();
        let current_theme = self.current_theme.clone();

        tokio::spawn(async move {
            loop {
                match listener.accept().await {
                    Ok((stream, _)) => {
                        let sender = sender.clone();
                        let current_theme = current_theme.clone();
                        tokio::spawn(handle_client(stream, sender, current_theme));
                    }
                    Err(e) => {
                        eprintln!("Error accepting connection: {}", e);
                    }
                }
            }
        });

        Ok(())
    }

    pub fn cleanup(&self) {
        let _ = std::fs::remove_file(&self.socket_path);
    }
}

async fn handle_client(mut stream: UnixStream, sender: broadcast::Sender<Theme>, current_theme: Arc<RwLock<Theme>>) {
    let mut receiver = sender.subscribe();
    
    // Send current theme immediately upon connection
    let theme_to_send = {
        current_theme.read().ok().map(|t| *t)
    };
    
    if let Some(theme) = theme_to_send {
        let _ = stream.write_all(format!("{}\n", theme).as_bytes()).await;
    }

    // Create a reader for incoming commands
    let (reader, mut writer) = stream.into_split();
    let mut reader = BufReader::new(reader);
    let mut line = String::new();

    loop {
        tokio::select! {
            // Handle incoming theme changes
            theme_result = receiver.recv() => {
                match theme_result {
                    Ok(theme) => {
                        if let Err(_) = writer.write_all(format!("{}\n", theme).as_bytes()).await {
                            break; // Client disconnected
                        }
                    }
                    Err(_) => break, // Channel closed
                }
            }
            // Handle client commands (like "quit")
            read_result = reader.read_line(&mut line) => {
                match read_result {
                    Ok(0) => break, // Client disconnected
                    Ok(_) => {
                        if line.trim() == "quit" {
                            break;
                        }
                        line.clear();
                    }
                    Err(_) => break,
                }
            }
        }
    }
}

pub struct IpcHandler {
    sender: broadcast::Sender<Theme>,
    current_theme: Arc<RwLock<Theme>>,
}

impl IpcHandler {
    pub fn new(sender: broadcast::Sender<Theme>, current_theme: Arc<RwLock<Theme>>) -> Self {
        Self { sender, current_theme }
    }
}

impl crate::handlers::ThemeHandler for IpcHandler {
    fn on_theme_change(&self, theme: Theme) {
        // Update current theme
        if let Ok(mut current) = self.current_theme.write() {
            *current = theme;
        }
        // Broadcast to all connected clients
        let _ = self.sender.send(theme);
    }
}