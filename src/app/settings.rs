/// # App Settings
#[derive(Debug, Clone, Copy)]
pub struct Settings {
    pub fps: u32,
}

impl Default for Settings {
    fn default() -> Self {
        Self { fps: 60 }
    }
}
