//! Module for calculating the volume and wet mass of cyllindrical tanks.

use std::collections::HashMap;

use serde::Serialize;

use crate::{
    debug,
    modules::{engines::Engine, fuel_type::FuelMix, Error},
    G,
};

use super::{
    fuselage_names::*,
    nose_tanks::{calculate_corrected_volume, NoseTankCore},
    Fuselage, Fuselages, TankType, Tanks,
};

#[cfg(test)]
mod densities {
    pub const STEEL_FUSELAGE_DENSITY: f64 = 0.7046255853625588; // kg/L
    pub const STEEL_FUSELAGE_UTIL_PERCENT: f64 = 83.0;
    pub const HP_STEEL_FUSELAGE_DENSITY: f64 = 1.161194204449523; // kg/L
    pub const HP_STEEL_FUSELAGE_UTIL_PERCENT: f64 = 75.0;
    pub const AL_FUSELAGE_DENSITY: f64 = 0.572370017780281; // kg/L
    pub const AL_FUSELAGE_UTIL_PERCENT: f64 = 87.0;
    pub const HP_AL_FUSELAGE_DENSITY: f64 = 1.6233800555626554; // kg/L
    pub const HP_AL_FUSELAGE_UTIL_PERCENT: f64 = 84.0;
    pub const AL_STRINGER_TANK_DENSITY: f64 = 0.6474421633361649; // kg/L
    pub const AL_STRINGER_TANK_UTIL_PERCENT: f64 = 92.0;
    pub const HP_AL_STRINGER_TANK_DENSITY: f64 = 2.1543208266760887; // kg/L
    pub const HP_AL_STRINGER_TANK_UTIL_PERCENT: f64 = 90.0;
    pub const REFINED_AL_STRINGER_TANK_DENSITY: f64 = 0.4793745811132077; // kg/L
    pub const REFINED_AL_STRINGER_TANK_UTIL_PERCENT: f64 = 92.0;
    pub const HP_REFINED_AL_STRINGER_TANK_DENSITY: f64 = 1.6195603377848609; // kg/L
    pub const HP_REFINED_AL_STRINGER_TANK_UTIL_PERCENT: f64 = 90.0;
    pub const AL_LI_STRINGER_TANK_DENSITY: f64 = 1.0134984503748028; // kg/L
    pub const AL_LI_STRINGER_TANK_UTIL_PERCENT: f64 = 97.0;
    pub const HP_AL_LI_STRINGER_TANK_DENSITY: f64 = 2.3147489733434568; // kg/L
    pub const HP_AL_LI_STRINGER_TANK_UTIL_PERCENT: f64 = 96.0;
    pub const REFINED_AL_LI_STRINGER_TANK_DENSITY: f64 = 0.9523829659300911; // kg/L
    pub const REFINED_AL_LI_STRINGER_TANK_UTIL_PERCENT: f64 = 97.0;
    pub const HP_REFINED_AL_LI_STRINGER_TANK_DENSITY: f64 = 1.9977123977865145; // kg/L
    pub const HP_REFINED_AL_LI_STRINGER_TANK_UTIL_PERCENT: f64 = 96.0;
    pub const STEEL_STIR_WELDED_TANK_DENSITY: f64 = 1.1815660325977602; // kg/L
    pub const STEEL_STIR_WELDED_TANK_UTIL_PERCENT: f64 = 97.0;
    pub const HP_STEEL_STIR_WELDED_TANK_DENSITY: f64 = 3.4033685400148843; // kg/L
    pub const HP_STEEL_STIR_WELDED_TANK_UTIL_PERCENT: f64 = 96.0;
}

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct CylindricalTank {
    pub length: f64,
    pub diameter: f64,
    pub fuselage: Fuselage,
}

impl Tanks for CylindricalTank {
    /// MIN_VSA Height ratio. min height = 1/2 diameter
    const MIN_VSA: f64 = 0.5;
    const MAX_VSA: f64 = 50.0;
    const TANK_TYPE: TankType = TankType::Cylindrical;

    fn init_fuselage_types() -> Fuselages {
        let mut hp_tanks: HashMap<&str, Fuselage> = HashMap::with_capacity(7);
        let mut non_hp_tanks: HashMap<&str, Fuselage> = HashMap::with_capacity(7);
        hp_tanks.insert(
            HP_STEEL_FUSELAGE_NAME,
            Fuselage::new(HP_STEEL_FUSELAGE_NAME, 1.161194204449523, 0.75),
        );
        hp_tanks.insert(
            HP_AL_FUSELAGE_NAME,
            Fuselage::new(HP_AL_FUSELAGE_NAME, 1.6233800555626554, 0.84),
        );
        hp_tanks.insert(
            HP_AL_STRINGER_TANK_NAME,
            Fuselage::new(HP_AL_STRINGER_TANK_NAME, 2.1543208266760887, 0.90),
        );
        hp_tanks.insert(
            HP_REFINED_AL_STRINGER_TANK_NAME,
            Fuselage::new(HP_REFINED_AL_STRINGER_TANK_NAME, 1.6195603377848609, 0.90),
        );
        hp_tanks.insert(
            HP_AL_LI_STRINGER_TANK_NAME,
            Fuselage::new(HP_AL_LI_STRINGER_TANK_NAME, 2.3147489733434568, 0.96),
        );
        hp_tanks.insert(
            HP_REFINED_AL_LI_STRINGER_TANK_NAME,
            Fuselage::new(
                HP_REFINED_AL_LI_STRINGER_TANK_NAME,
                1.9977123977865145,
                0.96,
            ),
        );
        hp_tanks.insert(
            HP_STEEL_STIR_WELDED_TANK_NAME,
            Fuselage::new(HP_STEEL_STIR_WELDED_TANK_NAME, 3.4033685400148843, 0.96),
        );
        non_hp_tanks.insert(
            STEEL_FUSELAGE_NAME,
            Fuselage::new(STEEL_FUSELAGE_NAME, 0.7046255853625588, 0.83),
        );
        non_hp_tanks.insert(
            AL_FUSELAGE_NAME,
            Fuselage::new(AL_FUSELAGE_NAME, 0.572370017780281, 0.87),
        );
        non_hp_tanks.insert(
            AL_STRINGER_TANK_NAME,
            Fuselage::new(AL_STRINGER_TANK_NAME, 0.6474421633361649, 0.92),
        );
        non_hp_tanks.insert(
            REFINED_AL_STRINGER_TANK_NAME,
            Fuselage::new(REFINED_AL_STRINGER_TANK_NAME, 0.4793745811132077, 0.92),
        );
        non_hp_tanks.insert(
            AL_LI_STRINGER_TANK_NAME,
            Fuselage::new(AL_LI_STRINGER_TANK_NAME, 1.0134984503748028, 0.97),
        );
        non_hp_tanks.insert(
            REFINED_AL_LI_STRINGER_TANK_NAME,
            Fuselage::new(REFINED_AL_LI_STRINGER_TANK_NAME, 0.9523829659300911, 0.97),
        );
        non_hp_tanks.insert(
            STEEL_STIR_WELDED_TANK_NAME,
            Fuselage::new(STEEL_STIR_WELDED_TANK_NAME, 1.1815660325977602, 0.97),
        );
        Fuselages::new(hp_tanks, non_hp_tanks)
    }
}

