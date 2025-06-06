use std::sync::OnceLock;

static QUIET_MODE: OnceLock<bool> = OnceLock::new();

pub fn set_quiet_mode(quiet: bool) {
    QUIET_MODE.set(quiet).ok();
}

pub fn is_quiet_mode() -> bool {
    *QUIET_MODE.get().unwrap_or(&false)
}

pub fn log_info(message: &str) {
    if !is_quiet_mode() {
        println!("{}", message);
    }
}