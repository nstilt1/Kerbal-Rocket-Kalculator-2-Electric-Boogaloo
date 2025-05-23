use super::{
    engines::Engine,
    tanks::{cylindrical_tanks::CylindricalTank, nose_tanks::NoseCone},
};

#[derive(Debug, Clone)]
pub struct Rocket {
    nose: Option<NoseCone>,
    tank: Option<CylindricalTank>,
    engine: Engine,
    fuel: String,
    num_engines: u8,
    pub(crate) mass: f64,
    twr: f64,
}

impl Rocket {
    pub fn new(
        nose: Option<NoseCone>,
        tank: Option<CylindricalTank>,
        engine: Engine,
        fuel: String,
        num_engines: u8,
        mass: f64,
        twr: f64,
    ) -> Self {
        Rocket {
            nose,
            tank,
            engine,
            fuel,
            num_engines,
            mass,
            twr,
        }
    }

    pub fn to_string(&self) -> String {
        let mut result = format!(
            "Mass: {}\nTWR: {}\n{}x {}\n{}x stacks",
            self.mass, self.twr, self.num_engines, self.engine.name, self.num_engines
        );
        if let Some(tank) = &self.tank {
            result.push_str(&format!(
                "Cylindrical tank with {}m length and {}m diameter and {}",
                tank.length, tank.diameter, tank.fuselage.name
            ));
        }
        if let Some(nose) = &self.nose {
            result.push_str(&format!(
                "Nosecone {} with {}m length and {}m diameter and {}",
                nose.core.name, nose.length, nose.diameter, nose.fuselage.name
            ));
        }
        result.push_str(&format!("{}", self.fuel));

        result
    }
    #[cfg(not(target_arch = "wasm32"))]
    pub fn print(&self) {
        println!("Mass: {}\nThrust to weight ratio: {}", self.mass, self.twr);
        println!(
            "{}x {}\n{}x stacks",
            self.num_engines, self.engine.name, self.num_engines
        );
        if let Some(tank) = &self.tank {
            println!(
                "Cylindrical tank with {}m length and {}m diameter and {}",
                tank.length, tank.diameter, tank.fuselage.name
            );
        }
        if let Some(nose) = &self.nose {
            println!(
                "Nosecone {} with {}m length and {}m diameter and {}",
                nose.core.name, nose.length, nose.diameter, nose.fuselage.name
            )
        }
        println!("{}", self.fuel);
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
