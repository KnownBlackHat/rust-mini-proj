use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Welcome To Guess Game!");

    println!("Guess number: ");
    let guess_number = rand::thread_rng().gen_range(1..100);

    loop {
        let mut usr_guess = String::new();
        std::io::stdin()
            .read_line(&mut usr_guess)
            .expect("Failed to read line");
        let usr_guess: u32 = match usr_guess.trim().parse() {
            Ok(value) => value,
            Err(_) => continue,
        };


        match usr_guess.cmp(&guess_number) {
            Ordering::Less => println!("Too Small"),
            Ordering::Greater => println!("Too Big"),
            Ordering::Equal => {
                println!("Yay! You won the game");
                break;
            }
        }
    }

}
