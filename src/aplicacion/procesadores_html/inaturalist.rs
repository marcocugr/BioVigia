use std::time::{Duration, SystemTime};

use crate::dominio::avistamiento::{Avistamiento, EvidenciaFotografica, Fuente, GradoCalidad};
use crate::dominio::coordenadas::{Coordenadas, Latitud, Longitud};
use crate::dominio::especie::CatalogoEspecies;

use super::ErrorFuente;
use super::html::{contenidos_de_etiquetas, ventana};

#[derive(Debug, Clone, PartialEq)]
pub struct ObservacionINaturalist {
    pub id_observacion: String,
    pub nombre_cientifico: String,
    pub fecha: SystemTime,
    pub latitud: f64,
    pub longitud: f64,
    pub calidad: Option<GradoCalidad>,
    pub autor: Option<String>,
    pub urls_fotos: Vec<String>,
}

impl ObservacionINaturalist {
    /// Convierte la observación en un avistamiento del dominio, validado contra el catálogo.
    pub fn a_avistamiento(&self, catalogo: &CatalogoEspecies) -> Result<Avistamiento, ErrorFuente> {
        let coordenadas = Coordenadas::new(Latitud::new(self.latitud)?, Longitud::new(self.longitud)?);
        let evidencia = EvidenciaFotografica::new(self.autor.clone(), self.urls_fotos.clone())?;
        Ok(Avistamiento::new(
            catalogo,
            Fuente::INaturalist,
            self.id_observacion.clone(),
            &self.nombre_cientifico,
            self.fecha,
            coordenadas,
            self.calidad,
            evidencia,
        )?)
    }
}

/// Extrae los datos de una observación del HTML guardado de su página en iNaturalist.
pub fn procesar_inaturalist(html: &str) -> Result<ObservacionINaturalist, ErrorFuente> {
    let id_observacion = extraer_id_observacion(html)
        .ok_or(ErrorFuente::CampoNoEncontrado("el identificador de la observación"))?;
    let nombre_cientifico = extraer_nombre_cientifico(html)
        .ok_or(ErrorFuente::CampoNoEncontrado("el nombre científico"))?;
    let (latitud, longitud) =
        extraer_coordenadas(html).ok_or(ErrorFuente::CampoNoEncontrado("las coordenadas"))?;
    Ok(ObservacionINaturalist {
        id_observacion,
        nombre_cientifico,
        fecha: extraer_fecha(html)?,
        latitud,
        longitud,
        calidad: extraer_calidad(html),
        autor: extraer_autor(html),
        urls_fotos: extraer_urls_fotos(html),
    })
}

/// Lee el identificador numérico que la página guarda en INITIAL_OBSERVATION_ID.
fn extraer_id_observacion(html: &str) -> Option<String> {
    const MARCA: &str = "INITIAL_OBSERVATION_ID = \"";
    let inicio = html.find(MARCA)? + MARCA.len();
    let resto = ventana(html, inicio, 25);
    let id = &resto[..resto.find('"')?];
    (!id.is_empty() && id.chars().all(|c| c.is_ascii_digit())).then(|| id.to_owned())
}

/// Lee el nombre científico, que va entre paréntesis en el título de la página.
fn extraer_nombre_cientifico(html: &str) -> Option<String> {
    let inicio_etiqueta = html.find("<title")?;
    let inicio_titulo = inicio_etiqueta + html[inicio_etiqueta..].find('>')? + 1;
    let fin_titulo = inicio_titulo + html[inicio_titulo..].find("</title>")?;
    let titulo = &html[inicio_titulo..fin_titulo];
    let abre = titulo.find('(')?;
    let cierra = abre + titulo[abre..].find(')')?;
    let nombre = titulo[abre + 1..cierra].trim();
    (!nombre.is_empty()).then(|| nombre.to_owned())
}

/// Lee la fecha de DCTERMS.issued y la pasa a las 00:00 UTC de ese día.
fn extraer_fecha(html: &str) -> Result<SystemTime, ErrorFuente> {
    let fecha = contenidos_de_etiquetas(html, "name", "DCTERMS.issued")
        .into_iter()
        .next()
        .ok_or(ErrorFuente::CampoNoEncontrado("la fecha"))?;
    parsear_fecha(&fecha).ok_or(ErrorFuente::ValorInvalido("la fecha"))
}

