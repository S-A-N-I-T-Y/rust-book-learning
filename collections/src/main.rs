use std::{collections::HashMap, format, println, result, vec};

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    // Vectors ( Vec<T> - store a list of values)
    // Strings ( String ) - store and manipulate UTF-8 text.
    // Hashmaps( <K, V> ) store key-value pairs

    // Vectors is like an array that can grow or shrink

    let scores = vec![70, 80, 55, 50, 18];
    let mut v: Vec<i32> = Vec::new();
    v = vec![1, 2, 3, 4, 5];

    let mut numbers = Vec::new();
    numbers.push(4);
    numbers.push(5);
    numbers.push(6);
    println!("{numbers:?}");

    let second_v = &v[1];
    println!("{second_v}");

    let third_v = &v.get(2);
    println!("{third_v:?}");

    // let a = &v[100];
    // println!("{a:?}"); // Panics // This causes a panic because index of 100 doesn't exists.

    let a = v.get(100);
    println!("{a:?}"); // None This returns none because v.get returns an Option<>

    match v.get(100) {
        Some(value) => {
            println!("The index is {value}")
        }
        None => println!("Index not found"),
    }

    let first = &v[0];
    // v.push(6); // This doesn't compile because first is an immutable reference to an element in the vector, then we try to modify it with push.
    // first could point to a memory that's no longer valid if this is done
    println!("{first}");

    // Iterating over a vector immutably.
    for i in &v {
        println!("{i}")
    }
    // Iterating over a vector mutably.
    for i in &mut v {
        *i += 50;
    }

    // This vector contains different type of data but every element has the same enum type: SpreadsheetCell
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(190.30),
    ];

    // When a vector goes out of scope all it's elements are dropped.
    {
        let v = vec![1, 2, 3];
    }
    // String Owned, growable, mutable string.
    // &str Borrowed string slice

    // name is a string slice
    let name = "Rust"; // Reference from an existing text.
    // name is an Owned String
    let name = String::from("Justin"); // An owned string that you can grow and modify.

    let s1 = String::new();
    let s2 = "Second".to_string();
    let s3 = String::from("Hello");

    // Updating strings
    let mut s = String::from("foo");
    s.push_str("bar");
    println!("{s}");

    let mut s1 = String::from("foo");
    let s2 = String::from("bar");

    s1.push_str(&s2); // Here push_str borrows the text from s2, it doesn't take Ownership of it.
    println!("{s2}");

    let mut s = String::from("lo");
    s.push('l');
    println!("{s}");

    let s1 = String::from("Hello, ");
    let s2 = String::from("World");

    let s3 = s1 + &s2; // Here s1 is moved, s3 takes the ownership if s1 while s2 is borrowed immutably.
    println!("{s3}");
    // The + operator uses an add method that takes ownership of it's first argument and borrows it's second argument.

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let result = format!("{s1}-{s2}-{s3}"); // format doesn't take ownership of the strings here.
    println!("{result}");

    let s = String::from("Hola"); // Each of this letter use one byte so the whole string uses four bytes.

    // let letter = s[0]; // This doesn't work because strings are UTF-encoded, and npot every character uses the same number of bytes.

    let hello = String::from("Здравствуйте");
    let s = &hello[0..4];
    println!("{s}");

    // Iterating and printing out the characters.
    for c in "Зд".chars() {
        println!("{c}");
    }
    // Iterating and printing out in bytes
    for b in "Зд".bytes() {
        println!("{b}");
    }
    // .chars() gives a unicode scalar values.
    // .bytes() gives raw bytes.

    // Hashmaps stores information as keys and their associated values.
    // HashMap<K, V> where K = key and V = value.

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 20);

    // Getting a value from an hashmap.
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    println!("{score}");
    // score.get(&team_name) for the key team_name and returns an Option<&i32>
    // copied() converts the &Option<i32> and returns Option<i32> by copying the integer.
    // unwrap_or(0) returns the integer value if found or 0 if not.

    // Iterating over a HashMap
    for (key, value) in scores {
        println!("{key}:{value}"); // order in which they are printed is arbitrary.
    }

    // Ownerships in hashmaps
    let field_name = String::from("Favorite Color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new(); // map here takes ownership of both field_name and field_value
    map.insert(field_name, field_value);

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 20);
    scores.insert(String::from("Blue"), 30); // Scores value is updated to 30 here.

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    // .or_insert is saying if this key doesn't have a value, insert this value, else leave the existing value.

    // Updating a Value based on it's previous value.
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        // If word already exists get a mutable reference to its current count.
        // If it doesn't exist, insert 0 and get a mutable reference to that new one.

        *count += 1; // *count dereferences the mutable reference to increase the actual count.
    }

    println!("{map:?}");
}
