#![allow(unused)]
#![feature(const_trait_impl)]

use std::io::{self, Write};

use crate::modules::{size::Size, calculator::Calculator, rocket_config::Rocket};

mod modules;

//const G: f64 = 9.81;
const G: f64 = 9.80665;
fn read(text: &str) -> String {
    let mut input = String::new();
    print!("{}", text);
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut input)
        .expect("Failed to read line");
    return input.trim().to_owned();
}

fn handle_output(mass: f64, target_delta_v: f64, minimum_twr: f64, calculator: &mut Calculator) {
    let (mut nose_plus_cylinder_results, mut cylinder_results, mut nose_results) = calculator.calculate();
        let mut output: Vec<Rocket> = Vec::with_capacity(nose_plus_cylinder_results.len() + cylinder_results.len() + nose_results.len());
        output.append(&mut nose_plus_cylinder_results.clone());
        output.append(&mut cylinder_results.clone());
        output.append(&mut nose_results.clone());
        output.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let mut outputs: Vec<Option<Rocket>> = Vec::new();

        if output.len() == 0 {
            outputs.push(None);
            println!("No rockets found");
        }else{
            outputs.push(Some(output[0].clone()));
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
}

const SIZES: [Size; 5] = [Size::Xs, Size::Sm, Size::Md, Size::Lg, Size::Xl];
const SIZE_STRS: [&str; 5] = ["xs", "sm", "md", "lg", "xl"];
fn main() {
    println!("Kerbal Kalculator 2: Electric Boogaloo! is at your service");
    let mut calculator = Calculator::new();
    let mut continue_building: Option<f64> = None;
    loop {
        // get the mass from the user, or continue with the mass of the last rocket being built
        let mut mass: f64 = if continue_building.as_ref().is_none() {
            let m = read("Enter the mass of your payload in tonnes > ");
            if &m == "exit" {
                break;
            }
            m.parse().expect("Failed to parse mass")
        } else {
            let m = continue_building.unwrap();
            println!("Starting mass: {}", m);
            m
        };

        let target_delta_v = read("Enter the target delta-v in m/s > ");
        if &target_delta_v == "exit" {
            break;
        }
        let mut target_delta_v: f64 = target_delta_v.parse().expect("Failed to parse target_delta_v");

        let minimum_twr = read("Enter the minimum TWR > ");
        if &minimum_twr == "exit" {
            break;
        }
        let mut minimum_twr: f64 = minimum_twr.parse().expect("Failed to parse minimum_twr");
        let mut maximum_twr: f64 = match read("Enter the maximum TWR > ").parse::<f64>() {
            Ok(v) => v,
            Err(_) => break
        };

        let is_vacuum = read("Is this stage in a vacuum? (y/n) > ");
        let is_vacuum = match is_vacuum.to_lowercase().as_str() {
            "y" => true,
            "n" => false,
            _ => break
        };

        let needs_gimballing = match read("Do you want gimballing? (y/n) > ").to_lowercase().as_str() {
            "y" => true,
            "n" => false,
            _ => break
        };

        let use_nosecone = match read("Do you want the center fuel tanks to have a nosecone? (y/n> > ").to_lowercase().as_str() {
            "y" => true,
            "n" => false,
            _ => break
        };
        let mut diameter = match read("Enter the payload's diameter in meters > ").parse::<f64>() {
            Ok(v) => v,
            Err(_) => break
        };
        let size = match diameter {
            0.3 => Size::Xs,
            1.25 => Size::Sm,
            1.3 => Size::Sm,
            _ => Size::Sm,
        };
        let unlocked_fuselages = read("Enter your unlocked fuselages separated by commas and excluding HP prefixes > ").to_string();
        calculator.init(mass, target_delta_v, minimum_twr, maximum_twr, needs_gimballing, is_vacuum, use_nosecone, size, unlocked_fuselages.clone());
        
        handle_output(mass, target_delta_v, minimum_twr, &mut calculator);

        loop {
            match read("Choose an action:\n1) Change target delta-v\n2) Change payload mass\n3) Change minimum twr\n4) Change maximum twr").as_str() {
                "1" => {
                    let dv = match read("Enter the new delta-v for this stage > ").parse::<f64>() {
                        Ok(v) => v,
                        Err(_) => break
                    };
                    target_delta_v = dv;
                    calculator.change_target_delta_v(dv);
                    handle_output(mass, target_delta_v, minimum_twr, &mut calculator);
                },
                "2" => {
                    let m = match read("Enter the new mass for this stage in tons > ").parse::<f64>() {
                        Ok(v) => v,
                        Err(_) => break
                    };
                    mass = m;
                    calculator.change_mass(mass);
                    handle_output(mass, target_delta_v, minimum_twr, &mut calculator);
                },
                "3" => {
                    minimum_twr = match read("Enter the new minimum twr for this stage > ").parse() {
                        Ok(v) => v,
                        Err(_) => break
                    };
                    calculator.change_minimum_twr(minimum_twr);
                    handle_output(mass, target_delta_v, minimum_twr, &mut calculator);
                },
                "4" => maximum_twr = match read("Enter the new maximum twr for this stage > ").parse() {
                    Ok(v) => v,
                    Err(_) => break
                },
                _ => break
            }
        }
    }
}
