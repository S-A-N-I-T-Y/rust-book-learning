use std::{
    fmt::{Debug, Display, Pointer},
    format,
    iter::Sum,
    println, string,
};

// We can use generics with structs.
struct Point<T> {
    x: T,
    y: T,
}
// Generics in methods
impl<T> Point<T> {
    // The T after impl declares the generic type parameter for this implementation and the T in Point<T> says were implementing methods for point using that type parameter,
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        self.x.powi(2) + self.y.powi(2).sqrt()
    }
}

struct Point2<T, U> {
    x: T,
    y: U,
}

struct Point3<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point3<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point3<X2, Y2>) -> Point3<X1, Y2> {
        Point3 {
            x: self.x,
            y: other.y,
        }
    }
}

// Generics are also automatically used in Enums Options and Result
// enum Option<T> {
//     Some(T),
//     None,
// }

// for Result
// enum Result<T, E> {
//     Ok(T), T - reps the success value type.
//     Err(E), E - reps the error value type.
// }

// Traits used to make different types to provide same behaviour.

struct NewsArticle {
    headline: String,
    author: String,
    location: String,
}

struct SocialPost {
    username: String,
    content: String,
}

// Both of the above struct have different contents, but supposed we want both of them to provide a similar summary, this is where Traits come into place.

// Now each of this types provides it's own version of the summarize() trait
pub trait Summary {
    fn summarize(&self) -> String; // This line says implement the summary trait for each struct
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

impl Summary for SocialPost {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub trait Summary2 {
    fn summarize_author(&self) -> String;

    fn summarize2(&self) -> String {
        String::from("(Read more...)") // we can decide to use our default method summary or provide our own behaviour
    }

    fn summarize3(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author()) // trait methods can call other trait methods
    }
}

// Using traits with Generics
// impl Trait as function parameter

// items: &impl Summary - this is a trait bound, it says the item passed in must implement the Summary trait.
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
// Using a trait bound syntax
// <T: Summary> - This is a trait bound syntax that says the generic type T must implement the Summary trait.
pub fn notify2<T: Summary>(item: &T) {
    println!("{}", item.summarize());
}

// When do we need either of the above trait bound syntax?
// When we have multiple parameters thatmust be of same type that implement the same trait, we can use the second trait bound syntax.

// Here item1 and item2 can be of different types but they must impl summary.
pub fn notify_first(item1: &impl Summary, item2: &impl Summary) {
    println!("{}", item1.summarize());
    println!("{}", item2.summarize())
}
// Here item1 and item2 must be of same type while implementing the summary trait.
pub fn notify_second<T: Summary>(item1: &T, item2: &impl Summary) {
    println!("{}", item1.summarize());
    println!("{}", item2.summarize());
}

// Multiple trait bounds
// We can use + to specify multiple trait bounds on a generic type parameter.
pub fn notify_multiple(item: &(impl Summary + Display)) {
    println!("{}", item.summarize());
    println!("{}", item)
}
// Using the trait bound syntax
pub fn notify_multiple2<T: Summary + Display>(item: &T) {
    println!("{}", item.summarize());
    println!("{}", item)
}
// Where Clause
// Used when functions have many genric parameters and trait bounds, their signatures can become difficult to read.
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
    0
}
// for the above function rust provides an alternative using "where"
fn some_function2<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    0
}
// the "where" clause simply moves the trait requirements below the function signature, making complex signatures easier to read.

// Returning a type that implements a trait
fn returns_summarize() -> impl Summary {
    SocialPost {
        username: String::from("rust_learner"),
        content: String::from("rust_docs"),
    }
}
// the function above returns some type that implements summary, but it must return only one concrete type

// This function does'nt work because the returned values aren't of the same types.
// fn returns_summarizable(switch: bool) -> impl Summary {
//     if switch {
//         // return NewsArticle
//     } else {
//         // Return a SocialPost
//     }
// }

// Conditional method implementations
// We can use trait bounds to conditionally implement methods for types that meet certain trait bounds.
struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

// Blanket implementations
// Blanket implementations are implementations of a trait on any type that implements another trait.
// impl<T: Display> ToString for T{

// }

