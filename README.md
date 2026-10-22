# BioVigía

## Problema

Un pescador o cazador no puede saber, antes de desplazarse a una zona, si allí hay especies invasoras que le van a impedir pescar o cazar lo que quiere. Cuando llega y se da cuenta, ya ha perdido el día.

## De dónde viene esto

Esto no es algo que hayamos leído en un libro. Es algo que nos pasó: fuimos a pescar cangrejos con mi abuelo y sus amigos, pusimos los reteles con cebo e hígado de pollo, y a las pocas horas empezamos a ver burbujitas en el agua. El visón americano se estaba comiendo nuestros cebos y los cangrejos que íbamos a pescar. No era un caso aislado; a otros pescadores de la zona les había pasado lo mismo. Si alguien nos hubiera avisado de que esa zona estaba afectada, no habríamos ido. Pero no hay ningún sitio donde consultarlo.

## Cómo llegan hoy los avisos, y por qué no basta

Los avisos llegan por WhatsApp, por grupos pequeños, o de voz en voz. Si eres de la zona y conoces a gente, te enteras. Pero si vienes de fuera (por ejemplo, un pescador de Madrid que va a un torneo a Valencia), no tienes ningún canal donde mirar. La información no está centralizada, no se puede verificar y no llega a quien la necesita.

## Quién sufre esto y cómo

- **Pescadores y cazadores locales**: pierden jornadas completas, no capturan lo que quieren y, si viven de ello, pierden dinero. Tienen familias que alimentar.
- **Pescadores y cazadores de fuera**: llegan a una zona nueva sin conocer a nadie, sin información y sin forma de saber si les va a merecer la pena.
- **Aficionados**: no pueden disfrutar de su hobby porque no saben si van a perder el tiempo.
- **Personas que avisan**: reportan un aviso, pero no tienen forma de saber si ya existe, si es coherente, si están siendo útiles o perdiendo su tiempo o si alguien si quiera lo va a leer.

## Qué datos existen ya

Hay plataformas como **iNaturalist** o **GBIF** donde la gente reporta observaciones de fauna, incluido el visón americano. De hecho, yo mismo reporté ese visón en iNaturalist esa misma tarde. Es una plataforma donde cualquiera puede subir una foto con su localización y dejar constancia de qué fauna ha visto. El problema es que no está orintado a este problema, sino en reportar TODO tipo de fauna y para colmo, los reportes, ahí se suelen quedar: en una base de datos global que el pescador o cazador de a pie no consulta, no entiende o ni siquiera conoce. Los datos existen, pero no están cruzados con lo que a nosotros nos importa: si eso que alguien ha visto me va a arruinar la jornada de pesca o caza.

## Qué hace falta procesar

Hoy, si yo quisiera ir a pescar barbos a un río con mi abuelo, tendría que hacerlo todo a mano. Primero de todo encontrar si es que hay, un grupo de WhatsApp y preguntar si alguien ha visto siluros por allí (se comen a los barbos), mirar si hay avisos recientes, cruzarlos con los de hace meses para ver si van a más o a menos, y aun así quedarme con la duda de si la información está actualizada o es de fiar. Eso no debería ser así. La app tendría que coger todos esos avisos, validar que son coherentes, filtrar los repetidos o poco fiables, cruzar los de distintas fuentes y fechas, y calcular cosas como (en este ejemplo) en qué zona no hay siluros y se puede pescar bien el barbo o dónde hay menor presencia/avistamientos reportados de siluros, que, dentro de lo que cabe, nos permita maximizar nuestras probabilidades de pescar barbos. En última instancia debería resumírmelo de forma que yo, en diez segundos, sepa si me merece la pena ir o no a cazar o pescar en esa zona, sin mas rodeos, un simple voy o no voy, punto.

## Por qué hace falta que esté en la nube

