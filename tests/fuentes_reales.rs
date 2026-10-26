use std::time::{Duration, SystemTime};

use biovigia::aplicacion::procesadores_html::gbif::procesar_gbif;
use biovigia::aplicacion::procesadores_html::gisd::procesar_gisd;
use biovigia::aplicacion::procesadores_html::globi::procesar_globi;
use biovigia::aplicacion::procesadores_html::inaturalist::procesar_inaturalist;
use biovigia::dominio::avistamiento::GradoCalidad;

const INATURALIST_VISON: &str = include_str!("fixtures/inaturalist/vison.html");
const INATURALIST_CANGREJO: &str = include_str!("fixtures/inaturalist/cangrejo.html");
const GBIF_VISON: &str = include_str!("fixtures/gbif/vison.html");
const GBIF_CANGREJO: &str = include_str!("fixtures/gbif/cangrejo.html");
const GISD: &str = include_str!("fixtures/gisd.html");
const GLOBI: &str = include_str!("fixtures/globi.html");

fn medianoche_utc(segundos: u64) -> SystemTime {
    SystemTime::UNIX_EPOCH + Duration::from_secs(segundos)
}

#[test]
fn inaturalist_vison_da_los_datos_de_la_observacion_395192703() {
    let observacion = procesar_inaturalist(INATURALIST_VISON).unwrap();
    assert_eq!(observacion.id_observacion, "395192703");
    assert_eq!(observacion.nombre_cientifico, "Neogale vison");
    assert_eq!(observacion.fecha, medianoche_utc(1_787_875_200));
    assert!(observacion.latitud > 39.0 && observacion.latitud < 41.0);
    assert!(observacion.longitud > -5.0 && observacion.longitud < -3.0);
    assert_eq!(observacion.calidad, Some(GradoCalidad::Investigacion));
    assert!(observacion.autor.is_some());
    assert!(!observacion.urls_fotos.is_empty());
    assert!(observacion.urls_fotos.iter().all(|url| url.starts_with("https://")));
}

#[test]
fn inaturalist_cangrejo_da_los_datos_de_la_observacion_394426183() {
    let observacion = procesar_inaturalist(INATURALIST_CANGREJO).unwrap();
    assert_eq!(observacion.id_observacion, "394426183");
    assert_eq!(observacion.nombre_cientifico, "Procambarus clarkii");
    assert_eq!(observacion.fecha, medianoche_utc(1_787_616_000));
    assert!(observacion.latitud > 39.0 && observacion.latitud < 41.0);
    assert!(observacion.longitud > -5.0 && observacion.longitud < -3.0);
    assert_eq!(observacion.calidad, Some(GradoCalidad::Investigacion));
    assert!(!observacion.urls_fotos.is_empty());
}

#[test]
fn gbif_vison_da_sinonimos_y_nombre_comun_en_espanol() {
    let ficha = procesar_gbif(GBIF_VISON, "Neogale vison").unwrap();
    assert_eq!(ficha.nombres_anteriores, vec!["Mustela vison", "Neovison vison", "Putorius vison"]);
    assert!(ficha.nombres_comunes_espanol.iter().any(|n| n.contains("Visón")));
}

#[test]
fn gbif_cangrejo_da_los_cuatro_nombres_comunes_en_espanol() {
    let ficha = procesar_gbif(GBIF_CANGREJO, "Procambarus clarkii").unwrap();
    for esperado in ["Acocil rojo", "Cangrejo americano", "Cangrejo de río americano", "Cangrejo rojo americano"] {
        assert!(ficha.nombres_comunes_espanol.iter().any(|n| n == esperado), "falta {esperado:?}");
    }
    assert!(!ficha.nombres_comunes_espanol.iter().any(|n| n.contains("American Crayfish")));
}

#[test]
fn gisd_reconoce_al_vison_con_cualquiera_de_sus_nombres() {
    assert!(procesar_gisd(GISD, &["Neogale vison", "Mustela vison", "Neovison vison"]));
}

#[test]
fn gisd_no_reconoce_una_especie_que_no_aparece() {
    assert!(!procesar_gisd(GISD, &["Procambarus clarkii"]));
    assert!(!procesar_gisd(GISD, &["Especie Falsa"]));
}

#[test]
fn globi_con_filtro_devuelve_solo_la_presa_pedida() {
    let relaciones = procesar_globi(GLOBI, "Neogale vison", Some("Procambarus clarkii"));
    assert_eq!(relaciones.len(), 1);
    assert_eq!(relaciones[0].depredador, "Neogale vison");
    assert_eq!(relaciones[0].presa, "Procambarus clarkii");
}

#[test]
fn globi_sin_filtro_devuelve_varias_presas_distintas_del_mismo_depredador() {
    let relaciones = procesar_globi(GLOBI, "Neogale vison", None);
    assert!(relaciones.len() > 1);
    assert!(relaciones.iter().all(|r| r.depredador == "Neogale vison"));
    let mut presas: Vec<&str> = relaciones.iter().map(|r| r.presa.as_str()).collect();
    presas.sort_unstable();
    presas.dedup();
    assert_eq!(presas.len(), relaciones.len());
}
