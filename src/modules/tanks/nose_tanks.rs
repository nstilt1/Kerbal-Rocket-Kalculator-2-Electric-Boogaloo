//! Module for calculating the volume and wet mass of a nose cone tank.

use std::collections::HashMap;

use serde::Serialize;

use crate::{
    debug,
    modules::{
        engines::Engine,
        tanks::cylindrical_tanks::{tank_volume, CylindricalTank},
        Error,
    },
    G,
};

use super::{
    cylindrical_tanks::{ellipsoid_volume, K},
    fuselage_names::*,
    Fuselage, Fuselages, Tanks,
};

const NOSE_1_CORRECTION_COEF: f64 = 1.34180454434038853861466122907586;
const NOSE_2_CORRECTION_COEF: f64 = 1.62346946577909534425998572260141;
const NOSE_3_CORRECTION_COEF: f64 = 1.35970328040517141054976946179522;
const NOSE_4_CORRECTION_COEF: f64 = 1.62497042872616614950231905822875;
const NOSE_5_CORRECTION_COEF: f64 = 1.27217604871392087062531572883017;
const NOSE_12_CORRECTION_COEF: f64 = 1.27211850249056235284683680220041;
const NOSE_13_CORRECTION_COEF: f64 = 1.34177037377202035273171532026026;

const HP_STEEL_FUSELAGE_DENSITY: f64 = 1.0493497032;
const HP_STEEL_FUSELAGE_UTIL_PERCENT: f64 = 75.0;
#[cfg(test)]
mod densities {
    pub const STEEL_FUSELAGE_DENSITY: f64 = 0.7050215444;
    pub const STEEL_FUSELAGE_UTIL_PERCENT: f64 = 83.0;
    pub const AL_FUSELAGE_DENSITY: f64 = 0.5997974356;
    pub const AL_FUSELAGE_UTIL_PERCENT: f64 = 87.0;
    pub const HP_AL_FUSELAGE_DENSITY: f64 = 1.6430042237;
    pub const HP_AL_FUSELAGE_UTIL_PERCENT: f64 = 84.0;
    pub const AL_STRINGER_TANK_DENSITY: f64 = 0.7186757557;
    pub const AL_STRINGER_TANK_UTIL_PERCENT: f64 = 92.0;
    pub const HP_AL_STRINGER_TANK_DENSITY: f64 = 2.3393934028;
    pub const HP_AL_STRINGER_TANK_UTIL_PERCENT: f64 = 90.0;
    pub const REFINED_AL_STRINGER_TANK_DENSITY: f64 = 0.5319629752;
    pub const REFINED_AL_STRINGER_TANK_UTIL_PERCENT: f64 = 92.0;
    pub const HP_REFINED_AL_STRINGER_TANK_DENSITY: f64 = 1.7487016847;
    pub const HP_REFINED_AL_STRINGER_TANK_UTIL_PERCENT: f64 = 90.0;
    pub const AL_LI_STRINGER_TANK_DENSITY: f64 = 1.1897409303;
    pub const AL_LI_STRINGER_TANK_UTIL_PERCENT: f64 = 97.0;
    pub const HP_AL_LI_STRINGER_TANK_DENSITY: f64 = 2.6723521459;
    pub const HP_AL_LI_STRINGER_TANK_UTIL_PERCENT: f64 = 96.0;
    pub const REFINED_AL_LI_STRINGER_TANK_DENSITY: f64 = 1.1121288578;
    pub const REFINED_AL_LI_STRINGER_TANK_UTIL_PERCENT: f64 = 97.0;
    pub const HP_REFINED_AL_LI_STRINGER_TANK_DENSITY: f64 = 2.3157513557;
    pub const HP_REFINED_AL_LI_STRINGER_TANK_UTIL_PERCENT: f64 = 96.0;
    pub const STEEL_STIR_WELDED_TANK_DENSITY: f64 = 1.3803043213;
    pub const STEEL_STIR_WELDED_TANK_UTIL_PERCENT: f64 = 97.0;
    pub const HP_STEEL_STIR_WELDED_TANK_DENSITY: f64 = 3.9418707664;
    pub const HP_STEEL_STIR_WELDED_TANK_UTIL_PERCENT: f64 = 96.0;
}

/// A nosecone and its dimensions/features.
#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct NoseCone {
    pub core: NoseTankCore,
    pub length: f64,
    pub diameter: f64,
    pub fuselage: Fuselage,
}

