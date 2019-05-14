use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("guess a number!");
    let secret_number = rand::thread_rng().gen_range(1,101);

    println!("secret_number : {}", secret_number);

    loop {
        println!("input a guess number");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("failed to read line");

        
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("you guessed:{}",guess);

        
        match guess.cmp(&secret_number){
            Ordering::Less => println!("TOO SMALL"),
            Ordering::Greater => println!("TOO BIG"),
            Ordering::Equal  => {
                println!("You Win");
                break;
            }
        }
    }
    let res = basis_rep(2,"10010");
    println!("res: {}",res);
}

fn basis_rep(base: i32 , num){

    let mut counter=0;
    for letter in num.iter(){
        println!("letter : {}",letter);
    }
    num
}
