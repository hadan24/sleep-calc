use anyhow::Context;
use clap::Parser;
use sleep_calc::{
    *,
    io::*,
    error::*
};


fn main() -> anyhow::Result<()> {
    let config = config::Config::parse();
    let fmt_opts = config.format_options().unpadded();
    println!();

    match (config.bedtime, config.waketime) {
        // given bed & wakeup times, find how many cycles can fit
        (Some(given_bedtime), Some(given_waketime)) => {
            let bedtime = parse_time(&given_bedtime)
                .context(parsing_context_msg(&given_bedtime))?;
            let waketime = parse_time(&given_waketime)
                .context(parsing_context_msg(&given_waketime))?;
            let (cycles, ideal_waketime) = {
                let (c, t) = get_max_cycles_between(&bedtime, &waketime);
                let t = format_time(&t, &fmt_opts)
                    .context(formatting_context_msg(&t))?;
                (c, t)
            };

            println!(
                "Between {} and {}, you can get a maximum of: [{cycles}] cycles.",
                format_time(&bedtime, &fmt_opts).context(formatting_context_msg(&bedtime))?,
                format_time(&waketime, &fmt_opts).context(formatting_context_msg(&waketime))?
            );
            println!("Try to wake up at: {ideal_waketime}.");
        },

        // given chosen wakeup time, calculate bedtimes
        (None, Some(given_waketime)) => {
            let waketime = parse_time(&given_waketime)
                .context(parsing_context_msg(&given_waketime))?;

            println!(
                "Wake-up time: {}",
                format_time(&waketime, &fmt_opts).context(formatting_context_msg(&waketime))?
            );
            let cycles = get_bedtimes(&waketime);
            let tbl = build_table(cycles, "Ideal Bedtimes", &fmt_opts.padded())
                .context(TABLE_FORMATTING_ERR_MSG)?;
            println!("{tbl}");
        },

        // given chosen bedtime, calculate wakeup times
        (Some(given_bedtime), None) => {
            let bedtime = parse_time(&given_bedtime)
                .context(parsing_context_msg(&given_bedtime))?;

            println!(
                "Bedtime: {}",
                format_time(&bedtime, &fmt_opts).context(formatting_context_msg(&bedtime))?,
            );
            let cycles = get_wakeup_times(&bedtime);
            let tbl = build_table(cycles, "Ideal Wake Times", &fmt_opts.padded())
                .context(TABLE_FORMATTING_ERR_MSG)?;
            println!("{tbl}");
        },

        // default behavior: given bedtime of now, calculate wakeup times
        (None, None) => {
            let now = time::OffsetDateTime::now_local()
                .context("Could not get local timezone offset")?
                .time();

            println!(
                "Bedtime: {}",
                format_time(&now, &fmt_opts).context(formatting_context_msg(&now))?,
            );
            let cycles = get_wakeup_times(&now);
            let tbl = build_table(cycles, "Ideal Wake Times", &fmt_opts.padded())
                .context(TABLE_FORMATTING_ERR_MSG)?;
            println!("{tbl}");
        }
    }

    Ok(())
}
