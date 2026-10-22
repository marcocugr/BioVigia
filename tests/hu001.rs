/// Test de integración HU001: Consulta de zona con especies invasoras
///
/// Flujo completo:
/// 1. iNaturalist → Obtener avistamientos de una especie
/// 2. GBIF → Enriquecer con sinónimos y nombres comunes
/// 3. GISD → Verificar si es invasora
/// 4. GloBI → Obtener relaciones de depredación
/// 5. Crear tramos de río usando TramoDeRio::desde_coordenadas (descubre nombre vía API)
/// 6. consultar_zona() → Analizar la zona con ventana temporal

mod procesadores_html;

use biovigia::dominio::avistamiento::{Avistamiento, EvidenciaFotografica, Fuente, GradoCalidad, RegistroAvistamientos};
use biovigia::dominio::coordenadas::{Coordenadas, Latitud, Longitud};
use biovigia::dominio::depredacion::{Depredacion, FaseVital, RedDepredacion};
use biovigia::dominio::especie::CatalogoEspecies;
use biovigia::dominio::tramo::{Rios, TramoDeRio, LineaTramo};
use procesadores_html::gbif::procesar_gbif;
use procesadores_html::gisd::procesar_gisd;
use procesadores_html::globi::procesar_globi;
use procesadores_html::inaturalist::procesar_inaturalist;
use std::collections::BTreeSet;
use std::time::{Duration, SystemTime};

