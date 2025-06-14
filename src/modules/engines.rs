use std::collections::HashMap;

use std::sync::LazyLock;

use crate::{modules::fuel_type::Fuel, TECH_TREE};

use super::{
    fuel_type::{FuelMix, FuelType},
    size::Size,
};

pub static ENGINES: LazyLock<[Engine; NUM_ENGINES]> = LazyLock::new(|| Engine::init_rp1_engines());

pub type Engine = EngineV1;

#[derive(Debug, PartialEq, Clone)]
pub struct EngineV1 {
    pub name: &'static str,
    pub parent_name: &'static str,
    pub is_solid: bool,
    pub thrust_asl: f64,
    pub thrust_vac: f64,
    pub min_thrust: f64,
    pub isp_asl: f64,
    pub isp_vac: f64,
    pub mass: f64,
    pub residuals: f64,
    pub rated_burn_time: f64,
    pub has_gimbal: bool,
    pub is_radial: bool,
    pub hp_fuel: bool,
    pub throttleable: bool,
    pub ullage: bool,
    pub ignitions: u8,
    pub diameter: f64,
    pub tank_volume_liters: f64,
    pub fuel_mix: Vec<Fuel>,
    pub configurations: Vec<EngineConfiguration>,
    pub tech_tree_node: &'static str,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EngineConfiguration {
    pub name: &'static str,
    pub thrust_kn: f64, // thrust of the engine config, presumuably ASL
    pub min_thrust_percentage: f64,
    pub mass: f64, // mass in tons
    pub isp_asl: f64,
    pub isp_vac: f64,
    pub rated_burn_time: f64,
    pub ullage: bool,
    pub hp_fuel: bool,
    pub num_ignitions: u8,
    pub is_initialized: bool,
    pub fuel_mix: Vec<Fuel>,
    pub tech_tree_node: &'static str,
}

impl EngineConfiguration {
    pub const fn new(
        name: &'static str,
        thrust_kn: f64,
        min_thrust_percentage: f64,
        mass: f64,
        isp_asl: f64,
        isp_vac: f64,
        rated_burn_time: f64,
        ullage: bool,
        hp_fuel: bool,
        num_ignitions: u8,
        fuel_mix: Vec<Fuel>,
        tech_tree_node: &'static str,
    ) -> Self {
        Self {
            name,
            thrust_kn,
            min_thrust_percentage,
            mass,
            isp_asl,
            isp_vac,
            rated_burn_time,
            ullage,
            hp_fuel,
            num_ignitions,
            is_initialized: true,
            fuel_mix,
            tech_tree_node,
        }
    }
    /// Calculates the thrust (vac) for a configuration.
    pub const fn thrust_vac(&self) -> f64 {
        self.thrust_kn * (self.isp_vac / self.isp_asl)
    }
    /// Zeroed initial engine config
    pub const fn zeroed() -> Self {
        Self {
            name: "",
            thrust_kn: 0.0,
            min_thrust_percentage: 0.0,
            mass: 0.0,
            isp_asl: 0.0,
            isp_vac: 0.0,
            rated_burn_time: 0.0,
            ullage: false,
            hp_fuel: false,
            num_ignitions: 0,
            is_initialized: false,
            fuel_mix: vec![],
            tech_tree_node: "start",
        }
    }
    /// Turns an engine configuration into an engine
    pub fn to_engine(&self, parent: &Engine) -> Engine {
        let mut result = parent.clone();
        result.parent_name = parent.parent_name;
        result.name = self.name;
        result.thrust_asl = self.thrust_kn;
        result.thrust_vac = self.thrust_vac();
        result.min_thrust = self.min_thrust_percentage;
        result.mass = self.mass;
        result.isp_asl = self.isp_asl;
        result.isp_vac = self.isp_vac;
        result.ullage = self.ullage;
        result.rated_burn_time = self.rated_burn_time;
        result.hp_fuel = self.hp_fuel;
        result.ignitions = self.num_ignitions;
        result.fuel_mix = self.fuel_mix.clone();
        result.tech_tree_node = self.tech_tree_node;
        result
    }
}

