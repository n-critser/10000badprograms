extern crate rand;



use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {

    let secret_number = rand::thread_rng().gen_range(1,101);
    println!("Hello, world!");
    //println!("the secret is : {}", secret_number);
    loop {
        println!("please insert your guess");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .ok()
            .expect("failed to read line");
        println!("you guessed: {}", guess);


        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too big"),
            Ordering::Equal => {
                println!("goldilocks wins");
                break;
            }
        }
    }    
    
}
