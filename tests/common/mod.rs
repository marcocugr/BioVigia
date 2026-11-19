use std::collections::BTreeSet;
use biovigia::aplicacion::extractor_inaturalist::extraer_avistamiento_inaturalist;
use biovigia::dominio::avistamiento::RegistroAvistamientos;
use biovigia::dominio::coordenadas::{Coordenadas, Latitud, Longitud};
use biovigia::dominio::depredacion::{FaseVital, RedDepredacion, Depredacion};
use biovigia::dominio::error::ErrorDominio;
use biovigia::dominio::especie::CatalogoEspecies;
use biovigia::dominio::tramo::{TramoDeRio, GeometriaTramo};

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
