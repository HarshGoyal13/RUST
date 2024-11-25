

// SIMPLE FUNCTION WITHOUT PARA METER OR RETURN TYPE
fn simple_function(){
    println!("Hello");
}


// FUNCTION WITH PARAMETER
fn Parameter(name: &str){
    println!("helo {name}")
}

// FUNCTION WITH RETURN TYPE
fn add(p1:i32, p2:i32) -> i32 {
         p1 + p2  // if i use semiclon here, then i should NOT use return keyword  or other we use return 
}

// FUNCTION WITH MULTIPLE RETURN TYPE
fn MultipleReturnType(x:i32, y:i32)-> (i32, i32){
        (x+y, x-y)
}

// FUNCTION WITH GENERIC
fn print<T: std::fmt::Debug>(item: T) {
    println!("{:?}", item); // T can be any type
}

fn main() {
    simple_function();

    Parameter("Harsh");

    let ReturnType = add(56 , 67);
    println!{"Sum is : {ReturnType}"};

    let (sum,dif)= MultipleReturnType(56 , 67);
    println!{"Sum is : {sum}, Difference is : {dif}"};

    print(42); // Prints an integer
    print("Hello, world!"); // Prints a string
}
