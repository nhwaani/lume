pub mod agent {
    pub mod tools;
}
pub mod config;
pub mod github;

pub use config::{AppConfig, Behavior, HexColor, Theme};

pub fn init() {
    println!("Lume Core Initialized");
}
