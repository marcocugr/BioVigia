use std::time::{Duration, SystemTime};

use biovigia::aplicacion::consulta::{ResultadoDepredador, consultar_zona};
use biovigia::aplicacion::inaturalist::INaturalist;
use biovigia::aplicacion::{ErrorFuente, nomenclator, reunir};
use biovigia::dominio::avistamiento::{RegistroAvistamientos, SEGUNDOS_POR_DIA};
use biovigia::dominio::coordenadas::{Coordenadas, Latitud, Longitud};
use biovigia::dominio::depredacion::RedDepredacion;
use biovigia::dominio::error::ErrorDominio;
use biovigia::dominio::especie::CatalogoEspecies;
use biovigia::dominio::tramo::{Recuadro, TramoDeRio};

const VISON: &str = include_str!("fixtures/inaturalist/vison.html");
const OBSERVACION_VISON: u64 = 1_787_875_200;

fn instante(segundos: u64) -> SystemTime {
    SystemTime::UNIX_EPOCH + Duration::from_secs(segundos)
}

fn punto(latitud: f64, longitud: f64) -> Coordenadas {
    Coordenadas::new(
        Latitud::new(latitud).unwrap(),
        Longitud::new(longitud).unwrap(),
    )
}

fn zona(una_esquina: Coordenadas, otra_esquina: Coordenadas) -> TramoDeRio {
    let recuadro = Recuadro::new(una_esquina, otra_esquina);
    TramoDeRio::new("TRAMO-01".into(), Box::new(recuadro)).unwrap()
}

fn zona_de_toledo() -> TramoDeRio {
    zona(punto(39.0, -5.0), punto(41.0, -3.0))
}

fn zona_del_ebro() -> TramoDeRio {
    zona(punto(41.6, -0.95), punto(41.7, -0.85))
}

fn escenario() -> (CatalogoEspecies, RedDepredacion, RegistroAvistamientos) {
    let (catalogo, red) = nomenclator::cargar().unwrap();
    let mut registro = RegistroAvistamientos::new();
    reunir(&mut registro, &catalogo, &INaturalist, VISON).unwrap();
    (catalogo, red, registro)
}

fn consultar(
    especie_objetivo: &str,
    zona: &TramoDeRio,
    dias_despues: u64,
) -> Result<Vec<ResultadoDepredador>, ErrorDominio> {
    let (catalogo, red, registro) = escenario();
    let referencia = instante(OBSERVACION_VISON + dias_despues * SEGUNDOS_POR_DIA);
    consultar_zona(
        &catalogo,
        &red,
        &registro,
        especie_objetivo,
        zona,
        referencia,
        15,
    )
}

fn vison(avistamientos: usize) -> ResultadoDepredador {
    ResultadoDepredador {
        nombre_cientifico: "Neogale vison".into(),
        avistamientos,
    }
}

#[test]
fn inaturalist_la_observacion_395192703_del_vison_se_registra_con_su_fecha_y_su_lugar() {
    let (catalogo, _, mut registro) = escenario();
    let zona = zona_de_toledo();
    let el_dia_de_la_observacion = instante(OBSERVACION_VISON);
    let el_dia_anterior = instante(OBSERVACION_VISON - SEGUNDOS_POR_DIA);
    assert_eq!(
        registro
            .que_cuentan("Neogale vison", &zona, el_dia_de_la_observacion, 0)
            .len(),
        1
    );
    assert!(
        registro
            .que_cuentan("Neogale vison", &zona, el_dia_anterior, 0)
            .is_empty()
    );
    assert_eq!(
        reunir(&mut registro, &catalogo, &INaturalist, VISON),
        Err(ErrorFuente::Dominio(ErrorDominio::AvistamientoRepetido))
    );
}

#[test]
fn hu001_con_un_vison_avistado_hace_5_dias_en_la_zona_hay_un_avistamiento() {
    let resultado = consultar("Cangrejo rojo americano", &zona_de_toledo(), 5);
    assert_eq!(resultado, Ok(vec![vison(1)]));
}

#[test]
fn hu001_pasada_la_ventana_de_15_dias_el_vison_queda_sin_avistamientos() {
    let resultado = consultar("Cangrejo rojo americano", &zona_de_toledo(), 16);
    assert_eq!(resultado, Ok(vec![vison(0)]));
}

#[test]
fn hu001_un_vison_avistado_fuera_de_la_zona_no_cuenta() {
    let resultado = consultar("Cangrejo rojo americano", &zona_del_ebro(), 5);
    assert_eq!(resultado, Ok(vec![vison(0)]));
}

#[test]
fn hu001_una_especie_objetivo_sin_depredadores_da_una_lista_vacia() {
    let resultado = consultar("Visón americano", &zona_de_toledo(), 5);
    assert_eq!(resultado, Ok(vec![]));
}