//const NUM_ENGINES: usize = 10;
const NUM_ENGINES: usize = 11;

impl Engine {
    pub fn max_volume(&self) -> f64 {
        if self.fuel_mix.is_empty() {
            return 0.0;
        }
        let mut total_volume_liters = 0.0;

        for fuel in self.fuel_mix.iter() {
            let utilization_factor = match fuel.name.to_lowercase().as_str() {
                //"nitrogen" => 1.0 / 200.0, // Apply utilization scaling
                //"helium" => 1.0 / 200.0, // Similar compression behavior
                //"lqdhydrogen" => 0.2, // LH2 tank mass scaling
                _ => 1.0 // Default utilization factor
            };

            let volume_flow_rate = fuel.flow_rate_lps * utilization_factor; // Apply tank utilization
            let volume = volume_flow_rate * self.rated_burn_time;
            total_volume_liters += volume;
        }

        total_volume_liters
    }


    pub fn fuel_density(&self) -> f64 {
        let mut weighted_density_sum = 0.0;
        let mut total_volume_ratio = 0.0;

        for fuel in self.fuel_mix.iter() {
            let multiplier = match fuel.name.to_lowercase().as_str() {
                //"nitrogen" => 2.85,
                _ => 1.0
            };
            weighted_density_sum += fuel.density * fuel.volume_ratio * multiplier;
            total_volume_ratio += fuel.volume_ratio;
        }

        if total_volume_ratio == 0.0 {
            return 0.0; // Prevent division by zero
        }

        weighted_density_sum / total_volume_ratio
    }

    pub fn fuel_mass(&self, volume: f64) -> f64 {
        self.fuel_density() * volume
    }
    /// Creates a new engine.
    ///
    /// Parameters:
    /// * `name` - Name of the engine
    /// * `thrust_asl` - Thrust at sea level in kN
    /// * `thrust_vac` - Thrust in a vacuum in kN
    /// * `min_thrust` - Minimum thrust percentage
    /// * `isp_asl` - Specific impulse (Isp) at sea level
    /// * `isp_vac` - Specific impulse (Isp) in a vacuum
    /// * `mass` - The mass of the engine in tons
    /// * `has_gimbal` - Whether or not the engine can gimbal
    /// * `is_radial` - Whether or not the engine is a radial engine
    /// * `hp_fuel` - Whether or not the engine requires high pressure fuel
    /// * `throttleable` - Whether or not the engine is throttleable
    /// * `Size` - The size/diameter of the engine
    /// * `fuel_types` - A slice of which fuel types this engine can use
    const fn new(
        name: &'static str,
        is_solid: bool,
        thrust_asl: f64,
        thrust_vac: f64,
        min_thrust: f64,
        isp_asl: f64,
        isp_vac: f64,
        mass: f64,
        residuals: f64,
        rated_burn_time: f64,
        has_gimbal: bool,
        is_radial: bool,
        hp_fuel: bool,
        throttleable: bool,
        ullage: bool,
        ignitions: u8,
        diameter: f64,
        tank_volume_liters: f64,
        fuel_mix: Vec<Fuel>,
        tech_tree_node: &'static str,
    ) -> Self {
        Engine {
            name,
            parent_name: "",
            is_solid,
            thrust_asl,
            thrust_vac,
            min_thrust,
            isp_asl,
            isp_vac,
            mass,
            residuals,
            rated_burn_time,
            has_gimbal,
            is_radial,
            hp_fuel,
            throttleable,
            ullage,
            ignitions,
            diameter,
            tank_volume_liters,
            fuel_mix,
            configurations: Vec::new(),
            tech_tree_node,
        }
    }

