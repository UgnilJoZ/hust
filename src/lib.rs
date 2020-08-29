#[macro_use]
extern crate serde_derive;
extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate serde_xml;
pub mod error;
pub use error::{Error, Result};
pub mod lights;
pub mod bridge;
pub use bridge::Bridge;
pub mod config;
mod discovery;
pub use discovery::find_bridges;