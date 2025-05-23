use std::fmt::Display;

use wasm_bindgen::JsError;

pub mod calculator;
pub mod engines;
pub mod fuel_type;
pub mod rocket_config;
pub mod size;
pub mod tanks;

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
    ($($arg:tt)*) => {};
}

#[derive(Debug, Clone)]
pub enum Error {
    MissingTech(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::MissingTech(v) => v,
        })
    }
}

impl From<Error> for JsError {
    fn from(value: Error) -> Self {
        Self::new(&value.to_string())
    }
}
