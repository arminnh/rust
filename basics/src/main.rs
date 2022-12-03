#![allow(unused)]

fn main() {
    // Declare variable with type
    let x: i32 = 42;
    println!("{}", x);

    // Block expression
    {
        let x: &str = "text";
        println!("{}", x)
    }

    // Tuple
    let pair = ('a', 123);
    println!("{}", pair.0);
    println!("{}", pair.1);

    // Tuple with type definition
    let pair: (char, i32) = ('b', 345);

    // Destructuring
    let (some_char, some_int) = pair;
    assert_eq!(some_char, 'b');
    assert_eq!(some_int, 345);

    // Iterator
    let sum_of_squares = vec![1, 4, 5, 6, 0]
        .iter()
        .map(|x| x * x)
        .fold(0, |x, y| x + y);
    println!("sum_of_squares: {}", sum_of_squares);

    // Function
    fn greet(x: String) {
        println!("Hello! {}", x);
    }

    fn random_number() -> String {
        4.to_string()
    }

    greet("person".to_string());
    greet(random_number());

    // Value from block expression. Just like in R!
    println!("{}", {
        greet("block expression".to_string());
        if true {
            1234567890
        } else {
            1
        }
    });
    // String method
    let some_string = "This is a string!";
    println!("{}", some_string.len());

    // Namespaces. crate::file::function
    let least = std::cmp::min(1, -4);

    // Types are namespaces too
    println!("{}", str::len(some_string));

    // Type system
    struct Number {
        odd: bool,
        value: i32,
    }

    let even = Number {
        odd: false,
        value: 2,
    };
    let odd = Number {
        value: 3,
        odd: true,
    };
    // Variable bindings are immutable. This won't work:
    // even.odd = true; // error[E0594]: cannot assign to `even.odd`, as `even` is not declared as mutable

    // Pattern matching
    fn print_number(n: Number) {
        match n.value {
            1 => println!("One"),
            2 => println!("Two"),
            _ => println!("{}", n.value), // Matches have to be exhausted
        }
    }
    print_number(even);

    // Declare method on type
    impl Number {
        fn is_positive(&self) -> bool {
            return self.value > 0;
        }
    }
    println!("{}", odd.is_positive());

    // Make mutable
    let mut mutable_number = Number {
        odd: true,
        value: 17,
    };
    mutable_number.value = -2;
    println!("{}", mutable_number.value);

    // Generic functions
    fn foobar<T>(arg: T) {
        // ...
    }

    fn foobar2<L, R>(left: L, right: R) {
        // ...
    }

    // Structs can be generic too
    struct Pair<T> {
        a: T,
        b: T,
    }
    let p1 = Pair { a: 3, b: 9 }; // Pair<i32>
    let p2 = Pair { a: true, b: false }; // Pair<bool>

    // Vec (heap allocated array) is generic
    let mut v1 = Vec::new();
    v1.push(1); // Vec<i32>
    let mut v2 = Vec::new();
    v2.push(true); // Vec<bool>

    // Panic stops execution with info about error message & filename & line number
    // panic!("OMG!"); // thread 'main' panicked at 'OMG!', src\main.rs:137:1

    // Optional. Contains something or nothing.
    // Option is not a struct, but an enum with 2 variants: None or Some(T)
    // Result is also an enum, containing something or an error.
    let o1: Option<i32> = Some(12345);
    o1.unwrap(); // this is fine

    let o2: Option<i32> = None;
    // panic when unwrap is called on nothing
    // o2.unwrap(); // thread 'main' panicked at 'called `Option::unwrap()` on a `None` value', src\main.rs:139:8

    // Functions that can fail typically return a result instead of throwing exception
    println!("{:?}", std::str::from_utf8(&[240, 159, 141, 137])); // Ok("🍉")
    println!("{:?}", std::str::from_utf8(&[195, 40])); // Err(Utf8Error { valid_up_to: 0, error_len: Some(1) })

    // Iterator from 1 to 2^31. Lazy evaluation/computation.
    let natural_numbers = 1..;
    for i in natural_numbers {
        if i % 12345 == 0 {
            println!("{}", i);
        }
        if i / 12345 == 3 {
            break;
        }
    }
    // Ranges
    println!("{}", (0..).contains(&100));
    println!("{}", (..=20).contains(&20));
    println!("{}", (3..6).contains(&4));

    // iterate over vec
    for i in vec![4, 7, 9] {
        println!("Number {}", i);
    }
    // iterate over slice
    for i in &[52, 49, 129] {
        println!("Number {}", i);
    }
    // iterate over iterator
    // char type is a unicode scalar value that is always a valid character
    for c in "rust".chars() {
        println!("Char {}", c);
    }
    // iterate filtered and mapped
    for c in "SuRPRISE INbOUND"
        .chars()
        .filter(|c| c.is_lowercase())
        .map(|c| c.to_uppercase())
    {
        print!("{}", c);
    }
}
