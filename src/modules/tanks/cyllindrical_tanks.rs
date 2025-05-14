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

    /// Calculates the correction coefficient for cylindrical tanks.
    fn calculate_correction_coefficient(samples: &[(f64, f64, f64)]) -> f64 {
        let total_ratio: f64 = samples.iter().map(|&(d, h, actual_volume)| {
            let ideal_volume = std::f64::consts::PI * (d / 2.0).powi(2) * h * 1000.0 * 0.83;
            println!("Acutal: {}\nIdeal: {}", actual_volume, ideal_volume);
            actual_volume / ideal_volume
        }).sum();
        
        total_ratio / samples.len() as f64
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
                    let corrected_volume = calculate_corrected_volume(diameter, height, correction_coefficient);
                    let diff = (corrected_volume - expected).abs();
                    assert!(diff < error, "\nDifference for sample {} is too large: {}\nEstimated: {}\nActual: {}\n", i + 1, diff, corrected_volume, expected);
                }
            }
        };
    }

    impl_cylindrical_tank_test!(
        tank_volume_correction_core_1_0x_kerolox,
        "1.0x-Kerolox",
        &[
            (1.0, 1.0, 543.2337),
            (1.0, 2.0, 1195.1142),
            (1.0, 5.0, 3150.7556),
            (1.0, 10.0, 6410.1581),
            //(5.0, 3.0, 35310.1939),
        ],
        &[
            (1.0, 0.5, 217.2935, 0.01),
            (0.5, 1.0, 149.3893, 2.01),
            (2.0, 3.0, 6953.3923, 0.01),
        ]
    );
}