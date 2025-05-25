use crate::{debug, G};

use super::{
    engines::Engine,
    rocket_config::Rocket,
    size::Size,
    tanks::{
        cylindrical_tanks::{
            compute_tank_height, cylindrical_dry_mass, tank_volume, CylindricalTank,
        },
        nose_tanks::{
            calculate_corrected_volume, calculate_nose_dry_mass, NoseCone, NoseConeVariant,
        },
        Fuselage, Tanks,
    },
    Error,
};

const HEIGHT_DECREMENT_AMT: f64 = 0.05;
const HEIGHT_INCREMENT_AMT: f64 = 0.05;
const MAX_ENGINES: u8 = 9;

#[derive(Debug, Clone, PartialEq)]
pub struct Calculator {
    size: Size,
    target_dv: f64,
    mass: f64,
    minimum_twr: f64,
    maximum_twr: f64,
    needs_gimballing: bool,
    in_vacuum: bool,
    use_nosecone: bool,
    unlocked_fusalages: String,
    unlocked_tech: String,
}

impl Calculator {
    pub fn new() -> Self {
        Calculator {
            size: Size::Xs,
            target_dv: 0.0,
            mass: 0.0,
            minimum_twr: 0.0,
            maximum_twr: 100000.0,
            needs_gimballing: false,
            in_vacuum: false,
            use_nosecone: false,
            unlocked_fusalages: "Steel Fuselage".to_string(),
            unlocked_tech: String::new(),
        }
    }

    /// Set the calculator's variables
    pub fn init(
        &mut self,
        mass: f64,
        target_dv: f64,
        minimum_twr: f64,
        maximum_twr: f64,
        needs_gimballing: bool,
        in_vacuum: bool,
        use_nosecone: bool,
        size: Size,
        unlocked_fuselages: String,
        unlocked_tech: String,
    ) {
        self.mass = mass * 1000.0;
        self.target_dv = target_dv;
        self.minimum_twr = minimum_twr;
        self.maximum_twr = maximum_twr;
        self.needs_gimballing = needs_gimballing;
        self.in_vacuum = in_vacuum;
        self.size = size;
        self.use_nosecone = use_nosecone;
        self.unlocked_fusalages = unlocked_fuselages;
        self.unlocked_tech = unlocked_tech;
    }

    pub fn change_target_delta_v(&mut self, dv: f64) {
        self.target_dv = dv;
    }

    pub fn change_mass(&mut self, mass: f64) {
        self.mass = mass;
    }

    pub fn change_minimum_twr(&mut self, twr: f64) {
        self.minimum_twr = twr;
    }

    pub fn change_maximum_twr(&mut self, twr: f64) {
        self.maximum_twr = twr;
    }

