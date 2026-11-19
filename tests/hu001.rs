mod common;

use biovigia::aplicacion::biovigia::{BioVigia, EstadoPresencia};
use common::{setup, get_tramo_from_csv};
use biovigia::dominio::avistamiento::RegistroAvistamientos;
use biovigia::dominio::coordenadas::{Coordenadas, Latitud, Longitud};
use biovigia::dominio::depredacion::RedDepredacion;
use biovigia::dominio::especie::CatalogoEspecies;
use biovigia::dominio::tramo::{GeometriaTramo, TramoDeRio};
use assertor::{assert_that, MapAssertion, EqualityAssertion};
use std::time::{SystemTime, Duration, UNIX_EPOCH};

#[test]
fn hu001_consultar_depredadores_de_trucha_devuelve_neogale_vison() {
    // Setup inicial: catálogo con 2 truchas (Salmo trutta y Oncorhynchus mykiss), red de depredación, avistamiento de visón en TR-001
    let mut biovigia = BioVigia::new(CatalogoEspecies::new(), RedDepredacion::new(), RegistroAvistamientos::new(), TramoDeRio::new("TEMP".to_string(), GeometriaTramo { noroeste: Coordenadas::new(Latitud::new(0.0).unwrap(), Longitud::new(0.0).unwrap()), noreste: Coordenadas::new(Latitud::new(0.0).unwrap(), Longitud::new(0.0).unwrap()), suroeste: Coordenadas::new(Latitud::new(0.0).unwrap(), Longitud::new(0.0).unwrap()), sureste: Coordenadas::new(Latitud::new(0.0).unwrap(), Longitud::new(0.0).unwrap()) }).unwrap());
    setup(&mut biovigia).unwrap();
    let fecha_avistamiento_vison = UNIX_EPOCH + Duration::from_secs(1_787_921_200); // 2025-06-23T15:00:00Z
    let fecha_consulta_5dias = fecha_avistamiento_vison + Duration::from_secs(5 * 24 * 60 * 60); // Consulta 5 días DESPUÉS del avistamiento
    let fecha_consulta_30dias = fecha_avistamiento_vison + Duration::from_secs(30 * 24 * 60 * 60); // Consulta 30 días DESPUÉS del avistamiento
    
    // Caso 1: Se consulta 5 días después del avistamiento (dentro de ventana de 15 días) en el tramo correcto
    // Resultado esperado: PresenciaDetectada porque el tiempo es reciente y la ubicación es correcta
    let resultado = biovigia.consultar_depredadores_en_zona("Trucha común", "TR-001", fecha_consulta_5dias, 15);
    assert_that!(resultado.get("Neogale vison").unwrap()).is_equal_to(&EstadoPresencia::PresenciaDetectada { avistamientos: 1 });
    
    // Caso 2: Se consulta 5 días después del avistamiento (dentro de ventana de 15 días) en un tramo diferente
    // Resultado esperado: SinRegistrosRecientes porque el tiempo es reciente pero la ubicación no es correcta
    biovigia.tramo = get_tramo_from_csv(1).unwrap();
    let resultado = biovigia.consultar_depredadores_en_zona("Trucha común", "TR-002", fecha_consulta_5dias, 15);
    assert_that!(resultado.get("Neogale vison").unwrap()).is_equal_to(&EstadoPresencia::SinRegistrosRecientes);
    
    // Caso 3: Se consulta 30 días después del avistamiento (fuera de ventana de 15 días) en el tramo correcto
    // Resultado esperado: SinRegistrosRecientes porque el tiempo no es reciente aunque la ubicación es correcta
    biovigia.tramo = get_tramo_from_csv(0).unwrap();
    let resultado = biovigia.consultar_depredadores_en_zona("Trucha común", "TR-001", fecha_consulta_30dias, 15);
    assert_that!(resultado.get("Neogale vison").unwrap()).is_equal_to(&EstadoPresencia::SinRegistrosRecientes);
    
    // Caso 4: Se consulta 30 días después del avistamiento (fuera de ventana de 15 días) en un tramo diferente
    // Resultado esperado: SinRegistrosRecientes porque el tiempo no es reciente y la ubicación no es correcta
    biovigia.tramo = get_tramo_from_csv(1).unwrap();
    let resultado = biovigia.consultar_depredadores_en_zona("Trucha común", "TR-002", fecha_consulta_30dias, 15);
    assert_that!(resultado.get("Neogale vison").unwrap()).is_equal_to(&EstadoPresencia::SinRegistrosRecientes);
    
    // Caso 5: Se elimina el avistamiento del visón del registro y se consulta en condiciones óptimas (5 días, tramo correcto)
    // Resultado esperado: SinRegistrosRecientes porque aunque el visón esté en el catálogo y deprede truchas, no hay avistamientos registrados
    biovigia.registro = RegistroAvistamientos::new();
    biovigia.tramo = get_tramo_from_csv(0).unwrap();
    let resultado = biovigia.consultar_depredadores_en_zona("Trucha común", "TR-001", fecha_consulta_5dias, 15);
    assert_that!(resultado.get("Neogale vison").unwrap()).is_equal_to(&EstadoPresencia::SinRegistrosRecientes);
}