/// Calculates the volume of a cylinder.
fn cylinder_volume(radius: f64, height: f64) -> f64 {
    std::f64::consts::PI * radius * radius * height
}
/// Calculates the volume of an ellipsoid.
pub fn ellipsoid_volume(a: f64, b: f64, c: f64) -> f64 {
    4.0 / 3.0 * std::f64::consts::PI * (a * b * c)
}

pub const K: f64 = 392.69893495497905;
//const N: f64 = 3.0000008452405535;

/// Calculates the volume of a tank with no nose or mount. Applies a correction
/// factor discovered during testing.
pub fn tank_volume(diameter: f64, height: f64) -> f64 {
    let r = diameter / 2.0;
    // (ellipsoid_volume(r, r, r/2.0) + cylinder_volume(r, height)) * 1000.0 - 392.6991174
    let base_volume = (ellipsoid_volume(r, r, r / 2.0) + cylinder_volume(r, height)) * 1000.0;
    let correction_factor = K * diameter.powi(3);
    base_volume - correction_factor
}

pub fn tank_height_given_max_volume(
    max_total_volume: f64,
    nose_height: f64,
    nose_fuselage: &Fuselage,
    nose_core: &NoseTankCore,
    cyl_fuselage: &Fuselage,
    engine: &Engine,
    num_tanks: u8,
    payload_mass: f64,
    in_vacuum: bool,
) -> Result<(f64, f64, f64, f64), Error> {
    let n = num_tanks as f64;
    let d = engine.size.get_diameter();
    let r = d / 2.0;
    let nose_volume = calculate_corrected_volume(d, nose_height, nose_core.correction_coefficient)
        * nose_fuselage.utilization;
    let max_volume = (max_total_volume / cyl_fuselage.utilization) - nose_volume;
    if max_volume < 0.0 {
        debug!(
            "\t\tError: Invalid max_volume in tank_height_given_max_volume: {}",
            max_volume
        );
        return Err(Error::InvalidHeight);
    }
    let fuel_density = engine.fuel_mix.density(engine.hp_fuel);
    // max_volume = tank_volume(d, h) * u
    // max_volume = 1000 * (ellipsoid_volume + cylinder_volume(r, h)) - correction_factor
    // (max_volume + correction_factor)/1000 = ellipsoid_volume + cylinder_volume(r,h)
    // (max_volume + correction_factor)/1000 - ellipsoid_volume = cylinder_volume(r,h)
    // n_1 = (max_volume + correction_factor) / 1000 - ellipsoid_volume
    // n_1 / (PI * r * r) = h
    let n_1 = (max_volume + K * d * d * d) / 1000.0 - ellipsoid_volume(r, r, r / 2.0);
    let h = n_1 / (std::f64::consts::PI * r * r);

    let tank_volume = tank_volume(d, h);
    let dry_mass = payload_mass
        + n * nose_volume * nose_fuselage.density * (1.0 - nose_fuselage.utilization)
        + n * tank_volume * cyl_fuselage.density * (1.0 - cyl_fuselage.utilization);
    let wet_mass = dry_mass
        + n * fuel_density
            * (nose_volume * nose_fuselage.utilization + tank_volume * cyl_fuselage.utilization);
    let thrust_n = if in_vacuum {
        engine.thrust_vac
    } else {
        engine.thrust_asl
    } * 1000.0
        * n;
    let twr = thrust_n / wet_mass / G;
    debug_assert!(
        (tank_volume * cyl_fuselage.utilization - max_total_volume).abs() < 0.001,
        "Tank volume was off. tank_volume * utilization = {}\nmax_total_volume = {}",
        tank_volume * cyl_fuselage.utilization,
        max_total_volume
    );
    Ok((h, twr, wet_mass, dry_mass))
}

/// Calculates the dry mass of a cylindrical tank in kg.
pub fn cylindrical_dry_mass(diameter: f64, height: f64, utilization: f64, density: f64) -> f64 {
    let volume = tank_volume(diameter, height);
    volume * (1.0 - utilization) * density
}

