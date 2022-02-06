use std::io;
use rand::Rng;
use std::cmp::Ordering;





fn main() {
    

    
    
    

    loop {
        let mut rng =  rand::thread_rng();
    let  secret_number = rng.gen_range(1..25);

    let mut guess = String::new();
        println!("Guess the number");
        println!("Please enter a number");
        io::stdin().read_line(&mut guess).expect("Can not read your number");
        let guess: u32 = guess.trim().parse().expect("Please enter a number");

        println!("Secret number is {} ", secret_number);

        println!("You enter : {} ", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("The number is less"),
            Ordering::Greater => println!("The number is bigger"),
            Ordering::Equal => println!("You are right"),   
        }
    }


    

}
