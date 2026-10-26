use std::time::SystemTime;

use crate::dominio::avistamiento::{PresenciaDepredador, RegistroAvistamientos};
use crate::dominio::depredacion::RedDepredacion;
use crate::dominio::error::ErrorDominio;
use crate::dominio::especie::CatalogoEspecies;
use crate::dominio::tramo::Rios;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PeticionZona {
    pub nombre_comun: String,
    pub nombre_rio: String,
    pub dias: u64,
    pub fecha_referencia: SystemTime,
}

#[derive(Debug, Clone, Copy)]
pub struct FuentesDeVerdad<'a> {
    pub catalogo: &'a CatalogoEspecies,
    pub red_depredacion: &'a RedDepredacion,
    pub registro_avistamientos: &'a RegistroAvistamientos,
    pub rios: &'a Rios,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResultadoDepredador {
    pub nombre_cientifico: String,
    pub cantidad_avistamientos: usize,
    pub presencia: PresenciaDepredador,
}

/// Dice, por cada depredador invasor del nombre común, cuántos avistamientos recientes tiene en el río.
pub fn consultar_zona(
    peticion: &PeticionZona,
    fuentes: &FuentesDeVerdad,
) -> Result<Vec<ResultadoDepredador>, ErrorDominio> {
    let depredadores = fuentes
        .red_depredacion
        .depredadores_de_nombre_comun(fuentes.catalogo, &peticion.nombre_comun)?;
    fuentes.rios.tramos_de_rio(&peticion.nombre_rio)?;
    depredadores
        .into_iter()
        .map(|nombre| resumir_depredador(nombre, peticion, fuentes))
        .collect()
}

/// Cuenta los avistamientos recientes del depredador en el río de la petición.
fn resumir_depredador(
    nombre_cientifico: &str,
    peticion: &PeticionZona,
    fuentes: &FuentesDeVerdad,
) -> Result<ResultadoDepredador, ErrorDominio> {
    let mut cantidad = 0;
    for avistamiento in fuentes
        .registro_avistamientos
        .avistamientos_de_especie(nombre_cientifico)
    {
        if avistamiento.es_reciente(peticion.fecha_referencia, peticion.dias)
            && avistamiento.pertenece_a_rio(fuentes.rios, &peticion.nombre_rio)?
        {
            cantidad += 1;
        }
    }
    Ok(ResultadoDepredador {
        nombre_cientifico: nombre_cientifico.to_owned(),
        cantidad_avistamientos: cantidad,
        presencia: PresenciaDepredador::desde_cantidad(cantidad),
    })
}
