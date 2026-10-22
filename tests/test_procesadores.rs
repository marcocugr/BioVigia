mod procesadores_html;

use biovigia::dominio::depredacion::{Depredacion, FaseVital, RedDepredacion};
use biovigia::dominio::especie::CatalogoEspecies;
use procesadores_html::globi::procesar_globi;
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
