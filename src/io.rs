use anyhow::Context;
use crate::{
    CyclePair,
    io,
    config::FormatOptions,
    error
};
use time::{
    Time,
    macros::format_description as fmt_desc
};
use cli_table::{format, Cell, Style, Table};


const FMTS: [&[time::format_description::BorrowedFormatItem<'static>]; 8] = [
    fmt_desc!("[hour repr:12 padding:none]:[minute] [period case_sensitive:false]"),    // 3:00 pm
    fmt_desc!("[hour repr:12 padding:none]:[minute][period case_sensitive:false]"), // 3:00pm
    fmt_desc!("[hour repr:12 padding:none] [period case_sensitive:false]"),         // 3 pm
    fmt_desc!("[hour repr:12 padding:none][period case_sensitive:false]"),          // 3pm
    fmt_desc!("[hour padding:none]:[minute]"),  // 15:00
    fmt_desc!("[hour padding:none]"),           // 18
    fmt_desc!("[hour padding:none][minute]"),   // 1500, 300
    fmt_desc!("[hour padding:zero][minute]"),   // 1500, 0300
];
pub fn parse_time(s: &str) -> anyhow::Result<Time> {
    let s = s.trim();
    let mut first_err = None;

    for f in FMTS {
        match Time::parse(s, f) {
            Ok(t) => return Ok(t),
            Err(e) => first_err.get_or_insert(e)
        };
    }
    
    Err(first_err.unwrap().into())  // only here if NO formats matched, error guaranteed exists
}

pub fn format_time(t: &Time, format_options: &FormatOptions)
    -> anyhow::Result<String>
{
    let fmt_desc = match (format_options.mode24, format_options.with_padding) {
        (true, true)    => fmt_desc!("[hour padding:space]:[minute]"),
        (true, false)   => fmt_desc!("[hour padding:none]:[minute]"),
        (false, true)   => fmt_desc!("[hour padding:space repr:12]:[minute] [period case:upper]"),
        (false, false)  => fmt_desc!("[hour padding:none repr:12]:[minute] [period case:upper]")
    };
    let str = t.format(fmt_desc)
        .context(format!("Could not format time: {t}"))?;
    Ok(str)
}

struct CycleDisplayPair(u8, String);
impl CycleDisplayPair {
    fn from_cycle_pair(p: CyclePair, fmt_opts: &FormatOptions) -> anyhow::Result<Self> {
        let (i, t) = (p.0, p.1);
        let time_str = io::format_time(&t, fmt_opts)
            .context(format!("{} `{t}` ({i}th sleep cycle)", error::FORMATTING_ERR_MSG))?;
        let display_time = match i {
            ..=4 => time_str,
            5.. => format!("{time_str} (recommended!)")
        };

        Ok(CycleDisplayPair(i, display_time))
    }

    fn cell(self) -> Vec<cli_table::CellStruct> {
        vec![self.0.cell().justify(format::Justify::Right), self.1.cell()]
    }
}

pub fn build_table(rows: Vec<CyclePair>, times_col_title: &str, fmt_opts: &FormatOptions)
    -> anyhow::Result<cli_table::TableDisplay>
{
    let rows: Vec<Vec<cli_table::CellStruct>> = rows.into_iter()
        .map(|p| CycleDisplayPair::from_cycle_pair(p, fmt_opts) )
        .collect::< anyhow::Result<Vec<CycleDisplayPair>> >()?
        .into_iter()
        .map(|p| p.cell())
        .collect();

    let sep = format::Separator::builder()
        .column(Some(format::VerticalLine::new('|')))
        .title(Some(format::HorizontalLine::new('+', '+', '+', '~')));

    let tbl = rows.table()
        .title( vec!["Cycles".cell().bold(true), times_col_title.cell().bold(true)] )
        .separator(sep.build())
        .display()
        .context("Could not create display-able table")?;
    Ok(tbl)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_12hr_am() {
        let parsed = parse_time("8:00 AM").expect("Should be able to parse valid 12hr time");
        let t = Time::from_hms(8, 0, 0).expect("Hard-coded Time should be valid");
        assert_eq!(parsed, t);
    }
    #[test]
    fn parse_12hr_pm() {
        let input = parse_time("8:00 PM").expect("Should be able to parse valid 12hr time");
        let t = Time::from_hms(8 + 12, 0, 0).expect("Hard-coded Time should be valid");
        assert_eq!(input, t);
    }
    #[test]
    fn parse_12hr_mixed_case() {
        let input = parse_time("3:30 pM").expect("Should be able to parse valid 12hr time");
        let t = Time::from_hms(3 + 12, 30, 0).expect("Hard-coded Time should be valid");
        assert_eq!(input, t);
    }
    #[test]
    fn parse_12hr_midnight() {
        let input = parse_time("12:00 AM").expect("Should be able to parse valid 12hr time");
        let t = Time::from_hms(0, 0, 0).expect("Hard-coded Time should be valid");
        assert_eq!(input, t);
    }
    #[test]
    fn parse_12hr_noon() {
        let input = parse_time("12:00 PM").expect("Should be able to parse valid 12hr time");
        let t = Time::from_hms(12, 0, 0).expect("Hard-coded Time should be valid");
        assert_eq!(input, t);
    }
    #[test]
    fn parse_24hr_am() {
        let input = parse_time("8:00").expect("Should be able to parse valid 24hr time");
        let t = Time::from_hms(8, 0, 0).expect("Hard-coded Time should be valid");
        assert_eq!(input, t);
    }
    #[test]
    fn parse_24hr_pm() {
        let input = parse_time("16:00").expect("Should be able to parse valid 24hr time");
        let t = Time::from_hms(16, 0, 0).expect("Hard-coded Time should be valid");
        assert_eq!(input, t);
    }
    #[test]
    fn parsing_invalid_time_fails() {
        assert!(parse_time("24:00 PM").is_err());
    }

    #[test]
    fn parse_12hr_no_space_before_period() {
        let input = parse_time("12:00AM").expect("Should be able to parse valid 12hr time");
        let t = Time::from_hms(0, 0, 0).expect("Hard-coded Time should be valid");
        assert_eq!(input, t);
    }
}