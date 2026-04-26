use chrono::{Datelike, NaiveDate, NaiveTime};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Frequenza {
    Mai,
    Giornaliera,
    Settimanale,
    Mensile,
    Annuale,
}

impl Frequenza {
    pub fn successiva(&self) -> Self {
        match self {
            Frequenza::Mai => Frequenza::Giornaliera,
            Frequenza::Giornaliera => Frequenza::Settimanale,
            Frequenza::Settimanale => Frequenza::Mensile,
            Frequenza::Mensile => Frequenza::Annuale,
            Frequenza::Annuale => Frequenza::Mai,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum AnticipoNotifica {
    Nessuna,
    CinqueMinuti,
    QuindiciMinuti,
    TrentaMinuti,
    UnOra,
    UnGiorno,
}

impl AnticipoNotifica {
    pub fn successiva(&self) -> Self {
        match self {
            AnticipoNotifica::Nessuna => AnticipoNotifica::CinqueMinuti,
            AnticipoNotifica::CinqueMinuti => AnticipoNotifica::QuindiciMinuti,
            AnticipoNotifica::QuindiciMinuti => AnticipoNotifica::TrentaMinuti,
            AnticipoNotifica::TrentaMinuti => AnticipoNotifica::UnOra,
            AnticipoNotifica::UnOra => AnticipoNotifica::UnGiorno,
            AnticipoNotifica::UnGiorno => AnticipoNotifica::Nessuna,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            AnticipoNotifica::Nessuna => "Nessuna",
            AnticipoNotifica::CinqueMinuti => "5 minuti prima",
            AnticipoNotifica::QuindiciMinuti => "15 minuti prima",
            AnticipoNotifica::TrentaMinuti => "30 minuti prima",
            AnticipoNotifica::UnOra => "1 ora prima",
            AnticipoNotifica::UnGiorno => "1 giorno prima",
        }
    }

    pub fn minuti(&self) -> i64 {
        match self {
            AnticipoNotifica::Nessuna => 0,
            AnticipoNotifica::CinqueMinuti => 5,
            AnticipoNotifica::QuindiciMinuti => 15,
            AnticipoNotifica::TrentaMinuti => 30,
            AnticipoNotifica::UnOra => 60,
            AnticipoNotifica::UnGiorno => 1440,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Evento {
    pub nome: String,
    pub descrizione: Option<String>,
    pub data_inizio: NaiveDate,
    pub ora_inizio: NaiveTime,
    pub ricorrenza: Frequenza,
    pub notifica_anticipo: AnticipoNotifica,
    #[serde(default)]
    pub riproduci_suono: bool,
}

impl Evento {
    pub fn appare_il(&self, data: NaiveDate) -> bool {
        if data < self.data_inizio {
            return false;
        }
        match self.ricorrenza {
            Frequenza::Mai => data == self.data_inizio,
            Frequenza::Giornaliera => true,
            Frequenza::Settimanale => (data - self.data_inizio).num_days() % 7 == 0,
            Frequenza::Mensile => data.day() == self.data_inizio.day(),
            Frequenza::Annuale => {
                data.day() == self.data_inizio.day() && data.month() == self.data_inizio.month()
            }
        }
    }
}
