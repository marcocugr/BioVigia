use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErrorDominio {
    CoordenadasInvalidas,
    CodigoTramoRequerido,
    NombreCientificoRequerido,
    NombreComunRequerido,
    NombreComunInexistente,
    NombreEspecieRepetido,
    EspecieDesconocida,
    EspecieNoInvasora,
    AvistamientoIncompleto,
    AvistamientoRepetido,
    DepredacionSobreMismaEspecie,
    FaseVitalRequerida,
    DepredacionRepetida,
}

impl fmt::Display for ErrorDominio {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mensaje = match self {
            Self::CoordenadasInvalidas => "coordenadas inválidas",
            Self::CodigoTramoRequerido => "el tramo de río necesita un nombre o código",
            Self::NombreCientificoRequerido => "la especie necesita su nombre científico",
            Self::NombreComunRequerido => "hace falta un nombre común que no esté vacío",
            Self::NombreComunInexistente => "el nombre común no corresponde a ninguna especie del catálogo",
            Self::NombreEspecieRepetido => "el nombre ya está registrado en el catálogo",
            Self::EspecieDesconocida => "la especie no está en el catálogo",
            Self::EspecieNoInvasora => "la especie no es invasora",
            Self::AvistamientoIncompleto => "el avistamiento necesita su identificador original",
            Self::AvistamientoRepetido => "el avistamiento ya está registrado",
            Self::DepredacionSobreMismaEspecie => "el depredador y la presa no pueden ser la misma especie",
            Self::FaseVitalRequerida => "la depredación debe afectar al menos a una fase vital",
            Self::DepredacionRepetida => "la misma depredación entre dos especies está repetida",
        };
        f.write_str(mensaje)
    }
}

impl Error for ErrorDominio {}