    /// Calculates the parts required to build a rocket with specific arguments.
    ///
    /// Returns (nose+cylinder_results, cylinder_results, nose_results)
    pub fn calculate(&self) -> Result<(Vec<Rocket>, Vec<Rocket>, Vec<Rocket>), Error> {
        debug!(
            "Mass = {}\ntarget_dv = {}\nminimum twr = {}\nsize = {}",
            self.mass,
            self.target_dv,
            self.minimum_twr,
            self.size.get_diameter()
        );
        let mut result: Vec<Rocket> = Vec::new();

        let mut engine_tech_map = Engine::init_all_engines(Engine::init_rp1_engines());
        let mut engines: Vec<Engine> = Vec::with_capacity(engine_tech_map.len() * 4);
        let unlocked_tech: Vec<&str> = self.unlocked_tech.split(',').collect();
        for tech in unlocked_tech.iter() {
            if let Some(vec) = engine_tech_map.get_mut(tech) {
                engines.append(vec);
                continue;
            }
            return Err(Error::MissingTech(format!(
                "Tech '{}' was not found in the engine_tech_map",
                tech
            )));
        }

        let nosecones = NoseConeVariant::nosecones();
        let nosecone_cores = &nosecones.cores;
        let unlocked_fuselages: Vec<&str> = self.unlocked_fusalages.split(',').collect();
        let nose_fuselage_types = NoseConeVariant::init_fuselage_types();
        let (nosecone_hp_fuselages, nosecone_non_hp_fuselages) = (
            nose_fuselage_types.hp_fuselages,
            nose_fuselage_types.non_hp_fuselages,
        );

        let mut unlocked_nosecone_hp_fuselages: Vec<&Fuselage> =
            Vec::with_capacity(unlocked_fuselages.len());
        let mut unlocked_nosecone_non_hp_fuselages: Vec<&Fuselage> =
            Vec::with_capacity(unlocked_fuselages.len());

        let cyl_fuselage_types = CylindricalTank::init_fuselage_types();
        let (cylinder_hp_fuselages, cylinder_non_hp_fuselages) = (
            cyl_fuselage_types.hp_fuselages,
            cyl_fuselage_types.non_hp_fuselages,
        );
        let mut unlocked_cylinder_hp_fuselages: Vec<&Fuselage> =
            Vec::with_capacity(unlocked_fuselages.len());
        let mut unlocked_cylinder_non_hp_fuselages: Vec<&Fuselage> =
            Vec::with_capacity(unlocked_fuselages.len());
        for unlocked_fuselage in unlocked_fuselages {
            let hp_unlocked_fuselage = format!("HP {}", unlocked_fuselage);
            unlocked_nosecone_hp_fuselages.push(
                nosecone_hp_fuselages
                    .get(hp_unlocked_fuselage.as_str())
                    .expect(&format!("fuselage not found: '{}'", unlocked_fuselage)),
            );
            unlocked_nosecone_non_hp_fuselages.push(
                nosecone_non_hp_fuselages
                    .get(unlocked_fuselage)
                    .expect("fuselage not found"),
            );
            unlocked_cylinder_hp_fuselages.push(
                cylinder_hp_fuselages
                    .get(hp_unlocked_fuselage.as_str())
                    .expect("fuselage not found"),
            );
            unlocked_cylinder_non_hp_fuselages.push(
                cylinder_non_hp_fuselages
                    .get(unlocked_fuselage)
                    .expect("fuselage not found"),
            );
        }

        unlocked_nosecone_hp_fuselages.sort_by_key(|s| s.name);
        unlocked_nosecone_non_hp_fuselages.sort_by_key(|s| s.name);
        unlocked_cylinder_hp_fuselages.sort_by_key(|s| s.name);
        unlocked_cylinder_non_hp_fuselages.sort_by_key(|s| s.name);

        let mut only_nose_results: Vec<Rocket> = Vec::new();
        let mut only_cylinder_results: Vec<Rocket> = Vec::new();
        'engine_loop: for engine in engines.iter() {
            let (cyl_fuselages, nose_fuselages) = if engine.hp_fuel {
                (
                    &unlocked_cylinder_hp_fuselages,
                    &unlocked_nosecone_hp_fuselages,
                )
            } else {
                (
                    &unlocked_cylinder_non_hp_fuselages,
                    &unlocked_nosecone_non_hp_fuselages,
                )
            };

            // the ratio (wetMass/dryMass) required to reach the delta-v is
            // derived from:
            // deltaV = ln(wetMass/dryMass)*g*isp
            // deltaV/g/isp = ln(wetMass/dryMass)
            // wetMass/dryMass = e^(deltaV/g/engine.isp)
            // BUT... ln() might have better performance over e^(), but we only
            // calculate e^x once.
            let target_ratio = std::f64::consts::E.powf(
                self.target_dv
                    / G
                    / if self.in_vacuum {
                        engine.isp_vac
                    } else {
                        engine.isp_asl
                    },
            );

            for (nose_fuselage, cyl_fuselage) in nose_fuselages.iter().zip(cyl_fuselages) {
                if engine.size.ne(&self.size) {
                    continue 'engine_loop;
                }
                if self.needs_gimballing && !engine.has_gimbal {
                    continue 'engine_loop;
                }

                'num_engine_loop: for num_engines in 1..=MAX_ENGINES {
                    // if num_engines == 2 || num_engines == 6 || num_engines == 8 {
                    //     continue 'num_engine_loop;
                    // }
                    let mass_offset = 0.0;
                    // partial mass = offset + engines' mass + payload mass
                    let partial_mass =
                        mass_offset + (num_engines as f64 * engine.mass * 1000.0) + self.mass;
                    debug!(
                        "engine.mass = {}\npartial_mass = {}",
                        engine.mass * 1000.0,
                        partial_mass
                    );
                    let thrust = if self.in_vacuum {
                        engine.thrust_vac
                    } else {
                        engine.thrust_asl
                    } * num_engines as f64
                        * 1000.0;

                    let fuel = &engine.fuel_mix;

                    let max_volume_per_stack = fuel.max_volume(engine.rated_burn_time);
                    if self.use_nosecone {
                        for nosecone_core in nosecone_cores {
                            let min_height = nosecone_core.base_length * NoseConeVariant::MIN_VSA;
                            let max_height = nosecone_core.base_length * NoseConeVariant::MAX_VSA;
                            let mut height = min_height;
                            'nose_height_loop: while height <= max_height {
                                let nose_volume = calculate_corrected_volume(
                                    self.size.get_diameter(),
                                    height,
                                    nosecone_core.correction_coefficient,
                                ) * nose_fuselage.utilization;
                                if nose_volume > max_volume_per_stack {
                                    break;
                                }
                                let nose_dry_mass = calculate_nose_dry_mass(
                                    self.size.get_diameter(),
                                    height,
                                    nose_fuselage.density,
                                    nose_fuselage.utilization,
                                    nosecone_core.correction_coefficient,
                                ) * num_engines as f64;
                                let nose_wet_mass =
                                    nose_dry_mass + fuel.mass(nose_volume * num_engines as f64);

                                let wet_mass = nose_wet_mass + partial_mass;
                                let dry_mass = nose_dry_mass + partial_mass;

                                let twr = thrust / wet_mass / G;
                                if twr < self.minimum_twr {
                                    // TODO: maybe consider breaking here, but first check that the results are the same
                                    continue 'num_engine_loop;
                                }
                                if twr > self.maximum_twr {
                                    height += HEIGHT_INCREMENT_AMT;
                                    continue 'nose_height_loop;
                                }

                                // we have enough twr, but do we have enough delta-v?
                                if wet_mass / dry_mass >= target_ratio {
                                    only_nose_results.push(Rocket::new(
                                        Some(NoseCone {
                                            core: nosecone_core.clone(),
                                            length: height,
                                            diameter: self.size.get_diameter(),
                                            fuselage: **nose_fuselage,
                                        }),
                                        None,
                                        *engine,
                                        fuel.fuel_volumes(nose_volume),
                                        num_engines,
                                        wet_mass,
                                        dry_mass,
                                        twr,
                                    ));
                                    break;
                                }

                                // not enough delta v, let's add a cylindrical tank
                                let min_cyl_height =
                                    0.1f64.max(self.size.get_diameter() * CylindricalTank::MIN_VSA);
                                let max_cyl_height = CylindricalTank::MAX_VSA;
                                let mut cyl_height = min_cyl_height;
                                'cyl_height_loop: while cyl_height <= max_cyl_height {
                                    let cyl_volume =
                                        tank_volume(self.size.get_diameter(), cyl_height)
                                            * cyl_fuselage.utilization;
                                    if nose_volume + cyl_volume > max_volume_per_stack {
                                        break;
                                    }
                                    let cyl_dry_mass = cylindrical_dry_mass(
                                        self.size.get_diameter(),
                                        cyl_height,
                                        cyl_fuselage.utilization,
                                        cyl_fuselage.density,
                                    ) * num_engines as f64;
                                    let cyl_wet_mass =
                                        cyl_dry_mass + fuel.mass(cyl_volume * num_engines as f64);

                                    let dry_mass = nose_dry_mass + partial_mass + cyl_dry_mass;
                                    let wet_mass = nose_wet_mass + partial_mass + cyl_wet_mass;

                                    let twr = thrust / wet_mass / G;
                                    if twr < self.minimum_twr {
                                        // TODO: consider breaking here instead of continuing
                                        continue 'num_engine_loop;
                                    }
                                    if twr > self.maximum_twr {
                                        cyl_height += HEIGHT_INCREMENT_AMT;
                                        continue 'cyl_height_loop;
                                    }

                                    // we have enough twr, but do we have enough delta-v?
                                    if wet_mass / dry_mass >= target_ratio {
                                        result.push(Rocket::new(
                                            Some(NoseCone {
                                                core: nosecone_core.clone(),
                                                length: height,
                                                diameter: self.size.get_diameter(),
                                                fuselage: **nose_fuselage,
                                            }),
                                            Some(CylindricalTank {
                                                length: cyl_height,
                                                diameter: self.size.get_diameter(),
                                                fuselage: **cyl_fuselage,
                                            }),
                                            *engine,
                                            fuel.fuel_volumes(nose_volume + cyl_volume),
                                            num_engines,
                                            wet_mass,
                                            dry_mass,
                                            twr,
                                        ));
                                        break;
                                    }
                                    cyl_height += 0.05;
                                }

                                height += 0.05;
                            }
                        }
                    }
                    /* No nosecones!! */
                    let min_cyl_height =
                        0.1f64.max(self.size.get_diameter() * CylindricalTank::MIN_VSA);
                    let max_cyl_height = CylindricalTank::MAX_VSA;
                    let mut cyl_height = min_cyl_height;
                    'cyl_height_loop_2: while cyl_height <= max_cyl_height {
                        let cyl_volume = tank_volume(self.size.get_diameter(), cyl_height)
                            * cyl_fuselage.utilization;
                        if cyl_volume > max_volume_per_stack {
                            debug!(
                                "cyl_volume exceeded max volume with height = {} min_height = {}",
                                cyl_height, min_cyl_height
                            );
                            break;
                        }
                        let cyl_dry_mass = cylindrical_dry_mass(
                            self.size.get_diameter(),
                            cyl_height,
                            cyl_fuselage.utilization,
                            cyl_fuselage.density,
                        ) * num_engines as f64;
                        let cyl_wet_mass =
                            cyl_dry_mass + fuel.mass(cyl_volume * num_engines as f64);

