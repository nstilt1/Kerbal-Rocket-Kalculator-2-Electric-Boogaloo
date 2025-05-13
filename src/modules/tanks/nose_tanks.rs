//! Module for calculating the volume and wet mass of a nose cone tank.

#[derive(Debug, PartialEq, Clone)]
pub struct NoseConeVariant {
    pub name: String,
    pub cores: Vec<NoseTankCore>,
}

impl NoseConeVariant {
    pub fn nosecones() -> Self {
        let cores = vec![
            NoseTankCore::new("Nose-1", 1.2608, 1.34180454434038853861466122907586),
            NoseTankCore::new("Nose-2", 1.3558, 1.62346946577909534425998572260141),
            NoseTankCore::new("Nose-3", 0.6402, 1.35970328040517141054976946179522),
            NoseTankCore::new("Nose-4", 0.3914, 1.62497042872616614950231905822875),
            NoseTankCore::new("Nose-5", 1.2148, 1.27217604871392087062531572883017),
            NoseTankCore::new("Nose-12", 5.0000, 0f64),
        ];
        NoseConeVariant {
            name: "Nosecones".to_string(),
            cores,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct NoseTankCore {
    pub name: String,
    /// The length of the tank in meters with V.ScaleAdj = 1.0000
    pub base_length: f64,
    correction_coefficient: f64,
}

impl NoseTankCore {
    /// Create a new NoseTank with the given name and base length.
    pub fn new(name: &str, base_length: f64, correction_coefficient: f64) -> Self {
        NoseTankCore { name: name.to_string(), base_length, correction_coefficient }
    }
}

/// Calculate the corrected volume of the tank based on the diameter, height,
/// and correction coefficient.
fn calculate_corrected_volume(diameter: f64, height: f64, correction_coefficient: f64) -> f64 {
    let radius = diameter / 2.0;
    let ideal_volume_liters = ((std::f64::consts::PI * radius.powi(2) * height) / 3.0) * 1000.0;
    let corrected_volume = ideal_volume_liters * correction_coefficient;
    corrected_volume
}

/// Calculate the dry mass for a nose tank based on the diameter, height, and 
/// the coefficient.
fn calculate_dry_mass(diameter: f64, height: f64, coefficient: f64) -> f64 {
    coefficient * diameter * height
}

fn calculate_cone_lengths(diameter: f64) -> (f64, f64, f64) {
    let base_length = diameter * 1.2608;
    let min_length = base_length * 0.25;
    let max_length = base_length * 4.0;
    (base_length, min_length, max_length)
}

#[cfg(test)]
mod tests {
    use super::*;

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
            let estimated_volume_liters = ((std::f64::consts::PI * radius.powi(2) * height) / 3.0) * 1000.0;
            let correction_factor = actual_volume / estimated_volume_liters;

            sum_ratio += correction_factor;
            count += 1;
        }

        sum_ratio / count as f64 // Average correction coefficient
    }

    /// Generate tests for nosecone tanks based on the provided samples and test 
    /// samples.
    macro_rules! impl_nosecone_test {
        ($name:ident, $core:literal, $samples:expr, $test_samples:expr) => {
            #[test]
            fn $name() {
                let correction_coefficient = calculate_correction_coefficient($samples);
                println!("\nCorrection coefficient for core '{}': {:.32}\n", $core, correction_coefficient);

                for (i, &(diameter, height, expected, error)) in $test_samples.iter().enumerate() {
                    let corrected_volume = calculate_corrected_volume(diameter, height, correction_coefficient);
                    let diff = (corrected_volume - expected).abs();
                    assert!(diff < error, "\nDifference for sample {} is too large: {}\n", i + 1, diff);
                }
            }
        };
    }
    // nose-1
    impl_nosecone_test!(
        tank_volume_correction_core_nose_1,
        "nose-1",
        &[
            (1.3, 0.4098, 243.2632),    // V.ScaleAdj = 0.2500
            //(1.3, 0.6556, 389.2212),    // V.ScaleAdj = 0.4000
            (1.3, 0.8195, 486.5264),    // V.ScaleAdj = 0.5000
            //(1.3, 0.9834, 583.83169999),// V.ScaleAdj = 0.6000
            (1.3, 1.2293, 729.7897),    // V.ScaleAdj = 0.7500
            //(1.3, 1.4751, 875.7475999), // V.ScaleAdj = 0.9000
            (1.3, 1.639, 973.0528),     // V.ScaleAdj = 1.0000
            //(1.3, 1.721, 1021.7055),    // V.ScaleAdj = 1.0500
            (1.3, 2.2127, 1313.6213),   // V.ScaleAdj = 1.3500
            (1.3, 2.8683, 1702.8424),   // V.ScaleAdj = 1.7500
            (1.3, 3.2781, 1946.1054),   // V.ScaleAdj = 2.0000
            (1.3, 4.0976, 2432.6318),   // V.ScaleAdj = 2.5000
            (1.3, 4.9171, 2919.1586),   // V.ScaleAdj = 3.0000
            (1.3, 5.7366, 3405.6849),   // V.ScaleAdj = 3.5000
            (1.3, 6.5562, 3892.2112999),// V.ScaleAdj = 4.0000
        ],
        &[
            (1.3, 0.6556, 389.2212, 0.02),    // V.ScaleAdj = 0.4000
            (1.3, 2.2947, 1362.274, 0.02),    // V.ScaleAdj = 1.3500
            (3.0, 5.2954, 16741.6431, 0.05),  // V.ScaleAdj = 2.0000
            (5.0, 12.6080, 110725.1485, 0.6),// V.ScaleAdj = 3.5000
        ]
    );
    // nose-2
    impl_nosecone_test!(
        tank_volume_correction_core_nose_2,
        "nose-2",
        &[
            (1.3, 0.4406, 316.5162),        // V.ScaleAdj = 0.2500
            (1.3, 0.8813, 633.032299999),   // V.ScaleAdj = 0.5000
            (1.3, 1.3219, 949.548599999),   // V.ScaleAdj = 0.7500
            (1.3, 1.7625, 1266.0646),       // V.ScaleAdj = 1.0000
            (1.3, 2.2032, 1582.5808),       // V.ScaleAdj = 1.2500
            (1.3, 2.6438, 1899.0973),       // V.ScaleAdj = 1.5000
            (1.3, 3.0844, 2215.6133),       // V.ScaleAdj = 1.7500
            (1.3, 3.5251, 2531.1292),       // V.ScaleAdj = 2.0000
        ],
        &[
            (1.3, 2.3794, 1709.1875, 0.09), // V.ScaleAdj = 1.3500
            (1.3, 7.0502, 5064.25849, 0.18),// V.ScaleAdj = 4.0000
            (3.0, 4.0674, 15559.2816, 0.64),// V.ScaleAdj = 1.0000
            (3.0, 12.2022, 46677.84629, 1.88),//VScaleAdj = 3.0000
            (5.0, 20.3370, 216101.1182, 8.65),//V.ScaleAdj = 3.0000
        ]
    );
    // nose-3
    impl_nosecone_test!(
        tank_volume_correction_core_nose_3,
        "nose-3",
        &[
            (1.3, 0.2081, 125.1712),        // VSA = 0.25
            (1.3, 0.4161, 250.3423999),     // VSA = 0.50
            (1.3, 0.6242, 375.5136999),     // VSA = 0.75
            (1.3, 0.8323, 500.6848999),     // VSA = 1.00
            //(1.3, 1.0403, 625.8560999),     // VSA = 1.25
            (1.3, 1.2484, 751.0273999),     // VSA = 1.50
            //(1.3, 1.4565, 876.1985999),     // VSA = 1.75
            (1.3, 1.6645, 1001.3697),       // VSA = 2.00
            //(1.3, 1.8726, 1126.541),        // VSA = 2.25
            (1.3, 2.0807, 1251.7121),       // VSA = 2.50
            (1.3, 2.4968, 1502.0548),       // VSA = 3.00
            (1.3, 2.9129, 1752.3972),       // VSA = 3.50
            (1.3, 3.3290, 2002.7394),       // VSA = 4.00
            (3.0, 4.8015, 15382.8958),      // VSA = 2.50
            (3.0, 6.7221, 21536.0562999),   // VSA = 3.50
        ],
        &[
            (1.3, 0.5410, 325.4451999, 0.02),// VSA = 0.65
            (1.3, 1.1236, 675.9245999, 0.022),// VSA = 1.35
            (3.0, 1.9206, 6153.158999, 0.09),// VSA = 1.00
            (3.0, 5.7618, 18459.4769, 0.26),// VSA = 3.00
            (5.0, 3.2010, 28486.844, 0.39), // VSA = 1.00
            (5.0, 9.6030, 85460.528, 1.2), // VSA = 3.00
        ]
    );
    // nose-4
    impl_nosecone_test!(
        tank_volume_correction_core_nose_4,
        "nose-4",
        &[
            (1.3, 0.1272, 91.45089999),         // VSA = 0.25
            (1.3, 0.5088, 365.8034999),         // VSA = 1.00
            (1.3, 1.0176, 731.6069999),         // VSA = 2.00
        ],
        &[
            (1.3, 0.3053, 219.4820999, 0.02),       // VSA = 0.60
            (1.3, 0.7632, 548.7052999, 0.02),       // VSA = 1.50
            (1.3, 1.9081, 1371.7632, 0.075),        // VSA = 3.75
            (3.0, 1.1742, 4495.535999, 0.20),       // VSA = 1.00
            (3.0, 3.5226, 13486.6087, 0.54),        // VSA = 3.00
            (5.0, 0.9785, 10406.3327, 0.42),        // VSA = 0.50
            (5.0, 6.8495, 72844.328999, 2.89),      // VSA = 3.50
        ]
    );
    // nose-5
    impl_nosecone_test!(
        tank_volume_correction_core_nose_5,
        "nose-5",
        &[
            (1.0, 1.2148, 404.5950999),         // VSA = 1.00
            (5.0, 18.2220, 151723.1667),        // VSA = 3.00
        ],
        &[
            (1.0, 0.6074, 202.2976, 0.00005),       // VSA = 0.50
            (1.3, 4.7377, 2666.6870999, 0.02),      // VSA = 3.00
            (3.0, 9.1110, 27310.170599, 0.001),     // VSA = 2.50
            (5.0, 15.185, 126435.9659, 0.005),      // VSA = 2.50
        ]
    );
    // nose-12
    impl_nosecone_test!(
        tank_volume_correction_core_nose_12,
        "nose-12",
        &[
            (1.0, 5.0000, 1665.1993),               // VSA = 1.00
            (5.0, 100.0, 832599.5758),              // VSA = 4.00
        ],
        &[
            (1.3, 3.2500, 1829.2216, 0.0003),       // VSA = 0.50
            (1.3, 9.7500, 5487.6655, 0.002),        // VSA = 1.50
            (1.3, 19.500, 10975.3311, 0.004),       // VSA = 3.00
            (3.0, 45.000, 134881.1424, 0.006),      // VSA = 3.00
        ]
    );

    // nose-13
    impl_nosecone_test!(
        tank_volume_correction_core_nose_13,
        "nose-13",
        &[
            (1.0, 3.7760, 1326.4132),               // VSA = 1.0
            (5.0, 75.520, 663206.5253),             // VSA = 4.0
        ],
        &[
            (1.3, 2.4544, 1457.0651, 0.0003),         // VSA = 0.50
            (1.3, 7.3632, 4371.1954, 0.001),         // VSA = 1.50
            (3.0, 11.3280, 35813.1556, 0.0013),       // VSA = 1.00
            (3.0, 28.3200, 89532.8875, 0.0016),       // VSA = 2.50
            (5.0, 37.7600, 331603.2626, 0.019),      // VSA = 2.00
        ]
    );

    /// Calculates the dry mass coefficient for some nose tanks
    fn calculate_dry_mass_coefficient(samples: &[(f64, f64, f64)]) -> f64 {
        let mut sum_ratio = 0.0;
        let mut count = 0;
    
        for &(diameter, height, dry_mass) in samples {
            let coefficient = dry_mass / (diameter * height);
            sum_ratio += coefficient;
            count += 1;
        }
    
        sum_ratio / count as f64 // Returns the average coefficient
    }
    
    /// Implements some tests for finding the dry mass coefficient and validates 
    /// that the coefficient is correct.
    macro_rules! impl_nosecone_mass_test {
        ($name:ident, $core:literal, $tank_type:literal, $samples:expr, $test_samples:expr) => {
            #[test]
            fn $name() {
                let dry_mass_coefficient = calculate_dry_mass_coefficient($samples);
                println!("\nDry mass coefficient for core '{}:{}': {:.5}\n", $core, $tank_type, dry_mass_coefficient);

                for (i, &(diameter, height, expected, error)) in $test_samples.iter().enumerate() {
                    let estimated_dry_mass = calculate_dry_mass(diameter, height, dry_mass_coefficient);
                    let diff = (estimated_dry_mass - expected).abs();
                    assert!(diff < error, "Dry mass Difference for sample {} is too large: {}\n", i + 1, diff);
                }
            }
        };
    }

    impl_nosecone_mass_test!(
        tank_dry_mass_core_nose_1_steel_fuselage,
        "Nose-1",
        "Steel Fuselage",
        &[
            (0.1, 0.0315, 0.016),//VSA = 0.25
            (0.1, 0.1261, 0.0639),//VSA = 1.00
            (1.0, 0.3152, 16.0),//VSA = 0.25
            (1.0, 0.3782, 19.2),//VSA = 0.30
            (1.0, 1.2608, 63.9),//VSA = 1.00
        ],
        &[
            (3.0, 7.5648, 3045.0, 0.5),//VSA = 2.0
        ]
    );

    #[test]
    fn test_tank_length() {
        let diameter = 5.0;
        let (base_length, min_length, max_length) = calculate_cone_lengths(diameter);
        let expected_base_length = 6.3040;
        assert!((base_length - expected_base_length).abs() < 0.01, "Base length is incorrect");
        let diameter = 3.0;
        let (base_length, min_length, max_length) = calculate_cone_lengths(diameter);
        let expected_base_length = 3.7824;
        assert!((base_length - expected_base_length).abs() < 0.01, "Base length is incorrect");
    }
}