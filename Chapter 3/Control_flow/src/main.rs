
/*

Control flow in Rust means deciding what code runs 
and when based on conditions or loops.
 It helps you control the order of execution in your program.

*/

fn main() {

    // IF-ELSE STATEMENT

    println!("IF-ELSE STATEMENT");
    let number = 3;

    if number < 5 {
        println!("Condition was true");
    } else {
        println!("Condition was not true");
    }


    // LOOP -> INFINITE LOOP THAT ONLY STOP WITH BREAK KEYWORD

    println!("INFINITE LOOP STATEMENT");
    let mut num1  = 5;

    loop {
        num1 += 1;
        println!("{num1}");

        if num1 == 10 {
            break;
        }
    }

    // WHILE LOOP STATEMENT

    println!("WHILE LOOP STATEMENT");
    let mut num2 = 6;

    while num2 < 10{
        num2 += 1;
        println!("{num2}");
    }

    // FOOR - LOOP STATEMENT

    println!("FOR LOOP STATEMENT");
    let arr : [i32; 5] = [4,5,6,7,8];

    for element in arr{
        println!("{element}");
    }

    // MATCH - STATEMENT

    println!("MATCH STATEMENT");
    let num4 = 55;

    match num4 {
        1 => println!("One"),
        4 => println!("Four"),
        8 => println!("Eight"),
        50..=100 => println!("Number between 50 to 100"),
        _=> println!("Number is something else"),
    }



}
