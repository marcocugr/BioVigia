use super::ErrorFuente;
use super::html::{texto_sin_etiquetas, ventana};

const TITULO_SINONIMOS: &str = "Sinónimos y combinaciones";
const SECCIONES_SIGUIENTES: [&str; 5] = [
    "Nombres comunes",
    "Tratamientos",
    "Bibliografía",
    "Identificadores",
    "Imágenes de ocurrencias",
];
const MAXIMO_SIN_CONTADOR: usize = 30;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FichaGbif {
    pub nombres_anteriores: Vec<String>,
    pub nombres_comunes_espanol: Vec<String>,
}

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

fn extraer_sinonimos(html: &str, nombre_actual: &str) -> Vec<String> {
    html.match_indices(TITULO_SINONIMOS)
        .map(|(posicion, _)| {
            sinonimos_de_seccion(&html[posicion + TITULO_SINONIMOS.len()..], nombre_actual)
        })
        .find(|sinonimos| !sinonimos.is_empty())
        .unwrap_or_default()
}

fn sinonimos_de_seccion(texto: &str, nombre_actual: &str) -> Vec<String> {
    let fin = SECCIONES_SIGUIENTES
        .iter()
        .filter_map(|titulo| texto.find(titulo))
        .min()
        .unwrap_or(texto.len());
    let seccion = &texto[..fin];
    let limite = numero_de_resultados(seccion).unwrap_or(MAXIMO_SIN_CONTADOR);
    let mut sinonimos: Vec<String> = Vec::new();
    for nombre in nombres_en_cursiva(seccion).into_iter().take(limite) {
        if nombre != nombre_actual && !sinonimos.contains(&nombre) {
            sinonimos.push(nombre);
        }
    }
    sinonimos
}

fn numero_de_resultados(seccion: &str) -> Option<usize> {
    let texto = texto_sin_etiquetas(ventana(seccion, 0, 400));
    let antes = &texto[..texto.find("resultado")?];
    antes.split_whitespace().next_back()?.parse().ok()
}

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

fn binomio(candidato: &str) -> Option<String> {
    let mut palabras = candidato.split_whitespace();
    let genero = palabras.next()?;
    let especie = palabras.next()?;
    let nombre = format!("{genero} {especie}");
    let genero_en_mayuscula = genero.chars().next()?.is_uppercase();
    let especie_en_minuscula = especie.chars().next()?.is_lowercase();
    let solo_letras = nombre.chars().all(|c| c.is_alphabetic() || c.is_whitespace());
    (genero_en_mayuscula && especie_en_minuscula && solo_letras).then_some(nombre)
}

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
        assert_eq!(binomio("Neovison Vison"), None);
        assert_eq!(binomio("Neovison v1son"), None);
        assert_eq!(binomio(""), None);
    }

    fn con_decoys() -> String {
        concat!(
            "<section><h2>Clasificación</h2><p><i>Mustela erminea</i></p></section>",
            "<script>{\"titulo\": \"Sinónimos y combinaciones\"}</script>",
            "<section><h2>Sinónimos y combinaciones</h2><p><span>3</span> resultados</p><ul>",
            "<li><i>Mustela vison</i> Schreber, 1777</li>",
            "<li><i>Neovison vison</i> (Schreber, 1777)</li>",
            "<li><i>Putorius vison</i> Schreber, 1777</li></ul></section>",
            "<section><h2>Nombres comunes</h2><p><i>Felis catus</i></p></section>",
            "<section><h2>Bibliografía</h2><p><i>Canis lupus</i> Linnaeus, 1758</p></section>"
        )
        .to_string()
    }

    #[test]
    fn los_sinonimos_son_solo_los_de_su_seccion() {
        let ficha = procesar_gbif(&con_decoys(), "Neogale vison").unwrap();
        assert_eq!(ficha.nombres_anteriores, vec!["Mustela vison", "Neovison vison", "Putorius vison"]);
    }

    #[test]
    fn el_numero_de_resultados_pone_el_limite() {
        let html = con_decoys().replace("<span>3</span> resultados", "2 resultados");
        let ficha = procesar_gbif(&html, "Neogale vison").unwrap();
        assert_eq!(ficha.nombres_anteriores, vec!["Mustela vison", "Neovison vison"]);
    }

    #[test]
    fn sin_contador_la_lectura_se_para_en_la_siguiente_seccion() {
        let html = con_decoys().replace("<p><span>3</span> resultados</p>", "");
        let ficha = procesar_gbif(&html, "Neogale vison").unwrap();
        assert_eq!(ficha.nombres_anteriores, vec!["Mustela vison", "Neovison vison", "Putorius vison"]);
    }

    #[test]
    fn un_titulo_repetido_sin_nombres_no_tapa_a_la_seccion_buena() {
        let html = con_decoys();
        assert_eq!(html.matches("Sinónimos y combinaciones").count(), 2);
        assert_eq!(procesar_gbif(&html, "Neogale vison").unwrap().nombres_anteriores.len(), 3);
    }

    #[test]
    fn sin_la_seccion_de_sinonimos_no_hay_nombres_anteriores_aunque_haya_cursivas() {
        let html = r#"<i>Mustela vison</i> {"@language": "spa", "@value": "Visón"}"#;
        let ficha = procesar_gbif(html, "Neogale vison").unwrap();
        assert!(ficha.nombres_anteriores.is_empty());
        assert_eq!(ficha.nombres_comunes_espanol, vec!["Visón"]);
    }

    #[test]
    fn una_seccion_sin_resultados_da_cero_sinonimos() {
        let html = r#"<h2>Sinónimos y combinaciones</h2><p>0 resultados</p><h2>Nombres comunes</h2>{"@language": "spa", "@value": "Acocil rojo"}"#;
        let ficha = procesar_gbif(html, "Procambarus clarkii").unwrap();
        assert!(ficha.nombres_anteriores.is_empty());
    }

    #[test]
    fn el_numero_de_resultados_se_lee_con_o_sin_etiquetas() {
        assert_eq!(numero_de_resultados("<p>3 resultados</p>"), Some(3));
        assert_eq!(numero_de_resultados("<b>12</b> resultados"), Some(12));
        assert_eq!(numero_de_resultados("sin contador"), None);
        assert_eq!(numero_de_resultados("muchos resultados"), None);
    }
}
