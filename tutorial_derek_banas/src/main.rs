#![allow(unused)]

// https://www.youtube.com/watch?v=ygL_xcavzQ4

use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io::{self, BufRead, BufReader, ErrorKind, Write};
use std::ops::Add; // allows us to perform addition with generics

fn greet_user() {
    println!("Name?");
    let mut name: String = String::new();
    let greeting: &str = "Nice to meet you!";
    io::stdin()
        .read_line(&mut name)
        .expect("Didn't recieve input");

    println!("Hello {}! {}", name.trim_end(), greeting);
}

fn age() {
    let age: &str = "47";

    // shadowing with different datatype perfectly fine to do?
    let mut age: u32 = age.parse().expect("Age wasn't a number");

    age += 1;
    println!("Age: {}", age);
}

const MILLION: i32 = 1_000_000;
const PI: f32 = 3.1415;

fn max_sizes() {
    println!("Max u8: {}", u8::MAX);
    println!("Max u16: {}", u16::MAX);
    println!("Max u32: {}", u32::MAX);
    println!("Max u64: {}", u64::MAX);
    println!("Max usize: {}", usize::MAX);
    println!("Max u128: {}", u128::MAX);
    println!("Max f32: {}", f32::MAX);
    println!("Max f64: {}", f64::MAX);
}

fn precision() {
    let num1: f32 = 1.111111111111111;
    println!("f32: {}", num1 + 0.111111111111111);
    let num2: f64 = 1.111111111111111;
    println!("64: {}", num2 + 0.111111111111111);
}

fn random() {
    let random_num = rand::thread_rng().gen_range(1..=100);
    println!("{}", random_num);
}

fn kind_of_ternary() {
    let mut age = 57;
    let can_vote = if age >= 18 { true } else { false };
    println!("Can vote: {}", can_vote);
}

fn matching_ranges(age: i32) {
    match age {
        1..=18 => println!("Important birthday!"),
        21 | 50 => println!("Important birthday!"),
        65.. => println!("Important birthday!"),
        _ => println!("Not so important."),
    }
}

fn matching_ordering() {
    let age = 18;
    let voting_age = 18;
    match age.cmp(&voting_age) {
        Ordering::Less => println!("Cannot vote"),
        Ordering::Equal => println!("Can vote"),
        Ordering::Greater => println!("Can vote"),
    }
}

fn arrays() {
    let array = [1, 2, 3, 4, 7, 8, 5, 32, 5];
    println!("{}", array[0]);
    println!("{}", array.len());

    // loop index
    let mut i = 0;
    loop {
        if array[i] % 2 == 0 {
            i += 1;
            continue;
        }
        if array[i] == 5 {
            break;
        }
        println!("Val: {}", array[i]);
        i += 1;
    }
    println!("");

    // while
    i = 0;
    while i < array.len() {
        println!("Val: {}", array[i]);
        i += 1;
        if array[i] == 4 {
            break;
        }
    }
    println!("");

    for v in array.iter() {
        println!("Val: {}", v)
    }
}

fn tuples() {
    let tup: (u8, String, f64) = (47, "Abcoehogea".to_string(), 50_000.01);
    println!("Name: {}", tup.1);
    let (age, _, _) = tup;
    println!("Age: {}", age);
}

fn strings() {
    let mut s1: String = String::new(); // growable string
    s1.push('A');
    s1.push_str("this is a string with words");

    for word in s1.split_whitespace() {
        println!("{}", word)
    }

    let s2 = s1.replace("A", "ABCDEF. ");
    println!("{}", s2);

    let s3 = String::from("random stuff here i  y h le h l i y i");
    let mut v: Vec<char> = s3.chars().collect();
    v.sort();
    v.dedup();
    v.into_iter().for_each(|c: char| print!("{}", c));
    println!("");

    let s4: &str = "Random string";
    let mut s5: String = s4.to_string();
    println!("{}, {}", s4, s5);

    let bytes = s5.as_bytes();
    let s6 = &s5[0..6];
    println!("Str length: {}", s6.len());
    s5.clear();
    println!("s5: {}", s5);

    let s6 = String::from("Part 1");
    let s7 = String::from("Part 2");
    let s8 = s6 + &s7; // s6 gets moved -> no longer available after this
    println!("{}, {}", s7, s8);

    for c in s8.bytes() {
        println!("{}", c)
    }
}

fn casting() {
    let i1: u8 = 5;
    let i2: u8 = 4;
    let i3: u32 = (i1 as u32) + (i2 as u32);
    println!("{}", i3);
}

fn enums() {
    enum Day {
        Ma,
        Di,
        Wo,
        Do,
        Vr,
        Za,
        Zo,
    }

    impl Day {
        fn is_weekend(&self) -> bool {
            match self {
                Day::Za | Day::Zo => true,
                _ => false,
            }
        }
    }

    let today: Day = Day::Za;
    match today {
        Day::Ma => println!("Oh no it's a Monday!"),
        _ => println!("It's not Monday my dudes!"),
    }

    println!("Weekend? -> {}", today.is_weekend())
}

fn vectors() {
    // Vectors can grow if mutable and can contain only 1 type
    let v1: Vec<i32> = Vec::new();
    let mut v2 = vec![1, 2, 3, 4];
    v2.push(5);
    println!("1st: {}", v2[0]);

    let second: &i32 = &v2[1];
    match v2.get(1) {
        Some(second) => println!("2nd: {}", second),
        None => println!("No 2nd value"),
    }

    for i in &mut v2 {
        *i = *i * *i;
    }
    for i in &v2 {
        println!("{}", i);
    }

    println!("Vec len {}", v2.len());
    println!("Pop: {:?}", v2.pop());
    println!("Vec len {}", v2.len());
}

fn return_tuple<T>(x: &T) -> (&T, &T) {
    return (x, x);
}

fn list_argument(list: Vec<i32>) -> i32 {
    list.iter().fold(0, |x, y| x + y)
}

fn list_argument_by_ref(list: &[i32]) -> i32 {
    let mut sum = 0;
    for &v in list.iter() {
        sum += v;
    }
    return sum;
}

fn generic_sum<T: Add<Output = T>>(x: T, y: T) -> T {
    return x + y;
}

// Ownership. Stack & Heap.
// 1. Each value has a variable which is its owner.
// 2. There is only 1 owner at a time.
// 3. When the owner goes out of scope, the value is deleted from memory.

fn main() {
    // println!("{}, {}", MILLION, PI)
    // greet_user()
    // age()
    // max_sizes()
    // precision()
    // random()
    // kind_of_ternary()
    // for i in 1..70 { print!("{}: ", i); matching_ranges(i) }
    // matching_ordering()
    // arrays()
    // tuples()
    // strings()
    // casting()
    // enums()
    // vectors()
    // let x: (&i32, &i32) = return_tuple(&1);
    // let y: (&char, &char) = return_tuple(&'a');
    // println!("{}, {}", x.1, y.0);
    // println!("{}", list_argument(vec![1, 2, 3, 100]));
    // println!("{}", list_argument_by_ref(&vec![1, 2, 3, 100]));
    // println!("{}", generic_sum(10, 20));
    // println!("{}", generic_sum(9.9, 13.15));
}