/// Computes the tank height required to reach a twr.
pub fn compute_tank_height(
    min_twr: f64,
    engine: &Engine,
    fuselage: &Fuselage,
    payload_mass: f64,
    num_tanks: u8,
    in_vacuum: bool,
) -> Result<(f64, f64, f64, f64, f64), Error> {
    // increase min_twr by a small amount to ensure that the output has that much
    // twr
    //let min_twr = min_twr + 0.1;
    let num_engines = num_tanks as f64;
    let num_tanks = num_tanks as f64;
    debug!("num_engines = {}", num_engines);
    debug_assert!(num_engines == num_tanks);

    let thrust_total = if in_vacuum {
        engine.thrust_vac
    } else {
        engine.thrust_asl
    } * num_engines
        * 1000.0;
    let max_wet_mass = thrust_total / (min_twr * G);
    debug!("Target wet mass = {}", max_wet_mass);
    //debug!("thrust_total = {}", thrust_total);
    //debug!("Engine.mass = {}", engine.mass);
    let diameter = engine.size.get_diameter();
    let r = diameter / 2.0;
    let engine_mass = engine.mass * 1000.0 * num_engines as f64;

    // twr = thrust_n / wet_mass / G
    // target_wet_mass = thrust_n / target_twr / G
    // wet_mass = payload_mass + engine_mass + fuel_mass + structural_mass
    // structural_mass + fuel_mass = wet_mass - payload_mass - engine_mass
    // structural_mass = tank_volume(diameter, h) * structural_density * unusable
    // fuel_mass = tank_volume(diameter, h) * fuel_density * usable
    // tank_volume(diameter, h) * (structural_density * unusableFraction + fuel_density * usableFraction) = wet_mass - payload_mass - engine_mass
    // tank_volume(diameter, h) = (wet_mass - payload_mass - engine_mass) / (structural_density * unusableFraction + fuel_density * usableFraction)

    // tank_volume(diameter, h) = ellipsoid_volume(diameter/2, diameter/2, diameter/4) + cylinders_volume(r, h) * 1000.0 - K * diameter.powi(3)
    // tank_volume(d, h) + K * diameter.powi(3) - ellipsoid_volume() = cylinders_volume(r, h)
    // cylinders_volume = tank_volume(d, h) + K * diameter.powi(3) - ellipsoid_volume()
    // num_tanks * PI * radius * radius * height = tank_volume(d, h) + K * diameter.powi(3) - ellipsoid_volume()
    // h = (tank_volume(d,h) + K * diameter.powi(3) - ellipsoid_volume())/(num_tanks*PI*radius*radius)
    // h = (((wet_mass - payload_mass - engine_mass) / (structural_density * unusable + fuel_density * usable)) + K * diameter.powi(3) - ellipsoid_volume())/(num_tanks * PI * radius * radius)

    // unsure if the following is usable. probably not since fuel_mass and
    // structural mass are functions of height:
    // h = (((fuel_mass + structural_mass) / (structural_density * unusable + fuel_density * usable)) + K * diameter.powi(3) - ellipsoid_volume())/(PI * radius * radius)
    // let n1 = max_wet_mass - payload_mass - engine_mass;
    //assert!(max_wet_mass > payload_mass + engine_mass);

    #[allow(unused_doc_comments)]
    /// Derivation attempt 2:
    ///
    /// Given:
    /// twr = thrust / wet_mass / g
    /// wet_mass = payload_mass + engine_mass + fuel_mass + structural_mass
    /// fuel_mass = tank_volume(h) * fuel_density * util
    /// tank_volume(h) = ellipsoid_volume + cyl_volume(h) - K*d^3
    /// cyl_volume(h) = pi*r*r*h
    /// structural_volume = tank_volume(h) * structural_density * (1-util)
    ///
    /// Derived:
    /// wet_mass = thrust / twr / g
    /// numerator_1 = wet_mass - payload_mass - engine_mass
    /// fuel_mass + structural_mass = numerator_1
    /// tank_volume(h) * (fuel_density * util + structural_density * (1-util)) = numerator_1
    /// denominator_1 = fuel_density * util + structural_density * (1 - util)
    /// tank_volume(h) = numerator_1 / denominator_1
    /// ellipsoid_volume + cyl_volume(h) - Kd^3 = numerator_1 / denominator_1
    /// cyl_volume(h) = numerator_1 / denominator_1 - ellipsoid_volume + Kd^3
    /// pi*r^2*h = numerator_1 / denominator_1 - ellipsoid_volume + Kd^3
    /// h = (num_1 / den_1 - ellipsoid_volume + Kd^3) / (pi*r*r)
    {
        if max_wet_mass <= payload_mass + engine_mass {
            return Err(Error::MaxWetMassBelowCurrentMass);
        }
    }
    // convert metric tons to kg
    let payload_mass = payload_mass * 1000.0;
    let engine_mass_total = engine.mass * 1000.0 * num_engines;
    debug!("engine_mass_total = {} kg", engine_mass_total);

    // convert kg/L to kg/m^3
    let fuel_density = engine.fuel_mix.density(engine.hp_fuel) * 1000.0;
    let structural_density = fuselage.density * 1000.0;
    let utilization = fuselage.utilization;
    debug_assert!(
        utilization < 1.0,
        "fuselage.utilization was not less than 1.0"
    );

    let mass_contribution = structural_density * (1.0 - utilization) + fuel_density * utilization;
    debug!("Total mass contribution factor: {}", mass_contribution);

    let correction_factor_total = (K * diameter * diameter * diameter * num_tanks) * 0.001;
    let ellipsoid_volume_total = num_tanks * ellipsoid_volume(r, r, r / 2.0);

    debug!("Correction factor: {}", correction_factor_total);
    debug!(
        "Structural density * 1.0 - usable_fraction = {}",
        structural_density * (1.0 - utilization)
    );
    debug!(
        "Fuel density * usable fraction = {}",
        fuel_density * utilization
    );

    /*
    let h = (max_wet_mass - payload_mass - engine_mass_total)
        / ((mass_contribution * num_tanks) + correction_factor_total - ellipsoid_volume_total)
        / (std::f64::consts::PI * r * r);
    */
    let numerator_1 = max_wet_mass - payload_mass - engine_mass_total;
    let denominator_1 = mass_contribution;
    let numerator_2 =
        (numerator_1 / denominator_1) - ellipsoid_volume_total + correction_factor_total;
    let denominator_2 = std::f64::consts::PI * r * r * num_tanks;
    let mut h = numerator_2 / denominator_2;

    #[cfg(test)]
    {
        debug!("Computed height: {} m", h);
        let structural_mass =
            structural_density * 0.001 * (1.0 - utilization) * tank_volume(diameter, h) * num_tanks;
        let fuel_mass = fuel_density * utilization * tank_volume(diameter, h) * 0.001 * num_tanks;
        debug!("Fuel mass inside fn: {} kg", fuel_mass);
        debug!("Structural mass inside fn: {} kg", structural_mass);
        debug!(
            "Wet mass inside fn: {} kg",
            payload_mass + engine_mass_total + fuel_mass + structural_mass
        );
    }
    if h > 50.0 {
        h = 50.0;
    }
    if h <= 0.0 {
        return Err(Error::InvalidHeight);
    }
    if h.is_nan() || h.is_infinite() {
        return Err(Error::InvalidHeight);
    }
    let tank_volume = tank_volume(diameter, h);
    let dry_mass = payload_mass
        + engine_mass_total
        + fuselage.density * tank_volume * (1.0 - fuselage.utilization) * num_tanks;
    let wet_mass = dry_mass
        + tank_volume * fuselage.utilization * engine.fuel_mix.density(engine.hp_fuel) * num_tanks;
    let thrust_n = if in_vacuum {
        engine.thrust_vac
    } else {
        engine.thrust_asl
    } * 1000.0
        * num_engines;
    let twr = thrust_n / wet_mass / G;
    Ok((
        h,
        twr,
        wet_mass,
        dry_mass,
        tank_volume * fuselage.utilization,
    ))
}
/// Computes the tank height required to reach a specific delta-v
///
/// ```norun
/// Given:
/// wet_mass = payload_mass + engine_mass + fuel_mass + structural_mass
/// dry_mass = payload_mass + engine_mass + structural_mass
/// fuel_mass = tank_volume(h) * fuel_density * util
/// structural_mass = tank_volume(h) * structural_density * (1 - util)
/// tank_volume(h) = ellipsoid_volume + cyl_volume(h) - K*d^3
/// cyl_volume(h) = pi*r^2*h
/// dv = Isp * g * ln(wet_mass/dry_mass)
///
/// Derivation:
/// dv/(Isp * g) = ln(wet_mass/dry_mass)
/// e^(dv/Isp/g) = wet_mass/dry_mass
/// e0 = e^(dv/Isp/g)
/// e0 = (payload_mass + engine_mass + fuel_mass + structural_mass)/(payload_mass + engine_mass + structural_mass)
/// e0 * (payload_mass + engine_mass + structural_mass) = payload_mass + engine_mass + fuel_mass + structural_mass
/// e0*payload_mass + e0*engine_mass + e0 * structural_mass = payload_mass + engine_mass + fuel_mass + structural_mass
/// e0*structural_mass - structural_mass - fuel_mass = payload_mass + engine_mass - e0*payload_mass - e0*engine_mass
/// structural_mass * (e0 - 1) - fuel_mass = (payload_mass + engine_mass) * (1 - e0)
/// num_1 = (payload_mass + engine_mass) * (1 - e0)
/// (tank_volume(h) * structural_density * (1-util))*(e0 - 1) - tank_volume(h) * fuel_density * util = num_1
/// tank_volume(h) * (structural_density * (1-util)*(e0-1) - fuel_density * util) = num_1
/// tank_volume(h) = num_1 / (structural_density * (1-util) * (e0 - 1) - fuel_density * util)
/// den_1 = structural_density * (1-util) * (e0 - 1) - fuel_density * util
/// ellipsoid_volume + cyl_volume(h) - K*d^3 = num_1 / den_1
/// cyl_volume(h) = num_1 / den_1 - ellipsoid_volume + K*d^3
/// pi*r*r*h = num_1 / den_1 - ellipsoid_volume + K * d^3
/// den_2 = pi * r * r
/// h = (num_1 / den_1 - ellipsoid_volume + K * d^3)/den_2
/// ```
pub fn compute_tank_height_for_delta_v(
    target_dv: f64,
    engine: &Engine,
    fuselage: &Fuselage,
    payload_mass: f64,
    num_tanks: u8,
    in_vacuum: bool,
) -> Result<(f64, f64, f64, f64), Error> {
    let isp = if in_vacuum {
        engine.isp_vac
    } else {
        engine.isp_asl
    };
    let engine_mass = engine.mass * 1000.0 * num_tanks as f64;
    let payload_mass = payload_mass * 1000.0;
    let e0 = std::f64::consts::E.powf(target_dv / (isp * G));
    //let e0 = f64::exp(target_dv / (isp * g));
    let d = engine.size.get_diameter();
    let r = d / 2.0;
    let ellipsoid_volume_total = ellipsoid_volume(r, r, r / 2.0) * num_tanks as f64;
    let correction_total = (K * d * d * d * num_tanks as f64) * 0.001;

    let fuel_density = engine.fuel_mix.density(engine.hp_fuel) * 1000.0;
    let structural_density = fuselage.density * 1000.0;

    let num_1 = (payload_mass + engine_mass) * (1.0 - e0);
    let den_1 = structural_density * (1.0 - fuselage.utilization) * (e0 - 1.0)
        - fuel_density * fuselage.utilization;
    let num_2 = num_1 / den_1 - ellipsoid_volume_total + correction_total;
    let den_2 = std::f64::consts::PI * r * r * num_tanks as f64;
    let h = num_2 / den_2;
    if h > 50.0 {
        return Err(Error::HeightTooLarge);
    }
    if h < 0.0 || h.is_nan() || h.is_infinite() {
        return Err(Error::InvalidHeight);
    }
    let volume = tank_volume(d, h) * num_tanks as f64;
    let dry_mass =
        payload_mass + engine_mass + volume * fuselage.density * (1.0 - fuselage.utilization);
    let wet_mass =
        dry_mass + volume * engine.fuel_mix.density(engine.hp_fuel) * fuselage.utilization;
    let thrust_n = if in_vacuum {
        engine.thrust_vac
    } else {
        engine.thrust_asl
    } * 1000.0
        * num_tanks as f64;
    let twr = thrust_n / wet_mass / G;
    Ok((h, twr, wet_mass, dry_mass))
}

