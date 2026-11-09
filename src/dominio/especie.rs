use crate::dominio::error::ErrorDominio;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Especie {
    nombre_cientifico: String,
    nombre_comun: String,
    invasora: bool,
    nombres_anteriores: Vec<String>,
}

impl Especie {
    pub fn new(
        nombre_cientifico: String,
        nombre_comun: String,
        invasora: bool,
        nombres_anteriores: Vec<String>,
    ) -> Result<Self, ErrorDominio> {
        let nombre_cientifico = nombre_cientifico.trim().to_owned();
        let nombre_comun = nombre_comun.trim().to_owned();
        let nombres_anteriores: Vec<String> = nombres_anteriores
            .iter()
            .map(|n| n.trim().to_owned())
            .collect();
        if nombre_cientifico.is_empty() || nombres_anteriores.iter().any(|n| n.is_empty()) {
            return Err(ErrorDominio::NombreCientificoRequerido);
        }
        if nombre_comun.is_empty() {
            return Err(ErrorDominio::NombreComunRequerido);
        }
        Ok(Self {
            nombre_cientifico,
            nombre_comun,
            invasora,
            nombres_anteriores,
        })
    }
}

#[derive(Debug, Clone)]
pub struct CatalogoEspecies {
    especies: Vec<Especie>,
}

impl CatalogoEspecies {
    pub fn new(especies: Vec<Especie>) -> Self {
        Self { especies }
    }

    fn buscar(&self, nombre: &str) -> Result<&Especie, ErrorDominio> {
        let nombre = nombre.trim();
        self.especies
            .iter()
            .find(|especie| {
                especie.nombre_cientifico == nombre
                    || especie
                        .nombres_anteriores
                        .iter()
                        .any(|anterior| anterior == nombre)
            })
            .ok_or(ErrorDominio::EspecieDesconocida)
    }

    pub(crate) fn especie(&self, nombre: &str) -> Result<&str, ErrorDominio> {
        Ok(&self.buscar(nombre)?.nombre_cientifico)
    }

    pub(crate) fn especie_invasora(&self, nombre: &str) -> Result<&str, ErrorDominio> {
        let especie = self.buscar(nombre)?;
        if especie.invasora {
            Ok(&especie.nombre_cientifico)
        } else {
            Err(ErrorDominio::EspecieNoInvasora)
        }
    }

    pub fn especie_con_nombre_comun(&self, nombre_comun: &str) -> Result<&str, ErrorDominio> {
        let buscado = clave(nombre_comun);
        self.especies
            .iter()
            .find(|especie| clave(&especie.nombre_comun) == buscado)
            .map(|especie| especie.nombre_cientifico.as_str())
            .ok_or(ErrorDominio::EspecieDesconocida)
    }
}

fn clave(nombre: &str) -> String {
    nombre
        .trim()
        .chars()
        .flat_map(char::to_lowercase)
        .map(sin_tilde)
        .collect()
}

fn sin_tilde(letra: char) -> char {
    match letra {
        'á' => 'a',
        'é' => 'e',
        'í' => 'i',
        'ó' => 'o',
        'ú' | 'ü' => 'u',
        otra => otra,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn catalogo() -> CatalogoEspecies {
        let cangrejo = Especie::new(
            "Procambarus clarkii".into(),
            "Cangrejo rojo americano".into(),
            false,
            vec![],
        );
        let vison = Especie::new(
            "Neogale vison".into(),
            "Visón americano".into(),
            true,
            vec!["Mustela vison".into()],
        );
        CatalogoEspecies::new(vec![cangrejo.unwrap(), vison.unwrap()])
    }

    #[test]
    fn el_pescador_encuentra_la_especie_sin_mayusculas_tildes_ni_espacios_de_los_lados() {
        let catalogo = catalogo();
        assert_eq!(
            catalogo.especie_con_nombre_comun("Cangrejo rojo americano"),
            Ok("Procambarus clarkii")
        );
        assert_eq!(
            catalogo.especie_con_nombre_comun("  CANGREJO ROJO AMERICANO "),
            Ok("Procambarus clarkii")
        );
        assert_eq!(
            catalogo.especie_con_nombre_comun("vison americano"),
            Ok("Neogale vison")
        );
    }

    #[test]
    fn un_nombre_comun_que_no_es_de_ninguna_especie_da_error_y_no_una_lista_vacia() {
        assert_eq!(
            catalogo().especie_con_nombre_comun("unicornio"),
            Err(ErrorDominio::EspecieDesconocida)
        );
    }
}
