# Herramientas de test
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

## Bibliotecas de aserciones
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

## Test runners
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
  
### Motores de test personalizados en Rust
Rust también permite definir motores de test personalizados, pero he visto que esta funcionalidad está disponible en nightly y que ademas es experimental porque esta en desarrollo y no es estable, aunque tiene definido un RFC.

Por este motivo no la voy a utilizar en este proyecto, prefiero emplear una toolchain estable ([documentación oficial](https://doc.rust-lang.org/unstable-book/language-features/custom-test-frameworks.html)) que a poder tener errores que ni sus desarrolladores han contemplado aun por estar en fase experimental.

## Decisión
Me quedo con las aserciones de la biblioteca estándar y cargo test como runner.

Es la combinación que cumple los cinco criterios sin instalar nada ni añadir dependencias: la biblioteca estándar es de sentido común y la usaría cualquier persona, cargo test ya lo estoy usando y funciona, por lo que no tengo razón para cambiarlo, los tests de este proyecto comparan valores pequeños (un número, una lista o un result) y el mensaje de assert_eq! basta para entender un fallo.

Respecto al objetivo 5, cargo test solo escribe en el directorio de compilación, que se cambia con CARGO_TARGET_DIR. Con --locked, además, no modifica Cargo.lock.

### Decisiones de cara al futuro
- Lo que asumo con esta decisión es que, si en el futuro aparecen comparaciones de estructuras grandes, pretty_assertions puede activarse añadiendo una línea por módulo.
- Si la suite se vuelve lenta, si aparecen tests inestables o si el tiempo de CI se convierte en un problema real, se considerará cargo-nextest por el tema de la concurrencia.
- Si aparecen muchos tests con la misma forma, se valorará muy seriamente rstest.

No es mi intención convertir esto en un producto completamente real más allá de esta asignatura, por eso no quiero añadir herramientas por si acaso.

Si el proyecto creciera hasta tener una integración continua grande, con muchos tests y tiempos de espera demasiado altos, entonces sí empezaría a tener sentido valorar cargo-nextest por su concurrencia y sus informes. Pero ahora mismo no es el caso.


## Fuentes consultadas
- [Cargo: ejecución de tests](https://doc.rust-lang.org/cargo/commands/cargo-test.html)
- [Cargo: organización y tipos de tests](https://doc.rust-lang.org/cargo/reference/cargo-targets.html)
- [pretty_assertions](https://docs.rs/pretty_assertions/latest/pretty_assertions/)
- [googletest](https://docs.rs/googletest/latest/googletest/)
- [cargo-nextest](https://nexte.st/)
- [rstest](https://docs.rs/rstest/latest/rstest/)
- [Custom test frameworks: Rust Unstable Book](https://doc.rust-lang.org/unstable-book/language-features/custom-test-frameworks.html)


# Aplicación de la estrategia de testing

## Niveles de test
- **Tests unitarios.** Están en el módulo `tests` de cada fichero de `src/`. Prueban un comportamiento concreto de una pieza: qué avistamientos se cuentan, qué especies depredan a otra, cómo se encuentra una especie por su nombre común, si unas coordenadas caen dentro de una zona o cómo se reúnen avistamientos de varias fuentes. Ahí vive la lógica de negocio de cada issue, y se prueba sin ficheros ni el resto del sistema; si falla, se sabe qué pieza es.
- **Tests de integración.** Están en `tests/hu001.rs` y usan la página guardada en `tests/fixtures/inaturalist/vison.html`. Comprueban que una página real de iNaturalist acaba produciendo el registro correcto, con su fecha y su lugar. Lo que puede romperse no es la lógica interna, sino el formato de una página externa que no controlo; solo una página real lo detecta.
- **Tests de historia de usuario.** También están en `tests/hu001.rs`. Ejercitan `consultar_zona` de principio a fin con la API pública: el caso feliz y los casos en los que no lo es, como estar fuera de la ventana temporal, fuera de la zona o consultar una especie sin depredadores. Es el criterio de aceptación de la HU001: comprueba que las piezas, juntas, resuelven el problema del pescador.

No hay tests que usen red ni navegador: los datos de prueba son páginas guardadas, para que los tests sean rápidos y repetibles. Tampoco se prueban getters, formatos de mensajes ni comportamientos que ya garantizan los tipos.

## Organización de los tests
- **Tests unitarios:** dentro de cada fichero de `src/`, en un módulo `tests` con `#[cfg(test)]`. Así pueden probar también funciones privadas.
- **Tests de integración:** en `tests/`, usando solo la API pública. `tests/hu001.rs` contiene el test del lector de iNaturalist y los de la HU001.
- **Datos de prueba:** la página guardada de la observación de un visón está en `tests/fixtures/inaturalist/vison.html`.

## Principios F.I.R.S.T.
- **Fast:** los tests no usan red ni más disco que la página guardada, que se incluye al compilar con `include_str!`. Tardan milisegundos.
- **Independent:** cada test construye sus propios datos y no depende del resultado ni del orden de ejecución de otro.
- **Repeatable:** ningún test lee el reloj —la fecha de referencia se pasa como parámetro—, usa la red o depende del orden de un `HashMap`; los resultados se devuelven ordenados.
- **Self-validating:** cada test pasa o falla mediante una aserción, sin necesidad de mirar manualmente ninguna salida.
- **Timely:** cada issue se cierra con el commit que añade tanto su código como su test, y el commit enlaza el issue.

## Ejecución de los tests
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

## Configuración
La clave `test` de `iv.yaml` apunta a `tests/hu001.rs`, que prueba tanto la lectura de la página real de iNaturalist como la historia de usuario completa, con casos felices y casos en los que la consulta no produce un resultado válido.