use std::io ;
use std::cmp::Ordering ; 
use rand::Rng ;

fn main() {
    println!("This is a simple Guessing Game Program") ;
    let secret_num = rand::thread_rng().gen_range(1..=100) ;

    loop {
        println!("Enter your guessing number : ") ;
        let mut num = String::new() ;
        io::stdin().read_line(&mut num).unwrap() ;

        let num: u32 = match num.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Plz enter a number only") ;
                continue ;
            }
        };

        match num.cmp(&secret_num) {
            Ordering::Less => println!("Too small") ,
            Ordering::Greater => println!("Too big") ,
            Ordering::Equal => {
                println!("Congrates! Right Guess") ;
                break ;
            }
        }

    }
}