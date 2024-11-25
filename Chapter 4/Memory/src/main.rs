

static GREETING: &str = "Hello, World!";      //STATIC



fn main() {


    let x = 5;   /// Allocated on the stack
    let y = x;    // HERE X IS COPY NOT MOVED, SO BOTH ARE VALID
    println!("{x}");


    /*

    When String::from("hello") is called:
    Memory is allocated on the heap to store "hello".
    A pointer, length, and capacity are stored on the stack to manage the heap memory.
    When ownership moves, only the stack data is copied, leaving the heap data intact.
    
    */

    let s1 = String::from("Hello");  // // Allocates memory on the heap
    let s2 = s1;  // Moves ownership of the heap memory to `s2`
    // println!(s1); // ERROR: `s1` is no longer valid



    println!("{}", GREETING); // `GREETING` lives for the program's lifetime


} 

// Both `x` and `y` are popped off the stack here.
// When `s2` goes out of scope, the heap memory is cleaned up.


