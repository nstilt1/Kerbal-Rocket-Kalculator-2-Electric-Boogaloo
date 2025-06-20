use std::sync::LazyLock;

use crate::modules::{engines::{or_2009_2013::NUM_OR_2009_2013_ENGINES, Engine, EngineConfiguration}, fuel_type::Fuel};

const NUM_ENGINES: usize = 2;
pub static OR_2014_2018_ENGINES: LazyLock<[Engine; NUM_ENGINES]> = LazyLock::new(|| init_engines());

#[rustfmt::skip]
fn init_engines() -> [Engine; NUM_ENGINES] {
    let engines = [
        Engine::new(
            "Rutherford Vacuum", // Engine 0
            false,
            15.0,
            25.8,
            68.0,
            200.0,
            343.0,
            0.04,
            0.8,
            288.0,
            true,
            false,
            false,
            true,
            true,
            5,
            1.0,
            0.0,
            vec![
                Fuel::new("RP-1", 2498.8753, 902.593743495332, 265.0, 993.0, 5, 29.0, false),
                Fuel::new("Liquid Oxygen", 2498.8753, 1596.28155650467, 307.0, 2128.0, 5, 29.0, false),
                // FuelType::Kerosene(0.218, 0.169),
                // FuelType::AK20(0.441, 0.677),
                // FuelType::Nitrogen(25.0, 31.3),
            ],
            "2014-2018 Orbital Rocketry",
        ),
        Engine::new(
            "Rutherford", // Engine 1
            false,
            24.4,
            24.9,
            68.0,
            311.0,
            317.8,
            0.035,
            0.8,
            150.0,
            true,
            false,
            false,
            true,
            true,
            5,
            1.0,
            0.0,
            vec![
                Fuel::new("RP-1", 1777.618, 642.075611025762, 35.9, 554.0, 3, 44.0, false),
                Fuel::new("Liquid Oxygen", 1777.618, 1135.54238897424, 43.0, 1339.0, 3, 44.0, false),
                // FuelType::Kerosene(0.218, 0.169),
                // FuelType::AK20(0.441, 0.677),
                // FuelType::Nitrogen(25.0, 31.3),
            ],
            "2014-2018 Orbital Rocketry",
        ),
    ];
    engines
}

#[rustfmt::skip]
pub(super) fn orbital_rocketry_2014_2018_mod(engines: &mut [Engine; NUM_OR_2009_2013_ENGINES]) {
    engines.iter_mut().find(|e| e.name == "Merlin 1D").unwrap().configurations.push(
        EngineConfiguration::new(
            "Merlin 1D+",
            748.1,
            825.0,
            40.0,
            0.470,
            282.0,
            311.0,
            162.0,
            true,
            false,
            4,
            vec![
                Fuel::new("Subcooled RP-1", 47133.4661, 18273.6449492648, 5072.0, 20184.0, 3, 6.0, false),
                Fuel::new("Subcooled Liquid Oxygen", 47133.4661, 28859.8211507352, 5707.0, 41377.0, 3, 6.0, false),
            ],
            "2014-2018 Orbital Rocketry",
        )
    );

    engines.iter_mut().find(|e| e.name == "Merlin 1D").unwrap().configurations.push(
        EngineConfiguration::new(
            "Merlin 1D++",
            848.0,
            914.1,
            36.0,
            0.470,
            288.5,
            311.0,
            162.0,
            true,
            false,
            4,
            vec![
                Fuel::new("Subcooled RP-1", 52508.6804, 20357.6155496024, 1080.0, 17916.0, 3, 7.0, false),
                Fuel::new("Subcooled Liquid Oxygen", 52508.6804, 32151.0648503976, 1251.0, 40990.0, 3, 7.0, false),
            ],
            "2014-2018 Orbital Rocketry",
        )
    );

    engines.iter_mut().find(|e| e.name == "Merlin 1D Vacuum").unwrap().configurations.push(
        EngineConfiguration::new(
            "Merlin 1D Vac+",
            609.3,
            934.1,
            39.0,
            0.490,
            227.0,
            348.0,
            400.0,
            true,
            false,
            4,
            vec![
                Fuel::new("Subcooled RP-1", 102768.5445, 38497.0960738452, 2094.0, 33162.0, 7, 14.6, false),
                Fuel::new("Subcooled Liquid Oxygen", 102768.5445, 64271.4484261548, 2468.0, 75802.0, 7, 14.6, false),
            ],
            "2014-2018 Orbital Rocketry",
        )
    );
}