/// Convierte un texto que empieza por AAAA-MM-DD en las 00:00 UTC de ese día.
fn parsear_fecha(texto: &str) -> Option<SystemTime> {
    let mut partes = texto.trim().get(..10)?.split('-');
    let anio: i64 = partes.next()?.parse().ok()?;
    let mes: i64 = partes.next()?.parse().ok()?;
    let dia: i64 = partes.next()?.parse().ok()?;
    if partes.next().is_some() || !(1..=12).contains(&mes) || dia < 1 || dia > dias_del_mes(anio, mes) {
        return None;
    }
    let dias = u64::try_from(dias_desde_1970(anio, mes, dia)).ok()?;
    Some(SystemTime::UNIX_EPOCH + Duration::from_secs(dias * 86_400))
}

/// Dice cuántos días tiene el mes, contando los años bisiestos.
fn dias_del_mes(anio: i64, mes: i64) -> i64 {
    match mes {
        2 if anio % 4 == 0 && (anio % 100 != 0 || anio % 400 == 0) => 29,
        2 => 28,
        4 | 6 | 9 | 11 => 30,
        _ => 31,
    }
}

/// Cuenta los días desde el 1 de enero de 1970 hasta la fecha, con el algoritmo de días civiles.
fn dias_desde_1970(anio: i64, mes: i64, dia: i64) -> i64 {
    let anio = if mes <= 2 { anio - 1 } else { anio };
    let era = anio.div_euclid(400);
    let anio_de_era = anio - era * 400;
    let dia_del_anio = (153 * ((mes + 9) % 12) + 2) / 5 + dia - 1;
    let dia_de_era = anio_de_era * 365 + anio_de_era / 4 - anio_de_era / 100 + dia_del_anio;
    era * 146_097 + dia_de_era - 719_468
}

/// Busca las coordenadas en un enlace de mapa y, si no, en los atributos data-lat y data-lng.
fn extraer_coordenadas(html: &str) -> Option<(f64, f64)> {
    coordenadas_de_enlace_de_mapa(html).or_else(|| coordenadas_de_atributos(html))
}

/// Lee latitud y longitud del primer enlace con la forma /@latitud,longitud que las tenga.
fn coordenadas_de_enlace_de_mapa(html: &str) -> Option<(f64, f64)> {
    html.match_indices("/@")
        .find_map(|(posicion, _)| par_de_coordenadas(ventana(html, posicion + 2, 48)))
}

/// Lee dos números separados por coma si son una latitud y una longitud posibles.
fn par_de_coordenadas(texto: &str) -> Option<(f64, f64)> {
    let mut partes = texto.split(',');
    let latitud = numero_al_principio(partes.next()?)?;
    let longitud = numero_al_principio(partes.next()?)?;
    (latitud.abs() <= 90.0 && longitud.abs() <= 180.0).then_some((latitud, longitud))
}

/// Lee el número decimal con el que empieza el texto.
fn numero_al_principio(texto: &str) -> Option<f64> {
    let fin = texto
        .find(|c: char| !(c.is_ascii_digit() || matches!(c, '.' | '-' | '+')))
        .unwrap_or(texto.len());
    texto[..fin].parse().ok()
}

/// Lee latitud y longitud de los atributos data-lat y data-lng (o data-lon).
fn coordenadas_de_atributos(html: &str) -> Option<(f64, f64)> {
    let latitud = numero_de_atributo(html, "data-lat")?;
    let longitud = numero_de_atributo(html, "data-lng").or_else(|| numero_de_atributo(html, "data-lon"))?;
    Some((latitud, longitud))
}

/// Lee el número que un atributo guarda entre comillas.
fn numero_de_atributo(html: &str, atributo: &str) -> Option<f64> {
    let marca = format!("{atributo}=\"");
    let inicio = html.find(&marca)? + marca.len();
    let valor = ventana(html, inicio, 24);
    valor[..valor.find('"')?].trim().parse().ok()
}

/// Lee la calidad del texto que sigue a "quality_grade", sin contar los enlaces de filtro.
fn extraer_calidad(html: &str) -> Option<GradoCalidad> {
    const MARCA: &str = "quality_grade";
    html.match_indices(MARCA)
        .filter(|(posicion, _)| {
            !matches!(html[..*posicion].chars().next_back(), Some('?' | '&' | '/'))
        })
        .find_map(|(posicion, _)| grado_en(ventana(html, posicion + MARCA.len(), 24)))
}