    pub fn init_rp1_engines() -> [Engine; NUM_ENGINES] {
        let mut engines = [
            Engine::new(
                "Aerobee", // Engine 0
                false,
                6.672,
                7.733,
                100.0,
                195.0,
                226.0,
                0.008,
                1.2,
                47.0,
                false,
                false,
                true,
                false,
                true,
                1,
                0.3,
                0.0,
                vec![
                    Fuel::new("Aniline Furfuryl 22%", 703.7168, 215.229453240051, 38.1, 262.0, 3, 48.4, true),
                    //FuelType::AnilineFurfuryl_22p(0.893, 0.930),
                    //FuelType::IRFNA_III(1.64, 2.56),
                    Fuel::new("IRFNA-III", 703.7168, 394.313033691616, 45.6, 662.0, 3, 48.4, true),
                    //FuelType::Nitrogen(78.1, 0.0978),
                    Fuel::new("Nitrogen", 703.7168, 94.1743130683326, 33.0, 56.5, 3, 48.4, true),
                ],
                "start",
            ),
            Engine::new(
                "U-1250", // Engine 1
                false,
                12.7,
                14.4,
                100.0,
                204.8,
                232.1,
                0.0156,
                1.2,
                56.0,
                false,
                false,
                true,
                false,
                true,
                1,
                0.3,
                1.0,
                vec![
                    Fuel::new("Kerosene", 854.5133, 254.17617555427, 45.9, 243.0, 2, 19.7, true),
                    Fuel::new("AK20", 854.5133, 484.064259404867, 55.6, 798.0, 2, 19.7, true),
                    Fuel::new("Nitrogen", 854.5133, 116.272865040863, 40.1, 69.2, 2, 19.7, true),
                    // FuelType::Kerosene(1.71, 1.33),
                    // FuelType::AK20(3.26, 5.0),
                    // FuelType::Nitrogen(157.0, 0.196),
                ],
                "start",
            ),
            Engine::new(
                "Veronique", // Engine 2
                false,
                39.2,
                49.3,
                100.0,
                198.0,
                249.0,
                0.1511,
                3.92,
                45.0,
                false,
                false,
                true,
                false,
                true,
                1,
                0.3,
                1.0,
                vec![
                    Fuel::new("Kerosene", 1252.9931, 430.967022848892, 69.8, 405.0, 1, 15.8, true),
                    Fuel::new("IRFNA-III", 1252.9931, 804.957952765303, 85.5, 1344.0, 1, 15.8, true),
                    Fuel::new("Water", 1252.9931, 17.068124385805, 52.4, 69.5, 1, 15.8, true),
                    // FuelType::Kerosene(5.46, 4.24),
                    // FuelType::IRFNA_III(10.2, 15.9),
                    // FuelType::Water(0.216, 0.216),
                ],
                "start",
            ),
            Engine::new(
                "A-4", // Engine 3
                false,
                238.8,
                284.7,
                100.0,
                203.0,
                242.0,
                0.9299,
                0.47,
                70.0,
                false,
                false,
                false,
                false,
                true,
                1,
                1.25,
                0.0,
                vec![
                    Fuel::new("Ethanol 75", 13002.0069, 6723.71115826852, 292.0, 5952.0, 1, 45.2, false),
                    Fuel::new("Liquid Oxygen", 13002.0069, 6149.56300299624, 284.0, 7300.0, 1, 45.2, false),
                    Fuel::new("HTP", 13002.0069, 128.732738735247, 196.0, 380.0, 1, 45.2, false),
                    // FuelType::Ethanol_75(63.6, 53.6),
                    // FuelType::Liquid_Oxygen(58.2, 66.4),
                    // FuelType::HTP(1.22, 1.74),
                ],
                "start",
            ),
            Engine::new(
                "RD-100", // Engine 4
                false,
                263.0,
                307.0,
                100.0,
                203.0,
                237.0,
                0.888,
                0.3,
                70.0,
                true,
                false,
                false,
                false,
                true,
                1,
                1.25,
                0.0,
                vec![
                    Fuel::new("Ethanol 75", 17090.265, 8976.61930184596, 1082.0, 8638.0, 2, 5.2, false),
                    Fuel::new("Liquid Oxygen", 17090.265, 7944.43515734428, 1038.0, 10103.0, 2, 5.2, false),
                    Fuel::new("HTP", 17090.265, 169.210540809752, 712.0, 954.0, 2, 5.2, false), 
                    // FuelType::Ethanol_75(71.3, 60.1),
                    // FuelType::Liquid_Oxygen(63.1, 72.0),
                    // FuelType::HTP(1.34, 1.92),
                ],
                "Post-War Rocketry Testing",
            ),
            Engine::new(
                "XLR10", // Engine 5
                false,
                92.5,
                110.5,
                100.0,
                179.6,
                214.5,
                0.192,
                0.3,
                103.0,
                true,
                false,
                false,
                false,
                true,
                1,
                1.25,
                0.0,
                vec![
                    Fuel::new("Ethanol 90", 6641.8164, 3625.7163182091, 152.0, 3089.0, 1, 58.4, false),
                    Fuel::new("Liquid Oxygen", 6641.8164, 2901.22589601941, 141.0, 3452.0, 1, 58.4, false),
                    Fuel::new("HTP", 6641.8164, 114.874185771482, 101.0, 265.0, 1, 58.4, false),
                    // FuelType::Ethanol_90(30.5, 24.7),
                    // FuelType::Liquid_Oxygen(24.4, 27.8),
                    // FuelType::HTP(0.966, 1.38),
                ],
                "Post-War Rocketry Testing",
            ),
            Engine::new(
                "XLR11", // Engine 6
                false,
                24.5,
                26.7,
                25.0,
                207.7,
                226.6,
                0.1499,
                1.6,
                300.0,
                false,
                false,
                true,
                true,
                false,
                u8::MAX,
                1.25,
                0.0,
                vec![
                    Fuel::new("Ethanol 75", 1628.6019, 747.961904080594, 98.6, 728.0, 2, 3.6, true),
                    Fuel::new("Liquid Oxygen", 1628.6019, 793.909719668295, 101.0, 1006.0, 2, 3.6, true),
                    Fuel::new("Nitrogen", 1628.6019, 86.7302762511113, 70.8, 92.5, 2, 3.6, true),
                    // FuelType::Ethanol_75(5.85, 4.92),
                    // FuelType::Liquid_Oxygen(6.21, 7.08),
                    // FuelType::Nitrogen(136.0, 0.170),
                ],
                "Post-War Rocketry Testing",
            ),
            Engine::new(
                "XLR41", // Engine 7
                false,
                282.8,
                333.0,
                100.0,
                203.0,
                239.0,
                0.791,
                0.3,
                70.0,
                true,
                false,
                false,
                false,
                true,
                1,
                1.25,
                0.0,
                vec![
                    Fuel::new("Ethanol 75", 16455.6654, 8352.14606173453, 367.0, 7398.0, 1, 52.7, false),
                    Fuel::new("Liquid Oxygen", 16455.6654, 7940.59196167311, 361.0, 9421.0, 1, 52.7, false),
                    Fuel::new("HTP", 16455.6654, 162.927376592366, 248.0, 481.0, 1, 52.7, false),
                    // FuelType::Ethanol_75(73.7, 62.1),
                    // FuelType::Liquid_Oxygen(70.1, 80.0),
                    // FuelType::HTP(1.44, 2.06),
                ],
                "Post-War Rocketry Testing",
            ),
            Engine::new(
                "ORM-65", // Engine 8
                false,
                1.7,
                1.8,
                29.0,
                210.0,
                215.0,
                0.014,
                0.8,
                80.0,
                false,
                false,
                true,
                true,
                true,
                1,
                0.3,
                0.0,
                vec![
                    Fuel::new("Kerosene", 116.2389, 32.2899408302602, 6.15, 31.2, 2, 22.2, true),
                    Fuel::new("AK20", 116.2389, 65.4103320231149, 7.54, 108.0, 2, 22.2, true),
                    Fuel::new("Nitrogen", 116.2389, 18.5386271466249, 5.57, 10.2, 2, 22.2, true),
                    // FuelType::Kerosene(0.218, 0.169),
                    // FuelType::AK20(0.441, 0.677),
                    // FuelType::Nitrogen(25.0, 31.3),
                ],
                "Post-War Rocketry Testing",
            ),
            Engine::new(
                "NAA-75-110 A-Series", // Engine 9
                false,
                333.6,
                383.0,
                100.0,
                216.0,
                248.0,
                0.74,
                0.3,
                65.0,
                true,
                false,
                false,
                false,
                true,
                1,
                1.85,
                0.0,
                vec![
                    Fuel::new("Ethanol75", 20055.1365, 9861.01679923026, 443.0, 8743.0, 2, 3.5, false),
                    Fuel::new("Liquid Oxygen", 20055.1365, 9849.19106173433, 443.0, 11681.0, 2, 3.5, false),
                    Fuel::new("HTP", 20055.1365, 344.928639035405, 305.0, 798.0, 2, 3.5, false),
                    // FuelType::Kerosene(0.218, 0.169),
                    // FuelType::AK20(0.441, 0.677),
                    // FuelType::Nitrogen(25.0, 31.3),
                ],
                "Early Rocketry",
            ),
            Engine::new(
                "RD-200", // Engine 10
                false,
                88.4,
                98.5,
                100.0,
                210.0,
                234.0,
                0.169,
                0.3,
                85.0,
                true,
                false,
                false,
                false,
                true,
                1,
                1.25,
                0.0,
                vec![
                    Fuel::new("Kerosene", 5751.4672, 1960.62387095336, 114.0, 1638.0, 2, 47.8, true),
                    Fuel::new("AK20", 5751.4672, 3733.89811218518, 140.0, 5868.0, 2, 47.8, true),
                    Fuel::new("HTP", 5751.4672, 56.9452168614608, 86.8, 168.0, 2, 47.8, true),
                    // FuelType::Kerosene(0.218, 0.169),
                    // FuelType::AK20(0.441, 0.677),
                    // FuelType::Nitrogen(25.0, 31.3),
                ],
                "Early Rocketry",
            ),
        ];
        // Aerobee engine configurations
        engines[0].configurations.push(EngineConfiguration::new(
            "XASR-1",
            11.7,
            100.0,
            0.010,
            200.0,
            235.44,
            40.0,
            true,
            true,
            1,
            vec![
                Fuel::new("Aniline Furfuryl 37%", 2918.9168, 912.317280595089, 159.0, 1120.0, 9, 12.3, true),
                //FuelType::AnilineFurfuryl_37p(1.58, 1.67),
                Fuel::new("IRFNA-III", 2918.9168, 1580.35290769801, 187.0, 2660.0, 9, 12.3, true),
                //FuelType::IRFNA_III(2.74, 4.29),
                //FuelType::Nitrogen(148.0, 0.185),
                Fuel::new("Nitrogen", 2918.9168, 426.246611706902, 138.0, 245.0, 9, 12.3, true),
            ],
            "Post-War Rocketry Testing",
        ));
        engines[0].configurations.push(EngineConfiguration::new(
            "XASR-2",
            11.7,
            100.0,
            0.010,
            200.0,
            235.44,
            40.0,
            true,
            true,
            1,
            vec![
                Fuel::new("Aniline Furfuryl 37%", 1156.1061, 361.344856842578, 62.9, 445.0, 3, 38.7, true),
                Fuel::new("IRFNA-III", 1156.1061, 625.936181785793, 74.0, 1050.0, 3, 38.7, true),
                Fuel::new("Helium", 1156.1061, 168.82506137163, 54.8, 60.8, 3, 38.7, true)
                // FuelType::AnilineFurfuryl_37p(1.58, 1.67),
                // FuelType::IRFNA_III(2.74, 4.29),
                // FuelType::Helium(148.0, 0.0264),
            ],
            "Early Rocketry",
        ));
        engines[0].configurations.push(EngineConfiguration::new(
            "AJ10-27",
            18.2,
            100.0,
            0.012,
            198.0,
            231.0,
            52.0,
            true,
            true,
            1,
            vec![
                Fuel::new("Aniline Furfuryl 37%", 1156.1061, 362.505742007479, 62.9, 447.0, 2, 19.5, true),
                Fuel::new("IRFNA-III", 1156.1061, 627.947114040259, 74.1, 1060.0, 2, 19.5, true),
                Fuel::new("Helium", 1156.1061, 165.653243952262, 54.6, 60.6, 2, 19.5, true)
                // FuelType::AnilineFurfuryl_37p(2.49, 2.64),
                // FuelType::IRFNA_III(4.32, 6.75),
                // FuelType::Helium(228.0, 0.0407),
            ],
            "Early Rocketry",
        ));

        // U-1250 engine configurations
        engines[1].configurations.push(EngineConfiguration::new(
            "U-1700",
            17.0,
            100.0,
            0.015,
            206.5,
            236.4,
            60.0,
            true,
            true,
            1,
            vec![
                Fuel::new("Kerosene", 854.5133, 248.652358189714, 45.7, 239.0, 1, 46.2, true),
                Fuel::new("AK20", 854.5133, 484.835452969084, 55.6, 799.0, 1, 46.2, true),
                Fuel::new("Nitrogen", 854.5133, 121.025488841202, 40.3, 70.6, 1, 46.2, true),
                // FuelType::Kerosene(2.22, 1.73),
                // FuelType::AK20(4.33, 6.65),
                // FuelType::Nitrogen(216.0, 0.271),
            ],
            "Post-War Rocketry Testing",
        ));
        engines[1].configurations.push(EngineConfiguration::new(
            "U-2000",
            19.6,
            100.0,
            0.013,
            205.6,
            241.3,
            60.0,
            true,
            true,
            1,
            vec![
                Fuel::new("Kerosene", 854.5133, 249.175342950689, 45.7, 239.0, 1, 31.1, true),
                Fuel::new("AK20", 854.5133, 479.620648520527, 55.4, 791.0, 1, 31.1, true),
                Fuel::new("Nitrogen", 854.5133, 125.717308528785, 40.5, 72.0, 1, 31.1, true),
                // FuelType::Kerosene(2.61, 2.02),
                // FuelType::AK20(5.02, 7.69),
                // FuelType::Nitrogen(263.0, 0.329),
            ],
            "Early Rocketry",
        ));

        // Veronique engine configurations
        engines[2].configurations.push(EngineConfiguration::new(
            "VeroniqueAGI",
            39.3,
            100.0,
            0.150,
            208.0,
            261.0,
            49.0,
            true,
            true,
            1,
            vec![
                Fuel::new("Turpentine", 1252.9931, 403.690170662349, 68.6, 420.0, 1, 20.1, true),
                Fuel::new("IWFNA", 1252.9931, 829.328033319713, 86.5, 1340.0, 1, 20.1, true),
                Fuel::new("Water", 1252.9931, 19.9748960179377, 52.5, 72.5, 1, 20.1, true),
                // FuelType::Turpentine(4.84, 4.21),
                // FuelType::IWFNA(9.95, 15.0),
                // FuelType::Water(0.240, 0.240),
            ],
            "Basic Rocketry",
        ));
        engines[2].configurations.push(EngineConfiguration::new(
            "Veronique61",
            58.8,
            100.0,
            0.150,
            208.0,
            261.0,
            56.0,
            true,
            true,
            1,
            vec![
                Fuel::new("Turpentine", 2909.4921, 937.382147108198, 159.0, 975.0, 2, 4.3, true),
                Fuel::new("IWFNA", 2909.4921, 1925.72757284317, 201.0, 3113.0, 2, 4.3, true),
                Fuel::new("Water", 2909.4921, 46.38238004863, 122.0, 168.0, 2, 4.3, true),
                // FuelType::Turpentine(7.25, 6.31),
                // FuelType::IWFNA(14.9, 22.5),
                // FuelType::Water(0.359, 0.359),
            ],
            "1956-1957 Orbital Rocketry",
        ));

        // A-4 engine configurations
        engines[3].configurations.push(EngineConfiguration::new(
            "A-9",
            249.1,
            100.0,
            0.972,
            220.0,
            255.0,
            115.0,
            true,
            false,
            1,
            vec![
                Fuel::new("Hydyne", 13002.0069, 6432.77522483618, 288.0, 5820.0, 1, 51.0, true),
                Fuel::new("Liquid Oxygen", 13002.0069, 6440.49893642857, 288.0, 7636.0, 1, 51.0, true),
                Fuel::new("HTP", 13002.0069, 128.732738735247, 196.0, 380.0, 1, 51.0, true),
                // FuelType::Hydyne(57.7, 49.6),
                // FuelType::Liquid_Oxygen(57.7, 65.9),
                // FuelType::HTP(1.15, 1.65),
            ],
            "Post-War Rocketry Testing",
        ));

        // RD-100 engine configurations
        engines[4].configurations.push(EngineConfiguration::new(
            "RD-101",
            358.0,
            100.0,
            0.888,
            210.0,
            237.0,
            85.0,
            true,
            false,
            1,
            vec![
                Fuel::new("Ethanol 90", 17268.2886, 8454.6224318071, 381.0, 7230.0, 1, 35.7, false),
                Fuel::new("Liquid Oxygen", 17268.2886, 8642.69302256611, 383.0, 10245.0, 1, 35.7, false),
                Fuel::new("HTP", 17268.2886, 170.973145626788, 261.0, 505.0, 1, 35.7, false),
                // FuelType::Ethanol_90(87.9, 71.2),
                // FuelType::Liquid_Oxygen(89.9, 103.0),
                // FuelType::HTP(1.78, 2.55),
            ],
            "Early Rocketry",
        ));

        // ORM-65 engine configurations
        engines[8].configurations.push(EngineConfiguration::new(
            "RDA-1-150",
            1.4,
            34.0,
            0.012,
            210.0,
            215.0,
            200.0,
            true,
            true,
            2,
            vec![
                Fuel::new("Kerosene", 236.5934, 65.7231519468102, 64.1, 115.0, 5, 53.2, true),
                Fuel::new("AK20", 236.5934, 133.136607869462, 74.9, 279.0, 5, 53.2, true),
                Fuel::new("Nitrogen", 236.5934, 37.7336401837275, 59.7, 69.1, 5, 53.2, true),
                // FuelType::Kerosene(0.178, 0.139),
                // FuelType::AK20(0.361, 0.554),
                // FuelType::Nitrogen(20.5, 0.0256),
            ],
            "Post-War Rocketry Testing",
        ));
        engines[8].configurations.push(EngineConfiguration::new(
            "RDA-1-300",
            2.9,
            34.0,
            0.012,
            210.0,
            215.0,
            200.0,
            true,
            true,
            2,
            vec![
                Fuel::new("Kerosene", 116.2389, 32.2899408302602, 6.15, 31.2, 1, 26.1, true),
                Fuel::new("AK20", 116.2389, 65.4103320231149, 7.54, 108.0, 1, 26.1, true),
                Fuel::new("Nitrogen", 116.2389, 18.5386271466249, 5.57, 10.2, 1, 26.1, true),
                // FuelType::Kerosene(0.178, 0.279),
                // FuelType::AK20(0.361, 1.12),
                // FuelType::Nitrogen(20.5, 0.0516),
            ],
            "Early Rocketry",
        ));
        // XLR11 engine configurations
        //engines[7].configurations[0] = EngineConfiguration::new("XLR11")
        engines[6].configurations.push(EngineConfiguration::new(
            "XLR-11-RM-5",
            24.5,
            25.0,
            0.212,
            209.0,
            228.0,
            300.0,
            false,
            false,
            u8::MAX,
            vec![
                Fuel::new("Ethanol75", 3918.2987, 2056.91286053099, 88.4, 1820.0, 5, 16.3, false),
                Fuel::new("Liquid Oxygen", 3918.2987, 1822.59080369389, 85.0, 2165.0, 5, 16.3, false),
                Fuel::new("HTP", 3918.2987, 38.7950357751122, 59.1, 115.0, 5, 16.3, false),
                // FuelType::Kerosene(0.178, 0.279),
                // FuelType::AK20(0.361, 1.12),
                // FuelType::Nitrogen(20.5, 0.0516),
            ],
            "Early Rocketry",
        ));
        engines[6].configurations.push(EngineConfiguration::new(
            "XLR-35-RM-1",
            33.8,
            100.0,
            0.185,
            211.0,
            234.4,
            220.0,
            false,
            false,
            1,
            vec![
                Fuel::new("Ethanol75", 3918.2987, 2056.91286053099, 88.4, 1820.0, 3, 52.0, false),
                Fuel::new("Liquid Oxygen", 3918.2987, 1822.59080369389, 85.0, 2165.0, 3, 52.0, false),
                Fuel::new("HTP", 3918.2987, 38.7950357751122, 59.1, 115.0, 3, 52.0, false),
                // FuelType::Kerosene(0.178, 0.279),
                // FuelType::AK20(0.361, 1.12),
                // FuelType::Nitrogen(20.5, 0.0516),
            ],
            "Early Rocketry",
        ));
        engines
    }

