use time::Time;
use cli_table::{format, Cell, Style, Table};

/*
no am/pm
    split on ':', plug in h/m
w/ am/pm
    strip + keep
    if am, plug in h/m
    if pm, plug in (h+12)/m
*/
pub fn parse_time(s: String) -> Result<Time, Box<dyn std::error::Error>> {
    let s = s.trim().to_lowercase();
    let suffixes = ["am", "pm"];
    let (s, found_suffix) = suffixes.iter()
        // apply callable that returns Option on each iter element/
        // return 1st non-None item, or None itself if all were None
        .find_map(|&suffix| {
            s.strip_suffix(suffix)
                .map(|stripped| (stripped.to_string(), Some(suffix)))
        })
        .unwrap_or((s, None));  // unwrap find_map Option or set to default value
    let mut it = s.split(":");

    match found_suffix {
        Some(suffix) => {
            let h = if suffix == "am" {
                let temp = it.next().unwrap().parse::<u8>()?;
                if temp == 12 {
                    temp - 12
                } else {
                    temp
                }
            } else {
                (it.next().unwrap().parse::<u8>()? + 12) % 24
            };
            Ok(Time::from_hms(h, it.next().unwrap().parse()?, 0)?)
        },
        None => {   // no am/pm
            let (h, m) = (
                it.next().unwrap().parse::<u8>()?,
                it.next().unwrap().parse::<u8>()?
            );
            Ok(Time::from_hms(h, m, 0)?)
        }
    }
}

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