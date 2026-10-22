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

    pub fn depredadores_de_nombre_comun(
        &self,
        catalogo: &CatalogoEspecies,
        nombre_comun: &str,
    ) -> Result<Vec<&str>, ErrorDominio> {
        let presas = catalogo.especies_con_nombre_comun(nombre_comun)?;
        let mut depredadores: Vec<&str> = self
            .depredaciones
            .iter()
            .filter(|d| presas.contains(&d.nombre_cientifico_presa.as_str()))
            .map(|d| d.nombre_cientifico_depredador.as_str())
            .collect::<std::collections::BTreeSet<_>>()
            .into_iter()
            .collect();
        depredadores.sort_unstable();
        Ok(depredadores)
    }
}
