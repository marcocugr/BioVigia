mod common;

use biovigia::aplicacion::biovigia::{BioVigia, EstadoPresencia};
use common::{setup, get_tramo_from_csv};
use biovigia::dominio::avistamiento::RegistroAvistamientos;
use biovigia::dominio::coordenadas::{Coordenadas, Latitud, Longitud};
use biovigia::dominio::depredacion::RedDepredacion;
use biovigia::dominio::especie::CatalogoEspecies;
use biovigia::dominio::tramo::{GeometriaTramo, TramoDeRio};
use assertor::{assert_that, MapAssertion, EqualityAssertion};

#[test]
fn hu001_consultar_depredadores_de_trucha_devuelve_neogale_vison() {
    // Setup inicial: catálogo con 2 truchas (Salmo trutta y Oncorhynchus mykiss), red de depredación, avistamiento de visón en TR-001
    let mut biovigia = BioVigia::new(CatalogoEspecies::new(), RedDepredacion::new(), RegistroAvistamientos::new(), TramoDeRio::new("TEMP".to_string(), GeometriaTramo { noroeste: Coordenadas::new(Latitud::new(0.0).unwrap(), Longitud::new(0.0).unwrap()), noreste: Coordenadas::new(Latitud::new(0.0).unwrap(), Longitud::new(0.0).unwrap()), suroeste: Coordenadas::new(Latitud::new(0.0).unwrap(), Longitud::new(0.0).unwrap()), sureste: Coordenadas::new(Latitud::new(0.0).unwrap(), Longitud::new(0.0).unwrap()) }).unwrap());
    setup(&mut biovigia).unwrap();
    
    // Caso 1: Visón avistado recientemente en TR-001 → debe aparecer como depredador detectado de ambas especies de trucha
    let resultado = biovigia.consultar_depredadores_en_zona("Trucha común", "TR-001");
    assert_that!(resultado.get("Neogale vison").unwrap()).is_equal_to(&EstadoPresencia::PresenciaDetectada { avistamientos: 1 });
    
    // Caso 2: Consultar en TR-002 (tramo en otra zona geográfica) → visón NO está en ese tramo
    biovigia.tramo = get_tramo_from_csv(1).unwrap();
    let resultado = biovigia.consultar_depredadores_en_zona("Trucha común", "TR-002");
    assert_that!(resultado.get("Neogale vison").unwrap()).is_equal_to(&EstadoPresencia::SinRegistrosRecientes);
}
