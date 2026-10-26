use std::collections::BTreeSet;

use crate::dominio::depredacion::{Depredacion, FaseVital};
use crate::dominio::especie::CatalogoEspecies;

use super::ErrorFuente;

const BLOQUE: &str = r#"<div class="interaction search-result">"#;
const DEPREDACION: &str = r#"<div class="interaction _preysOn">"#;
const NOMBRE_DE_TAXON: &str = r#"data-taxon-name=""#;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RelacionDepredacion {
    pub depredador: String,
    pub presa: String,
}

impl RelacionDepredacion {
    pub fn a_depredacion(&self, catalogo: &CatalogoEspecies) -> Result<Depredacion, ErrorFuente> {
        let fases: BTreeSet<FaseVital> = FaseVital::TODAS.iter().copied().collect();
        Ok(Depredacion::new(catalogo, &self.depredador, &self.presa, fases, None)?)
    }
}

pub fn procesar_globi(
    html: &str,
    nombre_depredador: &str,
    filtro_presa: Option<&str>,
) -> Vec<RelacionDepredacion> {
    let mut relaciones: Vec<RelacionDepredacion> = Vec::new();
    for bloque in bloques(html).into_iter().filter(|b| b.contains(DEPREDACION)) {
        let Some(presa) = nombres_de_taxon(bloque).into_iter().nth(1) else {
            continue;
        };
        if filtro_presa.is_some_and(|filtro| filtro != presa) {
            continue;
        }
        let relacion = RelacionDepredacion {
            depredador: nombre_depredador.to_owned(),
            presa,
        };
        if !relaciones.contains(&relacion) {
            relaciones.push(relacion);
        }
    }
    relaciones
}

fn bloques(html: &str) -> Vec<&str> {
    let mut inicios: Vec<usize> = html.match_indices(BLOQUE).map(|(posicion, _)| posicion).collect();
    inicios.push(html.len());
    inicios.windows(2).map(|par| &html[par[0]..par[1]]).collect()
}

fn nombres_de_taxon(bloque: &str) -> Vec<String> {
    let mut nombres = Vec::new();
    let mut resto = bloque;
    while let Some(posicion) = resto.find(NOMBRE_DE_TAXON) {
        resto = &resto[posicion + NOMBRE_DE_TAXON.len()..];
        let Some(fin) = resto.find('"') else {
            break;
        };
        if fin > 0 {
            nombres.push(resto[..fin].to_owned());
        }
        resto = &resto[fin + 1..];
    }
    nombres
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dominio::error::ErrorDominio;

    fn interaccion(tipo: &str, origen: &str, destino: &str) -> String {
        format!(
            r#"<div class="interaction search-result"><div class="interaction {tipo}"><div data-taxon-name="{origen}"></div><div data-taxon-name="{destino}"></div></div></div>"#
        )
    }

    fn html_de_prueba() -> String {
        [
            interaccion("_preysOn", "Mustela vison", "Procambarus clarkii"),
            interaccion("_preysOn", "Mustela vison", "Lithobates"),
            interaccion("_preysOn", "Mustela vison", "Lithobates"),
            interaccion("_eats", "Mustela vison", "Rattus"),
        ]
        .concat()
    }

    fn relacion(presa: &str) -> RelacionDepredacion {
        RelacionDepredacion {
            depredador: "Neogale vison".into(),
            presa: presa.into(),
        }
    }

    #[test]
    fn extrae_las_presas_sin_repetir_y_en_el_orden_de_la_pagina() {
        let relaciones = procesar_globi(&html_de_prueba(), "Neogale vison", None);
        assert_eq!(relaciones, vec![relacion("Procambarus clarkii"), relacion("Lithobates")]);
    }

    #[test]
    fn solo_cuentan_las_interacciones_de_depredacion() {
        let relaciones = procesar_globi(&html_de_prueba(), "Neogale vison", None);
        assert!(relaciones.iter().all(|r| r.presa != "Rattus"));
    }

    #[test]
    fn el_depredador_es_siempre_el_que_se_pide() {
        let relaciones = procesar_globi(&html_de_prueba(), "Neogale vison", None);
        assert!(relaciones.iter().all(|r| r.depredador == "Neogale vison"));
    }

    #[test]
    fn con_filtro_solo_sale_esa_presa() {
        let relaciones = procesar_globi(&html_de_prueba(), "Neogale vison", Some("Procambarus clarkii"));
        assert_eq!(relaciones, vec![relacion("Procambarus clarkii")]);
    }

    #[test]
    fn un_filtro_que_no_esta_en_la_pagina_da_lista_vacia() {
        assert!(procesar_globi(&html_de_prueba(), "Neogale vison", Some("Canis lupus")).is_empty());
    }

    #[test]
    fn una_interaccion_con_menos_de_dos_taxones_se_ignora() {
        let html = r#"<div class="interaction search-result"><div class="interaction _preysOn"><div data-taxon-name="Mustela vison"></div></div></div>"#;
        assert!(procesar_globi(html, "Neogale vison", None).is_empty());
    }

    #[test]
    fn un_html_vacio_o_sin_interacciones_da_lista_vacia() {
        assert!(procesar_globi("", "Neogale vison", None).is_empty());
        assert!(procesar_globi("<html>nada</html>", "Neogale vison", None).is_empty());
    }

    #[test]
    fn un_html_cortado_o_con_caracteres_raros_nunca_hace_panic() {
        for html in [
            r#"<div class="interaction search-result">"#,
            r#"<div class="interaction search-result"><div class="interaction _preysOn"><div data-taxon-name="ñ"#,
            r#"<div class="interaction search-result"><div class="interaction _preysOn">data-taxon-name=""#,
        ] {
            let _ = procesar_globi(html, "X y", None);
        }
    }

    #[test]
    fn la_relacion_se_convierte_en_una_depredacion_con_todas_las_fases_y_sin_fuente() {
        let mut catalogo = CatalogoEspecies::new();
        catalogo.registrar("Neogale vison".into(), vec!["Visón".into()], true, vec![]).unwrap();
        catalogo.registrar("Procambarus clarkii".into(), vec!["Cangrejo".into()], false, vec![]).unwrap();
        let depredacion = relacion("Procambarus clarkii").a_depredacion(&catalogo).unwrap();
        let esperada = Depredacion::new(
            &catalogo,
            "Neogale vison",
            "Procambarus clarkii",
            FaseVital::TODAS.iter().copied().collect(),
            None,
        )
        .unwrap();
        assert_eq!(depredacion, esperada);
        assert_eq!(depredacion.fuente(), None);
    }

    #[test]
    fn una_presa_que_no_esta_en_el_catalogo_no_se_convierte() {
        let mut catalogo = CatalogoEspecies::new();
        catalogo.registrar("Neogale vison".into(), vec!["Visón".into()], true, vec![]).unwrap();
        assert_eq!(
            relacion("Lithobates").a_depredacion(&catalogo),
            Err(ErrorFuente::Dominio(ErrorDominio::EspecieDesconocida))
        );
    }
}
