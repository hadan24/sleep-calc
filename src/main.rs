fn main() {
    let now = time::OffsetDateTime::now_local()
        .expect("Could not get timezone offset.")
        .truncate_to_second()
        .time();
    println!("\nCurrent time: {}", now);

    let rows: Vec<_> = sleep_calc::get_wakeup_times(&now)
        .into_iter()
        .map(|r| {
            use cli_table::{Cell, format::Justify};
            vec![r.0.cell().justify(Justify::Right), r.1.cell()]
        })
        .collect();
    let table = sleep_calc::build_table(rows, "Wake-Up Time");

    println!("{}", table);
}
