use time::{Duration, Time};
use cli_table::{format, Cell, Style, Table};

const FALL_ASLEEP: Duration = Duration::minutes(15);
const CYCLE: Duration = Duration::minutes(90);
type CyclePair = (u8, String);

pub fn get_wakeup_times(bedtime: &Time) -> Vec<CyclePair> {
    let sleep_time = *bedtime + FALL_ASLEEP;
    (1..7u8).rev()
        .map(|i| {
            let sleep_time = sleep_time + i*CYCLE;
            (i, format_time(&sleep_time))
        })
        .collect()
}

pub fn get_bedtimes(wakeup: &Time) -> Vec<CyclePair> {
    let sleep_offset = *wakeup - FALL_ASLEEP;
    (1..7u8).rev()
        .map(|i| {
            let sleep_time = sleep_offset - i*CYCLE;
            (i, format_time(&sleep_time))
        })
        .collect()
}

fn format_time(t: &Time) -> String {
    let (h, m) = (t.hour(), t.minute());
    match m {
        ..10 => format!("{}:0{}", h, m),
        _ => format!("{}:{}", h, m)
    }
}

pub fn build_table(rows: Vec<Vec<cli_table::CellStruct>>, times_col_title: &str)
    -> cli_table::TableDisplay
{
    let sep = format::Separator::builder()
        .column(Some(format::VerticalLine::new('|')))
        .title(Some(format::HorizontalLine::new('+', '+', '+', '-')));

    rows.table()
        .title( vec!["# Cycles".cell().bold(true), times_col_title.cell().bold(true)] )
        .separator(sep.build())
        .display()
        .expect("Could not format table.")
}