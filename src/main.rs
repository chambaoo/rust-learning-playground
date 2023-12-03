use std::vec;

fn main() {
    // --------------------------------
    // Variables and Mutability
    // --------------------------------
    let message = "hello, rust";
    println!("{}", message);

    let mut message = "before";
    println!("{}", message);
    message = "after";
    println!("{}", message);

    // Constants
    // the type of the value must be annotated
    const URL: &str = "https://example.com/";
    println!("{}", URL);

    // Shadowing
    let message = "shadowing";
    println!("{}", message);

    // --------------------------------
    // Data Types
    // --------------------------------

    // integer
    let a: i8 = 99;
    println!("{}", a);
    // compile error!
    // let a: i8 = 9999;

    // bool
    let a = true;
    let b: bool = false;
    println!("{}", a);
    println!("{}", b);

    // char
    let a = 'a';
    let b: char = 'b';
    println!("{}", a);
    println!("{}", b);

    // tuple
    let a = ("guitar", 38);
    println!("{}", a.0);
    println!("{}", a.1);
    // compile error!
    // println!("{}", a.2);

    // array
    let array = ["salt", "sugar", "syrup"];
    println!("{} {} {}", array[0], array[1], array[2]);
    // compile error!
    // println!("{}", array[3]);

    // vector
    let mut v: Vec<i32> = Vec::new();
    v.push(99);
    println!("{:?}", v);

    let v = vec![100, 200, 300];
    println!("{:?}", v);

    // string
    let proverb = "Rome wasn't built in a day.";
    println!("{}", proverb);

    let proverb = String::from("Don't put all your eggs in one basket.");
    println!("{}", proverb);

    let proverb = "Out of sight, out of mind.";
    let proverb_string = proverb.to_string();
    println!("{}", proverb_string);

    let mut builder = String::from("Don't judge a book");
    builder.push_str(" by its cover.");
    println!("{:?}", builder);

    // hashmap
    let mut releases = std::collections::HashMap::new();
    releases.insert("Wonderwall", 1995);
    releases.insert("Don't Look Back in Anger", 1996);
    releases.entry("Don't Look Back in Anger").or_insert(1996);
    releases.entry("Live Forever").or_insert(1994);
    println!("{:?}", releases);

    let atomic_numbers = std::collections::HashMap::from([
        (1, "H"),
        (2, "He"),
        (3, "Li"),
        (4, "Be"),
        (5, "B"),
        (6, "C"),
        (7, "N"),
        (8, "O"),
        (9, "F"),
        (10, "Ne"),
    ]);
    println!("{:?}", atomic_numbers);

    // --------------------------------
    // Functions
    // --------------------------------

    let a = void();
    println!("void() returns: {:?}", a);
    let a = return_void();
    println!("return_void() returns: {:?}", a);

    println!("{}", add(52351, 2435));

    // --------------------------------
    // enum
    // --------------------------------
    #[derive(Debug)]
    enum Color {
        Red,
        Blue,
        Green,
        Hex(String),
    }
    let red = Color::Red;
    let blue = Color::Blue;
    let green = Color::Green;
    let hex = Color::Hex("ffffff".to_string());

    println!("{:?}", red);
    println!("{:?}", blue);
    println!("{:?}", green);
    println!("{:?}", hex);

    // --------------------------------
    // Unrecoverable Errors with panic!
    // Recoverable Errors with Result
    // --------------------------------
    let some: Result<&str, &str> = Ok("ok");
    println!("{:?}", some);
    let err: Result<&str, &str> = Err("err");
    println!("{:?}", err);

    // --------------------------------
    // ? Error propagation
    // --------------------------------
    let message = match might_fail() {
        Ok(_) => "process succeeded".to_string(),
        Err(cause_message) => cause_message,
    };
    println!("{}", message);

    // calling panic!
    // println!("before panic!");
    // panic!("crash and burn");
    // println!("after panic!");

    // unwrap | expect
    let input: Result<&str, &str> = Ok("test");
    let input = input.unwrap();
    println!("{:?}", input);

    // --------------------------------
    // macro
    // meta programming
    // --------------------------------
    macro_rules! sum {
        ( $( $x:expr ),* ) => {
            {
                let mut result = 0;
                $(
                    result = result + $x;
                )*
                result
            }

        };
    }

    println!("{}", sum![1, 2, 3, 4, 5]);

    // --------------------------------
    // Control Flow
    // --------------------------------

    // if

    let a = if true { 10 } else { 20 };
    println!("{}", a);

    // functional programming
    let a = { 10 * 6 };
    println!("{}", a);

    fn fizz_buzz(value: i32) -> String {
        let result = if value % 15 == 0 {
            "fizz buzz".to_string()
        } else if value % 5 == 0 {
            "buzz".to_string()
        } else if value % 3 == 0 {
            "fizz".to_string()
        } else {
            value.to_string()
        };
        result
    }
    println!("{}", fizz_buzz(1));
    println!("{}", fizz_buzz(3));
    println!("{}", fizz_buzz(5));
    println!("{}", fizz_buzz(30));

    // pattern matching
    fn string_to_color_token(value: &str) -> Option<Color> {
        match value {
            "red" => Some(Color::Red),
            "blue" => Some(Color::Blue),
            "green" => Some(Color::Green),
            _ => None,
        }
    }
    println!("{:?}", string_to_color_token("green"));

    fn fizz_buzz_refactor(value: i32) -> String {
        let result = match value {
            v if v % 15 == 0 => "fizz buzz".to_string(),
            v if v % 5 == 0 => "buzz".to_string(),
            v if v % 3 == 0 => "fizz".to_string(),
            _ => value.to_string(),
        };
        result
    }
    println!("{}", fizz_buzz_refactor(1));
    println!("{}", fizz_buzz_refactor(3));
    println!("{}", fizz_buzz_refactor(5));
    println!("{}", fizz_buzz_refactor(30));

    // Matching with Option<T>
    let data = Some("some text");
    let print_data = match data {
        Some(text) => text,
        None => "none text",
    };
    println!("{}", print_data);

    // if let
    if let Some(color) = string_to_color_token("red") {
        // if string_to_color_token returns Option Some, print red!!
        println!("red!!");
    }

    // loop
    fn add_until_loop(from: i32, to: i32) -> i32 {
        let mut sum = 0;
        let mut temp = from;
        loop {
            sum += temp;
            if temp == to {
                break sum;
            }
            temp += 1;
        }
    }
    let result = add_until_loop(1, 3);
    println!("{}", result);

    // while
    fn add_until_while(from: i32, to: i32) -> i32 {
        let mut sum = 0;
        let mut temp = from;
        while temp <= to {
            sum += temp;
            temp += 1;
        }
        sum
    }
    let result = add_until_while(1, 3);
    println!("{}", result);

    // for (iterator)
    // for cannot return value
    let scores = vec![100, 30, 28, 34, 98];
    for score in scores.iter() {
        println!("score is {}", score);
    }

    // --------------------------------
    // Ownership
    // --------------------------------
}

fn void() {
    // return: empty tuple
}

fn return_void() {
    // return: empty tuple
    return ();
}

fn add(x: i32, y: i32) -> i32 {
    return x + y;
}

fn always_error() -> Result<(), String> {
    Err("Always error".to_string())
}

fn might_fail() -> Result<(), String> {
    let _result = always_error()?;
    Ok(())
}
