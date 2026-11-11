# Herramientas de tests

Esto es lo que he entendido que hace cada cosa:
- Biblioteca de aserciones: compara lo obtenido con lo esperado y falla con un mensaje si no coinciden.
- Test runner: descubre los tests, los ejecuta y presenta el resultado.

Los criterios que fijo a la hora de decidir son:
- Sencillez de implementar.
- Que funcionen y se puedan implementar sin ningun tipo de carencia en WSL y Linux.
- Que no dependa de APIs de nadie.
- Que los errores se expresen claramente y sepa que test/s han fallado y en que parte.
- Que a ser posible, este documentado personalmente por rust.

Tanto para la biblioteca de aserciones como para el test runner analizaré unica y exclusivamente todos los candidatos que considere oportunos para cada uno. Considero que es una muestra totalmente razonable y suficiente para una solida conclusion y eleccion por mi parte.

Todas las decisiones han sido basadas bajo mi criterio, y, basandome en las fuentes oficiales de la documentacion de rust.

## Biblioteca de aserciones
Voy a clasificar en dos grupos:

### Estandares
Los que evaluare son:
- [assert!](https://doc.rust-lang.org/std/macro.assert.html): afirma que una variable booleana es true, si no se cumple la asercion, salta la asercion.
  - Es bastante sencilla de usar, simplemente es poner `assert!(variable);`
  - Honestamente considero que si que la voy a usar en un momento u otro si que me interesará poder consultar algo tan trivial.
  - La documentacion me cita que puedo hacer custom de un mensaje de panic cuando salte la asercion, lo cual es bastante bueno, porque de no ponerlo te salta un error poco descriptivo.
  - El mensaje custom del panic se puede personalizar simplemente precediendo la asercion boolena de una coma y del mensaje, por ejemplo: `assert!(edad >= 18, "El usuario debe ser mayor de edad")`
- [assert_eq!](https://doc.rust-lang.org/std/macro.assert_eq.html): afirma que dos variables son iguales, si no se cumple esta igualdad, salta la asercion.
  - Como requisito para usarlo debes haber implementado un struct o enum el `PartialEq`, cosa que ya tengo definido en mi dominio en lo necesario.
  - Es sencillo de usar, basta con simplemente poner `assert_eq!(a, b);`.
  - Es mas que evidente que necesito usar esta funcionalidad en mi proyecto, para ver por ejemplo, cuando extraiga datos del HTML de iNaturalist si ha extraido los datos correctamente y compararlos uno a uno con los hardcodeados que yo estime conveniente.
  - De nuevo la documentacion me cita que puedo hacer custom de un mensaje de panic, que se hace de igual manera que en assert, por ejemplo: `assert_eq!(n1, n2, "n1 y n2 han de ser iguales");`
- [assert_ne!](https://doc.rust-lang.org/std/macro.assert_ne.html): es justo todo lo contrario que assert_eq, sino que espera que dos valores no sean iguales, las ventajas son las mismas que presenta el otro.
- [debug_assert!](https://doc.rust-lang.org/std/macro.debug_assert.html): parece lo mismo que assert! pero no lo es, assert! se ejecuta siempre y debug_assert! solo cuando yo programo. 
  - Para que solo salte cuando yo programo y no en produccion o para el usuario a la hora de compilar tienes que meter el flag `--release` al compilar, de esta forma no saltará porque el compilador borrará esas instrucciones del binario resultante.
  - Como requisito para usarlo debes haber implementado un struct o enum el `Debug`, cosa que ya tengo definido en mi dominio en lo necesario.
  - Este programa mas allá de esta asignatura no tengo pensado que en un principio esté destinado a ningún usuario, menos aún a "producción", por lo que no tiene sentido que vaya a usar ninguna biblioteca de aserciones que vaya precedida de la palabra "debug_".
- [debug_assert_eq!](https://doc.rust-lang.org/std/macro.debug_assert_eq.html): igual que assert_eq! pero con la funcionalidad de "debug_"
  - Inservible dadas las circunstancias de este proyecto y su alcance fuera de esta asignatura.
- [debug_assert_ne!](https://doc.rust-lang.org/std/macro.debug_assert_ne.html): igual que assert_ne! pero con la funcionalidad de "debug_"
  - Inservible dadas las circunstancias de este proyecto y su alcance fuera de esta asignatura.

### Externas
Los que evaluare son:
- [pretty_assertions](https://docs.rs/pretty_assertions/latest/pretty_assertions/): Te dicen que si usas mucho assert_eq! llegaras a la conclusión de que te salta la aserción pero de que tienes que ver tu mismo las diferencias una a una.
  - Se usa de forma sencilla simplemente poniendo arriba del código `use pretty_assertions::{assert_eq, assert_ne};`, de tal forma que sobreescriben ambos assert.
  - Es una dependencia externa pero me da igual, es solo poner en `Cargo.toml` una línea que diga `pretty_assertions = "1.4.1"` con esto encima `[dev-dependencies]`. No es mi problema porque esto se lo delego al gestor de dependencias que ya escogí y decidí con criterio en el objetivo pasado, Cargo.
  - La ventaja que te ofrece es que te las recalca de colores igual que si usas una IA para que toque tu codigo, que ves los cambios en rojo y en verde, es exactamente lo mismo.
  - Podría estar interesante en mi proyecto pero no considero que sea el caso, creo que está mas ligado a objetos que tienen JSONs enormes donde si que se pueda apreciar y valorar más claramente esta comparación.
  - Si me liara a poner campos y campos en el dominio y escalaran en cantidad de atributos quizás si se podría considerarlo, pero de momento no. 
- [googletest](https://docs.rs/googletest/latest/googletest/): ofrece una barbaridad, con todas las letras, de tipos de macros tal y como se puede ver al final de la cita.
  - La diferencia radica en eso, más expresividad que por el momento no necesito, pienso que me puede llegar a agobiar tener tantas opciones disponibles que no voy a usar en una fase tan temprana del proyecto como en la que estoy.
  - Como acabo de mencionar, la expresividad no se queda ahí, también te deja a diferencia del panic de todos los de arriba estudiados hasta ahora, poder hacer aserciones no fatales, o sea que se muestren por pantalla pero que no se corte el test cuando estas aparecen, que el test siga corriendo. A ver, esto lo veo útil si tienes una ingente cantidad de tests en tu CI y quieres testear módulos independientes sin frenarte solo cuando uno de ellos falla sin llegar al otro. Igualmente, voy a  tener pocos tests y no voy a tener que pasarme la vida esperando por tener CI precisamente enormes como para tener que necesitar hora mismo estas aserciones no fatales.
  - Es una dependencia externa pero me da igual, es solo poner en `Cargo.toml` una línea que diga `googletest = "0.14.4"` con esto encima `[dev-dependencies]`.
- [assertor](https://docs.rs/assertor/latest/assertor/): Se basa en encadenar métodos con punto "." en lugar de anidar funciones.
  - Cuando algo falla te desglosa exactamente qué faltaba y qué sobraba en un "aparentemente" (entrecomillado porque es subjetivo) formato muy limpio.
  - Es una dependencia externa pero me da igual, es solo ejecutar `cargo add --dev assertor` o hacerlo como puse mas arriba en googletest, meterlo tu a pelo en el `Cargo.toml`, pero sencillamente la primera opción es más cómoda y automática.
  - Tienes que poner arriba del código para usarla simplemente `use assertor::*;`
  - A ver, este puede estar bastante curioso porque sigue siendo todo one liner, por ejemplo `assert_that!("foobarbaz").contains("bar");`, si es cierto que es mucho mas limpio que encadenar funciones y según lo que he visto usas el assert_that! absolutamente para todo, no tienes que decidir entre los 3 nativos (assert!, assert_eq! y assert_ne!)
  - Creo que es el más interesante comentado hasta ahora y si que tiene un uso claro y real en mi proyecto ahora mismo.
- [more-asserts](https://docs.rs/more-asserts/latest/more_asserts/): Es la que tengo más clara que no pienso usar.
  - Básicamente sale del circulo de assert!, assert_eq! y assert_ne! introduciendote otras nuevas que visualmente a la hora de programar si que son correctas pero completamente irrelevantes porque sus funcionalidades ya se pueden hacer usando las 3 anteriores. Algunos ejemplos son assert_le!, assert_lt!, etc... igual que expanden este repertorio, como no podia ser de otra forma, tambien incluyendo debug_ de acordes a estos.
  - No pienso perder el tiempo ni buscando como se instala la dependencia.
  - Personalmente discrepo un poco su mera existencia, pero llego a comprenderla.

### Eleccion
**Me quedo con assertor**, es una nueva que no he usado hasta ahora en ningun proyecto mío personal, la veo bastante interesante, es la que más se ajusta a mi proyecto y me da esa libertad mínima pero no excesiva (como googletest) para poder construir los test como yo personalmente considero más claros y one liners, lo cual facilita la revisión de este trabajo por parte del corrector. Además me dice mucho más claro que falla, esto me es irrelevante, pero es un plus que me llevo de la mano junto con la expresividad. Además, esta decisión es una que personalmente quiero experimentar.

## Test runner
Nuevamente, clasifico en dos grupos, aunque en este caso, en rust no hay gran cosa:

### Estandares
Los que evaluare son:
- [cargo test](https://doc.rust-lang.org/cargo/commands/cargo-test.html): Es la clásica y la que se suele usar por estandar y ya viene instalada.
  - Se ejecuta como `cargo test [options] [testname] [-- test-options]`
  - En la descripcion de lo que cito que es el test runner, menciono que descubre test, para que los encuentre tienes que poner arriba de su linea `#[test]` o no te los encuentra.
  - Trae paralelismo por defecto, pero es en las funciones de test y no en los archivos.
  - Por defecto también te ejecuta todos los archivos sueltos enteros bajo la carpeta `tests/` y los [doctests](https://www.luisllamas.es/rust-documentacion-doctests/)
- [libtest](https://doc.rust-lang.org/test/index.html): Curioso que a la hora de analizar este me encuentre [un repositorio](https://github.com/rust-lang/libtest) donde dice haber sido archivado hace cinco años, pero tras investigarlo, he visto que no está abandonado, de hecho, se archivó porque su codigo fuente fue integrado directamente dentro del repositorio principal del compilador de rust.
  - Es una API, y esta en nightly... lo único que me da el nightly que vea relevante y digno de mencionar es que tu te puedes construir tu runner personalizado en tiempo de ejecucion usando `test_main` y `TestDescAndFn` pasandole en tiempo de ejecución la lista de los test.
  - Este si que trae concurrencia total por defecto a la hora de correr los test.
  - Te intercepta los print que tu hayas puesto en cada tets para que no se mezclen mientras corren en paralelo y solo te los muestra si falla el tets o si le metes el flag `--no-capture`, cosa que creo que es hasta molesto tener que usar un flag constantemente, bajo mi opinión personal. Me interesaría no usarle si tengo quinientos test, pero eso no va a ocurrir, y menos en este proyecto dado su alcance.
  - Al final te hace un breve resumen con cuantos han salido bien, cuantos han fallado, cuales han sido ignorados y los tiempos. Una cosa que no hace cargo test, lo unico que hace este es dejar pasar a mi terminal la salida de libtest.
  - A ver, es una API pero es interna del compilador, y no tengo que depender de ella ni importarla nunca, pero está en nightly y no me apetece descubrir posibles nuevos errores no documentados por estar atribuidos a una fase de desarrollo.

### Externos
El único que evaluaré es:
- [cargo-nextest](https://nexte.st/docs/design/how-it-works/): Es un binario externo que se baja de forma bastante sencilla con `curl -LsSf https://get.nexte.st/latest/linux | tar zxf - -C ${CARGO_HOME:-~/.cargo}/bin`
  - Sigue necesitando poner `#[test]`
  - Si le quisiera usar, el cambio seria minimo, en el justfile, en test, en vez de `cargo test` se pone `cargo nextest run`
  - La idea es bastante particular, original y curiosa desde mi punto de vista, lo que hace es usar el Cargo para compilar los binarios de prueba y consultales a cada uno de ellos la lista completa de tests disponibles para después, planificar y lanza **un** proceso del sistema operativo separado por cada función en paralelo. Que tras investigar, esta forma que puede parecer más lenta es en realidad hasta más rápida, pero al tener pocos test en mi proyecto no creo que me pueda beneficiar apropiadamente ce esta ventaja y peculiar forma de hacer las cosas.
  - Trae una cosa clave que cargo test no te da (no para mí desde luego), pero que si considero relevante señalar y es que hace retries de test que puedan necesitarlo por  razones como esperar a que x servicio este levantado e ir probando hasta que lo esté. Configuras los reintentos de forma sencilal con el flag `--retries`
  - Tendria que instalar muchas dependencias pero no tocaría el código, solo actúa como ejecutor y fuera.
  - Hay otra cosa importante, cargo test si soporta doctests pero nextest [no los soporta](https://nexte.st/docs/running/)
  - De igual forma que libtest tare un resumen al final de los test.
  - Tambien trae paralelismo y de forma mucho mas superior al de libtest, los aplica por falta de uno, dos niveles:
    - Proceso por test.
    - Entre los binarios.

### Eleccion
**Me quedo con cargo-nextest**, de nuevo, quiero experimentar cosas nuevas fuera del consumo colectivo de cualquier ingeniero de rust, además de que si van más rápido con lo impaciente que soy y además me parece bastante curiosa y eficiente lo de crear un proceso para cada función de test que pueda consultar en tiempo real con `ps aux` me parece fascinante. O sea, voy a tener que seguir poniendo `#[test]` y creo que aquí lo más razonable es ir por el camino seguro, no tirar por nightly y razonar bajo mi criterio que en este caso se ha basado en la eficiencia y preferencia personal.