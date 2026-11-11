use std::fmt::Debug;

use crate::dominio::error::ErrorDominio;

pub trait GeometriaTramo: Debug {}

#[derive(Debug)]
pub struct TramoDeRio {
    codigo: String,
    geometria: Box<dyn GeometriaTramo>,
}

impl TramoDeRio {
    pub fn new(codigo: String, geometria: Box<dyn GeometriaTramo>) -> Result<Self, ErrorDominio> {
        let codigo = codigo.trim().to_owned();
        if codigo.is_empty() {
            return Err(ErrorDominio::CodigoTramoRequerido);
        }
        Ok(Self { codigo, geometria })
    }
}

impl PartialEq for TramoDeRio {
    fn eq(&self, otro: &Self) -> bool {
        self.codigo == otro.codigo
    }
}

impl Eq for TramoDeRio {}
