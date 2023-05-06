use std::io;
use rand::Rng;
use std::cmp::Ordering;

/*
Steps for this program
1. take input in a mutable variable using io::stdin library
2. generate a random number
3. match the input variable with random number
*/

fn main() {
    println!("Welcome to guessing game \n Please input your guess");
    //generate random number here
    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);
    loop{
        let mut guess: String = String::new();
        io::stdin().
            read_line(&mut guess).
            expect("Failed to read line");
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("Please enter valid integer");
                continue;
            }
        };
        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too Small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("Yay! You got it right");
                break;
            }
        }
    }
}
