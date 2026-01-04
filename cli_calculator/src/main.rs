use std::io;

fn main() {
    println!("***Welcome to Rust CLI calculator!****");
    let num1 = get_number("Enter the first number");
    let num2 = get_number("Enter the second number");
    println!("Enter a operation (+, - , *  /)");
    let mut operation = String::new();
    io::stdin().read_line(&mut operation).expect("Failed to read line");
    let operation = operation.trim();
    let result = match operation {
        "+" => Some(add(num1, num2)),
        "-" => Some(num1 - num2),
        "*" => Some(num1 * num2),
        "/" => {
            if num2 != 0.0 {
                Some(num1 / num2)
            } else {
                println!("Error: Division by zero!");
                None
            }
        }
        _ => {
            println!("Invalid operation. Please enter +,-,* or /");
            None
        }
    };

    if let Some(res) = result {
        println!("Result: {}", res)
    }

    fn add(a: f64, b: f64) -> f64 {
        return a + b;
    }
}

fn  get_number(prompt:&str) ->f64{
    loop {
        println!("{}", prompt);
        let mut input=String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        match input.trim().parse::<f64>() {
            Ok(num)=>return num,
            Err(_)=>println!("Invalid number please enter a valid number"),
        }
    }
}
