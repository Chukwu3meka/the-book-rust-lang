use std::io;

pub fn main() {
    let mut x = 5;
    println!("The value of X is: {}", x);

    {
        x = 6;

        println!("The value of X is: {}", x);

        let x = x + 10;

        println!("{} ", x);
    }

    const MIDDLE_NAME: &str = "Vanilla";
    let first_name: _ = String::from("Chukwuemeka");
    let mut last_name: String = String::new();

    last_name.push_str("Hey");

    println!("{} {} {}", last_name, first_name, MIDDLE_NAME,);

    let spaces = "          ";
    let spaces = spaces.len();

    println!("Spaces: {}", spaces);

    let a = 7i8;
    let b = 7i16;
    let c = 7i32;
    let d = 7i64;
    let e = 7i128;

    println!("{},{},{},{},{},", a, b, c, d, e);

    let au = 7i8;
    let bu = 7i16;
    let cu = 7i32;
    let du = 7i64;
    let eu = 7i128;

    println!("{},{},{},{},{},", au, bu, cu, du, eu);

    let decimal = 98_222;
    let hex = 0xff;
    let octal = 0o77;
    let binary = 0b1111_0000;
    let byte = b'A';

    println!(
        "decimal: {},hex: {},octal: {},binary: {},byte: {},",
        decimal, hex, octal, binary, byte
    );

    let int_overflow: u8 = 255;
    println!("{}", int_overflow);

    let sample_tuples = (80, "Man", 's', String::from("A String"));
    println!("Sample Tuples: {:?}", sample_tuples);

    let (a, b, c, d) = sample_tuples;
    println!("a: {}; b: {}; c: {}; d: {}", a, b, c, d);

    let sample_array = ["a", "b", "c", "d", "e"];

    for item in sample_array.iter() {
        println!("Item: {}", item)
    }

    println!("Sample Array: {:?}", sample_array);
    println!("Sample Array: {:?}", sample_array);

    let init_arr_value = [10; 7];
    println!("init_arr_value: {:?}", init_arr_value);

    println!("Index One: {}", init_arr_value[0]);

    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    println!("Enter an Array Index");

    let mut index = {
        return String::new();
    };

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index.trim().parse().expect("Index is not a number");

    let month = months[index];

    println!("The value at index: {} is {}", index, month);

    // let mut exit_text = String::new();
    // std::io::stdin().read_line(&mut exit_text).expect("Crashed");
}
