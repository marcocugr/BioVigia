use std::collections::HashMap;
use std::fmt::Debug;

use crate::dominio::error::ErrorDominio;

pub trait GeometriaTramo: Debug {}

#[derive(Debug)]
pub struct TramoDeRio {
    codigo: String,
    nombre_rio: String,
    geometria: Box<dyn GeometriaTramo>,
}

impl TramoDeRio {
    pub fn new(
        codigo: String,
        nombre_rio: String,
        geometria: Box<dyn GeometriaTramo>,
    ) -> Result<Self, ErrorDominio> {
        let codigo = codigo.trim().to_owned();
        if codigo.is_empty() {
            return Err(ErrorDominio::CodigoTramoRequerido);
        }
        let nombre_rio = nombre_rio.trim().to_owned();
        if nombre_rio.is_empty() {
            return Err(ErrorDominio::NombreRioRequerido);
        }
        Ok(Self { codigo, nombre_rio, geometria })
    }

    pub fn codigo(&self) -> &str {
        &self.codigo
    }

    pub fn nombre_rio(&self) -> &str {
        &self.nombre_rio
    }
}

impl PartialEq for TramoDeRio {
    fn eq(&self, otro: &Self) -> bool {
        self.codigo == otro.codigo
    }
}

impl Eq for TramoDeRio {}

#[derive(Debug, Default)]
pub struct Rios {
    tramos_por_rio: HashMap<String, Vec<TramoDeRio>>,
}

impl Rios {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn registrar(&mut self, tramo: TramoDeRio) -> Result<(), ErrorDominio> {
        if self.tramos_por_rio.values().flatten().any(|t| t == &tramo) {
            return Err(ErrorDominio::TramoRepetido);
        }
        let clave = normalizar(&tramo.nombre_rio);
        self.tramos_por_rio.entry(clave).or_default().push(tramo);
        Ok(())
    }

    pub fn tramos_de_rio(&self, nombre_rio: &str) -> &[TramoDeRio] {
        let clave = normalizar(nombre_rio);
        self.tramos_por_rio
            .get(&clave)
            .map(Vec::as_slice)
            .unwrap_or(&[])
    }
}

fn normalizar(s: &str) -> String {
    s.trim()
        .chars()
        .map(|c| match c {
            'á' | 'à' | 'ä' | 'â' | 'Á' | 'À' | 'Ä' | 'Â' => 'a',
            'é' | 'è' | 'ë' | 'ê' | 'É' | 'È' | 'Ë' | 'Ê' => 'e',
            'í' | 'ì' | 'ï' | 'î' | 'Í' | 'Ì' | 'Ï' | 'Î' => 'i',
            'ó' | 'ò' | 'ö' | 'ô' | 'Ó' | 'Ò' | 'Ö' | 'Ô' => 'o',
            'ú' | 'ù' | 'ü' | 'û' | 'Ú' | 'Ù' | 'Ü' | 'Û' => 'u',
            other => other.to_ascii_lowercase(),
        })
        .collect()
}
