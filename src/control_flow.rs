pub fn main() {
    let mut number = 09;

    // if number {
    //     println!("Number is not equal to 0")
    // }

    if number < 5 {
        println!("Number is less than 5")
    } else if number > 5 {
        println!("Number is greater than 5")
    } else {
        println!("Number is equal to 5")
    }

    if number % 2 == 0 {
        println!("Number is divisible by 2")
    } else if number % 3 == 0 {
        println!("Number is divisible by 3")
    } else {
        println!("Number is not divisible by 2 or 3")
    }

    let condition = true;
    number = if condition { 7 } else { 3 };

    println!("Number is {}.", number);
}
