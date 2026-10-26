use std::collections::HashMap;
use std::fmt::Debug;

use crate::dominio::coordenadas::Coordenadas;
use crate::dominio::error::ErrorDominio;

pub const RADIO_ALREDEDORES_METROS: f64 = 500.0;

pub trait GeometriaTramo: Debug {
    fn contiene_coordenadas(&self, coordenadas: &Coordenadas, radio_metros: f64) -> Result<bool, ErrorDominio>;
}

#[derive(Debug, PartialEq)]
pub struct LineaTramo {
    puntos: Vec<Coordenadas>,
}

impl LineaTramo {
    pub fn new(puntos: Vec<Coordenadas>) -> Result<Self, ErrorDominio> {
        if puntos.len() < 2 {
            return Err(ErrorDominio::TramoInsuficiente);
        }
        Ok(Self { puntos })
    }
}

impl GeometriaTramo for LineaTramo {
    fn contiene_coordenadas(&self, coordenadas: &Coordenadas, radio_metros: f64) -> Result<bool, ErrorDominio> {
        if !radio_metros.is_finite() || radio_metros < 0.0 {
            return Err(ErrorDominio::RadioInvalido);
        }

        let distancia_minima = distancia_punto_a_linea(coordenadas, &self.puntos);
        Ok(distancia_minima <= radio_metros)
    }
}

fn distancia_punto_a_linea(punto: &Coordenadas, linea: &[Coordenadas]) -> f64 {
    linea
        .windows(2)
        .map(|segmento| distancia_punto_a_segmento(punto, &segmento[0], &segmento[1]))
        .fold(f64::INFINITY, f64::min)
}

fn distancia_punto_a_segmento(p: &Coordenadas, a: &Coordenadas, b: &Coordenadas) -> f64 {
    let lat_p = p.latitud().grados();
    let lon_p = p.longitud().grados();
    let lat_a = a.latitud().grados();
    let lon_a = a.longitud().grados();
    let lat_b = b.latitud().grados();
    let lon_b = b.longitud().grados();

    let dx = lon_b - lon_a;
    let dy = lat_b - lat_a;
    
    if dx == 0.0 && dy == 0.0 {
        return distancia_haversine(lat_p, lon_p, lat_a, lon_a);
    }

    let t = ((lon_p - lon_a) * dx + (lat_p - lat_a) * dy) / (dx * dx + dy * dy);
    let t_clamped = t.clamp(0.0, 1.0);

    let lat_proyeccion = lat_a + t_clamped * dy;
    let lon_proyeccion = lon_a + t_clamped * dx;

    distancia_haversine(lat_p, lon_p, lat_proyeccion, lon_proyeccion)
}

fn distancia_haversine(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    const R: f64 = 6371000.0;
    
    let lat1_rad = lat1.to_radians();
    let lat2_rad = lat2.to_radians();
    let dlat = (lat2 - lat1).to_radians();
    let dlon = (lon2 - lon1).to_radians();

    let a = (dlat / 2.0).sin().powi(2) 
        + lat1_rad.cos() * lat2_rad.cos() * (dlon / 2.0).sin().powi(2);
    let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());

    R * c
}

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

    pub fn contiene_coordenadas(&self, coordenadas: &Coordenadas, radio_metros: f64) -> Result<bool, ErrorDominio> {
        self.geometria.contiene_coordenadas(coordenadas, radio_metros)
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

    pub fn tramos_de_rio(&self, nombre_rio: &str) -> Result<&[TramoDeRio], ErrorDominio> {
        let clave = normalizar(nombre_rio);
        self.tramos_por_rio
            .get(&clave)
            .map(Vec::as_slice)
            .ok_or(ErrorDominio::RioInexistente)
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
