# Aplicación de la estrategia de testing

## Niveles de test
Voy a usar únicamente dos niveles de test.

### Unitarios
- Son los que prueban una sola función de forma aislada.
- Se escriben al final del mismo archivo del código que testean dentro de `#[cfg(test)] mod tests`, o sea esto lo que hace es decirle a rust que solo compile ese trozo cuando lances los tests sin meter ese código dentro del binario final. Y el `mod tests` es lo que realmente crea el módulo para agruparlos y que al pasarlos salgan ordenados bajo el nombre de su archivo, quiero dejar expresamente constancia de que comprendo esta diferencia.
- Tienes que poner `#[test]` antes de cada funcion de test para definir un test unitario para que nextest, mi test runner elegido con criterio, conozca su mera existencia.
- La implementacion, dada mi explicación de arriba, es sencilla e intuitiva, pongo un ejemplo visual de cómo se hace cada test unitario al final de un archivo:
    ```rust
    pub fn funcion_privada() { ... }

    #[cfg(test)] // <-- Solo compila esto si ejecutamos tests
    mod tests {  // <-- Crea el grupo "tests" dentro de "archivo.rs"
        use super::*; // <-- Sin esta línea, "funcion_privada" aquí dentro no existe y por tanto no puede ser usada:

        #[test]
        fn test_unitario_1() { ... }

        #[test]
        fn test_unitario_n() { ... }

    }
    ```

### Integración
- Son los que prueban el flujo completo combinando múltiples partes de mi proyecto.
- Estos requieren archivos separados bajo un directorio, llamado caracter a caracter, `tests/`.
- No necesitan `#[cfg(test)]` porque esos archivos ya están dedicados exclusivamente a tests y no se te van a mezclar con código de producción.
- A diferencia de los test unitarios, los de integración no necesitan ir entre `mod`, porque estos archivos bajo `tests/` ya son tratados como un binario separado, pero si que sigues teniendo que poner `#[test]` antes de cada función para definirle como test de integración.
- Tras todas estas consideraciones comentadas, nuevamente, la implementación vuelve a ser sencilla e intuitiva, pongo un ejemplo de cómo se hace cada test de integración en un archivo separado:
    ```rust
    // archivo: tests/test_integracion.rs
    use mi_proyecto::funcion_publica;

    #[test]
    fn test_integracion_1() { ... }

    #[test]
    fn test_integracion_n() { ... }
    ```

## Organización de test
Concluyendo:
- Los tests unitarios van al final de cada archivo.rs que sea necesario testear, independientemente de su ruta.
- Los tests de integración van en el directorio `tests/` sobre la raiz del proyecto y necesitan funciones de diversos archivos del mismo, pudiendo necesitar tambien archivos HTML.
- Los fixtures HTML de iNaturalist van en `tests/fixtures/inaturalist/`.

## Ejecución de los tests
Puedes ejecutar todos los test de forma sencilla delegando la tarea al gestor de tareas `just`:
```bash
just test
```

La alternativa manual a esto (contenido de `just test`) es:
```bash
cargo nextest run
```

Para ejecutar un test específico (unitario o de integración) basta con escribir su nombre, sin rutas de carpetas:
```bash
cargo nextest run nombre_del_test
```
Pero claro, si tienes dos test con el mismo nombre en archivos distintos, te ejecuta los dos.
Para evitarlo y lanzar solo uno concreto (sabiendo de antemano cúal) tienes que especificar su módulo delante, por ejemplo: `cargo nextest run archivo::tests::nombre_del_test`

Para ejecutar únicamente un archivo concreto de test de integración (los que están dentro de `tests/`) se usa el flag `--test`, poniendo solo el nombre del archivo a secas, sin ruta ni extensión `.rs`:
```bash
cargo nextest run --test nombre_del_archivo
```
