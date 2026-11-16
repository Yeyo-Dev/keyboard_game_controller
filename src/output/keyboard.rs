use std::error::Error;

use enigo::{Direction, Enigo, Key, Keyboard, Settings};

pub struct KeyboardOutput {
    enigo: Enigo,
}

impl KeyboardOutput {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        Ok(Self { enigo: Enigo::new(&Settings::default())? })
    }

    pub fn type_text(&mut self, text: &str) -> Result<(), Box<dyn Error>> {
        self.enigo.text(text)?;
        Ok(())
    }

    pub fn key_tap(&mut self, name: &str) -> Result<(), Box<dyn Error>> {
        let key = parse_key(name)?;
        self.enigo.key(key, Direction::Click)?;
        Ok(())
    }
}

fn parse_key(name: &str) -> Result<Key, Box<dyn Error>> {
    let key = match name.to_ascii_lowercase().as_str() {
        "backspace" => Key::Backspace,
        "space" => Key::Space,
        "enter" | "return" => Key::Return,
        "tab" => Key::Tab,
        "escape" | "esc" => Key::Escape,
        "delete" => Key::Delete,
        "up" => Key::UpArrow,
        "down" => Key::DownArrow,
        "left" => Key::LeftArrow,
        "right" => Key::RightArrow,
        "home" => Key::Home,
        "end" => Key::End,
        "pageup" => Key::PageUp,
        "pagedown" => Key::PageDown,
        other if other.chars().count() == 1 => {
            Key::Unicode(other.chars().next().expect("one character"))
        }
        _ => return Err(format!("Tecla no reconocida: {name}").into()),
    };
    Ok(key)
}
