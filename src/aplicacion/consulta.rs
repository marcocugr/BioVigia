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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dominio::avistamiento::{Avistamiento, EvidenciaFotografica, Fuente};
    use crate::dominio::coordenadas::{Coordenadas, Latitud, Longitud};
    use crate::dominio::depredacion::{Depredacion, FaseVital};
    use crate::dominio::tramo::{GeometriaTramo, LineaTramo, TramoDeRio};
    use std::collections::BTreeSet;
    use std::time::Duration;

    const AVISTAMIENTO_1_SEP: u64 = 1_788_264_000;
    const REFERENCIA_5_SEP: u64 = 1_788_609_600;
    const REFERENCIA_20_SEP: u64 = 1_789_905_600;
    const LONGITUD_AVISTAMIENTO: f64 = -4.55;

    #[derive(Debug)]
    struct GeometriaQueFalla;

    impl GeometriaTramo for GeometriaQueFalla {
        fn contiene_coordenadas(&self, _: &Coordenadas, _: f64) -> Result<bool, ErrorDominio> {
            Err(ErrorDominio::RadioInvalido)
        }
    }

    fn instante(segundos: u64) -> SystemTime {
        SystemTime::UNIX_EPOCH + Duration::from_secs(segundos)
    }

    fn punto(latitud: f64, longitud: f64) -> Coordenadas {
        Coordenadas::new(Latitud::new(latitud).unwrap(), Longitud::new(longitud).unwrap())
    }

    fn tramo_vertical(codigo: &str, rio: &str, longitud: f64) -> TramoDeRio {
        let linea = LineaTramo::new(vec![punto(39.80, longitud), punto(39.90, longitud)]).unwrap();
        TramoDeRio::new(codigo.into(), rio.into(), Box::new(linea)).unwrap()
    }

    fn catalogo() -> CatalogoEspecies {
        let mut catalogo = CatalogoEspecies::new();
        catalogo
            .registrar(
                "Procambarus clarkii".into(),
                vec!["Cangrejo rojo americano".into(), "Cangrejo americano".into()],
                true,
                vec![],
            )
            .unwrap();
        catalogo
            .registrar("Neogale vison".into(), vec!["Visón americano".into()], true, vec!["Mustela vison".into()])
            .unwrap();
        catalogo
            .registrar("Micropterus salmoides".into(), vec!["Black bass".into()], true, vec![])
            .unwrap();
        catalogo
            .registrar("Pacifastacus leniusculus".into(), vec!["Cangrejo señal".into()], true, vec![])
            .unwrap();
        catalogo
    }

    fn depredacion(catalogo: &CatalogoEspecies, depredador: &str, presa: &str) -> Depredacion {
        Depredacion::new(
            catalogo,
            depredador,
            presa,
            FaseVital::TODAS.iter().copied().collect::<BTreeSet<_>>(),
            None,
        )
        .unwrap()
    }

    fn red(catalogo: &CatalogoEspecies) -> RedDepredacion {
        let mut red = RedDepredacion::new();
        red.registrar(depredacion(catalogo, "Neogale vison", "Procambarus clarkii")).unwrap();
        red
    }

    fn avistamiento(catalogo: &CatalogoEspecies, id: &str, especie: &str, segundos: u64, longitud: f64) -> Avistamiento {
        Avistamiento::new(
            catalogo,
            Fuente::INaturalist,
            id.into(),
            especie,
            instante(segundos),
            punto(39.85, longitud),
            None,
            EvidenciaFotografica::new(None, vec![]).unwrap(),
        )
        .unwrap()
    }

    fn registro_con(avistamientos: Vec<Avistamiento>) -> RegistroAvistamientos {
        let mut registro = RegistroAvistamientos::new();
        for avistamiento in avistamientos {
            registro.registrar(avistamiento).unwrap();
        }
        registro
    }

    fn rios_de_prueba() -> Rios {
        let mut rios = Rios::new();
        rios.registrar(tramo_vertical("TAJO-01", "Tajo", LONGITUD_AVISTAMIENTO)).unwrap();
        rios.registrar(tramo_vertical("EBRO-01", "Ebro", -0.9)).unwrap();
        rios
    }

    fn consultar(
        catalogo: &CatalogoEspecies,
        red: &RedDepredacion,
        registro: &RegistroAvistamientos,
        rios: &Rios,
        nombre_comun: &str,
        nombre_rio: &str,
        referencia: u64,
    ) -> Result<Vec<ResultadoDepredador>, ErrorDominio> {
        let peticion = PeticionZona {
            nombre_comun: nombre_comun.into(),
            nombre_rio: nombre_rio.into(),
            dias: 15,
            fecha_referencia: instante(referencia),
        };
        let fuentes = FuentesDeVerdad {
            catalogo,
            red_depredacion: red,
            registro_avistamientos: registro,
            rios,
        };
        consultar_zona(&peticion, &fuentes)
    }

    fn escenario_del_vison() -> (CatalogoEspecies, RedDepredacion, RegistroAvistamientos, Rios) {
        let catalogo = catalogo();
        let red = red(&catalogo);
        let registro = registro_con(
            vec![avistamiento(&catalogo, "vison-1", "Neogale vison", AVISTAMIENTO_1_SEP, LONGITUD_AVISTAMIENTO)],
        );
        (catalogo, red, registro, rios_de_prueba())
    }

    fn vison(cantidad: usize) -> ResultadoDepredador {
        ResultadoDepredador {
            nombre_cientifico: "Neogale vison".into(),
            cantidad_avistamientos: cantidad,
            presencia: PresenciaDepredador::desde_cantidad(cantidad),
        }
    }

    #[test]
    fn con_referencia_5_sep_el_vison_tiene_1_avistamiento_y_presencia_detectada() {
        let (catalogo, red, registro, rios) = escenario_del_vison();
        let resultado = consultar(&catalogo, &red, &registro, &rios, "cangrejo", "Tajo", REFERENCIA_5_SEP);
        assert_eq!(resultado, Ok(vec![vison(1)]));
        assert_eq!(resultado.unwrap()[0].presencia, PresenciaDepredador::PresenciaDetectada { cantidad: 1 });
    }

    #[test]
    fn con_referencia_20_sep_el_vison_tiene_0_avistamientos_y_sin_registros_recientes() {
        let (catalogo, red, registro, rios) = escenario_del_vison();
        let resultado = consultar(&catalogo, &red, &registro, &rios, "cangrejo", "Tajo", REFERENCIA_20_SEP);
        assert_eq!(resultado, Ok(vec![vison(0)]));
        assert_eq!(resultado.unwrap()[0].presencia, PresenciaDepredador::SinRegistrosRecientes);
    }

    #[test]
    fn un_rio_sin_avistamientos_da_sin_registros_recientes() {
        let (catalogo, red, registro, rios) = escenario_del_vison();
        let resultado = consultar(&catalogo, &red, &registro, &rios, "cangrejo", "Ebro", REFERENCIA_5_SEP);
        assert_eq!(resultado, Ok(vec![vison(0)]));
    }

    #[test]
    fn un_avistamiento_a_200_metros_del_tramo_cuenta() {
        let catalogo = catalogo();
        let red = red(&catalogo);
        let registro = registro_con(
            vec![avistamiento(&catalogo, "cerca", "Neogale vison", AVISTAMIENTO_1_SEP, LONGITUD_AVISTAMIENTO + 0.00234)],
        );
        let resultado = consultar(&catalogo, &red, &registro, &rios_de_prueba(), "cangrejo", "Tajo", REFERENCIA_5_SEP);
        assert_eq!(resultado, Ok(vec![vison(1)]));
    }

    #[test]
    fn un_avistamiento_a_800_metros_del_tramo_no_cuenta() {
        let catalogo = catalogo();
        let red = red(&catalogo);
        let registro = registro_con(
            vec![avistamiento(&catalogo, "lejos", "Neogale vison", AVISTAMIENTO_1_SEP, LONGITUD_AVISTAMIENTO + 0.00938)],
        );
        let resultado = consultar(&catalogo, &red, &registro, &rios_de_prueba(), "cangrejo", "Tajo", REFERENCIA_5_SEP);
        assert_eq!(resultado, Ok(vec![vison(0)]));
    }

    #[test]
    fn un_avistamiento_de_la_propia_presa_no_sale_en_el_resultado() {
        let catalogo = catalogo();
        let red = red(&catalogo);
        let registro = registro_con(
            vec![avistamiento(&catalogo, "cangrejo-1", "Procambarus clarkii", AVISTAMIENTO_1_SEP, LONGITUD_AVISTAMIENTO)],
        );
        let resultado = consultar(&catalogo, &red, &registro, &rios_de_prueba(), "cangrejo", "Tajo", REFERENCIA_5_SEP).unwrap();
        assert!(resultado.iter().all(|r| r.nombre_cientifico != "Procambarus clarkii"));
        assert_eq!(resultado, vec![vison(0)]);
    }

    #[test]
    fn un_avistamiento_de_una_invasora_que_no_depreda_no_sale() {
        let catalogo = catalogo();
        let red = red(&catalogo);
        let registro = registro_con(
            vec![avistamiento(&catalogo, "bass-1", "Micropterus salmoides", AVISTAMIENTO_1_SEP, LONGITUD_AVISTAMIENTO)],
        );
        let resultado = consultar(&catalogo, &red, &registro, &rios_de_prueba(), "cangrejo", "Tajo", REFERENCIA_5_SEP).unwrap();
        assert_eq!(resultado, vec![vison(0)]);
    }

    #[test]
    fn varios_depredadores_salen_todos_una_vez_y_ordenados_por_nombre_cientifico() {
        let catalogo = catalogo();
        let mut red = red(&catalogo);
        red.registrar(depredacion(&catalogo, "Micropterus salmoides", "Procambarus clarkii")).unwrap();
        let registro = registro_con(
            vec![
                avistamiento(&catalogo, "vison-1", "Neogale vison", AVISTAMIENTO_1_SEP, LONGITUD_AVISTAMIENTO),
                avistamiento(&catalogo, "bass-1", "Micropterus salmoides", AVISTAMIENTO_1_SEP, LONGITUD_AVISTAMIENTO),
            ],
        );
        let resultado = consultar(&catalogo, &red, &registro, &rios_de_prueba(), "cangrejo", "Tajo", REFERENCIA_5_SEP).unwrap();
        let nombres: Vec<&str> = resultado.iter().map(|r| r.nombre_cientifico.as_str()).collect();
        assert_eq!(nombres, vec!["Micropterus salmoides", "Neogale vison"]);
    }

    #[test]
    fn un_depredador_que_come_dos_especies_del_mismo_nombre_comun_sale_una_vez() {
        let catalogo = catalogo();
        let mut red = red(&catalogo);
        red.registrar(depredacion(&catalogo, "Neogale vison", "Pacifastacus leniusculus")).unwrap();
        let registro = registro_con(vec![]);
        let resultado = consultar(&catalogo, &red, &registro, &rios_de_prueba(), "cangrejo", "Tajo", REFERENCIA_5_SEP).unwrap();
        assert_eq!(resultado, vec![vison(0)]);
    }

    #[test]
    fn dos_avistamientos_recientes_dan_presencia_detectada_con_2() {
        let catalogo = catalogo();
        let red = red(&catalogo);
        let registro = registro_con(
            vec![
                avistamiento(&catalogo, "vison-1", "Neogale vison", AVISTAMIENTO_1_SEP, LONGITUD_AVISTAMIENTO),
                avistamiento(&catalogo, "vison-2", "Neogale vison", AVISTAMIENTO_1_SEP + 3600, LONGITUD_AVISTAMIENTO),
            ],
        );
        let resultado = consultar(&catalogo, &red, &registro, &rios_de_prueba(), "cangrejo", "Tajo", REFERENCIA_5_SEP).unwrap();
        assert_eq!(resultado[0].presencia, PresenciaDepredador::PresenciaDetectada { cantidad: 2 });
    }

    #[test]
    fn solo_cuentan_los_avistamientos_dentro_de_la_ventana() {
        let catalogo = catalogo();
        let red = red(&catalogo);
        let registro = registro_con(
            vec![
                avistamiento(&catalogo, "reciente", "Neogale vison", AVISTAMIENTO_1_SEP, LONGITUD_AVISTAMIENTO),
                avistamiento(&catalogo, "antiguo", "Neogale vison", AVISTAMIENTO_1_SEP - 60 * 86_400, LONGITUD_AVISTAMIENTO),
            ],
        );
        let resultado = consultar(&catalogo, &red, &registro, &rios_de_prueba(), "cangrejo", "Tajo", REFERENCIA_5_SEP).unwrap();
        assert_eq!(resultado, vec![vison(1)]);
    }

    #[test]
    fn un_nombre_comun_sin_depredadores_registrados_da_lista_vacia() {
        let catalogo = catalogo();
        let red = red(&catalogo);
        let registro = registro_con(vec![]);
        let resultado = consultar(&catalogo, &red, &registro, &rios_de_prueba(), "black bass", "Tajo", REFERENCIA_5_SEP);
        assert_eq!(resultado, Ok(vec![]));
    }

    #[test]
    fn un_nombre_comun_que_no_existe_da_error_y_no_una_lista_vacia() {
        let (catalogo, red, registro, rios) = escenario_del_vison();
        let resultado = consultar(&catalogo, &red, &registro, &rios, "unicornio", "Tajo", REFERENCIA_5_SEP);
        assert_eq!(resultado, Err(ErrorDominio::NombreComunInexistente));
    }

    #[test]
    fn un_nombre_comun_vacio_da_error() {
        let (catalogo, red, registro, rios) = escenario_del_vison();
        let resultado = consultar(&catalogo, &red, &registro, &rios, "  ", "Tajo", REFERENCIA_5_SEP);
        assert_eq!(resultado, Err(ErrorDominio::NombreComunRequerido));
    }

    #[test]
    fn un_rio_que_no_existe_da_error_y_no_sin_registros() {
        let (catalogo, red, registro, rios) = escenario_del_vison();
        let resultado = consultar(&catalogo, &red, &registro, &rios, "cangrejo", "Guadalquivir", REFERENCIA_5_SEP);
        assert_eq!(resultado, Err(ErrorDominio::RioInexistente));
    }

    #[test]
    fn el_nombre_del_rio_no_distingue_mayusculas_tildes_ni_espacios_de_los_lados() {
        let (catalogo, red, registro, rios) = escenario_del_vison();
        let resultado = consultar(&catalogo, &red, &registro, &rios, "cangrejo", " TAJO ", REFERENCIA_5_SEP);
        assert_eq!(resultado, Ok(vec![vison(1)]));
    }

    #[test]
    fn rio_tajo_no_es_lo_mismo_que_tajo() {
        let (catalogo, red, registro, rios) = escenario_del_vison();
        let resultado = consultar(&catalogo, &red, &registro, &rios, "cangrejo", "Río Tajo", REFERENCIA_5_SEP);
        assert_eq!(resultado, Err(ErrorDominio::RioInexistente));
    }

    #[test]
    fn el_nombre_comun_se_resuelve_por_cualquiera_de_los_nombres_de_la_especie() {
        let (catalogo, red, registro, rios) = escenario_del_vison();
        let resultado = consultar(&catalogo, &red, &registro, &rios, "Cangrejo Rojo", "Tajo", REFERENCIA_5_SEP);
        assert_eq!(resultado, Ok(vec![vison(1)]));
    }

    #[test]
    fn un_error_de_la_geometria_se_propaga_y_no_se_convierte_en_sin_registros() {
        let catalogo = catalogo();
        let red = red(&catalogo);
        let registro = registro_con(
            vec![avistamiento(&catalogo, "vison-1", "Neogale vison", AVISTAMIENTO_1_SEP, LONGITUD_AVISTAMIENTO)],
        );
        let mut rios = Rios::new();
        rios.registrar(TramoDeRio::new("ROTO-01".into(), "Tajo".into(), Box::new(GeometriaQueFalla)).unwrap()).unwrap();
        let resultado = consultar(&catalogo, &red, &registro, &rios, "cangrejo", "Tajo", REFERENCIA_5_SEP);
        assert_eq!(resultado, Err(ErrorDominio::RadioInvalido));
    }
}
