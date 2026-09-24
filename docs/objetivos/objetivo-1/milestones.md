# Milestones
Definen que quiero entregar, no como entregarlo.

## Milestone 0: 

### Que se entrega
Persistencia de datos con rios y fauna fluvial de Toledo.

### Cual es el objetivo del milestone
Tener en un sistema de persistencia de datos en ficheros bajo una misma carpeta donde se almacenen 30 especies fluviales que existen en Toledo y otro donde se almacenen 15 los ríos de Toledo.
Las especies deben contar con:
- Nombre cientifico
- Nombre conocido en España. 

Los ríos deben contar con:
- Nombre.
- Provincia.
- Ciudad.
- Pueblo.

### Historias de usuario asignadas
- [HU001]


## Milestone 1

### Que se entrega
Un sistema de búsqueda en la cual se introduzca un nombre de un río de toledo y una especie a pescar, generandose alertas si hay especies invasoras que le depreden en esa zona. 

### Cual es el objetivo del milestone
Poder introducir/asociar especies invasoras en un rio de Toledo determinado.
Los sistema de persistencia de datos deben actualizarse de la siguiente forma:
- El relativo a ríos debe contar con un campo mas que represente qué especies habitan en él.
- El relativo a especies debe contar con dos datos adicionales, uno que represente si es presa de unas especies invasoras o no, y otro que incluya a quien caza o por quien es cazado.

### Historias de usuario asignadas
- [HU002]