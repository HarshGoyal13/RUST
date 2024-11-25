
use std::io; // Import the `io` module for input/output
use rand::Rng;
use std::cmp::Ordering;

fn main(){
    println!("Guess the Number");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
            // line has created a mutable variable that is currently bound to a new, empty instance of a String. Whew!
            let mut guess = String::new();

            io::stdin()
            .read_line(&mut guess)   //  calls the read_line method on the standard input handle to get input from the user
            .expect("Failed to read line");  // For Error Handeling

            let guess: u32 = guess.trim().parse().expect("Please type a number!");

            /*
            The {} set of curly brackets is a placeholder: 
            think of {} as little crab pincers that hold a value in place.
            When printing the value of a variable, 
            the variable name can go inside the curly brackets.
            */
            println!("You Guessed: {guess}");

            match guess.cmp(&secret_number){
                Ordering::Less => println!("Too small!"),
                Ordering::Greater => println!("Too big!"),
                Ordering::Equal => {
                println!("You win!");
                break;
                }
            }
    }
    

}
