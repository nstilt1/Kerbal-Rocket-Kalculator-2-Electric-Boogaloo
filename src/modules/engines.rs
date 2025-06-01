use std::collections::HashMap;

use serde::Serialize;

use crate::TECH_TREE;

use super::{
    fuel_type::{FuelMix, FuelType},
    size::Size,
};

const MAX_ENGINE_CONFIGS: usize = 4;
pub const ENGINES: [Engine; NUM_EGINES] = Engine::init_rp1_engines();

#[derive(Debug, PartialEq, Clone, Copy, Serialize)]
pub struct Engine {
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
    pub size: Size,
    pub tank_volume_liters: f64,
    pub fuel_mix: FuelMix,
    pub configurations: [EngineConfiguration; MAX_ENGINE_CONFIGS],
    pub tech_tree_node: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
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
    pub fuel_mix: FuelMix,
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
        fuel_mix: FuelMix,
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
            fuel_mix: FuelMix::new(&[]),
            tech_tree_node: "start",
        }
    }
    /// Turns an engine configuration into an engine
    pub fn to_engine(&self, parent: &Engine) -> Engine {
        let mut result = *parent;
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
        result.fuel_mix = self.fuel_mix;
        result.tech_tree_node = self.tech_tree_node;
        result
    }
}

const NUM_EGINES: usize = 10;

