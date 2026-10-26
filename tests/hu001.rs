use std::time::{Duration, SystemTime};

use biovigia::aplicacion::consulta::{consultar_zona, FuentesDeVerdad, PeticionZona, ResultadoDepredador};
use biovigia::aplicacion::procesadores_html::gbif::procesar_gbif;
use biovigia::aplicacion::procesadores_html::gisd::procesar_gisd;
use biovigia::aplicacion::procesadores_html::globi::procesar_globi;
use biovigia::aplicacion::procesadores_html::inaturalist::{procesar_inaturalist, ObservacionINaturalist};
use biovigia::dominio::avistamiento::{PresenciaDepredador, RegistroAvistamientos};
use biovigia::dominio::coordenadas::{Coordenadas, Latitud, Longitud};
use biovigia::dominio::depredacion::RedDepredacion;
use biovigia::dominio::error::ErrorDominio;
use biovigia::dominio::especie::CatalogoEspecies;
use biovigia::dominio::tramo::{LineaTramo, Rios, TramoDeRio};

const INATURALIST_VISON: &str = include_str!("fixtures/inaturalist/vison.html");
const INATURALIST_CANGREJO: &str = include_str!("fixtures/inaturalist/cangrejo.html");
const GBIF_VISON: &str = include_str!("fixtures/gbif/vison.html");
const GBIF_CANGREJO: &str = include_str!("fixtures/gbif/cangrejo.html");
const GISD: &str = include_str!("fixtures/gisd.html");
const GLOBI: &str = include_str!("fixtures/globi.html");

const DIA: u64 = 86_400;
const METROS_POR_GRADO: f64 = 111_194.93;

struct CasoReal {
    catalogo: CatalogoEspecies,
    red: RedDepredacion,
    registro: RegistroAvistamientos,
    rios: Rios,
    observacion_vison: ObservacionINaturalist,
    observacion_cangrejo: ObservacionINaturalist,
}

fn nombres_completos<'a>(actual: &'a str, anteriores: &'a [String]) -> Vec<&'a str> {
    std::iter::once(actual).chain(anteriores.iter().map(String::as_str)).collect()
}

fn punto(latitud: f64, longitud: f64) -> Coordenadas {
    Coordenadas::new(Latitud::new(latitud).unwrap(), Longitud::new(longitud).unwrap())
}

fn tramo_paralelo(codigo: &str, rio: &str, observacion: &ObservacionINaturalist, metros_al_este: f64) -> TramoDeRio {
    let desplazamiento = metros_al_este / (METROS_POR_GRADO * observacion.latitud.to_radians().cos());
    let longitud = observacion.longitud + desplazamiento;
    let linea = LineaTramo::new(vec![
        punto(observacion.latitud - 0.01, longitud),
        punto(observacion.latitud + 0.01, longitud),
    ])
    .unwrap();
    TramoDeRio::new(codigo.into(), rio.into(), Box::new(linea)).unwrap()
}

fn caso_real() -> CasoReal {
    let observacion_vison = procesar_inaturalist(INATURALIST_VISON).unwrap();
    let observacion_cangrejo = procesar_inaturalist(INATURALIST_CANGREJO).unwrap();
    let ficha_vison = procesar_gbif(GBIF_VISON, &observacion_vison.nombre_cientifico).unwrap();
    let ficha_cangrejo = procesar_gbif(GBIF_CANGREJO, &observacion_cangrejo.nombre_cientifico).unwrap();

    let nombres_vison = nombres_completos(&observacion_vison.nombre_cientifico, &ficha_vison.nombres_anteriores);
    let nombres_cangrejo = nombres_completos(&observacion_cangrejo.nombre_cientifico, &ficha_cangrejo.nombres_anteriores);

    let mut catalogo = CatalogoEspecies::new();
    catalogo
        .registrar(
            observacion_vison.nombre_cientifico.clone(),
            ficha_vison.nombres_comunes_espanol.clone(),
            procesar_gisd(GISD, &nombres_vison),
            ficha_vison.nombres_anteriores.clone(),
        )
        .unwrap();
    catalogo
        .registrar(
            observacion_cangrejo.nombre_cientifico.clone(),
            ficha_cangrejo.nombres_comunes_espanol.clone(),
            procesar_gisd(GISD, &nombres_cangrejo),
            ficha_cangrejo.nombres_anteriores.clone(),
        )
        .unwrap();

    let mut red = RedDepredacion::new();
    for relacion in procesar_globi(GLOBI, &observacion_vison.nombre_cientifico, None) {
        if let Ok(depredacion) = relacion.a_depredacion(&catalogo) {
            red.registrar(depredacion).unwrap();
        }
    }

    let mut registro = RegistroAvistamientos::new();
    registro
        .registrar(observacion_vison.a_avistamiento(&catalogo).unwrap())
        .unwrap();

    let mut rios = Rios::new();
    rios.registrar(tramo_paralelo("LUCILLOS-01", "Arroyo Lucillos", &observacion_vison, 200.0)).unwrap();
    rios.registrar(tramo_paralelo("ALEJADO-01", "Arroyo Alejado", &observacion_vison, 800.0)).unwrap();
    let ebro = LineaTramo::new(vec![punto(41.60, -0.90), punto(41.70, -0.90)]).unwrap();
    rios.registrar(TramoDeRio::new("EBRO-01".into(), "Ebro".into(), Box::new(ebro)).unwrap()).unwrap();

    CasoReal { catalogo, red, registro, rios, observacion_vison, observacion_cangrejo }
}

