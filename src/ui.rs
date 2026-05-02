use crate::app::{App, StatoApp};
use crate::model::{AnticipoNotifica, Evento};
use chrono::Datelike;
use ratatui::style::Stylize;
use ratatui::text::{Line, Span};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Wrap},
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
        .constraints([Constraint::Length(32), Constraint::Min(0)])
        .split(main_l[0]);

    match app.stato {
        StatoApp::Normale => {
            let anno = app.data_sel.year();
            let mese = app.data_sel.month();

            let mut linee_calendario: Vec<Line> = Vec::new();

            let titolo_mese = format!("      {} {}", MESI[(mese - 1) as usize], anno);
            linee_calendario.push(Line::from(Span::styled(
                titolo_mese,
                Style::default().fg(app.tema.testo),
            )));
            linee_calendario.push(Line::raw(""));
            linee_calendario.push(Line::from(Span::styled(
                " Lu  Ma  Me  Gi  Ve  Sa  Do",
                Style::default().fg(app.tema.testo),
            )));

            if let Some(primo) = chrono::NaiveDate::from_ymd_opt(anno, mese, 1) {
                let mut riga_corrente: Vec<Span> = Vec::new();

                let spazi = primo.weekday().number_from_monday() - 1;
                for _ in 0..spazi {
                    riga_corrente.push(Span::raw("    "));
                }

                let mut g = primo;
                while g.month() == mese {
                    let ha_eventi = eventi.iter().any(|e| e.appare_il(g));

                    let (testo_giorno, stile) = if g == app.data_sel {
                        (
                            format!("[{:2}]", g.day()),
                            Style::default().fg(app.tema.bordo_attivo),
                        )
                    } else if ha_eventi {
                        (
                            format!("*{:2} ", g.day()),
                            Style::default().fg(app.tema.errore),
                        )
                    } else {
                        (
                            format!(" {:2} ", g.day()),
                            Style::default().fg(app.tema.testo),
                        )
                    };

                    riga_corrente.push(Span::styled(testo_giorno, stile));

                    if g.weekday().number_from_monday() == 7 {
                        linee_calendario.push(Line::from(riga_corrente.clone()));
                        riga_corrente.clear();
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

                if !riga_corrente.is_empty() {
                    linee_calendario.push(Line::from(riga_corrente));
                }
            }

            f.render_widget(
                Paragraph::new(linee_calendario).block(
                    Block::default()
                        .title(" Calendario ")
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(app.tema.bordo_normale)),
                ),
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
                    if let Some(desc) = &ev.descrizione {
                        if !desc.trim().is_empty() {
                            let prima_riga = desc.lines().next().unwrap_or("");
                            txt.push_str(&format!("    ↳ {}\n", prima_riga));
                        }
                    }
                }
            }
            f.render_widget(
                Paragraph::new(txt).fg(app.tema.testo).block(
                    Block::default()
                        .title(" Anteprima ")
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(app.tema.bordo_normale)),
                ),
                corpo[1],
            );
        }

        StatoApp::Dettaglio => {
            let oggi: Vec<&Evento> = eventi
                .iter()
                .filter(|e| e.appare_il(app.data_sel))
                .collect();
            let mut linee: Vec<Line> = Vec::new();
            for (i, ev) in oggi.iter().enumerate() {
                if i == app.focus_index {
                    linee.push(Line::from(vec![
                        Span::styled(">> ", Style::default().fg(app.tema.bordo_attivo)),
                        Span::styled(ev.nome.clone(), Style::default().fg(app.tema.bordo_attivo)),
                    ]));
                } else {
                    linee.push(Line::from(vec![
                        Span::raw("   "),
                        Span::styled(ev.nome.clone(), Style::default().fg(app.tema.testo)),
                    ]));
                }
            }
            f.render_widget(
                Paragraph::new(linee).block(
                    Block::default()
                        .title(" Seleziona Evento ")
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(app.tema.bordo_normale)),
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
                        .fg(app.tema.testo)
                        .block(
                            Block::default()
                                .title(" Scheda Evento ")
                                .borders(Borders::ALL)
                                .border_style(Style::default().fg(app.tema.bordo_normale)),
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
                Style::default().fg(app.tema.bordo_attivo)
            } else if nome_vuoto {
                Style::default().fg(app.tema.errore)
            } else {
                Style::default().fg(app.tema.bordo_normale)
            };

            let s = |i| {
                if app.focus_index == i {
                    Style::default().fg(app.tema.bordo_attivo)
                } else {
                    Style::default().fg(app.tema.bordo_normale)
                }
            };

            f.render_widget(
                Paragraph::new(app.b_nome.as_str())
                    .fg(app.tema.testo)
                    .block(
                        Block::default()
                            .title(titolo_n)
                            .borders(Borders::ALL)
                            .border_style(stile_n),
                    ),
                form[0],
            );
            f.render_widget(
                Paragraph::new(app.b_desc.as_str())
                    .fg(app.tema.testo)
                    .block(
                        Block::default()
                            .title(" Descrizione ")
                            .borders(Borders::ALL)
                            .border_style(s(1)),
                    ),
                form[1],
            );
            f.render_widget(
                Paragraph::new(app.b_ora.as_str()).fg(app.tema.testo).block(
                    Block::default()
                        .title(" Ora (HH:MM) ")
                        .borders(Borders::ALL)
                        .border_style(s(2)),
                ),
                form[2],
            );
            f.render_widget(
                Paragraph::new(format!("{:?} (SPAZIO)", app.b_freq))
                    .fg(app.tema.testo)
                    .block(
                        Block::default()
                            .title(" Ricorrenza ")
                            .borders(Borders::ALL)
                            .border_style(s(3)),
                    ),
                form[3],
            );
            f.render_widget(
                Paragraph::new(format!("{} (SPAZIO)", app.b_notifica.as_str()))
                    .fg(app.tema.testo)
                    .block(
                        Block::default()
                            .title(" Notifica Anticipata ")
                            .borders(Borders::ALL)
                            .border_style(s(4)),
                    ),
                form[4],
            );
            let testo_suono = if app.b_suono {
                "[X] Attivo (SPAZIO)"
            } else {
                "[ ] Disattivato (SPAZIO)"
            };

            f.render_widget(
                Paragraph::new(testo_suono).fg(app.tema.testo).block(
                    Block::default()
                        .title(" Suono Notifica ")
                        .borders(Borders::ALL)
                        .border_style(s(5)), // Assegna il focus all'indice 5
                ),
                form[5],
            );
            f.render_widget(
                Paragraph::new("Stai editando un evento")
                    .fg(app.tema.testo)
                    .block(Block::default().title(" Editor ").borders(Borders::ALL)),
                corpo[0],
            );

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
        StatoApp::Conferma => {
            let oggi: Vec<&Evento> = eventi
                .iter()
                .filter(|e| e.appare_il(app.data_sel))
                .collect();
            let mut linee: Vec<Line> = Vec::new();
            for (i, ev) in oggi.iter().enumerate() {
                let stile = if i == app.focus_index {
                    Style::default().fg(app.tema.bordo_attivo)
                } else {
                    Style::default().fg(app.tema.testo)
                };
                let prefisso = if i == app.focus_index { ">> " } else { "   " };
                linee.push(Line::from(vec![
                    Span::styled(prefisso, stile),
                    Span::styled(ev.nome.clone(), stile),
                ]));
            }
            f.render_widget(
                Paragraph::new(linee).block(
                    Block::default()
                        .title(" Seleziona Evento ")
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(app.tema.bordo_normale)),
                ),
                corpo[0],
            );

            let avviso = "\n\n  Sei sicuro di voler eliminare questo evento?\n\n  Premi 'S' o INVIO per confermare\n  Premi 'N' o ESC per annullare";
            f.render_widget(
                Paragraph::new(avviso)
                    .alignment(ratatui::layout::Alignment::Center)
                    .style(Style::default().fg(app.tema.testo))
                    .block(
                        Block::default()
                            .title(" ATTENZIONE ")
                            .borders(Borders::ALL)
                            .border_style(Style::default().fg(app.tema.bordo_normale)),
                    ),
                corpo[1],
            );
        }
        StatoApp::Selezione => {
            let popup_layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Percentage(10),
                    Constraint::Percentage(80),
                    Constraint::Percentage(10),
                ])
                .split(area);

            let inner_layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Percentage(15),
                    Constraint::Percentage(70),
                    Constraint::Percentage(15),
                ])
                .split(popup_layout[1]);

            let area_popup = inner_layout[1];

            let items: Vec<ListItem> = app
                .file_picker
                .dir_items
                .iter()
                .map(|path| {
                    let is_dir = path.is_dir();
                    let nome_file = path.file_name().unwrap_or_default().to_string_lossy();

                    let (icona, nome_display) = if nome_file == ".." {
                        ("🔙", String::from(".. (Cartella Superiore)"))
                    } else if is_dir {
                        ("📁", nome_file.into_owned())
                    } else {
                        ("📄", nome_file.into_owned())
                    };

                    let style = if is_dir
                        || path
                            .extension()
                            .map_or(false, |e| e == "ics" || e == "ical")
                    {
                        Style::default().fg(app.tema.testo)
                    } else {
                        Style::default().fg(Color::DarkGray)
                    };

                    ListItem::new(format!("{} {}", icona, nome_display)).style(style)
                })
                .collect();

            let mut state_clone = app.file_picker.list_state.clone();

            let list = List::new(items)
                .block(
                    Block::default()
                        .title(format!(
                            " Scegli un file iCal - [{}] ",
                            app.file_picker.current_dir.display()
                        ))
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(app.tema.bordo_attivo)),
                )
                .highlight_style(
                    Style::default()
                        .bg(app.tema.bordo_attivo)
                        .fg(app.tema.testo)
                        .add_modifier(ratatui::style::Modifier::BOLD),
                )
                .highlight_symbol(">> ");

            f.render_widget(Clear, area_popup);
            f.render_stateful_widget(list, area_popup, &mut state_clone);
        }
    }

    let aiuti = match app.stato {
        StatoApp::Normale => {
            "Q: Esci | N: Nuovo | I:importa | INVIO: Dettagli | Frecce: Naviga | F5: Ricarica tema"
        }
        StatoApp::Dettaglio => {
            "ESC: Torna | N: Nuovo | E: Esporta | /: Scorri | D: Elimina | M: Modifica"
        }
        StatoApp::Creazione | StatoApp::Modifica => {
            "TAB: Campo | SPAZIO: Cambia | INVIO: Salva | ESC: Annulla"
        }
        StatoApp::Conferma => "S/Y/INVIO: Conferma | N/ESC: Annulla",
        StatoApp::Selezione => "INVIO: Apri Cartella/Seleziona | ESC: Annulla | /: Scorri",
    };
    f.render_widget(
        Paragraph::new(aiuti).fg(app.tema.testo).block(
            Block::default()
                .title(" Comandi ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(app.tema.bordo_normale)),
        ),
        main_l[1],
    );
    if let Some(msg) = &app.popup_msg {
        let area_popup = ratatui::layout::Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints([
                ratatui::layout::Constraint::Percentage(35),
                ratatui::layout::Constraint::Length(8),
                ratatui::layout::Constraint::Percentage(35),
            ])
            .split(area)[1];

        let inner_popup = ratatui::layout::Layout::default()
            .direction(ratatui::layout::Direction::Horizontal)
            .constraints([
                ratatui::layout::Constraint::Percentage(20),
                ratatui::layout::Constraint::Percentage(60),
                ratatui::layout::Constraint::Percentage(20),
            ])
            .split(area_popup)[1];

        let paragraph = Paragraph::new(msg.as_str())
            .alignment(ratatui::layout::Alignment::Center)
            .style(Style::default().fg(app.tema.testo))
            .block(
                Block::default()
                    .title(" Avviso (Premi un tasto) ")
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(app.tema.bordo_attivo)),
            );

        f.render_widget(Clear, inner_popup);
        f.render_widget(paragraph, inner_popup);
    }
}
