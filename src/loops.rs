pub fn main() {
    // loop {
    //     println!("again")
    // }

    // let mut counter = 0;

    // let result = loop {
    //     counter += 1;

    //     if counter == 10 {
    //         break counter * 2;
    //     }
    // };

    // println!("The result is {result}");

    // let mut count = 0;

    // 'counting_up: loop {
    //     println!("count {count}");
    //     let mut remaining = 10;

    //     loop {
    //         println!("remaining {remaining}");

    //         if remaining == 9 {
    //             break;
    //         }

    //         if count == 2 {
    //             break 'counting_up;
    //         }

    //         remaining -= 1;
    //     }

    let mut curr_value = 1;

    loop {
        let sum = curr_value + curr_value;

        if curr_value == i32::MAX {
            break;
        }

        println!("{0} + {0} = {1}", curr_value, sum);

        curr_value = sum;
    }

    println!("End of program");

    //     count += 1;
    // }

    // println!("End count = {count}");

    // let age = 23;
    // let curr_year = 2024;

    // let mut birth_year = curr_year;

    // 'curr_age: loop {
    //     if curr_year - birth_year == age {
    //         println!("If I'm {age} years old, my birth year is {birth_year}");
    //         break 'curr_age;
    //     }

    //     birth_year -= 1;

    //     println!("Previous year is {birth_year}");
    // }

    // 'growing: loop {
    //     println!("Age: {age}");

    //     age -= 1;

    //     let mut birth_year = 2024;

    //     loop {
    //         if age == 0 {
    //             println!("Birth year {birth_year}");
    //             break;
    //         }

    //         if birth_year == 2000 {
    //             println!("Least allowed birth year is {birth_year}");
    //             break 'growing;
    //         };

    //         println!("Current Year: {birth_year}");
    //         birth_year -= 1;
    //     }
    // }

    // println!("I'm {age} years old");

    // let mut number = 1111126;

    // while number != 0 {
    //     println!("{number}!");
    //     number -= 1;
    // }

    let mut index = 0;
    let intervals = [10, 20, 30, 40, 50, 60, 70, 80, 90, 100];

    while index < intervals.len() {
        println!("{:?}", intervals[index]);

        index += 1
    }

    println!("\nRust in Action\n");

    for interval in intervals {
        println!("{:?}", interval)
    }

    println!("\nRust in Action\n");

    for number in (1..4000).rev() {
        println!("{:?}", number)
    }
}
