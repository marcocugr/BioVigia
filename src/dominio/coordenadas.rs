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

    pub fn grados(&self) -> f64 {
        self.0
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

    pub fn grados(&self) -> f64 {
        self.0
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

    pub fn latitud(&self) -> &Latitud {
        &self.latitud
    }

    pub fn longitud(&self) -> &Longitud {
        &self.longitud
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn la_latitud_acepta_de_menos_90_a_90() {
        for grados in [-90.0, -45.5, 0.0, 39.85, 90.0] {
            assert_eq!(Latitud::new(grados).unwrap().grados(), grados);
        }
    }

    #[test]
    fn la_latitud_fuera_de_rango_o_no_numerica_da_error() {
        for grados in [90.0001, -90.0001, 200.0, f64::NAN, f64::INFINITY] {
            assert_eq!(Latitud::new(grados), Err(ErrorDominio::CoordenadasInvalidas));
        }
    }

    #[test]
    fn la_longitud_acepta_de_menos_180_a_180() {
        for grados in [-180.0, -4.55, 0.0, 120.0, 180.0] {
            assert_eq!(Longitud::new(grados).unwrap().grados(), grados);
        }
    }

    #[test]
    fn la_longitud_fuera_de_rango_o_no_numerica_da_error() {
        for grados in [180.0001, -180.0001, 400.0, f64::NAN, f64::NEG_INFINITY] {
            assert_eq!(Longitud::new(grados), Err(ErrorDominio::CoordenadasInvalidas));
        }
    }

    #[test]
    fn los_hemisferios_se_leen_de_una_letra_en_mayuscula_o_minuscula() {
        assert_eq!(HemisferioNS::try_from('N'), Ok(HemisferioNS::Norte));
        assert_eq!(HemisferioNS::try_from('s'), Ok(HemisferioNS::Sur));
        assert_eq!(HemisferioEW::try_from('E'), Ok(HemisferioEW::Este));
        assert_eq!(HemisferioEW::try_from('w'), Ok(HemisferioEW::Oeste));
    }

    #[test]
    fn una_letra_de_hemisferio_que_no_existe_da_error() {
        assert_eq!(HemisferioNS::try_from('X'), Err(ErrorDominio::CoordenadasInvalidas));
        assert_eq!(HemisferioNS::try_from('E'), Err(ErrorDominio::CoordenadasInvalidas));
        assert_eq!(HemisferioEW::try_from('N'), Err(ErrorDominio::CoordenadasInvalidas));
    }

    #[test]
    fn con_hemisferio_el_sur_y_el_oeste_dan_grados_negativos() {
        let coordenadas = Coordenadas::con_hemisferio(33.9, HemisferioNS::Sur, 18.4, HemisferioEW::Oeste).unwrap();
        assert_eq!(coordenadas.latitud().grados(), -33.9);
        assert_eq!(coordenadas.longitud().grados(), -18.4);
    }

    #[test]
    fn con_hemisferio_el_norte_y_el_este_dan_grados_positivos() {
        let coordenadas = Coordenadas::con_hemisferio(40.4, HemisferioNS::Norte, 3.7, HemisferioEW::Este).unwrap();
        assert_eq!(coordenadas.latitud().grados(), 40.4);
        assert_eq!(coordenadas.longitud().grados(), 3.7);
    }

    #[test]
    fn con_hemisferio_rechaza_grados_negativos_o_fuera_de_rango() {
        assert_eq!(
            Coordenadas::con_hemisferio(-10.0, HemisferioNS::Norte, 3.7, HemisferioEW::Este),
            Err(ErrorDominio::CoordenadasInvalidas)
        );
        assert_eq!(
            Coordenadas::con_hemisferio(91.0, HemisferioNS::Norte, 3.7, HemisferioEW::Este),
            Err(ErrorDominio::CoordenadasInvalidas)
        );
        assert_eq!(
            Coordenadas::con_hemisferio(40.0, HemisferioNS::Norte, 181.0, HemisferioEW::Oeste),
            Err(ErrorDominio::CoordenadasInvalidas)
        );
    }

    #[test]
    fn unas_coordenadas_guardan_su_latitud_y_su_longitud() {
        let coordenadas = Coordenadas::new(Latitud::new(39.85).unwrap(), Longitud::new(-4.55).unwrap());
        assert_eq!(coordenadas.latitud().grados(), 39.85);
        assert_eq!(coordenadas.longitud().grados(), -4.55);
    }
}