impl Engine {
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
        size: Size,
        tank_volume_liters: f64,
        fuel_mix: FuelMix,
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
            size,
            tank_volume_liters,
            fuel_mix,
            configurations: [EngineConfiguration::zeroed(); MAX_ENGINE_CONFIGS],
            tech_tree_node,
        }
    }

    pub const fn init_rp1_engines() -> [Engine; NUM_EGINES] {
        let mut engines = [
            Engine::new(
                "Aerobee", // Engine 0
                false,
                6.7,
                7.7,
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
                Size::Xs,
                0.0,
                FuelMix::new(&[
                    FuelType::AnilineFurfuryl_22p(0.893, 0.930),
                    FuelType::IRFNA_III(1.64, 2.56),
                    FuelType::Nitrogen(78.1, 0.0978),
                ]),
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
                Size::Xs,
                1.0,
                FuelMix::new(&[
                    FuelType::Kerosene(1.71, 1.33),
                    FuelType::AK20(3.26, 5.0),
                    FuelType::Nitrogen(157.0, 0.196),
                ]),
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
                Size::Xs,
                1.0,
                FuelMix::new(&[
                    FuelType::Kerosene(5.46, 4.24),
                    FuelType::IRFNA_III(10.2, 15.9),
                    FuelType::Water(0.216, 0.216),
                ]),
                "start",
            ),
            Engine::new(
                "Tiny Tim Booster", // Engine 3
                true,
                133.4,
                146.6,
                100.0,
                202.0,
                222.0,
                0.0673,
                3.9,
                5.0,
                false,
                false,
                false,
                false,
                false,
                1,
                Size::Xs,
                41.3903,
                FuelMix::new(&[FuelType::NGNC(42.1)]),
                "start",
            ),
            Engine::new(
                "A-4", // Engine 4
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
                Size::Sm,
                0.0,
                FuelMix::new(&[
                    FuelType::Ethanol_75(63.6, 53.6),
                    FuelType::Liquid_Oxygen(58.2, 66.4),
                    FuelType::HTP(1.22, 1.74),
                ]),
                "start",
            ),
            Engine::new(
                "RD-100", // Engine 5
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
                true,
                false,
                true,
                1,
                Size::Sm,
                0.0,
                FuelMix::new(&[
                    FuelType::Ethanol_75(71.3, 60.1),
                    FuelType::Liquid_Oxygen(63.1, 72.0),
                    FuelType::HTP(1.34, 1.92),
                ]),
                "Post-War Rocketry Testing",
            ),
            Engine::new(
                "XLR10", // Engine 6
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
                Size::Sm,
                0.0,
                FuelMix::new(&[
                    FuelType::Ethanol_90(30.5, 24.7),
                    FuelType::Liquid_Oxygen(24.4, 27.8),
                    FuelType::HTP(0.966, 1.38),
                ]),
                "Post-War Rocketry Testing",
            ),
            Engine::new(
                "XLR11", // Engine 7
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
                Size::Sm,
                0.0,
                FuelMix::new(&[
                    FuelType::Ethanol_75(5.85, 4.92),
                    FuelType::Liquid_Oxygen(6.21, 7.08),
                    FuelType::Nitrogen(136.0, 0.170),
                ]),
                "Post-War Rocketry Testing",
            ),
            Engine::new(
                "XLR41", // Engine 8
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
                Size::Sm,
                0.0,
                FuelMix::new(&[
                    FuelType::Ethanol_75(73.7, 62.1),
                    FuelType::Liquid_Oxygen(70.1, 80.0),
                    FuelType::HTP(1.44, 2.06),
                ]),
                "Post-War Rocketry Testing",
            ),
            Engine::new(
                "ORM-65", // Engine 9
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
                Size::Xs,
                0.0,
                FuelMix::new(&[
                    FuelType::Kerosene(0.218, 0.169),
                    FuelType::AK20(0.441, 0.677),
                    FuelType::Nitrogen(25.0, 31.3),
                ]),
                "Post-War Rocketry Testing",
            ),
        ];
        // Aerobee engine configurations
        engines[0].configurations[0] = EngineConfiguration::new(
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
            FuelMix::new(&[
                FuelType::AnilineFurfuryl_37p(1.58, 1.67),
                FuelType::IRFNA_III(2.74, 4.29),
                FuelType::Nitrogen(148.0, 0.185),
            ]),
            "Post-War Rocketry Testing",
        );
        engines[0].configurations[1] = EngineConfiguration::new(
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
            FuelMix::new(&[
                FuelType::AnilineFurfuryl_37p(1.58, 1.67),
                FuelType::IRFNA_III(2.74, 4.29),
                FuelType::Helium(148.0, 0.0264),
            ]),
            "Early Rocketry",
        );
        engines[0].configurations[2] = EngineConfiguration::new(
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
            FuelMix::new(&[
                FuelType::AnilineFurfuryl_37p(2.49, 2.64),
                FuelType::IRFNA_III(4.32, 6.75),
                FuelType::Helium(228.0, 0.0407),
            ]),
            "Early Rocketry",
        );

        // U-1250 engine configurations
        engines[1].configurations[0] = EngineConfiguration::new(
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
            FuelMix::new(&[
                FuelType::Kerosene(2.22, 1.73),
                FuelType::AK20(4.33, 6.65),
                FuelType::Nitrogen(216.0, 0.271),
            ]),
            "Post-War Rocketry Testing",
        );
        engines[1].configurations[1] = EngineConfiguration::new(
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
            FuelMix::new(&[
                FuelType::Kerosene(2.61, 2.02),
                FuelType::AK20(5.02, 7.69),
                FuelType::Nitrogen(263.0, 0.329),
            ]),
            "Early Rocketry",
        );

        // Veronique engine configurations
        engines[2].configurations[0] = EngineConfiguration::new(
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
            FuelMix::new(&[
                FuelType::Turpentine(4.84, 4.21),
                FuelType::IWFNA(9.95, 15.0),
                FuelType::Water(0.240, 0.240),
            ]),
            "Basic Rocketry",
        );
        engines[2].configurations[1] = EngineConfiguration::new(
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
            FuelMix::new(&[
                FuelType::Turpentine(7.25, 6.31),
                FuelType::IWFNA(14.9, 22.5),
                FuelType::Water(0.359, 0.359),
            ]),
            "1956-1957 Orbital Rocketry",
        );

        // A-4 engine configurations
        engines[4].configurations[0] = EngineConfiguration::new(
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
            FuelMix::new(&[
                FuelType::Hydyne(57.7, 49.6),
                FuelType::Liquid_Oxygen(57.7, 65.9),
                FuelType::HTP(1.15, 1.65),
            ]),
            "Post-War Rocketry Testing",
        );

        // RD-100 engine configurations
        engines[5].configurations[0] = EngineConfiguration::new(
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
            FuelMix::new(&[
                FuelType::Ethanol_90(87.9, 71.2),
                FuelType::Liquid_Oxygen(89.9, 103.0),
                FuelType::HTP(1.78, 2.55),
            ]),
            "Early Rocketry",
        );

        // ORM-65 engine configurations
        engines[9].configurations[0] = EngineConfiguration::new(
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
            FuelMix::new(&[
                FuelType::Kerosene(0.178, 0.139),
                FuelType::AK20(0.361, 0.554),
                FuelType::Nitrogen(20.5, 0.0256),
            ]),
            "Post-War Rocketry Testing",
        );
        engines[9].configurations[0] = EngineConfiguration::new(
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
            FuelMix::new(&[
                FuelType::Kerosene(0.178, 0.279),
                FuelType::AK20(0.361, 1.12),
                FuelType::Nitrogen(20.5, 0.0516),
            ]),
            "Early Rocketry",
        );
        // XLR11 engine configurations
        //engines[7].configurations[0] = EngineConfiguration::new("XLR11")

        engines
    }

    pub fn init_all_engines(engines: [Engine; NUM_EGINES]) -> HashMap<&'static str, Vec<Engine>> {
        let mut result: HashMap<&'static str, Vec<Engine>> =
            HashMap::with_capacity(TECH_TREE.len());
        for engine in engines.iter() {
            if let Some(vec) = result.get_mut(&engine.tech_tree_node) {
                vec.push(*engine);
            } else {
                result.insert(&engine.tech_tree_node, vec![*engine]);
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
                    let vec = vec![engine];
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
            fuel_mix: FuelMix::new(&[]),
            tech_tree_node: "start",
        };
        assert_eq!(engine.thrust_vac(), 16.24536);
    }
}
