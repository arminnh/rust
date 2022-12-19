mod cell;
mod cell_pos;
mod cell_range;
mod expression;
mod formula;
mod function;
mod number_or_cell_pos;
mod sheet;

use crate::sheet::Sheet;

pub fn run(input: String) -> String {
    let sheet = Sheet::parse_input(input);
    // print!("{}\n\n", sheet);
    let processed = sheet.process();
    let out = processed.to_string();
    print!("{}\n\n", out);
    out
}
