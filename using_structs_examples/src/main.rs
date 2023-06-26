//rectangles

#[derive(Debug)]
struct Rectangle {
    width: u32,
    length: u32,
}

fn main() {
    // using variables
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );

    // using tuples
    let rect1 = (30, 50);
    println!(
        "The area of the rectangle is {} square pixels.",
        area_by_tuple(rect1)
    );

    // using structs
    let rect1 = Rectangle {
        width: 30,
        length: 50,
    };
    println!("rect1 is {:#?}", rect1);
    println!(
            "The area of the rectangle is {} square pixels.",
            area_by_struct(&rect1)
        );
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_by_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_by_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.length
}