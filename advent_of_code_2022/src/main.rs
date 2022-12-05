use std::fs;
// use std::fs::File;
// use std::io::{BufRead, BufReader};

fn main() {
    // let file: File = File::open("inputs/day_1/input").unwrap();
    // BufReader::new(file)
    //     .split(b'\n')
    //     .for_each(|elf| println!("elf: {:?}", elf.unwrap()));
    // .for_each(|line: Result<String, io::Error>| println!("{}", line.unwrap()));

    let contents = fs::read_to_string("inputs/day_1/input");
    match contents {
        Ok(c) => {
            let elves: std::str::Split<&str> = c.split("\n\n");
            let calories_per_elf = elves.map(|elf: &str| {
                elf.split_ascii_whitespace().fold(0, |acc, calories_str| {
                    acc + calories_str.parse::<u32>().unwrap()
                })
            });
            println!("{:?}", calories_per_elf);
            println!("{:?}", calories_per_elf.max());
        }
        Err(err) => println!("Error reading file: {}", err),
    }
}
