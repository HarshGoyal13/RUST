

use std::fs::File;
use std::io;



fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            io::ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(file) => file,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            other_error => {
                panic!("Problem opening the file: {other_error:?}");
            }
        },
    };

    println!("File opened or created successfully: {:?}", greeting_file);
}









// fn divide(x: i32, y: i32) -> Result<i32, String> {
//     if y == 0 {
//         return Err(String::from("Cannot divide by zero"));
//     }

//     Ok(x / y)
// }

// fn main() {
//     let num = match divide(4, 2){
//         Ok(num) => num,
//         Err(err) => {
//             println!("Error: {err}");
//             -1
//         }
//     };

//     println!("Result: {num}");
// }
