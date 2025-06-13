#![feature(const_trait_impl)]

#[cfg(target_arch = "wasm32")]
use js_sys::Array;
#[cfg(target_arch = "wasm32")]
use modules::calculator::Calculator;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

pub mod modules;

const G: f64 = 9.81;
//const G: f64 = 9.80665;

pub const TECH_TREE: &[&'static str] = &[
    "start",
    "Post-War Rocketry Testing",
    "Early Rocketry",
    "Basic Rocketry",
    "1956-1957 Orbital Rocketry",
];

#[wasm_bindgen]
#[cfg(target_arch = "wasm32")]
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
    nose_height: f64,
) -> Result<String, JsError> {
    use modules::{rocket_config::Rocket, size::Size};

    assert!(target_delta_v > 0.0, "Target delta v was not positive");
    assert!(minimum_twr >= 0.01, "minimum twr was less than 0.01");
    assert!(
        maximum_twr > minimum_twr,
        "maximum twr was greater than minimum twr"
    );
    assert!(
        unlocked_fuselages.length() > 0,
        "unlocked fuselages.len was 0"
    );
    assert!(unlocked_tech.length() > 0, "unlocked tech.len was 0");

    let mut calculator = Calculator::new();
    let size = match diameter {
        0.3 => Size::Xs,
        1.25 => Size::Sm,
        1.3 => Size::Sm,
        _ => Size::Sm,
    };
    let unlocked_fuselages: Vec<String> = unlocked_fuselages
        .iter()
        .map(|v| v.as_string().expect("Should be a string"))
        .collect();
    let unlocked_tech: Vec<String> = unlocked_tech
        .iter()
        .map(|v| v.as_string().expect("Should be a string"))
        .collect();
    let unlocked_fuselages: String = unlocked_fuselages.join(",");
    let unlocked_tech: String = unlocked_tech.join(",");
    calculator.init(
        mass,
        target_delta_v,
        minimum_twr,
        maximum_twr,
        needs_gimballing,
        in_vacuum,
        use_nosecone,
        nose_height,
        diameter,
        unlocked_fuselages,
        unlocked_tech,
    );

    let mut output = calculator.calculate()?;

    if output.is_empty() {
        return Ok("No rockets found".to_string());
    }
    output.sort_by(|a, b| a.partial_cmp(b).unwrap());

    return Ok(output[0].to_string());
}

#[wasm_bindgen]
#[cfg(target_arch = "wasm32")]
pub fn max_dv(
    mass: f64,
    in_vacuum: bool,
    minimum_twr: f64,
    needs_gimballing: bool,
    use_nosecone: bool,
    unlocked_fuselages: Array,
    unlocked_tech: Array,
    extra_fuel_percentage: f64,
    nose_height: f64,
    use_custom_diameter: bool,
    custom_diameter: f64,
) -> Result<String, JsError> {
    let mut calculator = Calculator::new();
    let unlocked_fuselages: Vec<String> = unlocked_fuselages
        .iter()
        .map(|v| v.as_string().expect("Should be a string"))
        .collect();
    let unlocked_tech: Vec<String> = unlocked_tech
        .iter()
        .map(|v| v.as_string().expect("Should be a string"))
        .collect();
    let unlocked_fuselages: String = unlocked_fuselages.join(",");
    let unlocked_tech: String = unlocked_tech.join(",");
    calculator.prepare_for_max_dv(
        mass,
        in_vacuum,
        minimum_twr,
        needs_gimballing,
        use_nosecone,
        unlocked_fuselages,
        unlocked_tech,
        nose_height,
    );
    let mut result = calculator.max_dv(extra_fuel_percentage, use_custom_diameter, custom_diameter)?;
    result.sort_by(|a, b| a.partial_cmp(&b).unwrap());
    Ok(serde_json::to_string(&result).expect("serde_error"))
}

#[cfg(not(target_arch = "wasm32"))]
pub mod test_utils {}
