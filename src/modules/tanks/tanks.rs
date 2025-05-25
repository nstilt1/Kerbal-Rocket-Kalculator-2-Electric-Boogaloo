use super::super::{fuel_type::FuelType, size::Size};

#[derive(Debug, PartialEq, Clone)]
pub struct Tank {
    pub name: String,
    pub wet_mass: f64,
    pub dry_mass: f64,
    pub size: Size,
    pub cost: i64,
    pub fuel_type: FuelType,
    tank_type: TankType,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TankType {
    NoseTank,
    CylindricalTank,
    RadialTank,
}
impl TankType {
    pub fn get_max_tanks(&self) -> u8 {
        match self {
            TankType::NoseTank => 1,
            TankType::CylindricalTank => 4,
            TankType::RadialTank => 8,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct FuelStack {
    pub tanks: Vec<Tank>,
    pub size: Size,
    pub fuel_type: FuelType,
}
impl FuelStack {
    pub fn new(tanks: Vec<Tank>) -> Self {
        let t = tanks[0].to_owned();
        let size = t.size;
        FuelStack {
            tanks,
            size,
            fuel_type: t.fuel_type,
        }
    }

    pub fn multiple(tank: &Tank, count: u8) -> Self {
        let mut tanks: Vec<Tank> = Vec::new();
        for _x in 0..count {
            tanks.push(tank.clone());
        }
        FuelStack::new(tanks)
    }
}

impl FuelStack {
    pub fn get_wet_mass(&self) -> f64 {
        let mut result: f64 = 0.0;
        for t in self.tanks.to_owned() {
            result += t.wet_mass;
        }
        result
    }
    pub fn get_dry_mass(&self) -> f64 {
        let mut result: f64 = 0.0;
        for t in self.tanks.to_owned() {
            result += t.dry_mass;
        }
        result
    }
    pub fn get_name(&self) -> String {
        let mut result: String = "".to_owned();
        for t in self.tanks.to_owned() {
            if result.len() != 0 {
                result.push_str(", ");
            }
            result.push_str(&t.name);
        }
        result
    }
}
