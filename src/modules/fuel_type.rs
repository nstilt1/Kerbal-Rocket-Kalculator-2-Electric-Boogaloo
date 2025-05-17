//! A module for fuels and fuel mixes

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct FuelMix {
    fuels: &'static [FuelType]
}

impl FuelMix {
    /// Creates a new fuel mixture.
    pub const fn new(fuels: &'static [FuelType]) -> Self {
        Self {
            fuels
        }
    }

    /// Returns the density of this fuel mixture.
    pub fn density(&self) -> f64 {
        let total_flow_rate = self.flow_rate();
        if total_flow_rate == 0.0 {
            return 0.0; // Avoid division by zero
        }

        let mut weighted_density_sum = 0.0;
        for fuel in self.fuels {
            let flow_rate = fuel.flow_rate();
            let density = fuel.density();
            weighted_density_sum += flow_rate * density;
        }

        // Weighted average density
        weighted_density_sum / total_flow_rate
    }

    /// Returns the flow rate of this fuel mixture
    pub fn flow_rate(&self) -> f64 {
        let mut flow_rate_sum = 0.0;
        for fuel in self.fuels {
            flow_rate_sum += fuel.flow_rate();
        }
        flow_rate_sum
    }

    /// Returns the maximum volume of fuel that an engine can burn through in 
    /// its rated burn time. Unit = liters
    pub fn max_volume(&self, rated_burn_time: f64) -> f64 {
        self.flow_rate() * rated_burn_time
    }

    /// Returns the mass of this fuel mixture when filling the specified volume.
    pub fn mass(&self, volume: f64) -> f64 {
        self.density() * volume
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

#[derive(Debug, PartialEq, Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum FuelType {
    RP1(f64),
    PSPC,
    AnilineFurfuryl_22p(f64),
    IRFNA_III(f64),
    Nitrogen(f64),
    Kerosene(f64),
    AK20(f64),
    Water(f64),
    NGNC(f64),
    Ethanol_75(f64),
    Liquid_Oxygen(f64),
    HTP(f64),
}

impl FuelType {
    /// Returns the density of this fuel in kg/L
    pub fn density(&self) -> f64 {
        match self {
            Self::RP1(_) => 0.80655,
            //Self::PSPC => 1.73874, // Measured, actual 0.00174
            Self::PSPC => 1.74,
            Self::AnilineFurfuryl_22p(_) => 1.04410,
            //Self::IRFNA_III(_) => 1.56377, // Measured, actual 0.001658
            Self::IRFNA_III(_) => 1.658,
            //Self::Nitrogen(_) => 0.82310,
            Self::Nitrogen(_) => 0.824907,
            //Self::Kerosene(_) => 0.77531, // Measured, actual 0.00082
            Self::Kerosene(_) => 0.82,
            //Self::AK20(_) => 1.53390, // Measured, actual from CommonResources.cfg: 0.001499
            Self::AK20(_) => 1.499,
            //Self::Water(_) => 1.00229,
            Self::Water(_) => 1.0,
            //Self::NGNC(_) => 1.59941, // Measured, actual 0.0016
            Self::NGNC(_) => 1.6,
            //Self::Ethanol_75(_) => 0.84102, // Measured, actual: 0.00084175
            Self::Ethanol_75(_) => 0.84175,
            //Self::Liquid_Oxygen(_) => 1.13967, // Measured, actual 0.001141
            Self::Liquid_Oxygen(_) => 1.141,
            //Self::HTP(_) => 1.43236, // Measured, actual 0.001431
            Self::HTP(_) => 1.431,
        }
    }
    /// Returns the fuel flow rate for a specific engine in L/s
    pub fn flow_rate(&self) -> f64 {
        match self {
            Self::RP1(flow_rate) => *flow_rate,
            Self::AnilineFurfuryl_22p(flow_rate) => *flow_rate,
            Self::IRFNA_III(flow_rate) => *flow_rate,
            Self::PSPC => todo!(),
            Self::Nitrogen(flow_rate) => *flow_rate,
            Self::Kerosene(flow_rate) => *flow_rate,
            Self::AK20(flow_rate) => *flow_rate,
            Self::Water(flow_rate) => *flow_rate,
            Self::NGNC(flow_rate) => *flow_rate,
            Self::Ethanol_75(flow_rate) => *flow_rate,
            Self::Liquid_Oxygen(flow_rate) => *flow_rate,
            Self::HTP(flow_rate) => *flow_rate,
        }
    }

    /// Returns the maximum volume of fuel that an engine can burn through in 
    /// its rated burn time. Unit: Liters
    pub fn max_volume(&self, rated_burn_time: f64) -> f64 {
        self.flow_rate() * rated_burn_time
    }

    /// Returns the mass of this fuel given some volume. Unit: kg
    pub fn mass(&self, volume: f64) -> f64 {
        self.density() * volume
    }

    pub fn name(&self) -> &str {
        match self {
            Self::AK20(_) => "AK20",
            Self::AnilineFurfuryl_22p(_) => "AnilineFurfuryl_22p",
            Self::Ethanol_75(_) => "Ethanol 75",
            Self::HTP(_) => "HTP",
            Self::IRFNA_III(_) => "IRFNA_III",
            Self::Kerosene(_) => "Kerosene",
            Self::Liquid_Oxygen(_) => "Liquid Oxygen",
            Self::NGNC(_) => "NGNC",
            Self::Nitrogen(_) => "Nitrogen",
            Self::PSPC => "PSPC",
            Self::RP1(_) => "RP-1",
            Self::Water(_) => "Water",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
    }
}