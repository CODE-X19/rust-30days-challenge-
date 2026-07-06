use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    let secret_num = rand::rng().random_range(1..=100);

    loop {
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
            println!("your guss is either over or below the secret number range ");
            println!("the secrete number range is 1 -100 ");
            continue;
        } else {
            match guess.cmp(&secret_num) {
                Ordering::Less => {
                    println!("too low! try again");
                }
                Ordering::Greater => {
                    println!("too high! try again");
                }
                Ordering::Equal => {
                    println!("Congratulations! You guessed correctly!");
                }
            }

            if guess == secret_num {
                println!("did you want to continue ENTER Yes/No");
                let mut continue_input = String::new();
                io::stdin().read_line(&mut continue_input).unwrap();
                let continu = continue_input.trim();
                if continu == "yes" {
                    continue;
                } else if continu == "No" {
                    break;
                }
            }
        }
    }
}

/*use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    let secret_num = rand::rng().random_range(1..=100);

    loop {
        let mut guess_input = String::new();
        let mut exit_input = String::new();

        println!("Enter your guess:");
        io::stdin().read_line(&mut guess_input).unwrap();

        let guess: i32 = match guess_input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("please input a number");
                return;
            }
        };
        if guess > 100 {
            println!("your guss is over the secret number range ");
            println!("the secrete number range is 1 -100 ");
        } else if exit_input.trim() == "exit" {
            io::stdin().read_line(&mut exit_input).unwrap();
            break;
        } else {
            match guess.cmp(&secret_num) {
                Ordering::Less => {
                    println!("too low! try again");
                }
                Ordering::Greater => {
                    println!("too high! try again");
                }
                Ordering::Equal => {
                    println!("Congratulations! You guessed correctly!");
                }
            }

            if guess == secret_num {
                println!("did you want to continue ENTER Yes/No");
                let mut continue_input = String::new();
                io::stdin().read_line(&mut continue_input).unwrap();
                let continu = continue_input.trim();
                if continu == "yes" {
                    continue;
                } else if continu == "No" {
                    break;
                }
            }
        }
    }
}
 */