/// Reconoce el grado de calidad que aparece primero en el texto.
fn grado_en(texto: &str) -> Option<GradoCalidad> {
    let texto = texto.to_ascii_lowercase();
    [
        ("research", GradoCalidad::Investigacion),
        ("needs_id", GradoCalidad::NecesitaIdentificacion),
        ("needs id", GradoCalidad::NecesitaIdentificacion),
        ("needs-id", GradoCalidad::NecesitaIdentificacion),
        ("casual", GradoCalidad::Casual),
    ]
    .into_iter()
    .filter_map(|(palabra, grado)| texto.find(palabra).map(|posicion| (posicion, grado)))
    .min_by_key(|(posicion, _)| *posicion)
    .map(|(_, grado)| grado)
}

/// Lee el nombre del observador de la etiqueta DC.creator.
fn extraer_autor(html: &str) -> Option<String> {
    contenidos_de_etiquetas(html, "name", "DC.creator")
        .into_iter()
        .map(|autor| autor.trim().to_owned())
        .find(|autor| !autor.is_empty())
}

/// Recoge las URL de todas las etiquetas og:image, sin repetir y solo las http o https.
fn extraer_urls_fotos(html: &str) -> Vec<String> {
    let mut urls: Vec<String> = Vec::new();
    for url in contenidos_de_etiquetas(html, "property", "og:image") {
        let url = url.trim().to_owned();
        let es_http = url.starts_with("https://") || url.starts_with("http://");
        if es_http && !urls.contains(&url) {
            urls.push(url);
        }
    }
    urls
}

#[cfg(test)]
mod tests {
    use super::*;

    const MEDIANOCHE_28_AGO_2026: u64 = 1_787_875_200;

    fn html_del_vison() -> String {
        concat!(
            "<html><head>",
            r#"<meta content="2026-08-28" name="DCTERMS.issued">"#,
            r#"<meta content="Marco C" name="DC.creator">"#,
            r#"<meta content="https://inaturalist-open-data.s3.amazonaws.com/photos/111/large.jpg" property="og:image">"#,
            "<title>Visón americano (Neogale vison), observado por Marco C | iNaturalist</title>",
            "</head><body>",
            r#"<script>window.INITIAL_OBSERVATION_ID = "395192703";</script>"#,
            r#"<a href="https://www.google.com/maps/@39.85,-4.55,15z/data=!3m1">mapa</a>"#,
            r#"<span class="quality_grade research">Research Grade</span>"#,
            "<section><span>Casual</span><span>Needs ID</span><span>Research Grade</span></section>",
            "</body></html>"
        )
        .to_string()
    }

    fn catalogo() -> CatalogoEspecies {
        let mut catalogo = CatalogoEspecies::new();
        catalogo
            .registrar("Neogale vison".into(), vec!["Visón americano".into()], true, vec![])
            .unwrap();
        catalogo
    }

    #[test]
    fn extrae_los_datos_basicos_de_una_observacion() {
        let observacion = procesar_inaturalist(&html_del_vison()).unwrap();
        assert_eq!(observacion.id_observacion, "395192703");
        assert_eq!(observacion.nombre_cientifico, "Neogale vison");
        assert_eq!(observacion.fecha, SystemTime::UNIX_EPOCH + Duration::from_secs(MEDIANOCHE_28_AGO_2026));
        assert_eq!((observacion.latitud, observacion.longitud), (39.85, -4.55));
        assert_eq!(observacion.calidad, Some(GradoCalidad::Investigacion));
        assert_eq!(observacion.autor.as_deref(), Some("Marco C"));
        assert_eq!(
            observacion.urls_fotos,
            vec!["https://inaturalist-open-data.s3.amazonaws.com/photos/111/large.jpg"]
        );
    }

    #[test]
    fn un_html_vacio_da_error() {
        assert!(procesar_inaturalist("").is_err());
    }

    #[test]
    fn sin_identificador_da_campo_no_encontrado() {
        let html = html_del_vison().replace("INITIAL_OBSERVATION_ID", "OTRA_COSA");
        assert_eq!(
            procesar_inaturalist(&html),
            Err(ErrorFuente::CampoNoEncontrado("el identificador de la observación"))
        );
    }

    #[test]
    fn un_identificador_que_no_es_numerico_da_error() {
        let html = html_del_vison().replace("395192703", "abc");
        assert!(procesar_inaturalist(&html).is_err());
    }

    #[test]
    fn sin_coordenadas_da_campo_no_encontrado() {
        let html = html_del_vison().replace("/@39.85,-4.55,15z", "/sin-mapa");
        assert_eq!(
            procesar_inaturalist(&html),
            Err(ErrorFuente::CampoNoEncontrado("las coordenadas"))
        );
    }

