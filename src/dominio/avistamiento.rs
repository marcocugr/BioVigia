use std::collections::HashMap;
use std::time::SystemTime;

use crate::dominio::coordenadas::Coordenadas;
use crate::dominio::error::ErrorDominio;
use crate::dominio::especie::CatalogoEspecies;
use crate::dominio::tramo::TramoDeRio;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Fuente {
    INaturalist,
    Gbif,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Avistamiento {
    fuente: Fuente,
    id_original: String,
    nombre_cientifico_especie: String,
    fecha: SystemTime,
    coordenadas: Coordenadas,
}

impl Avistamiento {
    pub fn new(
        catalogo: &CatalogoEspecies,
        fuente: Fuente,
        id_original: String,
        nombre_especie: &str,
        fecha: SystemTime,
        coordenadas: Coordenadas,
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
        })
    }

    fn es_reciente(&self, referencia: SystemTime, dias: u64) -> bool {
        let (Some(dia_referencia), Some(dia_avistamiento)) =
            (dia_utc(referencia), dia_utc(self.fecha))
        else {
            return false;
        };
        let antiguedad = dia_referencia - dia_avistamiento;
        antiguedad >= 0 && antiguedad as u64 <= dias
    }

    fn ocurrio_en(&self, zona: &TramoDeRio) -> bool {
        zona.contiene(&self.coordenadas)
    }
}

fn dia_utc(instante: SystemTime) -> Option<i64> {
    let segundos = instante
        .duration_since(SystemTime::UNIX_EPOCH)
        .ok()?
        .as_secs();
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

    pub fn que_cuentan(
        &self,
        nombre_cientifico: &str,
        zona: &TramoDeRio,
        referencia: SystemTime,
        dias: u64,
    ) -> Vec<&Avistamiento> {
        self.avistamientos
            .values()
            .filter(|a| {
                a.nombre_cientifico_especie == nombre_cientifico
                    && a.es_reciente(referencia, dias)
                    && a.ocurrio_en(zona)
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dominio::coordenadas::{Latitud, Longitud};
    use crate::dominio::tramo::Recuadro;
    use std::time::Duration;

    const DIA: u64 = 86_400;
    const REFERENCIA: u64 = 1_788_264_000;

    fn instante(segundos: u64) -> SystemTime {
        SystemTime::UNIX_EPOCH + Duration::from_secs(segundos)
    }

    fn punto(latitud: f64, longitud: f64) -> Coordenadas {
        Coordenadas::new(
            Latitud::new(latitud).unwrap(),
            Longitud::new(longitud).unwrap(),
        )
    }

    fn zona() -> TramoDeRio {
        let recuadro = Recuadro::new(punto(39.8, -4.6), punto(39.9, -4.5));
        TramoDeRio::new("TAJO-01".into(), Box::new(recuadro)).unwrap()
    }

    fn catalogo() -> CatalogoEspecies {
        let mut catalogo = CatalogoEspecies::new();
        for (cientifico, comun) in [
            ("Neogale vison", "Visón americano"),
            ("Micropterus salmoides", "Black bass"),
        ] {
            catalogo
                .registrar(cientifico.into(), comun.into(), true, vec![])
                .unwrap();
        }
        catalogo
    }

    fn avistamiento(id: &str, especie: &str, segundos: u64, latitud: f64) -> Avistamiento {
        Avistamiento::new(
            &catalogo(),
            Fuente::INaturalist,
            id.into(),
            especie,
            instante(segundos),
            punto(latitud, -4.55),
        )
        .unwrap()
    }

    fn ids(avistamientos: Vec<&Avistamiento>) -> Vec<&str> {
        let mut ids: Vec<&str> = avistamientos
            .iter()
            .map(|a| a.id_original.as_str())
            .collect();
        ids.sort_unstable();
        ids
    }

    #[test]
    fn cuentan_los_de_hace_x_dias_o_menos_y_no_los_mas_antiguos_ni_los_posteriores() {
        let mut registro = RegistroAvistamientos::new();
        for (id, segundos) in [
            ("mismo-dia", REFERENCIA),
            ("limite", REFERENCIA - 15 * DIA),
            ("antiguo", REFERENCIA - 16 * DIA),
            ("posterior", REFERENCIA + DIA),
        ] {
            registro
                .registrar(avistamiento(id, "Neogale vison", segundos, 39.85))
                .unwrap();
        }
        let cuentan = registro.que_cuentan("Neogale vison", &zona(), instante(REFERENCIA), 15);
        assert_eq!(ids(cuentan), vec!["limite", "mismo-dia"]);
    }

    #[test]
    fn solo_cuentan_los_de_la_especie_pedida_y_los_de_la_zona() {
        let mut registro = RegistroAvistamientos::new();
        for (id, especie, latitud) in [
            ("en-la-zona", "Neogale vison", 39.85),
            ("fuera-de-la-zona", "Neogale vison", 41.65),
            ("otra-especie", "Micropterus salmoides", 39.85),
        ] {
            registro
                .registrar(avistamiento(id, especie, REFERENCIA, latitud))
                .unwrap();
        }
        let cuentan = registro.que_cuentan("Neogale vison", &zona(), instante(REFERENCIA), 15);
        assert_eq!(ids(cuentan), vec!["en-la-zona"]);
    }
}
