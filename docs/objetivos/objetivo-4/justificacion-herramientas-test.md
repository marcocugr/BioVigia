# Justificación: biblioteca de aserciones y test runner
Se usan las aserciones de la biblioteca estándar (`assert!`, `assert_eq!` y `assert_ne!`) y `cargo test` como test runner. Es la única combinación que cumple a la vez los cinco criterios fijados antes de elegir: viene con el lenguaje, no escribe fuera del directorio de compilación (configurable con `CARGO_TARGET_DIR`), enseña el valor esperado y el obtenido al fallar, sirve para tests unitarios y de integración y funciona con la toolchain y la edición del proyecto.

Se compararon `pretty_assertions` y `googletest` como bibliotecas de aserciones, y `cargo-nextest` como test runner. `rstest` no es ni una cosa ni la otra, sino una capa de fixtures sobre el runner.

El razonamiento completo (criterios de aceptación, criterios de búsqueda, opciones consideradas, qué se asume y cuándo se reconsidera) está en el [README](../../../README.md#herramientas-de-test) y en el issue [#60](https://github.com/marcocugr/BioVigia/issues/60).
