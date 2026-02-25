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
        initial_test();
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

fn initial_test() {
    let now = time::OffsetDateTime::now_local()
        .expect("Could not get timezone offset.")
        .truncate_to_second()
        .time();
    println!("\nCurrent time: {}", now);

    let rows: Vec<_> = sleep_calc::get_wakeup_times(&now)
        .into_iter()
        .map(|r| {
            use cli_table::{Cell, format::Justify};
            vec![r.0.cell().justify(Justify::Right), r.1.cell()]
        })
        .collect();
    let table = sleep_calc::build_table(rows, "Wake-Up Time");

    println!("{}", table);
}