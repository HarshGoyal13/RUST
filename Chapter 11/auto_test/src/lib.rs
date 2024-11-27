



/*

What Are Tests?
> Tests are small programs that check if your code is working correctly.
> The #[cfg(test)] module is only included during testing.
> Tests should be written in isolated environments to avoid unexpected dependencies.

For example: If you wrote a function to add two numbers, 
a test can ensure it always gives the correct result.
\

*/

fn add<T: std::ops::Add<Output = T>>(x:T, y:T)->T{
        x+y
}


#[cfg(test)]    // // Ensures this code is only compiled when running tests.
mod test {

    use super::add;

    #[test]
     fn add_num(){
        // Check if 5 + 4 is equal to 4
        //If the test fails, Rust will tell you what went wrong.
        assert_eq!(add(5,4),9);  // COMPARE TWO VALUES TO CHECK IF THEY ARE EQUAL
    }

    #[test]
    fn smaller_num(){
        // CHECK IF SOMETHING IS TRUE
        assert!(3<5);
    }
    #[test]
    fn not_equal(){
        // CHECK IF TWO THINGS ARE NOT EQUAL
        assert_ne!(5,6);
    }

    #[test]
    fn greater_num(){
         assert!(100 > 45, "IT'S TRUE VALUE");  // GIVE CUSTOM MESSAGE
    }

    
    // IF A TEST TAKES TOO LONG OR ISN'T IMPORTANT RIGHT NOW, YOU CAN IGNORE IT.

    #[test]
    #[ignore]   
    fn slow_test (){
        // SOMETHING THAT TAKES TIME
    }

}





