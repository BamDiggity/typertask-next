pub struct TextExpander;

impl TextExpander {
    pub fn new() -> Self {
        Self
    }
    
    pub fn start_hook(&self) {
        #[cfg(windows)]
        {
            tracing::info!("Starting Win32 WH_KEYBOARD_LL hook for text expansion");
        }
        #[cfg(not(windows))]
        {
            tracing::info!("Mock TextExpander hook started");
        }
    }
}
