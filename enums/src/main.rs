use std::println;

use crate::{
    Coin::Quarter,
    TrafficLight::{green, red, yellow},
};

//  Enum represents one value chosen from a set of possible variants.
enum TrafficLight {
    red,
    green,
    yellow,
}
enum IpAddr {
    V4(String),
    V6(String),
}
enum IpAdrr2 {
    V4(u8, u8, u8, u8),
    V6(String),
}
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

enum Coin {
    Penny,
    Quarter(String),
}
#[derive(Debug)]
enum Message2 {
    Write(String),
    Quit,
}

// enum Option<T> {
//     Some(T),
//     None,
// }
fn main() {
    let light = TrafficLight::red;
    // traffic light is the type, red, green and yellow are the variants.
    fn show_light(light: TrafficLight) {}
    // Enums can hold data
    // This is where enums become more powerful, A variant can contain information
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));
    let a = Message::Quit;
    let b = Message::Move { x: -10, y: 40 };
    let c = Message::Write(String::from("Hello Justin"));
    let d = Message::ChangeColor(99, 4, 16);

    let age = Some(25);
    let missing_age: Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    // let sum = x + y; This doesnt add because Option<i8> can either return an integer or None.
    let number = Some(7);
    match number {
        Some(value) => println!("There is a number"),
        None => println!("No number"),
    }

    fn middle_name() -> Option<String> {
        Some(String::from("Elochukwu"))
    }

    match middle_name() {
        Some(name) => println!("The middle name is {name}"),
        None => println!("There is no middle name"),
    }

    // Using if let
    // This is used if only we want to do something when the value is some.
    let config_max = Some(3u8);

    if let Some(max) = config_max {
        println!("The max number is {max}");
    } else {
        println!("No max number found");
    }

    match config_max {
        Some(max) => println!("The max value is {max}"),
        _ => (),
    }
    // This function only works with a quarter of a coin, if it receives another coin, it returns None immediately.
    fn quarter_state(coin: Coin) -> Option<String> {
        let Coin::Quarter(state) = coin else {
            return None;
        };

        Some(state)
    }

    let result = quarter_state(Coin::Quarter(String::from("Alaska")));

    println!("{result:?}");

    fn get_message() -> Option<Message2> {
        Some(Message2::Write(String::from("Hello, Rust")))
    }

    match get_message() {
        Some(Message2::Write(text)) => {
            println!("{text}");
        }
        Some(Message2::Quit) => {
            println!("Quitting");
        }
        None => {
            println!("No message available");
        }
    }
}
