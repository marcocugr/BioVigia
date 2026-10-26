pub mod gbif;
pub mod gisd;
pub mod globi;
mod html;
pub mod inaturalist;

use std::error::Error;
use std::fmt;

use crate::dominio::error::ErrorDominio;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErrorFuente {
    CampoNoEncontrado(&'static str),
    ValorInvalido(&'static str),
    Dominio(ErrorDominio),
}

impl fmt::Display for ErrorFuente {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CampoNoEncontrado(campo) => write!(f, "no se ha encontrado {campo} en el HTML"),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn los_errores_de_fuente_dicen_que_campo_falla() {
        assert_eq!(
            ErrorFuente::CampoNoEncontrado("la fecha").to_string(),
            "no se ha encontrado la fecha en el HTML"
        );
        assert_eq!(
            ErrorFuente::ValorInvalido("la fecha").to_string(),
            "el valor de la fecha no es válido"
        );
    }

    #[test]
    fn un_error_de_dominio_se_envuelve_y_conserva_su_mensaje() {
        let error: ErrorFuente = ErrorDominio::UrlFotoInvalida.into();
        assert_eq!(error, ErrorFuente::Dominio(ErrorDominio::UrlFotoInvalida));
        assert_eq!(error.to_string(), ErrorDominio::UrlFotoInvalida.to_string());
    }
}
