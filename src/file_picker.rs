use ratatui::widgets::ListState;
use std::{fs, path::PathBuf};

pub struct FilePicker {
    pub current_dir: PathBuf,
    pub dir_items: Vec<PathBuf>,
    pub list_state: ListState,
    pub selected_file: Option<PathBuf>,
}

impl FilePicker {
    pub fn new() -> Self {
        let start_dir = std::env::var("HOME")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from("/"));

        let mut picker = Self {
            current_dir: start_dir,
            dir_items: Vec::new(),
            list_state: ListState::default(),
            selected_file: None,
        };

        picker.carica_cartella();
        picker
    }

    pub fn carica_cartella(&mut self) {
        self.dir_items.clear();

        if self.current_dir.parent().is_some() {
            self.dir_items.push(self.current_dir.join(".."));
        }

        if let Ok(entries) = fs::read_dir(&self.current_dir) {
            let mut paths: Vec<PathBuf> = entries
                .filter_map(|e| e.ok())
                .map(|e| e.path())
                .filter(|path| {
                    let file_name = path.file_name().unwrap_or_default().to_string_lossy();
                    !file_name.starts_with('.')
                })
                .collect();

            paths.sort_by(|a, b| {
                let a_is_dir = a.is_dir();
                let b_is_dir = b.is_dir();
                if a_is_dir && !b_is_dir {
                    std::cmp::Ordering::Less
                } else if !a_is_dir && b_is_dir {
                    std::cmp::Ordering::Greater
                } else {
                    a.file_name().cmp(&b.file_name())
                }
            });

            self.dir_items.extend(paths);
        }

        self.list_state.select(if self.dir_items.is_empty() {
            None
        } else {
            Some(0)
        });
    }

    pub fn prossimo_elemento(&mut self) {
        let i = match self.list_state.selected() {
            Some(i) => {
                if i >= self.dir_items.len().saturating_sub(1) {
                    0 // Torna in cima se siamo alla fine
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
    }

    pub fn elemento_precedente(&mut self) {
        let i = match self.list_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.dir_items.len().saturating_sub(1)
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
    }
}
