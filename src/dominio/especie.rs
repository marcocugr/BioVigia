use std::collections::{HashMap, HashSet};

use crate::dominio::error::ErrorDominio;
use crate::dominio::texto::normalizar;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Especie {
    nombres_comunes: HashSet<String>,
    invasora: bool,
}

#[derive(Debug, Clone, Default)]
pub struct CatalogoEspecies {
    especies: HashMap<String, Especie>,
    nombres_anteriores: HashMap<String, String>,
}

impl CatalogoEspecies {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn registrar(
        &mut self,
        nombre_cientifico: String,
        nombres_comunes: Vec<String>,
        invasora: bool,
        nombres_anteriores: Vec<String>,
    ) -> Result<(), ErrorDominio> {
        let nombre_cientifico = nombre_cientifico.trim().to_owned();
        let nombres_anteriores: Vec<String> = nombres_anteriores
            .iter()
            .map(|n| n.trim().to_owned())
            .collect();
        if nombre_cientifico.is_empty() || nombres_anteriores.iter().any(|n| n.is_empty()) {
            return Err(ErrorDominio::NombreCientificoRequerido);
        }
        let nombres_comunes = nombres_comunes_validos(nombres_comunes)?;
        let mut nuevos = HashSet::new();
        for nombre in std::iter::once(&nombre_cientifico).chain(&nombres_anteriores) {
            if !nuevos.insert(nombre)
                || self.especies.contains_key(nombre)
                || self.nombres_anteriores.contains_key(nombre)
            {
                return Err(ErrorDominio::NombreEspecieRepetido);
            }
        }
        for anterior in nombres_anteriores {
            self.nombres_anteriores
                .insert(anterior, nombre_cientifico.clone());
        }
        self.especies.insert(
            nombre_cientifico,
            Especie {
                nombres_comunes,
                invasora,
            },
        );
        Ok(())
    }

    pub(crate) fn especie(&self, nombre: &str) -> Result<&str, ErrorDominio> {
        let nombre = nombre.trim();
        match self.especies.get_key_value(nombre) {
            Some((nombre_cientifico, _)) => Ok(nombre_cientifico),
            None => self
                .nombres_anteriores
                .get(nombre)
                .map(String::as_str)
                .ok_or(ErrorDominio::EspecieDesconocida),
        }
    }

    pub(crate) fn especie_invasora(&self, nombre: &str) -> Result<&str, ErrorDominio> {
        let nombre_cientifico = self.especie(nombre)?;
        if self.especies[nombre_cientifico].invasora {
            Ok(nombre_cientifico)
        } else {
            Err(ErrorDominio::EspecieNoInvasora)
        }
    }

    pub fn especies_con_nombre_comun(
        &self,
        nombre_comun: &str,
    ) -> Result<Vec<&str>, ErrorDominio> {
        let termino = nombre_comun.trim();
        if termino.is_empty() {
            return Err(ErrorDominio::NombreComunRequerido);
        }
        let termino_norm = normalizar(termino);
        let mut resultado: Vec<&str> = self
            .especies
            .iter()
            .filter(|(_, e)| {
                e.nombres_comunes
                    .iter()
                    .any(|n| normalizar(n).contains(&termino_norm))
            })
            .map(|(nombre_cientifico, _)| nombre_cientifico.as_str())
            .collect();
        if resultado.is_empty() {
            return Err(ErrorDominio::NombreComunInexistente);
        }
        resultado.sort_unstable();
        Ok(resultado)
    }
}

