use chrono::{Datelike, NaiveDate, NaiveDateTime, NaiveTime};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;
use std::io::Write;
use std::path::Path;

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
    pub data_fine: NaiveDate,
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

        let durata_giorni = (self.data_fine - self.data_inizio).num_days().max(0);

        match self.ricorrenza {
            Frequenza::Mai => data <= self.data_fine,
            Frequenza::Giornaliera => true,
            Frequenza::Settimanale => {
                let giorni_trascorsi = (data - self.data_inizio).num_days();
                giorni_trascorsi % 7 <= durata_giorni
            }
            Frequenza::Mensile => {
                let mut mese_inizio = chrono::NaiveDate::from_ymd_opt(
                    data.year(),
                    data.month(),
                    self.data_inizio.day(),
                )
                .unwrap_or(self.data_inizio);

                if mese_inizio > data {
                    let prev_month = if data.month() == 1 {
                        12
                    } else {
                        data.month() - 1
                    };
                    let prev_year = if data.month() == 1 {
                        data.year() - 1
                    } else {
                        data.year()
                    };
                    mese_inizio = chrono::NaiveDate::from_ymd_opt(
                        prev_year,
                        prev_month,
                        self.data_inizio.day(),
                    )
                    .unwrap_or(self.data_inizio);
                }

                let giorni_trascorsi = (data - mese_inizio).num_days();
                giorni_trascorsi >= 0 && giorni_trascorsi <= durata_giorni
            }
            Frequenza::Annuale => {
                let mut anno_inizio = chrono::NaiveDate::from_ymd_opt(
                    data.year(),
                    self.data_inizio.month(),
                    self.data_inizio.day(),
                )
                .unwrap_or(self.data_inizio);

                if anno_inizio > data {
                    anno_inizio = chrono::NaiveDate::from_ymd_opt(
                        data.year() - 1,
                        self.data_inizio.month(),
                        self.data_inizio.day(),
                    )
                    .unwrap_or(self.data_inizio);
                }

                let giorni_trascorsi = (data - anno_inizio).num_days();
                giorni_trascorsi >= 0 && giorni_trascorsi <= durata_giorni
            }
        }
    }
}
pub fn importa_ics(path: &Path) -> Result<Vec<Evento>, Box<dyn std::error::Error>> {
    let buf = BufReader::new(File::open(path)?);
    let reader = ical::IcalParser::new(buf);

    let mut nuovi_eventi = Vec::new();

    for calendar in reader {
        let cal = calendar.map_err(|_| "Errore nel parsing del calendario")?;

        for event in cal.events {
            let mut nome = String::from("Nuovo Evento");
            let mut descrizione = None;
            let mut data_inizio = chrono::Local::now().date_naive();
            let mut ora_inizio = NaiveTime::from_hms_opt(12, 0, 0).unwrap();
            let mut data_fine = chrono::Local::now().date_naive();
            let mut ricorrenza = Frequenza::Mai;
            let notifica_anticipo = AnticipoNotifica::Nessuna;

            for prop in event.properties {
                match prop.name.as_str() {
                    "SUMMARY" => {
                        if let Some(val) = prop.value {
                            nome = val;
                        }
                    }
                    "DESCRIPTION" => {
                        descrizione = prop.value;
                    }
                    "DTSTART" => {
                        if let Some(val) = prop.value {
                            let val_clean = val.replace("Z", "");
                            if val_clean.contains('T') {
                                if let Ok(dt) =
                                    NaiveDateTime::parse_from_str(&val_clean, "%Y%m%dT%H%M%S")
                                {
                                    data_inizio = dt.date();
                                    ora_inizio = dt.time();
                                    data_fine = dt.date();
                                }
                            } else {
                                if let Ok(d) = NaiveDate::parse_from_str(&val_clean, "%Y%m%d") {
                                    data_inizio = d;
                                    ora_inizio = NaiveTime::from_hms_opt(0, 0, 0).unwrap();
                                    data_fine = d;
                                }
                            }
                        }
                    }
                    "DTEND" => {
                        if let Some(val) = prop.value {
                            let val_clean = val.replace("Z", "");
                            if val_clean.contains('T') {
                                if let Ok(dt) =
                                    NaiveDateTime::parse_from_str(&val_clean, "%Y%m%dT%H%M%S")
                                {
                                    data_fine = dt.date();
                                }
                            } else {
                                if let Ok(d) = NaiveDate::parse_from_str(&val_clean, "%Y%m%d") {
                                    data_fine = d;
                                }
                            }
                        }
                    }
                    "RRULE" => {
                        if let Some(val) = prop.value {
                            let rule = val.to_uppercase();
                            ricorrenza = if rule.contains("FREQ=DAILY") {
                                Frequenza::Giornaliera
                            } else if rule.contains("FREQ=WEEKLY") {
                                Frequenza::Settimanale
                            } else if rule.contains("FREQ=MONTHLY") {
                                Frequenza::Mensile
                            } else if rule.contains("FREQ=YEARLY") {
                                Frequenza::Annuale
                            } else {
                                Frequenza::Mai
                            };
                        }
                    }
                    _ => {}
                }
            }

            nuovi_eventi.push(Evento {
                nome,
                descrizione,
                data_inizio,
                ora_inizio,
                data_fine,
                ricorrenza,
                notifica_anticipo,
                riproduci_suono: false,
            });
        }
    }

    Ok(nuovi_eventi)
}
pub fn esporta_ics(eventi: &[Evento], path: &Path) -> std::io::Result<()> {
    let mut file = File::create(path)?;

    write!(file, "BEGIN:VCALENDAR\r\n")?;
    write!(file, "VERSION:2.0\r\n")?;
    write!(file, "PRODID:-//Revents App//IT\r\n")?;

    let now = chrono::Local::now().format("%Y%m%dT%H%M%SZ");

    for ev in eventi {
        write!(file, "BEGIN:VEVENT\r\n")?;

        write!(file, "DTSTAMP:{}\r\n", now)?;
        let uid = format!(
            "{}-{}@revents",
            ev.data_inizio.format("%Y%m%d"),
            ev.nome.replace(' ', "")
        );
        write!(file, "UID:{}\r\n", uid)?;

        write!(file, "SUMMARY:{}\r\n", ev.nome)?;

        if let Some(desc) = &ev.descrizione {
            let desc_escaped = desc.replace('\n', "\\n");
            write!(file, "DESCRIPTION:{}\r\n", desc_escaped)?;
        }

        let dtstart = format!(
            "{}T{}",
            ev.data_inizio.format("%Y%m%d"),
            ev.ora_inizio.format("%H%M%S")
        );
        write!(file, "DTSTART:{}\r\n", dtstart)?;

        match ev.ricorrenza {
            Frequenza::Giornaliera => write!(file, "RRULE:FREQ=DAILY\r\n")?,
            Frequenza::Settimanale => write!(file, "RRULE:FREQ=WEEKLY\r\n")?,
            Frequenza::Mensile => write!(file, "RRULE:FREQ=MONTHLY\r\n")?,
            Frequenza::Annuale => write!(file, "RRULE:FREQ=YEARLY\r\n")?,
            Frequenza::Mai => {}
        }

        if ev.notifica_anticipo != AnticipoNotifica::Nessuna {
            write!(file, "BEGIN:VALARM\r\n")?;
            write!(file, "ACTION:DISPLAY\r\n")?;
            write!(file, "DESCRIPTION:Promemoria: {}\r\n", ev.nome)?;

            let min = match ev.notifica_anticipo {
                AnticipoNotifica::CinqueMinuti => "-PT5M",
                AnticipoNotifica::QuindiciMinuti => "-PT15M",
                AnticipoNotifica::TrentaMinuti => "-PT30M",
                AnticipoNotifica::UnOra => "-PT1H",
                AnticipoNotifica::UnGiorno => "-P1D",
                _ => "-PT0M",
            };
            write!(file, "TRIGGER:{}\r\n", min)?;
            write!(file, "END:VALARM\r\n")?;
        }

        write!(file, "END:VEVENT\r\n")?;
    }

    write!(file, "END:VCALENDAR\r\n")?;

    Ok(())
}
