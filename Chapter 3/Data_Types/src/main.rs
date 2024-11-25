

fn main() {

    // SCALAR TYPE - DATA TYPE

    println!("Scalar Data Type");
    
    let integer: i32 = 67; 
    let UnSignedinteger: u32 = 679; 
    let float: f32 = 67.89; 
    let char: char = 'H'; 
    let is_true:bool = false;

    println!("Integer Value : {integer}");
    println!("UnSignedinteger Value : {UnSignedinteger}");
    println!("float Value : {float}");
    println!("char Value : {char}");
    println!("is_true Value : {is_true}");


    // COMPOUND TYPE - DATA TYPE

    println!("Compound Data Type");

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {y}");

    let arr: [i8; 3] = [1, 2, 3];
    let first = arr[0];
    println!("Arr[0] : {first}");


}