impl NoseCone {
    pub fn display(&self) -> String {
        format!(
            "\nCore: {}\nLength: {}\nDiameter: {}\nFuselage: {}",
            self.core.name, self.length, self.diameter, self.fuselage.name
        )
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct NoseConeVariant {
    pub name: &'static str,
    pub cores: [NoseTankCore; 7],
}

impl Tanks for NoseConeVariant {
    const MIN_VSA: f64 = 0.25;
    const MAX_VSA: f64 = 4.0;

    fn init_fuselage_types() -> Fuselages {
        let mut hp_tanks: HashMap<&str, Fuselage> = HashMap::with_capacity(7);
        let mut non_hp_tanks: HashMap<&str, Fuselage> = HashMap::with_capacity(7);
        hp_tanks.insert(
            HP_STEEL_FUSELAGE_NAME,
            Fuselage::new(
                HP_STEEL_FUSELAGE_NAME,
                HP_STEEL_FUSELAGE_DENSITY,
                HP_STEEL_FUSELAGE_UTIL_PERCENT,
            ),
        );
        hp_tanks.insert(
            HP_AL_FUSELAGE_NAME,
            Fuselage::new(HP_AL_FUSELAGE_NAME, 1.6430042237, 0.84),
        );
        hp_tanks.insert(
            HP_AL_STRINGER_TANK_NAME,
            Fuselage::new(HP_AL_STRINGER_TANK_NAME, 2.3393934028, 0.90),
        );
        hp_tanks.insert(
            HP_REFINED_AL_STRINGER_TANK_NAME,
            Fuselage::new(HP_REFINED_AL_STRINGER_TANK_NAME, 1.7487016847, 0.90),
        );
        hp_tanks.insert(
            HP_AL_LI_STRINGER_TANK_NAME,
            Fuselage::new(HP_AL_LI_STRINGER_TANK_NAME, 2.6723521459, 0.96),
        );
        hp_tanks.insert(
            HP_REFINED_AL_LI_STRINGER_TANK_NAME,
            Fuselage::new(HP_REFINED_AL_LI_STRINGER_TANK_NAME, 2.3157513557, 0.96),
        );
        hp_tanks.insert(
            HP_STEEL_STIR_WELDED_TANK_NAME,
            Fuselage::new(HP_STEEL_STIR_WELDED_TANK_NAME, 3.9418707664, 0.96),
        );
        non_hp_tanks.insert(
            STEEL_FUSELAGE_NAME,
            Fuselage::new(STEEL_FUSELAGE_NAME, 0.7050215444, 0.83),
        );
        non_hp_tanks.insert(
            AL_FUSELAGE_NAME,
            Fuselage::new(AL_FUSELAGE_NAME, 0.5997974356, 0.87),
        );
        non_hp_tanks.insert(
            AL_STRINGER_TANK_NAME,
            Fuselage::new(AL_STRINGER_TANK_NAME, 0.7186757557, 0.92),
        );
        non_hp_tanks.insert(
            REFINED_AL_STRINGER_TANK_NAME,
            Fuselage::new(REFINED_AL_STRINGER_TANK_NAME, 0.5319629752, 0.92),
        );
        non_hp_tanks.insert(
            AL_LI_STRINGER_TANK_NAME,
            Fuselage::new(AL_LI_STRINGER_TANK_NAME, 1.1897409303, 0.97),
        );
        non_hp_tanks.insert(
            REFINED_AL_LI_STRINGER_TANK_NAME,
            Fuselage::new(REFINED_AL_LI_STRINGER_TANK_NAME, 1.1121288578, 0.97),
        );
        non_hp_tanks.insert(
            STEEL_STIR_WELDED_TANK_NAME,
            Fuselage::new(STEEL_STIR_WELDED_TANK_NAME, 1.3803043213, 0.97),
        );
        Fuselages::new(hp_tanks, non_hp_tanks)
    }
}

impl NoseConeVariant {
    pub const fn nosecones() -> Self {
        let cores = [
            NoseTankCore::new("Nose-1", 1.2608, NOSE_1_CORRECTION_COEF),
            NoseTankCore::new("Nose-2", 1.3558, NOSE_2_CORRECTION_COEF),
            NoseTankCore::new("Nose-3", 0.6402, NOSE_3_CORRECTION_COEF),
            NoseTankCore::new("Nose-4", 0.3914, NOSE_4_CORRECTION_COEF),
            NoseTankCore::new("Nose-5", 1.2148, NOSE_5_CORRECTION_COEF),
            NoseTankCore::new("Nose-12", 5.0000, NOSE_12_CORRECTION_COEF),
            NoseTankCore::new("Nose-13", 3.7760, NOSE_13_CORRECTION_COEF),
        ];
        NoseConeVariant {
            name: "Nosecones",
            cores,
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone, Serialize)]
pub struct NoseTankCore {
    pub name: &'static str,
    /// The length of the tank in meters with V.ScaleAdj = 1.0000
    pub base_length: f64,
    pub correction_coefficient: f64,
}

impl NoseTankCore {
    /// Create a new NoseTank with the given name and base length.
    pub const fn new(name: &'static str, base_length: f64, correction_coefficient: f64) -> Self {
        NoseTankCore {
            name,
            base_length,
            correction_coefficient,
        }
    }
}

/// Calculate the corrected volume of the tank based on the diameter, height,
/// and correction coefficient.
pub fn calculate_corrected_volume(diameter: f64, height: f64, correction_coefficient: f64) -> f64 {
    let radius = diameter / 2.0;
    let ideal_volume_liters = ((std::f64::consts::PI * radius.powi(2) * height) / 3.0) * 1000.0;
    let corrected_volume = ideal_volume_liters * correction_coefficient;
    corrected_volume
}

/// Calculate the dry mass for a nose tank based on the diameter, height, and
/// the coefficient.
pub fn calculate_nose_dry_mass(
    diameter: f64,
    height: f64,
    density: f64,
    max_utilization: f64,
    correction_coefficient: f64,
) -> f64 {
    let volume = calculate_corrected_volume(diameter, height, correction_coefficient);
    let structural_volume = volume / max_utilization * (100.0 - max_utilization);

    structural_volume * density
}

#[cfg(test)]
fn calculate_cone_lengths(diameter: f64) -> (f64, f64, f64) {
    let base_length = diameter * 1.2608;
    let min_length = base_length * 0.25;
    let max_length = base_length * 4.0;
    (base_length, min_length, max_length)
}

/// Computes the cylindrical tank height for a rocket with a nose with a specified
/// nose height and specified TWR.
///
/// ```norun
/// Given:
/// twr = thrust / wet_mass / g
/// m_wet = payload_mass + engine_mass + fuel_mass + structural_mass
/// fuel_mass = tank_volume(h) * fuel_density * utilization + nose_volume(nose_height) * fuel_density * utilization
/// fuel_mass = (fuel_density * utilization) * (tank_volume(h) + nose_volume(nose_height))
/// structural_mass = tank_volume(h) * structural_density * (1-utilization) + nose_volume(nose_height) * structural_density * (1-utilization)
/// structural_mass = (structural_density * (1-utilization)) * (tank_volume(h) + nose_volume(nose_height))
/// tank_volume(h) = ellipsoid_volume(r) + cylinder_volume(d, h) - K*d^3
/// cylinder_volume = pi * r * r * h
///
/// Derivation:
/// wet_mass = thrust / twr / g
/// fuel_mass + structural_mass = wet_mass - payload_mass - engine_mass
/// (fuel_density * utilization) * (tank_volume(h) + nose_volume) + (structural_density * (1-utilization)) * (tank_volume(h) + nose_volume) = wet_mass - payload_mass - engine_mass
/// (tank_volume(h) + nose_volume) * (fuel_density * utilization + structural_density * (1-utilization)) = wet_mass - payload_mass - engine_mass
/// num_1 = wet_mass - payload_mass - engine_mass
/// den_1 = fuel_density * utilization + structural_density * (1-utilization)
/// tank_volume + nose_volume = num_1 / den_1
/// ellipsoid_volume + cylinder_volume(d, h) - K*d^3 = num_1/den_1 - nose_volume
/// pi*r*r*h = num_1/den_1 - nose_volume - ellipsoid_volume + K * d * d * d
/// num_2 = num_1 / den_1 - nose_volume - ellipsoid_volume + K * d * d * d
/// h = num_2 / pi * r * r
/// ```
pub fn compute_tank_height_with_nose_for_twr(
    min_twr: f64,
    engine: &Engine,
    nose_fuselage: &Fuselage,
    nose_tank_core: &NoseTankCore,
    nose_height: f64,
    cylindrical_tank_fuselage: &Fuselage,
    payload_mass: f64,
    num_tanks: u8,
    in_vacuum: bool,
) -> Result<(f64, f64, f64, f64), Error> {
    let n = num_tanks as f64;
    let thrust_total = if in_vacuum {
        engine.thrust_vac
    } else {
        engine.thrust_asl
    } * n;
    let max_wet_mass = thrust_total / (min_twr * G) * 1000.0;
    let diameter = engine.size.get_diameter();
    let r = diameter / 2.0;
    let engine_mass = engine.mass * 1000.0;
    let payload_mass = payload_mass * 1000.0;
    let fuel_density = engine.fuel_mix.density(engine.hp_fuel) * 1000.0;
    let structural_density_cyl = cylindrical_tank_fuselage.density * 1000.0;
    let structural_density_nose = nose_fuselage.density * 1000.0;
    let utilization = cylindrical_tank_fuselage.utilization;
    let ellipsoid_volume = ellipsoid_volume(r, r, r / 2.0);

    // TODO:
    // return error if nose_height is too small or too large for this diameter +
    // nose_tank_core combo

    let v_nose =
        calculate_corrected_volume(diameter, nose_height, nose_tank_core.correction_coefficient)
            * 0.001;
    debug!(
        "nose_tank_core.correction_coefficient = {}",
        nose_tank_core.correction_coefficient
    );
    debug!("diameter = {}", diameter);
    debug!("v_nose = {}", v_nose);
    debug!("nose_height = {}", nose_height);
    //let m_fuel_nose_plus_m_struct_nose = v_nose * utilization * fuel_density + v_nose * (1.0 - utilization) * structural_density_nose;
    let m_fuel_nose_plus_m_struct_nose =
        v_nose * (utilization * fuel_density + (1.0 - utilization) * structural_density_nose);

    debug!("max_wet_mass = {}", max_wet_mass);
    let n_1 = max_wet_mass - payload_mass - n * (engine_mass + m_fuel_nose_plus_m_struct_nose);
    let d_1 = fuel_density * utilization + structural_density_cyl * (1.0 - utilization);
    let t_1 = n_1 / d_1 + n * (-ellipsoid_volume + K * diameter * diameter * diameter * 0.001);
    let mut h = t_1 / (std::f64::consts::PI * r * r * n);
    debug!("n1/d1 = {}", n_1 / d_1);

    if h > CylindricalTank::MAX_VSA {
        debug!(
            "Height was over MAX_VSA = {}: {}",
            CylindricalTank::MAX_VSA,
            h
        );
        h = CylindricalTank::MAX_VSA;
    }
    if h.is_infinite() || h.is_nan() || h.is_sign_negative() {
        debug!("h was invalid: {}", h);
        return Err(Error::InvalidHeight);
    }
    let wet_mass = max_wet_mass;
    let dry_mass = wet_mass
        - v_nose * (1.0 - nose_fuselage.utilization) * engine.fuel_mix.density(engine.hp_fuel) * num_tanks as f64;
    let tank_volume = tank_volume(diameter, h) * num_tanks as f64;
    let dry_mass =
        dry_mass - tank_volume * cylindrical_tank_fuselage.utilization * engine.fuel_mix.density(engine.hp_fuel);
    let thrust_n = if in_vacuum {
        engine.thrust_vac
    } else {
        engine.thrust_asl
    } * 1000.0
        * num_tanks as f64;
    let twr = thrust_n / wet_mass / G;
    Ok((h, twr, wet_mass, dry_mass))
}

pub fn compute_tank_height_with_nose_for_delta_v(
    target_dv: f64,
    engine: &Engine,
    cylindrical_tank_fuselage: &Fuselage,
    nose_fuselage: &Fuselage,
    nose_tank_core: &NoseTankCore,
    payload_mass: f64,
    num_tanks: u8,
    nose_height: f64,
    in_vacuum: bool,
) -> Result<(f64, f64, f64, f64), Error> {
    let n = num_tanks as f64;
    let isp = if in_vacuum {
        engine.isp_vac
    } else {
        engine.isp_asl
    };
    let engine_mass = engine.mass * 1000.0;
    let payload_mass = payload_mass * 1000.0;
    let e0 = std::f64::consts::E.powf(target_dv / (isp * G));
    let d = engine.size.get_diameter();
    let r = d / 2.0;
    let ellipsoid_volume = ellipsoid_volume(r, r, r / 2.0);
    let correction = K * d * d * d * 0.001;

    let fuel_density = engine.fuel_mix.density(engine.hp_fuel) * 1000.0;
    let cyl_structural_density = cylindrical_tank_fuselage.density * 1000.0;
    let nose_structural_density = nose_fuselage.density * 1000.0;
    let u_nose = nose_fuselage.utilization;
    let u_cyl = cylindrical_tank_fuselage.utilization;

    let v_nose =
        calculate_corrected_volume(d, nose_height, nose_tank_core.correction_coefficient) * 0.001;
    debug!("v_nose = {}", v_nose);

    let m_fuel_nose = v_nose * u_nose * fuel_density;
    let m_struct_nose = v_nose * (1.0 - u_nose) * nose_structural_density;

    let dry_mass_partial = payload_mass + n * (engine_mass + m_struct_nose);
    let n_1 = n * m_fuel_nose - (e0 - 1.0) * dry_mass_partial;
    debug!("n_1 = {}", n_1);
    debug!("e0 = {}", e0);
    debug!(
        "e0*dry_mass_partial - dry_mass_partial = {}",
        (e0 - 1.0) * dry_mass_partial
    );
    debug!("dry_mass_partial = {}", dry_mass_partial);
    debug!("m_fuel_nose = {}", m_fuel_nose);
    let d_1 = cyl_structural_density * (1.0 - u_cyl) * (e0 - 1.0) - fuel_density * u_cyl;
    debug!("d_1 = {}", d_1);
    let t_1 = n_1 / d_1 + n * (-ellipsoid_volume + correction);
    debug!("t_1 = {}", t_1);
    let h = t_1 / (std::f64::consts::PI * r * r * n);

    if h > CylindricalTank::MAX_VSA {
        debug!("h was greater than max vsa: {}", h);
        return Err(Error::HeightTooLarge);
    }
    if h.is_sign_negative() || h.is_nan() || h.is_infinite() {
        debug!("h was invalid: {}", h);
        return Err(Error::InvalidHeight);
    }
    let cyl_volume = tank_volume(d, h) * n;
    let m_struct_cyl = cyl_volume
        * cylindrical_tank_fuselage.density
        * (1.0 - cylindrical_tank_fuselage.utilization);

    let dry_mass = payload_mass + n * (engine_mass + m_struct_nose) + m_struct_cyl;
    let wet_mass = dry_mass + n * m_fuel_nose + engine.fuel_mix.density(engine.hp_fuel) * u_cyl * cyl_volume;
    let thrust_n = if in_vacuum {
        engine.thrust_vac
    } else {
        engine.thrust_asl
    } * 1000.0
        * n;
    let twr = thrust_n / wet_mass / G;
    Ok((h, twr, wet_mass, dry_mass))
}

#[cfg(test)]
mod tests {
    use crate::modules::{
        engines::ENGINES,
        tanks::cylindrical_tanks::{tank_volume, CylindricalTank},
    };

    pub use super::densities::*;
    use super::*;

    #[test]
    fn delta_v_nosecone_test() {
        let nose_height = 2.0;
        let target_dv = 2789.12345;
        let in_vacuum = false;
        let engine = ENGINES[4];
        println!("Engine: {}", engine.name);
        let num_tanks = 8;
        let num_tanks_f64 = num_tanks as f64;
        let payload_mass_tons = 5.5;
        let payload_mass_kg = payload_mass_tons * 1000.0;
        let engine_mass_kg = engine.mass * 1000.0 * num_tanks_f64;
        let diameter = engine.size.get_diameter();
        let cyl_fuselage_types = CylindricalTank::init_fuselage_types();
        let nose_fuselage_types = NoseConeVariant::init_fuselage_types();
        let cyl_fuselage = cyl_fuselage_types
            .non_hp_fuselages
            .get(STEEL_FUSELAGE_NAME)
            .unwrap();
        let nose_fuselage = nose_fuselage_types
            .non_hp_fuselages
            .get(STEEL_FUSELAGE_NAME)
            .unwrap();
        let nosecones = NoseConeVariant::nosecones();
        let core = &nosecones.cores[0];
        let (h, twr, _wet_mass, _dry_mass) = compute_tank_height_with_nose_for_delta_v(
            target_dv,
            &engine,
            cyl_fuselage,
            nose_fuselage,
            core,
            payload_mass_tons,
            num_tanks,
            nose_height,
            in_vacuum,
        )
        .unwrap();
        println!("h = {}", h);

        let nose_volume =
            calculate_corrected_volume(diameter, nose_height, core.correction_coefficient);
        let cyl_volume = tank_volume(diameter, h);
        let nose_structural_mass =
            nose_volume * nose_fuselage.density * (1.0 - nose_fuselage.utilization);
        let cyl_structural_mass =
            cyl_volume * cyl_fuselage.density * (1.0 - cyl_fuselage.utilization);
        let dry_mass = payload_mass_kg
            + engine_mass_kg
            + num_tanks_f64 * (nose_structural_mass + cyl_structural_mass);
        let wet_mass = dry_mass
            + num_tanks_f64 * cyl_volume * cyl_fuselage.utilization * engine.fuel_mix.density(engine.hp_fuel)
            + num_tanks_f64 * nose_volume * nose_fuselage.utilization * engine.fuel_mix.density(engine.hp_fuel);
        let delta_v = engine.isp_asl * G * f64::ln(wet_mass / dry_mass);
        let diff = delta_v - target_dv;
        assert!(diff.abs() < 0.0001);
        let thrust_n = if in_vacuum {
            engine.thrust_vac
        } else {
            engine.thrust_asl
        } * 1000.0
            * num_tanks_f64;

        let expected_twr = thrust_n / wet_mass / G;
        let diff = expected_twr - twr;
        assert!(diff.abs() < 0.00001);
    }

    #[test]
    fn twr_nosecone_test() {
        let nose_height = 3.2;
        let target_twr = 3.0;
        let engine = ENGINES[4];
        println!("Engine: {}", engine.name);
        let num_tanks_f64 = 5.0;
        let payload_mass_tons = 1.5;
        let payload_mass_kg = payload_mass_tons * 1000.0;
        let engine_mass_kg = engine.mass * 1000.0 * num_tanks_f64;
        let thrust_n = engine.thrust_asl * 1000.0 * num_tanks_f64;
        let in_vacuum = false;
        let diameter = engine.size.get_diameter();
        let cyl_fuselage_types = CylindricalTank::init_fuselage_types();
        let nose_fuselage_types = NoseConeVariant::init_fuselage_types();
        let cyl_fuselage = cyl_fuselage_types
            .non_hp_fuselages
            .get(STEEL_FUSELAGE_NAME)
            .unwrap();
        let nose_fuselage = nose_fuselage_types
            .non_hp_fuselages
            .get(STEEL_FUSELAGE_NAME)
            .unwrap();
        let nosecones = NoseConeVariant::nosecones();
        let core = &nosecones.cores[0];
        let (h, twr, _wet, _dry) = compute_tank_height_with_nose_for_twr(
            target_twr,
            &engine,
            nose_fuselage,
            &core,
            nose_height,
            cyl_fuselage,
            payload_mass_tons,
            num_tanks_f64 as u8,
            in_vacuum,
        )
        .unwrap();
        println!("h = {}", h);

        let nose_volume =
            calculate_corrected_volume(diameter, nose_height, core.correction_coefficient);
        let nose_dry_mass = nose_volume * (1.0 - nose_fuselage.utilization) * nose_fuselage.density;
        let nose_wet_mass = nose_dry_mass
            + engine
                .fuel_mix
                .mass(nose_volume * nose_fuselage.utilization, engine.hp_fuel);

        let cyl_volume = tank_volume(diameter, h);
        let cyl_dry_mass = cyl_volume * (1.0 - cyl_fuselage.utilization) * cyl_fuselage.density;
        let cyl_wet_mass =
            cyl_dry_mass + engine.fuel_mix.mass(cyl_fuselage.utilization * cyl_volume, engine.hp_fuel);

        //let dry_mass = engine_mass_kg + payload_mass_kg + nose_dry_mass * num_tanks_f64 + cyl_dry_mass * num_tanks_f64;
        let wet_mass = engine_mass_kg
            + payload_mass_kg
            + nose_wet_mass * num_tanks_f64
            + cyl_wet_mass * num_tanks_f64;

        let expected_twr = thrust_n / wet_mass / G;
        assert_eq!(expected_twr, target_twr);
        assert_eq!(expected_twr, twr);
    }

    const N1: f64 = NOSE_1_CORRECTION_COEF;

    // using a const since you can't easily pass arguments to `cargo test`
    const PRINT_STATS: bool = false;

    /// Calculate the correction coefficient for the tank volume based on sample
    /// measurements.
    ///
    /// The samples are tuples of
    /// * (`diameter_meters`, `height_meters`, `actual_volume_liters`)
    fn calculate_correction_coefficient(samples: &[(f64, f64, f64)]) -> f64 {
        let mut sum_ratio = 0.0;
        let mut count = 0;

        for &(diameter, height, actual_volume) in samples {
            let radius = diameter / 2.0;
            let estimated_volume_liters =
                ((std::f64::consts::PI * radius.powi(2) * height) / 3.0) * 1000.0;
            let correction_factor = actual_volume / estimated_volume_liters;

            sum_ratio += correction_factor;
            count += 1;
        }

        sum_ratio / count as f64 // Average correction coefficient
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

    /// Generate tests for nosecone tanks based on the provided samples and test
    /// samples.
    macro_rules! impl_nosecone_test {
        ($name:ident, $core:literal, $samples:expr, $test_samples:expr) => {
            #[test]
            fn $name() {
                let correction_coefficient = calculate_correction_coefficient($samples);
                cprintln!(
                    "\nCorrection coefficient for core '{}': {:.32}\n",
                    $core,
                    correction_coefficient
                );

                for (i, &(diameter, height, expected, error)) in $test_samples.iter().enumerate() {
                    let corrected_volume =
                        calculate_corrected_volume(diameter, height, correction_coefficient);
                    let diff = (corrected_volume - expected).abs();
                    assert!(
                        diff < error,
                        "\nDifference for sample {} is too large: {}\n",
                        i + 1,
                        diff
                    );
                }
            }
        };
    }
    // nose-1
    impl_nosecone_test!(
        tank_volume_correction_core_nose_1,
        "nose-1",
        &[
            (1.3, 0.4098, 243.2632), // V.ScaleAdj = 0.2500
            //(1.3, 0.6556, 389.2212),    // V.ScaleAdj = 0.4000
            (1.3, 0.8195, 486.5264), // V.ScaleAdj = 0.5000
            //(1.3, 0.9834, 583.83169999),// V.ScaleAdj = 0.6000
            (1.3, 1.2293, 729.7897), // V.ScaleAdj = 0.7500
            //(1.3, 1.4751, 875.7475999), // V.ScaleAdj = 0.9000
            (1.3, 1.639, 973.0528), // V.ScaleAdj = 1.0000
            //(1.3, 1.721, 1021.7055),    // V.ScaleAdj = 1.0500
            (1.3, 2.2127, 1313.6213),    // V.ScaleAdj = 1.3500
            (1.3, 2.8683, 1702.8424),    // V.ScaleAdj = 1.7500
            (1.3, 3.2781, 1946.1054),    // V.ScaleAdj = 2.0000
            (1.3, 4.0976, 2432.6318),    // V.ScaleAdj = 2.5000
            (1.3, 4.9171, 2919.1586),    // V.ScaleAdj = 3.0000
            (1.3, 5.7366, 3405.6849),    // V.ScaleAdj = 3.5000
            (1.3, 6.5562, 3892.2112999), // V.ScaleAdj = 4.0000
        ],
        &[
            (1.3, 0.6556, 389.2212, 0.02),    // V.ScaleAdj = 0.4000
            (1.3, 2.2947, 1362.274, 0.02),    // V.ScaleAdj = 1.3500
            (3.0, 5.2954, 16741.6431, 0.05),  // V.ScaleAdj = 2.0000
            (5.0, 12.6080, 110725.1485, 0.6), // V.ScaleAdj = 3.5000
        ]
    );
    // nose-2
    impl_nosecone_test!(
        tank_volume_correction_core_nose_2,
        "nose-2",
        &[
            (1.3, 0.4406, 316.5162),      // V.ScaleAdj = 0.2500
            (1.3, 0.8813, 633.032299999), // V.ScaleAdj = 0.5000
            (1.3, 1.3219, 949.548599999), // V.ScaleAdj = 0.7500
            (1.3, 1.7625, 1266.0646),     // V.ScaleAdj = 1.0000
            (1.3, 2.2032, 1582.5808),     // V.ScaleAdj = 1.2500
            (1.3, 2.6438, 1899.0973),     // V.ScaleAdj = 1.5000
            (1.3, 3.0844, 2215.6133),     // V.ScaleAdj = 1.7500
            (1.3, 3.5251, 2531.1292),     // V.ScaleAdj = 2.0000
        ],
        &[
            (1.3, 2.3794, 1709.1875, 0.09),    // V.ScaleAdj = 1.3500
            (1.3, 7.0502, 5064.25849, 0.18),   // V.ScaleAdj = 4.0000
            (3.0, 4.0674, 15559.2816, 0.64),   // V.ScaleAdj = 1.0000
            (3.0, 12.2022, 46677.84629, 1.88), //VScaleAdj = 3.0000
            (5.0, 20.3370, 216101.1182, 8.65), //V.ScaleAdj = 3.0000
        ]
    );
    // nose-3
    impl_nosecone_test!(
        tank_volume_correction_core_nose_3,
        "nose-3",
        &[
            (1.3, 0.2081, 125.1712),    // VSA = 0.25
            (1.3, 0.4161, 250.3423999), // VSA = 0.50
            (1.3, 0.6242, 375.5136999), // VSA = 0.75
            (1.3, 0.8323, 500.6848999), // VSA = 1.00
            //(1.3, 1.0403, 625.8560999),     // VSA = 1.25
            (1.3, 1.2484, 751.0273999), // VSA = 1.50
            //(1.3, 1.4565, 876.1985999),     // VSA = 1.75
            (1.3, 1.6645, 1001.3697), // VSA = 2.00
            //(1.3, 1.8726, 1126.541),        // VSA = 2.25
            (1.3, 2.0807, 1251.7121),     // VSA = 2.50
            (1.3, 2.4968, 1502.0548),     // VSA = 3.00
            (1.3, 2.9129, 1752.3972),     // VSA = 3.50
            (1.3, 3.3290, 2002.7394),     // VSA = 4.00
            (3.0, 4.8015, 15382.8958),    // VSA = 2.50
            (3.0, 6.7221, 21536.0562999), // VSA = 3.50
        ],
        &[
            (1.3, 0.5410, 325.4451999, 0.02),  // VSA = 0.65
            (1.3, 1.1236, 675.9245999, 0.022), // VSA = 1.35
            (3.0, 1.9206, 6153.158999, 0.09),  // VSA = 1.00
            (3.0, 5.7618, 18459.4769, 0.26),   // VSA = 3.00
            (5.0, 3.2010, 28486.844, 0.39),    // VSA = 1.00
            (5.0, 9.6030, 85460.528, 1.2),     // VSA = 3.00
        ]
    );
    // nose-4
    impl_nosecone_test!(
        tank_volume_correction_core_nose_4,
        "nose-4",
        &[
            (1.3, 0.1272, 91.45089999), // VSA = 0.25
            (1.3, 0.5088, 365.8034999), // VSA = 1.00
            (1.3, 1.0176, 731.6069999), // VSA = 2.00
        ],
        &[
            (1.3, 0.3053, 219.4820999, 0.02),  // VSA = 0.60
            (1.3, 0.7632, 548.7052999, 0.02),  // VSA = 1.50
            (1.3, 1.9081, 1371.7632, 0.075),   // VSA = 3.75
            (3.0, 1.1742, 4495.535999, 0.20),  // VSA = 1.00
            (3.0, 3.5226, 13486.6087, 0.54),   // VSA = 3.00
            (5.0, 0.9785, 10406.3327, 0.42),   // VSA = 0.50
            (5.0, 6.8495, 72844.328999, 2.89), // VSA = 3.50
        ]
    );
    // nose-5
    impl_nosecone_test!(
        tank_volume_correction_core_nose_5,
        "nose-5",
        &[
            (1.0, 1.2148, 404.5950999),  // VSA = 1.00
            (5.0, 18.2220, 151723.1667), // VSA = 3.00
        ],
        &[
            (1.0, 0.6074, 202.2976, 0.00005),   // VSA = 0.50
            (1.3, 4.7377, 2666.6870999, 0.02),  // VSA = 3.00
            (3.0, 9.1110, 27310.170599, 0.001), // VSA = 2.50
            (5.0, 15.185, 126435.9659, 0.005),  // VSA = 2.50
        ]
    );
    // nose-12
    impl_nosecone_test!(
        tank_volume_correction_core_nose_12,
        "nose-12",
        &[
            (1.0, 5.0000, 1665.1993),  // VSA = 1.00
            (5.0, 100.0, 832599.5758), // VSA = 4.00
        ],
        &[
            (1.3, 3.2500, 1829.2216, 0.0003),  // VSA = 0.50
            (1.3, 9.7500, 5487.6655, 0.002),   // VSA = 1.50
            (1.3, 19.500, 10975.3311, 0.004),  // VSA = 3.00
            (3.0, 45.000, 134881.1424, 0.006), // VSA = 3.00
        ]
    );

    // nose-13
    impl_nosecone_test!(
        tank_volume_correction_core_nose_13,
        "nose-13",
        &[
            (1.0, 3.7760, 1326.4132),   // VSA = 1.0
            (5.0, 75.520, 663206.5253), // VSA = 4.0
        ],
        &[
            (1.3, 2.4544, 1457.0651, 0.0003),   // VSA = 0.50
            (1.3, 7.3632, 4371.1954, 0.001),    // VSA = 1.50
            (3.0, 11.3280, 35813.1556, 0.0013), // VSA = 1.00
            (3.0, 28.3200, 89532.8875, 0.0016), // VSA = 2.50
            (5.0, 37.7600, 331603.2626, 0.019), // VSA = 2.00
        ]
    );

    /// Calculates the dry mass coefficient for some nose tanks
    fn calculate_dry_mass_coefficient(samples: &[(f64, f64, f64, f64, f64)]) -> f64 {
        let mut sum_density = 0.0;
        let mut count = 0;

        for &(diameter, height, dry_mass, max_utilization, correction_coefficient) in samples {
            let volume = calculate_corrected_volume(diameter, height, correction_coefficient);
            let structural_volume = volume / max_utilization * (100.0 - max_utilization);

            let density = dry_mass / structural_volume;
            sum_density += density;
            count += 1;
        }

        sum_density / count as f64 // Returns the average coefficient
    }

    /// Implements some tests for finding the dry mass coefficient and validates
    /// that the coefficient is correct.
    macro_rules! impl_nosecone_mass_test {
        ($name:ident, $core:literal, $tank_type:literal, $samples:expr, $test_samples:expr) => {
            #[test]
            fn $name() {
                let density = calculate_dry_mass_coefficient($samples);
                cprintln!("\nDry mass density for core '{}:{}': {:.10}\n", $core, $tank_type, density);

                for (i, &(diameter, height, expected, error, max_utilization, correction_coefficient)) in $test_samples.iter().enumerate() {
                    let estimated_dry_mass = calculate_nose_dry_mass(diameter, height, density, max_utilization, correction_coefficient);
                    let diff = (estimated_dry_mass - expected).abs();
                    assert!(diff < error, "Dry mass Difference for sample {} is too large: {}\nExpected: {}\nEstimate: {}\n", i + 1, diff, expected, estimated_dry_mass);
                }
            }
        };
    }

    impl_nosecone_mass_test!(
        tank_dry_mass_core_nose_1_steel_fuselage,
        "Nose-1",
        "Steel Fuselage",
        &[
            (0.1, 0.0315, 0.016, 83.0, NOSE_1_CORRECTION_COEF), //VSA = 0.25
            (0.1, 0.1261, 0.0639, 83.0, NOSE_1_CORRECTION_COEF), //VSA = 1.00
            (1.0, 0.3152, 16.0, 83.0, NOSE_1_CORRECTION_COEF),  //VSA = 0.25
            (1.0, 0.3782, 19.2, 83.0, NOSE_1_CORRECTION_COEF),  //VSA = 0.30
            (1.0, 1.2608, 63.9, 83.0, NOSE_1_CORRECTION_COEF),  //VSA = 1.00
            (1.3, 0.8195, 70.2, 83.0, NOSE_1_CORRECTION_COEF),  // VSA = 0.50
            (1.3, 5.7366, 492.0, 83.0, NOSE_1_CORRECTION_COEF), //VSA = 3.50
            (3.0, 13.2384, 6040.0, 83.0, NOSE_1_CORRECTION_COEF), // VSA = 3.50
        ],
        &[
            (3.0, 7.5648, 3450.0, 4.3, 83.0, NOSE_1_CORRECTION_COEF), //VSA = 2.0
            (4.0, 10.0864, 8180.0, 6.3, 83.0, NOSE_1_CORRECTION_COEF), // VSA = 2.0
        ]
    );

    impl_nosecone_mass_test!(
        tank_dry_mass_core_nose_2_steel_fuselage,
        "Nose-2",
        "Steel Fuselage",
        &[
            (0.1, 0.0339, 0.0208, 83.0, NOSE_2_CORRECTION_COEF), // VSA = 0.25
            (0.1, 0.1356, 0.0832, 83.0, NOSE_2_CORRECTION_COEF), // VSA = 1.0
            (1.0, 0.3390, 20.8, 83.0, NOSE_2_CORRECTION_COEF),   // VSA = 0.25
        ],
        &[
            (3.0, 4.0674, 2250.0, 3.94, 83.0, NOSE_2_CORRECTION_COEF),// VSA = 1.0
        ]
    );

    impl_nosecone_mass_test!(
        tank_dry_mass_core_nose_1_hp_steel_fuselage,
        "Nose-1",
        "HP Steel Fuselage",
        &[
            (0.1, 0.0315, 0.0387, 75.0, NOSE_1_CORRECTION_COEF), //VSA = 0.25
            (0.1, 0.1261, 0.155, 75.0, NOSE_1_CORRECTION_COEF),  // VSA = 1.0
            (1.0, 0.3152, 38.7, 75.0, NOSE_1_CORRECTION_COEF),   // VSA = 0.25
            (1.0, 1.2608, 155.0, 75.0, NOSE_1_CORRECTION_COEF),  // VSA = 1.0
        ],
        &[
            (3.0, 7.5648, 8360.0, 5.6, 75.0, NOSE_1_CORRECTION_COEF), // VSA = 2.0
            (5.0, 12.6080, 38700.0, 29.7, 75.0, NOSE_1_CORRECTION_COEF), // VSA = 2.0
        ]
    );

    impl_nosecone_mass_test!(
        tank_dry_mass_core_nose_1_al_fuselage,
        "Nose-1",
        "Al Fuselage",
        &[
            (0.1, 0.0315, 0.00992, 87.0, N1), // VSA = 0.25
            (0.1, 0.1261, 0.0397, 87.0, N1),  // VSA = 1.0
            (1.0, 0.3152, 9.92, 87.0, N1),    // VSA = 0.25
            (1.0, 1.2608, 39.7, 87.0, N1),    // VSA = 1.0
        ],
        &[
            (3.0, 7.5648, 2140.0, 3.6, 87.0, N1),  // VSA = 2.0
            (5.0, 12.6080, 9920.0, 3.7, 87.0, N1), // VSA = 2.0
        ]
    );

    macro_rules! get_fuselage_densities {
        ($name:ident, $fuselage:literal, $utilization:literal,
            $mass_1:literal, $mass_2:literal, $mass_3:literal,
            $mass_4:literal, $expct_mass_1:literal, $expct_mass_2:literal,
            $error_1:literal, $error_2:literal
        ) => {
            impl_nosecone_mass_test!(
                $name,
                "Nose-1",
                $fuselage,
                &[
                    (0.1, 0.0315, $mass_1, $utilization, N1), // VSA = 0.25
                    (0.1, 0.1261, $mass_2, $utilization, N1), // VSA = 1.0
                    (0.5, 0.3152, $mass_3, $utilization, N1), // VSA = 0.5
                    (1.0, 0.3152, $mass_4, $utilization, N1), // VSA = 0.25
                                                              //(1.0, 1.2608, $mass_4, $utilization, N1),// VSA = 1.0
                                                              //(2.0, 5.0432, $mass_5, $utilization, N1),// VSA = 2.0
                ],
                &[
                    (3.0, 7.5648, $expct_mass_1, $error_1, $utilization, N1), // VSA = 2.0
                    (5.0, 12.6080, $expct_mass_2, $error_2, $utilization, N1), // VSA = 2.0
                ]
            );
        };
    }
    get_fuselage_densities!(
        tank_dry_mass_core_nose_1_hp_al_fuselage,
        "HP Al Fuselage",
        84.0,
        0.0346,
        0.139,
        8.66,
        34.6,
        7480.0,
        34600.0,
        4.8,
        51.7
    );
    get_fuselage_densities!(
        tank_dry_mass_core_nose_1_al_stringer_tank,
        "Al Stringer Tank",
        92.0,
        0.00691,
        0.0277,
        1.73,
        6.92,
        1490.0,
        6920.0,
        4.7,
        0.43
    );
    get_fuselage_densities!(
        tank_dry_mass_core_nose_1_hp_al_stringer_tank,
        "HP Al Stringer Tank",
        90.0,    // Utilization
        0.0288,  // D=0.1 VSA=0.25
        0.115,   // D=0.1 VSA=1.00
        7.19,    // D=0.5 VSA=0.50
        28.8,    // D=1.0 VSA=0.25
        6210.0,  // D=3.0 VSA=2.00
        28800.0, // D=5.0 VSA=2.00
        6.7,     // Error for sample 1
        19.1     // Error for sample 2
    );
    get_fuselage_densities!(
        tank_dry_mass_core_nose_1_refined_al_stringer_tank,
        "Refined Al Stringer Tank",
        92.0,    // Utilization
        0.00512, // D=0.1 VSA=0.25
        0.0205,  // D=0.1 VSA=1.00
        1.28,    // D=0.5 VSA=0.50
        5.12,    // D=1.0 VSA=0.25
        1110.0,  // D=3.0 VSA=2.00
        5120.0,  // D=5.0 VSA=2.00
        3.7,     // Error for sample 1
        1.9      // Error for sample 2
    );
    get_fuselage_densities!(
        tank_dry_mass_core_nose_1_hp_refined_al_stringer_tank,
        "HP Refined Al Stringer Tank",
        90.0,    // Utilization
        0.0215,  // D=0.1 VSA=0.25
        0.0861,  // D=0.1 VSA=1.00
        5.38,    // D=0.5 VSA=0.50
        21.5,    // D=1.0 VSA=0.25
        4650.0,  // D=3.0 VSA=2.00
        21500.0, // D=5.0 VSA=2.00
        3.1,     // Error for sample 1
        13.9     // Error for sample 2
    );
    get_fuselage_densities!(
        tank_dry_mass_core_nose_1_al_li_stringer_tank,
        "Al-Li Stringer Tank",
        97.0,    // Utilization
        0.00407, // D=0.1 VSA=0.25
        0.0163,  // D=0.1 VSA=1.00
        1.02,    // D=0.5 VSA=0.50
        4.07,    // D=1.0 VSA=0.25
        880.0,   // D=3.0 VSA=2.00
        4070.0,  // D=5.0 VSA=2.00
        0.04,    // Error for sample 1
        4.3      // Error for sample 2
    );
    get_fuselage_densities!(
        tank_dry_mass_core_nose_1_hp_al_li_stringer_tank,
        "HP Al-Li Stringer Tank",
        96.0,    // Utilization
        0.0123,  // D=0.1 VSA=0.25
        0.0494,  // D=0.1 VSA=1.00
        3.09,    // D=0.5 VSA=0.50
        12.3,    // D=1.0 VSA=0.25
        2670.0,  // D=3.0 VSA=2.00
        12300.0, // D=5.0 VSA=2.00
        7.0,     // Error for sample 1
        29.0     // Error for sample 2
    );
    get_fuselage_densities!(
        tank_dry_mass_core_nose_1_refined_al_li_stringer_tank,
        "Refined Al-Li Stringer Tank",
        97.0,    // Utilization
        0.00381, // D=0.1 VSA=0.25
        0.0152,  // D=0.1 VSA=1.00
        0.953,   // D=0.5 VSA=0.50
        3.81,    // D=1.0 VSA=0.25
        823.0,   // D=3.0 VSA=2.00
        3810.0,  // D=5.0 VSA=2.00
        0.4,     // Error for sample 1
        1.6      // Error for sample 2
    );
    get_fuselage_densities!(
        tank_dry_mass_core_nose_1_hp_refined_al_li_stringer_tank,
        "HP Refined Al-Li Stringer Tank",
        96.0,    // Utilization
        0.0107,  // D=0.1 VSA=0.25
        0.0426,  // D=0.1 VSA=1.00
        2.67,    // D=0.5 VSA=0.50
        10.7,    // D=1.0 VSA=0.25
        2300.0,  // D=3.0 VSA=2.00
        10700.0, // D=5.0 VSA=2.00
        7.7,     // Error for sample 1
        16.3     // Error for sample 2
    );
    get_fuselage_densities!(
        tank_dry_mass_core_nose_1_steel_stir_welded_tank,
        "Steel Stir Welded Tank",
        97.0,    // Utilization
        0.00473, // D=0.1 VSA=0.25
        0.0189,  // D=0.1 VSA=1.00
        1.18,    // D=0.5 VSA=0.50
        4.73,    // D=1.0 VSA=0.25
        1020.0,  // D=3.0 VSA=2.00
        4730.0,  // D=5.0 VSA=2.00
        1.0,     // Error for sample 1
        3.2      // Error for sample 2
    );
    get_fuselage_densities!(
        tank_dry_mass_core_nose_1_hp_steel_stir_welded_tank,
        "HP Steel Stir Welded Tank",
        96.0,    // Utilization
        0.0182,  // D=0.1 VSA=0.25
        0.0727,  // D=0.1 VSA=1.00
        4.54,    // D=0.5 VSA=0.50
        18.2,    // D=1.0 VSA=0.25
        3920.0,  // D=3.0 VSA=2.00
        18200.0, // D=5.0 VSA=2.00
        8.16,    // Error for sample 1
        14.1     // Error for sample 2
    );
    /// Determines that fuselage densities are the same across different nose
    /// cores.
    #[test]
    fn noses_with_const_density() {
        let density = 0.7050215444;

        {
            let volume_n2 = calculate_corrected_volume(3.0, 4.0674, NOSE_2_CORRECTION_COEF);
            let struct_vol = volume_n2 / 83.0 * (100.0 - 83.0);
            let expected_dry_mass = 2250.0;
            let dry_mass = density * struct_vol;
            let diff = (expected_dry_mass - dry_mass).abs();
            assert!(diff < 4.3, "Dry mass diff was {}", diff);
        }

        // nose-3
        {
            let volume_n3 = calculate_corrected_volume(3.0, 3.8412, NOSE_3_CORRECTION_COEF);
            let actual_volume = 12306.3179;
            let diff = (volume_n3 - actual_volume).abs();
            assert!(diff < 0.2, "\nVolume diff was {}\n", diff);
            let struct_vol = volume_n3 / 83.0 * (100.0 - 83.0);
            let expected_dry_mass = 1780.0;
            let dry_mass = density * struct_vol;
            let diff = (expected_dry_mass - dry_mass).abs();
            assert!(
                diff < 3.0,
                "\nDry mass diff was {}\nDry mass = {}\nExpected = {}",
                diff,
                dry_mass,
                expected_dry_mass
            );
        }

        {
            let volume_n3 = calculate_corrected_volume(3.0, 1.9206, NOSE_3_CORRECTION_COEF);
            let actual_volume = 6153.159;
            let diff = (volume_n3 - actual_volume).abs();
            assert!(diff < 0.3, "\nVolume diff was {}\n", diff);
            let struct_vol = volume_n3 / 83.0 * (100.0 - 83.0);
            let expected_dry_mass = 888.0;
            let dry_mass = density * struct_vol;
            let diff = (expected_dry_mass - dry_mass).abs();
            assert!(
                diff < 1.0,
                "\nDry mass diff was {}\nDry mass = {}\nExpected = {}",
                diff,
                dry_mass,
                expected_dry_mass
            );
        }

        // nose-4
        {
            let volume_n4 = calculate_corrected_volume(3.0, 1.1742, NOSE_4_CORRECTION_COEF);
            let actual_volume = 4495.536;
            let diff = (volume_n4 - actual_volume).abs();
            assert!(diff < 0.3, "\nVolume diff was {}\n", diff);
            let struct_vol = volume_n4 / 83.0 * (100.0 - 83.0);
            let expected_dry_mass = 649.0;
            let dry_mass = density * struct_vol;
            let diff = (expected_dry_mass - dry_mass).abs();
            assert!(
                diff < 1.0,
                "\nDry mass diff was {}\nDry mass = {}\nExpected = {}",
                diff,
                dry_mass,
                expected_dry_mass
            );
        }

        // nose-5
        {
            let volume_n5 = calculate_corrected_volume(3.0, 3.6444, NOSE_5_CORRECTION_COEF);
            let actual_volume = 10924.0687;
            let diff = (volume_n5 - actual_volume).abs();
            assert!(diff < 0.3, "\nVolume diff was {}\n", diff);
            let struct_vol = volume_n5 / 83.0 * (100.0 - 83.0);
            let expected_dry_mass = 1580.0;
            let dry_mass = density * struct_vol;
            let diff = (expected_dry_mass - dry_mass).abs();
            assert!(
                diff < 3.0,
                "\nDry mass diff was {}\nDry mass = {}\nExpected = {}",
                diff,
                dry_mass,
                expected_dry_mass
            );
        }
    }

    #[test]
    #[allow(unused)]
    fn test_tank_length() {
        let diameter = 5.0;
        let (base_length, min_length, max_length) = calculate_cone_lengths(diameter);
        let expected_base_length = 6.3040;
        assert!(
            (base_length - expected_base_length).abs() < 0.01,
            "Base length is incorrect"
        );
        let diameter = 3.0;
        let (base_length, min_length, max_length) = calculate_cone_lengths(diameter);
        let expected_base_length = 3.7824;
        assert!(
            (base_length - expected_base_length).abs() < 0.01,
            "Base length is incorrect"
        );
    }

    fn write_fuselage_code(samples: &[(&str, f64, f64)]) {
        let mut hp_tanks: Vec<(String, f64, f64)> = Vec::new();
        let mut non_hp_tanks = hp_tanks.clone();
        for &(fuselage, utilization, density) in samples {
            if fuselage.to_lowercase().contains("hp") {
                hp_tanks.push((fuselage.to_string(), density, utilization));
            } else {
                non_hp_tanks.push((fuselage.to_string(), density, utilization));
            }
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
    fn generate_code() {
        write_fuselage_code(&[
            (
                "STEEL_FUSELAGE_NAME",
                STEEL_FUSELAGE_UTIL_PERCENT,
                STEEL_FUSELAGE_DENSITY,
            ),
            (
                "HP_STEEL_FUSELAGE_NAME",
                HP_STEEL_FUSELAGE_UTIL_PERCENT,
                HP_STEEL_FUSELAGE_DENSITY,
            ),
            (
                "AL_FUSELAGE_NAME",
                AL_FUSELAGE_UTIL_PERCENT,
                AL_FUSELAGE_DENSITY,
            ),
            (
                "HP_AL_FUSELAGE_NAME",
                HP_AL_FUSELAGE_UTIL_PERCENT,
                HP_AL_FUSELAGE_DENSITY,
            ),
            (
                "HP_AL_FUSELAGE_NAME",
                HP_AL_FUSELAGE_UTIL_PERCENT,
                HP_AL_FUSELAGE_DENSITY,
            ),
            (
                "AL_STRINGER_TANK_NAME",
                AL_STRINGER_TANK_UTIL_PERCENT,
                AL_STRINGER_TANK_DENSITY,
            ),
            (
                "HP_AL_STRINGER_TANK_NAME",
                HP_AL_STRINGER_TANK_UTIL_PERCENT,
                HP_AL_STRINGER_TANK_DENSITY,
            ),
            (
                "REFINED_AL_STRINGER_TANK_NAME",
                REFINED_AL_STRINGER_TANK_UTIL_PERCENT,
                REFINED_AL_STRINGER_TANK_DENSITY,
            ),
            (
                "HP_REFINED_AL_STRINGER_TANK_NAME",
                HP_REFINED_AL_STRINGER_TANK_UTIL_PERCENT,
                HP_REFINED_AL_STRINGER_TANK_DENSITY,
            ),
            (
                "AL_LI_STRINGER_TANK_NAME",
                AL_LI_STRINGER_TANK_UTIL_PERCENT,
                AL_LI_STRINGER_TANK_DENSITY,
            ),
            (
                "HP_AL_LI_STRINGER_TANK_NAME",
                HP_AL_LI_STRINGER_TANK_UTIL_PERCENT,
                HP_AL_LI_STRINGER_TANK_DENSITY,
            ),
            (
                "REFINED_AL_LI_STRINGER_TANK_NAME",
                REFINED_AL_LI_STRINGER_TANK_UTIL_PERCENT,
                REFINED_AL_LI_STRINGER_TANK_DENSITY,
            ),
            (
                "HP_REFINED_AL_LI_STRINGER_TANK_NAME",
                HP_REFINED_AL_LI_STRINGER_TANK_UTIL_PERCENT,
                HP_REFINED_AL_LI_STRINGER_TANK_DENSITY,
            ),
            (
                "STEEL_STIR_WELDED_TANK_NAME",
                STEEL_STIR_WELDED_TANK_UTIL_PERCENT,
                STEEL_STIR_WELDED_TANK_DENSITY,
            ),
            (
                "HP_STEEL_STIR_WELDED_TANK_NAME",
                HP_STEEL_STIR_WELDED_TANK_UTIL_PERCENT,
                HP_STEEL_STIR_WELDED_TANK_DENSITY,
            ),
        ]);
    }
}
