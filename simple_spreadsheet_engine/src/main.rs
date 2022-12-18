use std::fs;

fn main() {
    // let input = "10,=A1 - 1,=A1 - 2,=A1 - 3, 50".to_string();
    let input =
        fs::read_to_string("examples/arithmetic_with_literals.csv").expect("Could not load file.");
    simple_spreadsheet_engine::run(input);
}
