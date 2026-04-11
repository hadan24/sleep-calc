use anyhow::Context;
use clap::Parser;
use sleep_calc::{
    *,
    error::*
};


fn main() -> anyhow::Result<()> {
    let config = config::Config::parse();
    let fmt_opts = config.format_options().unpadded();

    match (config.bedtime, config.waketime) {
        // given bed & wakeup times, find how many cycles can fit
        (Some(given_bedtime), Some(given_waketime)) => {
            let bedtime = io::parse_time(&given_bedtime)
                .context(format!("{PARSING_ERR_MSG} `{given_bedtime}`. {SUBMIT_PARSING_FMT_MSG}"))?;
            let waketime = io::parse_time(&given_waketime)
                .context(format!("{PARSING_ERR_MSG} `{given_waketime}`. {SUBMIT_PARSING_FMT_MSG}"))?;
            let (cycles, ideal_waketime) = {
                let (c, t) = get_max_cycles_between(&bedtime, &waketime);
                let t = io::format_time(&t, &fmt_opts)
                    .context(format!("{FORMATTING_ERR_MSG} `{t}`"))?;
                (c, t)
            };
            println!(
                "Between {} and {}, you can get a maximum of: [{cycles}] cycles.",
                io::format_time(&bedtime, &fmt_opts).context(format!("{FORMATTING_ERR_MSG} `{bedtime}`"))?,
                io::format_time(&waketime, &fmt_opts).context(format!("{FORMATTING_ERR_MSG} `{waketime}`"))?
            );
            println!("Try to wake up at: {ideal_waketime}.");
        },

        // given chosen wakeup time, calculate bedtimes
        (None, Some(given_waketime)) => {
            let waketime = io::parse_time(&given_waketime)
                .context(format!("{PARSING_ERR_MSG} `{given_waketime}`. {SUBMIT_PARSING_FMT_MSG}"))?;

            println!(
                "Wake-up time: {}",
                io::format_time(&waketime, &fmt_opts).context(format!("{FORMATTING_ERR_MSG} `{waketime}`"))?
            );
            let cycles = get_bedtimes(&waketime);
            let tbl = io::build_table(cycles, "Ideal Bedtimes", &fmt_opts.padded())
                .context(TABLE_FORMATTING_ERR_MSG)?;
            println!("{tbl}");
        },

        // given chosen bedtime, calculate wakeup times
        (Some(given_bedtime), None) => {
            let bedtime = io::parse_time(&given_bedtime)
                .context(format!("{PARSING_ERR_MSG} `{given_bedtime}`. {SUBMIT_PARSING_FMT_MSG}"))?;

            println!(
                "Bedtime: {}",
                io::format_time(&bedtime, &fmt_opts).context(format!("{FORMATTING_ERR_MSG} `{bedtime}`"))?
            );
            let cycles = get_wakeup_times(&bedtime);
            let tbl = io::build_table(cycles, "Ideal Wake Times", &fmt_opts.padded())
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
                io::format_time(&now, &fmt_opts).context(format!("{FORMATTING_ERR_MSG} `{now}`"))?
            );
            let cycles = get_wakeup_times(&now);
            let tbl = io::build_table(cycles, "Ideal Wake Times", &fmt_opts.padded())
                .context(TABLE_FORMATTING_ERR_MSG)?;
            println!("{tbl}");
        }
    }

    Ok(())
}
