//! A module for fuels and fuel mixes

use crate::modules::utils::burn_time_secs;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct FuelMixture {
    pub fuels: &'static [Fuel],
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
    pub const fn new(
        name: &'static str,
        total_volume: f64,
        fuel_volume: f64,
        tank_dry_mass: f64,
        tank_wet_mass: f64,
        burn_time_minutes: u32,
        burn_time_seconds: f64,
        is_hp: bool,
    ) -> Self {
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

#[cfg(test)]
mod tests {
    use crate::modules::engines::ENGINES;

    mod max_volume_tests {
        use super::*;

        macro_rules! impl_max_volume_test {
            ($(($name:ident, $engine:literal, $expected:literal, $error:literal)),*) => {
                $(
                    #[test]
                    #[ignore = "The max_volume function has errors"]
                    fn $name() {
                        let engine = ENGINES.iter().find(|e| e.name == $engine).unwrap();
                        let computed_volume = engine.max_volume();

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
        #[ignore = "changed nitrogen density"]
        fn nitrogen_compressed_density() {
            let engine = ENGINES.iter().find(|e| e.name == "Aerobee").unwrap();
            let expected_density = (92.2 - 56.0) / 144.7293;
            let nitrogen = engine.fuel_mix[2];
            let diff = nitrogen.density * 200.0 - expected_density;
            assert!(diff.abs() < 0.001);
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
