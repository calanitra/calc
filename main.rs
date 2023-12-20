use std::io::{self, Write};

fn main() {
    let mut input = String::new();

    loop {
        print!("Enter expression (or 'exit' to quit): ");
        io::stdout().flush().unwrap();

        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let trimmed = input.trim();
        if trimmed == "exit" {
            break;
        }

        match evaluate_expression(trimmed) {
            Ok(result) => println!("Result: {}", result),
            Err(err) => println!("Error: {}", err),
        }
    }
}

fn evaluate_expression(expression: &str) -> Result<f64, &'static str> {
    let parts: Vec<&str> = expression.split_whitespace().collect();
    if parts.len() != 3 {
        return Err("Invalid expression format");
    }

    let operand1: f64 = match parts[0].parse() {
        Ok(num) => num,
        Err(_) => return Err("Invalid first operand"),
    };

    let operator: &str = parts[1];

    let operand2: f64 = match parts[2].parse() {
        Ok(num) => num,
        Err(_) => return Err("Invalid second operand"),
    };

    match operator {
        "+" => Ok(operand1 + operand2),
        "-" => Ok(operand1 - operand2),
        "*" => Ok(operand1 * operand2),
        "/" => {
            if operand2 == 0.0 {
                Err("Division by zero")
            } else {
                Ok(operand1 / operand2)
            }
        }
        _ => Err("Invalid operator"),
    }
}
