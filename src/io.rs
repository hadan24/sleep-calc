use time::{
    Time,
    macros::format_description
};
use cli_table::{format, Cell, Style, Table};


pub fn parse_time(s: String) -> Result<Time, Box<dyn std::error::Error>> {
    let fmts = [
        format_description!("[hour repr:12]:[minute] [period case_sensitive:false]"),
        format_description!("[hour]:[minute]"),
    ];
    enum ParseResults {
        FirstErr(time::error::Parse),
        ParsedTime(time::Time),
        Unknown
    }
    let mut res = ParseResults::Unknown;
    for f in fmts {
        match Time::parse(&s, f) {
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
    let fmt_desc = if format_options.mode24 {
        format_description!("[hour padding:space]:[minute]")
    }
    else {
        format_description!("[hour padding:space repr:12]:[minute] [period case:upper]")
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