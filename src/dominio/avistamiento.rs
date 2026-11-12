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

    pub fn avistamientos_en_ventana(&self, fecha_referencia: SystemTime, dias: u64) -> Vec<&Avistamiento> {
        let ventana = std::time::Duration::from_secs(dias * 24 * 60 * 60);
        self.avistamientos
            .values()
            .filter(|a| {
                if let Ok(diferencia) = fecha_referencia.duration_since(a.fecha) {
                    diferencia <= ventana
                } else {
                    false
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assertor::VecAssertion;

    #[test]
    fn filtrar_avistamientos_por_ventana_temporal() {
        let mut catalogo = CatalogoEspecies::new();
        catalogo.registrar("Neogale vison".to_string(), "Visón".to_string(), true, vec![]).unwrap();
        let hoy = SystemTime::now();
        let hace_6_dias = hoy - std::time::Duration::from_secs(6 * 24 * 60 * 60);
        let hace_20_dias = hoy - std::time::Duration::from_secs(20 * 24 * 60 * 60);
        let coordenadas = Coordenadas::new(crate::dominio::coordenadas::Latitud::new(40.0).unwrap(), crate::dominio::coordenadas::Longitud::new(-3.0).unwrap());
        let evidencia = EvidenciaFotografica::new(None, vec!["https://ejemplo.com/foto.jpg".to_string()]).unwrap();
        let mut registro = RegistroAvistamientos::new();
        registro.registrar(Avistamiento::new(&catalogo, Fuente::INaturalist, "1".to_string(), "Neogale vison", hace_6_dias, coordenadas, None, evidencia.clone()).unwrap()).unwrap();
        registro.registrar(Avistamiento::new(&catalogo, Fuente::INaturalist, "2".to_string(), "Neogale vison", hace_20_dias, coordenadas, None, evidencia).unwrap()).unwrap();
        let resultado = registro.avistamientos_en_ventana(hoy, 15);
        assertor::assert_that!(resultado).has_length(1);
    }
}
