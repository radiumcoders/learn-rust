// imports the input output liberary into the scope of this :D
use std::io;
//importing the rand to generate random numbers :D
use rand::Rng;
//import ordering
use std::cmp::Ordering;

fn main() {
    println!("Wellcomee to the guessing game!!");

    //mutable immutable vars
    // let apple = "apple"; immutable ;|
    // let mut banana = "nananananana" mutable :D

    //generate a random number between 1 to 100
    let sec_num = rand::rng().random_range(1..=100);

    loop {
        // form io we get standard input :)
        // refrence the mutable variable to readline to store the inputed guess ;o
        // using & to refrence and mut to make it mutable :D
        // create a mutable variable to store the users guess
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to get the user input :(");

        // parse the input as int
        let guess: u32 = guess.trim().parse().expect("pls enter number");

        //passing the refrence to the cmp function
        // match is like switch case bit not fully like them
        match guess.cmp(&sec_num) {
            // cmp gives Ordering::Less, Ordering::Greater, Ordering::Equal
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You guessed it right!!");

                break;
            }
        };
    }
    // println!("u guessed {guess}")
}
