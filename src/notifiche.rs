use crate::model::{AnticipoNotifica, Evento};
use chrono::{Local, NaiveDateTime};
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub fn invia_notifica_sistema(titolo: &str, messaggio: &str) {
    let _ = Command::new("notify-send")
        .arg("-i")
        .arg("calendar")
        .arg(titolo)
        .arg(messaggio)
        .spawn();
}

pub fn avvia_motore_notifiche(eventi_condivisi: Arc<Mutex<Vec<Evento>>>) {
    thread::spawn(move || {
        let mut notificati: Vec<(String, chrono::NaiveDate)> = Vec::new();

        loop {
            let ora_attuale = Local::now().naive_local();
            if let Ok(lista) = eventi_condivisi.lock() {
                for ev in lista.iter() {
                    if ev.notifica_anticipo == AnticipoNotifica::Nessuna {
                        continue;
                    }

                    let momento_evento = NaiveDateTime::new(ev.data_inizio, ev.ora_inizio);
                    let anticipo_minuti = ev.notifica_anticipo.minuti();
                    let momento_notifica =
                        momento_evento - chrono::Duration::minutes(anticipo_minuti);

                    if ora_attuale >= momento_notifica && ora_attuale < momento_evento {
                        let id = (ev.nome.clone(), ev.data_inizio);
                        if !notificati.contains(&id) {
                            invia_notifica_sistema(
                                "Promemoria Agenda",
                                &format!(
                                    "Tra {} inizia: {}",
                                    ev.notifica_anticipo.as_str(),
                                    ev.nome
                                ),
                            );
                            notificati.push(id);
                        }
                    }
                }
            }
            thread::sleep(Duration::from_secs(30));
        }
    });
}
