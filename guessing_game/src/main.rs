use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1,101);
    println!("The secret number is :{}", secret_number);

        println!("Please input your guess.");

        let mut guess = String::new();
        
        let length = io::stdin()
        .read_line(&mut guess)
        .expect("failed to read line");

        let some_u8_value = Some(0u8);
        if let Some(0u8) = some_u8_value {
            println!("three");
        }

}
