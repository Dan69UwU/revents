# Agenda TUI

R(emember)events è un'agenda minimale da terminale scritta in Rust. Include l'interfaccia principale (`revents`) e un daemon in background (`daemon`) per le notifiche desktop. I dati vengono salvati in un file `agenda.json` locale.

## 🚀 Installazione

1. **Compila il progetto:**
   ```bash
   git clone [https://github.com/Dan69UwU/revents](https://github.com/Dan69UwU/revents)
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
   *Testato con mako tramite il comando send-notify e nerd fonts.*

## 🎨 Personalizzazione Colori

L'interfaccia supporta temi personalizzati. Puoi modificare i colori creando un file chiamato `config.toml` nella stessa cartella dei tuoi eseguibili (es. `~/Agenda/config.toml`).

Aggiungi questa struttura al file per definire la tua palette:

```toml
[tema]
testo = "White"
bordo_normale = "DarkGray"
bordo_attivo = "Yellow"
errore = "Red"
```

**Colori supportati:** Puoi utilizzare i nomi standard supportati dal terminale (es. `Black`, `Red`, `Green`, `Yellow`, `Blue`, `Magenta`, `Cyan`, `Gray`, `DarkGray`, `LightRed`, `LightGreen`, `LightYellow`, `LightBlue`, `LightMagenta`, `LightCyan`, `White`).

💡 **Suggerimento rapido:** Puoi modificare e salvare il file `config.toml` mentre l'applicazione è aperta e premere il tasto **F5** per ricaricare istantaneamente il tema senza dover riavviare il programma!

## ⌨️ Utilizzo

Lancia l'interfaccia aprendo il terminale e digitando `~/Agenda/revents`.

**Comandi principali:**
* **Frecce**: Naviga tra giorni e settimane.
* **n**: Nuovo evento.
* **Invio**: Apri i dettagli di un evento / Salva le modifiche.
* **e** / **d** (nella schermata dettagli): Modifica / Elimina evento.
* **Frecce Su/Giù** (in creazione): Spostati tra i campi.
* **Spazio** (in creazione): Cambia i valori di Ricorrenza e Notifica.
* **F5**: Ricarica il tema dal file di configurazione.
* **q** / **Esc**: Esci.

## 📝 TODO
Funzioni che verranno aggiunte in seguito:
* Tasti configurabili
