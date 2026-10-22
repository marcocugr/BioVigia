/// Procesa el HTML de GISD para determinar si una especie es invasora.
///
/// GISD puede no encontrar algunos nombres científicos (actuales o anteriores).
/// Solo si encuentra la especie con alguno de los nombres, ES invasora.
///
/// # Argumentos
///
/// * `html` - El contenido HTML de GISD
/// * `nombres_cientificos` - Lista de nombres a probar (actual + anteriores)
///
/// # Retorna
///
/// `true` si la especie aparece en GISD (es invasora), `false` si no aparece
pub fn procesar_gisd(html: &str, nombres_cientificos: &[&str]) -> bool {
    // Si el HTML dice "0 species", no encontró nada
    if html.contains(r#"<strong>0</strong> species"#) {
        return false;
    }
    
    // Si dice "1 species" o más, verificar que algún nombre aparece
    if html.contains(r#"<strong>1</strong> species"#) || html.contains(r#"species"#) {
        // Buscar si alguno de los nombres científicos aparece en el HTML
        for nombre in nombres_cientificos {
            if html.contains(nombre) {
                return true;
            }
        }
    }
    
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gisd_encuentra_con_nombre_anterior() {
        let html = include_str!("../fixtures/gisd.html");
        
        // El HTML contiene "Neovison vison" que es un nombre anterior
        let nombres = vec!["Neogale vison", "Mustela vison", "Neovison vison"];
        
        assert!(procesar_gisd(html, &nombres), 
            "GISD debe reconocer el visón como invasor con nombre anterior");
    }

    #[test]
    fn gisd_no_encuentra_especie_inexistente() {
        let html = r#"<div id="info-count">The query resulted in <strong>0</strong> species"#;
        
        let nombres = vec!["Especie Falsa"];
        
        assert!(!procesar_gisd(html, &nombres), 
            "GISD no debe reconocer especies que no encuentra");
    }

    #[test]
    fn gisd_html_vacio() {
        assert!(!procesar_gisd("", &["Cualquier Nombre"]));
    }
}
