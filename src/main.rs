use clap::Parser;
use sleep_calc::*;

fn main() {
    let config = config::Config::parse();
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
        let now = match time::OffsetDateTime::now_local() {
            Ok(t) => t.time(),
            Err(e) => {
                eprintln!("Could not get timezone offset: {}", e);
                std::process::exit(1);
            }
        };

        println!("Bedtime: {}", output::format_time(&now, &config.format_options()));
        let cycles: Vec<CyclePair> = get_wakeup_times(&now, &config.format_options());
        let rows: Vec<_> = cycles.into_iter()
            .map(|r| r.cell())
            .collect();
        println!("{}", output::build_table(rows, "Wake-Up Time"));
    }
}
