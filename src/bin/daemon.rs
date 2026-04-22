use chrono::Local;
use revents::model::{AnticipoNotifica, Evento};
use std::env;
use std::fs;
use std::thread;
use std::time::Duration;

fn main() {
    let mut percorso_base = env::current_exe().expect("Impossibile ottenere il percorso");
    percorso_base.pop();
    let mut percorso_json = percorso_base.clone();
    percorso_json.push("agenda.json");

    let mut notificati: Vec<(String, chrono::NaiveDate)> = Vec::new();

    loop {
        if let Ok(contenuto) = fs::read_to_string(&percorso_json) {
            if let Ok(eventi) = serde_json::from_str::<Vec<Evento>>(&contenuto) {
                let ora_attuale = Local::now().naive_local();

                for ev in eventi {
                    if ev.notifica_anticipo == AnticipoNotifica::Nessuna {
                        continue;
                    }

                    let momento_evento = chrono::NaiveDateTime::new(ev.data_inizio, ev.ora_inizio);
                    let anticipo = chrono::Duration::minutes(ev.notifica_anticipo.minuti());
                    let momento_notifica = momento_evento - anticipo;

                    if ora_attuale >= momento_notifica && ora_attuale < momento_evento {
                        let id = (ev.nome.clone(), ev.data_inizio);

                        if !notificati.contains(&id) {
                            println!(
                                "[{}] Notifica per: {}",
                                ora_attuale.format("%H:%M:%S"),
                                ev.nome
                            );

                            let _ = std::process::Command::new("notify-send")
                                .arg("-u")
                                .arg("critical")
                                .arg("-t")
                                .arg("0")
                                .arg("-a")
                                .arg("AgendaTUI")
                                .arg("Promemoria Agenda")
                                .arg(format!(
                                    "Tra {} inizia: {}",
                                    ev.notifica_anticipo.as_str(),
                                    ev.nome
                                ))
                                .spawn();

                            notificati.push(id);
                        }
                    }
                }
            }
        }

        thread::sleep(Duration::from_secs(30));
    }
}
