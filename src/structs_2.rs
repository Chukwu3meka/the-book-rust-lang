#[derive(Debug)] // this will allow us print data with this struct to console
struct Rectangle {
    width: u32,
    height: u32,
    name: String,
}

pub fn main() {
    let width1 = 30;
    let height1 = 50;

    // println!(
    //     "The area of the rectangle is {} square pixels.",
    //     area(width1, height1)
    // );

    // ? refactoring with tuples
    let rect1 = (30, 50);

    // println!(
    //     "The area of the rectangle is {} square pixels.",
    //     area2(rect1)
    // );

    // ?  refactoring worth structs

    let rect2 = Rectangle {
        width: 30,
        height: 50,
        name: String::from("Test Data"),
    };

    println!(
        "The area of the rectangle is {} square pixels",
        area3(&rect2)
    );

    println!("1. rect2 is {rect2:?}",);
    println!("2. rect2 is {rect2:#?}",);
    println!("3. rect2 is {:?}", rect2);

    // ?dbg! macro taked ownership of the value to be displayed as opposed to println

    dbg!(&rect2);
    dbg!(&rect2);
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area3(dimensions: &Rectangle) -> u32 {
    dimensions.width * dimensions.height
}
