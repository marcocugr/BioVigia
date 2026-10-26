pub(crate) fn normalizar(texto: &str) -> String {
    texto
        .trim()
        .chars()
        .flat_map(char::to_lowercase)
        .map(quitar_tilde)
        .collect()
}

fn quitar_tilde(letra: char) -> char {
    match letra {
        'á' | 'à' | 'ä' | 'â' => 'a',
        'é' | 'è' | 'ë' | 'ê' => 'e',
        'í' | 'ì' | 'ï' | 'î' => 'i',
        'ó' | 'ò' | 'ö' | 'ô' => 'o',
        'ú' | 'ù' | 'ü' | 'û' => 'u',
        otra => otra,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pasa_a_minusculas_y_recorta_los_espacios_de_los_lados() {
        assert_eq!(normalizar("  TAJO "), "tajo");
    }

    #[test]
    fn quita_las_tildes_de_las_vocales() {
        assert_eq!(normalizar("Río Tajo"), "rio tajo");
        assert_eq!(normalizar("Visón Americano"), "vison americano");
        assert_eq!(normalizar("pingüino"), "pinguino");
    }

    #[test]
    fn la_enie_mayuscula_y_minuscula_dan_lo_mismo() {
        assert_eq!(normalizar("Ñandú"), "ñandu");
        assert_eq!(normalizar("ÑANDÚ"), normalizar("ñandú"));
    }

    #[test]
    fn la_enie_no_se_confunde_con_la_n() {
        assert_ne!(normalizar("año"), normalizar("ano"));
    }

    #[test]
    fn rio_tajo_y_tajo_siguen_siendo_distintos() {
        assert_ne!(normalizar("Río Tajo"), normalizar("Tajo"));
    }

    #[test]
    fn una_cadena_vacia_o_de_espacios_queda_vacia() {
        assert_eq!(normalizar(""), "");
        assert_eq!(normalizar("   "), "");
    }
}
