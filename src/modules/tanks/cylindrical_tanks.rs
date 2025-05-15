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

    
}