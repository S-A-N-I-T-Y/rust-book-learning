use std::{dbg, println};

fn main() {
    // Structs is a custom type that groups related values into named fields.

    let user = (true, String::from("user"), 3);
    // println!("{user[0]}") cant' access the user individual field.

    // Struct User defines the name of the shape of the type.
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_out: u64,
    }
    // Creates an instance of the already defined struct.
    let mut user = User {
        active: true,
        username: "sanity".to_string(),
        email: String::from("justinokpara19@gmail.com"),
        sign_in_out: 5,
    };

    user.username = String::from("npmsanity");

    fn build_user(email: String, username: String) -> User {
        User {
            active: true,
            username,
            email,
            sign_in_out: 3,
        }
    }

    let user2 = User {
        email: String::from("iamjustinokpara10@gmail.com"),
        ..user
    };
    // special structs

    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("{}", black.0);
    // destructure the tuple struct
    let Point(x, y, z) = origin;

    // A unit-like struct has no fields;
    struct AlwaysEqual;
    let subject = AlwaysEqual;

    // The struct here owns it's text, so this text can remain valid for as long as the struct is valid.
    // String: Owned string data.
    // &str: a borrowed string slice.
    struct UserProfile {
        username: String,
        email: String,
    }

    let width = 30;
    let height = 50;

    fn area(width: u32, height: u32) -> u32 {
        width * height
    }

    // Using a tuple for this

    let rect = (30, 50);
    fn area_of_rect(dimensions: (u32, u32)) -> u32 {
        dimensions.0 * dimensions.1
    }

    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    let rect = Rectangle {
        width: 35,
        height: 45,
    };

    fn area_of_rectangle(rectangle: &Rectangle) -> u32 {
        rectangle.height * rectangle.height
    }

    println!("{rect:#?}");
    println!("{rect:?}");

    let scale = 2;
    let width = dbg!(30 * scale);

    println!("{width}");

    dbg!(&rect);

    // Quick memory trick:
    // - {} → Display
    // - {:?} → Debug
    // - {:#?} → pretty Debug
    // - dbg!(...) → print a debugging expression and return its value

    struct Square {
        length: u32,
         breath: u32,
    }

    impl Square {
        fn area(&self) -> u32 {
            self.length * self.breath
        }
        fn double_breath(&mut self) {
            self.breath *= 2
        }
    }

    let sqr = Square {
        length: 10,
        breath: 20,
    };

    println!("{}", sqr.area());

    // - impl Rectangle begins an implementation block for Rectangle.
    // - fn area(...) defines a function associated with that type.
    // - &self means the method borrows the instance so it can read its fields without taking ownership.
    // - rect.area() calls the method on the instance.

    // What are &self and &Self
    // self refers to the particular instance the method is working with.
    // Self refers to the type being implemented - in this case square or rectangle.

    // Methods with additional parameters
    impl Rectangle {
        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }
    }

    let rect1 = Rectangle { width: 30, height: 40 };
    let rect2 = Rectangle{ width: 20, height: 25};

    println!("{}", rect1.can_hold(&rect2));
// Associated functions, functions that don't need an instance
impl Rectangle{
    fn square(size: u32) -> Self{
        Self { width: size, height: size }
    }
}
}
