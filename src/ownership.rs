// ? Ownership Rules

// Each value in Rust has an owner.
// There can only be one owner at a time.
// When the owner goes out of scope, the value will be dropped.

pub fn main() {
    // ? 1
    // let middle_name = "Vanilla";
    // let first_name = String::from("Chukwuemeka");

    // println!(
    //     "My Middle name is {0} First name is {1}.",
    //     middle_name, first_name
    // );

    // // {
    // let mut fullname = String::from(first_name);
    // fullname.push_str(" Maduekwe");
    // // }

    // drop(fullname);

    // println!("Fullname is {}", fullname);

    // ? 2
    // let x = 5;
    // let y = x;

    // let s1 = String::from("hello");
    // let s2 = s1;

    // // drop(s1);

    // println!("{}", s2);

    // ? 3
    // let s1 = String::from("hello");
    // let s2 = s1.clone();
    // drop(s2);
    // println!("{}", s1);

    // ? 4
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{s2}' is {len}.");
    // println!("{s1}");
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}
