use super::{size::Size, fuel_type::{FuelType}};

#[derive(Debug, PartialEq, Clone)]
pub struct Engine {
    pub name: String,
    pub thrust_asl: f64,
    pub thrust_vac: f64,
    pub min_thrust: f64,
    pub isp_asl: f64,
    pub isp_vac: f64,
    pub mass: f64,
    pub has_gimbal: bool,
    pub size: Size,
    pub fuel_type: FuelType
}

impl Engine {
    fn new(name: &str, thrust_asl: f64, thrust_vac: f64, min_thrust: f64, isp_asl: f64, isp_vac: f64, mass: f64, has_gimbal: bool, size: Size, fuel_type: FuelType) -> Self {
        Engine {
            name: name.to_owned(),
            thrust_asl,
            thrust_vac,
            min_thrust,
            isp_asl,
            isp_vac,
            mass,
            has_gimbal,
            size,
            fuel_type
        }
    }

    pub fn init_engines() -> Vec<Engine> {
        //let mut result: Vec<Engine> = Vec::new();

        vec![
            // Size::Xs
            Engine::new("48-7S \"Spark\"", 17.1, 20.0, 0.0, 270.0, 315.0, 0.130, true, Size::Xs, FuelType::Methalox),
            Engine::new("LV-1 \"Ant\"", 0.5, 2.0, 0.0, 80.0, 330.0, 0.02, true, Size::Xs, FuelType::Methalox),
            Engine::new("IX-6315 \"Dawn\"", 0.0, 0.2, 0.0, 100.0, 4200.0, 0.15, false, Size::Xs, FuelType::Methalox),
            // Size::Sm
            Engine::new("CR-7 R.A.P.I.E.R. Closed Cycle", 162.3, 180.0, 0.0, 275.0, 305.0, 2.0, true, Size::Sm, FuelType::Methalox),
            Engine::new("LV-1000 \"Cornet\"", 6.6, 38.0, 0.0, 65.0, 375.0, 0.45, true, Size::Sm, FuelType::Methalox),
            Engine::new("LV-909 \"Terrier\"", 30.4, 60.0, 0.0, 170.0, 335.0, 0.5, true, Size::Sm, FuelType::Methalox),
            Engine::new("LV-T30 \"Reliant\"", 221.6, 260.0, 0.0, 260.0, 305.0, 1.25, false, Size::Sm, FuelType::Methalox),
            Engine::new("LV-T45 \"Swivel\"", 188.1, 215.0, 0.0, 280.0, 320.0, 1.4, true, Size::Sm, FuelType::Methalox),
            Engine::new("S3 KS-25 \"Vector\"", 769.0, 850.0, 0.0, 285.0, 315.0, 4.0, true, Size::Sm, FuelType::Methalox),
            Engine::new("T-1 \"Dart\"", 159.4, 170.0, 0.0, 300.0, 320.0, 1.0, false, Size::Sm, FuelType::Methalox),
            // Size::Md
            Engine::new("LV-2000 \"Trumpet\"", 31.4, 160.0, 0.0, 75.0, 382.0, 1.65, true, Size::Md, FuelType::Methalox),
            Engine::new("RE-I5 \"Skipper\"", 525.5, 600.0, 0.0, 282.0, 322.0, 3.0, true, Size::Md, FuelType::Methalox),
            Engine::new("RE-L10 \"Poodle\"", 110.7, 215.0, 0.0, 175.0, 340.0, 1.75, true, Size::Md, FuelType::Methalox),
            Engine::new("RE-M3 \"Mainsail\"", 1381.1, 1600.0, 0.0, 265.0, 307.0, 6.0, true, Size::Md, FuelType::Methalox),
            // Size::Lg
            Engine::new("KR-2XL \"Rhino\"", 1534.6, 1750.0, 0.0, 285.0, 325.0, 8.0, true, Size::Lg, FuelType::Methalox),
            Engine::new("LV-3000 \"Tuba\"", 119.2, 510.0, 0.0, 90.0, 385.0, 5.0, true, Size::Lg, FuelType::Methalox),
            Engine::new("S3 KS-100 \"Mammoth-II\"", 3701.6, 4250.0, 0.0, 270.0, 310.0, 15.0, true, Size::Lg, FuelType::Methalox),
            Engine::new("SC-TT \"Labradoodle\"", 341.1, 650.0, 0.0, 180.0, 343.0, 5.25, true, Size::Lg, FuelType::Methalox),


            // FuelType::Hydrogen
            // Size::Sm
            Engine::new("LV-N \"Nerv\"", 20.8, 75.0, 0.0, 250.0, 900.0, 3.0, false, Size::Sm, FuelType::Hydrogen),
            // Size::Lg
            Engine::new("LV-SW \"Swerv\"", 154.5, 700.0, 0.0, 320.0, 1450.0, 10.0, true, Size::Lg, FuelType::Hydrogen)
        ]

        //result
    }
}