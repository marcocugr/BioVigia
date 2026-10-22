/// Procesa el HTML de iNaturalist para extraer datos de un avistamiento.
///
/// # Argumentos
///
/// * `html` - El contenido HTML de una observación de iNaturalist
///
/// # Retorna
///
/// Tupla con: `(id_observacion, nombre_cientifico, fecha, latitud, longitud, calidad, autor, urls_fotos)`
/// - `id_observacion`: String - ID de la observación en iNaturalist
/// - `nombre_cientifico`: String - Nombre científico de la especie
/// - `fecha`: String - Formato "YYYY-MM-DD"
/// - `latitud`: f64 - Coordenada latitud
/// - `longitud`: f64 - Coordenada longitud
/// - `calidad`: String - "Research Grade", "Needs ID", o "Casual"
/// - `autor`: String - Nombre del observador
/// - `urls_fotos`: Vec<String> - URLs de las fotos
pub fn procesar_inaturalist(html: &str) -> Option<(String, String, String, f64, f64, String, String, Vec<String>)> {
    let id = extraer_id_observacion(html)?;
    let nombre = extraer_nombre_cientifico(html)?;
    let fecha = extraer_fecha(html)?;
    let (lat, lon) = extraer_coordenadas(html)?;
    let calidad = extraer_calidad(html).unwrap_or_else(|| "Casual".to_string());
    let autor = extraer_autor(html)?;
    let fotos = extraer_urls_fotos(html);
    
    Some((id, nombre, fecha, lat, lon, calidad, autor, fotos))
}

fn extraer_calidad(html: &str) -> Option<String> {
    // Buscar "Research Grade", "Needs ID", "Casual"
    if html.contains("Research Grade") || html.contains("research ") {
        return Some("Research Grade".to_string());
    }
    if html.contains("Needs ID") || html.contains("needs-id") {
        return Some("Needs ID".to_string());
    }
    if html.contains("Casual") || html.contains("casual") {
        return Some("Casual".to_string());
    }
    None
}

fn extraer_coordenadas(html: &str) -> Option<(f64, f64)> {
    // Buscar coordenadas en el JSON embebido o en los atributos del mapa
    // Patrón 1: Buscar en URLs del mapa que contienen @lat,lon
    if let Some(pos) = html.find("/@") {
        let fragmento = &html[pos+2..pos.min(html.len()).saturating_add(50).min(html.len())];
        let partes: Vec<&str> = fragmento.split(',').collect();
        if partes.len() >= 2 {
            if let (Ok(lat), Ok(lon)) = (partes[0].parse::<f64>(), partes[1].parse::<f64>()) {
                if lat.abs() <= 90.0 && lon.abs() <= 180.0 {
                    return Some((lat, lon));
                }
            }
        }
    }
    
    // Patrón 2: Buscar en atributos data-lat y data-lng
    let lat = extraer_numero_atributo(html, "data-lat")?;
    let lon = extraer_numero_atributo(html, "data-lng")
        .or_else(|| extraer_numero_atributo(html, "data-lon"))?;
    
    Some((lat, lon))
}

fn extraer_numero_atributo(html: &str, atributo: &str) -> Option<f64> {
    let patron = format!("{}=\"", atributo);
    if let Some(inicio) = html.find(&patron) {
        let inicio_valor = inicio + patron.len();
        if let Some(fin) = html[inicio_valor..inicio_valor + 20].find('"') {
            if let Ok(valor) = html[inicio_valor..inicio_valor + fin].parse::<f64>() {
                return Some(valor);
            }
        }
    }
    None
}

fn extraer_id_observacion(html: &str) -> Option<String> {
    // Buscar INITIAL_OBSERVATION_ID en el JavaScript
    if let Some(inicio) = html.find("INITIAL_OBSERVATION_ID = \"") {
        let inicio_id = inicio + "INITIAL_OBSERVATION_ID = \"".len();
        if let Some(fin) = html[inicio_id..inicio_id + 50].find('"') {
            return Some(html[inicio_id..inicio_id + fin].to_string());
        }
    }
    None
}

