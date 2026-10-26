/// Dice si GISD encuentra la especie con alguno de sus nombres, lo que la marca como invasora.
pub fn procesar_gisd(html: &str, nombres_cientificos: &[&str]) -> bool {
    if html.contains("<strong>0</strong> species") {
        return false;
    }
    html.contains("species") && nombres_cientificos.iter().any(|nombre| html.contains(nombre))
}

#[cfg(test)]
mod tests {
    use super::*;

    const HTML_CON_UN_RESULTADO: &str = concat!(
        r#"<div id="info-count">The query resulted in <strong>1</strong> species in <strong>1</strong> page</div>"#,
        r#"<a href="species.php?sc=1"><i>Neovison vison</i></a> (American mink, mink)"#
    );

    #[test]
    fn encuentra_la_especie_con_un_nombre_anterior() {
        let nombres = ["Neogale vison", "Mustela vison", "Neovison vison"];
        assert!(procesar_gisd(HTML_CON_UN_RESULTADO, &nombres));
    }

    #[test]
    fn no_encuentra_una_especie_que_no_esta_en_la_pagina() {
        assert!(!procesar_gisd(HTML_CON_UN_RESULTADO, &["Procambarus clarkii"]));
    }

    #[test]
    fn una_busqueda_con_cero_resultados_no_encuentra_nada() {
        let html = r#"<div id="info-count">The query resulted in <strong>0</strong> species"#;
        assert!(!procesar_gisd(html, &["Especie Falsa"]));
    }

    #[test]
    fn un_html_vacio_o_una_lista_vacia_de_nombres_no_encuentra_nada() {
        assert!(!procesar_gisd("", &["Cualquier Nombre"]));
        assert!(!procesar_gisd(HTML_CON_UN_RESULTADO, &[]));
    }
}
