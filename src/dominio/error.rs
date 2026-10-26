use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErrorDominio {
    CoordenadasInvalidas,
    CodigoTramoRequerido,
    NombreRioRequerido,
    RioInexistente,
    TramoRepetido,
    RadioInvalido,
    TramoInsuficiente,
    NombreCientificoRequerido,
    NombreComunRequerido,
    NombreComunInexistente,
    NombreEspecieRepetido,
    EspecieDesconocida,
    EspecieNoInvasora,
    AvistamientoIncompleto,
    AvistamientoRepetido,
    UrlFotoInvalida,
    DepredacionSobreMismaEspecie,
    FaseVitalRequerida,
    DepredacionRepetida,
}

impl fmt::Display for ErrorDominio {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mensaje = match self {
            Self::CoordenadasInvalidas => "coordenadas inválidas",
            Self::CodigoTramoRequerido => "el tramo de río necesita un nombre o código",
            Self::NombreRioRequerido => "el tramo de río necesita el nombre del río al que pertenece",
            Self::RioInexistente => "el río no está registrado",
            Self::TramoRepetido => "el tramo de río ya está registrado",
            Self::RadioInvalido => "el radio no puede ser negativo ni infinito",
            Self::TramoInsuficiente => "el tramo de río necesita al menos 2 puntos",
            Self::NombreCientificoRequerido => "la especie necesita su nombre científico",
            Self::NombreComunRequerido => "hace falta un nombre común que no esté vacío",
            Self::NombreComunInexistente => "el nombre común no corresponde a ninguna especie del catálogo",
            Self::NombreEspecieRepetido => "el nombre ya está registrado en el catálogo",
            Self::EspecieDesconocida => "la especie no está en el catálogo",
            Self::EspecieNoInvasora => "la especie no es invasora",
            Self::AvistamientoIncompleto => "el avistamiento necesita su identificador original",
            Self::AvistamientoRepetido => "el avistamiento ya está registrado",
            Self::UrlFotoInvalida => "URL de foto inválida",
            Self::DepredacionSobreMismaEspecie => "el depredador y la presa no pueden ser la misma especie",
            Self::FaseVitalRequerida => "la depredación debe afectar al menos a una fase vital",
            Self::DepredacionRepetida => "la misma depredación entre dos especies está repetida",
        };
        f.write_str(mensaje)
    }
}

impl Error for ErrorDominio {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    const TODOS: [ErrorDominio; 19] = [
        ErrorDominio::CoordenadasInvalidas,
        ErrorDominio::CodigoTramoRequerido,
        ErrorDominio::NombreRioRequerido,
        ErrorDominio::RioInexistente,
        ErrorDominio::TramoRepetido,
        ErrorDominio::RadioInvalido,
        ErrorDominio::TramoInsuficiente,
        ErrorDominio::NombreCientificoRequerido,
        ErrorDominio::NombreComunRequerido,
        ErrorDominio::NombreComunInexistente,
        ErrorDominio::NombreEspecieRepetido,
        ErrorDominio::EspecieDesconocida,
        ErrorDominio::EspecieNoInvasora,
        ErrorDominio::AvistamientoIncompleto,
        ErrorDominio::AvistamientoRepetido,
        ErrorDominio::UrlFotoInvalida,
        ErrorDominio::DepredacionSobreMismaEspecie,
        ErrorDominio::FaseVitalRequerida,
        ErrorDominio::DepredacionRepetida,
    ];

    #[test]
    fn todos_los_errores_tienen_mensaje() {
        for error in TODOS {
            assert!(!error.to_string().trim().is_empty(), "{error:?} no tiene mensaje");
        }
    }

    #[test]
    fn cada_error_tiene_un_mensaje_distinto() {
        let mensajes: HashSet<String> = TODOS.iter().map(ToString::to_string).collect();
        assert_eq!(mensajes.len(), TODOS.len());
    }

    #[test]
    fn rio_inexistente_y_nombre_comun_inexistente_dicen_que_no_existen() {
        assert_eq!(ErrorDominio::RioInexistente.to_string(), "el río no está registrado");
        assert_eq!(
            ErrorDominio::NombreComunInexistente.to_string(),
            "el nombre común no corresponde a ninguna especie del catálogo"
        );
    }

    #[test]
    fn el_error_de_dominio_se_puede_usar_como_error_estandar() {
        let error: Box<dyn Error> = Box::new(ErrorDominio::RioInexistente);
        assert_eq!(error.to_string(), "el río no está registrado");
    }
}
