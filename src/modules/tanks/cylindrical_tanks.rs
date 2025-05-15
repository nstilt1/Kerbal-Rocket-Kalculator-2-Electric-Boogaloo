//! Module for calculating the volume and wet mass of cyllindrical tanks.

use super::{TankType, Tanks};

pub struct CylindricalTank {

}

impl Tanks for CylindricalTank {
    /// MIN_VSA Height ratio. min height = 1/2 diameter
    const MIN_VSA: f64 = 0.5;
    const MAX_VSA: f64 = 50.0;
    const TANK_TYPE: TankType = TankType::Cylindrical;
}

/// Calculates the corrected volume of a cylindrical tank.
fn calculate_corrected_volume(diameter: f64, height: f64, correction_coefficient: f64) -> f64 {
    let radius = diameter / 2.0;
    let volume = std::f64::consts::PI * radius.powi(2) * height * 1000.0;
    volume * correction_coefficient * 0.83
}

#[cfg(test)]
mod tests {
    use super::*;

    // using a const since you can't easily pass arguments to `cargo test`
    const PRINT_STATS: bool = true;

    fn cylinder_volume(radius: f64, height: f64) -> f64 {
        std::f64::consts::PI * radius * radius * height
    }
    fn ellipsoid_volume(a: f64, b: f64, c: f64) -> f64 {
        4.0 / 3.0 * std::f64::consts::PI * (a * b * c)
    }
    fn tank_volume(diameter: f64, height: f64) -> f64 {
        let r = diameter / 2.0;
        // (ellipsoid_volume(r, r, r/2.0) + cylinder_volume(r, height)) * 1000.0 - 392.6991174
        let base_volume = (ellipsoid_volume(r, r, r/2.0) + cylinder_volume(r, height)) * 1000.0;
        base_volume - 392.6991174
        //let correction_factor = 392.6991174 * diameter.powi(2);
        //base_volume - correction_factor
    }

    #[test]
    fn cylindrical_tank_constants() {
        
    }

    fn try_calc_volume(samples: &[(f64, f64, f64)]) {
        for (i, &(diameter, height, actual_volume)) in samples.iter().enumerate() {
            let actual_volume = actual_volume;
            let v = tank_volume(diameter, height) * 0.83;
            if (v - actual_volume).abs() > 0.001 {
                println!("\nEstimate: {:.5}\nActual: {:.5}\nSample: {:.5}\nError: {:.10}", v, actual_volume, i, actual_volume - v);
                //assert!((v - actual_volume).abs() < 0.000001);
            }
            //assert!((v - actual_volume).abs() < 0.01, "\nEstimate: {:.5}\nActual: {:.5}\nSample: {:.5}\nError: {:.5}", v, actual_volume, i, actual_volume - v);
            // let wall_thickness = 0.817;
            // let mut v = (wall_thickness * diameter) * (wall_thickness * diameter) * 0.25 * std::f64::consts::PI * height;
            // v *= 1000.0 * 0.83;
            // assert!((v - actual_volume).abs() < 1.1, "\nEstimate: {}\nActual: {}\nSample: {}\n", v, actual_volume, i);
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
            let volume = calculate_corrected_volume(diameter, height, correction_coefficient);
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
            //(5.0, 3.0, 35310.1939),
        ],
        &[
            (1.0, 0.5, 217.2935, 0.4),
            (1.0, 1.0, 543.2337, 60.08),
            (1.0, 2.0, 1195.1142, 6.5),
            (1.0, 0.5, 217.2935, 67.4),
            (1.0, 1.0, 543.2337, 26.08),
            (1.0, 2.0, 1195.1142, 56.5),
            (0.5, 1.0, 149.3893, 0.000001),
            (2.0, 3.0, 6953.3923, 0.00001),
        ]
    );
}