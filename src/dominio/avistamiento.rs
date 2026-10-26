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
    use crate::dominio::tramo::{GeometriaTramo, LineaTramo, TramoDeRio};
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

    #[derive(Debug)]
    struct GeometriaQueFalla;

    impl GeometriaTramo for GeometriaQueFalla {
        fn contiene_coordenadas(&self, _: &Coordenadas, _: f64) -> Result<bool, ErrorDominio> {
            Err(ErrorDominio::RadioInvalido)
        }
    }

    fn punto(latitud: f64, longitud: f64) -> Coordenadas {
        Coordenadas::new(Latitud::new(latitud).unwrap(), Longitud::new(longitud).unwrap())
    }

    fn rio_vertical(codigo: &str, nombre: &str, longitud: f64) -> TramoDeRio {
        let linea = LineaTramo::new(vec![punto(39.80, longitud), punto(39.90, longitud)]).unwrap();
        TramoDeRio::new(codigo.into(), nombre.into(), Box::new(linea)).unwrap()
    }

    fn avistamiento_con(id: &str, fuente: Fuente, longitud: f64) -> Avistamiento {
        Avistamiento::new(
            &catalogo_con_vison(),
            fuente,
            id.into(),
            "Neogale vison",
            instante(MEDIODIA_1_SEP_2026),
            punto(39.85, longitud),
            None,
            EvidenciaFotografica::new(None, vec![]).unwrap(),
        )
        .unwrap()
    }

    #[test]
    fn las_fotos_con_url_http_o_https_son_validas() {
        let evidencia = EvidenciaFotografica::new(
            Some("Marco C".into()),
            vec!["https://ejemplo.org/a.jpg".into(), "http://ejemplo.org/b.jpg".into()],
        );
        assert!(evidencia.is_ok());
    }

    #[test]
    fn una_foto_sin_esquema_con_espacios_o_vacia_da_error() {
        for url in ["ejemplo.org/a.jpg", "ftp://ejemplo.org/a.jpg", "https://", "https://ejemplo.org/a b.jpg", ""] {
            let evidencia = EvidenciaFotografica::new(None, vec![url.into()]);
            assert_eq!(evidencia, Err(ErrorDominio::UrlFotoInvalida), "con {url:?}");
        }
    }

    #[test]
    fn una_sola_url_mala_invalida_toda_la_evidencia() {
        let evidencia = EvidenciaFotografica::new(None, vec!["https://ejemplo.org/a.jpg".into(), "mala".into()]);
        assert_eq!(evidencia, Err(ErrorDominio::UrlFotoInvalida));
    }

    #[test]
    fn la_evidencia_sin_fotos_es_valida() {
        assert!(EvidenciaFotografica::new(Some("Marco C".into()), vec![]).is_ok());
    }

    #[test]
    fn sin_autor_o_con_autor_en_blanco_el_autor_es_anonimo() {
        for autor in [None, Some("".to_string()), Some("   ".to_string())] {
            let evidencia = EvidenciaFotografica::new(autor, vec![]).unwrap();
            assert_eq!(evidencia.autor, "Anónimo");
        }
    }

    #[test]
    fn con_autor_se_guarda_el_autor() {
        let evidencia = EvidenciaFotografica::new(Some("Marco C".into()), vec![]).unwrap();
        assert_eq!(evidencia.autor, "Marco C");
    }

    #[test]
    fn un_avistamiento_sin_identificador_original_da_error() {
        for id in ["", "   "] {
            let resultado = Avistamiento::new(
                &catalogo_con_vison(),
                Fuente::INaturalist,
                id.into(),
                "Neogale vison",
                instante(MEDIODIA_1_SEP_2026),
                punto(39.85, -4.55),
                None,
                EvidenciaFotografica::new(None, vec![]).unwrap(),
            );
            assert_eq!(resultado.unwrap_err(), ErrorDominio::AvistamientoIncompleto);
        }
    }

    #[test]
    fn el_identificador_original_se_guarda_sin_espacios_de_los_lados() {
        assert_eq!(avistamiento_con(" 395192703 ", Fuente::INaturalist, -4.55).id_original, "395192703");
    }

    #[test]
    fn un_avistamiento_de_una_especie_desconocida_da_error() {
        let resultado = Avistamiento::new(
            &catalogo_con_vison(),
            Fuente::INaturalist,
            "1".into(),
            "Canis lupus",
            instante(MEDIODIA_1_SEP_2026),
            punto(39.85, -4.55),
            None,
            EvidenciaFotografica::new(None, vec![]).unwrap(),
        );
        assert_eq!(resultado.unwrap_err(), ErrorDominio::EspecieDesconocida);
    }

    #[test]
    fn un_avistamiento_de_una_especie_no_invasora_da_error() {
        let mut catalogo = catalogo_con_vison();
        catalogo
            .registrar("Lutra lutra".into(), vec!["Nutria".into()], false, vec![])
            .unwrap();
        let resultado = Avistamiento::new(
            &catalogo,
            Fuente::INaturalist,
            "1".into(),
            "Lutra lutra",
            instante(MEDIODIA_1_SEP_2026),
            punto(39.85, -4.55),
            None,
            EvidenciaFotografica::new(None, vec![]).unwrap(),
        );
        assert_eq!(resultado.unwrap_err(), ErrorDominio::EspecieNoInvasora);
    }

    #[test]
    fn un_avistamiento_con_nombre_anterior_se_guarda_con_el_nombre_cientifico_vigente() {
        let avistamiento = Avistamiento::new(
            &catalogo_con_vison(),
            Fuente::INaturalist,
            "1".into(),
            "Mustela vison",
            instante(MEDIODIA_1_SEP_2026),
            punto(39.85, -4.55),
            Some(GradoCalidad::Casual),
            EvidenciaFotografica::new(None, vec![]).unwrap(),
        )
        .unwrap();
        assert_eq!(avistamiento.nombre_cientifico_especie, "Neogale vison");
        assert_eq!(avistamiento.calidad, Some(GradoCalidad::Casual));
    }

    #[test]
    fn registrar_el_mismo_avistamiento_de_la_misma_fuente_da_error() {
        let mut registro = RegistroAvistamientos::new();
        registro.registrar(avistamiento_con("1", Fuente::INaturalist, -4.55)).unwrap();
        let repetido = registro.registrar(avistamiento_con("1", Fuente::INaturalist, -4.55));
        assert_eq!(repetido, Err(ErrorDominio::AvistamientoRepetido));
    }

    #[test]
    fn el_mismo_identificador_en_otra_fuente_es_otro_avistamiento() {
        let mut registro = RegistroAvistamientos::new();
        registro.registrar(avistamiento_con("1", Fuente::INaturalist, -4.55)).unwrap();
        assert!(registro.registrar(avistamiento_con("1", Fuente::Gbif, -4.55)).is_ok());
    }

    #[test]
    fn avistamientos_de_especie_solo_devuelve_los_de_esa_especie() {
        let mut catalogo = catalogo_con_vison();
        catalogo
            .registrar("Micropterus salmoides".into(), vec!["Black bass".into()], true, vec![])
            .unwrap();
        let bass = Avistamiento::new(
            &catalogo,
            Fuente::INaturalist,
            "2".into(),
            "Micropterus salmoides",
            instante(MEDIODIA_1_SEP_2026),
            punto(39.85, -4.55),
            None,
            EvidenciaFotografica::new(None, vec![]).unwrap(),
        )
        .unwrap();
        let mut registro = RegistroAvistamientos::new();
        registro.registrar(avistamiento_con("1", Fuente::INaturalist, -4.55)).unwrap();
        registro.registrar(bass).unwrap();
        assert_eq!(registro.avistamientos_de_especie("Neogale vison").len(), 1);
        assert_eq!(registro.avistamientos_de_especie("Micropterus salmoides").len(), 1);
        assert!(registro.avistamientos_de_especie("Lutra lutra").is_empty());
    }

    #[test]
    fn un_avistamiento_dentro_del_radio_de_un_tramo_pertenece_al_rio() {
        let mut rios = Rios::new();
        rios.registrar(rio_vertical("T1", "Tajo", -4.55)).unwrap();
        let a_400_metros = avistamiento_con("1", Fuente::INaturalist, -4.55 + 0.00469);
        assert_eq!(a_400_metros.pertenece_a_rio(&rios, "Tajo"), Ok(true));
    }

    #[test]
    fn un_avistamiento_a_600_metros_de_un_tramo_no_pertenece_al_rio() {
        let mut rios = Rios::new();
        rios.registrar(rio_vertical("T1", "Tajo", -4.55)).unwrap();
        let a_600_metros = avistamiento_con("1", Fuente::INaturalist, -4.55 + 0.00703);
        assert_eq!(a_600_metros.pertenece_a_rio(&rios, "Tajo"), Ok(false));
    }

    #[test]
    fn un_avistamiento_cerca_de_otro_rio_no_pertenece_a_este() {
        let mut rios = Rios::new();
        rios.registrar(rio_vertical("T1", "Tajo", -4.55)).unwrap();
        rios.registrar(rio_vertical("E1", "Ebro", -0.90)).unwrap();
        let en_el_tajo = avistamiento_con("1", Fuente::INaturalist, -4.55);
        assert_eq!(en_el_tajo.pertenece_a_rio(&rios, "Ebro"), Ok(false));
        assert_eq!(en_el_tajo.pertenece_a_rio(&rios, "Tajo"), Ok(true));
    }

    #[test]
    fn si_el_rio_tiene_varios_tramos_basta_con_estar_en_uno() {
        let mut rios = Rios::new();
        rios.registrar(rio_vertical("T1", "Tajo", -3.00)).unwrap();
        rios.registrar(rio_vertical("T2", "Tajo", -4.55)).unwrap();
        let avistamiento = avistamiento_con("1", Fuente::INaturalist, -4.55);
        assert_eq!(avistamiento.pertenece_a_rio(&rios, "Tajo"), Ok(true));
    }

    #[test]
    fn un_rio_que_no_existe_da_error_y_nunca_false() {
        let rios = Rios::new();
        let avistamiento = avistamiento_con("1", Fuente::INaturalist, -4.55);
        assert_eq!(avistamiento.pertenece_a_rio(&rios, "Tajo"), Err(ErrorDominio::RioInexistente));
    }

    #[test]
    fn un_error_de_la_geometria_sale_como_error() {
        let mut rios = Rios::new();
        rios.registrar(TramoDeRio::new("T1".into(), "Tajo".into(), Box::new(GeometriaQueFalla)).unwrap())
            .unwrap();
        let avistamiento = avistamiento_con("1", Fuente::INaturalist, -4.55);
        assert_eq!(avistamiento.pertenece_a_rio(&rios, "Tajo"), Err(ErrorDominio::RadioInvalido));
    }

    #[test]
    fn la_presencia_con_0_avistamientos_es_sin_registros_recientes() {
        assert_eq!(PresenciaDepredador::desde_cantidad(0), PresenciaDepredador::SinRegistrosRecientes);
    }

    #[test]
    fn la_presencia_con_1_o_mas_avistamientos_es_presencia_detectada_con_su_cantidad() {
        assert_eq!(PresenciaDepredador::desde_cantidad(1), PresenciaDepredador::PresenciaDetectada { cantidad: 1 });
        assert_eq!(PresenciaDepredador::desde_cantidad(3), PresenciaDepredador::PresenciaDetectada { cantidad: 3 });
    }
}
