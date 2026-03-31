use time::{
    Time,
    macros::format_description
};
use cli_table::{format, Cell, Style, Table};


pub fn parse_time(s: String) -> Result<Time, Box<dyn std::error::Error>> {
    // prep valid fmts
    // check against each
    // propagate if all are Err
    // otherwise ret Time
    todo!()
}

pub fn format_time(t: &Time, format_options: &crate::config::FormatOptions) ->
    String  //Result<String, Box<dyn std::error::Error>> 
{
    let fmt_desc = if format_options.mode24 {
        format_description!("[hour padding:space]:[minute]")
    }
    else {
        format_description!("[hour padding:space repr:12]:[minute] [period case:upper]")
    };
    t.format(fmt_desc).unwrap()
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