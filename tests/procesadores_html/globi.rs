/// Procesa el HTML de GloBI para extraer relaciones de depredación.
///
/// GloBI no especifica la fase vital, por lo que se asume TODAS las fases.
///
/// # Argumentos
///
/// * `html` - El contenido HTML de GloBI
/// * `nombre_depredador` - Nombre científico del depredador
/// * `filtro_presa` - Opcional: si se especifica, solo devuelve esa presa
///
/// # Retorna
///
/// Vec de tuplas `(depredador, presa)` listas para crear Depredacion
pub fn procesar_globi(
    html: &str,
    nombre_depredador: &str,
    filtro_presa: Option<&str>,
) -> Vec<(String, String)> {
    let mut relaciones = Vec::new();
    
    let mut resto = html;
    while let Some(inicio) = resto.find(r#"<div class="interaction search-result">"#) {
        resto = &resto[inicio..];
        
        let fin = resto[1..].find(r#"<div class="interaction search-result">"#)
            .map(|p| p + 1)
            .unwrap_or(resto.len());
        
        let bloque = &resto[..fin];
        
        if bloque.contains(r#"<div class="interaction _preysOn">"#) {
            let nombres = extraer_taxon_names(bloque);
            
            if nombres.len() >= 2 {
                let presa = &nombres[1];
                
                if let Some(filtro) = filtro_presa {
                    if presa == filtro {
                        relaciones.push((nombre_depredador.to_string(), presa.clone()));
                    }
                } else {
                    relaciones.push((nombre_depredador.to_string(), presa.clone()));
                }
            }
        }
        
        resto = &resto[fin..];
    }
    
    relaciones
}

fn extraer_taxon_names(html: &str) -> Vec<String> {
    let mut nombres = Vec::new();
    let mut resto = html;
    
    while let Some(inicio) = resto.find(r#"data-taxon-name=""#) {
        resto = &resto[inicio + r#"data-taxon-name=""#.len()..];
        
        let Some(fin) = resto.find('"') else { break };
        
        let nombre = &resto[..fin];
        if !nombre.is_empty() {
            nombres.push(nombre.to_string());
        }
        
        resto = &resto[fin + 1..];
    }
    
    nombres
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extrae_nombres() {
        let html = r#"<div data-taxon-name="Mustela vison"><div data-taxon-name="Procambarus clarkii">"#;
        let nombres = extraer_taxon_names(html);
        assert_eq!(nombres.len(), 2);
        assert_eq!(nombres[0], "Mustela vison");
        assert_eq!(nombres[1], "Procambarus clarkii");
    }
}
