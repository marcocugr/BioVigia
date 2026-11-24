# Elección de la imagen base

## Criterios
Los criterios 1, 2 y 3 son eliminatorios (se cumplen o no) y el 4 ordena a las que pasen. Todos se comprueban con un comando o un enlace, y las medidas son del 24 de noviembre de 2026.

1. **Trazabilidad:** la imagen tiene repo publico con su Dockerfile, para poder ver que lleva dentro y que no esta comprometida.
2. **Version:** el rustc de la imagen es la ultima version estable que publica rust.
3. **Mantenimiento:** el ultimo push al repositorio de docker hub es de hace menos de 6 semanas, [que es lo que tarda rust en sacar version nueva.](https://rust-lang.github.io/rustup/concepts/channels.html) Si es mas antiguo, la imagen se puede quedar atras de la estable.
4. **Herramientas ajenas:** cuantas de estas seis trae la imagen sin hacer falta para compilar ni testear: git, ssh, curl, wget, jq y g++. Cuantas menos, mejor. El compilador de C (gcc) no cuenta porque rustc lo necesita para enlazar.

### Que necesito
Esto lo cumplen todas, asi que no sirve para distinguirlas y por eso no esta en los criterios:
- cargo y rust incluidos
- poder instalar just y nextest
- poder bajar las dependencias del cargo.toml (por el momento solo assertor) desde crates.io

### Como lo compruebo
Ultimo push (criterio 3):
```bash
curl -s https://hub.docker.com/v2/repositories/library/rust/ | python3 -m json.tool | grep last_updated
```
Version de rust (criterio 2), comparando la de la imagen con la ultima estable oficial:
```bash
docker run --rm --entrypoint sh rust:slim-trixie -c 'rustc --version'
curl -s https://static.rust-lang.org/dist/channel-rust-stable.toml | grep -m1 -A1 '^\[pkg.rust\]'
```
Herramientas ajenas (criterio 4):
```bash
docker run --rm --entrypoint sh rust:slim-trixie -c 'for t in git ssh curl wget jq g++; do command -v $t >/dev/null && echo "$t: si" || echo "$t: no"; done'
```
Lo mismo para cada imagen cambiando el nombre. Para cargo-chef y ekidd use el tag latest y para cimg el tag 1.100.0, porque no tiene latest.

### Evaluacion

#### [rust](https://hub.docker.com/_/rust)
- **SO:** Debian (Bookworm o Trixie, ambos pueden ser slim o no) o Alpine
- **Repo trazable?** [Si](https://github.com/rust-lang/docker-rust)
- **De lo que tengo, que tiene:** rust y cargo
- **De lo que tengo, que no tiene:** just, nextest y las dependencias del proyecto
- **Como podria bajar lo que no tiene:** just con una sola linea, nextest puede instalarse con su instalador oficial (otra linea) y las dependencias (assertor) ejecutando cargo fetch
- **Quien la mantiene:** Rust Project developers
- **Ultimo push:** 2026-11-24. **Pasa el criterio 3**
- **Version de rust (tag slim-trixie):** rustc 1.100.0, que es la ultima estable (publicada el 2026-11-09). **Pasa el criterio 2**
- **Herramientas ajenas (tag slim-trixie):** **0 de 6**, no trae ninguna de git, ssh, curl, wget, jq ni g++

#### [lukemathwalker/cargo-chef](https://hub.docker.com/r/lukemathwalker/cargo-chef)
- **SO:** Hereda la variante de rust oficial: Debian Bookworm/Trixie o Alpine
- **Repo trazable?** [Si](https://github.com/LukeMathWalker/cargo-chef)
- **De lo que tengo, que tiene:** rustc, cargo y cargo-chef preinstalado
- **De lo que tengo, que no tiene:** just, cargo-nextest y las dependencias del proyecto
- **Como podria bajar lo que no tiene:** Igual que en la imagen oficial de rust
- **Quien la mantiene:** LukeMathWalker y la comunidad, al fin y al cabo es proyecto de terceros, pero activamente mantenido
- **Ultimo push:** 2026-11-13. **Pasa el criterio 3**
- **Version de rust (tag latest):** rustc 1.100.0, la ultima estable. **Pasa el criterio 2**
- **Herramientas ajenas (tag latest):** **5 de 6**, trae git, ssh, curl, wget y g++ y solo le falta jq. Ademas del cargo-chef, que sirve para cachear dependencias rust en builds de docker multietapa y que yo no necesito

#### [cimg/rust](https://hub.docker.com/r/cimg/rust)
- **SO:** Ubuntu
- **Repo trazable?** [Si](https://github.com/CircleCI-Public/cimg-rust)
- **De lo que tengo, que tiene:** rust y cargo. Las variantes con sufijo -node y -browsers añaden ademas node y navegadores
- **De lo que tengo, que no tiene:** just, nextest y las dependencias del proyecto
- **Como podria bajar lo que no tiene:** Igual que en la imagen oficial de rust
- **Quien la mantiene:** CircleCI
- **Ultimo push:** 2026-11-13. **Pasa el criterio 3**
- **Version de rust (tag 1.100.0):** rustc 1.100.0, la ultima estable. **Pasa el criterio 2**
- **Herramientas ajenas (tag 1.100.0):** **6 de 6**, trae git, ssh, curl, wget, jq y g++. Es una "convenience image" de CI, por eso lleva todo eso que es completamente irrelevante para compilar un proyecto de rust

#### [ekidd/rust-musl-builder](https://hub.docker.com/r/ekidd/rust-musl-builder/)
- **SO:** Ubuntu
- **Repo trazable?** [Sí](https://github.com/emk/rust-musl-builder)
- **De lo que tengo, que tiene:** Rust, Cargo, musl libc, compiladores de C y de C++ y librerías C estáticas orientadas a producir binarios estáticos
- **De lo que tengo, que no tiene:** just, cargo-nextest y las dependencias del proyecto
- **Como podria bajar lo que no tiene:** Igual que en la imagen oficial de rust
- **Quien la mantiene:** Terceros y la comunidad, pero vamos que la imagen publicada no se actualiza desde hace mas de cuatro años
- **Ultimo push:** 2022-03-24. **No pasa el criterio 3**
- **Version de rust (tag latest):** rustc 1.59.0 cuando la ultima estable es la 1.100.0. **No pasa el criterio 2**
- **Herramientas ajenas (tag latest):** **4 de 6**, trae git, ssh, curl y g++, y no trae wget ni jq. Esta orientada a compilacion cruzada y binarios estaticos, por eso lleva librerias estaticas, OpenSSL y scripts propios que no hacen falta para un build normal
- **Resultado:** eliminada por los criterios 2 y 3, abandonada

### Resultado de la primera ronda
- **rust (oficial):** pasa los tres y trae 0 de 6 herramientas ajenas
- **lukemathwalker/cargo-chef:** pasa los tres y trae 5 de 6
- **cimg/rust:** pasa los tres y trae 6 de 6
- **ekidd/rust-musl-builder:** eliminada

Con el criterio 4 la imagen oficial queda la primera con diferencia.

### Variantes de la imagen oficial
La oficial tiene varias variantes, asi que hace falta otra ronda para elegir. Aparte del criterio 4 uso dos criterios mas:

5. **Plataforma:** el target de la variante es tier 1 en la [tabla de plataformas de rust](https://doc.rust-lang.org/nightly/rustc/platform-support.html). Esto es lo que dice el propio proyecto rust sobre lo que garantiza que funcione.
6. **Debian estable:** si la variante es Debian, que sea la release estable actual segun [debian.org](https://www.debian.org/releases/).

Resultado:
- **Alpine:** usa musl, y x86_64-unknown-linux-musl no es tier 1 mientras que x86_64-unknown-linux-gnu (glibc) si lo es. **Descartada por el criterio 5**, no por tamaño.
- **Debian normal (sin slim):** contiene "un gran número de paquetes Debian extremadamente comunes" segun su propia documentacion, asi que pierde en el criterio 4 frente a la slim, que contiene solo los paquetes mínimos para ejecutar Rust.
- **Debian Bookworm:** es la estable anterior, no la actual. **Descartada por el criterio 6**.
- **Debian Slim Trixie:** pasa todo. **Elegida.**

## Decision
Si resumimos cada una basicamente:
- **rust**: Por partes:
  - Debian slim: es la opcion mas adecuada si quieres compilar sin complicaciones, parte de lo minimo (0 de 6 herramientas ajenas), mantienes compatibilidad con glibc y solo añades just, nextest y las dependencias del proyecto. La variante normal (la que no es slim) solo tendria sentido si necesitara muchas herramientas del sistema y no quisiera instalarlas yo.
  - Alpine: es la mas ligera y usa busybox, asi que ni siquiera trae bash. Pero usa musl en lugar de glibc, y ese target no es tier 1 en rust, por eso la descarto por el criterio 5 y no porque me de miedo el tamaño. Mi prioridad no es optimizar hasta el ultimo byte, sino quitar muchisimas cosas innecesarias y poder trabajar sin problemas.
- **lukemathwalker/cargo-chef**: Interesante, pero lo que me trae es el cargo-chef, que sirve para cachear dependencias y evitar recompilarlas constantemente, y ademas git, ssh, curl, wget y g++ (5 de 6) que no necesito. En mi caso el proyecto es pequeño y no voy a desarrollarlo mas alla de estas asignaturas, asi que no aprovecho sus beneficios. Aunque hubiera variante slim, lo unico que añadiria seria cargo-chef, que sigo sin usar.
- **cimg/rust**: Trae las 6 herramientas ajenas (git, ssh, curl, wget, jq y g++), que no tienen nada que ver con compilar y testear mi proyecto. Vamos, que tiene mucha basura.
- **ekidd/rust-musl-builder**: Abandonada y obsoleta. Su ultimo push es de 2022 y trae rustc 1.59.0 cuando la ultima estable es la 1.100.0, asi que no cumple los criterios 2 y 3.

Sabiendo que ninguna imagen publica va a venir hecha a medida exactamente para mi Cargo.toml con solo assertor, just y nextest sin meter cosas de mas, siempre voy a tener que meterle cosas a las imagenes. Partiendo de ahi, prefiero la que menos cosas innecesarias trae y que ademas es la que mantiene el propio proyecto rust.

Finalmente y tras este analisis, me quedo con la imagen oficial en su variante **Debian Slim Trixie**.

Ahora bien, ¿Trixie o Bookworm? Las dos son slim, asi que lo decide el criterio 6:
- **[Trixie](https://www.debian.org/releases/trixie/)**: Es la version estable actual, con lo cual es ideal para proyectos nuevos por tener librerias mas modernas. Su ultima version salio el 21 de noviembre de 2026.
- **[Bookworm](https://www.debian.org/News/2026/20260712)**: Es la version estable anterior, recomendada solo si necesitas mantener compatibilidad con software o dependencias. Su ultima version salio el 12 de julio de 2026.

Con lo cual la respuesta es evidente, me quedo con Debian Slim Trixie.

### Cosas a tener en cuenta
Esta imagen no incluye curl ni wget por defecto (precisamente porque es slim, lo medi arriba). Para bajar just y nextest tengo dos opciones:
- Instalar curl y ca-certificates en la capa RUN del Dockerfile, que añade dos herramientas que no hacen falta.
- cargo install, que es mas lento.

Uso cargo install. Tarda mas, pero la imagen se construye una vez y se usa muchas, asi que el tiempo de construccion no cuenta, y asi la imagen se queda sin curl.