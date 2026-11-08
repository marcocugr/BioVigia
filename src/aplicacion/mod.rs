pub mod consulta;
pub mod inaturalist;
pub mod nomenclator;

use std::error::Error;
use std::fmt;

use crate::dominio::avistamiento::{Avistamiento, RegistroAvistamientos};
use crate::dominio::error::ErrorDominio;
use crate::dominio::especie::CatalogoEspecies;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErrorFuente {
    CampoNoEncontrado(&'static str),
    ValorInvalido(&'static str),
    Dominio(ErrorDominio),
}

impl fmt::Display for ErrorFuente {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CampoNoEncontrado(campo) => write!(f, "no se ha encontrado {campo} en la página"),
            Self::ValorInvalido(campo) => write!(f, "el valor de {campo} no es válido"),
            Self::Dominio(error) => write!(f, "{error}"),
        }
    }
}

impl Error for ErrorFuente {}

impl From<ErrorDominio> for ErrorFuente {
    fn from(error: ErrorDominio) -> Self {
        Self::Dominio(error)
    }
}

pub trait FuenteAvistamientos {
    fn leer(&self, pagina: &str, catalogo: &CatalogoEspecies) -> Result<Avistamiento, ErrorFuente>;
}

pub fn reunir(
    registro: &mut RegistroAvistamientos,
    catalogo: &CatalogoEspecies,
    fuente: &dyn FuenteAvistamientos,
    pagina: &str,
) -> Result<(), ErrorFuente> {
    registro.registrar(fuente.leer(pagina, catalogo)?)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dominio::avistamiento::Fuente;
    use crate::dominio::coordenadas::{Coordenadas, Latitud, Longitud};
    use crate::dominio::tramo::{Recuadro, TramoDeRio};
    use std::time::{Duration, SystemTime};

    const DIA: u64 = 86_400;

    struct FuenteDePrueba(Fuente);

    impl FuenteAvistamientos for FuenteDePrueba {
        fn leer(
            &self,
            pagina: &str,
            catalogo: &CatalogoEspecies,
        ) -> Result<Avistamiento, ErrorFuente> {
            let coordenadas = Coordenadas::new(Latitud::new(39.85)?, Longitud::new(-4.55)?);
            Ok(Avistamiento::new(
                catalogo,
                self.0,
                pagina.into(),
                "Neogale vison",
                SystemTime::UNIX_EPOCH + Duration::from_secs(DIA),
                coordenadas,
            )?)
        }
    }

    fn catalogo() -> CatalogoEspecies {
        let mut catalogo = CatalogoEspecies::new();
        catalogo
            .registrar(
                "Neogale vison".into(),
                "Visón americano".into(),
                true,
                vec![],
            )
            .unwrap();
        catalogo
    }

    #[test]
    fn los_avistamientos_de_distintas_fuentes_acaban_en_el_mismo_registro() {
        let catalogo = catalogo();
        let mut registro = RegistroAvistamientos::new();
        reunir(
            &mut registro,
            &catalogo,
            &FuenteDePrueba(Fuente::INaturalist),
            "1",
        )
        .unwrap();
        reunir(&mut registro, &catalogo, &FuenteDePrueba(Fuente::Gbif), "1").unwrap();
        let punto = |latitud, longitud| {
            Coordenadas::new(
                Latitud::new(latitud).unwrap(),
                Longitud::new(longitud).unwrap(),
            )
        };
        let recuadro = Recuadro::new(punto(39.8, -4.6), punto(39.9, -4.5));
        let zona = TramoDeRio::new("TAJO-01".into(), Box::new(recuadro)).unwrap();
        let referencia = SystemTime::UNIX_EPOCH + Duration::from_secs(DIA);
        assert_eq!(
            registro
                .que_cuentan("Neogale vison", &zona, referencia, 0)
                .len(),
            2
        );
    }

    #[test]
    fn el_mismo_avistamiento_leido_dos_veces_de_la_misma_fuente_da_error() {
        let catalogo = catalogo();
        let mut registro = RegistroAvistamientos::new();
        reunir(
            &mut registro,
            &catalogo,
            &FuenteDePrueba(Fuente::INaturalist),
            "1",
        )
        .unwrap();
        let repetido = reunir(
            &mut registro,
            &catalogo,
            &FuenteDePrueba(Fuente::INaturalist),
            "1",
        );
        assert_eq!(
            repetido,
            Err(ErrorFuente::Dominio(ErrorDominio::AvistamientoRepetido))
        );
    }
}
