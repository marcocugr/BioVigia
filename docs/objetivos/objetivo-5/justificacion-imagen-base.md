# Elección de la imagen base

## Criterios
Los criterios 1, 2 y 3 son eliminatorios (se cumplen o no), igual que el 6 y el 7, que se explican mas abajo y solo se aplican a las variantes de una misma imagen. Los criterios 4 y 5 ordenan a las que pasen y, si no coinciden, manda el 5, porque es lo que se paga en cada ejecucion de CI. Todos se comprueban con un comando o un enlace. Las medidas de los criterios 1, 2 y 3 son del 24 de noviembre de 2026 y las de los criterios 4 y 5 del 26 de noviembre de 2026.

1. **Trazabilidad:** la imagen tiene repo publico con su Dockerfile, para poder ver que lleva dentro y que no esta comprometida.
2. **Version:** el rustc de la imagen es la ultima version estable que publica rust.
3. **Mantenimiento:** el ultimo push al repositorio de docker hub es de hace menos de 6 semanas, [que es lo que tarda rust en sacar version nueva.](https://rust-lang.github.io/rustup/concepts/channels.html) Si es mas antiguo, la imagen se puede quedar atras de la estable.
4. **Tamaño:** lo que ocupa la imagen en el runner de github actions, segun `docker image inspect`. Cuanto menos, mejor porque esto mejora la seguridad, acelera los despliegues y reduce los costos de infraestructura.
5. **Tiempo de descarga:** lo que tarda `docker pull` de la imagen en un workflow de github actions, que es donde se va a usar en el objetivo 6. Cuanto menos, mejor.

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
Tamaño y tiempo de descarga (criterios 4 y 5), en un workflow de github actions:
```bash
TIMEFORMAT='%R s'
for img in rust:slim-trixie rust:trixie rust:slim-bookworm rust:alpine lukemathwalker/cargo-chef:latest cimg/rust:1.100.0; do
  echo "== $img"
  echo -n "pull: "
  time docker pull -q "$img" >/dev/null
  docker image inspect "$img" --format 'tamaño: {{.Size}} bytes'
done
```
Lo mido en el runner y no en mi portatil porque el runner es una maquina nueva en cada ejecucion y no tiene nada en cache, mientras que en mi portatil las capas que ya tengo descargadas falseaban el tiempo. Mido la imagen base sola porque lo que le añado encima (just, nextest y las dependencias del proyecto) es lo mismo en todas. Es una sola medicion por imagen.

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
- **Tamaño y tiempo de descarga (tag slim-trixie):** 329 MB y 6,8 s

#### [lukemathwalker/cargo-chef](https://hub.docker.com/r/lukemathwalker/cargo-chef)
- **SO:** Hereda la variante de rust oficial: Debian Bookworm/Trixie o Alpine
- **Repo trazable?** [Si](https://github.com/LukeMathWalker/cargo-chef)
- **De lo que tengo, que tiene:** rustc, cargo y cargo-chef preinstalado
- **De lo que tengo, que no tiene:** just, cargo-nextest y las dependencias del proyecto
- **Como podria bajar lo que no tiene:** Igual que en la imagen oficial de rust
- **Quien la mantiene:** LukeMathWalker y la comunidad, al fin y al cabo es proyecto de terceros, pero activamente mantenido
- **Ultimo push:** 2026-11-13. **Pasa el criterio 3**
- **Version de rust (tag latest):** rustc 1.100.0, la ultima estable. **Pasa el criterio 2**
- **Tamaño y tiempo de descarga (tag latest):** 606 MB y 15,0 s. Lleva ademas el cargo-chef, que sirve para cachear dependencias rust en builds de docker multietapa

#### [cimg/rust](https://hub.docker.com/r/cimg/rust)
- **SO:** Ubuntu
- **Repo trazable?** [Si](https://github.com/CircleCI-Public/cimg-rust)
- **De lo que tengo, que tiene:** rust y cargo. Las variantes con sufijo -node y -browsers añaden ademas node y navegadores
- **De lo que tengo, que no tiene:** just, nextest y las dependencias del proyecto
- **Como podria bajar lo que no tiene:** Igual que en la imagen oficial de rust
- **Quien la mantiene:** CircleCI
- **Ultimo push:** 2026-11-13. **Pasa el criterio 3**
- **Version de rust (tag 1.100.0):** rustc 1.100.0, la ultima estable. **Pasa el criterio 2**
- **Tamaño y tiempo de descarga (tag 1.100.0):** 714 MB y 25,2 s, la mas grande y la mas lenta de todas. Es una "convenience image" de CI, por eso lleva herramientas que no tienen nada que ver con compilar un proyecto de rust

#### [ekidd/rust-musl-builder](https://hub.docker.com/r/ekidd/rust-musl-builder/)
Tiene que pasra los 3 criterios si o si, partiendo de la base de que el ultimo push fue el 2022-03-24, no pasa el criterio 3 y por tanto, no la sigo analizando.

### Resultado de la primera ronda
- **rust (oficial):** pasa los tres, 329 MB y 6,8 s
- **lukemathwalker/cargo-chef:** pasa los tres, 606 MB y 15,0 s
- **cimg/rust:** pasa los tres, 714 MB y 25,2 s
- **ekidd/rust-musl-builder:** eliminada

Con los criterios 4 y 5 la imagen oficial queda la primera con diferencia: es la mas pequeña y la mas rapida.

### Variantes de la imagen oficial
La oficial tiene varias variantes, asi que hace falta otra ronda para elegir. Aparte de los criterios 4 y 5 uso dos criterios mas:

6. **Plataforma:** el target de la variante es tier 1 en la [tabla de plataformas de rust](https://doc.rust-lang.org/nightly/rustc/platform-support.html). Esto es lo que dice el propio proyecto rust sobre lo que garantiza que funcione.
7. **Debian estable:** si la variante es Debian, que sea la release estable actual segun [debian.org](https://www.debian.org/releases/).

Resultado:
- **Alpine:** usa musl, y x86_64-unknown-linux-musl no es tier 1 mientras que x86_64-unknown-linux-gnu (glibc) si lo es. **Descartada por el criterio 6**, no por tamaño.
- **Debian normal (sin slim):** contiene "un gran número de paquetes Debian extremadamente comunes" segun su propia documentacion, y se nota: 604 MB y 12,5 s frente a los 329 MB y 6,8 s de la slim. Pierde en los criterios 4 y 5.
- **Debian Bookworm:** es la estable anterior, no la actual. **Descartada por el criterio 7.**
- **Debian Slim Trixie:** pasa todo, con 329 MB y 6,8 s. **Elegida.**

## Decision
Si resumimos cada una basicamente:
- **rust**: Por partes:
  - Debian slim: es la opcion mas adecuada si quieres compilar sin complicaciones, parte de lo minimo (329 MB y 6,8 s, la mas pequeña y la mas rapida de las que cumplen los criterios), mantienes compatibilidad con glibc y solo añades just, nextest y las dependencias del proyecto. La variante normal (la que no es slim) solo tendria sentido si necesitara muchas herramientas del sistema y no quisiera instalarlas yo, y a cambio pesa 604 MB y tarda 12,5 s.
  - Alpine: usa busybox, asi que ni siquiera trae bash. Pero usa musl en lugar de glibc, y ese target no es tier 1 en rust, por eso la descarto por el criterio 6. Y aunque no la hubiera descartado, pesa 354 MB y tarda 9,2 s, mas que la slim de trixie.
- **lukemathwalker/cargo-chef**: Interesante, pero lo que me trae es el cargo-chef, que sirve para cachear dependencias y evitar recompilarlas constantemente. En mi caso el proyecto es pequeño y no voy a desarrollarlo mas alla de estas asignaturas, asi que no aprovecho sus beneficios. Ademas pesa 606 MB (casi el doble que la slim de trixie) y tarda 15,0 s (mas del doble).
- **cimg/rust**: 714 MB y 25,2 s, la mas grande y la mas lenta, y encima trae herramientas que no tienen nada que ver con compilar y testear mi proyecto. Vamos, que tiene mucha basura.
- **ekidd/rust-musl-builder**: No pasa los criterios.

Sabiendo que ninguna imagen publica va a venir hecha a medida exactamente para mi Cargo.toml con solo assertor, just y nextest sin meter cosas de mas, siempre voy a tener que meterle cosas a las imagenes. Partiendo de ahi, prefiero la que menos pesa y menos tarda en bajarse y que ademas es la que mantiene el propio proyecto rust.

Finalmente y tras este analisis, me quedo con la imagen oficial en su variante **Debian Slim Trixie**.

Ahora bien, ¿Trixie o Bookworm? Las dos son slim y en tamaño y tiempo estan casi igual (trixie 329 MB y 6,8 s, bookworm 323 MB y 7,8 s), pero el criterio 7 elimina a bookworm:
- **[Trixie](https://www.debian.org/releases/trixie/)**: Es la version estable actual, con lo cual es ideal para proyectos nuevos por tener librerias mas modernas. Su ultima version salio el 21 de noviembre de 2026.
- **[Bookworm](https://www.debian.org/News/2026/20260712)**: Es la version estable anterior, recomendada solo si necesitas mantener compatibilidad con software o dependencias. Su ultima version salio el 12 de julio de 2026.

Con lo cual la respuesta es evidente, me quedo con Debian Slim Trixie.

### Cosas a tener en cuenta
Esta imagen no incluye curl ni wget por defecto (precisamente porque es slim). Para bajar just y nextest tengo dos opciones:
- Instalar curl y ca-certificates en la capa RUN del Dockerfile, que añade dos herramientas que no hacen falta.
- cargo install, que es mas lento.

Uso cargo install. Tarda mas, pero la imagen se construye una vez y se usa muchas, asi que el tiempo de construccion no cuenta, y asi la imagen se queda sin curl.