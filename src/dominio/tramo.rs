use std::fmt::Debug;

use crate::dominio::error::ErrorDominio;
use crate::dominio::coordenadas::Coordenadas;

const RADIO_TOLERANCIA_METROS: f64 = 100.0;

#[derive(Debug)]
pub struct GeometriaTramo {
    pub noroeste: Coordenadas,
    pub noreste: Coordenadas,
    pub suroeste: Coordenadas,
    pub sureste: Coordenadas,
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

    pub fn pertenece(&self, coordenadas: &Coordenadas) -> bool {
        distancia_punto_a_rectangulo(coordenadas, &self.geometria) <= RADIO_TOLERANCIA_METROS
    }
}

fn distancia_punto_a_rectangulo(punto: &Coordenadas, rect: &GeometriaTramo) -> f64 {
    let lat_min = rect.suroeste.latitud.0.min(rect.sureste.latitud.0);
    let lat_max = rect.noroeste.latitud.0.max(rect.noreste.latitud.0);
    let lon_min = rect.noroeste.longitud.0.min(rect.suroeste.longitud.0);
    let lon_max = rect.noreste.longitud.0.max(rect.sureste.longitud.0);
    let lat_punto = punto.latitud.0;
    let lon_punto = punto.longitud.0;
    let lat_cercana = lat_punto.clamp(lat_min, lat_max);
    let lon_cercana = lon_punto.clamp(lon_min, lon_max);
    distancia_haversine(lat_punto, lon_punto, lat_cercana, lon_cercana)
}

fn distancia_haversine(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    const R: f64 = 6371000.0;
    let dlat = (lat2 - lat1).to_radians();
    let dlon = (lon2 - lon1).to_radians();
    let a = (dlat / 2.0).sin().powi(2) + lat1.to_radians().cos() * lat2.to_radians().cos() * (dlon / 2.0).sin().powi(2);
    R * 2.0 * a.sqrt().atan2((1.0 - a).sqrt())
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
    use assertor::{assert_that, BooleanAssertion};

    #[test]
    fn pertenece_detecta_coordenadas_dentro_en_radio_y_fuera() {
        let geometria = GeometriaTramo {
            noroeste: Coordenadas::new(Latitud::new(40.5).unwrap(), Longitud::new(-4.0).unwrap()),
            noreste: Coordenadas::new(Latitud::new(40.5).unwrap(), Longitud::new(-3.5).unwrap()),
            suroeste: Coordenadas::new(Latitud::new(40.0).unwrap(), Longitud::new(-4.0).unwrap()),
            sureste: Coordenadas::new(Latitud::new(40.0).unwrap(), Longitud::new(-3.5).unwrap()),
        };
        let tramo = TramoDeRio::new("TR-001".to_string(), geometria).unwrap();
        assert_that!(tramo.pertenece(&Coordenadas::new(Latitud::new(40.25).unwrap(), Longitud::new(-3.75).unwrap()))).is_true();
        assert_that!(tramo.pertenece(&Coordenadas::new(Latitud::new(40.5008).unwrap(), Longitud::new(-3.75).unwrap()))).is_true();
        assert_that!(tramo.pertenece(&Coordenadas::new(Latitud::new(41.0).unwrap(), Longitud::new(-3.75).unwrap()))).is_false();
    }
}
