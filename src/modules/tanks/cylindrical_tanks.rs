//! Module for calculating the volume and wet mass of cyllindrical tanks.

use super::{TankType, Tanks};

const STEEL_FUSELAGE_DENSITY: f64 = 0.7046255853625588; // kg/L
const STEEL_FUSELAGE_UTIL_PERCENT: f64 = 83.0;
const HP_STEEL_FUSELAGE_DENSITY: f64 = 1.161194204449523; // kg/L
const HP_STEEL_FUSELAGE_UTIL_PERCENT: f64 = 75.0;
const AL_FUSELAGE_DENSITY: f64 = 0.572370017780281; // kg/L
const AL_FUSELAGE_UTIL_PERCENT: f64 = 87.0;
const HP_AL_FUSELAGE_DENSITY: f64 = 1.6233800555626554; // kg/L
const HP_AL_FUSELAGE_UTIL_PERCENT: f64 = 84.0;
const AL_STRINGER_TANK_DENSITY: f64 = 0.6474421633361649; // kg/L
const AL_STRINGER_TANK_UTIL_PERCENT: f64 = 92.0;
const HP_AL_STRINGER_TANK_DENSITY: f64 = 2.1543208266760887; // kg/L
const HP_AL_STRINGER_TANK_UTIL_PERCENT: f64 = 90.0;
const REFINED_AL_STRINGER_TANK_DENSITY: f64 = 0.4793745811132077; // kg/L
const REFINED_AL_STRINGER_TANK_UTIL_PERCENT: f64 = 92.0;
const HP_REFINED_AL_STRINGER_TANK_DENSITY: f64 = 1.6195603377848609; // kg/L
const HP_REFINED_AL_STRINGER_TANK_UTIL_PERCENT: f64 = 90.0;
const AL_LI_STRINGER_TANK_DENSITY: f64 = 1.0134984503748028; // kg/L
const AL_LI_STRINGER_TANK_UTIL_PERCENT: f64 = 97.0;
const HP_AL_LI_STRINGER_TANK_DENSITY: f64 = 2.3147489733434568; // kg/L
const HP_AL_LI_STRINGER_TANK_UTIL_PERCENT: f64 = 96.0;
const REFINED_AL_LI_STRINGER_TANK_DENSITY: f64 = 0.9523829659300911; // kg/L
const REFINED_AL_LI_STRINGER_TANK_UTIL_PERCENT: f64 = 97.0;
const HP_REFINED_AL_LI_STRINGER_TANK_DENSITY: f64 = 1.9977123977865145; // kg/L
const HP_REFINED_AL_LI_STRINGER_TANK_UTIL_PERCENT: f64 = 96.0;
const STEEL_STIR_WELDED_TANK_DENSITY: f64 = 1.1815660325977602; // kg/L
const STEEL_STIR_WELDED_TANK_UTIL_PERCENT: f64 = 97.0;
const HP_STEEL_STIR_WELDED_TANK_DENSITY: f64 = 3.4033685400148843; // kg/L
const HP_STEEL_STIR_WELDED_TANK_UTIL_PERCENT: f64 = 96.0;

pub struct CylindricalTank {

}

impl Tanks for CylindricalTank {
    /// MIN_VSA Height ratio. min height = 1/2 diameter
    const MIN_VSA: f64 = 0.5;
    const MAX_VSA: f64 = 50.0;
    const TANK_TYPE: TankType = TankType::Cylindrical;
}

/// Calculates the volume of a cylinder.
fn cylinder_volume(radius: f64, height: f64) -> f64 {
    std::f64::consts::PI * radius * radius * height
}
/// Calculates the volume of an ellipsoid.
fn ellipsoid_volume(a: f64, b: f64, c: f64) -> f64 {
    4.0 / 3.0 * std::f64::consts::PI * (a * b * c)
}

const K: f64 = 392.69893495497905;
//const N: f64 = 3.0000008452405535;

/// Calculates the volume of a tank with no nose or mount. Applies a correction 
/// factor discovered during testing.
fn tank_volume(diameter: f64, height: f64) -> f64 {
    let r = diameter / 2.0;
    // (ellipsoid_volume(r, r, r/2.0) + cylinder_volume(r, height)) * 1000.0 - 392.6991174
    let base_volume = (ellipsoid_volume(r, r, r/2.0) + cylinder_volume(r, height)) * 1000.0;
    let correction_factor = K * diameter.powi(3);
    base_volume - correction_factor
}

#[cfg(test)]
mod tests {
    use super::*;

    // using a const since you can't easily pass arguments to `cargo test`
    const PRINT_STATS: bool = true;

