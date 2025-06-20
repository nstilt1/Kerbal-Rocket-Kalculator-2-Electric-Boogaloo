use std::sync::LazyLock;

use crate::modules::{engines::{lunar_landing::NUM_LUNAR_LANDING_ENGINES, Engine, EngineConfiguration}, fuel_type::Fuel};

const NUM_ENGINES: usize = 1;
pub static ORSC_2014_2018_ENGINES: LazyLock<[Engine; NUM_ENGINES]> = LazyLock::new(|| init_orsc_2014_2018_engines());

#[rustfmt::skip]
fn init_orsc_2014_2018_engines() -> [Engine; NUM_ENGINES] {
    let mut engines = [
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

    engines.iter_mut().find(|e| e.name == "RD-191").unwrap().configurations.push(
        EngineConfiguration::new(
            "RD-181",
            1917.2,
            2085.0,
            47.0,
            2.2,
            311.9, 
            339.2,
            255.0,
            true,
            false,
            1,
            vec![
                Fuel::new("RP-1", 188554.5109, 66408.897038561, 3782.0, 57374.0, 5, 5.8, false),
                Fuel::new("Liquid Oxygen", 188554.5109, 122145.613861439, 4590.0, 143958.0, 5, 5.8, false)
            ],
            "2014-2018 ORSC Engines",
        )
    );
    engines
}