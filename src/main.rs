use clap::Parser;

fn main() {
    let config = Config::parse();
    dbg!(&config);

    // given bed & wakeup times, find how many cycles can fit
    if config.bedtime.is_some() && config.wakeup.is_some() {
        todo!()
    }
    // given chosen wakeup time, calculate bedtimes
    else if config.wakeup.is_some() {
        todo!()
    }
    // given chosen bedtime, calculate wakeup times
    else if config.bedtime.is_some() {
        todo!()
    }
    // default behavior: given bedtime of now, calculate wakeup times
    else {
        let now = time::OffsetDateTime::now_local()
            .expect("Could not get timezone offset.")
            .time();
        println!("Bedtime: {}", sleep_calc::format_time(&now, &config.format_options()));
        let cycles: Vec<sleep_calc::CyclePair> = sleep_calc::get_wakeup_times(&now, &config.format_options());
        let rows: Vec<_> = cycles.into_iter()
            .map(|r| r.cell())
            .collect();
        let table = sleep_calc::build_table(rows, "Wake-Up Time");
        println!("{}", table);
    }
}

#[derive(Parser, Debug)]
struct Config {
    #[arg(short, long)]
    bedtime: Option<String>,

    #[arg(short, long)]
    wakeup: Option<String>,

    #[arg(short, long, default_value_t = false)]
    nap: bool,

    // 12-hour mode (default) or 24-hour mode
    #[arg(short, long, default_value_t = false)]
    mode24: bool
}
impl Config {
    fn format_options(&self) -> sleep_calc::FormatOptions {
        sleep_calc::FormatOptions { mode24: self.mode24 }
    }
}