    fn check_samples(samples: &[(f64, f64, f64)]) {
        println!("Checking samples with diameter {:.3}", samples[0].0);
        for (i, &(diameter, height, volume)) in samples.iter().enumerate() {
            let volume = volume / 83.0 * 100.0;
            let estimate = tank_volume(diameter, height);
            println!("Sample {} - Estimate: {:.5} Actual: {:.5} Error: {:.10}", i, estimate, volume, volume - estimate);
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

        let n = (n_samples * sum_log_d_log_error - sum_log_d * sum_log_error) /
                (n_samples * sum_log_d_sq - sum_log_d * sum_log_d);

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
                println!("\nEstimate: {:.5}\nActual: {:.5}\nSample: {:.5}\nError: {:.10}", v, actual_volume, i, actual_volume - v);
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
    fn calculate_dry_mass_coefficient(samples: &[(f64, f64, f64, f64, f64)]) -> f64 {
        let mut sum_density = 0.0;
        let mut count = 0;
    
        for &(diameter, height, dry_mass, max_utilization, correction_coefficient) in samples {
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
                    assert!(diff.abs() < error, "Diff for sample {} = {}\nExpected = {}\nEstimation = {}", i + 1, diff, expected, unused_mass);
                }
            }
        };
    }

    dry_mass_test!(
        &[
            (STEEL_FUSELAGE_UTIL_PERCENT, STEEL_FUSELAGE_DENSITY, 9800.0, 100.0),
            (HP_STEEL_FUSELAGE_UTIL_PERCENT, HP_STEEL_FUSELAGE_DENSITY, 23700.0, 100.0),
            (AL_FUSELAGE_UTIL_PERCENT, AL_FUSELAGE_DENSITY, 6080.0, 20.0),
            (HP_AL_FUSELAGE_UTIL_PERCENT, HP_AL_FUSELAGE_DENSITY, 21200.0, 100.0),
            (AL_STRINGER_TANK_UTIL_PERCENT, AL_STRINGER_TANK_DENSITY, 4240.0, 20.0),
            (HP_AL_STRINGER_TANK_UTIL_PERCENT, HP_AL_STRINGER_TANK_DENSITY, 17600.0, 100.0),
            (REFINED_AL_STRINGER_TANK_UTIL_PERCENT, REFINED_AL_STRINGER_TANK_DENSITY, 3140.0, 10.0),
            (HP_REFINED_AL_STRINGER_TANK_UTIL_PERCENT, HP_REFINED_AL_STRINGER_TANK_DENSITY, 13200.0, 100.0),
            (AL_LI_STRINGER_TANK_UTIL_PERCENT, AL_LI_STRINGER_TANK_DENSITY, 2500.0, 13.0),
            (HP_AL_LI_STRINGER_TANK_UTIL_PERCENT, HP_AL_LI_STRINGER_TANK_DENSITY, 7570.0, 10.0),
            (REFINED_AL_LI_STRINGER_TANK_UTIL_PERCENT, REFINED_AL_LI_STRINGER_TANK_DENSITY, 2340.0, 10.0),
            (HP_REFINED_AL_LI_STRINGER_TANK_UTIL_PERCENT, HP_REFINED_AL_LI_STRINGER_TANK_DENSITY, 6540.0, 10.0),
            (STEEL_STIR_WELDED_TANK_UTIL_PERCENT, STEEL_STIR_WELDED_TANK_DENSITY, 2900.0, 100.0),
            (HP_STEEL_STIR_WELDED_TANK_UTIL_PERCENT, HP_STEEL_STIR_WELDED_TANK_DENSITY, 11100.0, 100.0)
        ]
    );

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
        let diff = (unusable_volume * HP_AL_FUSELAGE_DENSITY - 21200.0);
        assert!(diff.abs() < 50.1, "Diff = {}", diff);
    }

    fn calculate_density(fuselage: &str, utilization: f64, dry_mass_kg: f64) {
        let volume = tank_volume(1.0, 1.0);
        let unused_volume = (100.0 - utilization) / 100.0 * volume;
        let density = dry_mass_kg / unused_volume;
        let var_name = {
            let replaced = fuselage.replace(' ', "_");
            let replaced = replaced.replace('-', "_");
            replaced.to_ascii_uppercase()
        };
        println!("const {}_DENSITY: f64 = {}; // kg/L", var_name, density);
        println!("const {}_UTIL_PERCENT: f64 = {:.1};", var_name, utilization);
    }

    #[test]
    fn dry_mass_tests_v2() {
        println!("\n");
        let density = calculate_density("Steel Fuselage", 83.0, 78.4);
        calculate_density("HP Steel Fuselage", 75.0, 190.0);
        calculate_density("Al Fuselage", 87.0, 48.7);
        calculate_density("HP Al Fuselage", 84.0, 170.0);
        calculate_density("Al Stringer Tank", 92.0, 33.9);
        calculate_density("HP Al Stringer Tank", 90.0, 141.0);
        calculate_density("Refined Al Stringer Tank", 92.0, 25.1);
        calculate_density("HP Refined Al Stringer Tank", 90.0, 106.0);
        calculate_density("Al-Li Stringer Tank", 97.0, 19.9);
        calculate_density("HP Al-Li Stringer Tank", 96.0, 60.6);
        calculate_density("Refined Al-Li Stringer Tank", 97.0, 18.7);
        calculate_density("HP Refined Al-Li Stringer Tank", 96.0, 52.3);
        calculate_density("Steel Stir-Welded Tank", 97.0, 23.2);
        calculate_density("HP Steel Stir-Welded Tank", 96.0, 89.1);
    }
}