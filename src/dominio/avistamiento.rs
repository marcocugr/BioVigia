use std::collections::HashMap;
use std::time::SystemTime;

use crate::dominio::coordenadas::Coordenadas;
use crate::dominio::error::ErrorDominio;
use crate::dominio::especie::CatalogoEspecies;
use crate::dominio::tramo::{RADIO_ALREDEDORES_METROS, Rios};

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

    /// Indica si el avistamiento es del día de la referencia o de hasta `dias` días antes (días UTC).
    pub fn es_reciente(&self, referencia: SystemTime, dias: u64) -> bool {
        let (Some(dia_referencia), Some(dia_avistamiento)) =
            (dia_utc(referencia), dia_utc(self.fecha))
        else {
            return false;
        };
        let antiguedad = dia_referencia - dia_avistamiento;
        antiguedad >= 0 && antiguedad as u64 <= dias
    }

    /// Indica si el avistamiento cae en algún tramo del río o en sus alrededores.
    pub fn pertenece_a_rio(&self, rios: &Rios, nombre_rio: &str) -> Result<bool, ErrorDominio> {
        for tramo in rios.tramos_de_rio(nombre_rio)? {
            if tramo.contiene_coordenadas(&self.coordenadas, RADIO_ALREDEDORES_METROS)? {
                return Ok(true);
            }
        }
        Ok(false)
    }
}

/// Devuelve los días UTC desde el 1 de enero de 1970, o nada si el instante es anterior.
fn dia_utc(instante: SystemTime) -> Option<i64> {
    let segundos = instante.duration_since(SystemTime::UNIX_EPOCH).ok()?.as_secs();
    i64::try_from(segundos / 86_400).ok()
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

    pub fn avistamientos_de_especie(&self, nombre_cientifico: &str) -> Vec<&Avistamiento> {
        self.avistamientos
            .values()
            .filter(|a| a.nombre_cientifico_especie == nombre_cientifico)
            .collect()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PresenciaDepredador {
    PresenciaDetectada { cantidad: usize },
    SinRegistrosRecientes,
}

impl PresenciaDepredador {
    /// Convierte un número de avistamientos en su estado de presencia.
    pub fn desde_cantidad(cantidad: usize) -> Self {
        match cantidad {
            0 => Self::SinRegistrosRecientes,
            n => Self::PresenciaDetectada { cantidad: n },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dominio::coordenadas::{Latitud, Longitud};
    use std::time::Duration;

    const DIA: u64 = 86_400;
    const MEDIODIA_1_SEP_2026: u64 = 1_788_264_000;
    const MEDIANOCHE_28_AGO_2026: u64 = 1_787_875_200;

    fn instante(segundos: u64) -> SystemTime {
        SystemTime::UNIX_EPOCH + Duration::from_secs(segundos)
    }

    fn catalogo_con_vison() -> CatalogoEspecies {
        let mut catalogo = CatalogoEspecies::new();
        catalogo
            .registrar("Neogale vison".into(), vec!["Visón americano".into()], true, vec!["Mustela vison".into()])
            .unwrap();
        catalogo
    }

    fn avistamiento_en(segundos: u64) -> Avistamiento {
        Avistamiento::new(
            &catalogo_con_vison(),
            Fuente::INaturalist,
            "obs-1".into(),
            "Neogale vison",
            instante(segundos),
            Coordenadas::new(Latitud::new(39.85).unwrap(), Longitud::new(-4.55).unwrap()),
            None,
            EvidenciaFotografica::new(None, vec![]).unwrap(),
        )
        .unwrap()
    }

    #[test]
    fn un_avistamiento_de_hace_x_menos_1_dias_es_reciente() {
        let avistamiento = avistamiento_en(MEDIODIA_1_SEP_2026 - 14 * DIA);
        assert!(avistamiento.es_reciente(instante(MEDIODIA_1_SEP_2026), 15));
    }

    #[test]
    fn un_avistamiento_de_hace_exactamente_x_dias_es_reciente() {
        let avistamiento = avistamiento_en(MEDIODIA_1_SEP_2026 - 15 * DIA);
        assert!(avistamiento.es_reciente(instante(MEDIODIA_1_SEP_2026), 15));
    }

    #[test]
    fn un_avistamiento_de_hace_x_mas_1_dias_no_es_reciente() {
        let avistamiento = avistamiento_en(MEDIODIA_1_SEP_2026 - 16 * DIA);
        assert!(!avistamiento.es_reciente(instante(MEDIODIA_1_SEP_2026), 15));
    }

    #[test]
    fn un_avistamiento_del_mismo_dia_es_reciente_aunque_sea_mas_tarde_que_la_referencia() {
        let avistamiento = avistamiento_en(MEDIANOCHE_28_AGO_2026 + 8 * 3600 + 44 * 60);
        assert!(avistamiento.es_reciente(instante(MEDIANOCHE_28_AGO_2026), 15));
    }

    #[test]
    fn un_avistamiento_del_mismo_dia_y_anterior_a_la_referencia_es_reciente() {
        let avistamiento = avistamiento_en(MEDIANOCHE_28_AGO_2026 + 8 * 3600);
        assert!(avistamiento.es_reciente(instante(MEDIANOCHE_28_AGO_2026 + 12 * 3600), 15));
    }

    #[test]
    fn un_avistamiento_posterior_a_la_referencia_en_otro_dia_no_es_reciente() {
        let avistamiento = avistamiento_en(MEDIANOCHE_28_AGO_2026 + DIA);
        assert!(!avistamiento.es_reciente(instante(MEDIANOCHE_28_AGO_2026 + DIA - 1), 15));
    }

    #[test]
    fn con_cero_dias_solo_cuenta_el_mismo_dia() {
        let avistamiento = avistamiento_en(MEDIANOCHE_28_AGO_2026 + 3600);
        assert!(avistamiento.es_reciente(instante(MEDIANOCHE_28_AGO_2026 + 20 * 3600), 0));
        assert!(!avistamiento.es_reciente(instante(MEDIANOCHE_28_AGO_2026 + DIA), 0));
    }

    #[test]
    fn el_ultimo_segundo_del_dia_anterior_no_cuenta_con_cero_dias() {
        let avistamiento = avistamiento_en(MEDIANOCHE_28_AGO_2026 - 1);
        assert!(!avistamiento.es_reciente(instante(MEDIANOCHE_28_AGO_2026), 0));
    }

    #[test]
    fn un_instante_anterior_a_1970_no_es_reciente() {
        let avistamiento = Avistamiento::new(
            &catalogo_con_vison(),
            Fuente::INaturalist,
            "obs-2".into(),
            "Neogale vison",
            SystemTime::UNIX_EPOCH - Duration::from_secs(DIA),
            Coordenadas::new(Latitud::new(0.0).unwrap(), Longitud::new(0.0).unwrap()),
            None,
            EvidenciaFotografica::new(None, vec![]).unwrap(),
        )
        .unwrap();
        assert!(!avistamiento.es_reciente(instante(MEDIODIA_1_SEP_2026), 100_000));
    }
}
