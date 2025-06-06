use crate::app::Application;
use crate::error::{Result, ThemeSwitcherError};
use crate::handlers::ThemeHandler;
use crate::{Theme, ThemeMonitor};
use cocoa::appkit::{NSApp, NSApplication, NSApplicationActivationPolicyAccessory};
use cocoa::base::{BOOL, YES, id, nil};
use cocoa::foundation::{NSArray, NSString};
use lazy_static::lazy_static;
use objc::declare::ClassDecl;
use objc::runtime::{Class, Object, Sel};
use objc::{class, msg_send, sel, sel_impl};
use objc_id::Id;
use std::cell::RefCell;
use std::error::Error;
use std::os::raw::c_void;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

const APPEARANCE_CHANGED_OBSERVER_NAME: &str = "ThemeSwitcherKVOHelper";

// Global storage for callbacks to ensure they live long enough
lazy_static! {
    static ref CALLBACK_STORAGE: Mutex<Vec<Arc<dyn Fn(Theme) + Send + Sync>>> =
        Mutex::new(Vec::new());
}

lazy_static! {
    static ref OBJC_SUBCLASS: &'static Class = {
        let superclass = class!(NSObject);
        let mut decl = ClassDecl::new(APPEARANCE_CHANGED_OBSERVER_NAME, superclass)
            .expect("Failed to create class decl");

        decl.add_ivar::<*const c_void>("callback_ptr");

        extern "C" fn observe_value(
            this: &Object,
            _sel: Sel,
            key_path: id,
            object: id,
            _change: id,
            _context: *mut c_void,
        ) {
            unsafe {
                let callback_ptr = *this.get_ivar::<*const c_void>("callback_ptr");
                if !callback_ptr.is_null() {
                    let key_str: id = msg_send![key_path, UTF8String];
                    let key_str = std::ffi::CStr::from_ptr(key_str as *const _).to_str().unwrap();

                    if key_str == "effectiveAppearance" {
                        let appearance: id = msg_send![object, effectiveAppearance];
                        let theme = appearance_to_theme(appearance);

                        // Cast back to Arc<dyn Fn(Theme) + Send + Sync>
                        let callback = &*(callback_ptr as *const Arc<dyn Fn(Theme) + Send + Sync>);
                        callback(theme);
                    }
                }
            }
        }

        unsafe {
            decl.add_method(
                sel!(observeValueForKeyPath:ofObject:change:context:),
                observe_value as extern "C" fn(&Object, Sel, id, id, id, *mut c_void),
            );
        }

        decl.register()
    };
}

fn appearance_to_theme(appearance: id) -> Theme {
    unsafe {
        let aqua = NSString::alloc(nil).init_str("NSAppearanceNameAqua");
        let dark_aqua = NSString::alloc(nil).init_str("NSAppearanceNameDarkAqua");
        let names = NSArray::arrayWithObjects(nil, &[aqua, dark_aqua]);

        let best_match: id = msg_send![appearance, bestMatchFromAppearancesWithNames: names];
        let is_dark: BOOL = msg_send![best_match, isEqualToString: dark_aqua];

        if is_dark == YES {
            Theme::Dark
        } else {
            Theme::Light
        }
    }
}

pub struct MacOSThemeMonitor {
    callback: Arc<dyn Fn(Theme) + Send + Sync>,
    observer: RefCell<Option<Id<Object>>>,
    running: AtomicBool,
    callback_index: RefCell<Option<usize>>,
}

impl MacOSThemeMonitor {
    pub fn new<F>(callback: F) -> Self
    where
        F: Fn(Theme) + Send + Sync + 'static,
    {
        Self {
            callback: Arc::new(callback),
            observer: RefCell::new(None),
            running: AtomicBool::new(false),
            callback_index: RefCell::new(None),
        }
    }
}

