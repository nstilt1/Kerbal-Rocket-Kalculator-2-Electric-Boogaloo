use crate::{
    console_log, debug,
    modules::tanks::{
        cylindrical_tanks::compute_tank_height_for_delta_v,
        nose_tanks::compute_tank_height_with_nose_for_delta_v,
    },
    G,
};

use super::{
    engines::Engine,
    rocket_config::Rocket,
    size::Size,
    tanks::{
        cylindrical_tanks::{
            compute_tank_height, cylindrical_dry_mass, tank_height_given_max_volume, tank_volume,
            CylindricalTank,
        },
        nose_tanks::{
            calculate_corrected_volume, calculate_nose_dry_mass,
            compute_tank_height_with_nose_for_twr, NoseCone, NoseConeVariant,
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
    diameter: f64,
    target_dv: f64,
    mass: f64,
    minimum_twr: f64,
    maximum_twr: f64,
    needs_gimballing: bool,
    in_vacuum: bool,
    use_nosecone: bool,
    nose_height: f64,
    unlocked_fusalages: String,
    unlocked_tech: String,
}

impl Calculator {
    pub fn new() -> Self {
        Calculator {
            diameter: 0.01,
            target_dv: 0.0,
            mass: 0.0,
            minimum_twr: 0.0,
            maximum_twr: 100000.0,
            nose_height: 0.0,
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
        nose_height: f64,
        diameter: f64,
        unlocked_fuselages: String,
        unlocked_tech: String,
    ) {
        self.mass = mass * 1000.0;
        self.target_dv = target_dv;
        self.minimum_twr = minimum_twr;
        self.maximum_twr = maximum_twr;
        self.needs_gimballing = needs_gimballing;
        self.in_vacuum = in_vacuum;
        self.diameter = diameter;
        self.use_nosecone = use_nosecone;
        self.nose_height = nose_height;
        self.unlocked_fusalages = unlocked_fuselages;
        self.unlocked_tech = unlocked_tech;
    }

    pub fn change_target_delta_v(&mut self, dv: f64) {
        self.target_dv = dv;
    }

    pub fn change_mass(&mut self, mass: f64) {
        self.mass = mass * 1000.0;
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
    pub fn calculate(&self) -> Result<Vec<Rocket>, Error> {
        debug!(
            "Mass = {}\ntarget_dv = {}\nminimum twr = {}\nsize = {}",
            self.mass, self.target_dv, self.minimum_twr, self.diameter
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

        let mut only_cylinder_results: Vec<Rocket> = Vec::new();
        'engine_loop: for engine in engines.iter_mut() {
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
                if self.needs_gimballing && !engine.has_gimbal {
                    continue 'engine_loop;
                }
                if engine.is_solid {
                    continue 'engine_loop;
                }
                engine.diameter = self.diameter;

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

                    let max_volume_per_stack = engine.max_volume();
                    if self.use_nosecone {
                        'nosecone_core_loop: for nosecone_core in nosecone_cores {
                            let core_base_length_x_diameter =
                                nosecone_core.base_length * engine.diameter;
                            let min_nose_height =
                                core_base_length_x_diameter * NoseConeVariant::MIN_VSA;
                            let max_nose_height =
                                core_base_length_x_diameter * NoseConeVariant::MAX_VSA;

                            if self.nose_height < min_nose_height
                                || self.nose_height > max_nose_height
                            {
                                // different cores have different `base_length`s; because of this
                                // one type of nose could be picked every time if it somehow has
                                // better properties than the others, namely the correction
                                // coefficient in the nose volume calculation
                                continue 'nosecone_core_loop;
                            }

                            let h_twr_wet_dry = compute_tank_height_with_nose_for_delta_v(
                                self.target_dv,
                                engine,
                                &cyl_fuselage,
                                &nose_fuselage,
                                nosecone_core,
                                self.mass,
                                num_engines,
                                self.nose_height,
                                self.in_vacuum,
                            );

                            if h_twr_wet_dry.as_ref().is_err() {
                                // TODO: determine if this is the loop that needs to continue
                                continue 'nosecone_core_loop;
                            }

                            // we have enough delta-v, but do we have enough TWR?
                            let (h, twr, wet_mass, dry_mass) = h_twr_wet_dry.unwrap();
                            if twr < self.minimum_twr || twr > self.maximum_twr {
                                // TODO: determine if this should be nosecone_core_loop or num_engine_loop
                                continue 'nosecone_core_loop;
                            }
                            result.push(Rocket::new(
                                Some(NoseCone {
                                    core: *nosecone_core,
                                    length: self.nose_height,
                                    diameter: engine.diameter,
                                    fuselage: **nose_fuselage,
                                }),
                                Some(CylindricalTank {
                                    length: h,
                                    diameter: engine.diameter,
                                    fuselage: **cyl_fuselage,
                                }),
                                engine.clone(),
                                "".to_string(),
                                num_engines,
                                wet_mass,
                                dry_mass,
                                twr,
                            ));
                            continue 'nosecone_core_loop;
                        }
                    }
                    /* No nosecones!! */
                    let min_cyl_height = 0.1f64.max(self.diameter * CylindricalTank::MIN_VSA);
                    let max_cyl_height = CylindricalTank::MAX_VSA;
                    let mut cyl_height = min_cyl_height;

                    let h_twr_wet_dry = compute_tank_height_for_delta_v(
                        self.target_dv,
                        engine,
                        cyl_fuselage,
                        self.mass,
                        num_engines,
                        self.in_vacuum,
                    );

                    if h_twr_wet_dry.as_ref().is_err() {
                        // TODO: Determine if this is the right loop to continue
                        continue 'num_engine_loop;
                    }
                    let (h, twr, wet_mass, dry_mass) = h_twr_wet_dry.unwrap();

                    // we have enough delta-v, but do we heve enough TWR?
                    if twr < self.minimum_twr || twr > self.maximum_twr {
                        // TODO: determine if a different loop needs to be continued
                        // depending on which condition is true
                        continue 'num_engine_loop;
                    }

                    result.push(Rocket::new(
                        None,
                        Some(CylindricalTank {
                            length: h,
                            diameter: engine.diameter,
                            fuselage: **cyl_fuselage,
                        }),
                        engine.clone(),
                        "".to_string(),
                        num_engines,
                        wet_mass,
                        dry_mass,
                        twr,
                    ));
                    continue 'num_engine_loop;
                }
            }
        }
        return Ok(result);
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
        nose_height: f64,
    ) {
        self.mass = mass * 1000.0;
        self.in_vacuum = in_vacuum;
        self.use_nosecone = use_nosecone;
        self.minimum_twr = minimum_twr;
        self.unlocked_fusalages = unlocked_fuselages;
        self.unlocked_tech = unlocked_tech;
        self.nose_height = nose_height;
    }

    pub fn max_dv(
        &self,
        extra_fuel_percentage: f64,
        use_custom_diameter: bool,
        custom_diameter: f64,
    ) -> Result<Vec<Rocket>, Error> {
        console_log!("use_nosecone: {}", self.use_nosecone);
        let mut result: Vec<Rocket> = Vec::new();
        let mut engine_tech_map = Engine::init_all_engines(Engine::init_rp1_engines());
        let mut engines: Vec<Engine> = Vec::with_capacity(engine_tech_map.len() * 4);
        let unlocked_tech: Vec<&str> = self.unlocked_tech.split(',').collect();
        for tech in unlocked_tech.iter() {
            if let Some(vec) = engine_tech_map.get_mut(tech) {
                if use_custom_diameter {
                    vec.iter_mut()
                        .for_each(|engine| engine.diameter = custom_diameter);
                }
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
            let d = engine.diameter;
            if !engine.has_gimbal && self.needs_gimballing {
                continue 'engine_loop;
            }
            if engine.is_solid {
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

            let fuel = &engine.fuel_mix;
            let max_volume_per_stack = engine.max_volume() * (extra_fuel_percentage / 100.0 + 1.0);

            for (nose_fuselage, cyl_fuselage) in nose_fuselages.iter().zip(cyl_fuselages) {
                'num_engine_loop: for num_engines in 1..=MAX_ENGINES {
                    let partial_mass = (num_engines as f64 * engine.mass * 1000.0) + self.mass;
                    let engine_thrust = if self.in_vacuum {
                        engine.thrust_vac
                    } else {
                        engine.thrust_asl
                    } * 1000.0;
                    if engine_thrust * num_engines as f64 / self.mass / G < self.minimum_twr {
                        debug!("ERROR: engine_thrust * num_engines / self.mass / G was less than minimum TWR");
                        continue 'engine_loop;
                    }

                    if self.use_nosecone {
                        'nosecone_core_loop: for nosecone_core in nosecone_cores {
                            let core_base_length_x_diameter =
                                nosecone_core.base_length * engine.diameter;
                            let min_height = core_base_length_x_diameter * NoseConeVariant::MIN_VSA;
                            let max_height = core_base_length_x_diameter * NoseConeVariant::MAX_VSA;
                            if self.nose_height < min_height || self.nose_height > max_height {
                                continue 'nosecone_core_loop;
                            }

                            let h_twr_wet_dry = compute_tank_height_with_nose_for_twr(
                                self.minimum_twr,
                                engine,
                                &nose_fuselage,
                                nosecone_core,
                                self.nose_height,
                                &cyl_fuselage,
                                self.mass,
                                num_engines,
                                self.in_vacuum,
                            );

                            if h_twr_wet_dry.as_ref().is_err() {
                                // TODO: determine if this is the loop that needs to continue
                                continue 'nosecone_core_loop;
                            }

                            // we have the minimum TWR, but are we over the maximum volume of fuel
                            // given the engine's rated burn time * extra fuel?
                            let (h, twr, wet_mass, dry_mass) = h_twr_wet_dry.unwrap();
                            let tank_volume = tank_volume(engine.diameter, h);
                            let nose_volume = calculate_corrected_volume(
                                d,
                                self.nose_height,
                                nosecone_core.correction_coefficient,
                            );
                            let fuel_volume = cyl_fuselage.utilization * tank_volume
                                + nose_fuselage.utilization * nose_volume;
                            if fuel_volume > max_volume_per_stack * extra_fuel_percentage {
                                // volume exceeds the amount of fuel that can be burnt by this engine
                                // decrease height
                                let h_twr_wet_dry = tank_height_given_max_volume(
                                    max_volume_per_stack * extra_fuel_percentage,
                                    self.nose_height,
                                    &nose_fuselage,
                                    nosecone_core,
                                    &cyl_fuselage,
                                    engine,
                                    num_engines,
                                    self.mass,
                                    self.in_vacuum,
                                );
                                if h_twr_wet_dry.as_ref().is_err() {
                                    // TODO: determine if this is the right loop to continue
                                    continue 'nosecone_core_loop;
                                }
                                // TWR should be above the minimum twr now since we decreased the height
                                debug_assert!(twr > self.minimum_twr - 0.0001);
                                if twr > self.maximum_twr {
                                    continue 'nosecone_core_loop;
                                }
                                // TWR is within the range. This is the most delta-v
                                // that this number of engines combine with this
                                // specific nose can have
                                result.push(Rocket::new(
                                    Some(NoseCone {
                                        core: *nosecone_core,
                                        length: self.nose_height,
                                        diameter: d,
                                        fuselage: **nose_fuselage,
                                    }),
                                    Some(CylindricalTank {
                                        length: h,
                                        diameter: d,
                                        fuselage: **cyl_fuselage,
                                    }),
                                    engine.clone(),
                                    "".to_string(),
                                    num_engines,
                                    wet_mass,
                                    dry_mass,
                                    twr,
                                ));
                                continue 'nosecone_core_loop;
                            } else {
                                // fuel_volume is less than max. We could squeeze
                                // out some extra delta-v, but the TWR would be
                                // below the minimum TWR
                                result.push(Rocket::new(
                                    Some(NoseCone {
                                        core: *nosecone_core,
                                        length: self.nose_height,
                                        diameter: d,
                                        fuselage: **nose_fuselage,
                                    }),
                                    Some(CylindricalTank {
                                        length: h,
                                        diameter: d,
                                        fuselage: **cyl_fuselage,
                                    }),
                                    engine.clone(),
                                    "".to_string(),
                                    num_engines,
                                    wet_mass,
                                    dry_mass,
                                    twr,
                                ));
                                continue 'nosecone_core_loop;
                            }
                        }
                    } else {
                        // no nosecones!
                        let min_cyl_height = 0.1f64.max(engine.diameter * CylindricalTank::MIN_VSA);
                        let max_cyl_height = CylindricalTank::MAX_VSA;

                        let h_twr_wet_dry = compute_tank_height(
                            self.minimum_twr,
                            engine,
                            cyl_fuselage,
                            self.mass,
                            num_engines,
                            self.in_vacuum,
                        );
                        if h_twr_wet_dry.as_ref().is_err() {
                            debug!("Error: compute_tank_height was an error.");
                            continue 'num_engine_loop;
                        }

                        // we have the minimum TWR, but are we over the maximum volume of fuel
                        // given the engine's rated burn time * extra fuel?
                        let (h, twr, wet_mass, dry_mass, volume) = h_twr_wet_dry.unwrap();
                        console_log!(
                            "volume = {}\nmax_volume_per_stack = {}\nengine = {}",
                            volume,
                            max_volume_per_stack,
                            engine.name
                        );
                        #[cfg(test)]
                        if engine.name.eq("Aerobee") {
                            console_log!("\n\n\n\n");
                            console_log!(
                                "h = {}\nmax_volume_per_stack = {}",
                                h,
                                max_volume_per_stack
                            );
                            console_log!("v_tank = {}", volume);
                            console_log!("fuel density = {}", engine.fuel_density());
                            assert_eq!(tank_volume(d, h) * cyl_fuselage.utilization, volume);
                            assert_eq!(cyl_fuselage.utilization, 0.75);
                            //assert_eq!((0.893 + 1.64 + 78.1)*engine.rated_burn_time  * (extra_fuel_percentage / 100.0 + 1.0), max_volume_per_stack);
                        }
                        if volume > max_volume_per_stack {
                            console_log!("volume exceeded max volume\n");
                            let h_twr_wet_dry = tank_height_given_max_volume(
                                max_volume_per_stack,
                                0.0,
                                &nose_fuselage,
                                &nosecone_cores[0],
                                cyl_fuselage,
                                engine,
                                num_engines,
                                self.mass,
                                self.in_vacuum,
                            );
                            if h_twr_wet_dry.as_ref().is_err() {
                                debug!("ERROR: tank_height_given_max_volume resulted in an error");
                                continue 'num_engine_loop;
                            }
                            console_log!("Old h = {}\nOld twr = {}", h, twr);
                            let (h, twr, wet_mass, dry_mass) = h_twr_wet_dry.unwrap();
                            debug_assert!(twr > self.minimum_twr - 0.0001);
                            if twr > self.maximum_twr {
                                debug!("*not an error*: twr > self.maximum_twr");
                                continue 'num_engine_loop;
                            }
                            console_log!("New h = {}", h);
                            console_log!("New twr = {}", twr);
                            console_log!("New volume = {}", tank_volume(d, h));
                            // TWR is within the range. This is the most delta-v
                            // that this tank will be able to have
                            result.push(Rocket::new(
                                None,
                                Some(CylindricalTank {
                                    length: h,
                                    diameter: d,
                                    fuselage: **cyl_fuselage,
                                }),
                                engine.clone(),
                                "".to_string(),
                                num_engines,
                                wet_mass,
                                dry_mass,
                                twr,
                            ))
                        } else {
                            // fuel volume is less than max. We could squeeze out
                            // some more delta-v, but the TWR would be below the
                            // minimum TWR
                            result.push(Rocket::new(
                                None,
                                Some(CylindricalTank {
                                    length: h,
                                    diameter: d,
                                    fuselage: **cyl_fuselage,
                                }),
                                engine.clone(),
                                "".to_string(),
                                num_engines,
                                wet_mass,
                                dry_mass,
                                twr,
                            ))
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
    use crate::modules::{
        engines::ENGINES,
        tanks::{fuselage_names::STEEL_FUSELAGE_NAME, nose_tanks::NoseTankCore},
    };

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
            0.039,
            10.0,
            0.05,
            20000.0,
            false,
            false,
            false,
            0.0,
            0.3,
            "Steel Fuselage".to_string(),
            "start".to_string(),
        );
        let mut output = calculator.calculate().unwrap();
        output.sort_by(|x, y| x.mass.partial_cmp(&y.mass).unwrap());

        println!("Rocket: {}", output[0].to_string().replace('\n', "/n"));
        assert_ne!(output.len(), 0);
    }

    #[test]
    fn max_dv_test() {
        let mut calculator = Calculator::new();
        calculator.prepare_for_max_dv(
            0.039,
            false,
            1.0,
            false,
            false,
            "Steel Fuselage".to_string(),
            "start,Post-War Rocketry Testing".to_string(),
            0.0,
        );
        let results = calculator.max_dv(1.5, false, 0.01).unwrap();
        println!(
            "Results: {}",
            serde_json::to_string_pretty(&results[0]).unwrap()
        );
    }

    mod sanity_checks {
        use crate::modules::engines::ENGINES;

        use super::*;

        #[test]
        fn aerobee_1x_test() {
            let target_dv = 10.0;
            let engine = ENGINES.iter().find(|e| e.name == "Aerobee").unwrap();
            let fuselages = CylindricalTank::init_fuselage_types();
            let fuselage = fuselages.hp_fuselages.get("HP Steel Fuselage").unwrap();
            let payload_mass_kg = 0.087 * 1000.0;
            let num_tanks = 1;
            let in_vacuum = false;
            let (h, twr, wet, dry) = compute_tank_height_for_delta_v(
                target_dv,
                engine,
                fuselage,
                payload_mass_kg,
                num_tanks,
                in_vacuum,
            )
            .unwrap();
            assert!(h < 5.1);
            assert!(twr > 5.0);
            assert!(wet < 500.0);
            assert!(dry < wet);
        }
    }

    #[test]
    fn xasr_1_test() {
        let engines = Engine::init_all_engines(Engine::init_rp1_engines());
        let post_war = engines.get("Post-War Rocketry Testing").unwrap();
        let engine = post_war.iter().find(|e| e.name.contains("XASR-1")).unwrap();
        let max_volume = engine.max_volume();
        let fuselages = CylindricalTank::init_fuselage_types();
        let fuselage = fuselages.hp_fuselages.get("HP Steel Fuselage").unwrap();
        debug!("XASR-1 max volume = {}", max_volume);
        let (h, twr, wet, dry) = tank_height_given_max_volume(
            max_volume,
            0.0,
            &Fuselage::default(),
            &NoseTankCore::default(),
            fuselage,
            engine,
            1,
            37.0,
            false,
        )
        .unwrap();
        debug!("h = {}\ntwr = {}\nwet = {}\ndry = {}", h, twr, wet, dry);
        let r = Rocket::new(
            None,
            Some(CylindricalTank {
                length: h,
                diameter: 0.3,
                fuselage: fuselage.clone(),
            }),
            engine.clone(),
            "".to_string(),
            1,
            wet,
            dry,
            twr,
        );
        debug!("{}", serde_json::to_string_pretty(&r).unwrap())
    }

    macro_rules! engine_test {
        ($test_name:ident, $engine_name:literal, $tech_level:literal) => {
            #[test]
            fn $test_name() {
                let engines = Engine::init_all_engines(Engine::init_rp1_engines());
                let engines_at_tech_level = engines.get($tech_level).unwrap();
                let engine = engines_at_tech_level
                    .iter()
                    .find(|e| e.name == $engine_name)
                    .unwrap();
                let max_volume = engine.max_volume();
                let fuselages = CylindricalTank::init_fuselage_types();
                let fuselage = if engine.hp_fuel {
                    fuselages.hp_fuselages.get("HP Steel Fuselage").unwrap()
                } else {
                    fuselages.non_hp_fuselages.get("Steel Fuselage").unwrap()
                };
                debug!("{} max_volume = {}", $engine_name, max_volume);
                let (h, twr, wet, dry) = tank_height_given_max_volume(
                    max_volume,
                    0.0,
                    &Fuselage::default(),
                    &NoseTankCore::default(),
                    fuselage,
                    engine,
                    1,
                    37.0,
                    false,
                )
                .unwrap();
                debug!("h = {}\ntwr = {}\nwet = {}\ndry = {}", h, twr, wet, dry);
                let r = Rocket::new(
                    None,
                    Some(CylindricalTank {
                        length: h,
                        diameter: 0.3,
                        fuselage: fuselage.clone(),
                    }),
                    engine.clone(),
                    "".to_string(),
                    1,
                    wet,
                    dry,
                    twr,
                );
                debug!("{}", serde_json::to_string_pretty(&r).unwrap())
            }
        };
    }

    engine_test!(u_1250_test, "U-1250", "start");

    engine_test!(u_1700_test, "U-1700", "Post-War Rocketry Testing");

    engine_test!(u_2000_test, "U-2000", "Early Rocketry");

    engine_test!(veronique_test, "Veronique", "start");

    engine_test!(veronique_agi_test, "VeroniqueAGI", "Basic Rocketry");

    engine_test!(
        veronique_61_test,
        "Veronique61",
        "1956-1957 Orbital Rocketry"
    );

    engine_test!(a4_test, "A-4", "start");

    engine_test!(a9_test, "A-9", "Post-War Rocketry Testing");

    engine_test!(rd_100_test, "RD-100", "Post-War Rocketry Testing");

    engine_test!(rd_101_test, "RD-101", "Early Rocketry");

    engine_test!(xlr_10_test, "XLR10", "Post-War Rocketry Testing");

    engine_test!(xlr_11_test, "XLR11", "Post-War Rocketry Testing");

    engine_test!(xlr_41_test, "XLR41", "Post-War Rocketry Testing");

    engine_test!(orm_65_test, "ORM-65", "Post-War Rocketry Testing");

    engine_test!(rda_1_150_test, "RDA-1-150", "Post-War Rocketry Testing");

    engine_test!(rda_1_300_test, "RDA-1-300", "Early Rocketry");

    engine_test!(naa_75_110_a_test, "NAA-75-110 A-Series", "Early Rocketry");

    engine_test!(rd_200_test, "RD-200", "Early Rocketry");

    engine_test!(xlr_11_rm_5_test, "XLR-11-RM-5", "Early Rocketry");

    engine_test!(xlr_35_rm_1_test, "XLR-35-RM-1", "Early Rocketry");
}
