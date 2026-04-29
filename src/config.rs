#[derive(Debug)]
pub struct Config {
    pub bedtime: Option<time::Time>,
    pub waketime: Option<time::Time>,
    pub nap: bool,

    // 12-hour mode (default) or 24-hour mode
    pub output_24hr_mode: bool
}
impl Config {
    pub fn format_options(&self) -> FormatOptions {
        FormatOptions {
            mode24: self.output_24hr_mode,
            with_padding: false
        }
    }
}

pub struct FormatOptions {
    pub mode24: bool,
    pub with_padding: bool
}

impl FormatOptions {
    pub fn padded(&self) -> Self {
        FormatOptions {
            with_padding: true,
            ..*self
        }
    }

    pub fn unpadded(&self) -> Self {
        FormatOptions {
            with_padding: false,
            ..*self
        }
    }
}