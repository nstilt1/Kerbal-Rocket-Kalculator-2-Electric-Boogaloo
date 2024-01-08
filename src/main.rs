use std::io::{self, Write};

use crate::modules::{size::Size, calculator::Calculator, rocket_config::Rocket};

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
const SIZE_STRS: [&str; 5] = ["xs", "sm", "md", "lg", "xl"];
fn main() {
    println!("Kerbal Kalculator 2: Electric Boogaloo! is at your service");
    let mut calculator = Calculator::new();
    let mut continue_building: Option<f64> = None;
    loop {
        // get the mass from the user, or continue with the mass of the last rocket being built
        let mass: f64 = if continue_building.as_ref().is_none() {
            let m = read("Enter the mass of your payload in tonnes > ");
            if &m == "exit" {
                break;
            }
            m.parse().expect("Failed to parse mass")
        } else {
            continue_building.unwrap()
        };

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

        let is_vacuum = read("Is this stage in a vacuum? (y/n) > ");
        let is_vacuum = match is_vacuum.as_str() {
            "y" => true,
            "n" => false,
            _ => break
        };

        let needs_gimballing = match read("Do you want gimballing? (y/n) > ").as_str() {
            "y" => true,
            "n" => false,
            _ => break
        };

        let mut outputs: Vec<Option<Rocket>> = Vec::new();
        for (size, str) in SIZES.iter().zip(SIZE_STRS.iter()) {
            calculator.init(mass, target_delta_v, minimum_twr, needs_gimballing, is_vacuum, *size);
            let mut output = calculator.calculate();
            output.sort_by(|a, b| a.partial_cmp(b).unwrap());

            if output.len() == 0 {
                outputs.push(None);
            }else{
                outputs.push(Some(output[0].clone()));
            }
        }

        println!("\n\nStarting mass: {}", mass);
        println!("Target dv: {}", target_delta_v);
        println!("Minimum TWR: {}", minimum_twr);
        println!("Available rockets:");
        for (o, size) in outputs.iter().zip(SIZE_STRS.iter()) {
            if o.is_some() {
                println!("\nSize: {}", size);
                o.as_ref().unwrap().print();
            }
        }
        
        continue_building = match read("Select a rocket to use by typing the corresponding size, or type 'new' to try again > ").as_str() {
            "xs" => Some(outputs[0].as_ref().unwrap().mass),
            "sm" => Some(outputs[1].as_ref().unwrap().mass),
            "md" => Some(outputs[2].as_ref().unwrap().mass),
            "lg" => Some(outputs[3].as_ref().unwrap().mass),
            "xl" => Some(outputs[4].as_ref().unwrap().mass),
            "new" => None,
            _ => break
        };
    }
}
