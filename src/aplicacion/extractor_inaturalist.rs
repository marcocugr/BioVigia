use std::fs;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::dominio::avistamiento::{Avistamiento, EvidenciaFotografica, Fuente, GradoCalidad};
use crate::dominio::coordenadas::{Coordenadas, Latitud, Longitud};
use crate::dominio::especie::CatalogoEspecies;
use crate::dominio::error::ErrorDominio;

pub fn extraer_avistamiento_inaturalist(ruta_html: &str) -> Result<Avistamiento, ErrorDominio> {
    let html = fs::read_to_string(ruta_html).map_err(|_| ErrorDominio::AvistamientoIncompleto)?;
    let nombre = extraer_nombre_cientifico(&html)?;
    let coordenadas = extraer_coordenadas(&html)?;
    let fecha = extraer_fecha(&html)?;
    let mut catalogo = CatalogoEspecies::new();
    catalogo.registrar(nombre.clone(), nombre.clone(), true, vec![])?;
    Avistamiento::new(&catalogo, Fuente::INaturalist, "395192703".to_owned(), &nombre, fecha, coordenadas, Some(GradoCalidad::Investigacion), EvidenciaFotografica::new(None, vec![])?)
}

fn entre<'a>(texto: &'a str, inicio: &str, fin: &str) -> Result<&'a str, ErrorDominio> {
    let resto = &texto[texto.find(inicio).ok_or(ErrorDominio::AvistamientoIncompleto)? + inicio.len()..];
    Ok(&resto[..resto.find(fin).ok_or(ErrorDominio::AvistamientoIncompleto)?])
}

fn num<T: std::str::FromStr>(texto: &str) -> Result<T, ErrorDominio> {
    texto.parse().map_err(|_| ErrorDominio::AvistamientoIncompleto)
}

fn extraer_nombre_cientifico(html: &str) -> Result<String, ErrorDominio> {
    Ok(entre(html, "sciname species secondary-name", "</a>")?.rsplit('>').next().unwrap_or_default().trim().to_owned())
}

fn extraer_coordenadas(html: &str) -> Result<Coordenadas, ErrorDominio> {
    let (lat, lon) = entre(html, "google.com/maps?q=", "\"")?.split_once(',').ok_or(ErrorDominio::AvistamientoIncompleto)?;
    Ok(Coordenadas::new(Latitud::new(num(lat)?)?, Longitud::new(num(lon)?)?))
}

fn extraer_fecha(html: &str) -> Result<SystemTime, ErrorDominio> {
    let t = entre(html.split("class=\"date\" title=\"").nth(2).ok_or(ErrorDominio::AvistamientoIncompleto)?, "", "\"")?;
    let (y, m, d): (i64, i64, i64) = (num(&t[0..4])?, num(&t[5..7])?, num(&t[8..10])?);
    let y = if m <= 2 { y - 1 } else { y };
    let doy = (153 * (if m > 2 { m - 3 } else { m + 9 }) + 2) / 5 + d - 1;
    let yoe = y.rem_euclid(400);
    let dias = y.div_euclid(400) * 146097 + yoe * 365 + yoe / 4 - yoe / 100 + doy - 719468;
    let hora = num::<i64>(&t[11..13])? * 3600 + num::<i64>(&t[14..16])? * 60 + num::<i64>(&t[17..19])?;
    let desfase = (if &t[19..20] == "-" { -1 } else { 1 }) * (num::<i64>(&t[20..22])? * 3600 + num::<i64>(&t[23..25])? * 60);
    Ok(UNIX_EPOCH + Duration::from_secs((dias * 86400 + hora - desfase) as u64))
}

#[cfg(test)]
mod tests {
    use super::*;
    use assertor::{assert_that, EqualityAssertion};

    #[test]
    fn extrae_avistamiento_de_html_inaturalist() {
        let mut catalogo = CatalogoEspecies::new();
        catalogo.registrar("Neogale vison".to_owned(), "Neogale vison".to_owned(), true, vec![]).unwrap();
        let coordenadas = Coordenadas::new(Latitud::new(40.0400030897).unwrap(), Longitud::new(-4.6470027317).unwrap());
        let fecha = UNIX_EPOCH + Duration::from_secs(1_787_921_200);
        let avistamiento_esperado = Avistamiento::new(&catalogo, Fuente::INaturalist, "395192703".to_owned(), "Neogale vison", fecha, coordenadas, Some(GradoCalidad::Investigacion), EvidenciaFotografica::new(None, vec![]).unwrap()).unwrap();
        let avistamiento = extraer_avistamiento_inaturalist("tests/fixtures/inaturalist/vison.html").unwrap();
        assert_that!(avistamiento).is_equal_to(avistamiento_esperado);
    }
}