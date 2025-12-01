# ⚡ Energy Drink Tracker

Dette er en **Energy Drink Tracker** webapplikation bygget med **Rust (Rocket)** og **Tailwind CSS**. 

Formålet er simpelt: Hold styr på dit (og dine venners) energidrik-forbrug i et intenst Cyberpunk-tema.

![Theme Preview](https://img.shields.io/badge/Theme-Cyberpunk_Neon-success)

## Funktioner

*   **Bruger System**: Opret brugere og log ind sikkert.
*   **Live Tracking**: Registrer hver gang du drikker en energidrik.
*   **Leaderboard**: Se hvem der har drukket flest energidrikke i realtid.
*   **Statistik**: Følg dit daglige gennemsnit og ugentlige total.
*   **Responsivt Design**: Virker på både desktop og mobil med et lækkert neon-interface.

## Teknologier

*   **Backend**: Rust (Rocket Framework)
    *   Håndterer API, sessions (cookies) og in-memory database.
*   **Frontend**: HTML, JavaScript & Tailwind CSS (via CDN)
    *   Ingen tunge frontend-frameworks, bare ren performance.

## Installation og Kørsel

Følg disse trin for at køre projektet lokalt.

### Forudsætninger
*   [Rust & Cargo](https://www.rust-lang.org/tools/install)
*   [Node.js](https://nodejs.org/) (Kun nødvendigt, hvis du vil ændre i CSS build-processen, ellers valgfrit)

### Start Projektet

1.  **Start Backend Serveren:**

    Gå ind i `backend`-mappen og kør:
    ```bash
    cd backend
    cargo run
    ```
    *Dette vil kompilere Rust-koden og starte serveren.*

2.  **Åbn Appen:**

    Når serveren skriver `Rocket has launched...`, åbn din browser og gå til:
    
    👉 **http://127.0.0.1:8000**

### Sådan bruger du den
1.  Opret en ny bruger på startsiden.
2.  Log ind med din nye bruger.
3.  Tryk på den store **DRINK ENERGY** knap, når du bunder en dåse.
4.  Se din rangering stige på leaderboardet!

## Projektstruktur

*   `backend/src/main.rs`: Al backend-logik (API endpoints, bruger-structs).
*   `frontend/static/`:
    *   `index.html`: Hovedsiden (Dashboard).
    *   `login.html`: Login og registrering.
    *   `styles.css`: Tailwind styles.

---
*Bemærk: Data gemmes kun i serverens hukommelse (RAM). Hvis du genstarter serveren, nulstilles tælleren og brugerne.*
