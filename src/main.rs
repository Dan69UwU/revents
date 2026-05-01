use revents::app::{App, StatoApp};
use revents::model::{AnticipoNotifica, Evento, Frequenza};
use revents::ui;

use chrono::{Duration, NaiveTime};
use crossterm::{
    event::{
        self, DisableMouseCapture, EnableMouseCapture, Event as CEvent, KeyCode, KeyEventKind,
    },
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::{env, fs, io};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut path_json = env::current_exe()?;
    path_json.pop();
    path_json.push("agenda.json");

    let eventi_caricati: Vec<Evento> = if path_json.exists() {
        let contenuto = fs::read_to_string(&path_json)?;
        serde_json::from_str(&contenuto).unwrap_or_default()
    } else {
        Vec::new()
    };

    let eventi_shared = Arc::new(Mutex::new(eventi_caricati));

    let mut app = App::new();

    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        let _ = disable_raw_mode();
        let _ = execute!(io::stdout(), LeaveAlternateScreen, DisableMouseCapture);
        original_hook(panic_info);
    }));

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = run_app(&mut terminal, &mut app, &eventi_shared, &path_json);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("Errore dell'applicazione: {:?}", err);
    }

    Ok(())
}

fn trova_indice_reale(
    lista: &[Evento],
    data: chrono::NaiveDate,
    focus_index: usize,
) -> Option<usize> {
    lista
        .iter()
        .enumerate()
        .filter(|(_, e)| e.data_inizio == data)
        .nth(focus_index)
        .map(|(i, _)| i)
}

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
    eventi_shared: &Arc<Mutex<Vec<Evento>>>,
    path_json: &PathBuf,
) -> io::Result<()>
where
    std::io::Error: From<B::Error>,
{
    loop {
        terminal.draw(|f| {
            let lista = eventi_shared.lock().unwrap();
            ui::draw(f, app, &lista);
        })?;

        if let CEvent::Key(key) = event::read()? {
            if key.kind != KeyEventKind::Press {
                continue;
            }

            let mut lista = eventi_shared.lock().unwrap();
            let mut dati_modificati = false;

            match app.stato {
                StatoApp::Normale => match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                    KeyCode::Char('n') => {
                        app.reset_buffer();
                        app.stato = StatoApp::Creazione;
                    }
                    KeyCode::Right => {
                        app.data_sel += Duration::days(1);
                        app.focus_index = 0;
                    }
                    KeyCode::Left => {
                        app.data_sel -= Duration::days(1);
                        app.focus_index = 0;
                    }
                    KeyCode::Up => {
                        app.data_sel -= Duration::days(7);
                        app.focus_index = 0;
                    }
                    KeyCode::Down => {
                        app.data_sel += Duration::days(7);
                        app.focus_index = 0;
                    }
                    KeyCode::Tab => {
                        let eventi_giorno = lista
                            .iter()
                            .filter(|e| e.data_inizio == app.data_sel)
                            .count();
                        if eventi_giorno > 0 {
                            app.focus_index = (app.focus_index + 1) % eventi_giorno;
                        }
                    }
                    KeyCode::Enter => {
                        if trova_indice_reale(&lista, app.data_sel, app.focus_index).is_some() {
                            app.stato = StatoApp::Dettaglio;
                        }
                    }
                    KeyCode::F(5) => {
                        let mut path_config = std::env::current_exe().unwrap_or_default();
                        path_config.pop();
                        path_config.push("config.toml");
                        app.tema = revents::config::TemaApp::carica(
                            path_config.to_str().unwrap_or("config.toml"),
                        );
                    }

                    _ => {}
                },
                StatoApp::Creazione | StatoApp::Modifica => match key.code {
                    KeyCode::Esc => {
                        app.reset_buffer();
                        app.stato = StatoApp::Normale;
                    }
                    KeyCode::Down | KeyCode::Tab => {
                        app.focus_index = (app.focus_index + 1) % 6;
                    }
                    KeyCode::Up => {
                        if app.focus_index == 0 {
                            app.focus_index = 5;
                        } else {
                            app.focus_index -= 1;
                        }
                    }
                    KeyCode::Right => {
                        if app.focus_index == 3 {
                            app.b_freq = match app.b_freq {
                                Frequenza::Mai => Frequenza::Giornaliera,
                                Frequenza::Giornaliera => Frequenza::Settimanale,
                                Frequenza::Settimanale => Frequenza::Mensile,
                                Frequenza::Mensile => Frequenza::Annuale,
                                _ => Frequenza::Mai,
                            };
                        } else if app.focus_index == 4 {
                            app.b_notifica = match app.b_notifica {
                                AnticipoNotifica::Nessuna => AnticipoNotifica::CinqueMinuti,
                                AnticipoNotifica::CinqueMinuti => AnticipoNotifica::QuindiciMinuti,
                                AnticipoNotifica::QuindiciMinuti => AnticipoNotifica::TrentaMinuti,
                                AnticipoNotifica::TrentaMinuti => AnticipoNotifica::UnOra,
                                AnticipoNotifica::UnOra => AnticipoNotifica::UnGiorno,
                                AnticipoNotifica::UnGiorno => AnticipoNotifica::Nessuna,
                            };
                        } else if app.focus_index == 5 {
                            app.b_suono = !app.b_suono;
                        }
                    }
                    KeyCode::Char(c) => {
                        if c == ' ' && app.focus_index == 3 {
                            app.b_freq = match app.b_freq {
                                Frequenza::Mai => Frequenza::Giornaliera,
                                Frequenza::Giornaliera => Frequenza::Settimanale,
                                Frequenza::Settimanale => Frequenza::Mensile,
                                Frequenza::Mensile => Frequenza::Annuale,
                                _ => Frequenza::Mai,
                            };
                        } else if c == ' ' && app.focus_index == 4 {
                            app.b_notifica = match app.b_notifica {
                                AnticipoNotifica::Nessuna => AnticipoNotifica::CinqueMinuti,
                                AnticipoNotifica::CinqueMinuti => AnticipoNotifica::QuindiciMinuti,
                                AnticipoNotifica::QuindiciMinuti => AnticipoNotifica::TrentaMinuti,
                                AnticipoNotifica::TrentaMinuti => AnticipoNotifica::UnOra,
                                AnticipoNotifica::UnOra => AnticipoNotifica::UnGiorno,
                                AnticipoNotifica::UnGiorno => AnticipoNotifica::Nessuna,
                            };
                        } else if c == ' ' && app.focus_index == 5 {
                            app.b_suono = !app.b_suono;
                        } else {
                            match app.focus_index {
                                0 => app.b_nome.push(c),
                                1 => app.b_desc.push(c),
                                2 => app.b_ora.push(c),
                                _ => {}
                            }
                        }
                    }
                    KeyCode::Backspace => match app.focus_index {
                        0 => {
                            app.b_nome.pop();
                        }
                        1 => {
                            app.b_desc.pop();
                        }
                        2 => {
                            app.b_ora.pop();
                        }
                        _ => {}
                    },
                    KeyCode::Enter => {
                        if !app.b_nome.is_empty() {
                            let ora_parsata = NaiveTime::parse_from_str(&app.b_ora, "%H:%M")
                                .unwrap_or_else(|_| NaiveTime::from_hms_opt(12, 0, 0).unwrap());

                            let nuovo_evento = Evento {
                                nome: app.b_nome.clone(),
                                descrizione: if app.b_desc.is_empty() {
                                    None
                                } else {
                                    Some(app.b_desc.clone())
                                },
                                data_inizio: app.data_sel,
                                ora_inizio: ora_parsata,
                                ricorrenza: app.b_freq.clone(),
                                notifica_anticipo: app.b_notifica.clone(),
                                riproduci_suono: app.b_suono,
                            };

                            if app.stato == StatoApp::Creazione {
                                lista.push(nuovo_evento);
                            } else if let Some(idx) = app.indice_modifica {
                                if idx < lista.len() {
                                    lista[idx] = nuovo_evento;
                                }
                            }

                            app.reset_buffer();
                            dati_modificati = true;
                            app.stato = StatoApp::Normale;
                        }
                    }
                    _ => {}
                },
                StatoApp::Dettaglio => match key.code {
                    KeyCode::Esc => app.stato = StatoApp::Normale,
                    KeyCode::Char('d') => app.stato = StatoApp::Conferma,
                    KeyCode::Char('n') => {
                        app.reset_buffer();
                        app.stato = StatoApp::Creazione;
                    }
                    KeyCode::Char('m') => {
                        if let Some(idx) = trova_indice_reale(&lista, app.data_sel, app.focus_index)
                        {
                            let ev = &lista[idx];
                            app.b_nome = ev.nome.clone();
                            app.b_desc = ev.descrizione.clone().unwrap_or_default();
                            app.b_ora = ev.ora_inizio.format("%H:%M").to_string();
                            app.b_freq = ev.ricorrenza.clone();
                            app.b_notifica = ev.notifica_anticipo.clone();
                            app.b_suono = ev.riproduci_suono;

                            app.indice_modifica = Some(idx);
                            app.focus_index = 0;
                            app.stato = StatoApp::Modifica;
                        }
                    }
                    KeyCode::Down => {
                        let eventi_giorno = lista
                            .iter()
                            .filter(|e| e.data_inizio == app.data_sel)
                            .count();
                        if eventi_giorno > 0 {
                            app.focus_index = (app.focus_index + 1) % eventi_giorno;
                        }
                    }
                    KeyCode::Up => {
                        let eventi_giorno = lista
                            .iter()
                            .filter(|e| e.data_inizio == app.data_sel)
                            .count();
                        if eventi_giorno > 0 {
                            if app.focus_index == 0 {
                                app.focus_index = eventi_giorno - 1;
                            } else {
                                app.focus_index -= 1;
                            }
                        }
                    }
                    _ => {}
                },
                StatoApp::Conferma => match key.code {
                    KeyCode::Char('s') | KeyCode::Char('y') | KeyCode::Enter => {
                        if let Some(idx) = trova_indice_reale(&lista, app.data_sel, app.focus_index)
                        {
                            lista.remove(idx);
                            dati_modificati = true;
                            app.focus_index = 0;
                            app.stato = StatoApp::Normale;
                        }
                    }
                    KeyCode::Char('n') | KeyCode::Esc => {
                        app.stato = StatoApp::Dettaglio;
                    }
                    _ => {}
                },
            }
            if dati_modificati {
                if let Ok(json) = serde_json::to_string_pretty(&*lista) {
                    let _ = fs::write(path_json, json);
                }
            }
        }
    }
}
