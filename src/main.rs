use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    println!("Guess the Number!");

    let value = rand::thread_rng().gen_range(1..=100);

    loop{
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read Line");

        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed : {guess}");
        println!("Please input your guess.");

        match guess.cmp(&value) {
            Ordering::Less => println!("Too Small \n"),
            Ordering::Greater => println!("Too Large \n"),
            Ordering::Equal => {
                println!("\n \n Correct, You Win!! XD");
                break;
            },
        }
    }

}

