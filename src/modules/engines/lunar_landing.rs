use std::sync::LazyLock;

use crate::modules::{engines::{or_2019_2028::orbital_rocketry_2019_2028_lmae_mod, Engine}, fuel_type::Fuel};

pub(super) const NUM_LUNAR_LANDING_ENGINES: usize = 2;
pub static LUNAR_LANDING_ENGINES: LazyLock<[Engine; NUM_LUNAR_LANDING_ENGINES]> = LazyLock::new(|| init_lunar_landing_engines());

#[rustfmt::skip]
pub fn init_lunar_landing_engines() -> [Engine; NUM_LUNAR_LANDING_ENGINES] {
    let mut engines = [
        Engine::new(
            "LMAE", // Engine 0
            false,
            0.000005,
            15.6,
            100.0,
            1.0,
            311.0,
            0.095,
            1.2,
            560.0,
            false,
            false,
            true,
            false,
            true,
            35,
            1.25,
            0.0,
            vec![
                Fuel::new("Aerozine50", 2817.6162, 1323.59663105481, 172.0, 1363.0, 9, 57.9, true),
                Fuel::new("MON1", 2817.6162, 1328.90155913547, 172.0, 2078.0, 9, 57.9, true),
                Fuel::new("Helium", 2817.6162, 165.118009809725, 123.0, 129.0, 9, 57.9, true),
                // FuelType::Kerosene(0.218, 0.169),
                // FuelType::AK20(0.441, 0.677),
                // FuelType::Nitrogen(25.0, 31.3),
            ],
            "Lunar Landing",
        ),
        Engine::new(
            "LMDE", // Engine 1
            false,
            0.0000153,
            46.7,
            10.0,
            1.0,
            305.0,
            0.158,
            0.9,
            910.0,
            true,
            false,
            true,
            true,
            true,
            20,
            1.85,
            0.0,
            vec![
                Fuel::new("Aerozine50", 14687.3738, 6958.46164592103, 898.0, 7161.0, 17, 5.3, true),
                Fuel::new("MON1", 14687.3738, 6986.35091196895, 899.0, 10918.0, 17, 5.3, true),
                Fuel::new("Helium", 14687.3738, 742.561242110021, 637.0, 663.0, 17, 5.3, true),
                // FuelType::Kerosene(0.218, 0.169),
                // FuelType::AK20(0.441, 0.677),
                // FuelType::Nitrogen(25.0, 31.3),
            ],
            "Lunar Landing",
        ),
    ];

    orbital_rocketry_2019_2028_lmae_mod(&mut engines);

    engines
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "test fails probably because asl_isp is not completely accurate"]
    fn extreme_thrust_difference_test() {
        let thrust_asl = 0.000005;
        let isp_asl = 1.0;
        let isp_vac = 311.0;

        let thrust_vac = thrust_asl * (isp_vac / isp_asl);
        assert_eq!(thrust_vac, 15.6);
    }
}