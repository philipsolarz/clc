use clap::Parser;

#[derive(Parser)]
#[command(name = "clc")]
#[command(about = "A simple calculator", long_about = None)]
struct Clc {
    operations: Vec<String>,
}

#[derive(Debug)]
enum Operation {
    Add(f64),
    Subtract(f64),
    Multiply(f64),
    Divide(f64),
}

fn parse_operations(args: &[String]) -> Result<Vec<Operation>, String> {
    let mut operations = Vec::new();
    let mut iter = args.iter();

    while let Some(op_str) = iter.next() {
        let op = match op_str.to_lowercase().as_str() {
            "add" | "+" => {
                let num = iter.next().ok_or("Missing number after +")?;
                Operation::Add(num.parse().map_err(|_| "Invalid number after +")?)
            },
            "subtract" | "-" => {
                let num = iter.next().ok_or("Missing number after -")?;
                Operation::Subtract(num.parse().map_err(|_| "Invalid number after -")?)
            },
            "multiply" | "*" => {
                let num = iter.next().ok_or("Missing number after *")?;
                Operation::Multiply(num.parse().map_err(|_| "Invalid number after *")?)
            },
            "divide" | "/" => {
                let num = iter.next().ok_or("Missing number after /")?;
                Operation::Divide(num.parse().map_err(|_| "Invalid number after /")?)
            },
            _ => return Err(format!("Invalid operation: {}", op_str)),
        };

        let num_str = iter.next().ok_or_else(||{
            format!("Missing number after {}", op_str)
        })?;

        let num: f64 = num_str.parse().map_err(|_|{
            format!("Invalid number after {}", op_str)
        })?;

        operations.push(op(num));
    }

    Ok(operations)
}

fn main() {
    let clc = Clc::parse();

    if clc.operations.is_empty() {
        eprintln!("No operations provided");
        return;
    }

    match parse_operations(&clc.operations) {
        Ok(ops) => {
            let mut result = 0.0;
            let mut expression = String::from("0");

            for operation in ops {
                match operation {
                    Operation::Add(num) => {
                        result += num;
                        expression += &format!(" + {}", num);
                    },
                    Operation::Subtract(num) => {
                        result -= num;
                        expression += &format!(" - {}", num);
                    },
                    Operation::Multiply(num) => {
                        result *= num;
                        expression += &format!(" * {}", num);
                    },
                    Operation::Divide(num) => {
                        if num == 0.0 {
                            eprintln!("Division by zero");
                            return;
                        }
                        result /= num;
                        expression += &format!(" / {}", num);
                    },
                }
            }

            println!("{} = {}", expression, result);
        }
        Err(err) => eprintln!("{}", err),
    }
}