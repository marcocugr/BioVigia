# Milestones
Definen qué quiero entregar, no cómo entregarlo. Un milestone no es "he terminado tal HU": marca el nivel de profundidad con el que se resuelve el problema en ese momento.

## Milestone 0: Modelo de dominio

### Qué se entrega
Las entidades y objetos valor mínimos que representan el dominio (por ejemplo: río, especie, avistamiento), modelados en código y organizados según las buenas prácticas del lenguaje elegido.
Sin lógica de negocio todavía: el objetivo es tener una estructura de datos correcta.

### Nivel de abstracción
Diseño del dominio (DDD): qué es objeto valor, qué es entidad, qué relaciones hay entre ellos.


## Milestone 1: Primera regla de negocio

### Qué se entrega
La lógica mínima que opera sobre el modelo del Milestone 0 para resolver un caso simple y verificable (por ejemplo: dado un río y una especie, determinar si hay avistamientos recientes de una especie invasora que la depreda).

### Nivel de abstracción
Primera regla de negocio sobre estructuras ya existentes