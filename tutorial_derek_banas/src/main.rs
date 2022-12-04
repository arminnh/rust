#![allow(unused)]

// https://www.youtube.com/watch?v=ygL_xcavzQ4

use rand::Rng;
use std::borrow::Borrow;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader, ErrorKind, Write};
use std::ops::Add;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

mod restaurant;
use crate::restaurant::order_food;

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

fn hashmap() {
    let mut heroes = HashMap::new();
    heroes.insert("Superman", "Clark Kent");
    heroes.insert("Batman", "Bruce Wayne");
    heroes.insert("The Flash", "Barry Allen");
    println!("{}", heroes.len());

    for (k, v) in heroes.iter() {
        println!("{}: {}", k, v);
    }

    if heroes.contains_key("Batman") {
        match heroes.get("Batman") {
            Some(x) => println!("Batman is a hero"),
            None => println!("Batman is not a hero"),
        }
    }

    match heroes.get("NotBatman") {
        Some(x) => println!("NotBatman is a hero"),
        None => println!("NotBatman is not a hero"),
    }
}

fn structs() {
    struct Customer {
        name: String,
        address: String,
        balance: u32,
    }
    let mut bob = Customer {
        name: String::from("Bob Smith"),
        address: String::from("555 Main Street"),
        balance: 23450,
    };
    println!("{}", bob.address);
    bob.address = String::from("505 Main St");
    println!("{}", bob.address);
}

fn generic_struct() {
    struct GenericRectangle<T, U> {
        length: T,
        height: U,
    }
    let rec = GenericRectangle {
        length: 4,
        height: 10.5,
    };
    println!("{} {}", rec.length, rec.height);

    let rec = GenericRectangle {
        length: "abc",
        height: 'd',
    };
    println!("{} {}", rec.length, rec.height);
}

fn traits() {
    trait Shape {
        fn new(length: f32, width: f32) -> Self;
        fn area(&self) -> f32;
    }

    struct Rectangle {
        length: f32,
        width: f32,
    }

    struct Circle {
        length: f32,
        width: f32,
    }

    impl Shape for Rectangle {
        fn new(length: f32, width: f32) -> Rectangle {
            return Rectangle { length, width };
        }
        fn area(&self) -> f32 {
            return self.length * self.width;
        }
    }

    impl Shape for Circle {
        fn new(length: f32, width: f32) -> Circle {
            return Circle { length, width };
        }
        fn area(&self) -> f32 {
            return (self.length / 2.0).powf(2.0) * PI;
        }
    }

    let rec: Rectangle = Shape::new(10.0, 1.0);
    let circle: Circle = Shape::new(1.0, 10.0);
    println!("{}", rec.area());
    println!("{}", circle.area());
}

fn files() {
    let path: &str = "lines.txt";
    let output: Result<File, io::Error> = File::create(path);
    let mut output: File = match output {
        Ok(file) => file,
        Err(error) => panic!("Problem creating file: {:?}", error),
    };

    write!(output, "Just some \n random words").expect("Failed to write to file");

    let input: File = File::open(path).unwrap();
    let buffered: BufReader<File> = BufReader::new(input);
    buffered
        .lines()
        .for_each(|line: Result<String, io::Error>| println!("{}", line.unwrap()));

    let output2: Result<File, io::Error> = File::create("rand.txt");
    let output2: File = match output2 {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("rand.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Cannot create file: {:?}", error),
            },
            _other => panic!("Problem opening file: {:?}", error),
        },
    };
}

fn closure() {
    let can_vote = |age: i32| age >= 18;
    println!("Can vote: {}", can_vote(1));
    println!("Can vote: {}", can_vote(18));

    let mut i = 5;
    let print_var = |i: &i32| println!("i: {}", i);
    print_var(&i);
    i = 10;
    print_var(&i);
    let mut change_var = || i += 1;
    change_var();
    print_var(&i);

    let sum = |a, b| a + b;
    let prod = |a, b| a * b;
    println!("{}", use_function(3, 8, sum));
    println!("{}", use_function(3, 8, prod));
}