    pub fn init_all_engines(engines: [Engine; NUM_ENGINES]) -> HashMap<&'static str, Vec<Engine>> {
        let mut result: HashMap<&'static str, Vec<Engine>> =
            HashMap::with_capacity(TECH_TREE.len());
        for engine in engines.iter() {
            if let Some(vec) = result.get_mut(&engine.tech_tree_node) {
                vec.push(engine.clone());
            } else {
                result.insert(&engine.tech_tree_node, vec![engine.clone()]);
            }
            let configurations = &engine.configurations;
            for config in configurations.iter() {
                if !config.is_initialized {
                    break;
                }
                let engine = config.to_engine(&engine);
                if let Some(vec) = result.get_mut(&engine.tech_tree_node) {
                    vec.push(engine);
                } else {
                    let vec = vec![engine.clone()];
                    result.insert(&engine.tech_tree_node, vec);
                }
            }
        }
        result
    }

    /// Gets the name of this engine, including the parent engine's name.
    pub fn get_name(&self) -> String {
        match self.parent_name.is_empty() {
            true => self.name.to_string(),
            false => format!("{}: {}", self.parent_name, self.name),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn engine_configs_thrust_vac() {
        let engine = EngineConfiguration {
            name: "Aerobee XASR-1",
            thrust_kn: 13.8,
            min_thrust_percentage: 100.0,
            isp_asl: 200.0,
            isp_vac: 235.44,
            rated_burn_time: 40.0,
            mass: 1.0,
            ullage: true,
            hp_fuel: true,
            num_ignitions: 1,
            is_initialized: true,
            fuel_mix: vec![],
            tech_tree_node: "start",
        };
        assert_eq!(engine.thrust_vac(), 16.24536);
    }

    #[test]
    fn max_volume_tests() {
        let engine = &ENGINES[0];
        let max_volume = engine.max_volume();
        assert!((max_volume - 145.0).abs() < 1.0);
    }
}
