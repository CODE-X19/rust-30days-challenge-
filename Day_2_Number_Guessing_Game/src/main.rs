use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    loop {

        
      let secret_num = rand::rng().random_range(1..=100);

      'guessing_loop :loop {
        let mut guess_input = String::new();

        println!("Enter your guess:");
        io::stdin().read_line(&mut guess_input).unwrap();

        let guess: i32 = match guess_input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("please input a number");
                continue;
            }
        };
        if guess > 100 || guess < 1 {
            println!("your guess is either over or below the secret number range ");
            println!("the secret number range is 1 -100 ");
            continue;
        }
            match guess.cmp(&secret_num) {
                Ordering::Less => {
                    println!("too low! try again");
                }
                Ordering::Greater => {
                    println!("too high! try again");
                }
                Ordering::Equal => {
                    println!("Congratulations! You guessed correctly!");
                     println!("did you want to continue ENTER Yes/No");
                loop {
                    
                let mut continue_input = String::new();
                io::stdin().read_line(&mut continue_input).unwrap();
                let answer = continue_input.trim().to_lowercase();
                if answer == "yes" {
                     
                     break 'guessing_loop;
                } else if answer == "no" {
                    return;
                }else {
                     println!("Please enter yes or no.");
                }
                }
                }
            }

           
                
               
            }
        }
    }   
    
   


