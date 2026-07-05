use std::io;
fn main() {
    let mut first_number_input = String::new();
    let mut second_number_input = String::new();
    let mut oprator_input = String::new();
    println!("input first number");
    io::stdin().read_line(&mut first_number_input).unwrap();
    println!("input your operator.");
    io::stdin().read_line(&mut oprator_input).unwrap();
    println!("input second  number");
    io::stdin().read_line(&mut second_number_input).unwrap();

    let first_number: f64 = match first_number_input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
        println!("Please enter a valid number.");
        return;
    }
};
        
    let second_number: f64 = match second_number_input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
        println!("Please enter a valid number.");
        return;
    }
};

    match oprator_input.trim() {
        "+" => {
            let result = add(first_number, second_number);
            println!("result:{}", result);
        }

        "-" => {
            let result = sub(first_number, second_number);
            println!("result:{}", result);
        }

        "/" => {
            if second_number == 0.0 {
                println!("Cannot divide by zero!");
            } else {
                let result = divide(first_number, second_number);
                println!("result:{}", result);
            }
        }

        "*" => {
            let result = multiply(first_number, second_number);
            println!("result:{}", result);
        }
        _ => println!("invalide"),
    }
}


fn add(a: f64, b: f64) -> f64 {
    a + b
}

fn sub(a: f64, b: f64) -> f64 {
    a - b
}

fn divide(a: f64, b: f64) -> f64 {
    a / b
}
fn multiply(a: f64, b: f64) -> f64 {
    a * b
}