                        let dry_mass = partial_mass + cyl_dry_mass;
                        let wet_mass = partial_mass + cyl_wet_mass;

                        let twr = thrust / wet_mass / G;
                        if twr < self.minimum_twr {
                            // TODO: consider breaking here instead of continuing
                            debug!("Calculated TWR ({}) was less than minimum TWR ({})\nthrust = {}kN, wet_mass = {}kg, G = {}m/s/s", twr, self.minimum_twr, thrust, wet_mass, G);
                            debug!(
                                "Engine: {}\nnum_engines: {}\ncyl_height: {}m",
                                engine.name, num_engines, cyl_height
                            );
                            debug!(
                                "cyl_diameter: {}m\ncyl_dry_mass: {}kg\ncyl_wet_mass: {}kg",
                                self.size.get_diameter(),
                                cyl_dry_mass,
                                cyl_wet_mass
                            );
                            debug!("partial_mass = {}kg\n", partial_mass);
                            continue 'num_engine_loop;
                        }
                        if twr > self.maximum_twr {
                            cyl_height += HEIGHT_INCREMENT_AMT;
                            continue 'cyl_height_loop_2;
                        }
                        debug!("TWR passed check = {}", twr);

                        // we have enough twr, but do we have enough delta-v?
                        if wet_mass / dry_mass >= target_ratio {
                            only_cylinder_results.push(Rocket::new(
                                None,
                                Some(CylindricalTank {
                                    length: cyl_height,
                                    diameter: self.size.get_diameter(),
                                    fuselage: **cyl_fuselage,
                                }),
                                *engine,
                                fuel.fuel_volumes(cyl_volume),
                                num_engines,
                                wet_mass,
                                dry_mass,
                                twr,
                            ));
                            break;
                        }
                        cyl_height += 0.05;
                    }
                }
            }
        }
        return Ok((result, only_cylinder_results, only_nose_results));
    }

    pub fn prepare_for_max_dv(
        &mut self,
        mass: f64,
        in_vacuum: bool,
        minimum_twr: f64,
        needs_gimballing: bool,
        use_nosecone: bool,
        unlocked_fuselages: String,
        unlocked_tech: String,
    ) {
        self.mass = mass;
        self.in_vacuum = in_vacuum;
        self.use_nosecone = use_nosecone;
        self.minimum_twr = minimum_twr;
        self.unlocked_fusalages = unlocked_fuselages;
        self.unlocked_tech = unlocked_tech;
    }

    pub fn max_dv(&self, extra_fuel_percentage: f64) -> Result<Vec<Rocket>, Error> {
        let mut result: Vec<Rocket> = Vec::new();
        let mut engine_tech_map = Engine::init_all_engines(Engine::init_rp1_engines());
        let mut engines: Vec<Engine> = Vec::with_capacity(engine_tech_map.len() * 4);
        let unlocked_tech: Vec<&str> = self.unlocked_tech.split(',').collect();
        for tech in unlocked_tech.iter() {
            if let Some(vec) = engine_tech_map.get_mut(tech) {
                engines.append(vec);
                continue;
            }
            return Err(Error::MissingTech(format!(
                "Tech '{}' was not found in the engine_tech_map",
                tech
            )));
        }

        let nosecones = NoseConeVariant::nosecones();
        let nosecone_cores = &nosecones.cores;
        let unlocked_fuselages: Vec<&str> = self.unlocked_fusalages.split(',').collect();

        let nose_fuselage_types = NoseConeVariant::init_fuselage_types();
        let (nosecone_hp_fuselages, nosecone_non_hp_fuselages) = (
            nose_fuselage_types.hp_fuselages,
            nose_fuselage_types.non_hp_fuselages,
        );
        let cyl_fuselage_types = CylindricalTank::init_fuselage_types();
        let (cylinder_hp_fuselages, cylinder_non_hp_fuselages) = (
            cyl_fuselage_types.hp_fuselages,
            cyl_fuselage_types.non_hp_fuselages,
        );
        let mut unlocked_nosecone_hp_fuselages: Vec<&Fuselage> =
            Vec::with_capacity(unlocked_fuselages.len());
        let mut unlocked_nosecone_non_hp_fuselages: Vec<&Fuselage> =
            Vec::with_capacity(unlocked_fuselages.len());
        let mut unlocked_cylinder_hp_fuselages: Vec<&Fuselage> =
            Vec::with_capacity(unlocked_fuselages.len());
        let mut unlocked_cylinder_non_hp_fuselages: Vec<&Fuselage> =
            Vec::with_capacity(unlocked_fuselages.len());
        for unlocked_fuselage in unlocked_fuselages {
            let hp_unlocked_fuselage = format!("HP {}", unlocked_fuselage);
            unlocked_nosecone_hp_fuselages.push(
                nosecone_hp_fuselages
                    .get(hp_unlocked_fuselage.as_str())
                    .expect(&format!("fuselage not found1: '{}'", hp_unlocked_fuselage)),
            );
            unlocked_nosecone_non_hp_fuselages.push(
                nosecone_non_hp_fuselages
                    .get(unlocked_fuselage)
                    .expect(&format!("fuselage not found2: '{}'", unlocked_fuselage)),
            );
            unlocked_cylinder_hp_fuselages.push(
                cylinder_hp_fuselages
                    .get(hp_unlocked_fuselage.as_str())
                    .expect(&format!("fuselage not found3: '{}'", hp_unlocked_fuselage)),
            );
            unlocked_cylinder_non_hp_fuselages.push(
                cylinder_non_hp_fuselages
                    .get(unlocked_fuselage)
                    .expect(&format!("fuselage not found4: '{}'", unlocked_fuselage)),
            );
        }
        unlocked_nosecone_hp_fuselages.sort_by_key(|s| s.name);
        unlocked_nosecone_non_hp_fuselages.sort_by_key(|s| s.name);
        unlocked_cylinder_hp_fuselages.sort_by_key(|s| s.name);
        unlocked_cylinder_non_hp_fuselages.sort_by_key(|s| s.name);

        'engine_loop: for engine in engines.iter() {
            if !engine.has_gimbal && self.needs_gimballing {
                continue 'engine_loop;
            }
            let (cyl_fuselages, nose_fuselages) = if engine.hp_fuel {
                (
                    &unlocked_cylinder_hp_fuselages,
                    &unlocked_nosecone_hp_fuselages,
                )
            } else {
                (
                    &unlocked_cylinder_non_hp_fuselages,
                    &unlocked_nosecone_non_hp_fuselages,
                )
            };

            for (nose_fuselage, cyl_fuselage) in nose_fuselages.iter().zip(cyl_fuselages) {
                'num_engine_loop: for num_engines in 1..=MAX_ENGINES {
                    let partial_mass = (num_engines as f64 * engine.mass * 1000.0) + self.mass;
                    let engine_thrust = if self.in_vacuum {
                        engine.thrust_vac
                    } else {
                        engine.thrust_asl
                    };
                    if engine_thrust * MAX_ENGINES as f64 / self.mass / G < self.minimum_twr {
                        continue 'engine_loop;
                    }
                    let thrust = engine_thrust * num_engines as f64 * 1000.0;

                    let fuel = &engine.fuel_mix;
                    let max_volume_per_stack = fuel.max_volume(engine.rated_burn_time)
                        * (extra_fuel_percentage / 100.0 + 1.0);

                    if self.use_nosecone {
                        for nosecone_core in nosecone_cores {
                            let min_height = nosecone_core.base_length * NoseConeVariant::MIN_VSA;
                            let max_height = nosecone_core.base_length * NoseConeVariant::MAX_VSA;
                            let mut height = max_height;
                            'nose_height_loop: while height >= min_height {
                                let nose_volume = calculate_corrected_volume(
                                    engine.size.get_diameter(),
                                    height,
                                    nosecone_core.correction_coefficient,
                                ) * nose_fuselage.utilization;
                                if nose_volume > max_volume_per_stack {
                                    height -= HEIGHT_DECREMENT_AMT;
                                    continue 'nose_height_loop;
                                }
                                let nose_dry_mass = calculate_nose_dry_mass(
                                    engine.size.get_diameter(),
                                    height,
                                    nose_fuselage.density,
                                    nose_fuselage.utilization,
                                    nosecone_core.correction_coefficient,
                                ) * num_engines as f64;
                                let nose_wet_mass =
                                    nose_dry_mass + fuel.mass(nose_volume * num_engines as f64);

                                let wet_mass = nose_wet_mass + partial_mass;
                                let dry_mass = nose_dry_mass + partial_mass;

                                let twr = thrust / wet_mass / G;

                                if twr < self.minimum_twr {
                                    height -= HEIGHT_DECREMENT_AMT;
                                    continue 'nose_height_loop;
                                }

                                // we have enough twr, but max volume is not yet reached
                                let min_cyl_height = 0.1f64
                                    .max(engine.size.get_diameter() * CylindricalTank::MIN_VSA);
                                let max_cyl_height = CylindricalTank::MAX_VSA;
                                let mut cyl_height = max_cyl_height;
                                'cyl_height_loop: while cyl_height >= min_cyl_height {
                                    let cyl_volume =
                                        tank_volume(engine.size.get_diameter(), cyl_height)
                                            * cyl_fuselage.utilization;
                                    if nose_volume + cyl_volume > max_volume_per_stack {
                                        cyl_height -= HEIGHT_DECREMENT_AMT;
                                        continue 'cyl_height_loop;
                                    }
                                    let cyl_dry_mass = cylindrical_dry_mass(
                                        engine.size.get_diameter(),
                                        cyl_height,
                                        cyl_fuselage.utilization,
                                        cyl_fuselage.density,
                                    ) * num_engines as f64;
                                    let cyl_wet_mass =
                                        cyl_dry_mass + fuel.mass(cyl_volume * num_engines as f64);

                                    let dry_mass = nose_dry_mass + partial_mass + cyl_dry_mass;
                                    let wet_mass = nose_wet_mass + partial_mass + cyl_wet_mass;

                                    let twr = thrust / wet_mass / G;
                                    if twr < self.minimum_twr {
                                        cyl_height -= HEIGHT_DECREMENT_AMT;
                                        continue 'cyl_height_loop;
                                    }

                                    // we have enough twr, add rocket to result
                                    result.push(Rocket::new(
                                        Some(NoseCone {
                                            core: nosecone_core.clone(),
                                            length: height,
                                            diameter: engine.size.get_diameter(),
                                            fuselage: **nose_fuselage,
                                        }),
                                        Some(CylindricalTank {
                                            length: cyl_height,
                                            diameter: engine.size.get_diameter(),
                                            fuselage: **cyl_fuselage,
                                        }),
                                        *engine,
                                        fuel.fuel_volumes(nose_volume + cyl_volume),
                                        num_engines,
                                        wet_mass,
                                        dry_mass,
                                        twr,
                                    ));
                                    height -= HEIGHT_DECREMENT_AMT;
                                    continue 'nose_height_loop;
                                }
                                height -= HEIGHT_DECREMENT_AMT;
                            }
                        }
                    } else {
                        // no nosecones!
                        let min_cyl_height =
                            0.1f64.max(engine.size.get_diameter() * CylindricalTank::MIN_VSA);
                        let max_cyl_height = CylindricalTank::MAX_VSA;
                        let mut cyl_height = max_cyl_height;
                        'cyl_height_loop_2: while cyl_height >= min_cyl_height {
                            let cyl_volume = tank_volume(engine.size.get_diameter(), cyl_height)
                                * cyl_fuselage.utilization;
                            if cyl_volume > max_volume_per_stack {
                                break;
                            }
                            let cyl_dry_mass = cylindrical_dry_mass(
                                engine.size.get_diameter(),
                                cyl_height,
                                cyl_fuselage.utilization,
                                cyl_fuselage.density,
                            ) * num_engines as f64;
                            let cyl_wet_mass =
                                cyl_dry_mass + fuel.mass(cyl_volume * num_engines as f64);
                            let dry_mass = partial_mass + cyl_dry_mass;
                            let wet_mass = partial_mass + cyl_wet_mass;

                            let twr = thrust / wet_mass / G;
                            if twr < self.minimum_twr {
                                cyl_height = if let Ok(v) = compute_tank_height(
                                    self.minimum_twr,
                                    &engine,
                                    &cyl_fuselage,
                                    self.mass,
                                    num_engines,
                                    self.in_vacuum,
                                ) {
                                    if cyl_height == v {
                                        // this should not happen if compute_tank_height returns a twr greater than or equal to minimum_twr
                                        break 'cyl_height_loop_2;
                                    }
                                    v
                                } else {
                                    // height was negative or infinity or NaN
                                    break 'cyl_height_loop_2;
                                };
                                continue 'cyl_height_loop_2;
                            }

                            result.push(Rocket::new(
                                None,
                                Some(CylindricalTank {
                                    length: cyl_height,
                                    diameter: engine.size.get_diameter(),
                                    fuselage: **cyl_fuselage,
                                }),
                                *engine,
                                fuel.fuel_volumes(cyl_volume),
                                num_engines,
                                wet_mass,
                                dry_mass,
                                twr,
                            ));
                            continue 'num_engine_loop;
                        }
                    }
                }
            }
        }
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fuselage_retrieval() {
        let unlocked_fuselages: Vec<&str> = "Steel Fuselage,Al Stringer Tank".split(',').collect();
        let nose_fuselage_types = NoseConeVariant::init_fuselage_types();
        let (nosecone_hp_fuselages, nosecone_non_hp_fuselages) = (
            nose_fuselage_types.hp_fuselages,
            nose_fuselage_types.non_hp_fuselages,
        );
        let cyl_fuselage_types = CylindricalTank::init_fuselage_types();
        let (cylinder_hp_fuselages, cylinder_non_hp_fuselages) = (
            cyl_fuselage_types.hp_fuselages,
            cyl_fuselage_types.non_hp_fuselages,
        );
        let keys = nosecone_hp_fuselages.keys();
        for k in keys {
            println!("{}", k);
        }
        let mut unlocked_nosecone_hp_fuselages: Vec<&Fuselage> =
            Vec::with_capacity(unlocked_fuselages.len());
        let mut unlocked_nosecone_non_hp_fuselages: Vec<&Fuselage> =
            Vec::with_capacity(unlocked_fuselages.len());
        let mut unlocked_cylinder_hp_fuselages: Vec<&Fuselage> =
            Vec::with_capacity(unlocked_fuselages.len());
        let mut unlocked_cylinder_non_hp_fuselages: Vec<&Fuselage> =
            Vec::with_capacity(unlocked_fuselages.len());
        for unlocked_fuselage in unlocked_fuselages {
            unlocked_nosecone_hp_fuselages.push(
                nosecone_hp_fuselages
                    .get(format!("HP {}", unlocked_fuselage).as_str())
                    .expect(&format!("fuselage not found: '{}'", unlocked_fuselage)),
            );
            unlocked_nosecone_non_hp_fuselages.push(
                nosecone_non_hp_fuselages
                    .get(unlocked_fuselage)
                    .expect("fuselage not found"),
            );
            unlocked_cylinder_hp_fuselages.push(
                cylinder_hp_fuselages
                    .get(format!("HP {}", unlocked_fuselage).as_str())
                    .expect("fuselage not found"),
            );
            unlocked_cylinder_non_hp_fuselages.push(
                cylinder_non_hp_fuselages
                    .get(unlocked_fuselage)
                    .expect("fuselage not found"),
            );

            unlocked_nosecone_hp_fuselages.sort_by_key(|s| s.name);
            unlocked_nosecone_non_hp_fuselages.sort_by_key(|s| s.name);
            unlocked_cylinder_hp_fuselages.sort_by_key(|s| s.name);
            unlocked_cylinder_non_hp_fuselages.sort_by_key(|s| s.name);
        }
    }

    #[test]
    fn calculator_test() {
        let mut calculator = Calculator::new();
        calculator.init(
            0.141,
            10.0,
            1.05,
            20000.0,
            false,
            false,
            false,
            Size::Sm,
            "Steel Fuselage".to_string(),
            "start".to_string(),
        );
        let (mut n_c_results, mut c_results, mut n_results) = calculator.calculate().unwrap();
        let mut output: Vec<Rocket> = Vec::new();
        output.append(&mut n_c_results);
        output.append(&mut n_results);
        output.append(&mut c_results);
        assert_ne!(output.len(), 0);
    }

    #[test]
    fn max_dv_test() {
        let mut calculator = Calculator::new();
        calculator.prepare_for_max_dv(
            0.5,
            false,
            1.02,
            false,
            false,
            "Steel Fuselage".to_string(),
            "start".to_string(),
        );
        let results = calculator.max_dv(1.5).unwrap();
        println!(
            "Results: {}",
            serde_json::to_string_pretty(&results[0]).unwrap()
        );
    }
}
