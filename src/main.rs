use std::{cmp::Ordering, io};

use rand::{self, Rng};

fn main() {
    
    let secret_number = rand::thread_rng().gen_range(1, 1001);
    let mut lifes=3;
    println!("wellcome to game 'what's number?'\n you have {lifes} attemps for to hit the secret number!");
    loop {
        if lifes<=0 {
           print!("GAME OVER");
            break;
        }
        println!("what's your guess?");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("error in capture your guess");

        println!("your guess is {guess} ");

        let guess: u32 = match guess.trim().parse(){
            Ok(num)=>num,
            Err(_)=>continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                lifes-=1;
                println!("lot small!")},
            Ordering::Greater =>{ 
                lifes-=1;
                println!("lot tall!")},
            Ordering::Equal => {
                println!("YEAAAH you're right! the number is {secret_number}!");
                break;
            }
        }
        println!("you have only {lifes} attemps!")
    }
}
