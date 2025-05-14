use super::{size::Size, fuel_type::FuelType};

const MAX_ENGINE_CONFIGS: usize = 4;
pub const ENGINES: [Engine; NUM_EGINES] = Engine::init_rp1_engines();

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Engine {
        pub name: &'static str, 
        is_solid: bool,
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
        pub fuel_types: &'static [FuelType],
        pub configurations: [EngineConfiguration; MAX_ENGINE_CONFIGS]
}

#[derive(Debug, Clone, Copy, PartialEq)]
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
        }
    }
}

const NUM_EGINES: usize = 5;

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
        fuel_types: &'static [FuelType],
    ) -> Self {
        Engine {
            name,
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
            fuel_types,
            configurations: [EngineConfiguration::zeroed(); MAX_ENGINE_CONFIGS],
        }
    }

    pub const fn init_rp1_engines() -> [Engine; NUM_EGINES] {
        let mut engines = [
            Engine::new("Aerobee", false, 6.7, 7.7, 100.0, 195.0, 226.0, 0.008, 1.2, 47.0, false, false, true, false, true, 1, Size::Xs, 0.0, &[FuelType::AnilineFurfuryl_22p(0.893), FuelType::IRFNA_III(1.64), FuelType::Nitrogen(78.1)]),
            Engine::new("U-1250", false, 12.7, 14.4, 100.0, 204.8, 232.1, 0.0156, 1.2, 56.0, false, false, true, false, true, 1, Size::Xs, 1.0, &[FuelType::Kerosene(1.71), FuelType::AK20(3.26), FuelType::Nitrogen(157.0)]),
            Engine::new("Veronique", false, 39.2, 49.3, 100.0, 198.0, 249.0, 0.1511, 3.92, 45.0, false, false, true, false, true, 1, Size::Xs, 1.0, &[FuelType::Kerosene(5.46), FuelType::IRFNA_III(10.2), FuelType::Water(0.216)]),
            Engine::new("Tiny Tim Booster", true, 133.4, 146.6, 100.0, 202.0, 222.0, 0.0673, 3.9, 5.0, false, false, false, false, false, 1, Size::Xs, 41.3903, &[FuelType::NGNC(42.1)]),
            Engine::new("A4", false, 238.8, 284.7, 100.0, 203.0, 242.0, 0.9299, 0.47, 70.0, false, false, false, false, true, 1, Size::Sm, 0.0, &[FuelType::Ethanol_75(63.6), FuelType::Liquid_Oxygen(58.2), FuelType::HTP(1.22)]),
        ];
        // Aerobee engine configurations
        engines[0].configurations[0] = EngineConfiguration::new("XASR-1", 13.8, 100.0, 0.010, 200.0, 235.44, 40.0, true, true, 1);
        engines[0].configurations[1] = EngineConfiguration::new("XASR-2", 13.8, 100.0, 0.010, 200.0, 235.44, 40.0, true, true, 1);
        engines[0].configurations[2] = EngineConfiguration::new("AJ10-27", 21.3, 100.0, 0.012, 198.0, 231.0, 52.0, true, true, 1);
        
        // U-1250 engine configurations
        engines[1].configurations[0] = EngineConfiguration::new("U-1700", 19.4, 100.0, 0.015, 206.5, 236.4, 60.0, true, true, 1);
        engines[1].configurations[1] = EngineConfiguration::new("U-2000", 23.0, 100.0, 0.013, 205.6, 241.3, 60.0, true, true, 1);

        // Veronique engine configurations
        engines[2].configurations[0] = EngineConfiguration::new("VeroniqueAGI", 49.3, 100.0, 0.150, 208.0, 261.0, 49.0, true, true, 1);
        engines[2].configurations[1] = EngineConfiguration::new("Veronique61", 73.8, 100.0, 0.150, 208.0, 261.0, 56.0, true, true, 1);
        
        engines
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
        };
        assert_eq!(engine.thrust_vac(), 16.24536);
    }
}