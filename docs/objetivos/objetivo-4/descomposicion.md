# Descomposición de la HU001 en issues (objetivo 4)

La historia de usuario [HU001 (#6)](https://github.com/marcocugr/BioVigia/issues/6) se descompuso en una serie de problemas más sencillos, cada uno con su issue y su criterio de aceptación. Cada caja del diagrama (`diagrama.png`, en esta misma carpeta) es un issue y cada flecha dice de qué otro issue surge.

Los issues se numeran en cascada: primero el central y después cada rama completa, con cada hijo justo detrás de su padre.

## Dónde está cada issue

| Issue | Qué resuelve | Código | Tests |
|---|---|---|---|
| #31 ¿Como consulto una zona? | Junta todas las piezas en el caso de uso | `src/aplicacion/consulta.rs` | `consulta.rs`, `tests/hu001.rs` |
| #32 ¿Como se tratan los avistamientos? | De un número de avistamientos a un estado de presencia | `PresenciaDepredador` en `src/dominio/avistamiento.rs` | `avistamiento.rs` |
| #33 ¿Quienes depredan a una especie dado su nombre común? | Depredadores invasores de un nombre común | `RedDepredacion` en `src/dominio/depredacion.rs` | `depredacion.rs` |
| #34 ¿Exactamente quien/es son todas las especies con las que nos referimos bajo un solo nombre común? | Especies de un nombre común | `CatalogoEspecies` en `src/dominio/especie.rs` | `especie.rs` |
| #35 Varias especies tienen dos o más nombres comunes | Varios nombres comunes por especie | `src/dominio/especie.rs` | `especie.rs` |
| #36 ¿Qué pasa si un nombre común no existe? | Error `NombreComunInexistente` | `src/dominio/especie.rs` | `especie.rs`, `consulta.rs` |
| #37 ¿Como sabes si un avistamiento es de hace x días o menos? | Ventana de días con fecha de referencia | `Avistamiento::es_reciente` | `avistamiento.rs` |
| #38 ¿Como se yo que un depredador ha sido avistado en un río determinado? | Avistamiento y río | `Avistamiento::pertenece_a_rio` | `avistamiento.rs`, `consulta.rs` |
| #39 ¿Qué pasa si un río no existe? | Error `RioInexistente` | `Rios::tramos_de_rio` en `src/dominio/tramo.rs` | `tramo.rs`, `consulta.rs` |
| #40 ¿Como se yo que unas coordenadas pertenecen a un tramo de un río o sus alrededores? | Distancia a la línea del tramo | `LineaTramo` en `src/dominio/tramo.rs` | `tramo.rs` |
| #41 ¿Cómo consulto todos los tramos ríos? | La fuente de la verdad de los ríos | `Rios` en `src/dominio/tramo.rs` | `tramo.rs` |
| #42 ¿A que río pertenece un tramo de río? | Nombre del río de un tramo | `TramoDeRio::nombre_rio` | `tramo.rs` |
| #43 ¿Tiene nombre un tramo de río? | El tramo guarda el nombre de su río | `TramoDeRio::new` | `tramo.rs` |
| #44 ¿Como organizo los test en el repositorio siguiendo las mejores prácticas de rust? | Herramientas y organización de los tests | [README](../../../README.md#tests) | |
| #45 y #46 iNaturalist | Qué datos y cómo se procesan | `src/aplicacion/procesadores_html/inaturalist.rs` | el mismo fichero y `tests/fuentes_reales.rs` |
| #47 y #48 GBIF | Qué datos y cómo se procesan | `.../gbif.rs` | el mismo fichero y `tests/fuentes_reales.rs` |
| #49 y #50 GloBI | Qué datos y cómo se procesan | `.../globi.rs` | el mismo fichero y `tests/fuentes_reales.rs` |
| #51 y #52 GISD | Qué datos y cómo se procesan | `.../gisd.rs` | el mismo fichero y `tests/fuentes_reales.rs` |

## Fuentes de la verdad

El dominio tiene cuatro, cada una dueña de un tipo de dato:

- `CatalogoEspecies`: las especies, sus nombres y si son invasoras.
- `RedDepredacion`: quién depreda a quién.
- `RegistroAvistamientos`: los avistamientos.
- `Rios`: los tramos de río agrupados por el nombre de su río.
