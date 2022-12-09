use std::collections::HashSet;
use std::fs;
use std::hash::Hash;

fn get_duplicates<T>(left: T, right: T) -> Vec<T::Item>
where
    T: IntoIterator,
    T::Item: Eq + Hash,
{
    let mut set = HashSet::new();
    left.into_iter().for_each(|x| {
        set.insert(x);
    });

    // let mut out: Vec<T::Item> = right.into_iter().filter(|x| set.contains(x)).collect();
    // out.dedup();
    // out
    right.into_iter().fold(Vec::new(), |mut acc, c| {
        if set.contains(&c) && !acc.contains(&c) {
            acc.push(c);
        }
        acc
    })
}

/// Lowercase item types a through z have priorities 1 through 26.
/// Uppercase item types A through Z have priorities 27 through 52.
fn get_priority(c: &char) -> u8 {
    if c.is_lowercase() {
        (*c as u8) - 'a' as u8 + 1
    } else {
        (*c as u8) - 'A' as u8 + 1 + 26
    }
}

/// The Elves have made a list of all of the items currently in each rucksack
/// (your puzzle input), but they need your help finding the errors. Every item
/// type is identified by a single lowercase or uppercase letter (that is,
/// a and A refer to different types of items).
/// Find the item type that appears in both compartments of each rucksack.
/// What is the sum of the priorities of those item types?
fn main() {
    if let Ok(contents) = fs::read_to_string("inputs/day_3") {
        let mut total: u32 = 0;
        contents.lines().for_each(|rucksack| {
            let nr_of_items = rucksack.len();
            if nr_of_items > 0 {
                let (left, right) = rucksack.split_at(nr_of_items / 2);
                let dups = get_duplicates(left.chars(), right.chars());
                let priority = dups.iter().fold(0, |acc, c| acc + get_priority(c));
                println!(
                    "{:?}, {:?}, {:?}, {:?}, {:?}",
                    rucksack, left, right, dups, priority
                );
                total += priority as u32;
            }
        });
        println!("{}", total);
    }
}
