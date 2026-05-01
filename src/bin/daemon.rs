use chrono::{Datelike, Local};
use notify_rust::{Notification, Timeout, Urgency};
use revents::model::{AnticipoNotifica, Evento, Frequenza};
use rodio::{Decoder, OutputStream, Sink};
use serde::Deserialize;
use std::env;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;

fn ottieni_percorso_audio() -> Option<PathBuf> {
    let mut percorso_base = env::current_exe().ok()?;
    percorso_base.pop();

    #[derive(Deserialize)]
    struct ConfigAudio {
        percorso_audio: Option<String>,
    }

    let mut percorso_config = percorso_base.clone();
    percorso_config.push("config.toml");

    if let Ok(contenuto) = fs::read_to_string(percorso_config) {
        if let Ok(config) = toml::from_str::<ConfigAudio>(&contenuto) {
            if let Some(p) = config.percorso_audio.filter(|s| !s.trim().is_empty()) {
                let percorso_custom = PathBuf::from(p);
                if percorso_custom.exists() {
                    return Some(percorso_custom);
                }
            }
        }
    }

    percorso_base.push("sound.wav");
    if percorso_base.exists() {
        Some(percorso_base)
    } else {
        None
    }
}

fn riproduci_suono(percorso: &PathBuf) {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    if let Ok(file) = File::open(percorso) {
        let source = Decoder::new(BufReader::new(file)).unwrap();
        sink.append(source);
        sink.sleep_until_end();
    }
}

fn main() {
    let mut percorso_base = env::current_exe().unwrap();
    percorso_base.pop();

    let mut percorso_json = percorso_base.clone();
    percorso_json.push("agenda.json");

    let mut percorso_stato = percorso_base.clone();
    percorso_stato.push("stato_notifiche.json");

    let mut percorso_audio = percorso_base.clone();
    percorso_audio.pop();
    percorso_audio.pop();
    percorso_audio.push("sound.wav"); //

    let mut notificati: Vec<(String, chrono::NaiveDate, String)> = if percorso_stato.exists() {
        if let Ok(contenuto) = fs::read_to_string(&percorso_stato) {
            serde_json::from_str(&contenuto).unwrap_or_else(|_| Vec::new())
        } else {
            Vec::new()
        }
    } else {
        Vec::new()
    };

    loop {
        let percorso_audio_custom = ottieni_percorso_audio();
        if let Ok(contenuto) = fs::read_to_string(&percorso_json) {
            if let Ok(eventi) = serde_json::from_str::<Vec<Evento>>(&contenuto) {
                let ora_attuale = Local::now().naive_local();
                let mut stato_modificato = false;
                let margine_giorni = chrono::Duration::days(7);
                let oggi = ora_attuale.date();

                for ev in eventi {
                    let accade_oggi = match ev.ricorrenza {
                        Frequenza::Mai => ev.data_inizio == oggi,
                        Frequenza::Giornaliera => oggi >= ev.data_inizio,
                        Frequenza::Settimanale => {
                            oggi >= ev.data_inizio && (oggi - ev.data_inizio).num_days() % 7 == 0
                        }
                        Frequenza::Mensile => {
                            oggi >= ev.data_inizio && oggi.day() == ev.data_inizio.day()
                        }
                        Frequenza::Annuale => {
                            oggi >= ev.data_inizio
                                && oggi.day() == ev.data_inizio.day()
                                && oggi.month() == ev.data_inizio.month()
                        }
                    };

                    if !accade_oggi {
                        continue;
                    }

                    let momento_evento = chrono::NaiveDateTime::new(oggi, ev.ora_inizio);
                    let id_anticipo = (ev.nome.clone(), oggi, "anticipo".to_string());
                    let id_esatta = (ev.nome.clone(), oggi, "esatta".to_string());

                    if ev.notifica_anticipo != AnticipoNotifica::Nessuna {
                        let anticipo = chrono::Duration::minutes(ev.notifica_anticipo.minuti());
                        let momento_notifica = momento_evento - anticipo;

                        if ora_attuale >= momento_notifica && ora_attuale < momento_evento {
                            if !notificati.contains(&id_anticipo) {
                                let mut notifica = Notification::new();
                                notifica
                                    .appname("AgendaTUI")
                                    .summary("⏰ PROMEMORIA")
                                    .body(&format!(
                                        "<b>IN ARRIVO</b>\n{}\n<i>Inizia alle {}</i>",
                                        ev.nome,
                                        ev.ora_inizio.format("%H:%M")
                                    ))
                                    .icon("appointment-soon")
                                    .urgency(Urgency::Normal)
                                    .timeout(Timeout::Never);
                                if ev.riproduci_suono && percorso_audio_custom.is_none() {
                                    notifica.sound_name("message-new-instant");
                                }
                                let _ = notifica.show();

                                if ev.riproduci_suono {
                                    if let Some(percorso) = &percorso_audio_custom {
                                        if percorso.exists() {
                                            let p = percorso.clone();
                                            thread::spawn(move || {
                                                riproduci_suono(&p);
                                            });
                                        }
                                    }
                                }

                                notificati.push(id_anticipo);
                                stato_modificato = true;
                            }
                        }
                    }

                    if ora_attuale >= momento_evento {
                        if !notificati.contains(&id_esatta) {
                            let mut notifica = Notification::new();
                            notifica
                                .appname("AgendaTUI")
                                .summary("⚠️ NOTIFICA EVENTO")
                                .body(&format!(
                                    "<b>GIÀ INIZIATO</b>\n{}\n<i>Oggi alle {}</i>",
                                    ev.nome,
                                    ev.ora_inizio.format("%H:%M")
                                ))
                                .icon("dialog-warning")
                                .urgency(Urgency::Normal)
                                .timeout(Timeout::Never);

                            if ev.riproduci_suono && percorso_audio_custom.is_none() {
                                notifica.sound_name("message-new-instant");
                            }

                            let _ = notifica.show();

                            if ev.riproduci_suono {
                                if let Some(percorso) = &percorso_audio_custom {
                                    if percorso.exists() {
                                        let p = percorso.clone();
                                        thread::spawn(move || {
                                            riproduci_suono(&p);
                                        });
                                    }
                                }
                            }

                            notificati.push(id_esatta);
                            stato_modificato = true;
                        }
                    }
                }
                let data_limite = oggi - margine_giorni;
                notificati.retain(|(_, data_id, _)| *data_id >= data_limite);
                if stato_modificato {
                    if let Ok(json_stato) = serde_json::to_string_pretty(&notificati) {
                        let _ = fs::write(&percorso_stato, json_stato);
                    }
                }
            }
        }
        thread::sleep(Duration::from_secs(30));
    }
}
