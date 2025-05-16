use std::collections::HashSet;

use crate::G;

use super::{engines::{Engine, ENGINES}, fuel_type::FuelType, rocket_config::Rocket, size::Size, tanks::{cylindrical_tanks::{cylindrical_dry_mass, tank_volume, CylindricalTank}, nose_tanks::{calculate_corrected_volume, calculate_nose_dry_mass, NoseCone, NoseConeVariant}, FuelStack, Fuselage, Tank, Tanks}};

#[derive(Debug, Clone, PartialEq)]
pub struct Calculator {
    size: Size,
    target_dv: f64,
    mass: f64,
    minimum_twr: f64,
    needs_gimballing: bool,
    in_vacuum: bool,
    use_nosecone: bool,
    unlocked_fusalages: String,
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
            use_nosecone: false,
            unlocked_fusalages: "Steel Fuselage".to_string(),
            tanks: Tank::init_tanks()
        }
    }

    /// Set the calculator's variables
    pub fn init(
        &mut self, 
        mass: f64, 
        target_dv: f64, 
        minimum_twr: f64, 
        needs_gimballing: bool, 
        in_vacuum: bool, 
        use_nosecone: bool,
        size: Size,
        unlocked_fuselages: String,
    ) {
        self.mass = mass * 1000.0;
        self.target_dv = target_dv;
        self.minimum_twr = minimum_twr;
        self.needs_gimballing = needs_gimballing;
        self.in_vacuum = in_vacuum;
        self.size = size;
        self.use_nosecone = use_nosecone;
        self.unlocked_fusalages = unlocked_fuselages;
    }

    /// Calculates the parts required to build a rocket with specific arguments.
    /// 
    /// Returns (nose+cylinder_results, cylinder_results, nose_results)
    pub fn calculate(&self) -> (Vec<Rocket>, Vec<Rocket>, Vec<Rocket>) {
        println!("Mass = {}\ntarget_dv = {}\nminimum twr = {}\nsize = {}", self.mass, self.target_dv, self.minimum_twr, self.size.get_diameter());
        let mut result: Vec<Rocket> = Vec::new();

        let engines = Engine::init_rp1_engines();
        let nosecones = NoseConeVariant::nosecones();
        let nosecone_cores = &nosecones.cores;
        let unlocked_fuselages: Vec<&str> = self.unlocked_fusalages.split(',').collect();
        let (nosecone_hp_fuselages, nosecone_non_hp_fuselages) = NoseConeVariant::init_fuselage_types();
        let mut unlocked_nosecone_hp_fuselages: Vec<&Fuselage> = Vec::with_capacity(unlocked_fuselages.len());
        let mut unlocked_nosecone_non_hp_fuselages: Vec<&Fuselage> = Vec::with_capacity(unlocked_fuselages.len());
        let (cylinder_hp_fuselages, cylinder_non_hp_fuselages) = CylindricalTank::init_fuselage_types();
        let mut unlocked_cylinder_hp_fuselages: Vec<&Fuselage> = Vec::with_capacity(unlocked_fuselages.len());
        let mut unlocked_cylinder_non_hp_fuselages: Vec<&Fuselage> = Vec::with_capacity(unlocked_fuselages.len());
        for unlocked_fuselage in unlocked_fuselages {
            unlocked_nosecone_hp_fuselages.push(nosecone_hp_fuselages.get(format!("HP {}", unlocked_fuselage).as_str()).expect(&format!("fuselage not found: '{}'", unlocked_fuselage)));
            unlocked_nosecone_non_hp_fuselages.push(nosecone_non_hp_fuselages.get(unlocked_fuselage).expect("fuselage not found"));
            unlocked_cylinder_hp_fuselages.push(cylinder_hp_fuselages.get(format!("HP {}", unlocked_fuselage).as_str()).expect("fuselage not found"));
            unlocked_cylinder_non_hp_fuselages.push(cylinder_non_hp_fuselages.get(unlocked_fuselage).expect("fuselage not found"));
            
            unlocked_nosecone_hp_fuselages.sort_by_key(|s| s.name);
            unlocked_nosecone_non_hp_fuselages.sort_by_key(|s| s.name);
            unlocked_cylinder_hp_fuselages.sort_by_key(|s| s.name);
            unlocked_cylinder_non_hp_fuselages.sort_by_key(|s| s.name);
        }

        let mut only_nose_results: Vec<Rocket> = Vec::new();
        let mut only_cylinder_results: Vec<Rocket> = Vec::new();
        'engine_loop: for engine in engines.iter() {
            let (cyl_fuselages, nose_fuselages) = if engine.hp_fuel {
                (&unlocked_cylinder_hp_fuselages, &unlocked_nosecone_hp_fuselages)
            } else {
                (&unlocked_cylinder_non_hp_fuselages, &unlocked_nosecone_non_hp_fuselages)
            };
            
            // the ratio (wetMass/dryMass) required to reach the delta-v is
            // derived from:
            // deltaV = ln(wetMass/dryMass)*g*isp
            // deltaV/g/isp = ln(wetMass/dryMass)
            // wetMass/dryMass = e^(deltaV/g/engine.isp)
            // BUT... ln() might have better performance over e^(), but we only 
            // calculate e^x once.
            let target_ratio = std::f64::consts::E.powf(self.target_dv / G / if self.in_vacuum {engine.isp_vac}else{engine.isp_asl});

            for (nose_fuselage, cyl_fuselage) in nose_fuselages.iter().zip(cyl_fuselages) {
                
                if engine.size.ne(&self.size) {
                    continue 'engine_loop;
                }
                if self.needs_gimballing && !engine.has_gimbal {
                    continue 'engine_loop;
                }

                'num_engine_loop: for num_engines in 1..=9 {
                    if num_engines == 2 || num_engines == 6 || num_engines == 8 {
                        continue 'num_engine_loop;
                    }
                    let mass_offset = 0.0;
                    // partial mass = offset + engines' mass + payload mass
                    let partial_mass = mass_offset + (num_engines as f64 * engine.mass * 1000.0) + self.mass;
                    println!("engine.mass = {}\npartial_mass = {}", engine.mass * 1000.0, partial_mass);
                    let thrust = if self.in_vacuum { engine.thrust_vac } else { engine.thrust_asl } * num_engines as f64;

                    for fuel in engine.fuel_types {
                        let max_volume_per_stack = fuel.max_volume(engine.rated_burn_time);
                        if self.use_nosecone {
                            for nosecone_core in nosecone_cores {
                                
                                let min_height = nosecone_core.base_length * NoseConeVariant::MIN_VSA;
                                let max_height = nosecone_core.base_length * NoseConeVariant::MAX_VSA;
                                let mut height = min_height;
                                while height <= max_height {
                                    let nose_volume = calculate_corrected_volume(self.size.get_diameter(), height, nosecone_core.correction_coefficient) * nose_fuselage.utilization;
                                    if nose_volume > max_volume_per_stack {
                                        break;
                                    }
                                    let nose_dry_mass = calculate_nose_dry_mass(self.size.get_diameter(), height, nose_fuselage.density, nose_fuselage.utilization, nosecone_core.correction_coefficient) * num_engines as f64;
                                    let nose_wet_mass = nose_dry_mass + fuel.mass(nose_volume * num_engines as f64) * num_engines as f64;
                                    
                                    let wet_mass = nose_wet_mass + partial_mass;
                                    let dry_mass = nose_dry_mass + partial_mass;

                                    let twr = thrust / wet_mass / G;
                                    if twr < self.minimum_twr {
                                        // TODO: maybe consider breaking here, but first check that the results are the same
                                        continue 'num_engine_loop;
                                    }

                                    // we have enough twr, but do we have enough delta-v?
                                    if wet_mass / dry_mass >= target_ratio {
                                        only_nose_results.push(
                                            Rocket::new(Some(NoseCone {
                                                core: nosecone_core.clone(),
                                                length: height,
                                                diameter: self.size.get_diameter(),
                                                fuselage: **nose_fuselage
                                            }),
                                            None,
                                            *engine,
                                            num_engines,
                                            wet_mass,
                                            twr
                                        ));
                                        break;
                                    }

                                    // not enough delta v, let's add a cylindrical tank
                                    let min_cyl_height = 0.1f64.max(self.size.get_diameter() * CylindricalTank::MIN_VSA);
                                    let max_cyl_height = CylindricalTank::MAX_VSA;
                                    let mut cyl_height = min_cyl_height;
                                    while cyl_height <= max_cyl_height {
                                        let cyl_volume = tank_volume(self.size.get_diameter(), cyl_height) * cyl_fuselage.utilization;
                                        if nose_volume + cyl_volume > max_volume_per_stack {
                                            break;
                                        }
                                        let cyl_dry_mass = cylindrical_dry_mass(self.size.get_diameter(), cyl_height, cyl_fuselage.utilization, cyl_fuselage.density) * num_engines as f64;
                                        let cyl_wet_mass = cyl_dry_mass + fuel.mass(cyl_volume * num_engines as f64);

                                        let dry_mass = nose_dry_mass + partial_mass + cyl_dry_mass;
                                        let wet_mass = nose_wet_mass + partial_mass + cyl_wet_mass;

                                        let twr = thrust / wet_mass / G;
                                        if twr < self.minimum_twr {
                                            // TODO: consider breaking here instead of continuing
                                            continue 'num_engine_loop;
                                        }

                                        // we have enough twr, but do we have enough delta-v?
                                        if wet_mass / dry_mass >= target_ratio {
                                            result.push(
                                                Rocket::new(
                                                    Some(
                                                        NoseCone { 
                                                            core: nosecone_core.clone(), 
                                                            length: height, 
                                                            diameter: self.size.get_diameter(), 
                                                            fuselage: **nose_fuselage
                                                        }
                                                    ),
                                                    Some(
                                                        CylindricalTank { 
                                                            length: cyl_height, 
                                                            diameter: self.size.get_diameter(), 
                                                            fuselage: **cyl_fuselage 
                                                        }
                                                    ),
                                                    *engine,
                                                    num_engines,
                                                    wet_mass,
                                                    twr
                                                )
                                            );
                                            break;
                                        }
                                        cyl_height += 0.05;
                                    }

                                    
                                    height += 0.05;
                                }
                            }
                        }
                        /* No nosecones!! */
                        let min_cyl_height = 0.1f64.max(self.size.get_diameter() * CylindricalTank::MIN_VSA);
                        let max_cyl_height = CylindricalTank::MAX_VSA;
                        let mut cyl_height = min_cyl_height;
                        while cyl_height <= max_cyl_height {
                            let cyl_volume = tank_volume(self.size.get_diameter(), cyl_height) * cyl_fuselage.utilization;
                            if cyl_volume > max_volume_per_stack {
                                println!("cyl_volume exceeded max volume with height = {} min_height = {}", cyl_height, min_cyl_height);
                                break;
                            }
                            let cyl_dry_mass = cylindrical_dry_mass(self.size.get_diameter(), cyl_height, cyl_fuselage.utilization, cyl_fuselage.density) * num_engines as f64;
                            let cyl_wet_mass = cyl_dry_mass + fuel.mass(cyl_volume * num_engines as f64);

                            let dry_mass = partial_mass + cyl_dry_mass;
                            let wet_mass = partial_mass + cyl_wet_mass;

                            let twr = thrust / wet_mass / G;
                            if twr < self.minimum_twr {
                                // TODO: consider breaking here instead of continuing
                                println!("Calculated TWR ({}) was less than minimum TWR ({})\nthrust = {}kN, wet_mass = {}kg, G = {}m/s/s", twr, self.minimum_twr, thrust, wet_mass, G);
                                println!("Engine: {}\nnum_engines: {}\ncyl_height: {}m", engine.name, num_engines, cyl_height);
                                println!("cyl_diameter: {}m\ncyl_dry_mass: {}kg\ncyl_wet_mass: {}kg", self.size.get_diameter(), cyl_dry_mass, cyl_wet_mass);
                                println!("fuel: {}\npartial_mass = {}kg\n", fuel.name(), partial_mass);
                                continue 'num_engine_loop;
                            }
                            println!("TWR passed check = {}", twr);

                            // we have enough twr, but do we have enough delta-v?
                            if wet_mass / dry_mass >= target_ratio {
                                only_cylinder_results.push(
                                    Rocket::new(
                                        None,
                                        Some(
                                            CylindricalTank { 
                                                length: cyl_height, 
                                                diameter: self.size.get_diameter(), 
                                                fuselage: **cyl_fuselage 
                                            }
                                        ),
                                        *engine,
                                        num_engines,
                                        wet_mass,
                                        twr
                                    )
                                );
                                break;
                            }
                            cyl_height += 0.05;
                        }
                    }
                }
            }
        }
        return (result, only_cylinder_results, only_nose_results);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fuselage_retrieval() {
        let unlocked_fuselages: Vec<&str> = "Steel Fuselage,Al Stringer Tank".split(',').collect();
        let (nosecone_hp_fuselages, nosecone_non_hp_fuselages) = NoseConeVariant::init_fuselage_types();
        let keys = nosecone_hp_fuselages.keys();
        for k in keys {
            println!("{}", k);
        }
        let mut unlocked_nosecone_hp_fuselages: Vec<&Fuselage> = Vec::with_capacity(unlocked_fuselages.len());
        let mut unlocked_nosecone_non_hp_fuselages: Vec<&Fuselage> = Vec::with_capacity(unlocked_fuselages.len());
        let (cylinder_hp_fuselages, cylinder_non_hp_fuselages) = CylindricalTank::init_fuselage_types();
        let mut unlocked_cylinder_hp_fuselages: Vec<&Fuselage> = Vec::with_capacity(unlocked_fuselages.len());
        let mut unlocked_cylinder_non_hp_fuselages: Vec<&Fuselage> = Vec::with_capacity(unlocked_fuselages.len());
        for unlocked_fuselage in unlocked_fuselages {
            unlocked_nosecone_hp_fuselages.push(nosecone_hp_fuselages.get(format!("HP {}", unlocked_fuselage).as_str()).expect(&format!("fuselage not found: '{}'", unlocked_fuselage)));
            unlocked_nosecone_non_hp_fuselages.push(nosecone_non_hp_fuselages.get(unlocked_fuselage).expect("fuselage not found"));
            unlocked_cylinder_hp_fuselages.push(cylinder_hp_fuselages.get(format!("HP {}", unlocked_fuselage).as_str()).expect("fuselage not found"));
            unlocked_cylinder_non_hp_fuselages.push(cylinder_non_hp_fuselages.get(unlocked_fuselage).expect("fuselage not found"));
            
            unlocked_nosecone_hp_fuselages.sort_by_key(|s| s.name);
            unlocked_nosecone_non_hp_fuselages.sort_by_key(|s| s.name);
            unlocked_cylinder_hp_fuselages.sort_by_key(|s| s.name);
            unlocked_cylinder_non_hp_fuselages.sort_by_key(|s| s.name);
        }
    }

    #[test]
    fn calculator_test() {
        let mut calculator = Calculator::new();
        calculator.init(0.141, 10.0, 1.05, false, false, false, Size::Sm, "Steel Fuselage".to_string());
        let (mut n_c_results, mut c_results, mut n_results) = calculator.calculate();
        let mut output: Vec<Rocket> = Vec::new();
        output.append(&mut n_c_results);
        output.append(&mut n_results);
        output.append(&mut c_results);
        assert_ne!(output.len(), 0);
    }
}