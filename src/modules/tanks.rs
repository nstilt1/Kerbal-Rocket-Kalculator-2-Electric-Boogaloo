use super::{size::Size, fuel_type::FuelType};

#[derive(Debug, PartialEq, Clone)]
pub struct Tank {
    pub name: String,
    pub wet_mass: f64,
    pub dry_mass: f64,
    pub size: Size,
    pub cost: i64,
    pub fuel_type: FuelType
}

#[derive(Debug, PartialEq, Clone)]
pub struct FuelStack {
    pub tanks: Vec<Tank>,
    pub size: Size,
    pub fuel_type: FuelType
}
impl FuelStack {
    pub fn new(tanks: Vec<Tank>) -> Self {
        let t = tanks[0].to_owned();
        let size = t.size;
        FuelStack {tanks, size, fuel_type: t.fuel_type}
    }

    pub fn multiple(tank: &Tank, count: u8) -> Self {
        let mut tanks: Vec<Tank> = Vec::new();
        for _x in 0..count {
            tanks.push(tank.clone());
        }
        FuelStack::new(tanks)
    }
}

impl Tank {
    fn new(name: &str, wet_mass: f64, dry_mass: f64, size: Size, cost: i64, fuel_type: FuelType) -> Self {
        return Tank {
            name: name.to_owned(),
            wet_mass,
            dry_mass,
            size,
            cost,
            fuel_type
        };
    }
    