#[cfg(test)]
mod tests {
    use crate::modules::engines::{Engine, ENGINES};
    use crate::modules::tanks::nose_tanks::NoseConeVariant;
    use crate::G;

    use super::densities::*;
    use super::*;

    #[test]
    fn max_tank_height_given_max_volume() {
        let max_total_volume = 139.465;
        let nose_height = 0.0;
        let nose_fuselages = NoseConeVariant::init_fuselage_types();
        let nose_fuselage = nose_fuselages
            .hp_fuselages
            .get(format!("HP {}", STEEL_FUSELAGE_NAME).as_str())
            .unwrap();
        let nose_cores = NoseConeVariant::nosecones();
        let nose_core = nose_cores.cores[0];
        let cyl_fuselages = CylindricalTank::init_fuselage_types();
        let cyl_fuselage = cyl_fuselages
            .hp_fuselages
            .get(format!("HP {}", STEEL_FUSELAGE_NAME).as_str())
            .unwrap();
        let engine = ENGINES[4];
        let num_tanks = 1;
        let payload_mass = 0.087;
        let in_vacuum = false;

        let (h, _twr, _wet, _dry) = tank_height_given_max_volume(
            max_total_volume,
            nose_height,
            nose_fuselage,
            &nose_core,
            cyl_fuselage,
            &engine,
            num_tanks,
            payload_mass,
            in_vacuum,
        )
        .unwrap();
        let v = tank_volume(engine.size.get_diameter(), h) * cyl_fuselage.utilization;
        let diff = v - max_total_volume;
        assert!(diff.abs() < 0.0001);
    }

    #[test]
    fn tank_height_delta_v_tests() {
        let payload_mass_tons = 0.4;
        let engines = ENGINES;
        let engine = engines[0];
        let diameter = engine.size.get_diameter();
        let fuselages = CylindricalTank::init_fuselage_types();
        let fuselage = fuselages.non_hp_fuselages.get(STEEL_FUSELAGE_NAME).unwrap();
        let target_dv = 789.123;
        let num_tanks = 3;
        let (h, twr, _wet_mass, _dry_mass) = compute_tank_height_for_delta_v(
            target_dv,
            &engine,
            fuselage,
            payload_mass_tons,
            num_tanks,
            false,
        )
        .unwrap();

        let volume = tank_volume(diameter, h) * num_tanks as f64;
        let structural_mass = volume * fuselage.density * (1.0 - fuselage.utilization);

        let dry_mass =
            payload_mass_tons * 1000.0 + engine.mass * 1000.0 * num_tanks as f64 + structural_mass;
        let wet_mass =
            dry_mass + volume * engine.fuel_mix.density(engine.hp_fuel) * fuselage.utilization;
        let delta_v = engine.isp_asl * G * f64::ln(wet_mass / dry_mass);
        let diff = target_dv - delta_v;
        assert!(diff.abs() < 0.00001);

        let thrust_n = engine.thrust_asl * 1000.0 * num_tanks as f64;
        let expected_twr = thrust_n / wet_mass / G;
        assert_eq!(twr, expected_twr);
    }

