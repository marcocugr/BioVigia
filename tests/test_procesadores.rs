mod procesadores_html;

use biovigia::dominio::depredacion::{Depredacion, FaseVital, RedDepredacion};
use biovigia::dominio::especie::CatalogoEspecies;
use procesadores_html::globi::procesar_globi;
use procesadores_html::gisd::procesar_gisd;
use std::collections::BTreeSet;

#[test]
fn test_globi_con_filtro_presa_especifica() {
    let html = include_str!("fixtures/globi.html");
    
    // Procesar SOLO Procambarus clarkii
    let relaciones = procesar_globi(html, "Neogale vison", Some("Procambarus clarkii"));
    
    // Solo debe haber devuelto Procambarus clarkii
    assert_eq!(relaciones.len(), 1);
    assert_eq!(relaciones[0].0, "Neogale vison");
    assert_eq!(relaciones[0].1, "Procambarus clarkii");
}

#[test]
fn test_globi_sin_filtro_devuelve_todas() {
    let html = include_str!("fixtures/globi.html");
    
    // Procesar TODAS las presas
    let relaciones = procesar_globi(html, "Neogale vison", None);
    
    // Debe haber múltiples presas
    assert!(relaciones.len() > 1, "Debería haber múltiples presas cuando no hay filtro");
    
    // Todas deben tener el mismo depredador
    for (dep, _) in &relaciones {
        assert_eq!(dep, "Neogale vison");
    }
}

#[test]
fn test_globi_crea_red_depredacion() {
    let html = include_str!("fixtures/globi.html");
    let relaciones = procesar_globi(html, "Neogale vison", None);
    
    // Crear catálogo con las especies necesarias
    let mut catalogo = CatalogoEspecies::new();
    
    // Registrar depredador con nombre anterior
    catalogo.registrar(
        "Neogale vison".to_string(),
        vec!["visón americano".to_string()],
        true,
        vec!["Mustela vison".to_string()],
    ).unwrap();
    
    // Registrar presas (solo las que sabemos que están en el HTML)
    catalogo.registrar(
        "Procambarus clarkii".to_string(),
        vec!["cangrejo rojo americano".to_string()],
        false,
        vec![],
    ).unwrap();
    
    catalogo.registrar(
        "Lithobates".to_string(),
        vec!["rana".to_string()],
        false,
        vec![],
    ).unwrap();
    
    // Crear red y añadir depredaciones
    let mut red = RedDepredacion::new();
    
    for (dep, presa) in &relaciones {
        // GloBI no especifica fase vital → TODAS
        let fases: BTreeSet<FaseVital> = FaseVital::TODAS.iter().copied().collect();
        
        // Intentar crear la depredación (puede fallar si la presa no está en el catálogo)
        if let Ok(depredacion) = Depredacion::new(&catalogo, dep, presa, fases, None) {
            let _ = red.registrar(depredacion);
        }
    }
    
    // Verificar que la red tiene depredaciones del Procambarus clarkii
    let depredadores_cangrejo = red.depredadores_de_nombre_comun(&catalogo, "cangrejo").unwrap();
    assert!(depredadores_cangrejo.contains(&"Neogale vison"), 
        "La red debe contener Neogale vison como depredador del cangrejo");
}

#[test]
fn test_gisd_reconoce_vison_como_invasor() {
    let html = include_str!("fixtures/gisd.html");
    
    // Probar con nombre actual y anteriores
    let nombres = vec!["Neogale vison", "Mustela vison", "Neovison vison"];
    
    let es_invasora = procesar_gisd(html, &nombres);
    
    assert!(es_invasora, "GISD debe reconocer el visón como invasor");
}

#[test]
fn test_gisd_integra_con_catalogo() {
    let html = include_str!("fixtures/gisd.html");
    
    let mut catalogo = CatalogoEspecies::new();
    
    // Nombres a probar en GISD
    let nombres_probar = vec!["Neogale vison", "Mustela vison", "Neovison vison"];
    
    let es_invasora = procesar_gisd(html, &nombres_probar);
    
    // Registrar en catálogo con el booleano correcto
    let resultado = catalogo.registrar(
        "Neogale vison".to_string(),
        vec!["visón americano".to_string()],
        es_invasora, // true porque GISD la encontró
        vec!["Mustela vison".to_string(), "Neovison vison".to_string()],
    );
    
    assert!(resultado.is_ok(), "El visón debe registrarse correctamente");
    assert!(es_invasora, "GISD debe haber identificado al visón como invasor");
}

use procesadores_html::gbif::procesar_gbif;

#[test]
fn test_gbif_integra_con_catalogo_vison() {
    let html = include_str!("fixtures/gbif/vison.html");
    
    let datos = procesar_gbif(html).expect("Debe extraer datos del visón");
    
    let mut catalogo = CatalogoEspecies::new();
    
    // Usar los datos extraídos para registrar en el catálogo
    let resultado = catalogo.registrar(
        datos.nombre_cientifico_actual.clone(),
        datos.nombres_comunes_espanol.clone(),
        true, // Ya sabemos por GISD que es invasora
        datos.nombres_anteriores.clone(),
    );
    
    assert!(resultado.is_ok(), "Debe registrarse correctamente");
    assert_eq!(datos.nombre_cientifico_actual, "Neogale vison");
    assert!(datos.nombres_anteriores.len() >= 2, "Debe tener al menos 2 sinónimos");
    assert!(datos.nombres_comunes_espanol.len() > 0, "Debe tener nombres en español");
}

#[test]
fn test_gbif_integra_con_catalogo_cangrejo() {
    let html = include_str!("fixtures/gbif/cangrejo.html");
    
    let datos = procesar_gbif(html).expect("Debe extraer datos del cangrejo");
    
    let mut catalogo = CatalogoEspecies::new();
    
    let resultado = catalogo.registrar(
        datos.nombre_cientifico_actual.clone(),
        datos.nombres_comunes_espanol.clone(),
        false, // No es invasora (es la presa)
        datos.nombres_anteriores.clone(),
    );
    
    assert!(resultado.is_ok(), "Debe registrarse correctamente");
    assert_eq!(datos.nombre_cientifico_actual, "Procambarus clarkii");
}
