use std::sync::LazyLock;

use crate::modules::{engines::{lunar_landing::NUM_LUNAR_LANDING_ENGINES, Engine, EngineConfiguration}, fuel_type::Fuel};

const NUM_ENGINES: usize = 1;
pub static ORSC_2014_2018_ENGINES: LazyLock<[Engine; NUM_ENGINES]> = LazyLock::new(|| init_orsc_2014_2018_engines());

#[rustfmt::skip]
fn init_orsc_2014_2018_engines() -> [Engine; NUM_ENGINES] {
    let engines = [
        Engine::new(
            "RD-191", // Engine 0
            false,
            1922.5,
            2085.0,
            27.0,
            311.2,
            337.5,
            2.2907,
            0.4,
            255.0,
            true,
            false,
            false,
            true,
            true,
            1,
            3.0,
            0.0,
            vec![
                Fuel::new("RG-1", 188554.5109, 65051.3079744056, 3762.0, 57950.0, 5, 7.7, false),
                Fuel::new("Liquid Oxygen", 188554.5109, 123503.202925594, 4609.0, 145527.0, 5, 7.7, false),
                // FuelType::Kerosene(0.218, 0.169),
                // FuelType::AK20(0.441, 0.677),
                // FuelType::Nitrogen(25.0, 31.3),
            ],
            "2014-2018 ORSC Engines",
        ),
    ];
    engines
}

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