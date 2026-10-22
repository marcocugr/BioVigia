use std::collections::HashMap;
use std::time::SystemTime;

use crate::dominio::coordenadas::Coordenadas;
use crate::dominio::error::ErrorDominio;
use crate::dominio::especie::CatalogoEspecies;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Fuente {
    INaturalist,
    Gbif,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GradoCalidad {
    Investigacion,
    NecesitaIdentificacion,
    Casual,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvidenciaFotografica {
    autor: String,
    urls: Vec<String>,
}

impl EvidenciaFotografica {
    const AUTOR_DESCONOCIDO: &'static str = "Anónimo";

    pub fn new(autor: Option<String>, urls: Vec<String>) -> Result<Self, ErrorDominio> {
        if !urls.iter().all(|url| es_url_valida(url)) {
            return Err(ErrorDominio::UrlFotoInvalida);
        }
        let autor = autor
            .filter(|a| !a.trim().is_empty())
            .unwrap_or_else(|| Self::AUTOR_DESCONOCIDO.to_owned());
        Ok(Self { autor, urls })
    }
}

fn es_url_valida(url: &str) -> bool {
    let con_esquema = url
        .strip_prefix("https://")
        .or_else(|| url.strip_prefix("http://"));
    con_esquema.is_some_and(|resto| !resto.is_empty() && !url.contains(char::is_whitespace))
}

#[derive(Debug, Clone, PartialEq)]
pub struct Avistamiento {
    fuente: Fuente,
    id_original: String,
    nombre_cientifico_especie: String,
    fecha: SystemTime,
    coordenadas: Coordenadas,
    calidad: Option<GradoCalidad>,
    evidencia: EvidenciaFotografica,
}

impl Avistamiento {
    pub fn new(
        catalogo: &CatalogoEspecies,
        fuente: Fuente,
        id_original: String,
        nombre_especie: &str,
        fecha: SystemTime,
        coordenadas: Coordenadas,
        calidad: Option<GradoCalidad>,
        evidencia: EvidenciaFotografica,
    ) -> Result<Self, ErrorDominio> {
        let id_original = id_original.trim().to_owned();
        if id_original.is_empty() {
            return Err(ErrorDominio::AvistamientoIncompleto);
        }
        let nombre_cientifico_especie = catalogo.especie_invasora(nombre_especie)?.to_owned();
        Ok(Self {
            fuente,
            id_original,
            nombre_cientifico_especie,
            fecha,
            coordenadas,
            calidad,
            evidencia,
        })
    }

    pub fn es_reciente(&self, referencia: SystemTime, dias: u64) -> bool {
        let Ok(ref_secs) = referencia.duration_since(SystemTime::UNIX_EPOCH) else {
            return false;
        };
        let Ok(self_secs) = self.fecha.duration_since(SystemTime::UNIX_EPOCH) else {
            return false;
        };
        if self_secs > ref_secs {
            return false;
        }
        let dias_ref = ref_secs.as_secs() / 86400;
        let dias_avistamiento = self_secs.as_secs() / 86400;
        dias_ref - dias_avistamiento <= dias
    }
}

#[derive(Debug, Clone, Default)]
pub struct RegistroAvistamientos {
    avistamientos: HashMap<(Fuente, String), Avistamiento>,
}

impl RegistroAvistamientos {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn registrar(&mut self, avistamiento: Avistamiento) -> Result<(), ErrorDominio> {
        let clave = (avistamiento.fuente, avistamiento.id_original.clone());
        if self.avistamientos.contains_key(&clave) {
            return Err(ErrorDominio::AvistamientoRepetido);
        }
        self.avistamientos.insert(clave, avistamiento);
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PresenciaDepredador {
    PresenciaDetectada { cantidad: usize },
    SinRegistrosRecientes,
}

impl PresenciaDepredador {
    pub fn desde_avistamientos(avistamientos: &[Avistamiento]) -> Self {
        match avistamientos.len() {
            0 => Self::SinRegistrosRecientes,
            n => Self::PresenciaDetectada { cantidad: n },
        }
    }
}
