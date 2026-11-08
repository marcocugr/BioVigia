use std::time::{Duration, SystemTime};

use crate::dominio::avistamiento::{Avistamiento, Fuente, SEGUNDOS_POR_DIA};
use crate::dominio::coordenadas::{Coordenadas, Latitud, Longitud};
use crate::dominio::especie::CatalogoEspecies;

use super::{ErrorFuente, FuenteAvistamientos};

#[derive(Debug, Clone, Copy)]
pub struct INaturalist;

impl FuenteAvistamientos for INaturalist {
    fn leer(&self, pagina: &str, catalogo: &CatalogoEspecies) -> Result<Avistamiento, ErrorFuente> {
        let id_observacion = extraer_id_observacion(pagina).ok_or(
            ErrorFuente::CampoNoEncontrado("el identificador de la observación"),
        )?;
        let nombre_cientifico = extraer_nombre_cientifico(pagina)
            .ok_or(ErrorFuente::CampoNoEncontrado("el nombre científico"))?;
        let (latitud, longitud) =
            extraer_coordenadas(pagina).ok_or(ErrorFuente::CampoNoEncontrado("las coordenadas"))?;
        let coordenadas = Coordenadas::new(Latitud::new(latitud)?, Longitud::new(longitud)?);
        Ok(Avistamiento::new(
            catalogo,
            Fuente::INaturalist,
            id_observacion,
            &nombre_cientifico,
            extraer_fecha(pagina)?,
            coordenadas,
        )?)
    }
}

fn extraer_id_observacion(html: &str) -> Option<String> {
    const MARCA: &str = "INITIAL_OBSERVATION_ID = \"";
    let inicio = html.find(MARCA)? + MARCA.len();
    let resto = ventana(html, inicio, 25);
    let id = &resto[..resto.find('"')?];
    (!id.is_empty() && id.chars().all(|c| c.is_ascii_digit())).then(|| id.to_owned())
}

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

fn extraer_fecha(html: &str) -> Result<SystemTime, ErrorFuente> {
    let fecha = contenidos_de_etiquetas(html, "name", "DCTERMS.issued")
        .into_iter()
        .next()
        .ok_or(ErrorFuente::CampoNoEncontrado("la fecha"))?;
    parsear_fecha(&fecha).ok_or(ErrorFuente::ValorInvalido("la fecha"))
}

fn parsear_fecha(texto: &str) -> Option<SystemTime> {
    let mut partes = texto.trim().get(..10)?.split('-');
    let anio: i64 = partes.next()?.parse().ok()?;
    let mes: i64 = partes.next()?.parse().ok()?;
    let dia: i64 = partes.next()?.parse().ok()?;
    if partes.next().is_some()
        || !(1..=12).contains(&mes)
        || dia < 1
        || dia > dias_del_mes(anio, mes)
    {
        return None;
    }
    let dias = u64::try_from(dias_desde_1970(anio, mes, dia)).ok()?;
    Some(SystemTime::UNIX_EPOCH + Duration::from_secs(dias * SEGUNDOS_POR_DIA))
}

fn dias_del_mes(anio: i64, mes: i64) -> i64 {
    match mes {
        2 if anio % 4 == 0 && (anio % 100 != 0 || anio % 400 == 0) => 29,
        2 => 28,
        4 | 6 | 9 | 11 => 30,
        _ => 31,
    }
}

fn dias_desde_1970(anio: i64, mes: i64, dia: i64) -> i64 {
    let anio = if mes <= 2 { anio - 1 } else { anio };
    let era = anio.div_euclid(400);
    let anio_de_era = anio - era * 400;
    let dia_del_anio = (153 * ((mes + 9) % 12) + 2) / 5 + dia - 1;
    let dia_de_era = anio_de_era * 365 + anio_de_era / 4 - anio_de_era / 100 + dia_del_anio;
    era * 146_097 + dia_de_era - 719_468
}

fn extraer_coordenadas(html: &str) -> Option<(f64, f64)> {
    coordenadas_de_enlace_de_mapa(html).or_else(|| coordenadas_de_atributos(html))
}

fn coordenadas_de_enlace_de_mapa(html: &str) -> Option<(f64, f64)> {
    html.match_indices("/@")
        .find_map(|(posicion, _)| par_de_coordenadas(ventana(html, posicion + 2, 48)))
}

fn par_de_coordenadas(texto: &str) -> Option<(f64, f64)> {
    let mut partes = texto.split(',');
    let latitud = numero_al_principio(partes.next()?)?;
    let longitud = numero_al_principio(partes.next()?)?;
    (latitud.abs() <= 90.0 && longitud.abs() <= 180.0).then_some((latitud, longitud))
}

fn numero_al_principio(texto: &str) -> Option<f64> {
    let fin = texto
        .find(|c: char| !(c.is_ascii_digit() || matches!(c, '.' | '-' | '+')))
        .unwrap_or(texto.len());
    texto[..fin].parse().ok()
}

fn coordenadas_de_atributos(html: &str) -> Option<(f64, f64)> {
    let latitud = numero_de_atributo(html, "data-lat")?;
    let longitud =
        numero_de_atributo(html, "data-lng").or_else(|| numero_de_atributo(html, "data-lon"))?;
    Some((latitud, longitud))
}

fn numero_de_atributo(html: &str, atributo: &str) -> Option<f64> {
    let marca = format!("{atributo}=\"");
    let inicio = html.find(&marca)? + marca.len();
    let valor = ventana(html, inicio, 24);
    valor[..valor.find('"')?].trim().parse().ok()
}

fn ventana(texto: &str, desde: usize, longitud: usize) -> &str {
    let desde = desde.min(texto.len());
    let mut hasta = desde.saturating_add(longitud).min(texto.len());
    while !texto.is_char_boundary(hasta) {
        hasta -= 1;
    }
    texto.get(desde..hasta).unwrap_or("")
}

fn contenidos_de_etiquetas(html: &str, atributo: &str, valor: &str) -> Vec<String> {
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
    fn una_pagina_que_no_es_una_observacion_da_error() {
        let catalogo = CatalogoEspecies::new();
        assert_eq!(
            INaturalist.leer("<html></html>", &catalogo),
            Err(ErrorFuente::CampoNoEncontrado(
                "el identificador de la observación"
            ))
        );
    }
}
