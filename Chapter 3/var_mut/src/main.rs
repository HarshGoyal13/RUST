fn main() {

    const THREE_HOURS_IN_SECOND :u32 = 60 * 60 * 3; 


    let x = 5;  // Immutable Variable
    //  x  = 55; We can not Re assign
    println!("Value of x : {x}");


    let mut y = 5;  // mutable Variable
     y  = 55;  // We can  Re assign
    println!("Value of y : {y}");


    println!("Value of Const : {THREE_HOURS_IN_SECOND}");




    let i = 9;
    println!("Value of i : {i}");
   {
    let i = 99;   // Shadow I
    println!("Value of i : {i}");
   }
    println!("Value of i : {i}");

 
}