    #[test]
    fn tank_height_tests() {
        let payload_mass_tons = 0.1;
        let payload_mass_kg = payload_mass_tons * 1000.0;
        let engines = Engine::init_rp1_engines();
        let engine = engines[0];
        let engine_mass_kg = engine.mass * 1000.0;
        //let engine_mass_tons = engine.mass;
        let thrust_n = engine.thrust_asl * 1000.0;
        let target_twr = 3.8;
        let diameter = engine.size.get_diameter();
        let fuselage_types = super::CylindricalTank::init_fuselage_types();
        let non_hp_fuselages = fuselage_types.non_hp_fuselages;
        let fuselage = non_hp_fuselages.get(STEEL_FUSELAGE_NAME).unwrap();
        let (h, twr, _wet_mass, _dry_mass, _volume) = compute_tank_height(
            target_twr,
            &engines[0],
            &fuselage,
            payload_mass_tons,
            1,
            false,
        )
        .unwrap();

        let volume = tank_volume(diameter, h);
        let mut wet_mass = payload_mass_kg + engine_mass_kg;
        wet_mass += engines[0]
            .fuel_mix
            .mass(volume * fuselage.utilization, engine.hp_fuel);
        wet_mass += volume * (1.0 - fuselage.utilization) * fuselage.density;
        assert!(fuselage.utilization < 1.0);
        assert!(engine.mass < 1.0);
        let twr = thrust_n / wet_mass / G;

        //println!("Wet mass = {}", wet_mass);
        assert_eq!(
            (twr - target_twr).abs() < 0.105,
            true,
            "TWR was outside the range"
        );
        //println!("TWR = {}\nTarget TWR = {}", twr, target_twr);

        for num_tanks in 2..=9 {
            println!();
            let (h, twr, _wet_mass, _dry_mass, _volume) = compute_tank_height(
                target_twr,
                &engines[0],
                &fuselage,
                payload_mass_tons,
                num_tanks,
                false,
            )
            .unwrap();

            let volume = tank_volume(diameter, h) * num_tanks as f64;
            let mut wet_mass = payload_mass_kg + engine_mass_kg * num_tanks as f64;
            let fuel_mass = engines[0]
                .fuel_mix
                .mass(volume * fuselage.utilization, engine.hp_fuel);
            //println!("Fuel mass in test: {} kg", fuel_mass);
            wet_mass += fuel_mass;
            let structural_mass = volume * (1.0 - fuselage.utilization) * fuselage.density;
            wet_mass += structural_mass;
            let expected_twr = thrust_n * num_tanks as f64 / wet_mass / G;

            //println!("Wet mass in test = {} kg", wet_mass);
            //println!("Structural mass in test = {} kg", structural_mass);
            assert!(
                (expected_twr - target_twr).abs() < 0.105,
                "Failed on num_tanks={}\ntwr = {}\ntarget = {}",
                num_tanks,
                twr,
                target_twr
            );
            let diff = expected_twr - twr;
            assert!(diff.abs() < 0.00001);
            //println!("TWR: {}\nTarget TWR: {}", twr, target_twr);
        }
    }

    #[test]
    fn tank_height_graphs() {
        const IN_VACUUM: bool = false;

        let engines = ENGINES;
        let fuselage_types = CylindricalTank::init_fuselage_types();
        let fuselage = fuselage_types
            .non_hp_fuselages
            .get(STEEL_FUSELAGE_NAME)
            .unwrap();

        const MAX_NUM_TANKS: u8 = 9;
        const MAX_PAYLOAD_KG: usize = 400;
        for engine in engines {
            let diameter = &engine.size.get_diameter();
            let engine_mass_kg = engine.mass * 1000.0;
            let thrust_n = if IN_VACUUM {
                engine.thrust_vac
            } else {
                engine.thrust_asl
            } * 1000.0;
            let mut twr_errors_payloads =
                [[(0f64, 0f64, 0f64, 0f64); MAX_NUM_TANKS as usize]; MAX_PAYLOAD_KG];
            let payload_masses_kg: [usize; 4] = [250, 300, 400, 500];
            for (i, payload_mass_kg) in payload_masses_kg.iter().enumerate() {
                let payload_mass_tons = *payload_mass_kg as f64 / 1000.0;
                let mut twr_errors = [(0f64, 0f64, 0f64, 0f64); MAX_NUM_TANKS as usize];
                for num_tanks in 1..=MAX_NUM_TANKS {
                    // error increases a lot when TWR_MIN = 1. Max error was at about 1.2-1.4. Probably because the height was capped to 50. Will eliminate any results where h = 50.0
                    const TWR_MIN: usize = 1;
                    const TWR_MAX: usize = 50;

                    let mut min_error = 1.0;
                    let mut max_error = -1.0;
                    let mut sum_error = 0.0;
                    let mut abs_sum_error = 0.0;
                    let mut valid_samples = 0;
                    let mut twr_errors_inner: [Option<f64>; TWR_MAX] = [None; TWR_MAX];
                    for target_twr_index in TWR_MIN..=TWR_MAX {
                        let target_twr = target_twr_index as f64 / 10.0;

                        let h = compute_tank_height(
                            target_twr,
                            &engine,
                            fuselage,
                            payload_mass_tons,
                            num_tanks,
                            IN_VACUUM,
                        );
                        if let Ok((h, twr, wet_mass, dry_mass, _volume)) = h {
                            if h == 50.0 {
                                twr_errors_inner[target_twr_index - 1] = None;
                                continue;
                            }
                            valid_samples += 1;
                            let volume = tank_volume(*diameter, h) * num_tanks as f64;
                            let mut wet_mass =
                                *payload_mass_kg as f64 + engine_mass_kg * num_tanks as f64;
                            let fuel_mass = engine
                                .fuel_mix
                                .mass(volume * fuselage.utilization, engine.hp_fuel);
                            wet_mass += fuel_mass;
                            let structural_mass =
                                volume * (1.0 - fuselage.utilization) * fuselage.density;
                            wet_mass += structural_mass;
                            let twr = thrust_n * num_tanks as f64 / wet_mass / G;
                            let ratio = twr / target_twr;
                            let percent_difference = ratio - 1.0;
                            twr_errors_inner[target_twr_index - 1] = Some(percent_difference);
                            if min_error > percent_difference {
                                min_error = percent_difference;
                            } else if max_error < percent_difference {
                                max_error = percent_difference;
                            }
                            sum_error += percent_difference;
                            abs_sum_error += percent_difference.abs();
                        }
                    }
                    draw_twr_error_chart_inner(
                        *payload_mass_kg,
                        engine_mass_kg * num_tanks as f64,
                        thrust_n * num_tanks as f64,
                        &twr_errors_inner,
                        &format!(
                            "height_chart_errors/inner/{}/{}_engines_{}_payload_mass.html",
                            engine.name, num_tanks, payload_mass_kg
                        ),
                    )
                    .unwrap();
                    twr_errors[num_tanks as usize - 1] = (
                        min_error,
                        max_error,
                        sum_error / valid_samples as f64,
                        abs_sum_error / valid_samples as f64,
                    );
                }
                draw_twr_error_chart_html(
                    *payload_mass_kg,
                    &twr_errors,
                    &format!(
                        "height_chart_errors/{}/payload_mass_{}.html",
                        engine.name, payload_mass_kg
                    ),
                )
                .unwrap();
                twr_errors_payloads[i] = twr_errors;
            }
        }
    }