fn nombres_comunes_validos(nombres: Vec<String>) -> Result<HashSet<String>, ErrorDominio> {
    let recortados: Vec<String> = nombres.iter().map(|n| n.trim().to_owned()).collect();
    if recortados.is_empty() || recortados.iter().any(String::is_empty) {
        return Err(ErrorDominio::NombreComunRequerido);
    }
    Ok(recortados.into_iter().collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cadenas(nombres: &[&str]) -> Vec<String> {
        nombres.iter().map(|n| n.to_string()).collect()
    }

    fn catalogo_de_prueba() -> CatalogoEspecies {
        let mut catalogo = CatalogoEspecies::new();
        catalogo
            .registrar(
                "Procambarus clarkii".into(),
                cadenas(&[
                    "Acocil rojo",
                    "Cangrejo americano",
                    "Cangrejo de río americano",
                    "Cangrejo rojo americano",
                ]),
                false,
                vec![],
            )
            .unwrap();
        catalogo
            .registrar(
                "Neogale vison".into(),
                cadenas(&["Visón Americano"]),
                true,
                cadenas(&["Mustela vison", "Neovison vison"]),
            )
            .unwrap();
        catalogo
    }

    #[test]
    fn una_especie_se_registra_con_varios_nombres_comunes_y_se_guardan_todos() {
        let catalogo = catalogo_de_prueba();
        assert_eq!(catalogo.especies["Procambarus clarkii"].nombres_comunes.len(), 4);
    }

    #[test]
    fn una_lista_vacia_de_nombres_comunes_da_error() {
        let mut catalogo = CatalogoEspecies::new();
        let resultado = catalogo.registrar("Especie uno".into(), vec![], true, vec![]);
        assert_eq!(resultado, Err(ErrorDominio::NombreComunRequerido));
    }

    #[test]
    fn un_nombre_comun_vacio_dentro_de_la_lista_da_error() {
        let mut catalogo = CatalogoEspecies::new();
        let resultado = catalogo.registrar("Especie uno".into(), cadenas(&["Cangrejo", ""]), true, vec![]);
        assert_eq!(resultado, Err(ErrorDominio::NombreComunRequerido));
    }

    #[test]
    fn un_nombre_comun_de_solo_espacios_dentro_de_la_lista_da_error() {
        let mut catalogo = CatalogoEspecies::new();
        let resultado = catalogo.registrar("Especie uno".into(), cadenas(&["Cangrejo", "   "]), true, vec![]);
        assert_eq!(resultado, Err(ErrorDominio::NombreComunRequerido));
    }

    #[test]
    fn los_nombres_comunes_repetidos_de_la_misma_especie_se_ignoran() {
        let mut catalogo = CatalogoEspecies::new();
        catalogo
            .registrar("Especie uno".into(), cadenas(&["Cangrejo", " Cangrejo ", "Cangrejo"]), true, vec![])
            .unwrap();
        assert_eq!(catalogo.especies["Especie uno"].nombres_comunes.len(), 1);
    }

    #[test]
    fn dos_especies_distintas_pueden_compartir_un_nombre_comun() {
        let mut catalogo = catalogo_de_prueba();
        catalogo
            .registrar("Pacifastacus leniusculus".into(), cadenas(&["Cangrejo señal"]), true, vec![])
            .unwrap();
        let especies = catalogo.especies_con_nombre_comun("cangrejo").unwrap();
        assert_eq!(especies, vec!["Pacifastacus leniusculus", "Procambarus clarkii"]);
    }

    #[test]
    fn un_nombre_cientifico_vacio_da_error() {
        let mut catalogo = CatalogoEspecies::new();
        let resultado = catalogo.registrar("  ".into(), cadenas(&["Cangrejo"]), true, vec![]);
        assert_eq!(resultado, Err(ErrorDominio::NombreCientificoRequerido));
    }

    #[test]
    fn un_nombre_anterior_vacio_da_error() {
        let mut catalogo = CatalogoEspecies::new();
        let resultado = catalogo.registrar("Especie uno".into(), cadenas(&["Cangrejo"]), true, cadenas(&[" "]));
        assert_eq!(resultado, Err(ErrorDominio::NombreCientificoRequerido));
    }

    #[test]
    fn un_nombre_cientifico_repetido_da_error() {
        let mut catalogo = catalogo_de_prueba();
        let resultado = catalogo.registrar("Neogale vison".into(), cadenas(&["Otro"]), true, vec![]);
        assert_eq!(resultado, Err(ErrorDominio::NombreEspecieRepetido));
    }

    #[test]
    fn un_nombre_anterior_que_choca_con_otra_especie_da_error() {
        let mut catalogo = catalogo_de_prueba();
        let resultado = catalogo.registrar("Otra especie".into(), cadenas(&["Otro"]), true, cadenas(&["Mustela vison"]));
        assert_eq!(resultado, Err(ErrorDominio::NombreEspecieRepetido));
    }

    #[test]
    fn el_nombre_cientifico_repetido_dentro_de_los_anteriores_da_error() {
        let mut catalogo = CatalogoEspecies::new();
        let resultado = catalogo.registrar("Especie uno".into(), cadenas(&["Uno"]), true, cadenas(&["Especie uno"]));
        assert_eq!(resultado, Err(ErrorDominio::NombreEspecieRepetido));
    }

    #[test]
    fn un_registro_fallido_no_deja_nada_guardado() {
        let mut catalogo = CatalogoEspecies::new();
        let _ = catalogo.registrar("Especie uno".into(), cadenas(&["Uno", ""]), true, cadenas(&["Antigua uno"]));
        assert_eq!(catalogo.especie("Antigua uno"), Err(ErrorDominio::EspecieDesconocida));
        assert_eq!(catalogo.especie("Especie uno"), Err(ErrorDominio::EspecieDesconocida));
    }

    #[test]
    fn especie_resuelve_el_nombre_actual_y_los_anteriores() {
        let catalogo = catalogo_de_prueba();
        assert_eq!(catalogo.especie("Neogale vison"), Ok("Neogale vison"));
        assert_eq!(catalogo.especie("Mustela vison"), Ok("Neogale vison"));
        assert_eq!(catalogo.especie("  Neovison vison "), Ok("Neogale vison"));
    }

    #[test]
    fn especie_desconocida_da_error() {
        let catalogo = catalogo_de_prueba();
        assert_eq!(catalogo.especie("Canis lupus"), Err(ErrorDominio::EspecieDesconocida));
    }

    #[test]
    fn especie_invasora_devuelve_la_especie_si_es_invasora() {
        let catalogo = catalogo_de_prueba();
        assert_eq!(catalogo.especie_invasora("Mustela vison"), Ok("Neogale vison"));
    }

    #[test]
    fn especie_invasora_da_error_si_la_especie_no_es_invasora() {
        let catalogo = catalogo_de_prueba();
        assert_eq!(catalogo.especie_invasora("Procambarus clarkii"), Err(ErrorDominio::EspecieNoInvasora));
    }

    #[test]
    fn especie_invasora_da_error_si_la_especie_no_existe() {
        let catalogo = catalogo_de_prueba();
        assert_eq!(catalogo.especie_invasora("Canis lupus"), Err(ErrorDominio::EspecieDesconocida));
    }

    #[test]
    fn cangrejo_encuentra_a_procambarus_clarkii() {
        let catalogo = catalogo_de_prueba();
        assert_eq!(catalogo.especies_con_nombre_comun("cangrejo"), Ok(vec!["Procambarus clarkii"]));
    }

    #[test]
    fn cangrejo_rojo_tambien_la_encuentra() {
        let catalogo = catalogo_de_prueba();
        assert_eq!(catalogo.especies_con_nombre_comun("cangrejo rojo"), Ok(vec!["Procambarus clarkii"]));
    }

    #[test]
    fn la_busqueda_no_distingue_mayusculas_ni_tildes() {
        let catalogo = catalogo_de_prueba();
        assert_eq!(catalogo.especies_con_nombre_comun("CANGREJO DE RIO"), Ok(vec!["Procambarus clarkii"]));
        assert_eq!(catalogo.especies_con_nombre_comun("vison"), Ok(vec!["Neogale vison"]));
        assert_eq!(catalogo.especies_con_nombre_comun("Visón"), Ok(vec!["Neogale vison"]));
    }

    #[test]
    fn la_busqueda_recorta_los_espacios_de_los_lados() {
        let catalogo = catalogo_de_prueba();
        assert_eq!(catalogo.especies_con_nombre_comun("  cangrejo  "), Ok(vec!["Procambarus clarkii"]));
    }

    #[test]
    fn una_especie_con_dos_nombres_que_contienen_la_busqueda_sale_una_sola_vez() {
        let catalogo = catalogo_de_prueba();
        let especies = catalogo.especies_con_nombre_comun("americano").unwrap();
        assert_eq!(especies, vec!["Neogale vison", "Procambarus clarkii"]);
    }

    #[test]
    fn pez_encuentra_pez_gato_y_pez_sol_sin_quedarse_con_la_primera_palabra() {
        let mut catalogo = CatalogoEspecies::new();
        catalogo.registrar("Ameiurus melas".into(), cadenas(&["Pez gato"]), true, vec![]).unwrap();
        catalogo.registrar("Lepomis gibbosus".into(), cadenas(&["Pez sol"]), true, vec![]).unwrap();
        assert_eq!(
            catalogo.especies_con_nombre_comun("pez"),
            Ok(vec!["Ameiurus melas", "Lepomis gibbosus"])
        );
        assert_eq!(catalogo.especies_con_nombre_comun("pez gato"), Ok(vec!["Ameiurus melas"]));
    }

    #[test]
    fn la_enie_se_encuentra_con_enie_minuscula_y_mayuscula() {
        let mut catalogo = CatalogoEspecies::new();
        catalogo.registrar("Rhea americana".into(), cadenas(&["Ñandú común"]), true, vec![]).unwrap();
        assert_eq!(catalogo.especies_con_nombre_comun("ñandu"), Ok(vec!["Rhea americana"]));
        assert_eq!(catalogo.especies_con_nombre_comun("ÑANDÚ"), Ok(vec!["Rhea americana"]));
        assert_eq!(catalogo.especies_con_nombre_comun("nandu"), Err(ErrorDominio::NombreComunInexistente));
    }

    #[test]
    fn una_busqueda_vacia_o_de_espacios_da_error_en_vez_de_devolver_todo() {
        let catalogo = catalogo_de_prueba();
        assert_eq!(catalogo.especies_con_nombre_comun(""), Err(ErrorDominio::NombreComunRequerido));
        assert_eq!(catalogo.especies_con_nombre_comun("   "), Err(ErrorDominio::NombreComunRequerido));
    }

    #[test]
    fn un_nombre_comun_que_no_tiene_ninguna_especie_da_nombre_comun_inexistente() {
        let catalogo = catalogo_de_prueba();
        assert_eq!(catalogo.especies_con_nombre_comun("unicornio"), Err(ErrorDominio::NombreComunInexistente));
    }

    #[test]
    fn un_catalogo_vacio_da_nombre_comun_inexistente() {
        let catalogo = CatalogoEspecies::new();
        assert_eq!(catalogo.especies_con_nombre_comun("cangrejo"), Err(ErrorDominio::NombreComunInexistente));
    }

    #[test]
    fn las_especies_salen_ordenadas_por_nombre_cientifico() {
        let mut catalogo = CatalogoEspecies::new();
        for nombre in ["Zeta zeta", "Alfa alfa", "Mu mu"] {
            catalogo.registrar(nombre.into(), cadenas(&["Cangrejo"]), true, vec![]).unwrap();
        }
        assert_eq!(
            catalogo.especies_con_nombre_comun("cangrejo"),
            Ok(vec!["Alfa alfa", "Mu mu", "Zeta zeta"])
        );
    }
}
