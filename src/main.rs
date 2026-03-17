use clap::Parser;
use sleep_calc::*;

fn main() {
    let config = config::Config::parse();
    let fmt_opts = config.format_options();
    dbg!(&config);

    match (config.bedtime, config.waketime) {
        // given bed & wakeup times, find how many cycles can fit
        (Some(bedtime), Some(waketime)) => {
            use io::{format_time, parse_time};
            let bedtime = format_time(&parse_time(bedtime).unwrap(), &fmt_opts);
            let waketime = format_time(&parse_time(waketime).unwrap(), &fmt_opts);
            println!("bed: {bedtime}\nwake: {waketime}");
        },

        // given chosen wakeup time, calculate bedtimes
        (None, Some(waketime)) => {
            todo!()
        },

        // given chosen bedtime, calculate wakeup times
        (Some(bedtime), None) => {
            todo!()
        },

        // default behavior: given bedtime of now, calculate wakeup times
        (None, None) => {
            let now = match time::OffsetDateTime::now_local() {
                Ok(t) => t.time(),
                Err(e) => {
                    eprintln!("Could not get timezone offset: {}", e);
                    std::process::exit(1);
                }
            };

            println!("Bedtime: {}", io::format_time(&now, &fmt_opts));
            let cycles: Vec<CyclePair> = get_wakeup_times(&now, &fmt_opts);
            let rows: Vec<_> = cycles.into_iter()
                .map(|r| r.cell())
                .collect();
            println!("{}", io::build_table(rows, "Wake-Up Time"));
        }
    }
}
