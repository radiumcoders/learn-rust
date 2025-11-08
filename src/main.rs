// imports the input output liberary into the scope of this :D
use std::io;

fn main(){
    println!("Wellcomee to the guessing game!!");
    
    //mutable immutable vars
    // let apple = "apple"; immutable ;|
    // let mut banana = "nananananana" mutable :D
    
    // create a mutable variable to store the users guess
    let mut guess = String::new();
    
    // form io we get standard input :)
    // refrence the mutable variable to readline to store the inputed guess ;o
    // using & to refrence and mut to make it mutable :D
    io::stdin().read_line(&mut guess);
    
    print!("u guessed {guess}")
}
