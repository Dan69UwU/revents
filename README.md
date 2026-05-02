
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
   cp sound.wav ~/Agenda/
   ```

3. **Attiva le notifiche:**
   Aggiungi il demone al tuo script di avvio (es. script di `dwl`, `.bash_profile` o `.xinitrc`):
   ```bash
   ~/Agenda/daemon &
   ```
   *Testato con mako tramite il comando send-notify e nerd fonts.*

## 🎨 Personalizzazione: Colori e Suoni

L'applicazione supporta un tema personalizzato e suoni di notifica custom. Puoi configurarli creando un file chiamato `config.toml` nella stessa cartella dei tuoi eseguibili (es. `~/Agenda/config.toml`).

Aggiungi questa struttura al file per definire l'audio e la tua palette di colori:
```toml
# --- IMPOSTAZIONI GENERALI ---
# Percorso del file audio personalizzato per le notifiche.
# Puoi usare un percorso relativo all'eseguibile ("notifica.wav") o assoluto ("/home/utente/Musica/beep.wav").
percorso_audio = "sound.wav"

[tema]
# --- GUIDA AI COLORI ---
# Puoi usare i nomi standard (Black, Red, Green, Yellow, Blue, Magenta, Cyan, Gray, DarkGray, ecc.)
# Oppure puoi usare i codici esadecimali (es. "#ff5555")

# Colore per i messaggi di errore e i campi obbligatori vuoti
errore = "Red"

# Colore dei bordi quando il riquadro non è attivo
bordo_normale = "DarkGray"

# Colore dei bordi quando il riquadro ha il focus (es. durante l'editing)
bordo_attivo = "Yellow"
  
# Colore del testo 
testo = "White"
```

* **File audio di default:** Se non usi il `config.toml`, il daemon cercherà automaticamente un file chiamato `sound.wav` nella cartella in cui si trova l'eseguibile.
* 💡 **Suggerimento rapido:** Premi **F5** mentre usi `revents` per ricaricare istantaneamente i colori senza dover riavviare il programma!

## ⌨️ Utilizzo

Lancia l'interfaccia aprendo il terminale e digitando `~/Agenda/revents`.

**Vista Calendario (Normale):**
* **Frecce / Tab**: Naviga tra giorni e settimane.
* **n**: Crea un nuovo evento nella data selezionata.
* **i**: Importa un calendario da un file `.ics`.
* **Invio**: Apri i dettagli degli eventi di quella giornata.
* **F5**: Ricarica il tema dal file di configurazione.
* **q** / **Esc**: Esci dall'applicazione.

**Pannello Creazione/Modifica:**
* **Frecce Su/Giù** / **Tab**: Spostati tra i campi (Nome, Descrizione, Ora, ecc.).
* **Frecce Destra/Sinistra** / **Spazio**: Cambia i valori ciclici (Ricorrenza, Anticipo Notifica, Suono).
* **Invio**: Salva l'evento.

**Pannello Dettagli Evento:**
* **Frecce Su/Giù**: Scorri la lista degli eventi in quel giorno.
* **n**: Crea rapidamente un nuovo evento per quel giorno.
* **m**: Modifica l'evento selezionato.
* **e**: Esporta l'evento selezionato in un file `.ics` (verrà salvato automaticamente nella tua cartella Home con il nome dell'evento).
* **d**: Elimina l'evento selezionato.

## 📝 TODO (Roadmap)
Funzioni che verranno aggiunte in seguito:
* [x] Supporto all'esportazione/importazione in formato iCal (`.ics`).
* [ ] Tasti completamente configurabili dall'utente.
