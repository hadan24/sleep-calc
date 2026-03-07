use time::{Duration, Time};
use cli_table::{format, Cell, Style, Table};

const FALL_ASLEEP: Duration = Duration::minutes(15);
const CYCLE: Duration = Duration::minutes(90);

pub struct CyclePair(u8, String);
impl CyclePair {
    pub fn cell(self) -> Vec<cli_table::CellStruct> {
        vec![self.0.cell().justify(format::Justify::Right), self.1.cell()]
    }
}
pub struct FormatOptions {
    pub mode24: bool
}

pub fn get_wakeup_times(bedtime: &Time, format_options: &FormatOptions) -> Vec<CyclePair> {
    let sleep_time = *bedtime + FALL_ASLEEP + CYCLE;
    (1..7u8).rev()
        .map(|i| {
            let sleep_time = sleep_time + i*CYCLE;
            CyclePair(i, format_time(&sleep_time, format_options))
        })
        .collect()
}

pub fn get_bedtimes(wakeup: &Time, format_options: &FormatOptions) -> Vec<CyclePair> {
    let sleep_offset = *wakeup - FALL_ASLEEP;
    (1..7u8).rev()
        .map(|i| {
            let sleep_time = sleep_offset - i*CYCLE;
            CyclePair(i, format_time(&sleep_time, format_options))
        })
        .collect()
}

pub fn format_time(t: &Time, format_options: &FormatOptions) -> String {
    let (h, m) = (t.hour(), t.minute());

    if format_options.mode24 {
        match m {
            ..10 => format!("{h}:0{m}"),
            10.. => format!("{h}:{m}")
        }
    }
    else {
        let mut ftime = String::new();
        match h {
            0       => ftime.push_str("12"),
            1..=12  => ftime.push_str(&format!("{h}")),
            13..    => ftime.push_str(&format!("{}", h-12)),
        }
        match m {
            ..10 => ftime.push_str(&format!(":0{m}")),
            10.. => ftime.push_str(&format!(":{m}"))
        }
        match h {
            ..13 => ftime.push_str(" AM"),
            13.. => ftime.push_str(" PM"),
        }
        ftime
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