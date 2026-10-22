use std::time::SystemTime;

use crate::dominio::avistamiento::{PresenciaDepredador, RegistroAvistamientos};
use crate::dominio::depredacion::RedDepredacion;
use crate::dominio::error::ErrorDominio;
use crate::dominio::especie::CatalogoEspecies;
use crate::dominio::tramo::Rios;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResultadoDepredador {
    pub nombre_cientifico: String,
    pub cantidad_avistamientos: usize,
    pub presencia: PresenciaDepredador,
}

pub fn consultar_zona(
    nombre_comun_presa: &str,
    nombre_rio: &str,
    dias: u64,
    fecha_referencia: SystemTime,
    catalogo: &CatalogoEspecies,
    red: &RedDepredacion,
    registro: &RegistroAvistamientos,
    rios: &Rios,
) -> Result<Vec<ResultadoDepredador>, ErrorDominio> {
    let _presas = catalogo.especies_con_nombre_comun(nombre_comun_presa)?;
    
    let depredadores = red.depredadores_de_nombre_comun(catalogo, nombre_comun_presa)?;
    
    let _tramos = rios.tramos_de_rio(nombre_rio)?;
    
    let mut resultados: Vec<ResultadoDepredador> = depredadores
        .iter()
        .map(|nombre_cientifico| {
            let avistamientos_filtrados: Vec<_> = registro
                .avistamientos_de_especie(nombre_cientifico)
                .iter()
                .filter(|a| a.es_reciente(fecha_referencia, dias))
                .filter(|a| a.pertenece_a_rio(rios, nombre_rio).unwrap_or(false))
                .map(|a| (*a).clone())
                .collect();
            
            let cantidad = avistamientos_filtrados.len();
            let presencia = PresenciaDepredador::desde_avistamientos(&avistamientos_filtrados);
            
            ResultadoDepredador {
                nombre_cientifico: nombre_cientifico.to_string(),
                cantidad_avistamientos: cantidad,
                presencia,
            }
        })
        .collect();
    
    resultados.sort_by(|a, b| a.nombre_cientifico.cmp(&b.nombre_cientifico));
    
    Ok(resultados)
}
