/// Procesa el HTML de GBIF para extraer datos de una especie.
///
/// De la ficha de GBIF se extrae:
/// - Sinónimos (nombres científicos anteriores)
/// - Nombres comunes en español
///
/// # Argumentos
///
/// * `html` - El contenido HTML de una ficha de GBIF
/// * `nombre_cientifico` - El nombre científico usado para buscar (nombre actual)
///
/// # Retorna
///
/// Tupla con: `(nombres_anteriores, nombres_comunes_espanol)`
/// - `nombres_anteriores`: Vec<String> con sinónimos y combinaciones
/// - `nombres_comunes_espanol`: Vec<String> con nombres comunes SOLO en español
pub fn procesar_gbif(html: &str, nombre_cientifico: &str) -> Option<(Vec<String>, Vec<String>)> {
    // HTML vacío o muy pequeño no es válido
    if html.len() < 100 {
        return None;
    }
    
    // Buscar sinónimos en el HTML
    let nombres_anteriores = extraer_sinonimos(html, nombre_cientifico);
    
    // Extraer nombres comunes en español
    let nombres_comunes = extraer_nombres_comunes_espanol(html);
    
    // Al menos debería tener nombres comunes para ser válido
    if nombres_comunes.is_empty() && nombres_anteriores.is_empty() {
        return None;
    }
    
    Some((nombres_anteriores, nombres_comunes))
}

fn extraer_sinonimos(html: &str, nombre_actual: &str) -> Vec<String> {
    let mut sinonimos = Vec::new();
    
    // Buscar nombres en formato itálico: <i>Genus species</i>
    let mut resto = html;
    while let Some(pos_i) = resto.find("<i>") {
        let inicio_nombre = pos_i + 3;
        if let Some(pos_cierre) = resto[inicio_nombre..].find("</i>") {
            let candidato = &resto[inicio_nombre..inicio_nombre + pos_cierre];
            
            // Validar que es un nombre científico (dos palabras, primera mayúscula)
            let partes: Vec<&str> = candidato.split_whitespace().collect();
            if partes.len() >= 2 {
                let nombre = format!("{} {}", partes[0], partes[1]);
                
                // No incluir el nombre actual ni duplicados
                if nombre != nombre_actual && !sinonimos.contains(&nombre) {
                    // Validar que parece nombre científico
                    if let Some(primera_letra) = nombre.chars().next() {
                        if primera_letra.is_uppercase() && nombre.chars().all(|c| c.is_alphabetic() || c.is_whitespace()) {
                            sinonimos.push(nombre);
                        }
                    }
                }
            }
            
            resto = &resto[inicio_nombre + pos_cierre + 4..];
        } else {
            break;
        }
    }
    
    sinonimos
}

fn extraer_nombres_comunes_espanol(html: &str) -> Vec<String> {
    let mut nombres = Vec::new();
    
    // Buscar entradas JSON con "@language": "spa"
    let mut resto = html;
    while let Some(pos_spa) = resto.find(r#""@language": "spa""#) {
        // Buscar el @value correspondiente
        if let Some(pos_value) = resto[pos_spa..pos_spa + 200].find(r#""@value": ""#) {
            let inicio_valor = pos_spa + pos_value + r#""@value": ""#.len();
            if let Some(fin) = resto[inicio_valor..inicio_valor + 100].find('"') {
                let nombre = &resto[inicio_valor..inicio_valor + fin];
                if !nombre.is_empty() {
                    nombres.push(nombre.trim().to_string());
                }
            }
        }
        
        resto = &resto[pos_spa + 20..];
    }
    
    nombres
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gbif_extrae_datos_vison() {
        let html = include_str!("../fixtures/gbif/vison.html");
        
        let (nombres_anteriores, nombres_comunes) = procesar_gbif(html, "Neogale vison")
            .expect("Debe extraer datos del visón");
        
        // Debe encontrar sinónimos del visón
        assert!(nombres_anteriores.len() > 0, "Debe encontrar al menos un sinónimo");
        
        // Debe tener nombres comunes en español
        assert!(nombres_comunes.len() > 0, "Debe encontrar nombres en español");
        assert!(nombres_comunes.iter().any(|n| n.contains("Visón") || n.contains("visón")));
    }

    #[test]
    fn gbif_extrae_datos_cangrejo() {
        let html = include_str!("../fixtures/gbif/cangrejo.html");
        
        let (_, nombres_comunes) = procesar_gbif(html, "Procambarus clarkii")
            .expect("Debe extraer datos del cangrejo");
        
        assert!(nombres_comunes.len() > 0, "Debe encontrar nombres en español");
    }

    #[test]
    fn gbif_html_vacio() {
        assert!(procesar_gbif("", "Nombre cualquiera").is_none());
    }
}
