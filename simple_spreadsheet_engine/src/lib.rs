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
    let mut sheet = Sheet::parse_input(input);
    print!("{}\n\n", sheet);
    sheet.resolve();
    let out = sheet.render();
    print!("{}\n\n", out);
    out
}
