use std::sync::LazyLock;

use crate::modules::{engines::{lunar_landing::NUM_LUNAR_LANDING_ENGINES, Engine, EngineConfiguration}, fuel_type::Fuel};

const NUM_ENGINES: usize = 1;
pub static OR_2019_2028_ENGINES: LazyLock<[Engine; NUM_ENGINES]> = LazyLock::new(|| init_orbital_rocketry_2019_2028_engines());

#[rustfmt::skip]
fn init_orbital_rocketry_2019_2028_engines() -> [Engine; NUM_ENGINES] {
    let engines = [
        Engine::new(
            "F-1B", // Engine 0
            false,
            8027.8,
            8815.0,
            72.0,
            272.3,
            299.0,
            9.6567,
            0.4,
            315.0,
            true,
            false,
            false,
            true,
            true,
            1,
            3.75,
            0.0,
            vec![
                Fuel::new("RP-1", 1012149.41, 388462.955086835, 20.763 * 1000.0, 334.252 * 1000.0, 5, 38.8, false),
                Fuel::new("Liquid Oxygen", 1012149.41, 623686.454913165, 24.174 * 1000.0, 735.800 * 1000.0, 5, 38.8, false),
                // FuelType::Kerosene(0.218, 0.169),
                // FuelType::AK20(0.441, 0.677),
                // FuelType::Nitrogen(25.0, 31.3),
            ],
            "2019-2028 Orbital Rocketry",
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
            24.5,
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