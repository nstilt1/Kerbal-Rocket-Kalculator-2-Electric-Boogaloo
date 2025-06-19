mod early_engines;
mod lunar_landing;
mod or_2019_2028;
mod or_2009_2013;
mod orsc_2014_2018;
use std::{collections::HashMap, sync::LazyLock};

use crate::modules::{engines::{lunar_landing::LUNAR_LANDING_ENGINES, or_2009_2013::OR_2009_2013_ENGINES, or_2019_2028::OR_2019_2028_ENGINES, orsc_2014_2018::ORSC_2014_2018_ENGINES}, fuel_type::Fuel};

pub static ENGINES: LazyLock<[Engine; NUM_ENGINES]> = LazyLock::new(|| Engine::init_rp1_engines());

//const NUM_ENGINES: usize = 10;
const NUM_ENGINES: usize = 11;
pub type Engine = EngineV1;

pub const TECH_TREE: &[&'static str] = &[
    "start",
    "Post-War Rocketry Testing",
    "Early Rocketry",
    "Basic Rocketry",
    "1956-1957 Orbital Rocketry",
    "Lunar Landing",
    "2009-2013 Orbital Rocketry",
    "2014-2018 ORSC Engines",
    "2019-2028 Orbital Rocketry",
];

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
    pub thrust_vac: f64,
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
            thrust_vac: thrust_kn * (isp_vac / isp_asl),
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

    pub const fn new_v2(
        name: &'static str,
        thrust_asl: f64,
        thrust_vac: f64,
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
            thrust_kn: thrust_asl,
            thrust_vac,
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
            thrust_vac: 0.0,
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
                _ => 1.0, // Default utilization factor
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
                _ => 1.0,
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

    pub fn init_all_engines() -> HashMap<&'static str, Vec<Engine>> {
        let mut result: HashMap<&'static str, Vec<Engine>> =
            HashMap::with_capacity(TECH_TREE.len());
        let engine_groups = &[
            &ENGINES.as_slice(), 
            &LUNAR_LANDING_ENGINES.as_slice(),
            &OR_2009_2013_ENGINES.as_slice(),
            &ORSC_2014_2018_ENGINES.as_slice(),
            &OR_2019_2028_ENGINES.as_slice(),
        ];
        for engines in engine_groups.iter() {
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
        }
        result
    }

    /// Gets the name of this engine, including the parent engine's name.
    pub fn get_name(&self) -> String {
        match self.parent_name.is_empty() {
            true => self.name.to_string(),
            false => format!("{}: {}", self.name, self.parent_name),
        }
    }
}
