fn main() {
    let s = String::from("hello");

    takes_ownership(s);

    let x = 5;

    makes_copy(x);

    let s1 = gives_ownership();

    let s2 = String::from("hello");

    let s3 = takes_and_gives_back(s2);

    println!("{s3}");

    let s1 = String:: from ("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of {s2} is {len}.");

    let mut s = String:: from("hello world");

    let word = first_word(&s);

    s.clear();

    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..];
    println!("Hello: {hello}, World: {world}")
}

fn takes_ownership(some_string: String) {
    println!("{some_string}");
}

fn makes_copy(some_integer: i32) {
    println!("{some_integer}");
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");

    some_string
}

fn takes_and_gives_back(mut a_string: String) -> String {
    a_string.push_str(", world");
    a_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item ==b' ' {
            return i;
        }
    }

    s.len()
}