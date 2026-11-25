use std::{assert_eq, assert_ne, panic};

fn add(left: u64, right: u64) -> u64 {
    left + right
}

fn add_two(a: u64) -> u64 {
    a + 2
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
#[derive(PartialEq, Debug)]
struct Person {
    age: u32,
}

struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {value}");
        }

        Guess { value }
    }
}
fn main() {
    // Checks whether something is true.
    assert!(5 > 2); // passes
    assert!(5 < 2); // fails

    // checks whether two values are equal
    assert_eq!(2 + 2, 4); // passes
    assert_eq!(2 + 2, 5); // fails

    // checks whether two values are different.
    assert_ne!(5, 3); // Passes
    assert_ne!(5, 5); // fails

    let result = String::from("Hello");
    assert!(
        result.contains("Carol"),
        "Greeting did not contain a name, value was `{result}`"
    )
}

#[cfg(test)]
mod tests {
    use std::{assert_eq, panic};

    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn another() {
        //     panic!("This test failed");
        // }
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 5,
        };

        let smaller = Rectangle {
            width: 5,
            height: 3,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 5,
        };

        let smaller = Rectangle {
            width: 5,
            height: 3,
        };
        assert!(!smaller.can_hold(&larger));
    }
    #[test]
    fn can_add_two() {
        let result = add_two(2);
        assert_eq!(result, 4);
    }

    #[test]
    // We can't combine a test that contains #[should_panic] with Result()
    // #[should_panic(expected = "less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(40);
    }

    #[test]
    // Our test can also return a result
    fn it_is_working() -> Result<(), String> {
        let result = add(2, 2);

        if result == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal to four"))
        }
    }

    #[test]
    fn one_humdred() {}

    #[test]
    fn add_two_and_two() {}

    #[test]
    fn add_three_and_two() {}

    #[test]
    // used if we want to ignore long test functions.
    #[ignore]
    fn expensive_test() {
        // Code that takes an hour to run
    }
}

// Command	What it does
// cargo test	Runs all non-ignored tests
// cargo test add	Runs tests whose names contain add
// cargo test one_hundred	Runs a matching test
// cargo test -- --test-threads=1	Runs tests with one thread
// cargo test -- --show-output	Shows output from successful tests
// cargo test -- --ignored	Runs only ignored tests
// cargo test -- --include-ignored	Runs ignored and non-ignored tests
// cargo test --test integration_test	Runs one integration test file
