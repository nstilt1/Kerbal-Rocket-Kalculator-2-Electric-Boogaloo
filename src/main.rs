use std::io::{self, Write};

use crate::modules::{size::Size, calculator::Calculator};


mod modules;

const G: f64 = 9.81;
fn read(text: &str) -> String {
    let mut input = String::new();
    print!("{}", text);
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut input)
        .expect("Failed to read line");
    return input.trim().to_owned();
}

const SIZES: [Size; 5] = [Size::Xs, Size::Sm, Size::Md, Size::Lg, Size::Xl];
fn main() {
    println!("Kerbal Kalculator 2: Electric Boogaloo! is at your service");
    let mut calculator = Calculator::new();
    loop {

        let mass = read("Enter the mass of your payload in tonnes > ");
        if &mass == "exit" {
            break;
        }
        let mass: f64 = mass.parse().expect("Failed to parse mass");

        let target_delta_v = read("Enter the target delta-v in m/s > ");
        if &target_delta_v == "exit" {
            break;
        }
        let target_delta_v: f64 = target_delta_v.parse().expect("Failed to parse target_delta_v");

        let minimum_twr = read("Enter the minimum TWR > ");
        if &minimum_twr == "exit" {
            break;
        }
        let minimum_twr: f64 = minimum_twr.parse().expect("Failed to parse minimum_twr");

        let is_vacuum = read("Is this stage in a vacuum? (y/n)");
        let is_vacuum = match is_vacuum.as_str() {
            "y" => true,
            "n" => false,
            _ => break
        };

        let needs_gimballing = match read("Do you want gimballing? (y/n) ").as_str() {
            "y" => true,
            "n" => false,
            _ => break
        };

        for (size, str) in SIZES.iter().zip(["xs", "sm", "md", "lg", "xl"].iter()) {
            calculator.init(mass, target_delta_v, minimum_twr, needs_gimballing, is_vacuum, *size);
            let mut output = calculator.calculate();
            output.sort_by(|a, b| a.partial_cmp(b).unwrap());
            println!("Checking size: {}", str);
            if output.len() == 0 {
                println!("No results found\n");
            }else{
                output[0].print();
                println!("\n");
            }
        }
    }
}
