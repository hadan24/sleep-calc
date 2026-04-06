use time::{Duration, Time};
use cli_table::{format, Cell};

pub mod config;
pub mod io;

const FALL_ASLEEP: Duration = Duration::minutes(15);
const CYCLE: Duration = Duration::minutes(90);

pub struct CyclePair(u8, String);
impl CyclePair {
    pub fn cell(self) -> Vec<cli_table::CellStruct> {
        vec![self.0.cell().justify(format::Justify::Right), self.1.cell()]
    }
}

pub fn get_wakeup_times(bedtime: &Time, format_options: &config::FormatOptions) -> Vec<CyclePair> {
    let sleep_time = *bedtime + FALL_ASLEEP + CYCLE;
    (1..7u8).rev()
        .map(|i| {
            let sleep_time = sleep_time + i*CYCLE;
            CyclePair(i, io::format_time(&sleep_time, format_options).unwrap())
        })
        .collect()
}

pub fn get_bedtimes(waketime: &Time, format_options: &config::FormatOptions) -> Vec<CyclePair> {
    let sleep_offset = *waketime - FALL_ASLEEP;
    (1..7u8).rev()
        .map(|i| {
            let sleep_time = sleep_offset - i*CYCLE;
            CyclePair(i, io::format_time(&sleep_time, format_options).unwrap())
        })
        .collect()
}
