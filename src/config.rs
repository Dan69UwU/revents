use ratatui::style::Color;
use serde::Deserialize;
use std::fs;
use std::str::FromStr;

#[derive(Deserialize)]
struct ConfigToml {
    tema: TemaToml,
}

#[derive(Deserialize)]
struct TemaToml {
    errore: String,
    evidenziato: String,
    bordo_normale: String,
    bordo_attivo: String,
    testo: String,
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

        let Ok(contenuto) = fs::read_to_string(percorso) else {
            return default;
        };

        let Ok(config_toml) = toml::from_str::<ConfigToml>(&contenuto) else {
            return default;
        };

        TemaApp {
            errore: Color::from_str(&config_toml.tema.errore).unwrap_or(default.errore),
            evidenziato: Color::from_str(&config_toml.tema.evidenziato)
                .unwrap_or(default.evidenziato),
            bordo_normale: Color::from_str(&config_toml.tema.bordo_normale)
                .unwrap_or(default.bordo_normale),
            bordo_attivo: Color::from_str(&config_toml.tema.bordo_attivo)
                .unwrap_or(default.bordo_attivo),
            testo: Color::from_str(&config_toml.tema.testo).unwrap_or(default.testo),
        }
    }
}