Porque yo reporto el visón desde mi río, y un pescador de Madrid que quiere venir a un torneo a Valencia necesita saberlo antes de coger el coche. Esos dos hechos tienen que encontrarse en algún sitio común, porque ni él lo tiene en su móvil ni yo se lo puedo contar por WhatsApp y no es probable que le haya corrido la voz de dichos avistamientos. Sin un lugar donde se junten todos los avisos de todas las personas, la información sigue sin llegar a quien la necesita.

## Referencias
- https://www.miteco.gob.es/content/dam/miteco/es/biodiversidad/publicaciones/estrategias/Estrategia-eei-acuaticas-24-07-24.pdf
- https://www.miteco.gob.es/es/prensa/ultimas-noticias/2023/09/las-especies-exoticas-invasoras-generan-ya-un-gasto-global-de-42.html
- https://www.miteco.gob.es/es/biodiversidad/temas/conservacion-de-especies/especies-exoticas-invasoras/ce_eei_peces.html
- https://www.inaturalist.org/
- https://www.gbif.org/es/

### Documentación adicional y configuración
La documentacion relativa al objetivo 0 se encuentra [aquí](docs/objetivos/objetivo-0/objetivo-0.md)

### Planificación
La planificación relativa al objetivo 1 se encuentra respectivamente en los siguientes puntos:
- [Jornadas de usuario](docs/objetivos/objetivo-1/user-journeys.md)
- [Personas](docs/objetivos/objetivo-1/personas.md)
- [Historias de usuario](docs/objetivos/objetivo-1/historias-usuarios.md)
- [Milestones](docs/objetivos/objetivo-1/milestones.md)

### Lenguaje
El modelo del dominio se implementa en **Rust**. Motivos:

- **Tipado estático fuerte**, comprobado en compilación.
- **No existe `null`**: lo opcional se expresa con `Option`, así que invariantes como «depredación sin depredador» o «avistamiento sin fuente» no compilan, en lugar de comprobarse en tiempo de ejecución.
- **Errores del dominio con `Result`**: los constructores devuelven `Result<Self, ErrorDominio>`, separando por diseño los errores del dominio de los de programación (`panic!`).
- **`enum` con datos** para los valores cerrados del dominio (`Fuente`, `FaseVital`) y *newtypes* (`Latitud`, `Longitud`) para que intercambiar valores del mismo tipo base no compile.
- **Herramientas oficiales integradas**: rustup para gestionar las versiones.

En el Milestone 0 se usa solo la biblioteca estándar, hasta que haya gestor de dependencias: la fecha de los avistamientos se representa con `std::time::SystemTime` y las URLs de las fotos se validan de forma mínima.

### Entorno de desarrollo
La versión de Rust del proyecto está fijada en el fichero [`rust-toolchain.toml`](rust-toolchain.toml), que es la única fuente de verdad para la versión.

