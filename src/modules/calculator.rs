use crate::G;

use super::{tanks::{Tank, self, FuelStack}, engines::Engine, rocket_config::Rocket, size::Size, fuel_type::FuelType};

#[derive(Debug, Clone, PartialEq)]
pub struct Calculator {
    size: Size,
    target_dv: f64,
    mass: f64,
    minimum_twr: f64,
    needs_gimballing: bool,
    in_vacuum: bool,
    pub engines: Vec<Engine>,
    pub tanks: Vec<FuelStack>
}

impl Calculator {
    pub fn new() -> Self {
        Calculator { 
            size: Size::Xs, 
            target_dv: 0.0, 
            mass: 0.0, 
            minimum_twr: 0.0, 
            needs_gimballing: false, 
            in_vacuum: false, 
            engines: Engine::init_engines(), 
            tanks: Tank::init_tanks()
        }
    }
    pub fn init(
        &mut self, 
        mass: f64, 
        target_dv: f64, 
        minimum_twr: f64, 
        needs_gimballing: bool, 
        in_vacuum: bool, 
        size: Size
    ) {
        self.mass = mass;
        self.target_dv = target_dv;
        self.minimum_twr = minimum_twr;
        self.needs_gimballing = needs_gimballing;
        self.in_vacuum = in_vacuum;
        self.size = size;
    }

    pub fn calculate(&self) -> Vec<Rocket> {
        let mut result: Vec<Rocket> = Vec::new();

        let (engine_indices, tank_indices) = self.get_indices();

        for eng in engine_indices.0..engine_indices.1 {
            let engine = self.engines[eng].to_owned();
            if self.needs_gimballing && !engine.has_gimbal {
                continue;
            }

            // the ratio (wetMass/dryMass) required to reach the delta-v
            // derived from:
            // deltaV = ln(wetMass/dryMass)*g*isp
            // deltaV/g/isp = ln(wetMass/dryMass)
            // wetMass/dryMass = e^(deltaV/g/engine.isp)
            let target_ratio = f64::exp(self.target_dv / G / if self.in_vacuum {engine.isp_vac}else{engine.isp_asl});

            for num_engines in 1..=9 {
                if num_engines == 2 || num_engines == 6 || num_engines == 8 {
                    continue;
                }

                let mass_offset = 0.0;
                let partial_mass  = mass_offset + (num_engines as f64 * engine.mass) + self.mass;

                for fuel_stack_index in tank_indices.0..tank_indices.1 {
                    let fuel_stack = self.tanks[fuel_stack_index].to_owned();
                    
                    let thrust: f64 = if self.in_vacuum {engine.thrust_vac}else{engine.thrust_asl} * num_engines as f64;

                    let wet_mass = num_engines as f64 * fuel_stack.get_wet_mass() + partial_mass;
                    let dry_mass = num_engines as f64 * fuel_stack.get_dry_mass() + partial_mass;

                    let twr = thrust / wet_mass / G;
                    if twr < self.minimum_twr {
                        continue;
                    }
                    // we have enough twr, but do we have enough delta-v?
                    if wet_mass / dry_mass >= target_ratio {
                        result.push(Rocket::new(fuel_stack, engine.clone(), num_engines, wet_mass, twr));
                        break;
                    }
                }
            }
        }
        return result;
    }

    /**
     * Get min/max indices of self.engines and self.tanks with the same size
     */
    pub fn get_indices(&self) -> ((usize, usize), (usize, usize)) {
        let mut i1 = -1;
        let mut i2 = -1;
        let mut j1 = -1;
        let mut j2 = -1;
        let mut t = 0;
        for e in self.engines.iter() {
            if i1 == -1 {
                if e.size == self.size && e.fuel_type == FuelType::Methalox {
                    i1 = t;
                }
            }else if i2 == -1 {
                if e.size != self.size && e.fuel_type == FuelType::Methalox {
                    i2 = t;
                    break;
                }
            }
            t += 1;
        }
        if i1 == -1 {
            println!("Engine size not found in list");
            return ((0,0),(0,0));
        }else if i2 == -1 {
            i2 = self.engines.len() as i32;
        }

        t = 0;
        for tank in self.tanks.iter() {
            if j1 == -1 {
                if tank.size == self.size && tank.fuel_type == FuelType::Methalox {
                    j1 = t;
                }
            }else if j2 == -1 {
                if tank.size != self.size && tank.fuel_type == FuelType::Methalox {
                    j2 = t;
                }
            }
            t += 1;
        }
        if j1 == -1 {
            println!("Engine size not found in list");
            return ((0,0),(0,0));
        }else if i2 == -1 {
            j2 = self.tanks.len() as i32;
        }
        
        ((i1 as usize, i2 as usize), (j1 as usize, j2 as usize))
    }
}