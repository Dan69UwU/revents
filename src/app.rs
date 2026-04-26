use crate::config::TemaApp;
use crate::model::{AnticipoNotifica, Frequenza};
use chrono::{Local, NaiveDate};

#[derive(PartialEq)]
pub enum StatoApp {
    Normale,
    Creazione,
    Dettaglio,
    Modifica,
}

pub struct App {
    pub data_sel: NaiveDate,
    pub stato: StatoApp,
    pub tema: TemaApp,
    pub focus_index: usize,
    pub indice_modifica: Option<usize>,

    pub b_nome: String,
    pub b_desc: String,
    pub b_ora: String,
    pub b_freq: Frequenza,
    pub b_notifica: AnticipoNotifica,
    pub b_suono:bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            data_sel: Local::now().date_naive(),
            stato: StatoApp::Normale,
            tema: TemaApp::carica("config.toml"),
            focus_index: 0,
            indice_modifica: None,
            b_nome: String::new(),
            b_desc: String::new(),
            b_ora: String::from("12:00"),
            b_freq: Frequenza::Mai,
            b_notifica: AnticipoNotifica::Nessuna,
            b_suono:false,
        }
    }

    pub fn reset_buffer(&mut self) {
        self.b_nome.clear();
        self.b_desc.clear();
        self.b_ora = String::from("12:00");
        self.b_freq = Frequenza::Mai;
        self.b_notifica = AnticipoNotifica::Nessuna;
        self.focus_index = 0;
        self.b_suono=false;

    }
}
