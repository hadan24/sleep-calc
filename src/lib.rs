pub mod config;
pub mod io;
pub mod error;

use time::{Duration, Time};


const FALL_ASLEEP: Duration = Duration::minutes(15);
const CYCLE: Duration = Duration::minutes(90);

pub struct CyclePair(u8, Time);

pub fn get_wakeup_times(bedtime: &Time) -> Vec<CyclePair> {
    let sleep_time = *bedtime + FALL_ASLEEP;
    (1..7u8).rev()
        .map(|i| CyclePair(i, sleep_time + i*CYCLE))
        .collect()
}

pub fn get_bedtimes(waketime: &Time) -> Vec<CyclePair> {
    let sleep_time = *waketime - FALL_ASLEEP;
    (1..7u8).rev()
        .map(|i| CyclePair(i, sleep_time - i*CYCLE))
        .collect()
}

pub fn get_max_cycles_between(bedtime: &Time, waketime: &Time) -> (u8, Time) {
    let sleep_start = *bedtime + FALL_ASLEEP;
    let sleep_time = sleep_start.duration_until(*waketime);

    let cycles = (sleep_time / CYCLE) as u8;
    
    (cycles, sleep_start + CYCLE*cycles)
}