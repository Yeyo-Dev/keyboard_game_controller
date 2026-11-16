mod core;
mod input;
mod output;

use std::{env, error::Error, thread, time::Duration};

use gilrs::EventType;

use crate::{
    core::{config::Config, engine::InputEngine},
    input::gamepad::{GamepadReader, button_name},
    output::{OutputBackend, OutputMode},
};

fn main() -> Result<(), Box<dyn Error>> {
    let live = env::args().any(|arg| arg == "--live");
    let config_path = env::args()
        .skip(1)
        .find(|arg| !arg.starts_with("--"))
        .unwrap_or_else(|| "config/default.json".to_string());

    println!("======================================");
    println!("       OS INPUT ENGINE ");
    println!("======================================");

    let config = Config::load(&config_path)?;
    println!("Perfil: {} ({})", config.profile_name, config.version);
    println!("Modo inicial: {:?}", config.initial_mode);

    let mut engine = InputEngine::new(&config);
    let output_mode = if live { OutputMode::Live } else { OutputMode::DryRun };
    let mut output = OutputBackend::new(output_mode)?;

    let mut gamepad = GamepadReader::new()?;

    println!(
        "Salida: {}",
        if live { "LIVE — se inyectará entrada" } else { "DRY-RUN — solo simulación en consola" }
    );
    println!("Conecta el control. Ctrl+C para salir.");
    if !live {
        println!("Cuando estés listo para escribir en otras aplicaciones, ejecuta con --live.");
    }

    loop {
        while let Some(event) = gamepad.next_event() {
            match event.event {
                EventType::ButtonPressed(button, _) => {
                    let name = button_name(button);
                    println!("PRESIONADO: {name}");

                    let actions = engine.button_pressed(&name);
                    for action in actions {
                        println!("ACCIÓN: {action:?}");
                        output.execute(&action)?;
                    }
                    println!("MODO ACTUAL: {:?}", engine.mode());
                }
                EventType::ButtonReleased(button, _) => {
                    let name = button_name(button);
                    engine.button_released(&name);
                    println!("LIBERADO: {name}");
                }
                EventType::AxisChanged(axis, value, _) => {
                    let filtered = engine.filter_axis(value);
                    println!("EJE: {axis:?} = {filtered:.2}");
                }
                EventType::Connected => println!("Control conectado."),
                EventType::Disconnected => {
                    engine.clear_pressed();
                    println!("Control desconectado; estado de botones reiniciado.");
                }
                _ => {}
            }
        }

        thread::sleep(Duration::from_millis(4));
    }
}
