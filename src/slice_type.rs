// Slices let you reference a contiguous sequence of elements in a collection rather than the whole collection.
//  A slice is a kind of reference, so it does not have ownership.

pub fn main() {
    let first_name = String::from("       He is Vanilla");

    let first_word = first_word(&first_name);
    println!("First word: {}", first_word);

    // let first_word_i = first_word(&first_name);
    // println!("First word: {}", first_word_i);

    // let mut s = String::from("hello world");

    // let word = first_word(&s);

    // first_word.clear();

    // println!("s: {s}, word: {word}");

    // ? string slices

    let mut s = String::from("hello world");
    // let hello = &s[0..5];
    // let world = &s[6..11];
    // println!("{hello} {world}",);

    let s_len = s.len();

    s = String::from("Chukwuemeka Maduekwe");

    let slice_1 = &s[..6];
    let slice_2 = &s[6..];
    let slice_3 = &s[..];
    let slice_4 = &s[..5];
    let slice_5 = &s[0..s_len];

    println!("slice_1: {slice_1}");
    println!("slice_2: {slice_2}");
    println!("slice_3: {slice_3}");
    println!("slice_4: {slice_4}");
    println!("slice_5: {slice_5}");
}

// fn second_word(s: &String) -> (usize, usize) {
//     //
// }

fn first_word(s: &String) -> &str {
    let trimmed_word = s.trim();
    let bytes = trimmed_word.as_bytes();

    println!("{:?}", bytes);

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &trimmed_word[..i];
        }
    }

    &trimmed_word[..]
}
