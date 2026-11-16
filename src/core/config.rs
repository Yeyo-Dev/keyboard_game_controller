use std::{collections::HashMap, fs, path::Path};

use serde::Deserialize;

use super::engine::{Action, Mode};

#[derive(Debug, Clone, Deserialize)]
pub struct Binding {
    pub mode: Mode,
    pub buttons: Vec<String>,
    pub action: Action,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub profile_name: String,
    pub version: String,
    pub initial_mode: Mode,
    pub deadzone: f32,
    pub mode_switches: HashMap<String, Mode>,
    pub bindings: Vec<Binding>,
}

impl Config {
    pub fn load(path: impl AsRef<Path>) -> Result<Self, Box<dyn std::error::Error>> {
        let raw = fs::read_to_string(path)?;
        let config: Self = serde_json::from_str(&raw)?;

        if !(0.0..1.0).contains(&config.deadzone) {
            return Err("deadzone debe ser mayor o igual a 0 y menor que 1".into());
        }

        for binding in &config.bindings {
            if binding.buttons.is_empty() {
                return Err("Cada binding debe contener al menos un botón".into());
            }
        }

        Ok(config)
    }
}
