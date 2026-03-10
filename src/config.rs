use clap::Parser;

#[derive(Parser, Debug)]
pub struct Config {
    #[arg(short, long)]
    pub bedtime: Option<String>,

    #[arg(short, long)]
    pub wakeup: Option<String>,

    #[arg(short, long, default_value_t = false)]
    pub nap: bool,

    // 12-hour mode (default) or 24-hour mode
    #[arg(short, long, default_value_t = false)]
    pub mode24: bool
}
impl Config {
    pub fn format_options(&self) -> FormatOptions {
        FormatOptions { mode24: self.mode24 }
    }
}

pub struct FormatOptions {
    pub mode24: bool
}