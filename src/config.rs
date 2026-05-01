use ratatui::style::Color;
use serde::Deserialize;
use std::fs;
use std::str::FromStr;

#[derive(Deserialize, Default)]
struct ConfigToml {
    #[serde(default)]
    tema: TemaToml,
}

#[derive(Deserialize, Default)]
struct TemaToml {
    errore: Option<String>,
    evidenziato: Option<String>,
    bordo_normale: Option<String>,
    bordo_attivo: Option<String>,
    testo: Option<String>,
}

#[derive(Clone, Debug)]
pub struct TemaApp {
    pub errore: Color,
    pub evidenziato: Color,
    pub bordo_normale: Color,
    pub bordo_attivo: Color,
    pub testo: Color,
}

impl TemaApp {
    pub fn carica(percorso: &str) -> Self {
        let default = TemaApp {
            errore: Color::Red,
            evidenziato: Color::Yellow,
            bordo_normale: Color::Reset,
            bordo_attivo: Color::Cyan,
            testo: Color::White,
        };

        let contenuto = match fs::read_to_string(percorso) {
            Ok(c) => c,
            Err(_) => return default,
        };

        let config_toml = match toml::from_str::<ConfigToml>(&contenuto) {
            Ok(c) => c,
            Err(_) => return default,
        };

        let estrai_colore = |campo: Option<String>, fallback: Color| -> Color {
            campo
                .and_then(|s| Color::from_str(&s).ok())
                .unwrap_or(fallback)
        };

        TemaApp {
            errore: estrai_colore(config_toml.tema.errore, default.errore),
            evidenziato: estrai_colore(config_toml.tema.evidenziato, default.evidenziato),
            bordo_normale: estrai_colore(config_toml.tema.bordo_normale, default.bordo_normale),
            bordo_attivo: estrai_colore(config_toml.tema.bordo_attivo, default.bordo_attivo),
            testo: estrai_colore(config_toml.tema.testo, default.testo),
        }
    }
}
