use std::fmt::Display;

use wasm_bindgen::JsError;

pub mod calculator;
pub mod engines;
pub mod fuel_type;
pub mod rocket_config;
pub mod tanks;
pub mod utils;

#[macro_export]
#[cfg(test)]
macro_rules! debug {
    ($($arg:tt)*) => {
        if true {
            println!($($arg)*);
        }
    };
}

#[macro_export]
#[cfg(not(test))]
macro_rules! debug {
    ($($arg:tt)*) => {};
}

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[cfg(target_arch = "wasm32")]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

#[cfg(target_arch = "wasm32")]
#[macro_export]
macro_rules! console_log_2 {
    ($($arg:tt)*) => {
        if true {
            let x = format!($($arg)*);
            crate::modules::log(&x);
        }
    };
}

#[cfg(not(target_arch = "wasm32"))]
#[macro_export]
macro_rules! console_log_2 {
    ($($arg:tt)*) => {
        if false {
            println!($($arg)*);
        }
    };
}
#[macro_export]
macro_rules! console_log {
    ($($arg:tt)*) => {
        if false {
            println!($($arg)*);
        }
    };
}

#[derive(Debug, Clone)]
pub enum Error {
    MissingTech(String),
    MaxWetMassBelowCurrentMass,
    InvalidHeight,
    HeightTooLarge,
    HeightOutsideOfNoseconeRange,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::MissingTech(v) => v,
            Self::MaxWetMassBelowCurrentMass => "Max wet mass is below current mass",
            Self::InvalidHeight => "Invalid height (NaN or negative)",
            Self::HeightTooLarge => "Height exceeds limit",
            Self::HeightOutsideOfNoseconeRange => "Nosecone height is outside of its range",
        })
    }
}

impl From<Error> for JsError {
    fn from(value: Error) -> Self {
        Self::new(&value.to_string())
    }
}
