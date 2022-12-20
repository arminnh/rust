// use std::fs;

fn main() {
    let input = "descr  ,amount, unit_price,total_price
    Cookies,     4,       2.95,=B2 * C2
    Coffee ,     1,=9.60 * 0.8,0
    Water  ,     2,       1.20,0
    Total  ,      ,           ,=SUM(D2:D4)"
        .to_string();
    // let input =
    // fs::read_to_string("examples/arithmetic_with_literals.csv").expect("Could not load file.");
    simple_spreadsheet_engine::run(input);
}
