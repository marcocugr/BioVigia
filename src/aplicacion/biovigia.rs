use std::collections::HashMap;
use std::time::SystemTime;
use crate::dominio::avistamiento::RegistroAvistamientos;
use crate::dominio::depredacion::RedDepredacion;
use crate::dominio::especie::CatalogoEspecies;
use crate::dominio::tramo::TramoDeRio;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EstadoPresencia {
    PresenciaDetectada { avistamientos: usize },
    SinRegistrosRecientes,
}

pub fn consultar_depredadores_en_zona(catalogo: &CatalogoEspecies, red: &RedDepredacion, registro: &RegistroAvistamientos, tramo: &TramoDeRio, nombre_comun: &str, fecha_referencia: SystemTime, dias_ventana: u64) -> HashMap<String, EstadoPresencia> {
    let especies = catalogo.especies_por_nombre_comun(nombre_comun);
    let mut resultado = HashMap::new();
    for especie in especies {
        let depredadores = red.depredadores_de(&especie);
        for depredador in depredadores {
            if resultado.contains_key(&depredador) { continue; }
            let avistamientos_temporales = registro.avistamientos_en_ventana(fecha_referencia, dias_ventana);
            let avistamientos_zona: Vec<_> = avistamientos_temporales.iter().filter(|a| a.nombre_cientifico() == depredador && tramo.pertenece(a.coordenadas())).collect();
            let contador = avistamientos_zona.len();
            let estado = if contador > 0 { EstadoPresencia::PresenciaDetectada { avistamientos: contador } } else { EstadoPresencia::SinRegistrosRecientes };
            resultado.insert(depredador, estado);
        }
    }
    resultado
}
