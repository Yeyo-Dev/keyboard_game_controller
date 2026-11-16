# OS Input Engine

Prototipo v0.2 de un motor de entrada con control Xbox/PlayStation para Windows y Linux.

## Qué incluye

- Lectura de eventos físicos con `gilrs`.
- Máquina de estados: `idle`, `writing`, `navigation`, `developer`, `macros`.
- Cambio de modo por botón (configurable desde JSON).
- Acordes de dos o más botones, definidos en JSON.
- Prevención de repetición: un acorde se ejecuta una vez hasta que se libera alguno de sus botones.
- Filtro deadzone básico para ejes.
- Salida `dry-run` por defecto y salida real opcional con `--live`.
- Pruebas unitarias del motor de estados y acordes.

## Requisitos

- Rust 1.87 o posterior (por `enigo 0.6.1`).
- Windows o Linux.
- En Linux, `gilrs` suele necesitar `pkg-config` y `libudev-dev`:

```bash
sudo apt install build-essential pkg-config libudev-dev
```

## Ejecutar

Desde la carpeta raíz:

```bash
cargo check
cargo test
cargo run
```

Por defecto, el programa usa **DRY-RUN**: muestra qué acción habría ejecutado, pero no escribe en otras aplicaciones.

Para habilitar la inyección real:

```bash
cargo run -- --live
```

Se puede indicar otro perfil:

```bash
cargo run -- config/default.json
cargo run -- config/default.json --live
```

Conecta el control antes o después de iniciar el programa. Para salir, usa `Ctrl+C`.

## Mapeo de ejemplo

En `config/default.json`:

- `north` cambia al modo `writing`.
- `west` cambia al modo `navigation`.
- En `writing`, `south` escribe `a`.
- En `writing`, `east` escribe `b`.
- En `writing`, `lefttrigger + south` escribe `1`.
- En `writing`, `rightthumb` pulsa espacio.
- En `navigation`, `south` pulsa Enter.
- En `navigation`, `east` pulsa Backspace.

Los nombres de botones son los nombres normalizados de `gilrs::Button` (por ejemplo, `South`, `LeftTrigger`, `RightThumb`). El significado físico de `South` depende del mapeo estándar del control y del fabricante.

## Formato de los acordes

```json
{
  "mode": "writing",
  "buttons": ["lefttrigger", "south"],
  "action": {
    "kind": "type_text",
    "text": "1"
  }
}
```

Acciones soportadas:

```json
{ "kind": "type_text", "text": "hola" }
{ "kind": "key_tap", "key": "backspace" }
```

Teclas admitidas por `key_tap`: `backspace`, `space`, `enter`, `tab`, `escape`, `delete`, `up`, `down`, `left`, `right`, `home`, `end`, `pageup`, `pagedown`, o un carácter individual.

## Diseño y limitaciones de esta versión

- Los cambios de modo son por pulsación (toggle), no por mantener presionado.
- La deadzone actual es axial y solo filtra el valor de cada eje; todavía no implementa curva radial ni compensación de sensibilidad.
- La lectura de ejes solo se imprime; el movimiento del ratón y el scroll aún no están conectados.
- El overlay `egui`, hot-reload de JSON, macros y backend específico `uinput` todavía no están implementados.
- En Windows, `gilrs` usa Windows Gaming Input por defecto; ciertos escenarios requieren que la aplicación tenga foco. Consulta la documentación de `gilrs` si no llegan eventos.
- En Linux/Wayland, la inyección puede depender del compositor y de los permisos del sistema. `enigo` no garantiza el mismo comportamiento en todos los entornos.

## Seguridad de uso

Empieza siempre con `cargo run` (DRY-RUN). Usa `--live` solo cuando hayas comprobado el mapeo: las acciones se enviarán a la aplicación que tenga el foco. No configures acordes que puedan borrar, enviar o ejecutar contenido accidentalmente.

## Verificación

```bash
cargo fmt --check
cargo test
cargo check
```
