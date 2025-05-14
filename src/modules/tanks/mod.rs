pub mod tanks;
pub use tanks::*;
pub mod nose_tanks;
pub mod cyllindrical_tanks;

pub enum TankType {
    Cylindrical,
    Nosecone,
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
            TankType::Nosecone => 0.25 * diameter
        }
    }
}