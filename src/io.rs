use time::{
    Time,
    macros::format_description
};
use cli_table::{format, Cell, Style, Table};


pub fn parse_time(s: &str) -> Result<Time, Box<dyn std::error::Error>> {
    let fmts = [
        format_description!("[hour repr:12 padding:none]:[minute] [period case_sensitive:false]"),
        format_description!("[hour padding:none]:[minute]"),
    ];
    enum ParseResults {
        FirstErr(time::error::Parse),
        ParsedTime(time::Time),
        Unknown
    }
    let mut res = ParseResults::Unknown;
    for f in fmts {
        match Time::parse(&s.trim(), f) {
            Ok(parsed_time) => {
                res = ParseResults::ParsedTime(parsed_time);
                break;
            }
            Err(e) => match res {
                ParseResults::Unknown => res = ParseResults::FirstErr(e),

                // if is FirstErr, already found err to return, skip
                // if is ParsedTime (should be impossible), already succesfully parsed, skip
                _ => break
            }
        }
    }
    
    match res {
        ParseResults::FirstErr(e)   => Err(Box::new(e)),
        ParseResults::ParsedTime(t) => Ok(t),
        ParseResults::Unknown => Err("shouldn't be possible".into())
    }
}

pub fn format_time(t: &Time, format_options: &crate::config::FormatOptions) ->
    Result<String, Box<dyn std::error::Error>> 
{
    let fmt_desc = match format_options.mode24 {
        true    => format_description!("[hour padding:space]:[minute]"),
        false   => format_description!("[hour padding:space repr:12]:[minute] [period case:upper]")
    };
    Ok(t.format(fmt_desc)?)
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
        .inspect_err(|e| eprintln!("Could not format table: {}", e))
        .unwrap()   // if can't format, abort since std::io::Error is typically OS issue
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
        let input = parse_time("24:00 PM");
        assert!(input.is_err());
    }
}