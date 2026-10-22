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
            Self::RadioInvalido => "el radio debe ser un número positivo",
            Self::TramoInsuficiente => "el tramo de río necesita al menos 2 puntos",
            Self::NombreCientificoRequerido => "la especie necesita su nombre científico",
            Self::NombreComunRequerido => "la especie necesita un nombre común",
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
