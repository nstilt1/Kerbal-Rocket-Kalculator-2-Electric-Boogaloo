use serde::{ser::SerializeStruct, Serialize};

use crate::{debug, G};

use super::{
    engines::Engine,
    tanks::{cylindrical_tanks::CylindricalTank, nose_tanks::NoseCone},
    utils::ln,
};

#[derive(Debug, Clone)]
pub struct Rocket {
    nose: Option<NoseCone>,
    tank: Option<CylindricalTank>,
    nose_length: Option<f64>,
    nose_core: Option<&'static str>,
    nose_fuselage: Option<&'static str>,
    cyl_length: Option<f64>,
    cyl_fuselage: Option<&'static str>,
    engine: Engine,
    diameter: f64,
    fuel: String,
    num_engines: u8,
    pub(crate) mass: f64,
    dry_mass: f64,
    delta_v_asl: f64,
    delta_v_vac: f64,
    twr: f64,
    burn_time: f64,
}

/// Rounds a number to a specific number of decimal places
fn round(value: f64, decimal_places: usize) -> f64 {
    format!("{:.*}", decimal_places, value).parse().unwrap()
}

impl Serialize for Rocket {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let len = 13;
        let mut state = serializer.serialize_struct("Rocket", len)?;
        state.serialize_field("engine", &self.engine.get_name())?;
        state.serialize_field("numEngines", &self.num_engines)?;
        state.serialize_field("diameter", &round(self.diameter, 3))?;
        state.serialize_field("wetMass", &round(self.mass, 3))?;
        state.serialize_field("dryMass", &round(self.dry_mass, 3))?;
        state.serialize_field("deltaVAsl", &round(self.delta_v_asl, 0))?;
        state.serialize_field("deltaVVac", &round(self.delta_v_vac, 0))?;
        state.serialize_field("twr", &round(self.twr, 2))?;
        state.serialize_field("noseCore", self.nose_core.unwrap_or("N/A"))?;
        state.serialize_field("noseFuselage", self.nose_fuselage.unwrap_or("N/A"))?;
        state.serialize_field("noseLength", &round(self.nose_length.unwrap_or(0.0), 2))?;
        state.serialize_field("cylFuselage", &self.cyl_fuselage.unwrap_or("N/A"))?;
        state.serialize_field("cylLength", &round(self.cyl_length.unwrap_or(0.0), 3))?;
        state.end()
    }
}

impl Rocket {
    pub fn new(
        nose: Option<NoseCone>,
        tank: Option<CylindricalTank>,
        engine: Engine,
        fuel: String,
        num_engines: u8,
        mass: f64,
        dry_mass: f64,
        twr: f64,
        burn_time: f64,
    ) -> Self {
        let natural_logarithm_g = ln(mass / dry_mass) * G;
        debug!("Ln Ratio: {}", mass / dry_mass);
        let (nose_length, nose_fuselage, nose_core) = if let Some(v) = &nose {
            (Some(v.length), Some(v.fuselage.name), Some(v.core.name))
        } else {
            (None, None, None)
        };
        let (tank_length, tank_fuselage) = if let Some(v) = &tank {
            (Some(v.length), Some(v.fuselage.name))
        } else {
            (None, None)
        };
        Rocket {
            nose,
            nose_length,
            nose_core,
            nose_fuselage,
            tank: tank,
            cyl_length: tank_length,
            cyl_fuselage: tank_fuselage,
            engine: engine.clone(),
            diameter: engine.diameter,
            fuel,
            num_engines,
            mass: mass / 1000.0,
            dry_mass: dry_mass / 1000.0,
            delta_v_vac: natural_logarithm_g * engine.isp_vac,
            delta_v_asl: natural_logarithm_g * engine.isp_asl,
            twr,
            burn_time,
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