// NB The Orphan Rule
// Rust restricts which traits you can implement for which types.
// You can implement a trait for a type if
// The trait is defiend in your crate, or
// The type is defined in your crate
// You can't implement a trait from another crate on a type from another crate.
// Eg:
// we can implement our own Summary trait for Vec!<T>
// we can also implement the standard library Display trait for our SocialPost struct type.
// But we can't implement the standard library Display trait for the standard library Vec<T> type as both are of different crates.
// This helps prevent crates from conflicting by implementing the same external trait for the same external type.

// Default trait implementation
// Used when we want a trait to provide a default behaviour

// Lifetimes Structs
struct ImportantExcerpt<'a> {
    part: &'a str,
}

// Lifetimes in method definitions.
// When a struct has a lifetime parameter, that lifetime can/must be used in the struct's method definitions.

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}
fn main() {
    // Generics - write code that works with different types.
    // Traits - define what each types can do, so generic code can require certain behaviour.
    // Lifetimes - tell the compiler how references relate to each other so it can verify they are valid
    // Why do we use generics
    // Imagine writing this two functions below

    fn largest_i32(list: &[i32]) -> &i32 {
        let mut largest = &list[0];

        for item in list {
            if item > largest {
                largest = item
            }
        }

        largest
    }

    fn largest_char(list: &[char]) -> &char {
        let mut largest = &list[0];

        for item in list {
            if item > largest {
                largest = item
            }
        }

        largest
    }

    // This above function bodies are typically the same, differences are only the types and names
    // Therefore using generic function

    fn largest<T: PartialOrd>(list: &[T]) -> &T {
        let mut largest = &list[0];

        for item in list {
            if item > largest {
                // Initially the > doesn't work because rust doesn't know if the type can be compared with > so therefore we use the PartialOrd trait bound in the generic type
                largest = item;
            }
        }

        largest
    }

    let integer = Point { x: 1, y: 2 };
    let floar = Point { x: 1.1, y: 1.2 };
    let mixed = Point2 { x: 2.2, y: 50 };

    let age = Some(23);
    let name = Some("justin");
    let nothing: Option<i32> = None;
    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());

    let p1 = Point3 { x: 5, y: 10.4 };
    let p2 = Point3 { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2); // here p3.x = p1.x and p3.y = p2.y

    let post = SocialPost {
        username: String::from("sanity"),
        content: String::from("Learning Rust type shii"),
    };

    notify(&post);
    println!("{}", post.summarize());

    // Lifetimes
    // This is a way to tell the compiler how long references are valid for, so it can prevent dangling references.

    let r; // 'a
    {
        let x = 5; // 'b
        // r = &x;
        r = 0
    }
    println!("r:{r}"); // This will throw an error because x is dropped before r is used, therefore r is a dangling reference.

    // fn longest(x: &str, y: &str) -> &str {
    //     if x.len() > y.len() { x } else { y }
    // }
    // This above function does'nt run

    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() { x } else { y }
    }

    let string1 = String::from("abcd");

    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is{result}");
    // Lifetimes describes relationships, not necessarily identical lifetimes.
    // If the returned reference could come from either input, both inputs need to be relared to the output lifetime.
    // If the returned reference comes only from x, only x needs to be related to the output lifetime.

    fn longest2<'a>(x: &'a str, y: &str) -> &'a str {
        x
    }

    // A function can return a reference to it's own local variable
    // fn longest3<'a>(x: &str, y: &str) -> &'a str {
    //     let result = String::from("Longest");
    //     // return &result.as_str();
    // }

    let novel = String::from("Rain Maker. By John Grisham...");
    let first_sentence = novel.split('.').next().unwrap();

    let i = ImportantExcerpt {
        part: { first_sentence },
    };

    // Lifetime Elision: when Rust fills in the annotation.
    fn first_word(s: &str) -> &str {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..1];
            }
        }
        &s[..]
    }

    // Rules for lifetime elision
    // Every reference parameter get its own lifetime.
    // If there is exactly one input lifetime, Rust assigns it to all output references.
    // For methods, if one input parameter is &self, or &mut self, Rust assigns the lifetime of self to output refrences.

    // Static lifetime
    // The 'static lifetime is the entire duration of the program.
    let s: &'static str = "I have a static lifetime.";
    println!("{s}");

    // Combining generics, traits and lifetimes.
    fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where
        T: Display,
    {
        println!("Announcement!: {ann}");

        if x.len() > y.len() { x } else { y }
    }
}
