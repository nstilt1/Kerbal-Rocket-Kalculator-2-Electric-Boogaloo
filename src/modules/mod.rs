pub mod tanks;
pub mod engines;
pub mod calculator;
pub mod rocket_config;
pub mod fuel_type;
pub mod size;

#[macro_export]
#[cfg(test)]
macro_rules! debug {
    ($($arg:tt)*) => {
        println!($($arg)*);
    };
}

#[macro_export]
#[cfg(not(test))]
macro_rules! debug {
    ($($arg:tt)*) => {
        
    };
}