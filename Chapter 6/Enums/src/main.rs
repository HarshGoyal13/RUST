enum Shape {
    Circle(u32), 
    Rectangle(u32, u32),
}

fn area(shape: Shape) -> u32 {
    match shape {
        Shape::Circle(radius) => radius * radius,  // Area of the circle (simplified formula)
        Shape::Rectangle(width, height) => width * height,  // Area of the rectangle
    }
}

fn add(num1: i32, num2: Option<i32>) -> i32 {
    match num2 {
        Some(i) => i + num1,  // Otherwise, add num1 to the value in Some
        None => num1,  // If None, return num1 wrapped in Some
    }
}



fn main() {
    let circle = Shape::Circle(7);
    let rectangle = Shape::Rectangle(7, 9);

    // Printing areas
    println!("Circle area: {}", area(circle));  // Outputs 49 (7 * 7)
    println!("Rectangle area: {}", area(rectangle));  // Outputs 63 (7 * 9)

    // Printing addition result
    println!("Add result: {}", add(4, Some(5)));  // Outputs Some(9)



    let dice_roll = 100;

    match dice_roll {
        3 => println!("You have a fancy hat👒"),
        6 => println!("Your hat was removed❌"),
        other => println!("Your Move {other} Steps")
    }


    let shape = Shape::Circle(33);
    if let Shape::Circle(radius) = &shape{
        println!("Shape is Circle");
    }





}
