use std::collections::BTreeSet;
use std::fs;
use biovigia::aplicacion::extractor_inaturalist::extraer_avistamiento_inaturalist;
use biovigia::dominio::avistamiento::RegistroAvistamientos;
use biovigia::dominio::coordenadas::{Coordenadas, Latitud, Longitud};
use biovigia::dominio::depredacion::{FaseVital, RedDepredacion, Depredacion};
use biovigia::dominio::error::ErrorDominio;
use biovigia::dominio::especie::CatalogoEspecies;
use biovigia::dominio::tramo::{TramoDeRio, GeometriaTramo};

pub fn get_especie_from_csv(index: usize) -> Result<(String, String, bool, Vec<String>), ErrorDominio> {
    let contenido = fs::read_to_string("tests/fixtures/catalogo.csv").map_err(|_| ErrorDominio::AvistamientoIncompleto)?;
    
    let linea = contenido.lines().skip(1).nth(index).ok_or(ErrorDominio::EspecieDesconocida)?;
    let partes: Vec<&str> = linea.split(',').collect();
    if partes.len() < 4 {
        return Err(ErrorDominio::EspecieDesconocida);
    }
    
    let nombre_cientifico = partes[0].to_string();
    let nombre_comun = partes[1].to_string();
    let invasora = partes[2] == "true";
    let nombres_anteriores = if partes[3].is_empty() {
        vec![]
    } else {
        partes[3].split(';').map(|s| s.to_string()).collect()
    };
    
    Ok((nombre_cientifico, nombre_comun, invasora, nombres_anteriores))
}

pub fn get_depredacion_from_csv(index: usize) -> Result<(String, String, BTreeSet<FaseVital>), ErrorDominio> {
    let contenido = fs::read_to_string("tests/fixtures/red.csv").map_err(|_| ErrorDominio::AvistamientoIncompleto)?;
    
    let linea = contenido.lines().skip(1).nth(index).ok_or(ErrorDominio::DepredacionRepetida)?;
    let partes: Vec<&str> = linea.split(',').collect();
    if partes.len() < 4 {
        return Err(ErrorDominio::DepredacionRepetida);
    }
    
    let depredador = partes[0].to_string();
    let presa = partes[1].to_string();
    let fases: BTreeSet<FaseVital> = partes[2].split(';').filter_map(|fase| {
        match fase {
            "HuevoEmbrion" => Some(FaseVital::HuevoEmbrion),
            "LarvaAlevin" => Some(FaseVital::LarvaAlevin),
            "Juvenil" => Some(FaseVital::Juvenil),
            "Adulto" => Some(FaseVital::Adulto),
            _ => None,
        }
    }).collect();
    
    Ok((depredador, presa, fases))
}

pub fn get_tramo_from_csv(index: usize) -> Result<TramoDeRio, ErrorDominio> {
    let contenido = fs::read_to_string("tests/fixtures/tramo.csv").map_err(|_| ErrorDominio::CodigoTramoRequerido)?;
    
    let linea = contenido.lines().skip(1).nth(index).ok_or(ErrorDominio::CodigoTramoRequerido)?;
    let partes: Vec<&str> = linea.split(',').collect();
    if partes.len() < 9 {
        return Err(ErrorDominio::CodigoTramoRequerido);
    }
    
    let codigo = partes[0].to_string();
    let geometria = GeometriaTramo {
        noroeste: Coordenadas::new(
            Latitud::new(partes[1].parse().map_err(|_| ErrorDominio::CoordenadasInvalidas)?)?,
            Longitud::new(partes[2].parse().map_err(|_| ErrorDominio::CoordenadasInvalidas)?)?
        ),
        noreste: Coordenadas::new(
            Latitud::new(partes[3].parse().map_err(|_| ErrorDominio::CoordenadasInvalidas)?)?,
            Longitud::new(partes[4].parse().map_err(|_| ErrorDominio::CoordenadasInvalidas)?)?
        ),
        suroeste: Coordenadas::new(
            Latitud::new(partes[5].parse().map_err(|_| ErrorDominio::CoordenadasInvalidas)?)?,
            Longitud::new(partes[6].parse().map_err(|_| ErrorDominio::CoordenadasInvalidas)?)?
        ),
        sureste: Coordenadas::new(
            Latitud::new(partes[7].parse().map_err(|_| ErrorDominio::CoordenadasInvalidas)?)?,
            Longitud::new(partes[8].parse().map_err(|_| ErrorDominio::CoordenadasInvalidas)?)?
        ),
    };
    
    TramoDeRio::new(codigo, geometria)
}

pub fn setup(catalogo: &mut CatalogoEspecies, red: &mut RedDepredacion, registro: &mut RegistroAvistamientos, tramo: &mut TramoDeRio) -> Result<(), ErrorDominio> {
    catalogo.registrar("Neogale vison".to_string(), "Visón americano".to_string(), true, vec![])?;
    catalogo.registrar("Procambarus clarkii".to_string(), "Cangrejo rojo americano".to_string(), true, vec![])?;
    catalogo.registrar("Salmo trutta".to_string(), "Trucha común".to_string(), false, vec![])?;
    red.registrar(Depredacion::new(catalogo, "Neogale vison", "Salmo trutta", BTreeSet::from([FaseVital::Adulto, FaseVital::Juvenil]), None)?)?;
    red.registrar(Depredacion::new(catalogo, "Neogale vison", "Procambarus clarkii", BTreeSet::from([FaseVital::Adulto]), None)?)?;
    red.registrar(Depredacion::new(catalogo, "Procambarus clarkii", "Salmo trutta", BTreeSet::from([FaseVital::HuevoEmbrion]), None)?)?;
    registro.registrar(extraer_avistamiento_inaturalist("tests/fixtures/inaturalist/vison.html")?)?;
    *tramo = TramoDeRio::new("TR-001".to_string(), GeometriaTramo {
        noroeste: Coordenadas::new(Latitud::new(40.05)?, Longitud::new(-4.65)?),
        noreste: Coordenadas::new(Latitud::new(40.05)?, Longitud::new(-4.64)?),
        suroeste: Coordenadas::new(Latitud::new(40.03)?, Longitud::new(-4.65)?),
        sureste: Coordenadas::new(Latitud::new(40.03)?, Longitud::new(-4.64)?),
    })?;
    Ok(())
}
