use std::{env, error::Error};

mod day01;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => {
            println!("01: {} {}", day01::solve_1(), day01::solve_2());
        }
        _ => match args[1].as_str() {
            "1" => println!("{} {}", day01::solve_1(), day01::solve_2()),
            _ => return Err(format!("Invalid day: '{}'", args[1]).into()),
        },
    }
    Ok(())
}