    use charming::{
        component::{Axis, Legend, Title},
        element::{AxisType, LineStyle, Symbol},
        series::Line,
        Chart, HtmlRenderer,
    };
    use std::fs;

    fn draw_twr_error_chart_html(
        payload_mass_kg: usize,
        twr_errors: &[(f64, f64, f64, f64)],
        output_path: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let x_data: Vec<String> = (1..=twr_errors.len()).map(|n| n.to_string()).collect();
        let mut min_errors = vec![];
        let mut max_errors = vec![];
        let mut avg_errors = vec![];
        let mut abs_avg_errors = vec![];
        for &(min, max, avg, abs_avg) in twr_errors {
            min_errors.push(min);
            max_errors.push(max);
            avg_errors.push(avg);
            abs_avg_errors.push(abs_avg);
        }

        let chart = Chart::new()
            .title(Title::new().text(format!("TWR Errors for Payload {} kg", payload_mass_kg)))
            .legend(Legend::new())
            .x_axis(Axis::new().type_(AxisType::Category).data(x_data))
            .y_axis(Axis::new().min(-0.1).max(0.01))
            .series(
                Line::new()
                    .name("min_error")
                    .data(min_errors)
                    .symbol(Symbol::None)
                    .line_style(LineStyle::new().width(2)),
            )
            .series(
                Line::new()
                    .name("max_error")
                    .data(max_errors)
                    .symbol(Symbol::None)
                    .line_style(LineStyle::new().width(2)),
            )
            .series(
                Line::new()
                    .name("avg_error")
                    .data(avg_errors)
                    .symbol(Symbol::None)
                    .line_style(LineStyle::new().width(2)),
            );
        // .series(Line::new()
        //     .name("abs_avg_error")
        //     .data(abs_avg_errors)
        //     .symbol(Symbol::None)
        //     .line_style(LineStyle::new().width(2)));

        let html = HtmlRenderer::new("chart", 800, 600).render(&chart)?;
        fs::create_dir_all(std::path::Path::new(output_path).parent().unwrap()).unwrap();
        fs::write(output_path, html)?;
        Ok(())
    }

    fn draw_twr_error_chart_inner(
        payload_mass_kg: usize,
        engine_mass: f64,
        engine_thrust: f64,
        twr_errors: &[Option<f64>],
        output_path: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let x_data: Vec<String> = (1..=twr_errors.len())
            .map(|n| (n as f64 / 10.0).to_string())
            .collect();
        let mut errors = vec![];
        let mut first_error = f64::NAN;
        let mut first_err_index = 0;
        for (i, err) in twr_errors.iter().enumerate() {
            errors.push(*err);
            if first_error.is_nan() {
                if let Some(err) = err {
                    first_error = *err;
                    first_err_index = i;
                }
            }
        }
        let mut last_err = f64::NAN;
        let mut last_err_index = twr_errors.len() + 1;
        for (i, err) in twr_errors.iter().enumerate().rev() {
            if last_err.is_nan() {
                if let Some(err) = err {
                    last_err = *err;
                    last_err_index = i;
                }
            }
        }
        let rise = last_err - first_error;
        let run = (last_err_index - first_err_index) as f64 * 0.1;
        let slope = rise / run;
        //let normalized_slope = slope / engine_mass;
        //let normalized_slope = slope / engine_thrust;
        //println!("\tError % slope = {} % / twr", slope);

        let chart = Chart::new()
            .title(Title::new().text(format!("TWR Errors for Payload {} kg", payload_mass_kg)))
            .legend(Legend::new())
            .x_axis(
                Axis::new()
                    .type_(AxisType::Category)
                    .data(x_data)
                    .min(0.0)
                    .max(50.0),
            )
            .y_axis(Axis::new().min(-0.01).max(0.09))
            .series(
                Line::new()
                    .name("error %")
                    .data(errors)
                    .symbol(Symbol::None)
                    .line_style(LineStyle::new().width(2)),
            );
        // .series(Line::new()
        //     .name("abs_avg_error")
        //     .data(abs_avg_errors)
        //     .symbol(Symbol::None)
        //     .line_style(LineStyle::new().width(2)));

        let html = HtmlRenderer::new(output_path, 800, 600).render(&chart)?;
        fs::create_dir_all(std::path::Path::new(output_path).parent().unwrap()).unwrap();
        fs::write(output_path, html)?;
        Ok(())
    }

    // using a const since you can't easily pass arguments to `cargo test`
    const PRINT_STATS: bool = true;

    fn check_samples(samples: &[(f64, f64, f64)]) {
        println!("Checking samples with diameter {:.3}", samples[0].0);
        for (i, &(diameter, height, volume)) in samples.iter().enumerate() {
            let volume = volume / 83.0 * 100.0;
            let estimate = tank_volume(diameter, height);
            println!(
                "Sample {} - Estimate: {:.5} Actual: {:.5} Error: {:.10}",
                i,
                estimate,
                volume,
                volume - estimate
            );
        }
    }