fn extraer_nombre_cientifico(html: &str) -> Option<String> {
    // Buscar en el title: "American Mink (Neogale vison)"
    if let Some(inicio) = html.find("<title") {
        if let Some(cierre) = html[inicio..].find('>') {
            let inicio_titulo = inicio + cierre + 1;
            if let Some(fin) = html[inicio_titulo..].find("</title>") {
                let titulo = &html[inicio_titulo..inicio_titulo + fin];
                // Buscar el patrón (Nombre cientifico)
                if let Some(inicio_paren) = titulo.find('(') {
                    if let Some(fin_paren) = titulo[inicio_paren..].find(')') {
                        let nombre = &titulo[inicio_paren + 1..inicio_paren + fin_paren];
                        return Some(nombre.trim().to_string());
                    }
                }
            }
        }
    }
    None
}

fn extraer_fecha(html: &str) -> Option<String> {
    // Buscar en meta DCTERMS.issued
    if let Some(inicio) = html.find(r#"<meta content=""#) {
        let resto = &html[inicio..];
        if let Some(pos_issued) = resto.find(r#"name="DCTERMS.issued""#) {
            // Buscar hacia atrás el content más cercano
            let fragmento = &resto[..pos_issued];
            if let Some(ultimo_content) = fragmento.rfind(r#"<meta content=""#) {
                let inicio_fecha = ultimo_content + r#"<meta content=""#.len();
                if let Some(fin) = fragmento[inicio_fecha..].find('"') {
                    return Some(fragmento[inicio_fecha..inicio_fecha + fin].to_string());
                }
            }
        }
    }
    None
}



fn extraer_autor(html: &str) -> Option<String> {
    // Buscar en meta DC.creator
    if let Some(inicio) = html.find(r#"<meta content=""#) {
        let resto = &html[inicio..];
        if let Some(pos_creator) = resto.find(r#"name="DC.creator""#) {
            let fragmento = &resto[..pos_creator];
            if let Some(ultimo_content) = fragmento.rfind(r#"<meta content=""#) {
                let inicio_autor = ultimo_content + r#"<meta content=""#.len();
                if let Some(fin) = fragmento[inicio_autor..].find('"') {
                    return Some(fragmento[inicio_autor..inicio_autor + fin].to_string());
                }
            }
        }
    }
    None
}

fn extraer_urls_fotos(html: &str) -> Vec<String> {
    let mut urls = Vec::new();
    
    // Buscar en meta og:image
    if let Some(inicio) = html.find(r#"property="og:image""#) {
        // Buscar hacia atrás el content
        if let Some(content_pos) = html[..inicio].rfind(r#"content=""#) {
            let inicio_url = content_pos + r#"content=""#.len();
            if let Some(fin) = html[inicio_url..inicio + 50].find('"') {
                let url = html[inicio_url..inicio_url + fin].to_string();
                if !url.is_empty() {
                    urls.push(url);
                }
            }
        }
    }
    
    urls
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inaturalist_extrae_datos_vison() {
        let html = include_str!("../fixtures/inaturalist/vison.html");
        
        let datos = procesar_inaturalist(html).expect("Debe extraer datos del visón");
        
        assert_eq!(datos.0, "395192703"); // id_observacion
        assert_eq!(datos.1, "Neogale vison"); // nombre_cientifico
        assert_eq!(datos.2, "2026-08-28"); // fecha
        // Las coordenadas deben estar en el rango de Toledo, España
        assert!(datos.3 > 39.0 && datos.3 < 41.0); // latitud
        assert!(datos.4 > -5.0 && datos.4 < -3.0); // longitud
        assert_eq!(datos.5, "Research Grade"); // calidad
        assert_eq!(datos.6, "Marco C"); // autor
        assert!(datos.7.len() > 0); // urls_fotos
        assert!(datos.7[0].contains("inaturalist-open-data.s3.amazonaws.com"));
    }

    #[test]
    fn inaturalist_extrae_datos_cangrejo() {
        let html = include_str!("../fixtures/inaturalist/cangrejo.html");
        
        let datos = procesar_inaturalist(html).expect("Debe extraer datos del cangrejo");
        
        assert_eq!(datos.0, "394426183"); // id_observacion
        assert_eq!(datos.1, "Procambarus clarkii"); // nombre_cientifico
        assert_eq!(datos.2, "2026-08-25"); // fecha
        // Las coordenadas deben estar en el rango de Toledo, España
        assert!(datos.3 > 39.0 && datos.3 < 41.0);
        assert!(datos.4 > -5.0 && datos.4 < -3.0);
        assert!(!datos.5.is_empty()); // calidad (cualquier valor)
        assert_eq!(datos.6, "Marco C");
        assert!(datos.7.len() > 0);
    }

    #[test]
    fn inaturalist_html_vacio() {
        assert!(procesar_inaturalist("").is_none());
    }
}