impl ThemeMonitor for MacOSThemeMonitor {
    fn start(&self) -> std::result::Result<(), Box<dyn Error>> {
        if self.running.load(Ordering::SeqCst) {
            return Err("Monitor is already running".into());
        }

        unsafe {
            // Initialize NSApplication if needed
            let app = NSApp();
            if app == nil {
                let _: id = msg_send![class!(NSApplication), sharedApplication];
            }
            let app = NSApp();

            let observer: id = msg_send![*OBJC_SUBCLASS, alloc];
            let observer: id = msg_send![observer, init];

            // Store callback in global storage and get a stable pointer
            let callback_clone = Arc::clone(&self.callback);
            let mut storage = CALLBACK_STORAGE.lock().unwrap();
            storage.push(callback_clone);
            let callback_index = storage.len() - 1;
            *self.callback_index.borrow_mut() = Some(callback_index);

            let callback_ptr = &storage[callback_index] as *const Arc<dyn Fn(Theme) + Send + Sync>
                as *const c_void;
            (*observer).set_ivar("callback_ptr", callback_ptr);

            let key_path = NSString::alloc(nil).init_str("effectiveAppearance");
            // NSKeyValueObservingOptionNew = 0x01
            let _: () = msg_send![
                app,
                addObserver: observer
                forKeyPath: key_path
                options: 0x01
                context: nil
            ];

            *self.observer.borrow_mut() = Some(Id::from_ptr(observer));
            self.running.store(true, Ordering::SeqCst);

            Ok(())
        }
    }

    fn stop(&self) -> std::result::Result<(), Box<dyn Error>> {
        if !self.running.load(Ordering::SeqCst) {
            return Ok(());
        }

        unsafe {
            if let Some(mut observer) = self.observer.borrow_mut().take() {
                let app = NSApp();
                let key_path = NSString::alloc(nil).init_str("effectiveAppearance");

                let _: () = msg_send![
                    app,
                    removeObserver: &*observer
                    forKeyPath: key_path
                ];

                observer.set_ivar("callback_ptr", std::ptr::null::<c_void>());
            }

            // Remove callback from storage
            if let Some(index) = self.callback_index.borrow_mut().take() {
                let mut storage = CALLBACK_STORAGE.lock().unwrap();
                if index < storage.len() {
                    storage.remove(index);
                }
            }

            self.running.store(false, Ordering::SeqCst);
            Ok(())
        }
    }

    fn get_current_theme(&self) -> Theme {
        unsafe {
            let app = NSApp();
            if app == nil {
                return Theme::Light;
            }

            let appearance: id = msg_send![app, effectiveAppearance];
            appearance_to_theme(appearance)
        }
    }
}

impl Drop for MacOSThemeMonitor {
    fn drop(&mut self) {
        let _ = self.stop();
    }
}

/// macOS-specific Application implementation
pub struct MacOSApplication {
    monitor: MacOSThemeMonitor,
}

impl MacOSApplication {
    pub fn new(handler: Arc<dyn ThemeHandler>) -> Result<Self> {
        Self::initialize()?;

        let monitor = MacOSThemeMonitor::new(move |theme| {
            handler.on_theme_change(theme);
        });

        Ok(Self { monitor })
    }
    
    pub fn get_current_theme(&self) -> Theme {
        self.monitor.get_current_theme()
    }

    fn initialize() -> Result<()> {
        unsafe {
            let app = NSApp();
            if app == nil {
                return Err(ThemeSwitcherError::PlatformError(
                    "Failed to initialize NSApplication".to_string(),
                ));
            }
            app.setActivationPolicy_(NSApplicationActivationPolicyAccessory);
        }
        Ok(())
    }
}

impl Application for MacOSApplication {
    fn run(self: Box<Self>) -> Result<()> {
        use crate::config::log_info;
        
        log_info("Starting theme monitor...");

        // Print initial theme
        log_info(&format!("Current theme: {}", self.monitor.get_current_theme()));

        // Start monitoring
        self.monitor.start()?;
        log_info("Monitoring for theme changes. Press Ctrl+C to stop.");

        // Run the macOS event loop
        unsafe {
            let app = NSApp();
            app.run();
        }

        Ok(())
    }
}
