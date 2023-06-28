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


fn main() {
    println!("Kerbal Kalculator at your service");
    let mut calc = Calculator::new();
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

        let size = match read("What size diameter do you desire? (xs/sm/md/lg) > ").as_str() {
            "xs" => Size::Xs,
            "sm" => Size::Sm,
            "md" => Size::Md,
            "lg" => Size::Lg,
            _ => break
        };

        calc.init(mass, target_delta_v, minimum_twr, needs_gimballing, is_vacuum, size);

        let mut output = calc.calculate();
        output.sort_by(|a, b| a.partial_cmp(b).unwrap());
        if output.len() == 0 {
            println!("No results found");
        }else{
            output[0].print();
        }


    }
    
}
