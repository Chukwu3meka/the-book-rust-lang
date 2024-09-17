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
    let r1 = &mut s;
    let r2 = &mut s;

    println!("{}, {}.", r1, r2);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
