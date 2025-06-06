use std::error::Error;
use tokio::net::{UnixListener, UnixStream};
use tokio::io::{AsyncWriteExt, AsyncBufReadExt, BufReader};
use tokio::sync::broadcast;
use crate::{Theme, config};

pub struct IpcServer {
    socket_path: String,
    sender: broadcast::Sender<Theme>,
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
        })
    }

    pub fn get_broadcaster(&self) -> broadcast::Sender<Theme> {
        self.sender.clone()
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

        tokio::spawn(async move {
            loop {
                match listener.accept().await {
                    Ok((stream, _)) => {
                        let sender = sender.clone();
                        tokio::spawn(handle_client(stream, sender));
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

async fn handle_client(mut stream: UnixStream, sender: broadcast::Sender<Theme>) {
    let mut receiver = sender.subscribe();
    
    // Send current theme immediately upon connection
    if let Ok(current_theme) = receiver.recv().await {
        let _ = stream.write_all(format!("{}\n", current_theme).as_bytes()).await;
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
}

impl IpcHandler {
    pub fn new(sender: broadcast::Sender<Theme>) -> Self {
        Self { sender }
    }
}

impl crate::handlers::ThemeHandler for IpcHandler {
    fn on_theme_change(&self, theme: Theme) {
        // Broadcast to all connected clients
        let _ = self.sender.send(theme);
    }
}