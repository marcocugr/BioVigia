use std::collections::BTreeSet;

use crate::dominio::avistamiento::Fuente;
use crate::dominio::error::ErrorDominio;
use crate::dominio::especie::CatalogoEspecies;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FaseVital {
    HuevoEmbrion,
    LarvaAlevin,
    Juvenil,
    Adulto,
}

impl FaseVital {
    pub const TODAS: [FaseVital; 4] = [
        FaseVital::HuevoEmbrion,
        FaseVital::LarvaAlevin,
        FaseVital::Juvenil,
        FaseVital::Adulto,
    ];
}

#[derive(Debug, Clone)]
pub struct Depredacion {
    nombre_cientifico_depredador: String,
    nombre_cientifico_presa: String,
    fases_afectadas: BTreeSet<FaseVital>,
    fuente: Option<Fuente>,
}

impl Depredacion {
    pub fn new(
        catalogo: &CatalogoEspecies,
        depredador: &str,
        presa: &str,
        fases_afectadas: BTreeSet<FaseVital>,
        fuente: Option<Fuente>,
    ) -> Result<Self, ErrorDominio> {
        let nombre_cientifico_depredador = catalogo.especie_invasora(depredador)?.to_owned();
        let nombre_cientifico_presa = catalogo.especie(presa)?.to_owned();
        if nombre_cientifico_depredador == nombre_cientifico_presa {
            return Err(ErrorDominio::DepredacionSobreMismaEspecie);
        }
        if fases_afectadas.is_empty() {
            return Err(ErrorDominio::FaseVitalRequerida);
        }
        Ok(Self {
            nombre_cientifico_depredador,
            nombre_cientifico_presa,
            fases_afectadas,
            fuente,
        })
    }
}

impl Depredacion {
    pub fn fuente(&self) -> Option<Fuente> {
        self.fuente
    }
}

impl PartialEq for Depredacion {
    fn eq(&self, otra: &Self) -> bool {
        self.nombre_cientifico_depredador == otra.nombre_cientifico_depredador
            && self.nombre_cientifico_presa == otra.nombre_cientifico_presa
            && self.fases_afectadas == otra.fases_afectadas
    }
}

impl Eq for Depredacion {}

#[derive(Debug, Clone, Default)]
pub struct RedDepredacion {
    depredaciones: Vec<Depredacion>,
}

impl RedDepredacion {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn registrar(&mut self, depredacion: Depredacion) -> Result<(), ErrorDominio> {
        if self.depredaciones.iter().any(|otra| {
            otra.nombre_cientifico_depredador == depredacion.nombre_cientifico_depredador
                && otra.nombre_cientifico_presa == depredacion.nombre_cientifico_presa
        }) {
            return Err(ErrorDominio::DepredacionRepetida);
        }
        self.depredaciones.push(depredacion);
        Ok(())
    }