    /// Calculates correction parameters for cylindrical tanks. Applied as:
    ///
    /// correction_factor = k * diameter^n
    fn estimate_correction_params(samples: &[(f64, f64)]) -> (f64, f64) {
        let mut sum_log_d = 0.0;
        let mut sum_log_error = 0.0;
        let mut sum_log_d_sq = 0.0;
        let mut sum_log_d_log_error = 0.0;
        let n_samples = samples.len() as f64;

        for &(diameter, error) in samples.iter() {
            let log_d = diameter.ln();
            let log_error = error.abs().ln(); // Use absolute value to avoid log of negative numbers

            sum_log_d += log_d;
            sum_log_error += log_error;
            sum_log_d_sq += log_d * log_d;
            sum_log_d_log_error += log_d * log_error;
        }

        let n = (n_samples * sum_log_d_log_error - sum_log_d * sum_log_error)
            / (n_samples * sum_log_d_sq - sum_log_d * sum_log_d);

        let k = (sum_log_error - n * sum_log_d) / n_samples;

        (k.exp(), n) // Convert back from logarithmic space
    }

    #[test]
    fn const_tuning() {
        let samples = &[
            (0.5, -49.0873265169),
            (1.0, -392.6991204490),
            (2.0, -3141.5925782475),
        ];
        let (k, n) = estimate_correction_params(samples);
        println!("k = {}\nn = {}", k, n);
    }

    #[test]
    fn cylindrical_tank_constants() {
        let samples = &[
            (0.5, 1.0, 149.3893),
            (0.5, 2.0, 312.3594),
            (0.5, 5.0, 801.269800000001),
        ];
        check_samples(samples);
        let samples = &[
            (1.0, 0.5, 217.2935),
            (1.0, 1.0, 543.2337),
            (1.0, 2.0, 1195.1142),
            (1.0, 5.0, 3150.7556),
            (1.0, 10.0, 6410.1581),
        ];
        check_samples(samples);
        let samples = &[
            (2.0, 1.0, 1738.3481),
            (2.0, 2.0, 4345.86990000001),
            (2.0, 5.0, 12168.4363),
        ];
        check_samples(samples);
    }

    fn try_calc_volume(samples: &[(f64, f64, f64)]) {
        for (i, &(diameter, height, actual_volume)) in samples.iter().enumerate() {
            let actual_volume = actual_volume;
            let v = tank_volume(diameter, height) * 0.83;
            if (v - actual_volume).abs() > 0.001 {
                println!(
                    "\nEstimate: {:.5}\nActual: {:.5}\nSample: {:.5}\nError: {:.10}",
                    v,
                    actual_volume,
                    i,
                    actual_volume - v
                );
            }
        }
    }

    /// Calculates the correction coefficient for cylindrical tanks.
    fn calculate_correction_coefficient(samples: &[(f64, f64, f64)]) -> f64 {
        let mut sum_ratio = 0.0;
        let mut count = 0;
        try_calc_volume(samples);
        for &(diameter, height, actual_volume) in samples {
            let radius = diameter / 2.0;
            let actual_volume = actual_volume / 83.0 * 100.0;
            let estimated_volume_liters = std::f64::consts::PI * radius.powi(2) * height * 1000.0;
            let correction_factor = actual_volume / estimated_volume_liters;

            sum_ratio += correction_factor;
            count += 1;
        }

        sum_ratio / count as f64
    }

    /// Calculates the dry mass coefficient for some nose tanks
    #[allow(unused)]
    fn calculate_dry_mass_coefficient(samples: &[(f64, f64, f64, f64, f64)]) -> f64 {
        let mut sum_density = 0.0;
        let mut count = 0;

        for &(diameter, height, dry_mass, max_utilization, _correction_coefficient) in samples {
            let volume = tank_volume(diameter, height);
            let structural_volume = volume / max_utilization * (100.0 - max_utilization);

            let density = dry_mass / structural_volume;
            sum_density += density;
            count += 1;
        }

        sum_density / count as f64 // Returns the average coefficient
    }
    /// Conditionally calls `println!` if --print-nose-tank-stats is present.
    macro_rules! cprintln {
        ($($arg:tt)*) => {
            if PRINT_STATS {
            //if std::env::args().any(|arg| arg == "--print-nose-tank-stats") {
                println!($($arg)*);
            }
        };
    }
    /// Generate tests for cylindrical tanks based on the provided samples and test
    /// samples.
    macro_rules! impl_cylindrical_tank_test {
        ($name:ident, $core:literal, $samples:expr, $test_samples:expr) => {
            #[test]
            fn $name() {
                let correction_coefficient = calculate_correction_coefficient($samples);
                cprintln!("\nCorrection coefficient for core '{}': {:.32}\n", $core, correction_coefficient);

                for (i, &(diameter, height, expected, error)) in $test_samples.iter().enumerate() {
                    let corrected_volume = tank_volume(diameter, height) * 0.83;
                    let diff = (corrected_volume - expected).abs();
                    assert!(diff < error, "\nDifference for sample {} is too large: {:.5}\nEstimated: {:.5}\nActual: {:.5}\nError: {:.5}", i + 1, diff, corrected_volume, expected, corrected_volume - expected);
                }
            }
        };
    }

    impl_cylindrical_tank_test!(
        tank_volume_correction_core_1_0x_kerolox,
        "1.0x-Kerolox",
        &[
            (1.0, 0.5, 217.2935),
            (1.0, 1.0, 543.2337),
            (1.0, 2.0, 1195.1142),
            (1.0, 5.0, 3150.7556),
            (1.0, 10.0, 6410.1581),
        ],
        &[
            (1.0, 0.5, 217.2935, 0.001),
            (1.0, 1.0, 543.2337, 0.001),
            (1.0, 2.0, 1195.1142, 0.001),
            (1.0, 0.5, 217.2935, 0.001),
            (1.0, 1.0, 543.2337, 0.001),
            (1.0, 2.0, 1195.1142, 0.001),
            (0.5, 1.0, 149.3893, 0.0001),
            (2.0, 3.0, 6953.3923, 0.0012),
            (5.0, 5.0, 67904.2194000001, 0.0121),
            (0.1, 0.1, 0.543200000000001, 0.0001),
            (10.0, 10.0, 543233.755000001, 0.097),
        ]
    );

    macro_rules! dry_mass_test {
        ($samples:expr) => {
            #[test]
            fn dry_mass_tests_macro() {
                let diameter = 5.0;
                let height = 5.0;
                let volume = tank_volume(diameter, height);
                for (i, &(utilization, density, expected, error)) in $samples.iter().enumerate() {
                    let unutilization = (100.0 - utilization) / 100.0;
                    let unused_mass = volume * unutilization * density;
                    let diff = unused_mass - expected;
                    assert!(
                        diff.abs() < error,
                        "Diff for sample {} = {}\nExpected = {}\nEstimation = {}",
                        i + 1,
                        diff,
                        expected,
                        unused_mass
                    );
                }
            }
        };
    }

