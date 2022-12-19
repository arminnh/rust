use std::fs;

// TODO: parametrized tests instead of asserting each variant manually.
// Could do it through macros or with a package https://crates.io/crates/rstest

fn compare_files(name: &str) {
    let input = fs::read_to_string("examples/".to_owned() + name + ".csv").unwrap();
    let expected_output = fs::read_to_string("examples/".to_owned() + name + ".out.csv").unwrap();
    assert_eq!(
        simple_spreadsheet_engine::run(input),
        expected_output.trim()
    );
}

#[test]
fn arithmetic_with_literals() {
    compare_files("arithmetic_with_literals")
}

#[test]
fn arithmetic_with_references() {
    compare_files("arithmetic_with_references");
}

#[test]
fn functions_on_rows() {
    compare_files("functions_on_rows");
}

#[test]
fn functions_on_columns() {
    compare_files("functions_on_columns");
}

#[test]
fn functions_on_blocks() {
    compare_files("functions_on_blocks");
}

// #[test]
// fn shopping() {
//     compare_files("shopping");
// }