#[test]
fn hu001_pescador_consulta_especie_invasora_en_rio() {
    // === PASO 1: Procesar avistamientos de iNaturalist ===
    
    // Avistamiento del visón (depredador invasor)
    let html_vison = include_str!("fixtures/inaturalist/vison.html");
    let (id_observacion_vison, nombre_cientifico_vison, _fecha_vison, lat_vison, lon_vison, calidad_vison, autor_vison, urls_fotos_vison) =
        procesar_inaturalist(html_vison)
            .expect("Debe procesar avistamiento del visón");
    
    // Avistamiento del cangrejo (presa)
    let html_cangrejo = include_str!("fixtures/inaturalist/cangrejo.html");
    let (_id_observacion_cangrejo, nombre_cientifico_cangrejo, _fecha_cangrejo, _lat_cangrejo, _lon_cangrejo, _calidad_cangrejo, _autor_cangrejo, _urls_fotos_cangrejo) =
        procesar_inaturalist(html_cangrejo)
            .expect("Debe procesar avistamiento del cangrejo");
    
    // === PASO 2: Enriquecer con GBIF (nombres científicos anteriores y nombres comunes en español) ===
    
    let html_gbif_vison = include_str!("fixtures/gbif/vison.html");
    let (nombres_cientificos_anteriores_vison, nombres_comunes_espanol_vison) = procesar_gbif(html_gbif_vison, &nombre_cientifico_vison)
        .expect("Debe obtener datos de GBIF del visón");
    
    let html_gbif_cangrejo = include_str!("fixtures/gbif/cangrejo.html");
    let (nombres_cientificos_anteriores_cangrejo, nombres_comunes_espanol_cangrejo) = procesar_gbif(html_gbif_cangrejo, &nombre_cientifico_cangrejo)
        .expect("Debe obtener datos de GBIF del cangrejo");
    
    // === PASO 3: Verificar si son invasoras con GISD ===
    
    let html_gisd = include_str!("fixtures/gisd.html");
    
    // Preparar todos los nombres del visón (actual + nombres científicos anteriores) para GISD
    let mut nombres_vison_completos: Vec<&str> = vec![&nombre_cientifico_vison];
    for nombre_anterior in &nombres_cientificos_anteriores_vison {
        nombres_vison_completos.push(nombre_anterior);
    }
    let vison_es_invasora = procesar_gisd(html_gisd, &nombres_vison_completos);
    
    // El cangrejo no aparecerá en GISD (no es invasor en este contexto)
    let mut nombres_cangrejo_completos: Vec<&str> = vec![&nombre_cientifico_cangrejo];
    for nombre_anterior in &nombres_cientificos_anteriores_cangrejo {
        nombres_cangrejo_completos.push(nombre_anterior);
    }
    let cangrejo_es_invasor = procesar_gisd(html_gisd, &nombres_cangrejo_completos);
    
    // === PASO 4: Crear catálogo de especies ===
    
    let mut catalogo = CatalogoEspecies::new();
    
    catalogo.registrar(
        nombre_cientifico_vison.clone(),
        nombres_comunes_espanol_vison.clone(),
        vison_es_invasora,
        nombres_cientificos_anteriores_vison.clone(),
    ).expect("Debe registrar el visón en el catálogo");
    
    catalogo.registrar(
        nombre_cientifico_cangrejo.clone(),
        nombres_comunes_espanol_cangrejo.clone(),
        cangrejo_es_invasor,
        nombres_cientificos_anteriores_cangrejo.clone(),
    ).expect("Debe registrar el cangrejo en el catálogo");
    
    // === PASO 5: Procesar relaciones de depredación con GloBI ===
    
    let html_globi = include_str!("fixtures/globi.html");
    let relaciones = procesar_globi(html_globi, &nombre_cientifico_vison, None);
    
    let mut red = RedDepredacion::new();
    
    for (depredador, presa) in &relaciones {
        // GloBI no especifica fase vital → TODAS
        let fases: BTreeSet<FaseVital> = FaseVital::TODAS.iter().copied().collect();
        
        // Intentar crear la depredación (puede fallar si la presa no está en el catálogo)
        if let Ok(depredacion) = Depredacion::new(&catalogo, depredador, presa, fases, None) {
            let _ = red.registrar(depredacion);
        }
    }
    
    // === PASO 6: Registrar avistamientos (solo del visón, que es invasor) ===
    
    let mut registro = RegistroAvistamientos::new();
    
    // Fecha base: 28 agosto 2026 (fecha del avistamiento del visón)
    let fecha_base = SystemTime::UNIX_EPOCH + Duration::from_secs(1_788_000_000); // ~2026-08-28
    
    // Coordenadas del visón
    let coords_vison = Coordenadas::new(
        Latitud::new(lat_vison).expect("Latitud válida"),
        Longitud::new(lon_vison).expect("Longitud válida")
    );
    
    // Evidencia fotográfica del visón
    let evidencia_vison = EvidenciaFotografica::new(
        Some(autor_vison),
        urls_fotos_vison,
    ).expect("Evidencia válida");
    
    // Mapear calidad de iNaturalist a GradoCalidad del dominio
    let grado_calidad_vison = match calidad_vison.as_str() {
        "Research Grade" => Some(GradoCalidad::Investigacion),
        "Needs ID" => Some(GradoCalidad::NecesitaIdentificacion),
        "Casual" => Some(GradoCalidad::Casual),
        _ => None,
    };
    
    let avistamiento_vison = Avistamiento::new(
        &catalogo,
        Fuente::INaturalist,
        id_observacion_vison,
        &nombre_cientifico_vison,
        fecha_base,
        coords_vison,
        grado_calidad_vison,
        evidencia_vison,
    ).expect("Debe crear avistamiento del visón");
    
    registro.registrar(avistamiento_vison).expect("Debe registrar avistamiento del visón");
    
    // === PASO 7: Crear tramos del río ===
    // En HU001, el pescador YA SABE en qué río está pescando
    // El sistema usa ese nombre para filtrar avistamientos cercanos
    
    let mut rios = Rios::new();
    
    // Crear línea del río en la zona del avistamiento
    let linea = LineaTramo::new(vec![
        // Línea que pasa cerca del avistamiento del visón
        Coordenadas::new(
            Latitud::new(lat_vison - 0.01).unwrap(),
            Longitud::new(lon_vison - 0.01).unwrap()
        ),
        Coordenadas::new(
            Latitud::new(lat_vison).unwrap(),
            Longitud::new(lon_vison).unwrap()
        ),
        Coordenadas::new(
            Latitud::new(lat_vison + 0.01).unwrap(),
            Longitud::new(lon_vison + 0.01).unwrap()
        ),
    ]).expect("Debe crear línea");
    
    // El pescador indica que está en este río (nombre proporcionado por el usuario)
    let nombre_rio_usuario = "Arroyo Lucillos";  // Basado en la localización de Lucillos, Toledo
    
    let tramo = TramoDeRio::new(
        "TRAMO-001".to_string(),
        nombre_rio_usuario.to_string(),
        coords_vison,  // Centro en las coordenadas del avistamiento
        5000.0,        // Radio de 5 km
        Box::new(linea),
    ).expect("Debe crear tramo");
    
    rios.registrar(tramo).expect("Debe registrar tramo");
    
    // === PASO 8: Consultar zona (HU001) ===
    
    // Fecha de consulta: 5 días después del avistamiento del visón
    let fecha_consulta = fecha_base + Duration::from_secs(5 * 86400);
    
    // Ventana temporal: 15 días (como especifica HU001)
    let dias_ventana = 15;
    
    // Nombre común a buscar (el pescador busca "cangrejo")
    let nombre_comun_presa = "cangrejo";
    
    let resultado = biovigia::dominio::consulta::consultar_zona(
        nombre_comun_presa,
        nombre_rio_usuario,  // Usar el nombre proporcionado por el pescador
        dias_ventana,
        fecha_consulta,
        &catalogo,
        &red,
        &registro,
        &rios,
    );
    
    // === VERIFICACIONES ===
    
    // Debe encontrar resultados
    assert!(resultado.is_ok(), "La consulta debe ejecutarse correctamente: {:?}", resultado.err());
    let resultados_depredadores = resultado.unwrap();
    
    // Debe identificar al visón como depredador del cangrejo
    assert!(
        !resultados_depredadores.is_empty(),
        "Debe encontrar depredadores del cangrejo"
    );
    
    // Buscar el visón en los resultados
    let contiene_vison = resultados_depredadores.iter().any(|r| {
        r.nombre_cientifico.contains("vison") || r.nombre_cientifico.contains("Neogale")
    });
    
    assert!(
        contiene_vison,
        "Debe identificar al visón americano como depredador del cangrejo. Encontrado: {:?}",
        resultados_depredadores
    );
    
    // Verificar que el visón estaba marcado como invasor
    assert!(vison_es_invasora, "GISD debe haber identificado al visón como invasor");
    
    // Verificar que se encontró avistamiento reciente del visón en el río
    let vison_result = resultados_depredadores.iter()
        .find(|r| r.nombre_cientifico.contains("vison") || r.nombre_cientifico.contains("Neogale"));
    
    if let Some(vison) = vison_result {
        assert!(
            vison.cantidad_avistamientos > 0,
            "Debe haber encontrado avistamientos del visón en el río {}",
            nombre_rio_usuario
        );
    }
    
    println!("✅ HU001 completada exitosamente");
    println!("   Río: {}", nombre_rio_usuario);
    println!("   Depredadores encontrados: {:?}", resultados_depredadores);
}
