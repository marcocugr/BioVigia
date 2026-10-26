use std::collections::HashMap;
use std::fmt::Debug;

use crate::dominio::coordenadas::Coordenadas;
use crate::dominio::error::ErrorDominio;
use crate::dominio::texto::normalizar;

pub const RADIO_ALREDEDORES_METROS: f64 = 500.0;
const METROS_POR_GRADO: f64 = 111_194.93;

pub trait GeometriaTramo: Debug {
    fn contiene_coordenadas(
        &self,
        coordenadas: &Coordenadas,
        radio_metros: f64,
    ) -> Result<bool, ErrorDominio>;
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

    pub fn distancia_metros(&self, punto: &Coordenadas) -> f64 {
        self.puntos
            .windows(2)
            .map(|segmento| distancia_a_segmento(punto, &segmento[0], &segmento[1]))
            .fold(f64::INFINITY, f64::min)
    }
}

impl GeometriaTramo for LineaTramo {
    fn contiene_coordenadas(
        &self,
        coordenadas: &Coordenadas,
        radio_metros: f64,
    ) -> Result<bool, ErrorDominio> {
        if !radio_metros.is_finite() || radio_metros < 0.0 {
            return Err(ErrorDominio::RadioInvalido);
        }
        Ok(self.distancia_metros(coordenadas) <= radio_metros)
    }
}

fn distancia_a_segmento(punto: &Coordenadas, a: &Coordenadas, b: &Coordenadas) -> f64 {
    let escala_longitud = punto.latitud().grados().to_radians().cos();
    let a = en_metros(punto, a, escala_longitud);
    let b = en_metros(punto, b, escala_longitud);
    let (dx, dy) = (b.0 - a.0, b.1 - a.1);
    let largo_cuadrado = dx * dx + dy * dy;
    if largo_cuadrado == 0.0 {
        return a.0.hypot(a.1);
    }
    let t = ((-a.0) * dx + (-a.1) * dy) / largo_cuadrado;
    let t = t.clamp(0.0, 1.0);
    (a.0 + t * dx).hypot(a.1 + t * dy)
}

