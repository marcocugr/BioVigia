use super::ErrorFuente;
use super::html::ventana;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FichaGbif {
    pub nombres_anteriores: Vec<String>,
    pub nombres_comunes_espanol: Vec<String>,
}

/// Extrae de una ficha de GBIF los nombres científicos anteriores y los nombres comunes en español.
pub fn procesar_gbif(html: &str, nombre_actual: &str) -> Result<FichaGbif, ErrorFuente> {
    let nombres_anteriores = extraer_sinonimos(html, nombre_actual);
    let nombres_comunes_espanol = extraer_nombres_comunes_espanol(html);
    if nombres_anteriores.is_empty() && nombres_comunes_espanol.is_empty() {
        return Err(ErrorFuente::CampoNoEncontrado("los nombres de la especie"));
    }
    Ok(FichaGbif {
        nombres_anteriores,
        nombres_comunes_espanol,
    })
}

/// Recoge los nombres científicos en cursiva de la página, sin repetir y sin el nombre actual.
fn extraer_sinonimos(html: &str, nombre_actual: &str) -> Vec<String> {
    let mut sinonimos: Vec<String> = Vec::new();
    for nombre in nombres_en_cursiva(html) {
        if nombre != nombre_actual && !sinonimos.contains(&nombre) {
            sinonimos.push(nombre);
        }
    }
    sinonimos
}

/// Devuelve los nombres en dos palabras que aparecen entre <i> y </i>.
fn nombres_en_cursiva(texto: &str) -> Vec<String> {
    let mut nombres = Vec::new();
    let mut desde = 0;
    while let Some(relativa) = texto[desde..].find("<i>") {
        let inicio = desde + relativa + 3;
        let Some(largo) = texto[inicio..].find("</i>") else {
            break;
        };
        if let Some(nombre) = binomio(&texto[inicio..inicio + largo]) {
            nombres.push(nombre);
        }
        desde = inicio + largo + 4;
    }
    nombres
}

/// Devuelve las dos primeras palabras si parecen género y especie.
fn binomio(candidato: &str) -> Option<String> {
    let mut palabras = candidato.split_whitespace();
    let genero = palabras.next()?;
    let especie = palabras.next()?;
    let nombre = format!("{genero} {especie}");
    let empieza_en_mayuscula = genero.chars().next()?.is_uppercase();
    let solo_letras = nombre.chars().all(|c| c.is_alphabetic() || c.is_whitespace());
    (empieza_en_mayuscula && solo_letras).then_some(nombre)
}

/// Recoge los nombres comunes marcados con el idioma "spa" en los datos de la página.
fn extraer_nombres_comunes_espanol(html: &str) -> Vec<String> {
    const IDIOMA: &str = r#""@language": "spa""#;
    const VALOR: &str = r#""@value": ""#;
    let mut nombres: Vec<String> = Vec::new();
    let mut desde = 0;
    while let Some(relativa) = html[desde..].find(IDIOMA) {
        let inicio = desde + relativa + IDIOMA.len();
        desde = inicio;
        let Some(posicion_valor) = ventana(html, inicio, 200).find(VALOR) else {
            continue;
        };
        let texto = ventana(html, inicio + posicion_valor + VALOR.len(), 100);
        let Some(fin) = texto.find('"') else {
            continue;
        };
        let nombre = texto[..fin].trim();
        if !nombre.is_empty() && !nombres.iter().any(|n| n == nombre) {
            nombres.push(nombre.to_owned());
        }
    }
    nombres
}

#[cfg(test)]
mod tests {
    use super::*;

    fn html_del_vison() -> String {
        concat!(
            "<nav>Clasificación Synonyms Nombres comunes</nav>",
            "<h2>Sinónimos y combinaciones</h2><p>3 resultados</p><ul>",
            "<li><i>Mustela vison</i> Schreber, 1777</li>",
            "<li><i>Neovison vison</i> (Schreber, 1777)</li>",
            "<li><i>Putorius vison</i> Schreber, 1777</li></ul>",
            "<h2>Nombres comunes</h2>",
            r#"<script>{"v":[{"@language": "eng", "@value": "American Mink"},{"@language": "spa", "@value": "Visón Americano"}]}</script>"#,
            "<footer>fin de la ficha</footer>"
        )
        .to_string()
    }

    #[test]
    fn extrae_los_nombres_comunes_en_espanol_y_no_los_de_otros_idiomas() {
        let ficha = procesar_gbif(&html_del_vison(), "Neogale vison").unwrap();
        assert_eq!(ficha.nombres_comunes_espanol, vec!["Visón Americano"]);
    }

    #[test]
    fn extrae_varios_nombres_comunes_sin_repetir() {
        let html = r#"{"@language": "spa", "@value": "Acocil rojo"},{"@language": "spa", "@value": "Cangrejo americano"},{"@language": "spa", "@value": "Acocil rojo"}"#;
        let ficha = procesar_gbif(html, "Procambarus clarkii").unwrap();
        assert_eq!(ficha.nombres_comunes_espanol, vec!["Acocil rojo", "Cangrejo americano"]);
    }

    #[test]
    fn los_sinonimos_nunca_incluyen_el_nombre_actual() {
        let html = format!("{}<i>Neogale vison</i>", html_del_vison());
        let ficha = procesar_gbif(&html, "Neogale vison").unwrap();
        assert!(!ficha.nombres_anteriores.contains(&"Neogale vison".to_string()));
    }

    #[test]
    fn un_html_vacio_o_sin_nombres_da_error() {
        assert_eq!(procesar_gbif("", "X y"), Err(ErrorFuente::CampoNoEncontrado("los nombres de la especie")));
        assert!(procesar_gbif("<html>nada</html>", "X y").is_err());
    }

    #[test]
    fn un_html_corto_o_con_caracteres_raros_nunca_hace_panic() {
        for html in [
            r#""@language": "spa""#,
            r#""@language": "spa", "@value": "#,
            r#""@language": "spa", "@value": "ñ"#,
            "<i>",
            "<i>Mustela",
            "<i>ñ ñ</i>",
            &format!(r#""@language": "spa" {}ñ "@value": "x"#, "a".repeat(190)),
        ] {
            let _ = procesar_gbif(html, "X y");
        }
    }

    #[test]
    fn el_binomio_exige_genero_en_mayuscula_y_solo_letras() {
        assert_eq!(binomio("Neovison vison"), Some("Neovison vison".to_string()));
        assert_eq!(binomio("Neovison vison (Schreber, 1777)"), Some("Neovison vison".to_string()));
        assert_eq!(binomio("neovison vison"), None);
        assert_eq!(binomio("Neovison"), None);
        assert_eq!(binomio("Neovison v1son"), None);
        assert_eq!(binomio(""), None);
    }
}
