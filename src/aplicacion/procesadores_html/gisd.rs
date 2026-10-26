use super::html::{texto_sin_etiquetas, ventana};

const RECUENTO: &str = "The query resulted in";

/// Dice si GISD devuelve la especie con alguno de sus nombres, lo que la marca como invasora.
pub fn procesar_gisd(html: &str, nombres_cientificos: &[&str]) -> bool {
    let Some(inicio) = html.find(RECUENTO) else {
        return false;
    };
    let resultados = &html[inicio + RECUENTO.len()..];
    match numero_de_especies(resultados) {
        Some(especies) if especies > 0 => nombres_cientificos
            .iter()
            .map(|nombre| nombre.trim())
            .any(|nombre| !nombre.is_empty() && resultados.contains(nombre)),
        _ => false,
    }
}

/// Lee el número de especies que anuncia el recuento, con o sin etiquetas alrededor.
fn numero_de_especies(resultados: &str) -> Option<usize> {
    let texto = texto_sin_etiquetas(ventana(resultados, 0, 100));
    let mut palabras = texto.split_whitespace();
    let numero = palabras.next()?.parse().ok()?;
    palabras.next()?.starts_with("species").then_some(numero)
}

#[cfg(test)]
mod tests {
    use super::*;

    const HTML_CON_UN_RESULTADO: &str = concat!(
        r#"<form><input type="text" name="sn" value=""></form>"#,
        r#"<div id="info-count">The query resulted in <strong>1</strong> species in <strong>1</strong> page</div>"#,
        r#"<a href="species.php?sc=1"><i>Neovison vison</i></a> (American mink, mink)"#
    );

    #[test]
    fn encuentra_la_especie_con_un_nombre_anterior() {
        let nombres = ["Neogale vison", "Mustela vison", "Neovison vison"];
        assert!(procesar_gisd(HTML_CON_UN_RESULTADO, &nombres));
    }

    #[test]
    fn no_encuentra_una_especie_que_no_esta_en_los_resultados() {
        assert!(!procesar_gisd(HTML_CON_UN_RESULTADO, &["Procambarus clarkii"]));
    }

    #[test]
    fn una_busqueda_con_cero_resultados_no_encuentra_nada() {
        let html = r#"<div id="info-count">The query resulted in <strong>0</strong> species"#;
        assert!(!procesar_gisd(html, &["Especie Falsa"]));
    }

    #[test]
    fn cero_resultados_no_cuenta_aunque_la_pagina_repita_el_nombre_buscado() {
        let html = concat!(
            r#"<input name="sn" value="Mustela vison">"#,
            "<p>The query resulted in 0 species</p>",
            "<p>Sin resultados para Mustela vison</p>"
        );
        assert!(!procesar_gisd(html, &["Mustela vison"]));
    }

    #[test]
    fn el_nombre_del_formulario_anterior_al_recuento_no_cuenta() {
        let html = concat!(
            r#"<input name="sn" value="Mustela vison">"#,
            "<p>The query resulted in 1 species in 1 page</p><i>Neovison vison</i>"
        );
        assert!(!procesar_gisd(html, &["Mustela vison"]));
        assert!(procesar_gisd(html, &["Mustela vison", "Neovison vison"]));
    }

    #[test]
    fn lee_el_recuento_con_o_sin_etiquetas() {
        assert!(procesar_gisd("The query resulted in 2 species <i>Neovison vison</i>", &["Neovison vison"]));
        assert!(procesar_gisd("The query resulted in <b>2</b> species <i>Neovison vison</i>", &["Neovison vison"]));
    }

    #[test]
    fn sin_recuento_o_con_un_recuento_ilegible_no_encuentra_nada() {
        assert!(!procesar_gisd("<i>Neovison vison</i>", &["Neovison vison"]));
        assert!(!procesar_gisd("The query resulted in muchas species <i>Neovison vison</i>", &["Neovison vison"]));
        assert!(!procesar_gisd("The query resulted in 3 cosas <i>Neovison vison</i>", &["Neovison vison"]));
    }

    #[test]
    fn un_html_vacio_o_cortado_no_encuentra_nada_ni_hace_panic() {
        assert!(!procesar_gisd("", &["Cualquier Nombre"]));
        assert!(!procesar_gisd("The query resulted in", &["X y"]));
        assert!(!procesar_gisd("The query resulted in ñññññññññññññññññññññññññññññññññññññññññññññññññññ", &["X y"]));
    }

    #[test]
    fn una_lista_vacia_o_con_nombres_en_blanco_no_encuentra_nada() {
        assert!(!procesar_gisd(HTML_CON_UN_RESULTADO, &[]));
        assert!(!procesar_gisd(HTML_CON_UN_RESULTADO, &["", "  "]));
    }
}
