pub mod config;
pub mod io;

use time::{Duration, Time};
use cli_table::{format, Cell};
use config::FormatOptions;

const FALL_ASLEEP: Duration = Duration::minutes(15);
const CYCLE: Duration = Duration::minutes(90);

pub struct CyclePair(u8, String);
impl CyclePair {
    pub fn cell(self) -> Vec<cli_table::CellStruct> {
        vec![self.0.cell().justify(format::Justify::Right), self.1.cell()]
    }
}

pub fn get_wakeup_times(bedtime: &Time, format_options: &FormatOptions) -> Vec<CyclePair> {
    let sleep_time = *bedtime + FALL_ASLEEP;
    (1..7u8).rev()
        .map(|i| {
            let time_str = io::format_time(&(sleep_time + i*CYCLE), format_options)
                .inspect_err(|e| eprintln!("Got error: {e}\nwhile formatting time: {sleep_time} ({i}th cycle)"))
                .unwrap_or("Formatting failed! Check error logs and/or report an issue.".into());
            let display_time = match i {
                ..=4 => time_str,
                5.. => format!("{time_str} (recommended!)")
            };
            CyclePair(i, display_time)
        })
        .collect()
}

pub fn get_bedtimes(waketime: &Time, format_options: &FormatOptions) -> Vec<CyclePair> {
    let sleep_time = *waketime - FALL_ASLEEP;
    (1..7u8).rev()
        .map(|i| {
            let time_str = io::format_time(&(sleep_time - i*CYCLE), format_options)
                .inspect_err(|e| eprintln!("Got error: {e}\nwhile formatting time: {sleep_time} ({i}th cycle)"))
                .unwrap_or("Formatting failed! Check error logs and/or report an issue.".into());
            let display_time = match i {
                ..=4 => time_str,
                5.. => format!("{time_str} (recommended!)")
            };
            CyclePair(i, display_time)
        })
        .collect()
}

pub fn get_max_cycles_between(bedtime: &Time, waketime: &Time) -> (u8, Time) {
    let sleep_start = *bedtime + FALL_ASLEEP;
    let sleep_time = sleep_start.duration_until(*waketime);

    let cycles = (sleep_time / CYCLE) as u8;
    
    (cycles, sleep_start + CYCLE*cycles)
}