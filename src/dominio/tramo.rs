use std::fmt::Debug;

use crate::dominio::error::ErrorDominio;
use crate::dominio::coordenadas::Coordenadas;

const RADIO_TOLERANCIA_METROS: f64 = 100.0;

#[derive(Debug)]
pub struct GeometriaTramo {
    noroeste: Coordenadas,
    noreste: Coordenadas,
    suroeste: Coordenadas,
    sureste: Coordenadas,
}

#[derive(Debug)]
pub struct TramoDeRio {
    codigo: String,
    geometria: GeometriaTramo,
}

impl TramoDeRio {
    pub fn new(codigo: String, geometria: GeometriaTramo) -> Result<Self, ErrorDominio> {
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
