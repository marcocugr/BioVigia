# Justificación: gestor de dependencias y gestor de tareas

El razonamiento detallado está en los issues:
- Gestor de dependencias: [#21](https://github.com/marcocugr/BioVigia/issues/21)
- Gestor de tareas: [#20](https://github.com/marcocugr/BioVigia/issues/20)

**Resumen:** Cargo como gestor de dependencias y `just` como gestor de tareas. No se reutiliza una herramienta para las dos funciones.

## Criterios
Se evalúa con criterios objetivos: estándar del lenguaje, número de dependencias del proyecto, mantenimiento activo (versión y fecha de la última release), coste de instalación y deuda técnica. No se usan la familiaridad, la popularidad ni la valoración subjetiva de la documentación.

## Gestor de dependencias: Cargo
- **Estándar.** Cargo es el gestor de paquetes oficial de Rust ([documentación](https://doc.rust-lang.org/cargo/)). Las dependencias se declaran en `Cargo.toml`, que es el formato estándar, y las versiones quedan fijadas en `Cargo.lock`.
- **Número de dependencias.** El producto mínimamente viable necesita unas 5 dependencias directas (HTTP, JSON, línea de órdenes...). Cargo funciona igual de bien con pocas que con muchas, así que este criterio no inclina la decisión hacia ninguna otra herramienta.
- **Alternativas.** En Rust no existe una alternativa real y equiparable al gestor oficial, por lo que no hay comparación que hacer.
- **Gestión de dependencias incluida.** `cargo add`, `cargo update` y `cargo tree` permiten añadir, actualizar e inspeccionar dependencias sin herramientas adicionales.

## Gestor de tareas: just

### Requisito
El objetivo exige un fichero de tareas al que apunte `iv.yaml` (clave `automatizar`, subclave `fichero`), con tareas `install`, `test` y `check`.

### Por qué no basta Cargo
Cargo ejecuta acciones individuales (`cargo check`, `cargo test`, `cargo run`, `cargo fmt`, `cargo clippy`), pero:
- `Cargo.toml` no define tareas, así que no hay fichero de tareas al que apuntar.
- Los alias de `.cargo/config.toml` solo renombran subcomandos de Cargo y no permiten tareas compuestas.

Cubrir el requisito con Cargo obligaría a añadir scripts o un crate de automatización.

### Comparación
| | just | cargo-make | cargo-xtask |
|---|---|---|---|
| Fichero de tareas | `justfile` | `Makefile.toml` | Crate Rust propio en el repo |
| Instalación | Binario externo | Ninguna externa | Ninguna externa |
| Mantenimiento | Activo: [versión 1.58.0 (3 de agosto de 2026)](https://crates.io/crates/just) | Última release estable: [0.37.24 (18 de enero de 2025)](https://github.com/sagiegurari/cargo-make/releases) | Patrón, sin releases formales |
| Deuda técnica | Baja: un fichero con tareas explícitas | Media: configuración extensa y más superficie de mantenimiento | Alta para el alcance actual: hay que mantener código Rust de automatización |

Releases: [just](https://github.com/casey/just/releases), [cargo-make](https://github.com/sagiegurari/cargo-make/releases).

### Decisión
`just`. Es la única opción que da un fichero de tareas claro para `iv.yaml` sin configuración extensa ni código de automatización que mantener, y está activamente mantenida.

### Por qué no se reutiliza Cargo como gestor de tareas
Cargo gestiona crates, versiones y compilación, pero no puede alojar el fichero de tareas que exige el objetivo. Por eso las dos funciones quedan en herramientas distintas: Cargo resuelve y compila mientras que `just` orquesta los comandos del proyecto y puede invocar a Cargo.

## Tareas definidas
| Tarea | Función |
|---|---|
| `install` | Descarga las dependencias del proyecto |
| `test` | Ejecuta las pruebas |
| `check` | Comprueba únicamente la sintaxis (`cargo check`): que el código compile, sin pruebas, formato ni linter |
