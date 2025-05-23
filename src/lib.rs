#![feature(const_trait_impl)]

#[cfg(target_arch = "wasm32")]
use js_sys::Array;
#[allow(unused)]
use modules::{calculator::Calculator};
use wasm_bindgen::prelude::*;

pub mod modules;

const G: f64 = 9.80665;

pub const TECH_TREE: &[&'static str] = &[
    "start",
    "Post-War Rocketry Testing",
    "Early Rocketry",
    "Basic Rocketry",
    "1956-1957 Orbital Rocketry"
];

#[wasm_bindgen]
#[cfg(target_arch="wasm32")]
pub fn calculate(
    mass: f64,
    target_delta_v: f64,
    minimum_twr: f64,
    maximum_twr: f64,
    in_vacuum: bool,
    needs_gimballing: bool,
    use_nosecone: bool,
    diameter: f64,
    unlocked_fuselages: Array,
    unlocked_tech: Array,
) -> Result<String, JsError> {
    use modules::{rocket_config::Rocket, size::Size};

    let mut calculator = Calculator::new();
    let size = match diameter {
        0.3 => Size::Xs,
        1.25 => Size::Sm,
        1.3 => Size::Sm,
        _ => Size::Sm,
    };
    let unlocked_fuselages: Vec<String> = unlocked_fuselages.iter().map(|v| v.as_string().expect("Should be a string")).collect();
    let unlocked_tech: Vec<String> = unlocked_tech.iter().map(|v| v.as_string().expect("Should be a string")).collect();
    let unlocked_fuselages: String = unlocked_fuselages.join(",");
    let unlocked_tech: String = unlocked_tech.join(",");
    calculator.init(mass, target_delta_v, minimum_twr, maximum_twr, needs_gimballing, in_vacuum, use_nosecone, size, unlocked_fuselages, unlocked_tech);

    let (mut nose_plus_cyl_results, mut cyl_results, mut nose_results) = calculator.calculate()?;
    let mut output: Vec<Rocket> = Vec::with_capacity(nose_plus_cyl_results.len() + cyl_results.len() + nose_results.len());
    output.append(&mut nose_plus_cyl_results);
    output.append(&mut cyl_results);
    output.append(&mut nose_results);

    if output.is_empty() {
        return Ok("No rockets found".to_string());
    }
    output.sort_by(|a, b| a.partial_cmp(b).unwrap());

    return Ok(output[0].to_string());
}

#[cfg(not(target_arch = "wasm32"))]
pub mod test_utils {

}