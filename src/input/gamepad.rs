use gilrs::{Event, Gilrs};

pub struct GamepadReader {
    gilrs: Gilrs,
}

impl GamepadReader {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self { gilrs: Gilrs::new()? })
    }

    pub fn next_event(&mut self) -> Option<Event> {
        self.gilrs.next_event()
    }
}

/// Gilrs exposes a normalized button layout. The Debug names are normalized
/// to lowercase so they can be referenced from JSON (e.g. South -> "south").
pub fn button_name(button: gilrs::Button) -> String {
    format!("{button:?}").to_ascii_lowercase()
}
