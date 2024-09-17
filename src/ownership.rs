pub fn main() {
    //

    // let mut a = String::from("hello");
    //
    // let a = 2;
    // // {
    //
    // // let b = &a;
    // let b = a.clone();
    //
    // println!("{a}");
    // //
    // // a.push_str(", Stable");
    // // }

    //
    // println!("{b}");
    // {
    //     let s = "hello";
    // }

    // let mut s = String::from("hello");
    //
    // s.push_str(", world!");
    //
    // println!("{}", s);
    //
    // let default_string = "Hey";
    //
    // println!("{default_string}");

    // let s = String::from("Sample string");

    // takes_ownership_and_return(s);

    // let x = s;

    let mut s = String::from("The hello world");

    s.clear();

    let word = first_word(&s);

    s = String::from("Bugs in Rust");

    println!("{word}");

    // let hello = &s[0..5];

    // println!("{} world", hello);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
//
//
// fn takes_ownership_and_return() -> (String, usize) {
//     // println!("{sm}, sm");
//
//     let mut a = String::from("Vanilla 🦀");
//
//     (&a, a.len())
// }
