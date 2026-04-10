use anyhow::Context;
use clap::Parser;
use sleep_calc::*;

fn main() -> anyhow::Result<()> {
    let config = config::Config::parse();
    let fmt_opts = config.format_options();

    match (config.bedtime, config.waketime) {
        // given bed & wakeup times, find how many cycles can fit
        (Some(given_bedtime), Some(given_waketime)) => {
            let fmt_opts = fmt_opts.unpadded();
            let bedtime = io::parse_time(&given_bedtime)
                .context(format!("Couldn't parse given bedtime: `{given_bedtime}`.\n\tEnsure the given time is in an accepted format.\n\tIf this new format should also be accepted, submit an issue."))?;
            let waketime = io::parse_time(&given_waketime)
                .context(format!("Couldn't parse given wake-up time: `{given_waketime}`.\n\tEnsure the given time is in an accepted format.\n\tIf this new format should also be accepted, submit an issue."))?;
            let (cycles, ideal_waketime) = {
                let (c, t) = get_max_cycles_between(&bedtime, &waketime);
                let t = io::format_time(&t, &fmt_opts)
                    .context(format!("Couldn't format `{t}`"))?;
                (c, t)
            };
            println!(
                "Between {} and {}, you can get a maximum of: [{cycles}] cycles.",
                io::format_time(&bedtime, &fmt_opts).context(format!("Couldn't format `{bedtime}`"))?,
                io::format_time(&waketime, &fmt_opts).context(format!("Couldn't format `{waketime}`"))?
            );
            println!("Try to wake up at: {ideal_waketime}.");
        },

        // given chosen wakeup time, calculate bedtimes
        (None, Some(given_waketime)) => {
            let waketime = io::parse_time(&given_waketime)
                .context(format!("Couldn't parse given wake-up time: `{given_waketime}`.\n\tEnsure the given time is in an accepted format.\n\tIf this new format should also be accepted, submit an issue."))?;

            println!("Wake-up time: {}", io::format_time(&waketime, &fmt_opts.unpadded()).context(format!("Couldn't format `{waketime}`"))?);
            let cycles: Vec<CyclePair> = get_bedtimes(&waketime, &fmt_opts.padded());
            let rows: Vec<_> = cycles.into_iter()
                .map(|r| r.cell())
                .collect();
            println!("{}", io::build_table(rows, "Ideal Bedtimes")?);
        },

        // given chosen bedtime, calculate wakeup times
        (Some(given_bedtime), None) => {
            let bedtime = io::parse_time(&given_bedtime)
                .context(format!("Couldn't parse given bedtime: `{given_bedtime}`.\n\tEnsure the given time is in an accepted format.\n\tIf this new format should also be accepted, submit an issue."))?;

            println!("Bedtime: {}", io::format_time(&bedtime, &fmt_opts.unpadded()).context(format!("Couldn't format `{bedtime}`"))?);
            let cycles: Vec<CyclePair> = get_wakeup_times(&bedtime, &fmt_opts.padded());
            let rows: Vec<_> = cycles.into_iter()
                .map(|r| r.cell())
                .collect();
            println!("{}", io::build_table(rows, "Ideal Wake Times")?);
        },

        // default behavior: given bedtime of now, calculate wakeup times
        (None, None) => {
            let now = time::OffsetDateTime::now_local()
                .context("Could not get local timezone offset")?
                .time();

            println!("Bedtime: {}", io::format_time(&now, &fmt_opts.unpadded()).context(format!("Couldn't format `{now}`"))?);
            let cycles: Vec<CyclePair> = get_wakeup_times(&now, &fmt_opts.padded());
            let rows: Vec<_> = cycles.into_iter()
                .map(|r| r.cell())
                .collect();
            println!("{}", io::build_table(rows, "Wake-Up Time")?);
        }
    }

    Ok(())
}
