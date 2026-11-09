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

El proyecto no tiene dependencias externas: todo se hace con la biblioteca estándar. La fecha de los avistamientos se representa con `std::time::SystemTime` y el lector de la página de iNaturalist busca los campos con búsquedas de texto, sin analizador de HTML.

### Estructura del código

- `src/dominio/`: el modelo y sus reglas, sin leer ficheros ni usar la red. Catálogo de especies, red de depredación, avistamientos, tramos de río y coordenadas.
- `src/aplicacion/consulta.rs`: el caso de uso de la HU001 (`consultar_zona`).
- `src/aplicacion/mod.rs`: `FuenteAvistamientos`, el trait que cumple cualquier fuente de avistamientos, y `reunir`, que construye un único registro con los avistamientos de todas las fuentes.
- `src/aplicacion/inaturalist.rs`: la fuente iNaturalist, que lee la página guardada de una observación.
- `src/aplicacion/nomenclator.rs`: las especies conocidas y sus depredaciones, en un solo sitio.
- `tests/`: tests de integración y datos de prueba (ver [Tests](#tests)).

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

**Actualización del objetivo 4:** el producto mínimamente viable no necesita ninguna dependencia directa, porque los datos de las fuentes se leen de páginas HTML guardadas y no de sus APIs. Cargo se sigue usando para fijar la versión del compilador y las dependencias en `Cargo.lock`, que ahora solo contiene el propio paquete.

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
- `Especie::new` crea una especie y rechaza nombres vacíos. `CatalogoEspecies::new` recibe las especies ya creadas; el catálogo no decide qué especies se dan de alta.
- `especie` recibe un nombre (actual o anterior) y devuelve el nombre científico vigente, o un error si la especie es desconocida.
- `especie_invasora` hace lo mismo, pero solo devuelve la especie si es invasora, en caso contrario devuelve un error.
- `especie_con_nombre_comun` recibe el nombre con el que busca el pescador y devuelve el nombre científico de la especie. No distingue mayúsculas, tildes ni espacios de los lados, porque el nombre se normaliza igual al buscar. Un nombre que no es de ninguna especie da error, para que un nombre mal escrito no parezca una especie sin depredadores.

**Orden check:**

    just check

Esta orden ejecuta `cargo check` (mira el [justfile](./justfile)), que comprueba únicamente que el código compila (sintaxis y tipos), incluida esta entidad. No ejecuta pruebas ni formato ni linter: las pruebas se ejecutan con `just test` (ver [Tests](#tests)).


## Tests

### Herramientas de test
La justificación también está en [docs/objetivos/objetivo-4/justificacion-herramientas-test.md](docs/objetivos/objetivo-4/justificacion-herramientas-test.md) y en el issue [#60](https://github.com/marcocugr/BioVigia/issues/60).

Antes de escribir tests, había que decidir qué herramientas usar. No todas hacen lo mismo: una cosa es comprobar que el resultado obtenido coincide con el esperado, otra descubrir y ejecutar los tests, y otra preparar datos o repetir un test con varias entradas.

- **Biblioteca de aserciones:** compara lo obtenido con lo esperado y falla con un mensaje si no coinciden. En Rust, `assert!`, `assert_eq!` y `assert_ne!` forman parte de la biblioteca estándar. Existen alternativas como `pretty_assertions` o `googletest`.
- **Test runner:** descubre los tests, los ejecuta y presenta el resultado. En Rust, Cargo ya incluye uno mediante `cargo test`. Otra opción conocida es `cargo-nextest`.
- **Fixtures y tests con parámetros:** sirven para preparar datos de prueba o ejecutar el mismo test con distintos valores. No sustituyen ni a las aserciones ni al runner; se apoyan en él. En Rust, `rstest` es una opción habitual.

Antes de elegir, fijé estos criterios:
1. Que viniera con el lenguaje, para que el proyecto funcionara igual en WSL, Docker y en la máquina de quien corrija el código.
2. Que no escribiera en el directorio de los fuentes, salvo en el directorio de compilación, y que ese directorio pudiera cambiarse. Esto lo exige el objetivo 5.
3. Que, al fallar un test, el mensaje mostrara el valor esperado y el obtenido.
4. Que sirviera tanto para tests unitarios como para tests de integración.
5. Que funcionara con la toolchain estable y la edición 2024 fijada en `rust-toolchain.toml`.

Para buscar opciones, empecé por lo que ya trae el lenguaje, consultando [The Rust Book, capítulo 11](https://doc.rust-lang.org/book/ch11-00-testing.html) y [`cargo test`](https://doc.rust-lang.org/cargo/commands/cargo-test.html) en un libro que vi que se llamaba "The Cargo Book". Después busqué en [crates.io](https://crates.io) y [docs.rs](https://docs.rs) herramientas que cubrieran lo que la biblioteca estándar no ofrecía. Solo consideré opciones con documentación propia, licencia libre e información sobre la toolchain compatible, descartando automaticamente las que no indicaban nada de esto.

Las alternativas que examiné salen de [nexte.st](https://nexte.st), [pretty_assertions](https://docs.rs/pretty_assertions) y [googletest](https://github.com/google/googletest-rust).

#### Bibliotecas de aserciones
- **`assert!`, `assert_eq!` y `assert_ne!`:** son las aserciones de la biblioteca estándar. No requieren añadir ninguna dependencia, funcionan en tests unitarios y de integración, y `assert_eq!` muestra el valor esperado y el obtenido cuando falla.
- **`pretty_assertions`:** mejora la presentación de las diferencias al comparar valores, especialmente útil con estructuras grandes. Sin embargo, es una dependencia de desarrollo y solo aporta una mejora de legibilidad, donde además, puede modificar `Cargo.lock`, aunque `--locked` impide que Cargo lo cambie.
- **`googletest`:** ofrece comparadores combinables mediante `assert_that!` y `expect_that!`, con mensajes muy descriptivos. También es una dependencia de desarrollo y, en el momento de elegir, su API no era estable hasta la versión 1.0.

#### Test runners
- **`cargo test`:** viene con Cargo. Descubre y ejecuta tests unitarios, de integración y doctests; permite filtrar por nombre o por fichero. No escribe fuera del directorio de compilación, que puede cambiarse con `CARGO_TARGET_DIR`.
- **`cargo-nextest`:** ejecuta cada test en un proceso independiente, admite reintentos y puede generar informes en formatos como JUnit o JSON. Hay que instalarlo, no ejecuta los doctests por sí solo y estos deben lanzarse aparte con `cargo test --doc`.

`rstest` no compite directamente con las anteriores: añade fixtures y tests con parámetros sobre `cargo test`.

#### Decisión
Elegí las aserciones de la biblioteca estándar y `cargo test` como runner.

Es la única combinación que cumple los cinco criterios sin instalar nada ni añadir dependencias. Además, los tests de este proyecto comparan valores pequeños —un número, una lista o un `Result`—, de modo que el mensaje de `assert_eq!` es suficiente para entender un fallo.

Lo que se asume con esta decisión es que, si hubiera que comparar estructuras grandes, los mensajes podrían ser menos legibles; y que no se dispone de reintentos automáticos ni de informes JUnit o JSON.

Se reconsiderará si:
- Aparecen comparaciones de estructuras grandes: entonces `pretty_assertions` puede activarse añadiendo una línea por módulo.
- La suite se vuelve lenta o aparecen tests inestables, por ejemplo si en un objetivo futuro se usa red real: entonces se considerará `cargo-nextest`.
- Aparecen muchos tests con la misma forma: entonces se valorará `rstest`.

Respecto al objetivo 5, `cargo test` solo escribe en el directorio de compilación, que se cambia con `CARGO_TARGET_DIR`; con `--locked`, además, no modifica `Cargo.lock`.

### Niveles de test
- **Tests unitarios.** Están en el módulo `tests` de cada fichero de `src/`. Prueban un comportamiento concreto de una pieza: qué avistamientos se cuentan, qué especies depredan a otra, cómo se encuentra una especie por su nombre común, si unas coordenadas caen dentro de una zona o cómo se reúnen avistamientos de varias fuentes. Ahí vive la lógica de negocio de cada issue, y se prueba sin ficheros ni el resto del sistema; si falla, se sabe qué pieza es.
- **Tests de integración.** Están en `tests/hu001.rs` y usan la página guardada en `tests/fixtures/inaturalist/vison.html`. Comprueban que una página real de iNaturalist acaba produciendo el registro correcto, con su fecha y su lugar. Lo que puede romperse no es la lógica interna, sino el formato de una página externa que no controlo; solo una página real lo detecta.
- **Tests de historia de usuario.** También están en `tests/hu001.rs`. Ejercitan `consultar_zona` de principio a fin con la API pública: el caso feliz y los casos en los que no lo es, como estar fuera de la ventana temporal, fuera de la zona o consultar una especie sin depredadores. Es el criterio de aceptación de la HU001: comprueba que las piezas, juntas, resuelven el problema del pescador.

No hay tests que usen red ni navegador: los datos de prueba son páginas guardadas, para que los tests sean rápidos y repetibles. Tampoco se prueban getters, formatos de mensajes ni comportamientos que ya garantizan los tipos.

### Organización de los tests
- **Tests unitarios:** dentro de cada fichero de `src/`, en un módulo `tests` con `#[cfg(test)]`. Así pueden probar también funciones privadas.
- **Tests de integración:** en `tests/`, usando solo la API pública. `tests/hu001.rs` contiene el test del lector de iNaturalist y los de la HU001.
- **Datos de prueba:** la página guardada de la observación de un visón está en `tests/fixtures/inaturalist/vison.html`.

### Principios F.I.R.S.T.
- **Fast:** los tests no usan red ni más disco que la página guardada, que se incluye al compilar con `include_str!`. Tardan milisegundos.
- **Independent:** cada test construye sus propios datos y no depende del resultado ni del orden de ejecución de otro.
- **Repeatable:** ningún test lee el reloj —la fecha de referencia se pasa como parámetro—, usa la red o depende del orden de un `HashMap`; los resultados se devuelven ordenados.
- **Self-validating:** cada test pasa o falla mediante una aserción, sin necesidad de mirar manualmente ninguna salida.
- **Timely:** cada issue se cierra con el commit que añade tanto su código como su test, y el commit enlaza el issue.

### Ejecución de los tests
Para ejecutar todos los tests desde cero:

```bash
just test
```

Para usar otro directorio de compilación, sin tocar `Cargo.lock`, como pide el objetivo 5:

```bash
CARGO_TARGET_DIR=/tmp/biovigia-build cargo test --locked
```

Para ejecutar solo un fichero de test:

```bash
cargo test --test hu001
```

Para ejecutar solo los tests cuyo nombre contenga una palabra:

```bash
cargo test cuentan
```

### Configuración
La clave `test` de `iv.yaml` apunta a `tests/hu001.rs`, que prueba tanto la lectura de la página real de iNaturalist como la historia de usuario completa, con casos felices y casos en los que la consulta no produce un resultado válido.