    /// Devuelve, sin repetir y ordenados, los depredadores de las especies que tienen ese nombre común.
    pub fn depredadores_de_nombre_comun(
        &self,
        catalogo: &CatalogoEspecies,
        nombre_comun: &str,
    ) -> Result<Vec<&str>, ErrorDominio> {
        let presas = catalogo.especies_con_nombre_comun(nombre_comun)?;
        let depredadores: BTreeSet<&str> = self
            .depredaciones
            .iter()
            .filter(|d| presas.contains(&d.nombre_cientifico_presa.as_str()))
            .map(|d| d.nombre_cientifico_depredador.as_str())
            .collect();
        Ok(depredadores.into_iter().collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn catalogo() -> CatalogoEspecies {
        let mut catalogo = CatalogoEspecies::new();
        let especies: [(&str, &[&str], bool, &[&str]); 4] = [
            ("Neogale vison", &["Visón americano"], true, &["Mustela vison"]),
            ("Procambarus clarkii", &["Cangrejo rojo americano"], false, &[]),
            ("Pacifastacus leniusculus", &["Cangrejo señal"], true, &[]),
            ("Micropterus salmoides", &["Black bass"], true, &[]),
        ];
        for (cientifico, comunes, invasora, anteriores) in especies {
            catalogo
                .registrar(
                    cientifico.into(),
                    comunes.iter().map(|n| n.to_string()).collect(),
                    invasora,
                    anteriores.iter().map(|n| n.to_string()).collect(),
                )
                .unwrap();
        }
        catalogo
    }

    fn todas_las_fases() -> BTreeSet<FaseVital> {
        FaseVital::TODAS.iter().copied().collect()
    }

    fn depredacion(catalogo: &CatalogoEspecies, depredador: &str, presa: &str) -> Depredacion {
        Depredacion::new(catalogo, depredador, presa, todas_las_fases(), None).unwrap()
    }

    #[test]
    fn todas_las_fases_son_cuatro_y_distintas() {
        let fases: BTreeSet<FaseVital> = FaseVital::TODAS.iter().copied().collect();
        assert_eq!(fases.len(), 4);
    }

    #[test]
    fn una_depredacion_valida_guarda_sus_datos_con_los_nombres_cientificos_vigentes() {
        let catalogo = catalogo();
        let depredacion = Depredacion::new(
            &catalogo,
            "Mustela vison",
            "Procambarus clarkii",
            BTreeSet::from([FaseVital::Adulto]),
            Some(Fuente::Gbif),
        )
        .unwrap();
        assert_eq!(depredacion.nombre_cientifico_depredador, "Neogale vison");
        assert_eq!(depredacion.nombre_cientifico_presa, "Procambarus clarkii");
        assert_eq!(depredacion.fases_afectadas.len(), 1);
        assert_eq!(depredacion.fuente(), Some(Fuente::Gbif));
    }

    #[test]
    fn la_presa_no_tiene_que_ser_invasora() {
        let catalogo = catalogo();
        assert!(Depredacion::new(&catalogo, "Neogale vison", "Procambarus clarkii", todas_las_fases(), None).is_ok());
    }

    #[test]
    fn un_depredador_que_no_es_invasor_da_error() {
        let catalogo = catalogo();
        let resultado = Depredacion::new(&catalogo, "Procambarus clarkii", "Neogale vison", todas_las_fases(), None);
        assert_eq!(resultado.unwrap_err(), ErrorDominio::EspecieNoInvasora);
    }

    #[test]
    fn un_depredador_desconocido_da_error() {
        let catalogo = catalogo();
        let resultado = Depredacion::new(&catalogo, "Canis lupus", "Procambarus clarkii", todas_las_fases(), None);
        assert_eq!(resultado.unwrap_err(), ErrorDominio::EspecieDesconocida);
    }

    #[test]
    fn una_presa_desconocida_da_error() {
        let catalogo = catalogo();
        let resultado = Depredacion::new(&catalogo, "Neogale vison", "Rattus rattus", todas_las_fases(), None);
        assert_eq!(resultado.unwrap_err(), ErrorDominio::EspecieDesconocida);
    }

    #[test]
    fn el_depredador_y_la_presa_no_pueden_ser_la_misma_especie_ni_con_un_nombre_anterior() {
        let catalogo = catalogo();
        for presa in ["Neogale vison", "Mustela vison"] {
            let resultado = Depredacion::new(&catalogo, "Neogale vison", presa, todas_las_fases(), None);
            assert_eq!(resultado.unwrap_err(), ErrorDominio::DepredacionSobreMismaEspecie);
        }
    }

    #[test]
    fn una_depredacion_sin_fases_da_error() {
        let catalogo = catalogo();
        let resultado = Depredacion::new(&catalogo, "Neogale vison", "Procambarus clarkii", BTreeSet::new(), None);
        assert_eq!(resultado.unwrap_err(), ErrorDominio::FaseVitalRequerida);
    }

    #[test]
    fn registrar_la_misma_pareja_dos_veces_da_error_aunque_cambien_las_fases() {
        let catalogo = catalogo();
        let mut red = RedDepredacion::new();
        red.registrar(depredacion(&catalogo, "Neogale vison", "Procambarus clarkii")).unwrap();
        let con_otras_fases = Depredacion::new(
            &catalogo,
            "Mustela vison",
            "Procambarus clarkii",
            BTreeSet::from([FaseVital::Juvenil]),
            None,
        )
        .unwrap();
        assert_eq!(red.registrar(con_otras_fases), Err(ErrorDominio::DepredacionRepetida));
    }

    #[test]
    fn cangrejo_devuelve_el_vison_porque_depreda_a_procambarus_clarkii() {
        let catalogo = catalogo();
        let mut red = RedDepredacion::new();
        red.registrar(depredacion(&catalogo, "Neogale vison", "Procambarus clarkii")).unwrap();
        assert_eq!(red.depredadores_de_nombre_comun(&catalogo, "cangrejo"), Ok(vec!["Neogale vison"]));
    }

    #[test]
    fn una_presa_sin_depredadores_registrados_da_lista_vacia_y_no_error() {
        let catalogo = catalogo();
        let red = RedDepredacion::new();
        assert_eq!(red.depredadores_de_nombre_comun(&catalogo, "cangrejo"), Ok(vec![]));
    }

    #[test]
    fn un_depredador_de_otra_especie_no_sale() {
        let catalogo = catalogo();
        let mut red = RedDepredacion::new();
        red.registrar(depredacion(&catalogo, "Neogale vison", "Micropterus salmoides")).unwrap();
        assert_eq!(red.depredadores_de_nombre_comun(&catalogo, "cangrejo"), Ok(vec![]));
    }

    #[test]
    fn un_depredador_que_come_dos_especies_del_mismo_nombre_comun_sale_una_sola_vez() {
        let catalogo = catalogo();
        let mut red = RedDepredacion::new();
        red.registrar(depredacion(&catalogo, "Neogale vison", "Procambarus clarkii")).unwrap();
        red.registrar(depredacion(&catalogo, "Neogale vison", "Pacifastacus leniusculus")).unwrap();
        assert_eq!(red.depredadores_de_nombre_comun(&catalogo, "cangrejo"), Ok(vec!["Neogale vison"]));
    }

    #[test]
    fn los_depredadores_salen_ordenados_por_nombre_cientifico() {
        let catalogo = catalogo();
        let mut red = RedDepredacion::new();
        red.registrar(depredacion(&catalogo, "Neogale vison", "Procambarus clarkii")).unwrap();
        red.registrar(depredacion(&catalogo, "Micropterus salmoides", "Procambarus clarkii")).unwrap();
        assert_eq!(
            red.depredadores_de_nombre_comun(&catalogo, "cangrejo"),
            Ok(vec!["Micropterus salmoides", "Neogale vison"])
        );
    }

    #[test]
    fn un_nombre_comun_que_no_existe_da_error_en_vez_de_lista_vacia() {
        let catalogo = catalogo();
        let red = RedDepredacion::new();
        assert_eq!(
            red.depredadores_de_nombre_comun(&catalogo, "unicornio"),
            Err(ErrorDominio::NombreComunInexistente)
        );
    }

    #[test]
    fn un_nombre_comun_vacio_da_error() {
        let catalogo = catalogo();
        let red = RedDepredacion::new();
        assert_eq!(
            red.depredadores_de_nombre_comun(&catalogo, " "),
            Err(ErrorDominio::NombreComunRequerido)
        );
    }
}
