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

    pub fn depredadores_de(&self, presa: &str) -> Vec<&str> {
        let mut depredadores: Vec<&str> = self
            .depredaciones
            .iter()
            .filter(|d| d.nombre_cientifico_presa == presa)
            .map(|d| d.nombre_cientifico_depredador.as_str())
            .collect();
        depredadores.sort_unstable();
        depredadores
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn catalogo() -> CatalogoEspecies {
        let mut catalogo = CatalogoEspecies::new();
        for (cientifico, comun, invasora) in [
            ("Procambarus clarkii", "Cangrejo rojo americano", false),
            ("Neogale vison", "Visón americano", true),
            ("Micropterus salmoides", "Black bass", true),
        ] {
            catalogo
                .registrar(cientifico.into(), comun.into(), invasora, vec![])
                .unwrap();
        }
        catalogo
    }

    fn red() -> RedDepredacion {
        let catalogo = catalogo();
        let mut red = RedDepredacion::new();
        for (depredador, presa) in [
            ("Neogale vison", "Procambarus clarkii"),
            ("Micropterus salmoides", "Procambarus clarkii"),
            ("Neogale vison", "Micropterus salmoides"),
        ] {
            let fases = FaseVital::TODAS.into_iter().collect();
            let depredacion = Depredacion::new(&catalogo, depredador, presa, fases, None).unwrap();
            red.registrar(depredacion).unwrap();
        }
        red
    }

    #[test]
    fn los_depredadores_de_una_presa_salen_ordenados_por_nombre_cientifico_y_sin_repetir() {
        assert_eq!(
            red().depredadores_de("Procambarus clarkii"),
            vec!["Micropterus salmoides", "Neogale vison"]
        );
    }

    #[test]
    fn una_presa_sin_depredadores_devuelve_una_lista_vacia() {
        assert!(red().depredadores_de("Neogale vison").is_empty());
    }
}
