use super::{tanks::FuelStack, engines::Engine};

#[derive(Debug, Clone)]
pub struct Rocket {
    fuel_config: FuelStack,
    engine: Engine,
    num_engines: u8,
    pub(crate) mass: f64,
    twr: f64
}

impl Rocket {
    pub fn new(fuel_stack: FuelStack, 
        engine: Engine, 
        num_engines: u8, 
        mass: f64, 
        twr: f64
    ) -> Self{
        Rocket {
            fuel_config: fuel_stack,
            engine,
            num_engines,
            mass,
            twr
        }
    }

    pub fn print(&self) {
        println!("{}x {}\n{}x stacks with {}", self.num_engines, self.engine.name, self.num_engines, self.fuel_config.get_name());
    }
}

impl PartialOrd for Rocket {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.mass.partial_cmp(&other.mass)
    }
}

impl PartialEq for Rocket {
    fn eq(&self, other: &Self) -> bool {
        self.mass == other.mass
    }
}