fn en_metros(origen: &Coordenadas, otro: &Coordenadas, escala_longitud: f64) -> (f64, f64) {
    let este = (otro.longitud().grados() - origen.longitud().grados()) * escala_longitud;
    let norte = otro.latitud().grados() - origen.latitud().grados();
    (este * METROS_POR_GRADO, norte * METROS_POR_GRADO)
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
        Ok(Self {
            codigo,
            nombre_rio,
            geometria,
        })
    }

    pub fn codigo(&self) -> &str {
        &self.codigo
    }

    pub fn nombre_rio(&self) -> &str {
        &self.nombre_rio
    }

    pub fn contiene_coordenadas(
        &self,
        coordenadas: &Coordenadas,
        radio_metros: f64,
    ) -> Result<bool, ErrorDominio> {
        self.geometria
            .contiene_coordenadas(coordenadas, radio_metros)
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dominio::coordenadas::{Latitud, Longitud};

    fn punto(latitud: f64, longitud: f64) -> Coordenadas {
        Coordenadas::new(Latitud::new(latitud).unwrap(), Longitud::new(longitud).unwrap())
    }

    fn linea(puntos: &[(f64, f64)]) -> LineaTramo {
        LineaTramo::new(puntos.iter().map(|&(lat, lon)| punto(lat, lon)).collect()).unwrap()
    }

    fn metros_en_longitud(metros: f64, latitud: f64) -> f64 {
        metros / (METROS_POR_GRADO * latitud.to_radians().cos())
    }

    fn tramo(codigo: &str, rio: &str) -> TramoDeRio {
        TramoDeRio::new(
            codigo.into(),
            rio.into(),
            Box::new(linea(&[(39.80, -4.55), (39.90, -4.55)])),
        )
        .unwrap()
    }

    #[test]
    fn el_radio_de_alrededores_por_defecto_son_500_metros() {
        assert_eq!(RADIO_ALREDEDORES_METROS, 500.0);
    }

    #[test]
    fn una_linea_con_menos_de_2_puntos_da_error() {
        assert_eq!(LineaTramo::new(vec![]), Err(ErrorDominio::TramoInsuficiente));
        assert_eq!(
            LineaTramo::new(vec![punto(39.8, -4.5)]),
            Err(ErrorDominio::TramoInsuficiente)
        );
    }

    #[test]
    fn una_linea_con_2_puntos_es_valida() {
        assert!(LineaTramo::new(vec![punto(39.8, -4.5), punto(39.9, -4.5)]).is_ok());
    }

    #[test]
    fn un_punto_sobre_el_tramo_esta_a_distancia_cero_y_entra_con_radio_cero() {
        let tramo = linea(&[(39.80, -4.55), (39.90, -4.55)]);
        let sobre_el_tramo = punto(39.85, -4.55);
        assert!(tramo.distancia_metros(&sobre_el_tramo) < 0.001);
        assert_eq!(tramo.contiene_coordenadas(&sobre_el_tramo, 0.0), Ok(true));
    }

    #[test]
    fn un_punto_a_menos_del_radio_entra() {
        let tramo = linea(&[(39.80, -4.55), (39.90, -4.55)]);
        let a_400_metros = punto(39.85, -4.55 + metros_en_longitud(400.0, 39.85));
        assert_eq!(tramo.contiene_coordenadas(&a_400_metros, 500.0), Ok(true));
    }

    #[test]
    fn un_punto_mas_alla_del_radio_no_entra() {
        let tramo = linea(&[(39.80, -4.55), (39.90, -4.55)]);
        let a_400_metros = punto(39.85, -4.55 + metros_en_longitud(400.0, 39.85));
        assert_eq!(tramo.contiene_coordenadas(&a_400_metros, 300.0), Ok(false));
    }

    #[test]
    fn un_punto_justo_en_el_borde_del_radio_entra() {
        let tramo = linea(&[(39.80, -4.55), (39.90, -4.55)]);
        let p = punto(39.85, -4.55 + metros_en_longitud(400.0, 39.85));
        let distancia = tramo.distancia_metros(&p);
        assert_eq!(tramo.contiene_coordenadas(&p, distancia), Ok(true));
        assert_eq!(tramo.contiene_coordenadas(&p, distancia - 0.001), Ok(false));
    }

    #[test]
    fn la_distancia_a_un_tramo_oblicuo_es_la_real_a_latitud_40() {
        let (lat_a, lon_a) = (40.0, -4.0);
        let (lat_b, lon_b) = (40.017986, -4.0 + 0.023478);
        let tramo = linea(&[(lat_a, lon_a), (lat_b, lon_b)]);
        let lat_medio = (lat_a + lat_b) / 2.0;
        let lon_medio = (lon_a + lon_b) / 2.0;
        let desplazamiento = 400.0 / 2.0_f64.sqrt();
        let p = punto(
            lat_medio + desplazamiento / METROS_POR_GRADO,
            lon_medio - metros_en_longitud(desplazamiento, lat_medio),
        );
        let distancia = tramo.distancia_metros(&p);
        assert!((distancia - 400.0).abs() < 4.0, "distancia calculada: {distancia}");
    }

    #[test]
    fn mas_alla_del_extremo_la_distancia_es_hasta_el_extremo() {
        let tramo = linea(&[(39.80, -4.55), (39.81, -4.55)]);
        let mil_metros_al_norte = punto(39.81 + 1000.0 / METROS_POR_GRADO, -4.55);
        let distancia = tramo.distancia_metros(&mil_metros_al_norte);
        assert!((distancia - 1000.0).abs() < 1.0, "distancia calculada: {distancia}");
    }

    #[test]
    fn un_segmento_con_dos_puntos_iguales_mide_hasta_ese_punto() {
        let tramo = linea(&[(39.80, -4.55), (39.80, -4.55)]);
        let trescientos_metros_al_norte = punto(39.80 + 300.0 / METROS_POR_GRADO, -4.55);
        let distancia = tramo.distancia_metros(&trescientos_metros_al_norte);
        assert!((distancia - 300.0).abs() < 0.5, "distancia calculada: {distancia}");
    }

    #[test]
    fn con_varios_segmentos_se_toma_el_mas_cercano() {
        let tramo = linea(&[(39.80, -4.55), (39.85, -4.55), (39.85, -4.40)]);
        let cerca_del_segundo = punto(39.85 + 100.0 / METROS_POR_GRADO, -4.47);
        let distancia = tramo.distancia_metros(&cerca_del_segundo);
        assert!((distancia - 100.0).abs() < 1.0, "distancia calculada: {distancia}");
    }

    #[test]
    fn un_radio_negativo_no_finito_o_nan_da_error() {
        let tramo = linea(&[(39.80, -4.55), (39.90, -4.55)]);
        let p = punto(39.85, -4.55);
        for radio in [-1.0, f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            assert_eq!(tramo.contiene_coordenadas(&p, radio), Err(ErrorDominio::RadioInvalido));
        }
    }

    #[test]
    fn un_tramo_sin_codigo_da_error() {
        let resultado = TramoDeRio::new("  ".into(), "Tajo".into(), Box::new(linea(&[(39.8, -4.5), (39.9, -4.5)])));
        assert_eq!(resultado.unwrap_err(), ErrorDominio::CodigoTramoRequerido);
    }

    #[test]
    fn un_tramo_sin_nombre_de_rio_da_error() {
        for nombre in ["", "   "] {
            let resultado = TramoDeRio::new("T1".into(), nombre.into(), Box::new(linea(&[(39.8, -4.5), (39.9, -4.5)])));
            assert_eq!(resultado.unwrap_err(), ErrorDominio::NombreRioRequerido);
        }
    }

    #[test]
    fn el_tramo_guarda_codigo_y_nombre_sin_espacios_de_los_lados() {
        let tramo = TramoDeRio::new(" T1 ".into(), " Tajo ".into(), Box::new(linea(&[(39.8, -4.5), (39.9, -4.5)]))).unwrap();
        assert_eq!(tramo.codigo(), "T1");
        assert_eq!(tramo.nombre_rio(), "Tajo");
    }

    #[test]
    fn dos_tramos_con_el_mismo_codigo_son_el_mismo_tramo() {
        assert_eq!(tramo("T1", "Tajo"), tramo("T1", "Ebro"));
        assert_ne!(tramo("T1", "Tajo"), tramo("T2", "Tajo"));
    }

    #[test]
    fn el_tramo_delega_en_su_geometria() {
        let tramo = tramo("T1", "Tajo");
        assert_eq!(tramo.contiene_coordenadas(&punto(39.85, -4.55), 10.0), Ok(true));
        assert_eq!(tramo.contiene_coordenadas(&punto(41.0, -1.0), 10.0), Ok(false));
    }

    #[test]
    fn un_tramo_registrado_se_encuentra_por_el_nombre_de_su_rio() {
        let mut rios = Rios::new();
        rios.registrar(tramo("T1", "Tajo")).unwrap();
        let tramos = rios.tramos_de_rio("Tajo").unwrap();
        assert_eq!(tramos.len(), 1);
        assert_eq!(tramos[0].codigo(), "T1");
    }

    #[test]
    fn el_nombre_del_rio_no_distingue_mayusculas_tildes_ni_espacios_de_los_lados() {
        let mut rios = Rios::new();
        rios.registrar(tramo("T1", "Río Guadiana")).unwrap();
        for nombre in ["río guadiana", " RIO GUADIANA ", "Rio Guadiana"] {
            assert_eq!(rios.tramos_de_rio(nombre).unwrap().len(), 1, "con {nombre:?}");
        }
    }

    #[test]
    fn rio_tajo_no_es_lo_mismo_que_tajo() {
        let mut rios = Rios::new();
        rios.registrar(tramo("T1", "Tajo")).unwrap();
        assert_eq!(rios.tramos_de_rio("Río Tajo").unwrap_err(), ErrorDominio::RioInexistente);
    }

    #[test]
    fn dos_tramos_del_mismo_rio_con_codigos_distintos_salen_los_dos() {
        let mut rios = Rios::new();
        rios.registrar(tramo("T1", "Tajo")).unwrap();
        rios.registrar(tramo("T2", "tajo")).unwrap();
        assert_eq!(rios.tramos_de_rio("Tajo").unwrap().len(), 2);
    }

    #[test]
    fn los_tramos_de_rios_distintos_no_se_mezclan() {
        let mut rios = Rios::new();
        rios.registrar(tramo("T1", "Tajo")).unwrap();
        rios.registrar(tramo("E1", "Ebro")).unwrap();
        assert_eq!(rios.tramos_de_rio("Ebro").unwrap()[0].codigo(), "E1");
    }

    #[test]
    fn registrar_dos_veces_el_mismo_codigo_da_error() {
        let mut rios = Rios::new();
        rios.registrar(tramo("T1", "Tajo")).unwrap();
        assert_eq!(rios.registrar(tramo("T1", "Tajo")), Err(ErrorDominio::TramoRepetido));
    }

    #[test]
    fn el_mismo_codigo_en_otro_rio_tambien_da_error() {
        let mut rios = Rios::new();
        rios.registrar(tramo("T1", "Tajo")).unwrap();
        assert_eq!(rios.registrar(tramo("T1", "Ebro")), Err(ErrorDominio::TramoRepetido));
    }

    #[test]
    fn un_rio_que_no_existe_da_rio_inexistente() {
        let mut rios = Rios::new();
        rios.registrar(tramo("T1", "Tajo")).unwrap();
        assert_eq!(rios.tramos_de_rio("Guadalquivir").unwrap_err(), ErrorDominio::RioInexistente);
    }

    #[test]
    fn sin_ningun_tramo_registrado_todo_rio_es_inexistente() {
        let rios = Rios::new();
        assert_eq!(rios.tramos_de_rio("Tajo").unwrap_err(), ErrorDominio::RioInexistente);
    }
}
