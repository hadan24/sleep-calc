use clap::Parser;
use sleep_calc::*;

fn main() {
    let config = config::Config::parse();
    let fmt_opts = config.format_options();

    match (config.bedtime, config.waketime) {
        // given bed & wakeup times, find how many cycles can fit
        (Some(given_bedtime), Some(given_waketime)) => {
            let fmt_opts = fmt_opts.unpadded();
            let bedtime = io::parse_time(&given_bedtime).unwrap();
            let waketime = io::parse_time(&given_waketime).unwrap();
            let (cycles, ideal_wake) = get_max_cycles_between(&bedtime, &waketime);
            println!(
                "Between {} and {}, you can get a maximum of: [{cycles}] cycles.",
                io::format_time(&bedtime, &fmt_opts).unwrap(),
                io::format_time(&waketime, &fmt_opts).unwrap(),
            );
            println!("Try to wake up at: {}.", io::format_time(&ideal_wake, &fmt_opts).unwrap());
        },

        // given chosen wakeup time, calculate bedtimes
        (None, Some(given_waketime)) => {
            let waketime = io::parse_time(&given_waketime).unwrap();

            println!("Wake-up time: {}", io::format_time(&waketime, &fmt_opts.unpadded()).unwrap());
            let cycles: Vec<CyclePair> = get_bedtimes(&waketime, &fmt_opts.padded());
            let rows: Vec<_> = cycles.into_iter()
                .map(|r| r.cell())
                .collect();
            println!("{}", io::build_table(rows, "Ideal Bedtimes"));
        },

        // given chosen bedtime, calculate wakeup times
        (Some(given_bedtime), None) => {
            let bedtime = io::parse_time(&given_bedtime).unwrap();

            println!("Bedtime: {}", io::format_time(&bedtime, &fmt_opts.unpadded()).unwrap());
            let cycles: Vec<CyclePair> = get_wakeup_times(&bedtime, &fmt_opts.padded());
            let rows: Vec<_> = cycles.into_iter()
                .map(|r| r.cell())
                .collect();
            println!("{}", io::build_table(rows, "Ideal Wake Times"));
        },

        // default behavior: given bedtime of now, calculate wakeup times
        (None, None) => {
            let now = match time::OffsetDateTime::now_local() {
                Ok(t) => t.time(),
                Err(e) => {
                    eprintln!("Could not get timezone offset: {}", e);
                    return;
                }
            };

            println!("Bedtime: {}", io::format_time(&now, &fmt_opts.unpadded()).unwrap());
            let cycles: Vec<CyclePair> = get_wakeup_times(&now, &fmt_opts.padded());
            let rows: Vec<_> = cycles.into_iter()
                .map(|r| r.cell())
                .collect();
            println!("{}", io::build_table(rows, "Wake-Up Time"));
        }
    }
}
