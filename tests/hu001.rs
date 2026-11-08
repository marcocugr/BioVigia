use std::time::{Duration, SystemTime};

use biovigia::aplicacion::inaturalist::INaturalist;
use biovigia::aplicacion::{ErrorFuente, reunir};
use biovigia::dominio::avistamiento::RegistroAvistamientos;
use biovigia::dominio::coordenadas::{Coordenadas, Latitud, Longitud};
use biovigia::dominio::error::ErrorDominio;
use biovigia::dominio::especie::CatalogoEspecies;
use biovigia::dominio::tramo::{Recuadro, TramoDeRio};

const VISON: &str = include_str!("fixtures/inaturalist/vison.html");
const DIA: u64 = 86_400;
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

fn zona_de_toledo() -> TramoDeRio {
    let recuadro = Recuadro::new(punto(39.0, -5.0), punto(41.0, -3.0));
    TramoDeRio::new("TOLEDO-01".into(), Box::new(recuadro)).unwrap()
}

#[test]
fn inaturalist_la_observacion_395192703_del_vison_se_registra_con_su_fecha_y_su_lugar() {
    let mut catalogo = CatalogoEspecies::new();
    catalogo
        .registrar(
            "Neogale vison".into(),
            "Visón americano".into(),
            true,
            vec![],
        )
        .unwrap();
    let mut registro = RegistroAvistamientos::new();
    reunir(&mut registro, &catalogo, &INaturalist, VISON).unwrap();
    let zona = zona_de_toledo();
    let el_dia_de_la_observacion = instante(OBSERVACION_VISON);
    let el_dia_anterior = instante(OBSERVACION_VISON - DIA);
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
