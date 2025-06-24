// standard input library
use std::io::stdin;

// random number dependency
use rand::Rng;

fn main() {
    
    // outer loop that we will leave it once match is sucecesful
    '_outer_loop: loop {

        
    // let _number: u32 = 10;  // static number of 10
    let _number: i32 = rand::rng().gen_range(1..=5); // dynamic number based on random range generated 
        
    // prompt the user to input a number
    println!("Pick a number >>>");

    loop {

        //Empty New String
        let mut line = String::new();

        //Reading the input
        let _input = stdin().read_line(&mut line);
        
        // Guess
        let _guess: Option<i32> = _input.ok().map_or(None, |_| line.trim().parse().ok());
        
        // matching guess [same as if else statement / switch case in C++]
        match _guess{
            None => println!("enter a number..."),
            Some(n) if n == _number => {
                println!("Bravo! You guessed it 😆!");
                break '_outer_loop;
            }
            Some(n) if n < _number => println!("Too low"),
            Some(n) if n > _number => println!("Too high"),
            Some(_) => println!("Error!")
        }
    }

    }
}