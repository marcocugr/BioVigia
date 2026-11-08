use crate::dominio::error::ErrorDominio;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HemisferioNS {
    Norte,
    Sur,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HemisferioEW {
    Este,
    Oeste,
}

impl TryFrom<char> for HemisferioNS {
    type Error = ErrorDominio;

    fn try_from(letra: char) -> Result<Self, Self::Error> {
        match letra.to_ascii_uppercase() {
            'N' => Ok(Self::Norte),
            'S' => Ok(Self::Sur),
            _ => Err(ErrorDominio::CoordenadasInvalidas),
        }
    }
}

impl TryFrom<char> for HemisferioEW {
    type Error = ErrorDominio;

    fn try_from(letra: char) -> Result<Self, Self::Error> {
        match letra.to_ascii_uppercase() {
            'E' => Ok(Self::Este),
            'W' => Ok(Self::Oeste),
            _ => Err(ErrorDominio::CoordenadasInvalidas),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Latitud(f64);

impl Latitud {
    pub fn new(grados: f64) -> Result<Self, ErrorDominio> {
        if (-90.0..=90.0).contains(&grados) {
            Ok(Self(grados))
        } else {
            Err(ErrorDominio::CoordenadasInvalidas)
        }
    }

    fn con_hemisferio(grados: f64, hemisferio: HemisferioNS) -> Result<Self, ErrorDominio> {
        if !(0.0..=90.0).contains(&grados) {
            return Err(ErrorDominio::CoordenadasInvalidas);
        }
        match hemisferio {
            HemisferioNS::Norte => Self::new(grados),
            HemisferioNS::Sur => Self::new(-grados),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Longitud(f64);

impl Longitud {
    pub fn new(grados: f64) -> Result<Self, ErrorDominio> {
        if (-180.0..=180.0).contains(&grados) {
            Ok(Self(grados))
        } else {
            Err(ErrorDominio::CoordenadasInvalidas)
        }
    }

    fn con_hemisferio(grados: f64, hemisferio: HemisferioEW) -> Result<Self, ErrorDominio> {
        if !(0.0..=180.0).contains(&grados) {
            return Err(ErrorDominio::CoordenadasInvalidas);
        }
        match hemisferio {
            HemisferioEW::Este => Self::new(grados),
            HemisferioEW::Oeste => Self::new(-grados),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Coordenadas {
    latitud: Latitud,
    longitud: Longitud,
}

impl Coordenadas {
    pub fn new(latitud: Latitud, longitud: Longitud) -> Self {
        Self { latitud, longitud }
    }

    pub fn con_hemisferio(
        grados_latitud: f64,
        hemisferio_ns: HemisferioNS,
        grados_longitud: f64,
        hemisferio_ew: HemisferioEW,
    ) -> Result<Self, ErrorDominio> {
        Ok(Self::new(
            Latitud::con_hemisferio(grados_latitud, hemisferio_ns)?,
            Longitud::con_hemisferio(grados_longitud, hemisferio_ew)?,
        ))
    }

    pub fn entre(&self, una_esquina: &Coordenadas, otra_esquina: &Coordenadas) -> bool {
        let (sur, norte) = ordenados(una_esquina.latitud.0, otra_esquina.latitud.0);
        let (oeste, este) = ordenados(una_esquina.longitud.0, otra_esquina.longitud.0);
        (sur..=norte).contains(&self.latitud.0) && (oeste..=este).contains(&self.longitud.0)
    }
}

fn ordenados(a: f64, b: f64) -> (f64, f64) {
    if a <= b { (a, b) } else { (b, a) }
}
