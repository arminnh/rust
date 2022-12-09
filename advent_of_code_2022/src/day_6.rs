use std::{collections::HashSet, fs, hash::Hash};

fn no_duplicates<T>(iter: T) -> bool
where
    T: IntoIterator,
    T::Item: Eq + Hash,
{
    let mut unique = HashSet::new();
    // HashShet.insert returns whether the value was newly inserted.
    iter.into_iter().all(move |x| unique.insert(x))
}

/// How many characters need to be processed before the first start-of-packet marker is detected?
fn main() {
    if let Ok(contents) = fs::read_to_string("inputs/day_6/input") {
        contents.lines().for_each(|line| {
            if line.len() > 0 {
                for chars in line
                    .char_indices()
                    .into_iter()
                    .collect::<Vec<(usize, char)>>()
                    .windows(4)
                {
                    if no_duplicates(chars.iter().map(|(_, c)| c)) {
                        println!("{:?}", chars);
                        println!("Nr of characters to be processed: {}", chars[3].0 + 1);
                        return;
                    }
                }
            }
        });
    }
}