    #[test]
    fn sin_fecha_da_campo_no_encontrado() {
        let html = html_del_vison().replace("DCTERMS.issued", "OTRA");
        assert_eq!(procesar_inaturalist(&html), Err(ErrorFuente::CampoNoEncontrado("la fecha")));
    }

    #[test]
    fn una_fecha_imposible_da_valor_invalido() {
        let html = html_del_vison().replace("2026-08-28", "2026-02-30");
        assert_eq!(procesar_inaturalist(&html), Err(ErrorFuente::ValorInvalido("la fecha")));
    }

    #[test]
    fn sin_autor_ni_foto_la_observacion_sigue_siendo_valida() {
        let html = html_del_vison()
            .replace("DC.creator", "otra")
            .replace("og:image", "otra");
        let observacion = procesar_inaturalist(&html).unwrap();
        assert_eq!(observacion.autor, None);
        assert!(observacion.urls_fotos.is_empty());
    }

    #[test]
    fn las_coordenadas_se_leen_tambien_de_los_atributos_data() {
        let html = html_del_vison().replace("/@39.85,-4.55,15z", "/sin-mapa")
            + r#"<div data-lat="39.7" data-lng="-4.1"></div>"#;
        let observacion = procesar_inaturalist(&html).unwrap();
        assert_eq!((observacion.latitud, observacion.longitud), (39.7, -4.1));
    }

    #[test]
    fn unas_coordenadas_imposibles_no_se_aceptan() {
        let html = html_del_vison().replace("/@39.85,-4.55,15z", "/@95.0,-4.55,15z");
        assert!(procesar_inaturalist(&html).is_err());
    }

    #[test]
    fn un_html_corto_o_con_caracteres_raros_nunca_hace_panic() {
        for html in [
            "INITIAL_OBSERVATION_ID = \"39",
            "INITIAL_OBSERVATION_ID = \"",
            "<title",
            "<title>(",
            "/@",
            "/@ñ,ñ",
            r#"<meta content="ñ" name="DCTERMS.issued""#,
            "data-lat=\"ñññññññññññññññññññññññññ",
        ] {
            let _ = procesar_inaturalist(html);
        }
        let borde = format!("INITIAL_OBSERVATION_ID = \"395192703\"{}ñxxxx", "a".repeat(39));
        let _ = procesar_inaturalist(&borde);
    }

    #[test]
    fn parsea_fechas_con_y_sin_hora() {
        let esperado = Some(SystemTime::UNIX_EPOCH + Duration::from_secs(MEDIANOCHE_28_AGO_2026));
        assert_eq!(parsear_fecha("2026-08-28"), esperado);
        assert_eq!(parsear_fecha(" 2026-08-28T10:44:00+02:00 "), esperado);
    }

    #[test]
    fn parsea_el_primer_dia_de_1970_y_un_29_de_febrero_bisiesto() {
        assert_eq!(parsear_fecha("1970-01-01"), Some(SystemTime::UNIX_EPOCH));
        assert_eq!(
            parsear_fecha("2024-02-29"),
            Some(SystemTime::UNIX_EPOCH + Duration::from_secs(19_782 * 86_400))
        );
    }

    #[test]
    fn rechaza_fechas_mal_formadas_imposibles_o_anteriores_a_1970() {
        for fecha in ["2026-02-29", "2026-13-01", "2026-00-10", "2026-04-31", "28/08/2026", "2026-8-28", "1969-12-31", "", "hola"] {
            assert_eq!(parsear_fecha(fecha), None, "con {fecha:?}");
        }
    }

    #[test]
    fn dias_del_mes_cuenta_los_bisiestos() {
        assert_eq!(dias_del_mes(2024, 2), 29);
        assert_eq!(dias_del_mes(2026, 2), 28);
        assert_eq!(dias_del_mes(1900, 2), 28);
        assert_eq!(dias_del_mes(2000, 2), 29);
        assert_eq!(dias_del_mes(2026, 4), 30);
        assert_eq!(dias_del_mes(2026, 8), 31);
    }

    #[test]
    fn la_observacion_se_convierte_en_un_avistamiento_del_dominio() {
        let catalogo = catalogo();
        let observacion = procesar_inaturalist(&html_del_vison()).unwrap();
        let esperado = Avistamiento::new(
            &catalogo,
            Fuente::INaturalist,
            "395192703".into(),
            "Neogale vison",
            observacion.fecha,
            Coordenadas::new(Latitud::new(39.85).unwrap(), Longitud::new(-4.55).unwrap()),
            Some(GradoCalidad::Investigacion),
            EvidenciaFotografica::new(
                Some("Marco C".into()),
                vec!["https://inaturalist-open-data.s3.amazonaws.com/photos/111/large.jpg".into()],
            )
            .unwrap(),
        )
        .unwrap();
        assert_eq!(observacion.a_avistamiento(&catalogo), Ok(esperado));
    }

