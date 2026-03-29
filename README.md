# Agenda TUI

R(emember)events è un'agenda minimale da terminale scritta in Rust. Include l'interfaccia principale (`revents`) e un daemon in background (`daemon`) per le notifiche desktop. I dati vengono salvati in un file `agenda.json` locale.

## 🚀 Installazione

1. **Compila il progetto:**
   ```bash
   git clone https://github.com/Dan69UwU/revents
   cd revents
   cargo build --release
   ```

2. **Copia i file:**
   I due eseguibili devono stare nella stessa cartella per condividere i dati.
   ```bash
   mkdir -p ~/Agenda
   cp target/release/revents ~/Agenda/
   cp target/release/daemon ~/Agenda/
   ```

3. **Attiva le notifiche:**
   Aggiungi il demone al tuo script di avvio (es. script di `dwl`, `.bash_profile` o `.xinitrc`):
   ```bash
   ~/Agenda/daemon &
   ```
  testato con make tramite il comando send-notify
  
## ⌨️ Utilizzo

Lancia l'interfaccia aprendo il terminale e digitando `~/Agenda/revents`.

**Comandi principali:**
* **Frecce / Tab**: Naviga tra giorni e settimane.
* **n**: Nuovo evento.
* **Invio**: Apri i dettagli di un evento / Salva le modifiche.
* **e** / **d** (nella schermata dettagli): Modifica / Elimina evento.
* **Frecce Su/Giù** (in creazione): Spostati tra i campi.
* **Spazio** (in creazione): Cambia i valori di Ricorrenza e Notifica.
* **q** / **Esc**: Esci.``
## TODO
funzione che verrano aggiunge in seguito:
* temi personalizzabili
