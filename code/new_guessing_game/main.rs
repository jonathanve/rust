use rand::Rng;
use std::cmp::Ordering;
use std::io::Error;
use std::io::ErrorKind;
use std::io;

pub struct Guess {
    value: i32,
}

impl Guess {

    fn new(value: i32) -> Result<Guess, io::Error> {
        if value < 1 || value > 100 {
            println!("Guess value must be between 1 and 100, got {}.", value);
            return Err(Error::new(ErrorKind::Other, "Value must be between 1 and 100"));
        }

        Ok(Guess {
            value
        })
    }

    fn value(&self) -> i32 {
        self.value
    }
}

fn main() {
    println!("# Guess the number from 1 to 100!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guess:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess = match guess.trim().parse() {
            Ok(num) => match Guess::new(num) {
                Ok(guess) => guess.value(),
                Err(_) => continue,
            },
            Err(_) => {
                println!("Guess value must be an integer");
                continue
            },
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