    dry_mass_test!(&[
        (
            STEEL_FUSELAGE_UTIL_PERCENT,
            STEEL_FUSELAGE_DENSITY,
            9800.0,
            100.0
        ),
        (
            HP_STEEL_FUSELAGE_UTIL_PERCENT,
            HP_STEEL_FUSELAGE_DENSITY,
            23700.0,
            100.0
        ),
        (AL_FUSELAGE_UTIL_PERCENT, AL_FUSELAGE_DENSITY, 6080.0, 20.0),
        (
            HP_AL_FUSELAGE_UTIL_PERCENT,
            HP_AL_FUSELAGE_DENSITY,
            21200.0,
            100.0
        ),
        (
            AL_STRINGER_TANK_UTIL_PERCENT,
            AL_STRINGER_TANK_DENSITY,
            4240.0,
            20.0
        ),
        (
            HP_AL_STRINGER_TANK_UTIL_PERCENT,
            HP_AL_STRINGER_TANK_DENSITY,
            17600.0,
            100.0
        ),
        (
            REFINED_AL_STRINGER_TANK_UTIL_PERCENT,
            REFINED_AL_STRINGER_TANK_DENSITY,
            3140.0,
            10.0
        ),
        (
            HP_REFINED_AL_STRINGER_TANK_UTIL_PERCENT,
            HP_REFINED_AL_STRINGER_TANK_DENSITY,
            13200.0,
            100.0
        ),
        (
            AL_LI_STRINGER_TANK_UTIL_PERCENT,
            AL_LI_STRINGER_TANK_DENSITY,
            2500.0,
            13.0
        ),
        (
            HP_AL_LI_STRINGER_TANK_UTIL_PERCENT,
            HP_AL_LI_STRINGER_TANK_DENSITY,
            7570.0,
            10.0
        ),
        (
            REFINED_AL_LI_STRINGER_TANK_UTIL_PERCENT,
            REFINED_AL_LI_STRINGER_TANK_DENSITY,
            2340.0,
            10.0
        ),
        (
            HP_REFINED_AL_LI_STRINGER_TANK_UTIL_PERCENT,
            HP_REFINED_AL_LI_STRINGER_TANK_DENSITY,
            6540.0,
            10.0
        ),
        (
            STEEL_STIR_WELDED_TANK_UTIL_PERCENT,
            STEEL_STIR_WELDED_TANK_DENSITY,
            2900.0,
            100.0
        ),
        (
            HP_STEEL_STIR_WELDED_TANK_UTIL_PERCENT,
            HP_STEEL_STIR_WELDED_TANK_DENSITY,
            11100.0,
            100.0
        )
    ]);

    #[test]
    fn dry_mass_tests() {
        let diameter = 5.0;
        let height = 5.0;
        let volume = tank_volume(diameter, height);
        let unutilization = (100.0 - STEEL_FUSELAGE_UTIL_PERCENT) / 100.0;
        let unused = volume * unutilization * STEEL_FUSELAGE_DENSITY;
        let expected = 9800.0;
        let diff = unused - expected;
        assert!(diff.abs() < 0.1, "Diff = {:.5}", diff);
        let unusable_volume = volume * (100.0 - HP_AL_FUSELAGE_UTIL_PERCENT) / 100.0;
        let diff = unusable_volume * HP_AL_FUSELAGE_DENSITY - 21200.0;
        assert!(diff.abs() < 50.1, "Diff = {}", diff);
    }

    /// Generates code to use.
    ///
    /// (fuselage: &str, utilization: f64, dry_mass_kg: f64)
    fn calculate_densities(samples: &[(&str, f64, f64)]) {
        let mut hp_tanks: Vec<(String, f64, f64)> = Vec::new();
        let mut non_hp_tanks: Vec<(String, f64, f64)> = Vec::new();
        for &(fuselage, utilization, dry_mass_kg) in samples {
            let volume = tank_volume(1.0, 1.0);
            let unused_volume = (100.0 - utilization) / 100.0 * volume;
            let density = dry_mass_kg / unused_volume;
            let var_name = {
                let replaced = fuselage.replace(' ', "_");
                let replaced = replaced.replace('-', "_");
                replaced.to_ascii_uppercase()
            };
            if var_name.contains("HP") {
                hp_tanks.push((
                    format!("{}_NAME", var_name.to_string()),
                    density,
                    utilization,
                ));
            } else {
                non_hp_tanks.push((
                    format!("{}_NAME", var_name.to_string()),
                    density,
                    utilization,
                ));
            }
            //println!("const {}_DENSITY: f64 = {}; // kg/L", var_name, density);
            //println!("const {}_UTIL_PERCENT: f64 = {:.1};", var_name, utilization);
            println!("pub const {}_NAME: &str = \"{}\";", var_name, fuselage);
        }
        for (fuselage, density, utilization) in hp_tanks {
            println!(
                "hp_tanks.insert({}, Fuselage::new({}, {}, {:.2}));",
                fuselage,
                fuselage,
                density,
                utilization / 100.0
            );
        }
        for (fuselage, density, utilization) in non_hp_tanks {
            println!(
                "non_hp_tanks.insert({}, Fuselage::new({}, {}, {:.2}));",
                fuselage,
                fuselage,
                density,
                utilization / 100.0
            );
        }
    }

    #[test]
    fn dry_mass_tests_v2() {
        println!("\n");
        calculate_densities(&[
            ("Steel Fuselage", 83.0, 78.4),
            ("HP Steel Fuselage", 75.0, 190.0),
            ("Al Fuselage", 87.0, 48.7),
            ("HP Al Fuselage", 84.0, 170.0),
            ("Al Stringer Tank", 92.0, 33.9),
            ("HP Al Stringer Tank", 90.0, 141.0),
            ("Refined Al Stringer Tank", 92.0, 25.1),
            ("HP Refined Al Stringer Tank", 90.0, 106.0),
            ("Al-Li Stringer Tank", 97.0, 19.9),
            ("HP Al-Li Stringer Tank", 96.0, 60.6),
            ("Refined Al-Li Stringer Tank", 97.0, 18.7),
            ("HP Refined Al-Li Stringer Tank", 96.0, 52.3),
            ("Steel Stir-Welded Tank", 97.0, 23.2),
            ("HP Steel Stir-Welded Tank", 96.0, 89.1),
        ]);
    }
}
