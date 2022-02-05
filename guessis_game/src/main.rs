use std::io;
use rand::{thread_rng, Rng};





fn main() {
    println!("Guess the number");

    let mut rng =  rand::thread_rng();
    let  secret_number = rng.gen_range(1..25);

    

    println!("Please enter a number");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Can not read your number");

    println!("Secret number is {} ", secret_number);

    println!("You enter : {} ", guess);
}
