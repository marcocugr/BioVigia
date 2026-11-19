use std::collections::HashMap;
use std::time::SystemTime;
use crate::dominio::avistamiento::RegistroAvistamientos;
use crate::dominio::depredacion::RedDepredacion;
use crate::dominio::especie::CatalogoEspecies;
use crate::dominio::tramo::TramoDeRio;

pub struct BioVigia {
    pub catalogo: CatalogoEspecies,
    pub red: RedDepredacion,
    pub registro: RegistroAvistamientos,
    pub tramo: TramoDeRio,
}

impl BioVigia {
    pub fn new(catalogo: CatalogoEspecies, red: RedDepredacion, registro: RegistroAvistamientos, tramo: TramoDeRio) -> Self {
        Self { catalogo, red, registro, tramo }
    }

    pub fn consultar_depredadores_en_zona(&self, nombre_comun: &str, nombre_tramo: &str) -> HashMap<String, EstadoPresencia> {
        if self.tramo.codigo() != nombre_tramo {
            return HashMap::new();
        }

        let especies = self.catalogo.especies_por_nombre_comun(nombre_comun);
        let mut resultado = HashMap::new();
        let fecha_referencia = SystemTime::now();
        let dias_ventana = 365;

        for especie in especies {
            let depredadores = self.red.depredadores_de(&especie);
            for depredador in depredadores {
                if resultado.contains_key(&depredador) { continue; }
                let avistamientos_temporales = self.registro.avistamientos_en_ventana(fecha_referencia, dias_ventana);
                let avistamientos_zona: Vec<_> = avistamientos_temporales.iter().filter(|a| a.nombre_cientifico() == depredador && self.tramo.pertenece(a.coordenadas())).collect();
                let contador = avistamientos_zona.len();
                let estado = if contador > 0 { EstadoPresencia::PresenciaDetectada { avistamientos: contador } } else { EstadoPresencia::SinRegistrosRecientes };
                resultado.insert(depredador, estado);
            }
        }
        resultado
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EstadoPresencia {
    PresenciaDetectada { avistamientos: usize },
    SinRegistrosRecientes,
}