    #[test]
    fn la_observacion_de_una_especie_no_registrada_no_se_convierte() {
        let observacion = procesar_inaturalist(&html_del_vison()).unwrap();
        let resultado = observacion.a_avistamiento(&CatalogoEspecies::new());
        assert_eq!(
            resultado,
            Err(ErrorFuente::Dominio(crate::dominio::error::ErrorDominio::EspecieDesconocida))
        );
    }

    #[test]
    fn una_url_de_foto_mala_no_se_convierte_en_avistamiento() {
        let mut observacion = procesar_inaturalist(&html_del_vison()).unwrap();
        observacion.urls_fotos = vec!["mala".into()];
        assert_eq!(
            observacion.a_avistamiento(&catalogo()),
            Err(ErrorFuente::Dominio(crate::dominio::error::ErrorDominio::UrlFotoInvalida))
        );
    }

    #[test]
    fn las_coordenadas_se_leen_del_primer_enlace_de_mapa_que_las_tenga() {
        let html = html_del_vison().replace(
            "<body>",
            r#"<body><a href="https://www.inaturalist.org/people/@marco">perfil</a>"#,
        );
        let observacion = procesar_inaturalist(&html).unwrap();
        assert_eq!((observacion.latitud, observacion.longitud), (39.85, -4.55));
    }

    #[test]
    fn la_calidad_se_lee_de_quality_grade() {
        for (grado, esperado) in [
            ("research", GradoCalidad::Investigacion),
            ("needs_id", GradoCalidad::NecesitaIdentificacion),
            ("casual", GradoCalidad::Casual),
        ] {
            let html = html_del_vison().replace("quality_grade research", &format!("quality_grade {grado}"));
            assert_eq!(procesar_inaturalist(&html).unwrap().calidad, Some(esperado), "con {grado}");
        }
    }

    #[test]
    fn la_calidad_se_lee_tambien_de_un_json() {
        let html = html_del_vison().replace(
            r#"class="quality_grade research""#,
            r#"data-datos='{"quality_grade":"needs_id"}'"#,
        );
        assert_eq!(
            procesar_inaturalist(&html).unwrap().calidad,
            Some(GradoCalidad::NecesitaIdentificacion)
        );
    }

    #[test]
    fn un_enlace_de_filtro_con_quality_grade_no_cuenta_como_la_calidad() {
        let html = html_del_vison().replace(
            "<body>",
            r#"<body><a href="/observations?taxon_id=1&quality_grade=casual">casuales</a>"#,
        );
        assert_eq!(procesar_inaturalist(&html).unwrap().calidad, Some(GradoCalidad::Investigacion));
    }

    #[test]
    fn sin_quality_grade_la_calidad_se_queda_sin_dato_aunque_la_pagina_nombre_los_tres_grados() {
        let html = html_del_vison().replace("quality_grade", "otra_clase");
        let observacion = procesar_inaturalist(&html).unwrap();
        assert!(html.contains("Research Grade") && html.contains("Casual"));
        assert_eq!(observacion.calidad, None);
    }

    #[test]
    fn las_fotos_son_todas_las_de_og_image_en_orden_sin_repetir() {
        let html = html_del_vison().replace(
            "<title>",
            concat!(
                r#"<meta property="og:image" content="https://inaturalist-open-data.s3.amazonaws.com/photos/222/large.jpg">"#,
                r#"<meta content="https://inaturalist-open-data.s3.amazonaws.com/photos/222/large.jpg" property="og:image">"#,
                "<title>"
            ),
        );
        assert_eq!(
            procesar_inaturalist(&html).unwrap().urls_fotos,
            vec![
                "https://inaturalist-open-data.s3.amazonaws.com/photos/111/large.jpg",
                "https://inaturalist-open-data.s3.amazonaws.com/photos/222/large.jpg",
            ]
        );
    }

    #[test]
    fn las_fotos_que_no_son_http_o_https_se_descartan() {
        let html = html_del_vison().replace(
            "<title>",
            r#"<meta content="//cdn.ejemplo.org/a.jpg" property="og:image"><meta content="" property="og:image"><title>"#,
        );
        assert_eq!(procesar_inaturalist(&html).unwrap().urls_fotos.len(), 1);
    }
}
