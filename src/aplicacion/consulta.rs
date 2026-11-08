use std::time::SystemTime;

use crate::dominio::avistamiento::RegistroAvistamientos;
use crate::dominio::depredacion::RedDepredacion;
use crate::dominio::error::ErrorDominio;
use crate::dominio::especie::CatalogoEspecies;
use crate::dominio::tramo::TramoDeRio;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResultadoDepredador {
    pub nombre_cientifico: String,
    pub avistamientos: usize,
}

pub fn consultar_zona(
    catalogo: &CatalogoEspecies,
    red: &RedDepredacion,
    registro: &RegistroAvistamientos,
    especie_objetivo: &str,
    zona: &TramoDeRio,
    referencia: SystemTime,
    dias: u64,
) -> Result<Vec<ResultadoDepredador>, ErrorDominio> {
    let presa = catalogo.especie_con_nombre_comun(especie_objetivo)?;
    Ok(red
        .depredadores_de(presa)
        .into_iter()
        .map(|depredador| ResultadoDepredador {
            nombre_cientifico: depredador.to_owned(),
            avistamientos: registro
                .que_cuentan(depredador, zona, referencia, dias)
                .len(),
        })
        .collect())
}
