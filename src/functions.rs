use std::io;

pub fn main() {
    // another_function(22); // takes concrete values/arguments

    // ? Statements are instructions that perform some action and do not return a value.
    // ? Expressions evaluate to a resultant value. Let’s look at some examples.

    // let y = {
    //     let mut x = 3;
    //     x += 1;

    //     x

    //     // println!("x * 2: {}", x * 2);
    // };

    let my_age = {
        let this_year = 2023;
        let birth_year = 1998;

        this_year - birth_year
    };

    // fn my_name() -> String {
    //     let name = String::from("Chukwuemeka Maduekwe");

    //     name
    // }

    fn shuffle_name(name: String) -> String {
        // let index = name.len();
        let shuffled_name = String::new();

        // while index > 0 {
        //     // shuffled_name = String::push(name[index]);
        //     // shuffled_name = String::push_str(&mut shuffled_name, "");
        //     String::push_str(&mut shuffled_name, name[index]);

        //     println!(name[index])
        // }

        shuffled_name
    }

    println!(
        "My name in reverse is {:?}, and currently I am {} years old.",
        shuffle_name(String::from("Chukwuemeka Maduekwe")),
        my_age
    );

    // // println!("My name is {:?}", my_name());

    // // println!("y: {}", y);
    // println!("Sum of 5 and 7: {}", sum(5, 7));
    // println!("Enter you name");

    let name = get_name();

    println!("Name: {name}");
}

// fn another_function(x: i32) {
//     // accepts parameters
//     println!("x: {}", x);
// }

// fn sum(a: i32, b: i32) -> i32 {
//     a + b
// }

fn get_name() -> String {
    let mut name = String::new();

    io::stdin()
        .read_line(&mut name)
        .expect("Invalid name entered");

    name
}
