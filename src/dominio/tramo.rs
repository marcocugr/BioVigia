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
