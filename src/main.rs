use time::*;
use cli_table::{Cell, Style, Table, format::{HorizontalLine, Justify, Separator, VerticalLine}};

const FALL_ASLEEP: Duration = Duration::minutes(15);
const CYCLE: Duration = Duration::minutes(90);

fn main() {
    let now = OffsetDateTime::now_local()
        .expect("Could not get timezone offset.")
        .truncate_to_second()
        .time();
    println!("\nCurrent time: {}", now);
    let sleep_start = now + FALL_ASLEEP;

    let sep = Separator::builder()
        .column(Some(VerticalLine::new('|')))
        .title(Some(HorizontalLine::new('+', '+', '+', '-')));

    let rows: Vec<_> = (1..7).rev()
        .map(|i| {
            let t: Time = sleep_start + i*CYCLE;
            let h = t.hour();
            let m = t.minute();
            let t = match m {
                ..10 => format!("{}:0{}", h, m),
                _ => format!("{}:{}", h, m)
            };
            
            vec![i.cell().justify(Justify::Right), t.cell()]
        })
        .collect();
    let table = rows.table()
        .title(vec!["# Cycles".cell().bold(true), "Wake-Up Time".cell().bold(true)])
        .separator(sep.build())
        .display()
        .expect("Could not format table.");

    println!("{}", table);
}
