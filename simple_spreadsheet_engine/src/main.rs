mod cell;
mod cell_pos;
mod cell_range;
mod expression;
mod formula;
mod function;
mod number_or_cell_pos;
mod sheet;

use crate::sheet::Sheet;

fn run(input: &str) -> &str {
    println!("{}\n", input);
    let sheet = Sheet::parse_input(input);
    print!("{}\n\n", sheet);

    ""
}

fn main() {
    println!("Hello, world!");
}
