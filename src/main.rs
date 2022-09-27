use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Welcome to Guessing Game!");
    let secret_number = rand::thread_rng().gen_range(1..=10);
    println!("Secret number is :{secret_number}");
    loop {
        println!("Enter your Guess! :");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read input");
        let guess:u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("Yahoo, You Won!");
                break;
            }
        }
    }
}