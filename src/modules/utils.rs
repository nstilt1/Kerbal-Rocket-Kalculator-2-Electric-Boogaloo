use crate::G;

use super::{
    engines::Engine,
    tanks::{
        cylindrical_tanks::tank_volume,
        nose_tanks::{calculate_corrected_volume, NoseTankCore},
        Fuselage,
    },
};

fn exp_taylor(x: f64, terms: usize) -> f64 {
    let mut sum = 1.0;
    let mut term = 1.0;
    for n in 1..terms {
        term *= x / n as f64;
        sum += term;
    }
    sum
}

pub fn ln(x: f64) -> f64 {
    assert!(x > 0.0, "Natural log is undefined for non-positive numbers");

    let mut guess = x - 1.0;
    for _ in 0..10 {
        let exp_guess = exp_taylor(guess, 50);
        guess -= (exp_guess - x) / exp_guess;
    }
    (guess * 100_000.0).round() / 100_000.0
}

pub fn twr_wet_dry(
    payload_mass_kg: f64,
    engine: &Engine,
    cyl_fuselage: &Fuselage,
    nose_fuselage: &Fuselage,
    cyl_height: f64,
    nose_height: f64,
    nose_core: &NoseTankCore,
    num_engines: u8,
    in_vacuum: bool,
) -> (f64, f64, f64, f64) {
    let engine_mass_kg = engine.mass * 1000.0;
    let diameter = engine.diameter;

    let thrust_n = if in_vacuum {
        engine.thrust_vac
    } else {
        engine.thrust_asl
    } * 1000.0
        * num_engines as f64;
    let tank_volume = tank_volume(diameter, cyl_height);
    let nose_volume =
        calculate_corrected_volume(diameter, nose_height, nose_core.correction_coefficient);
    let tank_structural_mass =
        tank_volume * cyl_fuselage.density * (1.0 - cyl_fuselage.utilization);
    let nose_structural_mass =
        nose_volume * nose_fuselage.density * (1.0 - nose_fuselage.utilization);
    let tank_fuel_volume = tank_volume * cyl_fuselage.utilization;
    let nose_fuel_volume = nose_volume * nose_fuselage.utilization;
    let tank_fuel_mass = tank_fuel_volume * engine.fuel_density();
    let nose_fuel_mass = nose_fuel_volume * engine.fuel_density();
    let dry_mass = payload_mass_kg
        + num_engines as f64 * (tank_structural_mass + nose_structural_mass + engine_mass_kg);
    let wet_mass = dry_mass + num_engines as f64 * (tank_fuel_mass + nose_fuel_mass);

    let twr = thrust_n / wet_mass / G;
    let fuel_volume = nose_fuel_volume + tank_fuel_volume;
    (twr, wet_mass, dry_mass, fuel_volume)
}

/// Converts burn times from minutes + seconds to seconds
pub const fn burn_time_secs(minutes: u32, seconds: f64) -> f64 {
    minutes as f64 * 60.0 + seconds
}
