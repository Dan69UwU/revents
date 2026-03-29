use crate::app::{App, StatoApp};
use crate::model::{AnticipoNotifica, Evento};
use chrono::{Datelike, Weekday};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, Wrap},
};

const MESI: [&str; 12] = [
    "Gennaio",
    "Febbraio",
    "Marzo",
    "Aprile",
    "Maggio",
    "Giugno",
    "Luglio",
    "Agosto",
    "Settembre",
    "Ottobre",
    "Novembre",
    "Dicembre",
];

pub fn draw(f: &mut Frame, app: &App, eventi: &[Evento]) {
    let area = f.area();

    let main_l = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(3), Constraint::Length(3)])
        .split(area);

    let corpo = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(main_l[0]);

    match app.stato {
        StatoApp::Normale => {
            let anno = app.data_sel.year();
            let mese = app.data_sel.month();
            let mut cal_str = format!(
                "      {} {}\n\n Lu  Ma  Me  Gi  Ve  Sa  Do\n",
                MESI[(mese - 1) as usize],
                anno
            );

            if let Some(primo) = chrono::NaiveDate::from_ymd_opt(anno, mese, 1) {
                let spazi = primo.weekday().number_from_monday() - 1;
                for _ in 0..spazi {
                    cal_str.push_str("    ");
                }

                let mut g = primo;
                while g.month() == mese {
                    let ha_eventi = eventi.iter().any(|e| e.appare_il(g));

                    let style_char = if g == app.data_sel {
                        format!("[{:2}]", g.day())
                    } else if ha_eventi {
                        format!("*{:2} ", g.day())
                    } else {
                        format!(" {:2} ", g.day())
                    };

                    cal_str.push_str(&style_char);

                    if g.weekday().number_from_monday() == 7 {
                        cal_str.push('\n');
                    }

                    if let Some(next_g) = g.succ_opt() {
                        g = next_g;
                        if g.month() != mese {
                            break;
                        }
                    } else {
                        break;
                    }
                }
            }
            f.render_widget(
                Paragraph::new(cal_str)
                    .block(Block::default().title(" Calendario ").borders(Borders::ALL)),
                corpo[0],
            );

            let oggi: Vec<&Evento> = eventi
                .iter()
                .filter(|e| e.appare_il(app.data_sel))
                .collect();
            let mut txt = format!("Eventi del {}:\n\n", app.data_sel);
            if oggi.is_empty() {
                txt.push_str("Nessun evento salvato.");
            } else {
                for ev in oggi {
                    let icon = if ev.notifica_anticipo != AnticipoNotifica::Nessuna {
                        "🔔"
                    } else {
                        ""
                    };
                    txt.push_str(&format!(
                        "• [{}] {} {}\n",
                        ev.ora_inizio.format("%H:%M"),
                        ev.nome,
                        icon
                    ));
                }
            }
            f.render_widget(
                Paragraph::new(txt)
                    .block(Block::default().title(" Anteprima ").borders(Borders::ALL)),
                corpo[1],
            );
        }

        StatoApp::Dettaglio => {       
            let oggi: Vec<&Evento> = eventi
                .iter()
                .filter(|e| e.appare_il(app.data_sel))
                .collect();
            let mut lista = String::new();
            for (i, ev) in oggi.iter().enumerate() {
                let sel = if i == app.focus_index { ">> " } else { "   " };
                lista.push_str(&format!("{}{}\n", sel, ev.nome));
            }
            f.render_widget(
                Paragraph::new(lista).block(
                    Block::default()
                        .title(" Seleziona Evento ")
                        .borders(Borders::ALL),
                ),
                corpo[0],
            );

            if let Some(ev) = oggi.get(app.focus_index) {
                let det = format!(
                    "NOME: {}\nORA: {}\nRICORRENZA: {:?}\nNOTIFICA: {}\nINIZIO: {}\n\nDESCRIZIONE:\n{}",
                    ev.nome,
                    ev.ora_inizio.format("%H:%M"),
                    ev.ricorrenza,
                    ev.notifica_anticipo.as_str(),
                    ev.data_inizio,
                    ev.descrizione.as_deref().unwrap_or("---")
                );
                f.render_widget(
                    Paragraph::new(det)
                        .block(
                            Block::default()
                                .title(" Scheda Evento ")
                                .borders(Borders::ALL),
                        )
                        .wrap(Wrap { trim: true }),
                    corpo[1],
                );
            }
        }

        StatoApp::Creazione | StatoApp::Modifica => {
            let form = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3), 
                    Constraint::Length(3), 
                    Constraint::Length(3), 
                    Constraint::Length(3), 
                    Constraint::Length(3), 
                    Constraint::Min(0),
                ])
                .split(corpo[1]);

            let nome_vuoto = app.b_nome.trim().is_empty();
            let titolo_n = if nome_vuoto {
                " Nome (OBBLIGATORIO) "
            } else {
                " Nome "
            };
            let stile_n = if app.focus_index == 0 {
                if nome_vuoto {
                    Style::default().fg(Color::Red)
                } else {
                    Style::default().fg(Color::Yellow)
                }
            } else {
                Style::default()
            };

            let s = |i| {
                if app.focus_index == i {
                    Style::default().fg(Color::Yellow)
                } else {
                    Style::default()
                }
            };

            f.render_widget(
                Paragraph::new(app.b_nome.as_str()).block(
                    Block::default()
                        .title(titolo_n)
                        .borders(Borders::ALL)
                        .border_style(stile_n),
                ),
                form[0],
            );
            f.render_widget(
                Paragraph::new(app.b_desc.as_str()).block(
                    Block::default()
                        .title(" Descrizione ")
                        .borders(Borders::ALL)
                        .border_style(s(1)),
                ),
                form[1],
            );
            f.render_widget(
                Paragraph::new(app.b_ora.as_str()).block(
                    Block::default()
                        .title(" Ora (HH:MM) ")
                        .borders(Borders::ALL)
                        .border_style(s(2)),
                ),
                form[2],
            );
            f.render_widget(
                Paragraph::new(format!("{:?} (SPAZIO)", app.b_freq)).block(
                    Block::default()
                        .title(" Ricorrenza ")
                        .borders(Borders::ALL)
                        .border_style(s(3)),
                ),
                form[3],
            );
            f.render_widget(
                Paragraph::new(format!("{} (SPAZIO)", app.b_notifica.as_str())).block(
                    Block::default()
                        .title(" Notifica Anticipata ")
                        .borders(Borders::ALL)
                        .border_style(s(4)),
                ),
                form[4],
            );

            f.render_widget(
                Paragraph::new("Stai editando un evento")
                    .block(Block::default().title(" Editor ").borders(Borders::ALL)),
                corpo[0],
            );

            // --- GESTIONE CURSORE ---
            let area_attiva = form[app.focus_index];
            match app.focus_index {
                0 => f.set_cursor_position((
                    area_attiva.x + app.b_nome.len() as u16 + 1,
                    area_attiva.y + 1,
                )),
                1 => f.set_cursor_position((
                    area_attiva.x + app.b_desc.len() as u16 + 1,
                    area_attiva.y + 1,
                )),
                2 => f.set_cursor_position((
                    area_attiva.x + app.b_ora.len() as u16 + 1,
                    area_attiva.y + 1,
                )),
                _ => {}
            }
        }
    }

    let aiuti = match app.stato {
        StatoApp::Normale => "Q: Esci | N: Nuovo | INVIO: Dettagli | Frecce: Naviga",
        StatoApp::Dettaglio => "ESC: Torna | J/K: Scorri | D: Elimina | M: Modifica",
        StatoApp::Creazione | StatoApp::Modifica => {
            "TAB: Campo | SPAZIO: Cambia | INVIO: Salva | ESC: Annulla"
        }
    };
    f.render_widget(
        Paragraph::new(aiuti).block(Block::default().title(" Comandi ").borders(Borders::ALL)),
        main_l[1],
    );
}
