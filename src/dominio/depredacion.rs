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

    pub fn depredadores_de(&self, presa: &str) -> Vec<String> {
        self.depredaciones
            .iter()
            .filter(|d| d.nombre_cientifico_presa == presa)
            .map(|d| d.nombre_cientifico_depredador.clone())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assertor::VecAssertion;

    #[test]
    fn buscar_depredadores_de_presa_devuelve_todos_los_depredadores() {
        let mut red = RedDepredacion::new();
        red.registrar(Depredacion { nombre_cientifico_depredador: "Neogale vison".to_string(), nombre_cientifico_presa: "Procambarus clarkii".to_string(), fases_afectadas: BTreeSet::from([FaseVital::Adulto]), fuente: None }).unwrap();
        red.registrar(Depredacion { nombre_cientifico_depredador: "Lutra lutra".to_string(), nombre_cientifico_presa: "Procambarus clarkii".to_string(), fases_afectadas: BTreeSet::from([FaseVital::Adulto]), fuente: None }).unwrap();
        let depredadores = red.depredadores_de("Procambarus clarkii");
        assertor::assert_that!(depredadores).has_length(2);
    }
}