    /// Define the tank configurations we will consider in our calculations. It seems very difficult to make this a const
    pub fn init_tanks() -> Vec<FuelStack> {
        let mut tanks: Vec<Tank> = Vec::new();

        tanks.push(Tank::new("FL-T100", 0.56, 0.06, Size::Sm, 0, FuelType::Methalox));
        tanks.push(Tank::new("FL-T200", 1.13, 0.13, Size::Sm, 0, FuelType::Methalox));
        tanks.push(Tank::new("FL-T400", 2.25, 0.25, Size::Sm, 0, FuelType::Methalox));
        tanks.push(Tank::new("FL-T800", 4.5, 0.5, Size::Sm, 0, FuelType::Methalox));

        tanks.push(Tank::new("X200-8", 4.5, 0.5, Size::Md, 0, FuelType::Methalox));
        tanks.push(Tank::new("X200-16", 9.0, 1.0, Size::Md, 0, FuelType::Methalox));
        tanks.push(Tank::new("X200-32", 18.0, 2.0, Size::Md, 0, FuelType::Methalox));
        tanks.push(Tank::new("X200-64", 36.0, 4.0, Size::Md, 0, FuelType::Methalox));

        tanks.push(Tank::new("S3-3600", 20.25, 2.25, Size::Lg, 0, FuelType::Methalox));
        tanks.push(Tank::new("S3-7200", 40.5, 4.5, Size::Lg, 0, FuelType::Methalox));
        tanks.push(Tank::new("S3-14400", 81.0, 9.0, Size::Lg, 0, FuelType::Methalox));
        tanks.push(Tank::new("S3-28800", 162.0, 18.0, Size::Lg, 0, FuelType::Methalox));

        tanks.push(Tank::new("S4-6400", 36.0, 4.0, Size::Xl, 0, FuelType::Methalox));
        tanks.push(Tank::new("S4-12800", 72.0, 8.0, Size::Xl, 0, FuelType::Methalox));
        tanks.push(Tank::new("S4-25600", 144.0, 16.0, Size::Xl, 0, FuelType::Methalox));
        tanks.push(Tank::new("S4-53200", 288.0, 32.0, Size::Xl, 0, FuelType::Methalox));


        // get configs
        let mut result: Vec<FuelStack> = Vec::new();


        // Add 16 oscars for max amount
        let oscar = Tank::new("Oscar-B", 0.1, 0.01, Size::Xs, 0, FuelType::Methalox);

        for i in 1..=16 {
            result.push(FuelStack::multiple(&oscar, i));
        }
        
        // Add all of the different combinations of tanks
        for i in 0..tanks.len()/4 {
            let size = i *4;
            result.push(FuelStack::new(vec![tanks[size].to_owned()]));
            result.push(FuelStack::new(vec![tanks[size + 1].to_owned()]));
            result.push(FuelStack::new(vec![tanks[size].to_owned(), tanks[size + 1].to_owned()]));
            result.push(FuelStack::new(vec![tanks[size + 2].to_owned()]));
            result.push(FuelStack::new(vec![tanks[size].to_owned(), tanks[size + 2].to_owned()]));
            result.push(FuelStack::new(vec![tanks[size + 1].to_owned(), tanks[size + 2].to_owned()]));
            result.push(FuelStack::new(vec![tanks[size].to_owned(), tanks[size + 1].to_owned(), tanks[size + 2].to_owned()]));
            result.push(FuelStack::new(vec![tanks[size + 3].to_owned()]));
            result.push(FuelStack::new(vec![tanks[size].to_owned(), tanks[size + 3].to_owned()]));
            result.push(FuelStack::new(vec![tanks[size + 1].to_owned(), tanks[size + 3].to_owned()]));
            result.push(FuelStack::new(vec![tanks[size].to_owned(), tanks[size + 1].to_owned(), tanks[size + 3].to_owned()]));
            result.push(FuelStack::new(vec![tanks[size + 2].to_owned(), tanks[size + 3].to_owned()]));
            result.push(FuelStack::new(vec![tanks[size].to_owned(), tanks[size + 2].to_owned(), tanks[size + 3].to_owned()]));
            result.push(FuelStack::new(vec![tanks[size + 1].to_owned(), tanks[size + 2].to_owned(), tanks[size + 3].to_owned()]));
            result.push(FuelStack::new(vec![tanks[size].to_owned(), tanks[size + 1].to_owned(), tanks[size + 2].to_owned(), tanks[size + 3].to_owned()]));
            result.push(FuelStack::new(vec![tanks[size + 3].to_owned(), tanks[size + 3].to_owned()]));
        }

        let mut h_tanks: Vec<Tank> = Vec::new();

        h_tanks.push(Tank::new("HFT-T-125", 0.703, 0.08, Size::Sm, 0, FuelType::Hydrogen));
        h_tanks.push(Tank::new("HFT-S-250", 1.406, 0.16, Size::Md, 0, FuelType::Hydrogen));
        h_tanks.push(Tank::new("HFT-T-250", 2.813, 0.31, Size::Md, 0, FuelType::Hydrogen));
        h_tanks.push(Tank::new("HFT-S-375", 4.5, 0.5, Size::Lg, 0, FuelType::Hydrogen));
        h_tanks.push(Tank::new("HFT-T-375", 9.0, 1.0, Size::Lg, 0, FuelType::Hydrogen));
        h_tanks.push(Tank::new("HFT-S-500", 11.25, 1.25, Size::Xl, 0, FuelType::Hydrogen));
        h_tanks.push(Tank::new("HFT-T-500", 22.5, 2.5, Size::Xl, 0, FuelType::Hydrogen));

        return result;
    }
}

impl FuelStack {
    pub fn get_wet_mass(&self) -> f64 {
        let mut result: f64 = 0.0;
        for t in self.tanks.to_owned() {
            result += t.wet_mass;
        }
        result
    }
    pub fn get_dry_mass(&self) -> f64 {
        let mut result: f64 = 0.0;
        for t in self.tanks.to_owned() {
            result += t.dry_mass;
        }
        result
    }
    pub fn get_name(&self) -> String {
        let mut result: String = "".to_owned();
        for t in self.tanks.to_owned() {
            if result.len() != 0 {
                result.push_str(", ");
            }
            result.push_str(&t.name);
        }
        result
    }
}