use rand::prelude::*;
use std::cmp::Ordering;
use std::{io, println, string};

fn main() {
    print_measurement(5, 'L');
    print_number(5);
    another_function();
    println!("Guess the number!");
    let secret_number = rand::rng().random_range(1..=100);

    loop {
        println!("Please input your guess");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Correct, You win!");
                break;
            }
        }
    }

    let mut x = 5;
    x = 75;
    let x = x + 5;
    println!("The value of x is : {x}");

    let word = "hello";
    let word = word.len();

    let m = 11;
    {
        let m = m + 2;
        println!("{m}");
    }

    println!("{m}");

    let guess: u32 = "42".parse().expect("Not a number");

    // Intergral literals
    let decimal = 98_222;
    let hex = 0xfff;
    let octal = 0o77;
    let binary = 0b11111_000;
    let byte = b'A';

    let is_ready: bool = true;
    let letter = 'z';
    let symbol = 'Z';
    let emoji = '👌';

    let character = 'A';
    let text = "A";

    // Compund Types
    // Compund types group multiple values
    // Tuple: A fixed group of values that may have different types
    let person = ("ada", 35, true);
    let (name, age, active) = person;

    let point = (10, 20);
    let x_rect = point.0;
    let y_rect = point.1;

    let numbers = [10, 20, 45, 30, 50];

    let first_number = numbers[0];
    let second_number = numbers[1];
    let repeated = [3; 5];
    // Array type syntax is [element_type;, length]

    // Statements vs Expressions
    // A statement performs an action but does not produce a value
    // An expression evaluates to a value

    // Conditional Statements

    let number = 3;
    if number < 5 {
        println!("Number is too small");
    } else {
        println!("Number is atleast 5");
    }

    let number = 6;

    if number % 4 == 0 {
        println!("Divisible by 4");
    } else if number % 3 == 0 {
        println!("Divisible by 3");
    } else if number % 2 == 0 {
        println!("Divisible by 2");
    } else {
        println!("Not divisible by 4, 3, or 2");
    }

    let mut number = 3;

    while number != 0 {
        println!("{number}");
        number -= 1
    }

    let numbers = [10, 20, 30];

    for number in numbers {
        println!("Number is {number}")
    }

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("{counter}");

    // Ownership Learning.
    {
        let s = String::from("hello");
        println!("{s}");

        let x = 5;
        let y = x;
        println!("{x}, {y}");
        // This works because i32 is a simple fixed size value that implements the copy traits.

        let s1 = String::from("hello");
        let s2 = s1;
        println!("{s2}");
        // println!("{s1}"); // Error because ownership is already moved to s2
    }

    let s = String::from("Hello");
    takes_ownership(s);

    let u = 5;
    makes_copy(u);

    let s1 = gives_ownership();
    println!("{s1}");

    let s2 = String::from("hello");
    let len = calculate_length(&s2);
    println!("The length of {s2} is {len}");

    let mut s = String::from("hello");
    change(&mut s);
    println!("{s}");

    // Two simultaenous mutable references are not allowed.

    let mut s = String::from("Value");
    let r1 = &mut s;
    let r2 = &mut s; // This would lead to an error

    // We can't combine active immutable references with a mutable reference to the same value
    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &mut s; // Error because r1 is still in use or scope.

    // A ref does'nt necesarily remain active until the close, it's scope for borrowing purposes can end after it's use.

    let mut s = String::from("Ken Carson");
    let r1 = &s;
    let r2 = &s;

    println!("{r1}, {r2}"); // After this r1 and r2 are done.
    let r3 = &mut s; //This works because the ref r1 and r2 are done and moved out of scope
    r3.push_str(" ft destroy lonely");
    println!("{r3}");

    // Slices
    let s = String::from("Hello World");
    let hello = &s[0..5]; //This let's us refer to a part of this data without owning it. 
    let world = &s[6..11]; //This let's us refer to a part of this data without owning it.

    let my_string = String::from("hello People");
    let literal = "hello world";
    let word = first_word(&my_string);
    println!("{word}");

    fn first_word(s: &str) -> &str {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[..i];
            }
        }
        &s[..]
    }
}

// Chapter 4 cheat sheet
// Keep this nearby while you practice.
// Syntax	Meaning
// let s = String::from("hi");	s owns a String
// let s2 = s1;	Moves a String from s1 to s2
// let s2 = s1.clone();	Clones the string data
// let y = x; for i32	Copies the integer
// &s	Creates an immutable reference
// &mut s	Creates a mutable reference
// &s[0..3]	Borrows a slice of a string
// &str	A string slice
// &[i32]	A slice of integers
// drop	Cleans up an owned value when its scope ends

fn takes_ownership(text: String) {
    println!("{text}");
}
fn makes_copy(num: i32) {
    println!("{num}");
}
fn gives_ownership() -> String {
    let s = String::from("Hello");
    return s;
}
fn calculate_length(s: &String) -> usize {
    return s.len();
}
fn change(text: &mut String) {
    // text.push_str(", World"); This can't work because were borrowing and immutable reference
    text.push_str(", World");
}
// Rust prevents dangling ref

// fn dangle() -> &String {
//     let s = String::from("Dangle");
//     // &s this can't work because it's gonna be a dangling reference as once s is out of scope the ref is also out of scope
// }
fn no_dangle() -> String {
    let s = String::from("Hello");
    return s; // Here, ownership of s can be moved to whatever call this function
}
fn another_function() {
    println!("This is another function printing stuff");
}

fn print_number(x: i32) {
    println!("The number is {x}");
}
fn print_measurement(value: i32, unit: char) {
    println!("Measurement: {value}{unit}")
}
