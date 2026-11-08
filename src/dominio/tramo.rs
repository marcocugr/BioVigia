use std::fmt::Debug;

use crate::dominio::coordenadas::Coordenadas;
use crate::dominio::error::ErrorDominio;

pub trait GeometriaTramo: Debug {
    fn contiene(&self, coordenadas: &Coordenadas) -> bool;
}

#[derive(Debug)]
pub struct Recuadro {
    una_esquina: Coordenadas,
    otra_esquina: Coordenadas,
}

impl Recuadro {
    pub fn new(una_esquina: Coordenadas, otra_esquina: Coordenadas) -> Self {
        Self {
            una_esquina,
            otra_esquina,
        }
    }
}

impl GeometriaTramo for Recuadro {
    fn contiene(&self, coordenadas: &Coordenadas) -> bool {
        coordenadas.entre(&self.una_esquina, &self.otra_esquina)
    }
}

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

    pub fn contiene(&self, coordenadas: &Coordenadas) -> bool {
        self.geometria.contiene(coordenadas)
    }
}

impl PartialEq for TramoDeRio {
    fn eq(&self, otro: &Self) -> bool {
        self.codigo == otro.codigo
    }
}

impl Eq for TramoDeRio {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dominio::coordenadas::{Latitud, Longitud};

    fn punto(latitud: f64, longitud: f64) -> Coordenadas {
        Coordenadas::new(
            Latitud::new(latitud).unwrap(),
            Longitud::new(longitud).unwrap(),
        )
    }

    #[test]
    fn un_tramo_contiene_lo_que_cae_dentro_de_su_recuadro_y_no_lo_de_fuera() {
        let recuadro = Recuadro::new(punto(39.9, -4.5), punto(39.8, -4.6));
        let tramo = TramoDeRio::new("TAJO-01".into(), Box::new(recuadro)).unwrap();
        assert!(tramo.contiene(&punto(39.85, -4.55)));
        assert!(tramo.contiene(&punto(39.8, -4.6)));
        assert!(!tramo.contiene(&punto(39.95, -4.55)));
        assert!(!tramo.contiene(&punto(39.85, -4.4)));
    }
}
