use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErrorDominio {
    CoordenadasInvalidas,
    CodigoTramoRequerido,
    NombreCientificoRequerido,
    NombreComunRequerido,
    EspecieDesconocida,
    EspecieNoInvasora,
    AvistamientoIncompleto,
    DepredacionSobreMismaEspecie,
    FaseVitalRequerida,
}

impl fmt::Display for ErrorDominio {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mensaje = match self {
            Self::CoordenadasInvalidas => "coordenadas inválidas",
            Self::CodigoTramoRequerido => "el tramo de río necesita un nombre o código",
            Self::NombreCientificoRequerido => "la especie necesita su nombre científico",
            Self::NombreComunRequerido => "la especie necesita un nombre común",
            Self::EspecieDesconocida => "la especie no está en el catálogo",
            Self::EspecieNoInvasora => "la especie no es invasora",
            Self::AvistamientoIncompleto => "el avistamiento necesita su identificador original",
            Self::DepredacionSobreMismaEspecie => "el depredador y la presa no pueden ser la misma especie",
            Self::FaseVitalRequerida => "la depredación debe afectar al menos a una fase vital",
        };
        f.write_str(mensaje)
    }
}

impl Error for ErrorDominio {}
