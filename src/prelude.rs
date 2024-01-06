//! Crate prelude

// Re-export the crate Error.
pub use crate::error::Error;

// Alias Result to be the crate Result.
pub type Result<T> = core::result::Result<T, Error>;

// Generic Wrapper tuple struct for newtype pattern,
// mostly for external type to type From/TryFrom conversions
pub struct W<T>(pub T);

// Personal preference.
pub use std::format as f;

pub const CONSTANT_PATH: &'static str = "./src/utils/constants.txt";
pub const DEFAULT_OUTPUT_DIR: &'static str = include_str!("./utils/constants.txt");
pub const PRETTY_TIME_FORMAT: &'static str = "%a - %d %b %Y - %T";
pub const FILE_NAME_TIME_FORMAT: &'static str = "%d-%m-%Y_%H:%M:%S";