fn consultar(caso: &CasoReal, nombre_comun: &str, nombre_rio: &str, dias_despues: u64) -> Result<Vec<ResultadoDepredador>, ErrorDominio> {
    let peticion = PeticionZona {
        nombre_comun: nombre_comun.into(),
        nombre_rio: nombre_rio.into(),
        dias: 15,
        fecha_referencia: caso.observacion_vison.fecha + Duration::from_secs(dias_despues * DIA),
    };
    let fuentes = FuentesDeVerdad {
        catalogo: &caso.catalogo,
        red_depredacion: &caso.red,
        registro_avistamientos: &caso.registro,
        rios: &caso.rios,
    };
    consultar_zona(&peticion, &fuentes)
}

fn vison(cantidad: usize, caso: &CasoReal) -> ResultadoDepredador {
    ResultadoDepredador {
        nombre_cientifico: caso.observacion_vison.nombre_cientifico.clone(),
        cantidad_avistamientos: cantidad,
        presencia: PresenciaDepredador::desde_cantidad(cantidad),
    }
}

#[test]
fn hu001_el_pescador_consulta_el_arroyo_a_los_5_dias_y_hay_presencia_del_vison() {
    let caso = caso_real();
    let resultado = consultar(&caso, "cangrejo", "Arroyo Lucillos", 5).unwrap();
    println!("{resultado:#?}");
    assert_eq!(resultado, vec![vison(1, &caso)]);
    assert_eq!(resultado[0].presencia, PresenciaDepredador::PresenciaDetectada { cantidad: 1 });
}

#[test]
fn hu001_a_los_23_dias_el_mismo_arroyo_queda_sin_registros_recientes() {
    let caso = caso_real();
    let resultado = consultar(&caso, "cangrejo", "Arroyo Lucillos", 23).unwrap();
    println!("{resultado:#?}");
    assert_eq!(resultado, vec![vison(0, &caso)]);
    assert_eq!(resultado[0].presencia, PresenciaDepredador::SinRegistrosRecientes);
}

#[test]
fn hu001_el_ultimo_dia_de_la_ventana_todavia_cuenta_y_el_siguiente_no() {
    let caso = caso_real();
    assert_eq!(consultar(&caso, "cangrejo", "Arroyo Lucillos", 15), Ok(vec![vison(1, &caso)]));
    assert_eq!(consultar(&caso, "cangrejo", "Arroyo Lucillos", 16), Ok(vec![vison(0, &caso)]));
}

#[test]
fn hu001_un_arroyo_a_800_metros_del_avistamiento_no_cuenta() {
    let caso = caso_real();
    assert_eq!(consultar(&caso, "cangrejo", "Arroyo Alejado", 5), Ok(vec![vison(0, &caso)]));
}

#[test]
fn hu001_otro_rio_lejano_no_tiene_registros() {
    let caso = caso_real();
    assert_eq!(consultar(&caso, "cangrejo", "Ebro", 5), Ok(vec![vison(0, &caso)]));
}

#[test]
fn hu001_un_nombre_comun_que_no_existe_da_error() {
    let caso = caso_real();
    assert_eq!(consultar(&caso, "unicornio", "Arroyo Lucillos", 5), Err(ErrorDominio::NombreComunInexistente));
}

#[test]
fn hu001_un_rio_que_no_existe_da_error() {
    let caso = caso_real();
    assert_eq!(consultar(&caso, "cangrejo", "Guadalquivir", 5), Err(ErrorDominio::RioInexistente));
}

#[test]
fn hu001_solo_el_vison_es_depredador_invasor_del_cangrejo_segun_la_red() {
    let caso = caso_real();
    let depredadores = caso.red.depredadores_de_nombre_comun(&caso.catalogo, "cangrejo").unwrap();
    assert_eq!(depredadores, vec![caso.observacion_vison.nombre_cientifico.as_str()]);
}

#[test]
fn hu001_la_observacion_del_cangrejo_no_entra_como_avistamiento_porque_no_consta_invasora() {
    let caso = caso_real();
    let resultado = caso.observacion_cangrejo.a_avistamiento(&caso.catalogo);
    assert_eq!(
        resultado.map(|_| ()),
        Err(biovigia::aplicacion::procesadores_html::ErrorFuente::Dominio(ErrorDominio::EspecieNoInvasora))
    );
}

#[test]
fn hu001_la_fecha_se_lee_de_la_pagina_y_no_del_reloj() {
    let caso = caso_real();
    assert!(caso.observacion_vison.fecha > SystemTime::UNIX_EPOCH);
    let hoy_en_el_futuro = caso.observacion_vison.fecha + Duration::from_secs(400 * DIA);
    assert!(!caso.registro.avistamientos_de_especie("Neogale vison")[0].es_reciente(hoy_en_el_futuro, 15));
}
