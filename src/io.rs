use anyhow::Context;
use time::{
    Time,
    macros::format_description
};
use cli_table::{format, Cell, Style, Table};


pub fn parse_time(s: &str) -> anyhow::Result<Time> {
    let s = s.trim();
    let mut first_err = None;
    let fmts = [
        format_description!("[hour repr:12 padding:none]:[minute] [period case_sensitive:false]"),
        format_description!("[hour repr:12 padding:none]:[minute][period case_sensitive:false]"),
        format_description!("[hour padding:none]:[minute]"),
    ];

    for f in fmts {
        match Time::parse(s, f) {
            Ok(t) => return Ok(t),
            Err(e) => first_err.get_or_insert(e)
        };
    }
    
    Err(first_err.unwrap().into())  // only here if NO formats matched, error guaranteed exists
}

pub fn format_time(t: &Time, format_options: &crate::config::FormatOptions)
    -> anyhow::Result<String>
{
    let fmt_desc = match (format_options.mode24, format_options.with_padding) {
        (true, true)    => format_description!("[hour padding:space]:[minute]"),
        (true, false)   => format_description!("[hour padding:none]:[minute]"),
        (false, true)   => format_description!("[hour padding:space repr:12]:[minute] [period case:upper]"),
        (false, false)  => format_description!("[hour padding:none repr:12]:[minute] [period case:upper]")
    };
    let str = t.format(fmt_desc)
        .context(format!("Could not format time: {t}"))?;
    Ok(str)
}

pub fn build_table(rows: Vec<Vec<cli_table::CellStruct>>, times_col_title: &str)
    -> anyhow::Result<cli_table::TableDisplay>
{
    let sep = format::Separator::builder()
        .column(Some(format::VerticalLine::new('|')))
        .title(Some(format::HorizontalLine::new('+', '+', '+', '-')));

    let tbl = rows.table()
        .title( vec!["Cycles".cell().bold(true), times_col_title.cell().bold(true)] )
        .separator(sep.build())
        .display()
        .context("Could not format table for display")?;
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