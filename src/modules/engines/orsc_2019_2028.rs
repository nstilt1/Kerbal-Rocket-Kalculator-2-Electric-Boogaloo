use std::sync::LazyLock;

use crate::modules::{
    engines::{lunar_landing::NUM_LUNAR_LANDING_ENGINES, Engine, EngineConfiguration},
    fuel_type::Fuel,
};

const NUM_ENGINES: usize = 3;
pub static ORSC_2019_2028_ENGINES: LazyLock<[Engine; NUM_ENGINES]> =
    LazyLock::new(|| init_engines());

#[rustfmt::skip]
fn init_engines() -> [Engine; NUM_ENGINES] {
    let engines = [
        Engine::new(
            "Raptor-Vacuum", // Engine 0
            false,
            1238.7,
            1961.3,
            40.0,
            240.0,
            380.0,
            2.912,
            0.8,
            3600.0,
            true,
            false,
            false,
            true,
            true,
            u8::MAX,
            3.5,
            0.0,
            vec![
                Fuel::new("Subcooled Liquid Methane", 461180.8316, 198307.760886622, 9769.0, 99034.0, 12, 58.2, false),
                Fuel::new("Subcooled Liquid Oxygen", 461180.8316, 262873.070713378, 10706.0, 335617.0, 12, 58.2, false),
                // FuelType::Kerosene(0.218, 0.169),
                // FuelType::AK20(0.441, 0.677),
                // FuelType::Nitrogen(25.0, 31.3),
            ],
            "2019-2028 ORSC Engines",
        ),
        Engine::new(
            "Raptor-1", // Engine 1
            false,
            1710.6,
            1814.2,
            40.0,
            330.0,
            350.0,
            2.08,
            0.8,
            1800.0,
            true,
            false,
            false,
            true,
            true,
            u8::MAX,
            2.5,
            0.0,
            vec![
                Fuel::new("Subcooled Liquid Methane", 236089.8914, 101589.479638992, 5002.0, 50731.0, 6, 36.6, false),
                Fuel::new("Subcooled Liquid Oxygen", 236089.8914, 134500.411761008, 5479.0, 171722.0, 6, 36.6, false),
                // FuelType::Kerosene(0.218, 0.169),
                // FuelType::AK20(0.441, 0.677),
                // FuelType::Nitrogen(25.0, 31.3),
            ],
            "2019-2028 ORSC Engines",
        ),
        Engine::new(
            "BE-4", // Engine 2
            false,
            2368.0,
            2647.5,
            30.0,
            305.0,
            341.0,
            2.25,
            0.7,
            400.0,
            true,
            false,
            false,
            true,
            true,
            4,
            3.5,
            0.0,
            vec![
                Fuel::new("Liquid Methane", 451848.3403, 192848.877328133, 9551.0, 91629.0, 7, 52.1, false),
                Fuel::new("Liquid Oxygen", 451848.3403, 258999.462971867, 10510.0, 306028.0, 7, 52.1, false),
                // FuelType::Kerosene(0.218, 0.169),
                // FuelType::AK20(0.441, 0.677),
                // FuelType::Nitrogen(25.0, 31.3),
            ],
            "2019-2028 ORSC Engines",
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
