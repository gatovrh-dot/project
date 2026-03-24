use rand::RngExt;
use std::cmp::Ordering;
use std::io;
fn main() {
    println!("Guess thr game!");

    let secret_number = rand::rng().random_range(1..=100);

    loop {
        println!("Please enter the number!");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("fail to read line!");

            let guess: u8 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a number!");
            continue;
        }
    };

    println!("You guessed: {guess}");

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