fn use_function<T>(a: i32, b: i32, function: T) -> i32
where
    T: Fn(i32, i32) -> i32,
{
    function(a, b)
}

fn smart_pointers() {
    let b_int1 = Box::new(10);
    println!("{}", b_int1);

    struct TreeNode<T> {
        pub left: Option<Box<TreeNode<T>>>,
        pub right: Option<Box<TreeNode<T>>>,
        pub key: T,
    }

    impl<T> TreeNode<T> {
        pub fn new(key: T) -> Self {
            TreeNode {
                left: None,
                right: None,
                key: key,
            }
        }
        pub fn left(mut self, node: TreeNode<T>) -> Self {
            self.left = Some(Box::new(node));
            self
        }
        pub fn right(mut self, node: TreeNode<T>) -> Self {
            self.right = Some(Box::new(node));
            self
        }
    }

    let root = TreeNode::new(1)
        .left(TreeNode::new(2))
        .right(TreeNode::new(3).right(TreeNode::new(99)));

    // root.left.as_ref().unwrap().left(TreeNode::new(1000));
    // --> cannot move out of a shared reference
    // move occurs because value has type `TreeNode<i32>`, which does not implement the `Copy` trait

    println!("{}", root.key);
    println!("{}", root.left.as_ref().unwrap().key);
    println!("{}", root.right.as_ref().unwrap().key);
    println!(
        "{}",
        root.right.as_ref().unwrap().right.as_ref().unwrap().key
    );

    // println!("{}", root.left.unwrap().right.unwrap().key);
    // -> thread 'main' panicked at 'called `Option::unwrap()` on a `None` value', src\main.rs:444:45
}

fn concurrency() {
    let thread1 = thread::spawn(|| {
        for i in 1..25 {
            println!("Spawned thread '{}'", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..20 {
        println!("Main thread: {}", i);
        thread::sleep(Duration::from_millis(1));
    }

    thread1.join().unwrap();
}

fn concurrency_bank() {
    pub struct Bank {
        balance: f32,
    }
    // fn withdraw(bank: &mut Bank, amount: f32) {
    //     bank.balance -= amount;
    // }
    // let mut bank = Bank { balance: 100.0 };
    // withdraw(&mut bank, 5.0);
    // println!("Balance: {}", bank.balance);

    // fn customer(bank: &mut Bank) {
    //     withdraw(bank, 5.0);
    // }

    // // Cannot have closure outlive current function. It borrows from it. -> smart pointer
    // thread::spawn(|| customer(&mut bank)).join().unwrap();
    // // -> may outlive borrowed value `bank`

    fn withdraw(bank: &Arc<Mutex<Bank>>, amount: f32) {
        let mut bank_ref = bank.lock().unwrap();

        if bank_ref.balance < amount {
            println!("No more money! {}", bank_ref.balance);
        } else {
            bank_ref.balance -= amount;
            println!("Remaining balance: {}", bank_ref.balance);
        }
    }

    fn customer(bank: Arc<Mutex<Bank>>) {
        withdraw(&bank, 6.0);
    }

    let bank: Arc<Mutex<Bank>> = Arc::new(Mutex::new(Bank { balance: 61.0 }));
    (0..15)
        .map(|_| {
            let bank_ref: Arc<Mutex<Bank>> = bank.clone();
            thread::spawn(|| customer(bank_ref))
        })
        .for_each(|handle: thread::JoinHandle<()>| {
            handle.join().unwrap();
        });
    println!("Total {}", bank.lock().unwrap().balance);
}

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
    // hashmap()
    // structs()
    // generic_struct()
    // traits()
    // restaurant::order_food()
    // files()
    // closure()
    // smart_pointers()
    // concurrency()
    concurrency_bank()
}
