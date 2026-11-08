use std::collections::{HashMap, HashSet};

use crate::dominio::error::ErrorDominio;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Especie {
    nombre_comun: String,
    invasora: bool,
}

#[derive(Debug, Clone, Default)]
pub struct CatalogoEspecies {
    especies: HashMap<String, Especie>,
    nombres_anteriores: HashMap<String, String>,
}

impl CatalogoEspecies {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn registrar(
        &mut self,
        nombre_cientifico: String,
        nombre_comun: String,
        invasora: bool,
        nombres_anteriores: Vec<String>,
    ) -> Result<(), ErrorDominio> {
        let nombre_cientifico = nombre_cientifico.trim().to_owned();
        let nombre_comun = nombre_comun.trim().to_owned();
        let nombres_anteriores: Vec<String> = nombres_anteriores
            .iter()
            .map(|n| n.trim().to_owned())
            .collect();
        if nombre_cientifico.is_empty() || nombres_anteriores.iter().any(|n| n.is_empty()) {
            return Err(ErrorDominio::NombreCientificoRequerido);
        }
        if nombre_comun.is_empty() {
            return Err(ErrorDominio::NombreComunRequerido);
        }
        let mut nuevos = HashSet::new();
        for nombre in std::iter::once(&nombre_cientifico).chain(&nombres_anteriores) {
            if !nuevos.insert(nombre)
                || self.especies.contains_key(nombre)
                || self.nombres_anteriores.contains_key(nombre)
            {
                return Err(ErrorDominio::NombreEspecieRepetido);
            }
        }
        for anterior in nombres_anteriores {
            self.nombres_anteriores
                .insert(anterior, nombre_cientifico.clone());
        }
        self.especies.insert(
            nombre_cientifico,
            Especie {
                nombre_comun,
                invasora,
            },
        );
        Ok(())
    }

    pub(crate) fn especie(&self, nombre: &str) -> Result<&str, ErrorDominio> {
        let nombre = nombre.trim();
        match self.especies.get_key_value(nombre) {
            Some((nombre_cientifico, _)) => Ok(nombre_cientifico),
            None => self
                .nombres_anteriores
                .get(nombre)
                .map(String::as_str)
                .ok_or(ErrorDominio::EspecieDesconocida),
        }
    }

    pub(crate) fn especie_invasora(&self, nombre: &str) -> Result<&str, ErrorDominio> {
        let nombre_cientifico = self.especie(nombre)?;
        if self.especies[nombre_cientifico].invasora {
            Ok(nombre_cientifico)
        } else {
            Err(ErrorDominio::EspecieNoInvasora)
        }
    }

    pub fn especie_con_nombre_comun(&self, nombre_comun: &str) -> Result<&str, ErrorDominio> {
        let buscado = clave(nombre_comun);
        self.especies
            .iter()
            .find(|(_, especie)| clave(&especie.nombre_comun) == buscado)
            .map(|(nombre_cientifico, _)| nombre_cientifico.as_str())
            .ok_or(ErrorDominio::EspecieDesconocida)
    }
}

fn clave(nombre: &str) -> String {
    nombre
        .trim()
        .chars()
        .flat_map(char::to_lowercase)
        .map(sin_tilde)
        .collect()
}

fn sin_tilde(letra: char) -> char {
    match letra {
        'á' => 'a',
        'é' => 'e',
        'í' => 'i',
        'ó' => 'o',
        'ú' | 'ü' => 'u',
        otra => otra,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn catalogo() -> CatalogoEspecies {
        let mut catalogo = CatalogoEspecies::new();
        catalogo
            .registrar(
                "Procambarus clarkii".into(),
                "Cangrejo rojo americano".into(),
                false,
                vec![],
            )
            .unwrap();
        catalogo
            .registrar(
                "Neogale vison".into(),
                "Visón americano".into(),
                true,
                vec!["Mustela vison".into()],
            )
            .unwrap();
        catalogo
    }

    #[test]
    fn el_pescador_encuentra_la_especie_sin_mayusculas_tildes_ni_espacios_de_los_lados() {
        let catalogo = catalogo();
        assert_eq!(
            catalogo.especie_con_nombre_comun("Cangrejo rojo americano"),
            Ok("Procambarus clarkii")
        );
        assert_eq!(
            catalogo.especie_con_nombre_comun("  CANGREJO ROJO AMERICANO "),
            Ok("Procambarus clarkii")
        );
        assert_eq!(
            catalogo.especie_con_nombre_comun("vison americano"),
            Ok("Neogale vison")
        );
    }

    #[test]
    fn un_nombre_comun_que_no_es_de_ninguna_especie_da_error_y_no_una_lista_vacia() {
        assert_eq!(
            catalogo().especie_con_nombre_comun("unicornio"),
            Err(ErrorDominio::EspecieDesconocida)
        );
    }
}
