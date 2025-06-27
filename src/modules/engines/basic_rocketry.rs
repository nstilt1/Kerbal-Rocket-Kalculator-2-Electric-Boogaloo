use std::sync::LazyLock;

use crate::modules::{
    engines::{early_engines::NUM_EARLY_ENGINES, Engine, EngineConfiguration},
    fuel_type::Fuel,
};

const NUM_ENGINES: usize = 4;
pub static BASIC_ROCKETRY_ENGINES: LazyLock<[Engine; NUM_ENGINES]> =
    LazyLock::new(|| init_engines());

#[rustfmt::skip]
fn init_engines() -> [Engine; NUM_ENGINES] {
    let engines = [
        Engine::new(
            "S.2.253/S3.42/S5.2", // Engine 0
            false,
            81.4,
            94.8,
            100.0,
            219.0,
            255.0,
            0.3009,
            0.8,
            95.0,
            true,
            false,
            true,
            false,
            true,
            1,
            1.25,
            0.0,
            vec![
                Fuel::new("Kerosene", 4919.7352, 1662.68936339343, 1381.0, 2673.0, 2, 35.9, true),
                Fuel::new("AK20", 4919.7352, 3166.49856563579, 1622.0, 6479.0, 2, 35.9, true),
                Fuel::new("Nitrogen", 4919.7352, 90.5472709707792, 1130.0, 1152.0, 2, 35.9, true),
                // FuelType::Kerosene(0.218, 0.169),
                // FuelType::AK20(0.441, 0.677),
                // FuelType::Nitrogen(25.0, 31.3),
            ],
            "Basic Rocketry",
        ),
        Engine::new(
            "LR43/LR89", // Engine 1
            false,
            535.9,
            617.4,
            100.0,
            230.0,
            265.0,
            0.5695,
            0.3,
            65.0,
            true,
            false,
            false,
            false,
            true,
            1,
            1.25,
            0.0,
            vec![
                Fuel::new("Ethanol90", 21178.6522, 10472.843438106, 468.0, 8952.0, 1, 26.7, false),
                Fuel::new("Liquid Oxygen", 21178.6522, 10705.808761894, 472.0, 12687.0, 1, 26.7, false),
                // FuelType::Kerosene(0.218, 0.169),
                // FuelType::AK20(0.441, 0.677),
                // FuelType::Nitrogen(25.0, 31.3),
            ],
            "Basic Rocketry",
        ),
        Engine::new(
            "Stentor Booster", // Engine 2
            false,
            110.0,
            121.0,
            100.0,
            200.0,
            220.0,
            0.339,
            0.4,
            60.0,
            false,
            false,
            false,
            false,
            true,
            1,
            1.0,
            0.0,
            vec![
                Fuel::new("RP-1", 4444.0451, 790.151217508549, 77.9, 716.0, 1, 43.8, false),
                Fuel::new("HTP", 4444.0451, 3653.89388249145, 119.0, 5348.0, 1, 43.8, false),
                // FuelType::Kerosene(0.218, 0.169),
                // FuelType::AK20(0.441, 0.677),
                // FuelType::Nitrogen(25.0, 31.3),
            ],
            "Basic Rocketry",
        ),
        Engine::new(
            "XLR25", // Engine 3
            false,
            66.7,
            73.7,
            17.0,
            208.0,
            230.0,
            0.157,
            0.8,
            175.0,
            false,
            false,
            false,
            true,
            false,
            6,
            1.0,
            0.0,
            vec![
                Fuel::new("Ethanol75", 8253.227, 4350.51302469865, 186.0, 3849.0, 4, 3.0, false),
                Fuel::new("Liquid Oxygen", 8253.227, 3820.99885829786, 179.0, 4539.0, 4, 3.0, false),
                Fuel::new("HTP", 8253.227, 81.7151170034899, 125.0, 241.0, 4, 3.0, false),
                // FuelType::Kerosene(0.218, 0.169),
                // FuelType::AK20(0.441, 0.677),
                // FuelType::Nitrogen(25.0, 31.3),
            ],
            "Basic Rocketry",
        ),
    ];
    engines
}

#[rustfmt::skip]
pub(super) fn basic_rocketry_mod_1(engines: &mut [Engine; NUM_EARLY_ENGINES]) {
    engines.iter_mut().find(|e| e.name == "RD-100").unwrap().configurations.push(
        EngineConfiguration::new(
            "5D60",
            413.2,
            465.8,
            100.0,
            0.867,
            220.0,
            248.0,
            130.0,
            true,
            false,
            1,
            vec![
                Fuel::new("Ethanol90", 36653.4544, 17945.6763175396, 808.0, 15346.0, 3, 4.3, false),
                Fuel::new("Liquid Oxygen", 36653.4544, 18344.8726120911, 814.0, 21745.0, 3, 4.3, false),
                Fuel::new("HTP", 36653.4544, 362.905470369311, 553.0, 1072.0, 3, 4.3, false),
            ],
            "Basic Rocketry",
        )
    );
    engines.iter_mut().find(|e| e.name == "RD-100").unwrap().configurations.push(
        EngineConfiguration::new(
            "RD-102",
            389.8,
            428.0,
            100.0,
            0.885,
            214.0,
            235.0,
            83.0,
            true,
            false,
            1,
            vec![
                Fuel::new("Ethanol90", 30214.7441, 14793.2582757026, 666.0, 12650.0, 2, 36.7, false),
                Fuel::new("Liquid Oxygen", 30214.7441, 15122.3299575669, 671.0, 17926.0, 2, 36.7, false),
                Fuel::new("HTP", 30214.7441, 299.1558667305, 456.0, 884.0, 2, 36.7, false),
            ],
            "Basic Rocketry",
        )
    );
    engines.iter_mut().find(|e| e.name == "RD-100").unwrap().configurations.push(
        EngineConfiguration::new(
            "RD-103",
            435.0,
            490.3,
            100.0,
            0.870,
            220.0,
            248.0,
            130.0,
            true,
            false,
            1,
            vec![
                Fuel::new("Ethanol90", 31891.9745, 15614.4369165987, 703.0, 13352.0, 2, 32.3, false),
                Fuel::new("Liquid Oxygen", 31891.9745, 15961.7754759442, 708.0, 18921.0, 2, 32.3, false),
                Fuel::new("HTP", 31891.9745, 315.762107457151, 481.0, 933.0, 2, 32.3, false),
            ],
            "Basic Rocketry",
        )
    );
    engines.iter_mut().find(|e| e.name == "NAA-75-110 A-Series").unwrap().configurations.push(
        EngineConfiguration::new(
            "A-6",
            333.6,
            383.0,
            100.0,
            0.740,
            216.0,
            248.0,
            121.0,
            true,
            false,
            1,
            vec![
                Fuel::new("Ethanol75", 22598.5235, 11111.5882891797, 2573.0, 11926.0, 2, 19.1, false),
                Fuel::new("Liquid Oxygen", 22598.5235, 11098.2628148451, 2572.0, 15235.0, 2, 19.1, false),
                Fuel::new("HTP", 22598.5235, 388.672395975197, 1929.0, 2485.0, 2, 19.1, false),
            ],
            "Basic Rocketry",
        )
    );
}
