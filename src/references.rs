// ? The Rules of References
// At any given time, you can have either one mutable reference or any number of immutable references.
// References must always be valid.

pub fn main() {
    // ? Referencing example
    let s1 = String::from("hello");
    let len = calculate_length(&s1);

    println!("The length of '{s1}' is {len}. {}", len);

    // ? Modify referenced value
    let mut s = String::from("hello");

    change(&mut s);
    println!("The value of s is {s}");

    // ? Two mutable references
    // let r1 = &mut s;
    // let r2 = &mut s;

    // println!("{}, {}.", r1, r2);

    {
        let r1 = &mut s;
        println!("{}.", r1,);
    }
    let r2 = &mut s;

    println!("{}.", r2);

    // ? Combining mutable and immutable reference

    // let first_name = String::from("dgdgfddfd");

    // * You cannot use a mutable ref after using an immutable ref on the same value
    let mut v = String::from("hello");

    let v1 = &v; // no problem
    let v2: &String = &v; // no problem
    println!("{}, and {}", v1, v2,);

    let v3 = &mut v; // BIG PROBLEM

    println!("{}", v3);

    v = String::from("Chukwuemeka");
    // println!("{}", v3);

    // ? Dangling References
    // A pointer that references a location iun memory that may have been given to someone else

    let reference_to_nothing = dangle();
}

fn dangle() -> String {
    // error code
    // let s = String::from("hello");
    // &s

    // correct code
    let s = String::from("hello");
    s
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
