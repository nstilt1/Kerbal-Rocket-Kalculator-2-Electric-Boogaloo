pub mod tanks;
use std::collections::HashMap;

use serde::Serialize;
pub use tanks::*;
pub mod cylindrical_tanks;
pub mod nose_tanks;

pub enum TankType {
    Cylindrical,
    Nosecone,
}

pub struct Fuselages {
    pub hp_fuselages: HashMap<&'static str, Fuselage>,
    pub non_hp_fuselages: HashMap<&'static str, Fuselage>
}

impl Fuselages {
    pub fn new(hp_fuselages: HashMap<&'static str, Fuselage>, non_hp_fuselages: HashMap<&'static str, Fuselage>) -> Self {
        Self {
            hp_fuselages,
            non_hp_fuselages
        }
    }
}

#[const_trait]
pub trait Tanks {
    const MIN_VSA: f64;
    const MAX_VSA: f64;
    const MIN_DIAMETER: f64 = 0.1;
    const MAX_DIAMETER: f64 = 0.1;
    const TANK_TYPE: TankType = TankType::Nosecone;

    fn min_height(&self, diameter: f64) -> f64 {
        match Self::TANK_TYPE {
            TankType::Cylindrical => Self::MIN_VSA * diameter,
            TankType::Nosecone => 0.25 * diameter,
        }
    }

    /// Initializes fuselage types as a pair of hashmaps.
    ///
    /// (HP Map<Name, (density, utilization)>, Non-HP Map<Name, (density, utilization)>)
    fn init_fuselage_types() -> Fuselages;
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub struct Fuselage {
    pub name: &'static str,
    pub density: f64,
    pub utilization: f64,
}

impl Fuselage {
    pub const fn new(name: &'static str, density: f64, utilization: f64) -> Self {
        assert!(utilization <= 100.0);
        assert!(utilization > 0.0);
        let utilization = if utilization > 1.0 {
            utilization / 100.0
        } else {
            utilization
        };
        Self {
            name,
            density,
            utilization,
        }
    }
    pub const fn unused_volume_ratio(&self) -> f64 {
        1.0 - self.utilization
    }
}

pub mod fuselage_names {
    pub const STEEL_FUSELAGE_NAME: &str = "Steel Fuselage";
    pub const HP_STEEL_FUSELAGE_NAME: &str = "HP Steel Fuselage";
    pub const AL_FUSELAGE_NAME: &str = "Al Fuselage";
    pub const HP_AL_FUSELAGE_NAME: &str = "HP Al Fuselage";
    pub const AL_STRINGER_TANK_NAME: &str = "Al Stringer Tank";
    pub const HP_AL_STRINGER_TANK_NAME: &str = "HP Al Stringer Tank";
    pub const REFINED_AL_STRINGER_TANK_NAME: &str = "Refined Al Stringer Tank";
    pub const HP_REFINED_AL_STRINGER_TANK_NAME: &str = "HP Refined Al Stringer Tank";
    pub const AL_LI_STRINGER_TANK_NAME: &str = "Al-Li Stringer Tank";
    pub const HP_AL_LI_STRINGER_TANK_NAME: &str = "HP Al-Li Stringer Tank";
    pub const REFINED_AL_LI_STRINGER_TANK_NAME: &str = "Refined Al-Li Stringer Tank";
    pub const HP_REFINED_AL_LI_STRINGER_TANK_NAME: &str = "HP Refined Al-Li Stringer Tank";
    pub const STEEL_STIR_WELDED_TANK_NAME: &str = "Steel Stir-Welded Tank";
    pub const HP_STEEL_STIR_WELDED_TANK_NAME: &str = "HP Steel Stir-Welded Tank";
}
