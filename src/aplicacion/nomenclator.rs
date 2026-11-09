use crate::dominio::depredacion::{Depredacion, FaseVital, RedDepredacion};
use crate::dominio::error::ErrorDominio;
use crate::dominio::especie::{CatalogoEspecies, Especie};

const ESPECIES: [(&str, &str, bool, &[&str]); 2] = [
    (
        "Neogale vison",
        "Visón americano",
        true,
        &["Mustela vison", "Neovison vison", "Putorius vison"],
    ),
    ("Procambarus clarkii", "Cangrejo rojo americano", false, &[]),
];

const DEPREDACIONES: [(&str, &str); 1] = [("Neogale vison", "Procambarus clarkii")];

pub fn cargar() -> Result<(CatalogoEspecies, RedDepredacion), ErrorDominio> {
    let mut especies = Vec::new();
    for (cientifico, comun, invasora, anteriores) in ESPECIES {
        let anteriores = anteriores.iter().map(|nombre| nombre.to_string()).collect();
        especies.push(Especie::new(
            cientifico.into(),
            comun.into(),
            invasora,
            anteriores,
        )?);
    }
    let catalogo = CatalogoEspecies::new(especies);
    let mut depredaciones = Vec::new();
    for (depredador, presa) in DEPREDACIONES {
        let fases = FaseVital::TODAS.into_iter().collect();
        depredaciones.push(Depredacion::new(&catalogo, depredador, presa, fases, None)?);
    }
    Ok((catalogo, RedDepredacion::new(depredaciones)))
}
