use core::{error, panic};
use std::{
    error::Error,
    fs::{self, File},
    io::{self, ErrorKind, Read},
    net::IpAddr,
};

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must between 1 and 100, got {value}")
        }
        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

// This is the Result type returns either an Ok(T) of value or Err(T) of error.
// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

// By default main() returns nothing, but we can change the return type to Result<(), Box<dyn Error>> to allow us to use the ? operator in main() and return an error if one occurs.
// Result<(), Box<dyn Error>> means that the function will return Ok(()) if it runs successfully.
// Box<dyn Error> is a trait object that allows us to return any type of error that implements the Error trait. This is useful because it allows us to return different types of errors from different parts of our code without having to specify the exact type of error in the function signature.
// Ok(()) is the unit type, which is used when there is no meaningful value to return. In this case, it indicates that the function has completed successfully without any errors.
fn main() -> Result<(), Box<dyn Error>> {
    // Rust has two types of error handling
    // 1 Recoverable: Something is wrong but the program can still be able to continue - Result<T, E>
    // Unrecoverable: program can't continue  - panics!

    // panic!("Crash and burn"); // Used when a programs encounters a problem it cannot reasonably continue from.

    let home: IpAddr = "127.0.0.1"
        .parse()
        .expect("Hardcoded IP Address should be valuediu");
    // parse() here might fail, therefore it returns a Result, expect() tells rust if this operation fails, panic and display this explanation.

    // let file = File::open("hello.txt").unwrap();
    // let file = File::open("hello.txt").expect("hello.txt should be included in this project");
    // Both of the lines of code above panics if the file fails to open, but expect is better as it provides a more meaningful panic message.

    let guess = Guess::new(50); // returns guess value 50
    let guess = Guess::new(100); // panics because guess is greater than 100

    // Here File::open() might succeed or fail
    let greeting_file_result = File::open("hello.text");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        // Err(error) => panic!("Problem opening file: {error:?}"),
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },

            _ => {
                panic!("Problem opening the file: {error:?}")
            }
        },
    };

    // Here unwrap_or_else let's us specify what should happen when a Result contains an error.
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {error:?}");
            })
        } else {
            panic!("Problem opening the file: {error:?}")
        }
    });
    // unwrap() is a shortcut for handling a Result wehn you want to panic if it contains an error.
    let greeting_file = File::open("hello.txt").unwrap(); // if Ok, return the value inside it, if Err, panic

    // Expect behaves just like unwrap() but let's us choose a panic message.
    let greeting_file =
        File::open("hello.txt").expect("hello.txt should be included in this project");
    // Neither unwrap() or expect() recovers from an error, they both panic.

    // Propagating Errors: This is returning an error to the calling code if something goes wrong.

    // fn read_username_from_file() -> Result<String, io::Error> {
    //     let username_file_result = File::open("hello.txt");

    //     let mut username_file = match username_file_result {
    //         Ok(file) => file,
    //         Err(e) => return Err(e),
    //     };

    //     let mut username = String::new();

    //     match username_file.read_to_string(&mut username) {
    //         Ok(_) => Ok(username),
    //         Err(e) => Err(e),
    //     }
    // }

    // The ? Operator
    // let mut username_file = match File::open("hello.txt") {
    //     Ok(file) => file,
    //     Err(e) => return Err(e),
    // };

    let mut username_file = File::open("hello.txt")?; // If result is Ok(value), extract the value and continue, Err(error), return the error from the current situation.

    fn read_username_from_file() -> Result<String, io::Error> {
        let username_file_result = File::open("hello.txt");

        let mut username_file = match username_file_result {
            Ok(file) => file,
            Err(e) => return Err(e),
        };

        let mut username = String::new();

        match username_file.read_to_string(&mut username) {
            Ok(_) => Ok(username),
            Err(e) => Err(e),
        }
    }

    // This is a much cleaner version of the propagating error above using the ? operator
    {
        fn read_username_from_file() -> Result<String, io::Error> {
            let mut username_file = File::open("hello.txt")?;
            let mut username = String::new();

            username_file.read_to_string(&mut username)?;

            Ok(username)
        }
    }

    // NB The ? operator does not panic, it returns the error from the current function.
    // Only use the ? operator in a function whose return type is compatible with the error or absence being propagated.

    // Chaining in Error Propagation
    {
        fn read_username_from_file() -> Result<String, io::Error> {
            let mut username = String::new();

            File::open("hello.txt")?.read_to_string(&mut username)?;
            Ok(username)
        }
    }

    // Shorter function for reading a file into a string
    {
        fn read_username_from_file() -> Result<String, io::Error> {
            fs::read_to_string("hello.txt") // Opens the file, reads the contents into a string and then returns it.
        }
    }

    // The ? also works with Option
    {
        fn last_char_of_first_line(text: &str) -> Option<char> {
            text.lines().next()?.chars().last()
        }

        // text.lines() gets an iterator over the lines of the string
        // next() gets the first line,
        // and then chars() gets an iterator over the characters in that line,
        // and last() gets the last character.
        // If any of these operations return None, the function will return None.
    }

    // ? on a Result will propagate an error
    // ? on an Option will will propagate None

    Ok(())
}
