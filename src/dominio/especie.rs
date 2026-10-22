use std::collections::{HashMap, HashSet};

use crate::dominio::error::ErrorDominio;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Especie {
    nombres_comunes: HashSet<String>,
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
        nombres_comunes: Vec<String>,
        invasora: bool,
        nombres_anteriores: Vec<String>,
    ) -> Result<(), ErrorDominio> {
        let nombre_cientifico = nombre_cientifico.trim().to_owned();
        let nombres_anteriores: Vec<String> = nombres_anteriores
            .iter()
            .map(|n| n.trim().to_owned())
            .collect();
        if nombre_cientifico.is_empty() || nombres_anteriores.iter().any(|n| n.is_empty()) {
            return Err(ErrorDominio::NombreCientificoRequerido);
        }
        if nombres_comunes.is_empty() {
            return Err(ErrorDominio::NombreComunRequerido);
        }
        let nombres_comunes: HashSet<String> = nombres_comunes
            .into_iter()
            .map(|n| n.trim().to_owned())
            .filter(|n| {
                if n.is_empty() {
                    return false;
                }
                true
            })
            .collect();
        if nombres_comunes.is_empty() {
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
                nombres_comunes,
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
}
