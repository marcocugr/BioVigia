# Elección de la imagen base

## Criterios 
- Me tiene que poder dejar usar rust 1.99
- Tener cargo si o si incluido
- Tiene que tener o bien dejarme instalar just y nextest
- Tiene que tener repo publico donde pueda ver si la imagen es de verdad segura y no esta comprometida
- Me tiene que dejar poder bajar las dependencias que necesito, desde crates.io
- Tener SOLO lo justo, lo minimo y lo necesario en cuanto a rust y herramientas del SO.

### Que necesito
- nextest
- just
- cargo, con version 1.99 del compilador
- dependencias de cargo.toml (por el momento solo assertor)

### Como evaluaré cada candidata
Me haré 4 preguntas para evaluar a cada una:
- ¿Quién mantiene la imagen?
- ¿Qué sistema operativo utiliza?
- ¿Qué paquetes adicionales contiene?
- ¿Qué herramientas de compilación incluye?

### Evaluacion
Me da igual que sea una tabla, no esta generada por un engranaje de ejecucion:

| Imagen base | SO | Repo trazable? | De lo que tengo, que tiene | De lo que tengo, que no tiene | Como podria bajar lo que no tiene | Quien la mantiene | Tiene muchos paquetes adicionales completamente innecesarios? | Compila la 1.99? |
|--------------|--------------|--------------|--------------|--------------|--------------|--------------|--------------|--------------|
| [rust](https://hub.docker.com/_/rust) | Debian (Bookworm o Trixie, ambos pueden ser slim o no) o Alpine | [Si](https://github.com/rust-lang/docker-rust) | rust y cargo | just, nextest y las dependencias del proyecto | just con una sola linea, nextest puede instalarse con su instalador oficial (otra linea) y las dependencias (assertor) ejecutando cargo fetch | Rust Project developers | **Depende de la variante:**  La normal contiene “un gran número de paquetes Debian extremadamente comunes”, por lo que sí. La slim contiene solo los paquetes mínimos para ejecutar Rust. Alpine es aún más pequeña, pero usa musl libc y suele omitir herramientas como git o bash. | Si |
| [lukemathwalker/cargo-chef](https://hub.docker.com/r/lukemathwalker/cargo-chef) | Hereda la variante de rust oficial: Debian Bookworm/Trixie o Alpine | [Si](https://github.com/LukeMathWalker/cargo-chef) | rustc, cargo y cargo-chef preinstalado | just, cargo-nextest y las dependencias del proyecto | Igual que la celda de arriba | LukeMathWalker y la comunidad, al fin y al cabo es proyecto de terceros, pero activamente mantenido | **Si, pero poco:** Lo que te meten es [cargo-chef](https://docs.fly.io/rust/the-basics/cargo-chef), que sirve para cachear dependencias rust en builds de docker multietapa. | Si |
| [cimg/rust](https://hub.docker.com/r/cimg/rust) | Ubuntu | [Si](https://github.com/CircleCI-Public/cimg-rust) | rust y cargo, pero en algunas variantes de CI incluye utilidades extra como nextest | just, nextest (salvo que el tag concreto lo incluya) y las dependencias del proyecto | Igual que la celda de arriba | CircleCI | **Si, y bastantes** porque es una "convenience image" de CI que te incluye: git, ssh, curl, jq, y otros componentes de Ubuntu que soncompletamente irrelevantes para compilar un proyecto de rust | Si |
| [ekidd/rust-musl-builder](https://hub.docker.com/r/ekidd/rust-musl-builder/) | Ubuntu | [Sí](https://github.com/emk/rust-musl-builder) | Rust, Cargo, musl libc, compiladores de C y de C++ y librerías C estáticas orientadas a producir binarios estáticos | just, cargo-nextest y las dependencias del proyecto | Igual que en la celda de arriba | Terceros y la comunidad, peor vamos que la imagen publicada no se actualiza desde hace más de cuatro años | Si: está orientada a compilación cruzada y binarios estáticos, nada mas tienes que ver que te trae compiladores de C y de C++, librerías estáticas, OpenSSL, git, scripts propios y otras herramientas innecesarias para un build normal | **No garantizado:** la imagen publicada está desactualizada y sus versiones de rust/toolchain no son fiables para compilar Rust 1.99, de hecho, no se recomienda. Vamos que está abandonado. |


## Decision
Si resumimos cada una basicamente:
- **rust**: Por partes:
  - Debian: es la opcion más adecuada si quieres compilar sin complicaciones, parte de lo mínimo, mantienes compatibilidad con glibc y solo añades just, nextest y las dependencias del proyecto. La variante normal (la que no es slim) solo tendría sentido si necesitaras muchas herramientas del sistema y no quisieras instalarlas tu.
  - Alpine: es la más ligera, usa musl en lugar de glibc, y eso puede complicar la compilación o el comportamiento de algunas dependencias. Además que según he visto la ganancia de tamaño no justifica ese riesgo si tu prioridad es compilar rust 1.99 de forma sencilla y reproducible.
- **lukemathwalker/cargo-chef**: Interesante, pero lo unico que me trae es el cargo-chef que sirve para cachear dependencias y evitar recompilarlas constantemente, en mi caso, el proyecto es pequeño y como ya he mencionado otras veces, no voy a  desarrollarle mas alla de estas asignaturas, por lo que no puedo nutrirme de sus beneficios reales.
- **cimg/rust**: No voy a usar git en un docker, menos aun ssh y tiene componentes de ubuntu que me dan completamente igual, sobran y no tienen que ver nada con el proyecto. Vamos, que tiene mucha basura.
- **ekidd/rust-musl-builder**: Abandonada y obsoleta, no puedo fijar 1.99, no la mantiene oficialmente rust, y tiene una cantidad de basura que me es completamente irrelevante como cosas de C y C++.

Teniendo todas las cartas sobre la mesa, la idea que se me ocurre es de la imagen oficial, bajarme el debian slim, meterle 3 lineas de instalacion correspondientes a las dependcias, a just y a nextest y tendria lo justo, lo minimo y lo necesario en relativo a rust para mi proyecto.

Ahora bien, date cuenta que he dicho de rust y no del sistema, porque del sistema no es minimo, pero en este caso, como tambien dije antes, prefiero NO jugarmela y poder dormir tranquilo sabiendo que puedo compilar sencillamente y se va a reproducir bien en el docker.

Pero tambien surje una pregunta, hay cosas del SO que puede que sean innecesarias en esta slim de Debian y que por tanto no me hagan falta, es aqui donde quiero valorar mas seriamente un Alpine, siempre y cuando me pueda recortar aun mas estas cosas innecesarias SIN dar problemas. De hecho usa busybox, no estaria usando ni bash, pero, segun lo que he visto, busybox es el estandar para contenedores ligeros.

Sabiendo que ninguna imagen pública va a venir (casi nunca, por no decir nunca) hecha a medida exactamente para mi Cargo.toml con solo assertor, just y nextest sin meter "basura" por otro lado, siempre voy a  tener que meterle cosas a las imagenes, partiendo de ahi, prefiero tener pocas cosas que sean innecesarias como es el caso de debian a tener unicamente lo justo y que eso conlleve posibles problemas que no tendría que tener, mi prioridad aquí como dije en los criterios no es optimizar hasta el mas mínimo byte, sino quitar muchisimas cosas innecesarias pero poder trabajar cómodo y sin problemas.

Finalmente y tras este analisis, me quedo con debian slim

Ahora bien, trixie o bookworm? Porque he mencionado dos, voy a analizarlas:
- **[Trixie](https://www.debian.org/releases/trixie/)**: Es la versión estable actual, con lo cual es ideal para proyectos nuevos por tener librerías más modernas. La ultima version ademas fue ayer 21 de noviembre.
- **[Bookworm](https://www.debian.org/News/2026/20260712)**: Es la versión estable anterior, recomendada solo si necesitas mantener compatibilidad con software o dependencias. Su ultima version fue este 12 de julio de 2026

Con lo cual la respuesta es evidente, me quedo con Debian Slim Trixie.

### Cosas a tener en cuenta
Esta candidata con la que me quedo, como bien dije, no incluye curl ni wget por defecto (precisamente porque es slim), por lo que tengo dos opciones para bajar just y nextest:
- O me bajo curl y ca-certificates en la capa RUN del dockerfile
- O cargo install que es mas lento