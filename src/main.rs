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


}
