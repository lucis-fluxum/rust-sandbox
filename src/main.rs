extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    let secret_num = rand::thread_rng().gen_range(0, 100) + 1;

    // println!("The secret number is {}", secret_num);

    loop {
        println!("Enter a guess: ");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        print!("{} is ", guess);

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("too small."),
            Ordering::Greater => println!("too big."),
            Ordering::Equal => {
                println!("correct!");
                break;
            }
        }
    }
}
