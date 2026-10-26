/// Devuelve hasta `longitud` bytes desde `desde` sin cortar nunca un carácter por la mitad.
pub(super) fn ventana(texto: &str, desde: usize, longitud: usize) -> &str {
    let desde = desde.min(texto.len());
    let mut hasta = desde.saturating_add(longitud).min(texto.len());
    while !texto.is_char_boundary(hasta) {
        hasta -= 1;
    }
    texto.get(desde..hasta).unwrap_or("")
}

/// Devuelve el `content` de cada etiqueta que lleve `atributo="valor"`, en cualquier orden de atributos.
pub(super) fn contenidos_de_etiquetas(html: &str, atributo: &str, valor: &str) -> Vec<String> {
    let marca = format!("{atributo}=\"{valor}\"");
    let mut contenidos = Vec::new();
    let mut desde = 0;
    while let Some(relativa) = html[desde..].find(&marca) {
        let posicion = desde + relativa;
        desde = posicion + marca.len();
        let Some(inicio) = html[..posicion].rfind('<') else {
            continue;
        };
        let Some(fin) = html[posicion..].find('>') else {
            continue;
        };
        if let Some(contenido) = valor_de_atributo(&html[inicio..posicion + fin], "content") {
            contenidos.push(contenido);
        }
    }
    contenidos
}

/// Lee el valor entre comillas de un atributo dentro del texto de una etiqueta.
fn valor_de_atributo(etiqueta: &str, atributo: &str) -> Option<String> {
    let marca = format!(" {atributo}=\"");
    let inicio = etiqueta.find(&marca)? + marca.len();
    let resto = &etiqueta[inicio..];
    let fin = resto.find('"')?;
    Some(resto[..fin].to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn la_ventana_devuelve_los_bytes_pedidos() {
        assert_eq!(ventana("abcdef", 1, 3), "bcd");
    }

    #[test]
    fn la_ventana_no_se_sale_del_texto() {
        assert_eq!(ventana("abc", 1, 50), "bc");
        assert_eq!(ventana("abc", 10, 5), "");
        assert_eq!(ventana("", 0, 5), "");
    }

    #[test]
    fn la_ventana_no_corta_un_caracter_multibyte_por_la_mitad() {
        assert_eq!(ventana("añb", 0, 2), "a");
        assert_eq!(ventana("añb", 0, 3), "añ");
        assert_eq!(ventana("ñññ", 0, 3), "ñ");
    }

    #[test]
    fn la_ventana_que_empieza_a_mitad_de_un_caracter_queda_vacia() {
        assert_eq!(ventana("ñb", 1, 5), "");
    }

    #[test]
    fn lee_el_content_con_el_content_antes_del_nombre() {
        let html = r#"<meta content="2026-08-28" name="DCTERMS.issued">"#;
        assert_eq!(contenidos_de_etiquetas(html, "name", "DCTERMS.issued"), vec!["2026-08-28"]);
    }

    #[test]
    fn lee_el_content_con_el_nombre_antes_del_content() {
        let html = r#"<meta name="DC.creator" content="Marco C" />"#;
        assert_eq!(contenidos_de_etiquetas(html, "name", "DC.creator"), vec!["Marco C"]);
    }

    #[test]
    fn devuelve_todas_las_etiquetas_que_coinciden_en_orden() {
        let html = r#"<meta content="a" property="og:image"><meta content="b" property="og:image">"#;
        assert_eq!(contenidos_de_etiquetas(html, "property", "og:image"), vec!["a", "b"]);
    }

    #[test]
    fn ignora_las_etiquetas_sin_content_y_los_data_content() {
        let html = r#"<meta name="x" data-content="no"><meta name="x">"#;
        assert!(contenidos_de_etiquetas(html, "name", "x").is_empty());
    }

    #[test]
    fn no_falla_con_html_vacio_o_sin_cierre() {
        assert!(contenidos_de_etiquetas("", "name", "x").is_empty());
        assert!(contenidos_de_etiquetas(r#"<meta name="x" content="a""#, "name", "x").is_empty());
        assert!(contenidos_de_etiquetas(r#"name="x" content="a">"#, "name", "x").is_empty());
    }
}
