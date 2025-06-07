//! A module for fuels and fuel mixes

use serde::Serialize;

use crate::{modules::utils::burn_time_secs, G};

//pub type FuelMix = FuelMixture;

#[derive(Debug, PartialEq, Clone, Copy, Serialize)]
pub struct FuelMix {
    pub fuels: &'static [FuelType],
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct FuelMixture {
    pub fuels: &'static [Fuel]
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Fuel {
    pub name: &'static str,
    pub density: f64,
    pub flow_rate_lps: f64,
    pub flow_rate_kgps: f64,
    pub volume_ratio: f64,
    pub is_hp: bool,
}

impl Fuel {
    pub const fn new(name: &'static str, total_volume: f64, fuel_volume: f64, tank_dry_mass: f64, tank_wet_mass: f64, burn_time_minutes: u32, burn_time_seconds: f64, is_hp: bool) -> Self {
        let burn_time = burn_time_secs(burn_time_minutes, burn_time_seconds);
        Self {
            name,
            density: (tank_wet_mass - tank_dry_mass) / fuel_volume,
            flow_rate_kgps: (tank_wet_mass - tank_dry_mass) / burn_time,
            flow_rate_lps: fuel_volume / burn_time,
            volume_ratio: fuel_volume / total_volume,
            is_hp,
        }
    }
}

impl FuelMix {
    /// Creates a new fuel mixture.
    pub const fn new(fuels: &'static [FuelType]) -> Self {
        Self { fuels }
    }

    /// Returns the density of this fuel mixture in kg/L
    pub fn density(&self, hp_fuel: bool) -> f64 {
        let total_flow_rate = self.flow_rate_lps(); // lps
        if total_flow_rate == 0.0 {
            return 0.0; // Avoid division by zero
        }

        let mut weighted_density_sum = 0.0;
        let mut total_corrected_volume = 0.0;

        for fuel in self.fuels {
            let flow_rate = fuel.flow_rate_lps();
            let mut density = fuel.density();
            let mut fuel_volume = flow_rate / density;

            if fuel.is_gas() && hp_fuel {
                //let uncompressed_volume = 97.193;
                //let compressed_volume = 19438.6;
                //let compression_ratio = compressed_volume / uncompressed_volume;
                let compression_ratio = 1.0 / 200.0;
                fuel_volume *= compression_ratio;
            }
            weighted_density_sum += flow_rate * density;
            total_corrected_volume += fuel_volume;
        }

        // Weighted average density
        weighted_density_sum / total_corrected_volume
    }

    /// Returns the flow rate of this fuel mixture in kg/s
    pub fn flow_rate(&self) -> f64 {
        let mut flow_rate_sum = 0.0;
        for fuel in self.fuels {
            flow_rate_sum += fuel.flow_rate();
        }
        flow_rate_sum
    }

    /// Returns the flow rate of this fuel mixture in L/s
    pub fn flow_rate_lps(&self) -> f64 {
        let mut flow_rate_sum = 0.0;
        for fuel in self.fuels {
            flow_rate_sum += fuel.flow_rate_lps();
        }
        flow_rate_sum
    }

    /// Returns the maximum volume of fuel that an engine can burn through in
    /// its rated burn time. Unit = liters
    pub fn max_volume(&self, rated_burn_time: f64, hp_fuel: bool) -> f64 {
        if self.fuels.is_empty() {
            return 0.0;
        }

        let compression_ratio = 200.0;

        let mut total_volume_liters = 0.0;

        for fuel in self.fuels {
            let mass_flow_i = fuel.flow_rate();
            let mass_i = mass_flow_i * rated_burn_time;

            let mut density_i = fuel.density();
            if fuel.is_gas() && hp_fuel {
                density_i *= compression_ratio;
            }

            if density_i <= 0.0 {
                continue;
            }

            let volume_i = mass_i / density_i;
            total_volume_liters += volume_i;
        }
        total_volume_liters
    }

    /// Returns the mass of this fuel mixture when filling the specified volume.
    pub fn mass(&self, volume: f64, hp_fuel: bool) -> f64 {
        self.density(hp_fuel) * volume
    }

    /// Returns the fuel volumes for this mixture to fill up a volume.
    pub fn fuel_volumes(&self, volume: f64) -> String {
        let mut output = "Fuels:\n".to_string();
        let total_flow_rate = self.flow_rate();
        if total_flow_rate <= 0.0 {
            return "Fuel flow rates summed up to 0.0".to_string();
        }
        for fuel in self.fuels {
            let flow_rate = fuel.flow_rate();
            let percentage = flow_rate / total_flow_rate;
            let fuel_volume = percentage * volume;
            output.push_str(&format!("{}: {:.4} L", fuel.name(), fuel_volume));
        }
        output
    }
}

#[derive(Debug, PartialEq, Clone, Copy, Serialize)]
#[allow(non_camel_case_types)]
pub enum FuelType {
    RP1(f64, f64),
    PSPC,
    AnilineFurfuryl_22p(f64, f64),
    AnilineFurfuryl_22p_v2(f64, f64),
    AnilineFurfuryl_37p(f64, f64),
    IRFNA_III(f64, f64),
    IRFNA_III_v2(f64, f64),
    Nitrogen(f64, f64),
    Kerosene(f64, f64),
    AK20(f64, f64),
    Water(f64, f64),
    NGNC(f64),
    Ethanol_75(f64, f64),
    Ethanol_90(f64, f64),
    Liquid_Oxygen(f64, f64),
    HTP(f64, f64),
    Hydyne(f64, f64),
    Helium(f64, f64), // flow_rate_Lps, flow_rate_kgps
    Turpentine(f64, f64),
    IWFNA(f64, f64),
}

impl FuelType {
    /// Returns the density of this fuel in kg/L
    pub fn density(&self) -> f64 {
        match self {
            Self::RP1(lps, kgps) => kgps / lps,
            //Self::PSPC => 1.73874, // Measured, actual 0.00174
            Self::PSPC => 1.74,
            Self::AnilineFurfuryl_22p(lps, kgps) => 1.042,
            //Self::AnilineFurfuryl_37p(lps, kgps) => kgps / lps,
            Self::AnilineFurfuryl_22p_v2(volume_liters, mass_kg) => volume_liters / mass_kg,
            Self::AnilineFurfuryl_37p(_, _) => 1.0585,
            //Self::IRFNA_III(_) => 1.56377, // Measured, actual 0.001658
            //Self::IRFNA_III(lps, kgps) => 1.658,
            //Self::IRFNA_III(lps, kgps) => kgps / lps,
            Self::IRFNA_III(_, _) => 1.564,
            Self::IRFNA_III_v2(volume_liters, mass_kg) => volume_liters / mass_kg,
            //Self::Nitrogen(_) => 0.82310,
            Self::Nitrogen(lps, kgps) => 0.00082,
            //Self::Kerosene(_) => 0.77531, // Measured, actual 0.00082
            Self::Kerosene(lps, kgps) => kgps / lps,
            //Self::AK20(_) => 1.53390, // Measured, actual from CommonResources.cfg: 0.001499
            Self::AK20(lps, kgps) => kgps / lps,
            //Self::Water(_) => 1.00229,
            Self::Water(lps, kgps) => kgps / lps,
            //Self::NGNC(_) => 1.59941, // Measured, actual 0.0016
            Self::NGNC(lps) => *lps,
            //Self::Ethanol_75(_) => 0.84102, // Measured, actual: 0.00084175
            Self::Ethanol_75(lps, kgps) => kgps / lps,
            Self::Ethanol_90(lps, kgps) => kgps / lps,
            //Self::Liquid_Oxygen(_) => 1.13967, // Measured, actual 0.001141
            Self::Liquid_Oxygen(lps, kgps) => kgps / lps,
            //Self::HTP(_) => 1.43236, // Measured, actual 0.001431
            Self::HTP(lps, kgps) => kgps / lps,
            Self::Hydyne(lps, kgps) => kgps / lps,
            Self::Helium(lps, kgps) => kgps / lps,
            Self::Turpentine(lps, kgps) => kgps / lps,
            Self::IWFNA(lps, kgps) => kgps / lps,
        }
    }
    /// Returns the fuel flow rate for a specific engine in kg/s
    pub fn flow_rate(&self) -> f64 {
        match self {
            Self::RP1(_, kgps) => *kgps,
            Self::AnilineFurfuryl_22p(_lps, kgps) => *kgps,
            Self::AnilineFurfuryl_22p_v2(volume, mass) => todo!(),
            Self::AnilineFurfuryl_37p(_, kgps) => *kgps,
            Self::IRFNA_III(_lps, kgps) => *kgps,
            Self::IRFNA_III_v2(_, _) => todo!(),
            Self::PSPC => todo!(),
            Self::Nitrogen(_lps, kgps) => *kgps,
            Self::Kerosene(_, kgps) => *kgps,
            Self::AK20(_, kgps) => *kgps,
            Self::Water(_, kgps) => *kgps,
            Self::NGNC(flow_rate) => *flow_rate,
            Self::Ethanol_75(_, kgps) => *kgps,
            Self::Ethanol_90(_, kgps) => *kgps,
            Self::Liquid_Oxygen(_, kgps) => *kgps,
            Self::HTP(_, kgps) => *kgps,
            Self::Hydyne(_, kgps) => *kgps,
            Self::Helium(_, kgps) => *kgps,
            Self::Turpentine(_, kgps) => *kgps,
            Self::IWFNA(_, kgps) => *kgps,
        }
    }

    pub fn flow_rate_lps(&self) -> f64 {
        match self {
            Self::RP1(lps, _kgps) => *lps,
            Self::AnilineFurfuryl_22p(lps, _kgps) => *lps,
            Self::AnilineFurfuryl_22p_v2(_, _) => todo!(),
            Self::AnilineFurfuryl_37p(lps, _kgps) => *lps,
            Self::IRFNA_III(lps, _kgps) => *lps,
            Self::IRFNA_III_v2(_, _) => todo!(),
            Self::PSPC => todo!(),
            Self::Nitrogen(lps, _kgps) => *lps,
            Self::Kerosene(lps, _kgps) => *lps,
            Self::AK20(lps, _kgps) => *lps,
            Self::Water(lps, _kgps) => *lps,
            Self::NGNC(flow_rate) => *flow_rate,
            Self::Ethanol_75(lps, _kgps) => *lps,
            Self::Ethanol_90(lps, _kgps) => *lps,
            Self::Liquid_Oxygen(lps, _kgps) => *lps,
            Self::HTP(lps, _kgps) => *lps,
            Self::Hydyne(lps, _kgps) => *lps,
            Self::Helium(lps, _kgps) => *lps,
            Self::Turpentine(lps, _kgps) => *lps,
            Self::IWFNA(lps, _kgps) => *lps,
        }
    }

    /// Returns the maximum volume of fuel that an engine can burn through in
    /// its rated burn time. Unit: Liters
    pub fn max_volume(&self, rated_burn_time: f64) -> f64 {
        self.flow_rate_lps() * rated_burn_time
    }

    /// Returns the mass of this fuel given some volume. Unit: kg
    pub fn mass(&self, volume: f64) -> f64 {
        self.density() * volume
    }

    pub fn name(&self) -> &str {
        match self {
            Self::AK20(_, _) => "AK20",
            Self::AnilineFurfuryl_22p(_, _) => "Aniline-Furfuryl 22%",
            Self::AnilineFurfuryl_22p_v2(_, _) => "Aniline-Furfuryl 22%",
            Self::AnilineFurfuryl_37p(_, _) => "Aniline-Furfuryl 37%",
            Self::Ethanol_75(_, _) => "Ethanol 75",
            Self::Ethanol_90(_, _) => "Ethanol 90",
            Self::HTP(_, _) => "HTP",
            Self::IRFNA_III(_, _) => "IRFNA_III",
            Self::IRFNA_III_v2(_, _) => "IRFNA III",
            Self::Kerosene(_, _) => "Kerosene",
            Self::Liquid_Oxygen(_, _) => "Liquid Oxygen",
            Self::NGNC(_) => "NGNC",
            Self::Nitrogen(_, _) => "Nitrogen",
            Self::PSPC => "PSPC",
            Self::RP1(_, _) => "RP-1",
            Self::Water(_, _) => "Water",
            Self::Hydyne(_, _) => "Hydyne",
            Self::Helium(_, _) => "Helium",
            Self::Turpentine(_, _) => "Turpentine",
            Self::IWFNA(_, _) => "IWFNA",
        }
    }

    pub fn is_gas(&self) -> bool {
        match self {
            Self::AK20(_, _) => false,
            Self::AnilineFurfuryl_22p(_, _) => false,
            Self::AnilineFurfuryl_22p_v2(_, _) => false,
            Self::AnilineFurfuryl_37p(_, _) => false,
            Self::Ethanol_75(_, _) => false,
            Self::Ethanol_90(_, _) => false,
            Self::HTP(_, _) => false,
            Self::IRFNA_III(_, _) => false,
            Self::IRFNA_III_v2(_, _) => false,
            Self::Kerosene(_, _) => false,
            Self::Liquid_Oxygen(_, _) => false,
            Self::NGNC(_) => false,
            Self::Nitrogen(_, _) => true,
            Self::PSPC => false,
            Self::RP1(_, _) => false,
            Self::Water(_, _) => false,
            Self::Hydyne(_, _) => false,
            Self::Helium(_, _) => true,
            Self::Turpentine(_, _) => false,
            Self::IWFNA(_, _) => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::modules::engines::ENGINES;

    use super::*;

    mod max_volume_tests {
        use super::*;

        macro_rules! impl_max_volume_test {
            ($(($name:ident, $engine:literal, $expected:literal, $error:literal)),*) => {
                $(
                    #[test]
                    #[ignore = "The max_volume function has errors"]
                    fn $name() {
                        let engine = ENGINES.iter().find(|e| e.name == $engine).unwrap();
                        let computed_volume = engine.fuel_mix.max_volume(engine.rated_burn_time, engine.hp_fuel);

                        let diff = computed_volume - $expected;
                        assert!(
                            diff.abs() < $error,
                            "\nError: {} max volume should be {} but was found to be {}\n",
                            $engine,
                            $expected,
                            computed_volume
                        );
                    }
                )*
            };
        }

        impl_max_volume_test!(
            (aerobee_max_volume, "Aerobee", 144.7293, 0.0001),
            (veronique_max_volume, "Veronique", 742.8906, 0.0001)
        );
    }

    mod sanity_checks {
        use super::*;

        #[test]
        #[ignore = "changed density function for hp fuels"]
        fn single_liquid() {
            let mix = FuelMix::new(&[FuelType::Ethanol_90(1.0, 2.0)]);

            assert_eq!(mix.density(false), 2.0);
            assert_eq!(mix.density(true), 2.0);
        }

        #[test]
        #[ignore = "changed density function for hp fuels"]
        fn single_gas() {
            let mix = FuelMix::new(&[FuelType::Helium(1.0, 2.0)]);
            assert_eq!(mix.density(false), 2.0);
            assert_eq!(mix.density(true), 400.0);
        }

        #[test]
        #[ignore = "changed nitrogen density"]
        fn nitrogen_compressed_density() {
            let engine = ENGINES.iter().find(|e| e.name == "Aerobee").unwrap();
            let expected_density = (92.2 - 56.0) / 144.7293;
            let nitrogen = engine.fuel_mix.fuels[2];
            let diff = nitrogen.density() * 200.0 - expected_density;
            assert!(diff.abs() < 0.001);
        }

        #[test]
        #[ignore = "changed nitrogen density"]
        fn density_sanity_check() {
            let engine = ENGINES.iter().find(|e| e.name == "Aerobee").unwrap();

            let flow_rate_kgps = engine.fuel_mix.flow_rate();
            let flow_rate_lps = engine.fuel_mix.flow_rate_lps();
            let expected_density = flow_rate_kgps / flow_rate_lps;
            let actual_density = engine.fuel_mix.density(false);

            assert_eq!(expected_density, actual_density);
        }
    }

    /// Calculates the density of a fuel.
    ///
    /// Arguments:
    /// * `wet_mass` - the wet mass of the tank in kg
    /// * `dry_mass` - the dry mass of the tank in kg
    /// * `volume` - the volume of the tank in L
    fn calculate_density(name: &str, wet_mass: f64, dry_mass: f64, volume: f64) {
        let fuel_mass = wet_mass - dry_mass;
        println!("\n{} density = {:.5}", name, fuel_mass / volume);
    }

    fn calculate_density_2(name: &str, flow_rate_liters_per_sec: f64, flow_rate_mass_per_sec: f64) {
        println!(
            "\n{} density = {:.5}",
            name,
            flow_rate_mass_per_sec / flow_rate_liters_per_sec
        );
    }

    #[test]
    fn get_densities() {
        calculate_density("RP-1", 996.0, 149.0, 1050.1546);
        calculate_density("PSPC", 1.93, 0.0, 1.11);
        calculate_density("AnilineFurfuryl_22p", 19900.0, 2420.0, 16741.6446);
        calculate_density("IRFNA-III", 28600.0, 2420.0, 16741.6446);
        calculate_density("Liquid Nitrogen", 16200.0, 2420.0, 16741.6446);
        calculate_density("Kerosene", 15400.0, 2420.0, 16741.6446);
        calculate_density("AK20", 28100.0, 2420.0, 16741.6446);
        calculate_density("Water", 19200.0, 2420.0, 16741.6446);
        calculate_density("NGNC", 66.2, 0.0, 41.3903);
        calculate_density("Ethanol_75", 16500.0, 2420.0, 16741.6446);
        calculate_density("Liquid Oxygen", 21500.0, 2420.0, 16741.6446);
        calculate_density("HTP", 26400.0, 2420.0, 16741.6446);
        calculate_density("AnilineFurfuryl_37p", 588.0, 70.6, 488.9104);
        calculate_density("Ethanol_90", 467.0, 70.6, 488.9104);
        //calculate_density("Hydyne", 491.0, 70.6, 488.9104);
        calculate_density_2("Hydyne", 57.7, 49.6);
    }
}
