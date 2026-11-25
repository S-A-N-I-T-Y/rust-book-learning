use modules::hosting;
use modules::deliver_order;

// For structs, enums, and other items, usuallly import the items itself.
use std::collections::HashMap;
use std::fmt::Result;
use std::io::Result as IoResult;
// Rust let's us group items that share a path prefix, We group them into nested paths.
// use std::{cmp::Ordering, io};
// If we need to use both std::io and std::io::Write, we can use self;
use std::io::{self, Write}; // This imports io itself and Write from inside io. 

// We can import all public items from a module with *
use std::collections::*;

fn fix_incorrect_order() {
    // super:: let's us access the the parent module, then find deliver_order there.
    deliver_order();
}

// Making a struct public doesn't automatically mean all it's child variants are public.
pub struct Breakfast {
    pub toast: String,
    seasonal_fruit: String,
}

impl Breakfast {
    pub fn summer(toast: &str) -> Breakfast {
        Breakfast {
            toast: String::from(toast),
            seasonal_fruit: String::from("peaches"),
        }
    }
}
// Making an enum public exposes all it's variant to be public
pub enum Appetizer {
    Soup,
    Salad,
}

fn main() {
    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
    }

    let mut meal = Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");

    let mut map = HashMap::new();
    map.insert(1, 2);
}
// Package: This is a bundle managed by cargo. It contains a Cargo.toml file and at least on crate.
// Crate: A crate is a tree of modules that produces a library or executable. A crate can be a binary crate or a library crate.
// Module: A module is a collection of items such as functions, structs, enums, traits, and other modules. Modules let you organize code within a crate into groups for readability and easy reuse.
// A binary crate builds an executable program, it has the main function in it.
// A library crate does not have a main function and is intended to be used as a dependency in other projects.
// pub mod hosting makes hosting module accessible from it's ancestor modules
// pub fn add_to_wailist makes the function accessible through that module
// Making a module public does'nt automatically make everything inside it public
// Child modules can access private items inside the ancestors, but parent modules can not access private items in their child modules.
