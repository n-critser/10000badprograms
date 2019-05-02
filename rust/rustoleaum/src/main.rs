use std::io;
use rand::Rng;


fn main() {
    println!("guess a number!");
    let secret_number = rand::thread_rng().gen_range(1,101);

    println!("secret_number : {}", secret_number);

    println!("input a number");

    let mut guess = String::new();
    io::stdin().read_line(&mut guess)
        .expect("failed to read line");

    println!("you guessed:{}",guess);
    
}

// fn add1(){
//     let x = 1;
//     return x;
// }
