use biovigia::aplicacion::consulta_depredadores::{consultar_depredadores_en_zona, EstadoPresencia};
use biovigia::aplicacion::setup::setup;
use biovigia::dominio::avistamiento::RegistroAvistamientos;
use biovigia::dominio::coordenadas::{Coordenadas, Latitud, Longitud};
use biovigia::dominio::depredacion::RedDepredacion;
use biovigia::dominio::especie::CatalogoEspecies;
use biovigia::dominio::tramo::{GeometriaTramo, TramoDeRio};
use assertor::{assert_that, MapAssertion, EqualityAssertion};
use std::time::SystemTime;

#[test]
fn hu001_consultar_depredadores_de_trucha_devuelve_neogale_vison() {
    let mut catalogo = CatalogoEspecies::new();
    let mut red = RedDepredacion::new();
    let mut registro = RegistroAvistamientos::new();
    let mut tramo = TramoDeRio::new("TEMP".to_string(), GeometriaTramo {
        noroeste: Coordenadas::new(Latitud::new(0.0).unwrap(), Longitud::new(0.0).unwrap()),
        noreste: Coordenadas::new(Latitud::new(0.0).unwrap(), Longitud::new(0.0).unwrap()),
        suroeste: Coordenadas::new(Latitud::new(0.0).unwrap(), Longitud::new(0.0).unwrap()),
        sureste: Coordenadas::new(Latitud::new(0.0).unwrap(), Longitud::new(0.0).unwrap()),
    }).unwrap();
    setup(&mut catalogo, &mut red, &mut registro, &mut tramo).unwrap();
    let resultado = consultar_depredadores_en_zona(&catalogo, &red, &registro, &tramo, "Trucha común", SystemTime::now(), 365);
    assert_that!(resultado).contains_key(&"Neogale vison".to_string());
    assert_that!(resultado.get("Neogale vison").unwrap()).is_equal_to(&EstadoPresencia::PresenciaDetectada { avistamientos: 1 });
}
