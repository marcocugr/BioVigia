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
La justificación también está en [docs/objetivos/objetivo-4/justificacion-herramientas-test.md](./docs/objetivos/objetivo-4/justificacion-herramientas-test.md) y en el issue [#60](https://github.com/marcocugr/BioVigia/issues/60).

Antes de escribir tests había que decidir qué herramientas usar y como no podia ser de otra forma, no todas hacen lo mismo.
Una cosa es comprobar que el resultado obtenido coincide con el esperado.
Otra es descubrir y ejecutar los tests.
Y otra es preparar datos o repetir un test con varias entradas.

Esto es lo que he entendido que hace cada cosa:
- Biblioteca de aserciones: compara lo obtenido con lo esperado y falla con un mensaje si no coinciden. En Rust, assert!, assert_eq! y assert_ne! vienen con la biblioteca estándar, pero también existen otros como pretty_assertions y googletest.
- Test runner: descubre los tests, los ejecuta y presenta el resultado. En Rust, Cargo ya incluye uno con cargo test, aunque hay otra opción que es cargo-nextest.
- Fixtures y tests con parámetros: sirven para preparar datos de prueba o repetir un test con varias entradas. No sustituyen ni a las aserciones ni al runner. En Rust, rstest es una opción habitual.

No tenía ni idea de dónde buscar este tipo de cosas ni qué opciones existían. Recurrí a [Perplexity](https://www.perplexity.ai/es/hub/blog/getting-started-with-perplexity) para que me recomendara fuentes fiables y me indicara qué candidatos debia eplorar por MI cuenta. Perplexity me citó dichas fuentes para que yo mismo evaluara mis propias conclusiones sobre las cosas, lo cual es algo muy distinto de que haya decidido por mí ni que haya condicionado mi razonamiento.

Los criterios que fijé antes de elegir fueron estos.
1. Que viniera con el lenguaje, para que funcionara igual en WSL, Docker y en la máquina de quien corrija.
2. Que no escribiera en el directorio de los fuentes, salvo en el de compilación, y que ese directorio se pudiera cambiar. Esto lo exige el objetivo 5.
3. Que al fallar un test el mensaje mostrara el valor esperado y el obtenido.
4. Que sirviera para tests unitarios y de integración.
5. Que funcionara con la toolchain estable y la edición 2024 que fija rust-toolchain.toml.

Las alternativas que examiné salen de nexte.st, pretty_assertions y googletest.

#### Bibliotecas de aserciones
- assert!, assert_eq! y assert_ne!: son las aserciones de la biblioteca estándar.
  - Sus propios nombres ya denotan sentido común con las expectativas que uno trae.
  - Comprueban si algo es igual a otra cosa o no lo es, punto.
  - Sin líos ni complicaciones y además ya viene con el lenguaje, así que no me tengo que comer la cabeza con dependencias de los demás ni historias raras.
  - Es lo justo, lo mínimo y lo necesario para este proyecto, en este punto.
- pretty_assertions: sobrescribe assert_eq! y assert_ne! y añade un diff coloreado.
  - Al principio pensé que su única diferencia con los de arriba era poner colores. Después de leer su documentación, veo que no es exactamente así.
  - Lo que aporta de verdad es un diff más legible de las diferencias, lo cual puede tener sentido cuando comparas estructuras grandes, listas largas o textos multilínea.
  - En mi caso, los valores que comparo son pequeños: un número, una lista corta o un result. El mensaje normal de assert_eq! ya me dice qué esperaba y qué he obtenido.
  - Por eso, ahora mismo no veo que esa mejora justifique añadir una dependencia, o sea no es que la herramienta no aporte nada, directamente es que su ventaja no encaja con mi problema actual.
- googletest: ofrece matchers combinables con assert_that! y expect_that!. Cubre mucho más que la igualdad simple.
  - También permite aserciones no fatales, de modo que un test puede registrar varios fallos sin parar en el primero (cosa bastante util por cierto), esot puede ser útil en un dominio con muchas reglas o validaciones complejas, el cual precisamente no es el caso de mi proyecto.
  - Lo que compruebo es bastante directo: que una especie existe, que una especie es invasora, que una consulta devuelve los avistamientos esperados, que un result contiene el error correcto, etc...
  - Para eso, las aserciones nativas son más simples y me bastan.
  - Además, es una dependencia externa y, en el momento de elegir, su API no era estable hasta la versión 1.0. No quiero ni de broma depender de algo más complejo y más aún cuando todavía no necesito esa expresividad.

#### Test runners
- cargo test: ya lo estoy usando en el justfile.
  - Para qué voy a cambiarlo, es que simplemente por qué iría yo a cambiarlo. Lo veo perfecto tal y como está ahora. Viene con Cargo, no hay que instalar nada extra, y cubre todo lo que necesito.
  - Ejecuta tests unitarios, de integración y doctests.
  - También permite filtrar por nombre o por fichero.
  - No tiene reintentos, pero eso para mí, no es ni de lejos ni una carencia ni una desventaja como se presenta en internet: mis tests no dependen de una API externa, no usan red ni tienen llamadas que puedan fallar por algo transitorio y si un test falla, lo más probable es que el código o el test sea el problema, por lo que no necesito que se reintente automáticamente.
- cargo-nextest: ejecuta cada test en un proceso independiente.
  - Admite reintentos y genera informes en JUnit o JSON.
  - También mejora el aislamiento entre tests y puede ejecutarlos de forma más paralela.
  - Hay que instalarlo, vale, eso de nuevo no es ningún problema porque ya viene con cargo por lo menos, la cosa es que cubre todo eso más reintentos, aislamiento y no sé qué historia.
  - De nuevo, el tema de reintentos no me aporta nada en este momento, con lo cual, para añadir más dependencias al proyecto sin necesidad, mejor que se quede donde está, vamos que no es mi caso, no digo que sea una mala herramienta.
  - Si la suite creciera, si hubiera tests lentos, inestables o si necesitara informes para integración continua, probablemente tendría sentido considerarlo.
- rstest directamente es que ni compite con las anteriores.
  - Añade fixtures y tests con parámetros sobre cargo test y además es una dependencia externa.
  - Al principio pensé que podía encajar con lo de “when I”, “given I” y “then”, pero no es exactamente eso, es es más bien una forma de escribir escenarios de aceptación, normalmente asociada a BDD y a herramientas como Cucumber (que ni sabia lo que era ni de su existencia hasta lo leí). En Python sí he usado ese tipo de enfoque y si fuese así aquí, lo usaría sin dudarlo, el problema es que en rstest la idea es otra.
  - Lo que hace rstest es permitir definir fixtures y tests parametrizados. Lo que he entendido es que un fixture es una función que prepara algo que varios tests necesitan, por ejemplo, unos datos de prueba, una configuración o un objeto que se reutiliza. Después, ese fixture se pasa como argumento al test, ahorrándote repetir la misma preparación test tras test, o sea es como una especie de set up.
  - Podria estar interesante a la hora de meter cosas en el catalogo y en las fuentes de la verdad para que varios tests trabajen sobre esa configuración común.
  - También permite escribir un test una vez y ejecutarlo con varios casos, tu cojes y pones los casos en una tabla y rstest genera un test independiente por cada uno. Por ejemplo, puedes comprobar una función con varias entradas y sus resultados esperados sin repetir el mismo código una y otra vez, algo parecido toqué en mis practicas de Constella.
  - Pero ahora mismo no tengo ese tipo de tests en el repositorio, los casos que pruebo son suficientemente distintos como para que escribirlos de forma explícita sea más claro. Si más adelante aparecen muchos tests con la misma forma, por ejemplo muchas zonas, especies o ventanas temporales que se comprueban igual, sí valoraré rstest.

#### Decisión
Me quedo con las aserciones de la biblioteca estándar y cargo test como runner.

Es la combinación que cumple los cinco criterios sin instalar nada ni añadir dependencias: la biblioteca estándar es de sentido común y la usaría cualquier persona, cargo test ya lo estoy usando y funciona, por lo que no tengo razón para cambiarlo, los tests de este proyecto comparan valores pequeños (un número, una lista o un result) y el mensaje de assert_eq! basta para entender un fallo.

Respecto al objetivo 5, cargo test solo escribe en el directorio de compilación, que se cambia con CARGO_TARGET_DIR. Con --locked, además, no modifica Cargo.lock.

##### Decisiones de cara al futuro
- Lo que asumo con esta decisión es que, si en el futuro aparecen comparaciones de estructuras grandes, pretty_assertions puede activarse añadiendo una línea por módulo.
- Si la suite se vuelve lenta, si aparecen tests inestables o si el tiempo de CI se convierte en un problema real, se considerará cargo-nextest por el tema de la concurrencia.
- Si aparecen muchos tests con la misma forma, se valorará muy seriamente rstest.

No es mi intención convertir esto en un producto completamente real más allá de esta asignatura, por eso no quiero añadir herramientas por si acaso.

Si el proyecto creciera hasta tener una integración continua grande, con muchos tests y tiempos de espera demasiado altos, entonces sí empezaría a tener sentido valorar cargo-nextest por su concurrencia y sus informes. Pero ahora mismo no es el caso.

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