Único paso manual: instalar [rustup](https://rustup.rs/), que trabaja en espacio de usuario (`~/.rustup` y `~/.cargo`) sin modificar el sistema (en Windows, desde WSL):

    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

Después, desde la raíz del repositorio:

    rustup toolchain install   # instala la versión fijada en rust-toolchain.toml
    rustc --version            # comprueba que es la fijada en rust-toolchain.toml

Para cambiar de versión de Rust se modifica la línea `channel` de `rust-toolchain.toml` en un commit.

### Gestor de dependencias

**Opciones consideradas:** Cargo, y los sistemas de build Bazel (rules_rust) y Nix, que también pueden gestionar crates.

**Criterios:** estándar del lenguaje, número de dependencias (unas 5 directas en el producto mínimamente viable: HTTP, JSON, línea de órdenes), complejidad de mantenimiento y recomendaciones de la comunidad Rust como contraste.

**Reflexión final:** Cargo es el gestor oficial de Rust. Declara las dependencias en `Cargo.toml` y fija las versiones exactas en `Cargo.lock`. Bazel y Nix resuelven problemas de build a gran escala que este proyecto no tiene, y siguen partiendo del formato de Cargo. Cargo funciona igual con pocas dependencias que con muchas, y en la comunidad Rust no hay debate sobre usarlo. Se elige Cargo.

No requiere instalación adicional: viene con rustup (ver [Entorno de desarrollo](#entorno-de-desarrollo)). Razonamiento ampliado en el issue [#21](https://github.com/marcocugr/BioVigia/issues/21).

### Gestor de tareas

**Requisito:** un fichero de tareas al que pueda apuntar `iv.yaml`, con `install`, `test` y `check`. Cargo cubre comandos sueltos (`cargo run`, `cargo test`, `cargo check`, `cargo fmt`, `cargo clippy`), pero no tiene fichero de tareas, y los alias de `.cargo/config.toml` solo renombran subcomandos de Cargo.

**Opciones consideradas:**

| Opción | Fichero de tareas | Instalación | Mantenimiento |
|---|---|---|---|
| just | `justfile` | Externa (binario) | 1.58.0, 3 de agosto de 2026 |
| make | `Makefile` | Del sistema; no viene con Rust | Madura; última versión 4.4.1 (2023) |
| task | `Taskfile.yml` | Externa (binario único) | v3.54.0, 1 de octubre de 2026 |
| cargo-make | `Makefile.toml` | Externa (`cargo install`) | 0.37.24, enero de 2025 |
| cargo-xtask | Crate Rust propio | Ninguna | Patrón, sin releases formales |

**Criterios y fuentes:** el requisito anterior, mantenimiento y deuda técnica, coste de instalación y la opinión de la comunidad. Además de la documentación oficial de cada herramienta, se han consultado recomendaciones en foros técnicos (Stack Overflow, Reddit) y discusiones en repositorios de GitHub. Inicialmente se había tenido en cuenta ese criterio aunque se hubiese puesto justo todo lo contrario; tras la revisión se incorpora a la decisión. La conclusión general de esas fuentes: make sigue siendo válido y muy habitual; just se recomienda para un fichero de tareas sencillo y legible; task es una alternativa moderna sólida; cargo-make y cargo-xtask compensan sobre todo en proyectos Rust con automatización compleja.

**Reflexión final:** cargo-make añade configuración extensa y cargo-xtask obliga a mantener código de automatización, lo que sería deuda técnica para el alcance actual. just, make y task cumplen el requisito y entre ellos la diferencia técnica es pequeña. Se elige just por tener una sintaxis más sencilla que make, estar activamente mantenido y encajar con lo que recomienda la comunidad para este uso. El coste asumido es una herramienta externa, de entre 4,5 y 5,5 MB sin dependencias de runtime. Cargo no se reutiliza como gestor de tareas: gestiona crates y versiones, y no puede alojar ese fichero. Si el proyecto creciera, make o task serían alternativas válidas.

Las tareas viven en el `justfile` de la raíz del repositorio. Razonamiento ampliado en el issue [#20](https://github.com/marcocugr/BioVigia/issues/20).

Instalación (una sola vez, ejecutada desde fuera del repositorio):

    cargo install just --locked
    just --version       # comprueba la instalación
    just --list          # lista las tareas del justfile

### Comprobación sintaxis de una clase
La entidad que se comprueba es `CatalogoEspecies`, en [`src/dominio/especie.rs`](src/dominio/especie.rs).

**Qué es:** el catálogo de especies que conoce el sistema. Cada especie se identifica por su nombre científico y guarda su nombre común y si es invasora. También admite nombres científicos anteriores (sinónimos), para que un aviso que use un nombre antiguo se asocie a la especie correcta.

**Qué hace:**
- `registrar` da de alta una especie. Rechaza nombres vacíos y nombres repetidos, incluidos los que chocan con el nombre científico o los nombres anteriores de otra especie.
- `especie` recibe un nombre (actual o anterior) y devuelve el nombre científico vigente, o un error si la especie es desconocida.
- `especie_invasora` hace lo mismo, pero solo devuelve la especie si es invasora, en caso contrario devuelve un error.

**Uso en el servicio:** BioVigía solo debe aceptar avistamientos y depredaciones de especies que existan en el catálogo y que sean invasoras. El catálogo es la fuente de la verdad de esa validación, de modo que un aviso con una especie desconocida o no invasora se rechaza antes de entrar en el sistema.

**Orden check:**

    just check

Esta orden ejecuta `cargo check` (mira el [justfile](./justfile)), que comprueba únicamente que el código compila (sintaxis y tipos), incluida esta entidad. Todavía no ejecuta pruebas ni formato ni linter.


## Tests

### Aserciones vs Test Runner

**ASERCIONES:** Son las macros que comparan lo obtenido con lo esperado. En Rust usamos las macros de la biblioteca estándar:
- `assert!`: verifica que una condición es verdadera
- `assert_eq!`: verifica que dos valores son iguales
- `assert_ne!`: verifica que dos valores son diferentes

**TEST RUNNER:** Es el sistema que busca los tests, los ejecuta y genera el informe. En Rust es `cargo test`, integrado en Cargo.

**Por qué no usamos crates externos:** Rust ya trae todo lo necesario en la biblioteca estándar. Los criterios de aceptación se escriben ANTES de elegir herramientas, y `cargo test` + las macros estándar cubren perfectamente las necesidades del proyecto sin añadir dependencias innecesarias.

### Organización de los tests

#### Tests unitarios
Los tests unitarios van **dentro de cada fichero**, en un módulo `tests` con la anotación `#[cfg(test)]`. Esto permite:
- Mantener los tests cerca del código que prueban
- Acceder a funciones y estructuras privadas
- Ejecutarse solo cuando corremos `cargo test`

Ejemplo de estructura:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registrar_especie_valida() {
        let mut catalogo = CatalogoEspecies::new();
        assert!(catalogo.registrar(...).is_ok());
    }
}
```

#### Tests de integración
Los tests de integración van en el directorio `tests/` en la raíz del proyecto. Cada archivo en `tests/` es un crate independiente que:
- Solo puede acceder a la API pública de la biblioteca
- Prueba que los módulos funcionan juntos correctamente
- Se ejecuta en paralelo con otros tests de integración

#### Datos de prueba
Los archivos HTML y datos de prueba se guardan en `tests/fixtures/`:
- `tests/fixtures/inaturalist.html`
- `tests/fixtures/gbif.html`
- `tests/fixtures/globi.html`
- `tests/fixtures/gisd.html`

### Principios F.I.R.S.T.

Los tests del proyecto siguen los principios F.I.R.S.T.:

- **Fast (Rápidos)**: Los tests unitarios son instantáneos. Los tests de integración que necesitan I/O (como parsear HTML) se mantienen en `tests/` separados de la lógica de dominio pura.

- **Independent (Independientes)**: Cada test crea sus propios datos y no depende del estado de otros tests. Usamos funciones helper para construir objetos de prueba cuando es necesario.

- **Repeatable (Repetibles)**: No dependemos de servicios externos en los tests. Los datos de APIs se guardan en `tests/fixtures/` y se procesan localmente. La fecha de referencia se pasa como parámetro en lugar de usar el reloj del sistema.

- **Self-validating (Auto-validables)**: Cada test tiene un resultado claro: pasa o falla. No requieren inspección manual de logs o archivos.

- **Timely (Oportunos)**: Los tests se escriben mientras se desarrolla la funcionalidad, no después. Los criterios de aceptación de cada issue se convierten directamente en tests.

### Ejecución de tests

#### Desde cero
```bash
just test
```

Esta orden ejecuta todos los tests.

#### Con directorio de compilación personalizado
```bash
CARGO_TARGET_DIR=/tmp/biovigia-build cargo test --locked
```

El flag `--locked` asegura que no se modifica `Cargo.lock`, como requiere el objetivo 5.

#### Tests específicos
```bash
cargo test hu001        # Solo tests que contengan "hu001"
cargo test --test hu001 # Solo el archivo de integración específico
```

### Configuración

La clave `test` en `iv.yaml` apunta al archivo de tests de integración principal que verifica la historia de usuario completa.
