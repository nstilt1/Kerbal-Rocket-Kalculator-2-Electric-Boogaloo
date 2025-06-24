use std::sync::LazyLock;

use crate::modules::{
    engines::{or_2014_2018::orbital_rocketry_2014_2018_mod, Engine},
    fuel_type::Fuel,
};

pub(super) const NUM_OR_2009_2013_ENGINES: usize = 5;
pub static OR_2009_2013_ENGINES: LazyLock<[Engine; NUM_OR_2009_2013_ENGINES]> =
    LazyLock::new(|| init_orbital_rocketry_2009_2013_engines());

#[rustfmt::skip]
fn init_orbital_rocketry_2009_2013_engines() -> [Engine; NUM_OR_2009_2013_ENGINES] {
    let mut engines = [
        Engine::new(
            "Merlin 1A", // Engine 0
            false,
            324.7,
            369.2,
            100.0,
            253.7,
            288.5,
            0.7628,
            0.4,
            170.0,
            true,
            false,
            false,
            false,
            true,
            1,
            1.25,
            0.0,
            vec![
                Fuel::new("RP-1", 28320.8683, 11172.5825194512, 585.0, 9602.0, 3, 37.6, false),
                Fuel::new("Liquid Oxygen", 28320.8683, 17148.2857805488, 672.0, 20238.0, 3, 37.6, false),
                // FuelType::Kerosene(0.218, 0.169),
                // FuelType::AK20(0.441, 0.677),
                // FuelType::Nitrogen(25.0, 31.3),
            ],
            "2009-2013 Orbital Rocketry",
        ),
        Engine::new(
            "Merlin 1C", // Engine 1
            false,
            422.8,
            482.6,
            100.0,
            267.0,
            304.8,
            0.6328,
            0.4,
            170.0,
            true,
            false,
            false,
            false,
            true,
            1,
            1.3,
            0.0,
            vec![
                Fuel::new("RP-1", 31908.6274, 12587.9534812469, 660.0, 10818.0, 3, 18.2, false),
                Fuel::new("Liquid Oxygen", 31908.6274, 19320.6739187531, 757.0, 22802.0, 3, 18.2, false),
                // FuelType::Kerosene(0.218, 0.169),
                // FuelType::AK20(0.441, 0.677),
                // FuelType::Nitrogen(25.0, 31.3),
            ],
            "2009-2013 Orbital Rocketry",
        ),
        Engine::new(
            "Merlin 1C Vacuum", // Engine 1
            false,
            259.3,
            524.9,
            60.0,
            166.0,
            336.0,
            0.7628,
            0.7,
            345.0,
            true,
            false,
            false,
            true,
            true,
            4,
            2.5,
            0.0,
            vec![
                Fuel::new("Kerosene", 63490.4415, 25046.9791159312, 6857.0, 26319.0, 6, 33.8, false),
                Fuel::new("Liquid Oxygen", 63490.4415, 38443.4623840688, 7661.0, 51525.0, 6, 33.8, false),
                // FuelType::Kerosene(0.218, 0.169),
                // FuelType::AK20(0.441, 0.677),
                // FuelType::Nitrogen(25.0, 31.3),
            ],
            "2009-2013 Orbital Rocketry",
        ),
        Engine::new(
            "Merlin 1D", // Engine 3
            false,
            673.2,
            742.4,
            39.0,
            282.0,
            311.8,
            0.4728,
            0.7,
            180.0,
            true,
            false,
            false,
            true,
            true,
            4,
            1.3,
            0.0,
            vec![
                Fuel::new("RP-1", 47358.6624, 17835.272662653, 967.0, 15360.0, 3, 15.6, false),
                Fuel::new("Liquid Oxygen", 47358.6624, 29523.389737347, 1136.0, 34822.0, 3, 15.6, false),
                // FuelType::Kerosene(0.218, 0.169),
                // FuelType::AK20(0.441, 0.677),
                // FuelType::Nitrogen(25.0, 31.3),
            ],
            "2009-2013 Orbital Rocketry",
        ),
        Engine::new(
            "Merlin 1D Vacuum", // Engine 4
            false,
            504.0,
            805.0,
            45.0,
            216.0,
            345.0,
            0.4928,
            0.7,
            375.0,
            true,
            false,
            false,
            true,
            true,
            4,
            2.5,
            0.0,
            vec![
                Fuel::new("RP-1", 50392.2995, 18876.9550514893, 1027.0, 16261.0, 3, 33.1, false),
                Fuel::new("Liquid Oxygen", 50392.2995, 31515.3444485107, 1210.0, 37169.0, 3, 33.1, false),
                // FuelType::Kerosene(0.218, 0.169),
                // FuelType::AK20(0.441, 0.677),
                // FuelType::Nitrogen(25.0, 31.3),
            ],
            "2009-2013 Orbital Rocketry",
        ),
    ];

    orbital_rocketry_2014_2018_mod(&mut engines);
    engines
}

/*
#[rustfmt::skip]
pub(super) fn orbital_rocketry_2019_2028_lmae_mod(lunar_engines: &mut [Engine; NUM_LUNAR_LANDING_ENGINES]) {
    lunar_engines.iter_mut().find(|e| e.name == "LMAE").unwrap().configurations.push(
        EngineConfiguration::new(
            "RS-18",
            5.0,
            100.0,
            0.095,
            72.0,
            356.0,
            560.0,
            true,
            true,
            u8::MAX,
            vec![
                Fuel::new("Liquid Methane", 6469.718, 3263.67859737906, 404.0, 1793.0, 10, 14.2, true),
                Fuel::new("Liquid Oxygen", 6469.718, 2617.88322080276, 377.0, 3364.0, 10, 14.2, true),
                Fuel::new("Helium", 6469.718, 588.156181818183, 292.0, 313.0, 10, 14.2, true),
            ],
            "2019-2028 Orbital Rocketry",
        )
    );
}
*/
