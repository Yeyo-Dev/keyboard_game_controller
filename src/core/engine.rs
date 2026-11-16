use std::collections::HashSet;

use serde::Deserialize;

use super::config::Config;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Mode {
    Idle,
    Writing,
    Navigation,
    Developer,
    Macros,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Action {
    TypeText { text: String },
    KeyTap { key: String },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct ChordId(usize);

pub struct InputEngine {
    mode: Mode,
    deadzone: f32,
    mode_switches: std::collections::HashMap<String, Mode>,
    bindings: Vec<super::config::Binding>,
    pressed: HashSet<String>,
    fired_chords: HashSet<ChordId>,
}

impl InputEngine {
    pub fn new(config: &Config) -> Self {
        Self {
            mode: config.initial_mode,
            deadzone: config.deadzone,
            mode_switches: config.mode_switches.clone(),
            bindings: config.bindings.clone(),
            pressed: HashSet::new(),
            fired_chords: HashSet::new(),
        }
    }

    pub fn mode(&self) -> Mode {
        self.mode
    }

    /// Processes a new physical press. A chord fires once until one of its
    /// buttons is released, preventing repeated output while held.
    pub fn button_pressed(&mut self, button: &str) -> Vec<Action> {
        let button = normalize(button);
        let was_new = self.pressed.insert(button.clone());

        if !was_new {
            return Vec::new();
        }

        // Mode switches take precedence over ordinary bindings.
        if let Some(next_mode) = self.mode_switches.get(&button) {
            self.mode = *next_mode;
            self.fired_chords.clear();
            return Vec::new();
        }

        let mut actions = Vec::new();

        for (index, binding) in self.bindings.iter().enumerate() {
            if binding.mode != self.mode {
                continue;
            }

            let chord: Vec<String> = binding.buttons.iter().map(|b| normalize(b)).collect();
            let chord_is_down = chord.iter().all(|key| self.pressed.contains(key));

            if chord_is_down {
                let id = ChordId(index);
                if self.fired_chords.insert(id) {
                    actions.push(binding.action.clone());
                }
            }
        }

        actions
    }

    pub fn button_released(&mut self, button: &str) {
        let button = normalize(button);
        self.pressed.remove(&button);

        // A chord can fire again only after at least one member is released.
        for (index, binding) in self.bindings.iter().enumerate() {
            if binding.buttons.iter().any(|b| normalize(b) == button) {
                self.fired_chords.remove(&ChordId(index));
            }
        }
    }

    pub fn filter_axis(&self, value: f32) -> f32 {
        if value.abs() < self.deadzone { 0.0 } else { value }
    }

    pub fn clear_pressed(&mut self) {
        self.pressed.clear();
        self.fired_chords.clear();
    }
}

fn normalize(value: &str) -> String {
    value.trim().to_ascii_lowercase()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::config::Config;
    use std::collections::HashMap;

    fn test_config() -> Config {
        Config {
            profile_name: "test".into(),
            version: "test".into(),
            initial_mode: Mode::Writing,
            deadzone: 0.2,
            mode_switches: HashMap::from([("north".into(), Mode::Navigation)]),
            bindings: vec![
                crate::core::config::Binding {
                    mode: Mode::Writing,
                    buttons: vec!["lefttrigger".into(), "south".into()],
                    action: Action::TypeText { text: "1".into() },
                },
                crate::core::config::Binding {
                    mode: Mode::Navigation,
                    buttons: vec!["south".into()],
                    action: Action::KeyTap { key: "enter".into() },
                },
            ],
        }
    }

    #[test]
    fn chord_fires_only_after_all_members_are_pressed() {
        let mut engine = InputEngine::new(&test_config());
        assert!(engine.button_pressed("lefttrigger").is_empty());
        assert!(matches!(
            engine.button_pressed("south").as_slice(),
            [Action::TypeText { text }] if text == "1"
        ));
    }

    #[test]
    fn held_chord_does_not_repeat() {
        let mut engine = InputEngine::new(&test_config());
        engine.button_pressed("lefttrigger");
        assert_eq!(engine.button_pressed("south").len(), 1);
        assert!(engine.button_pressed("south").is_empty());
    }

    #[test]
    fn chord_can_fire_again_after_release() {
        let mut engine = InputEngine::new(&test_config());
        engine.button_pressed("lefttrigger");
        engine.button_pressed("south");
        engine.button_released("south");
        assert_eq!(engine.button_pressed("south").len(), 1);
    }

    #[test]
    fn mode_switch_changes_active_bindings() {
        let mut engine = InputEngine::new(&test_config());
        engine.button_pressed("north");
        assert_eq!(engine.mode(), Mode::Navigation);
        assert!(matches!(
            engine.button_pressed("south").as_slice(),
            [Action::KeyTap { key }] if key == "enter"
        ));
    }

    #[test]
    fn deadzone_suppresses_small_axis_values() {
        let engine = InputEngine::new(&test_config());
        assert_eq!(engine.filter_axis(0.1), 0.0);
        assert_eq!(engine.filter_axis(-0.8), -0.8);
    }
}
