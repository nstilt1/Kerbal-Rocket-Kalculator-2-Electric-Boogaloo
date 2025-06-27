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
        let len = 14;
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
        state.serialize_field("maxAltitude", &round(self.calculate_max_altitude(), 0))?;
        state.end()
    }
}

const SCALE_HEIGHT: f64 = 8500.0; // m, for a rough barometric model

fn ambient_pressure(h: f64) -> f64 {
    // simple exponential atmosphere
    101325.0 * (-h / SCALE_HEIGHT).exp()
}

fn interpolate(x0: f64, x1: f64, frac: f64) -> f64 {
    x0 + (x1 - x0) * frac
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
        let burn_time = {
            let fuel_mass = mass - dry_mass;
            let flow_rate: f64 = engine.fuel_mix.iter().map(|fuel| fuel.flow_rate_kgps).sum::<f64>() * num_engines as f64;
            fuel_mass / flow_rate
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

    #[cfg(not(target_arch = "wasm32"))]
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

    /// Calculates the maximum altitude of this rocket, but it assumes constant 
    /// thrust and constant Isp.
    fn calculate_max_altitude(&self) -> f64 {
        let dt = 0.1;        // time step in seconds
        let mut t  = 0.0;
        let mut m  = self.mass;     // wet mass
        let mut v  = 0.0;           // velocity
        let mut h  = 0.0;           // altitude

        while t < self.burn_time {
            // --- 1. local ambient pressure & interpolation frac ---
            let p_amb = ambient_pressure(h);
            // frac = 1.0 at vacuum (p_amb=0), 0.0 at sea level (p_amb=101325)
            let frac = (101325.0 - p_amb) / 101325.0;

            // --- 2. thrust & Isp at this point ---
            let thrust = interpolate(
                self.engine.thrust_asl,
                self.engine.thrust_vac,
                frac,
            );
            let isp = interpolate(
                self.engine.isp_asl,
                self.engine.isp_vac,
                frac,
            );

            // --- 3. mass flow & update mass ---
            let mdot = thrust / (isp * G);
            m -= mdot * dt;
            if m <= self.dry_mass { break; } // avoid negative mass

            // --- 4. acceleration & integrate ---
            let a = thrust / m - G;
            v += a * dt;
            h += v * dt;

            // never go below ground
            if h < 0.0 {
                h = 0.0;
                v = 0.0;
            }

            t += dt;
        }

        // coast to apogee
        h + v * v / (2.0 * G)
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
