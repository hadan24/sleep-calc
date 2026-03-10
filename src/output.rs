use time::Time;
use cli_table::{format, Cell, Style, Table};

pub fn format_time(t: &Time, format_options: &crate::config::FormatOptions) -> String {
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