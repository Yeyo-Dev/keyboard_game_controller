mod keyboard;

use std::error::Error;

use crate::core::engine::Action;
use keyboard::KeyboardOutput;

#[derive(Debug, Clone, Copy)]
pub enum OutputMode {
    DryRun,
    Live,
}

pub struct OutputBackend {
    mode: OutputMode,
    keyboard: Option<KeyboardOutput>,
}

impl OutputBackend {
    pub fn new(mode: OutputMode) -> Result<Self, Box<dyn Error>> {
        let keyboard = match mode {
            OutputMode::DryRun => None,
            OutputMode::Live => Some(KeyboardOutput::new()?),
        };

        Ok(Self { mode, keyboard })
    }

    pub fn execute(&mut self, action: &Action) -> Result<(), Box<dyn Error>> {
        match self.mode {
            OutputMode::DryRun => {
                println!("[DRY-RUN] No se inyectó nada: {action:?}");
            }
            OutputMode::Live => {
                let keyboard = self.keyboard.as_mut().expect("backend live inicializado");
                match action {
                    Action::TypeText { text } => keyboard.type_text(text)?,
                    Action::KeyTap { key } => keyboard.key_tap(key)?,
                }
            }
        }
        Ok(())
    }
}
