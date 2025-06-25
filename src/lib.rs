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
    unlocked_fuselages: Array,
    unlocked_tech: Array,
    nose_height: f64,
    use_custom_diameter: bool,
    custom_diameter: f64,
    extra_fuel_percentage: f64,
    use_multiple_engines: bool,
    max_num_engines: u8,
    max_num_tanks: u8,
) -> Result<String, JsError> {
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
        unlocked_fuselages,
        unlocked_tech,
    );

    let mut output = calculator.calculate(
        use_custom_diameter,
        custom_diameter,
        extra_fuel_percentage,
        use_multiple_engines,
        max_num_engines,
        max_num_tanks,
    )?;

    output.sort_by(|a, b| a.partial_cmp(b).unwrap());

    return Ok(serde_json::to_string(&output).expect("serde error"));
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
    use_multiple_engines: bool,
    max_num_engines: u8,
    max_num_tanks: u8,
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
    let mut result = calculator.max_dv(
        extra_fuel_percentage,
        use_custom_diameter,
        custom_diameter,
        use_multiple_engines,
        max_num_engines,
        max_num_tanks,
    )?;
    result.sort_by(|a, b| a.partial_cmp(&b).unwrap());
    let json = serde_json::to_string(&result).expect("serde_error");
    let size_mb = json.len() as f64 / (1024.0 * 1024.0);
    console_log_2!("JSON Size: {} MB", size_mb);
    Ok(json)
}

#[cfg(not(target_arch = "wasm32"))]
pub mod test_utils {}
