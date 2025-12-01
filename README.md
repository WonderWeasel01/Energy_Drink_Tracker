# ⚡ Energy Drink Tracker

This is an **Energy Drink Tracker** web application built with **Rust (Rocket)** and **Tailwind CSS**.

The goal is simple: Keep track of your (and your friends') energy drink consumption in an intense Cyberpunk theme.

![Theme Preview](https://img.shields.io/badge/Theme-Cyberpunk_Neon-success)

## Features

*   **User System**: Create users and log in securely.
*   **Live Tracking**: Record every time you drink an energy drink.
*   **Leaderboard**: See who has consumed the most energy drinks in real-time.
*   **Statistics**: Track your daily average and weekly total.
*   **Responsive Design**: Works on both desktop and mobile with a sleek neon interface.

## Technologies

*   **Backend**: Rust (Rocket Framework)
    *   Handles API, sessions (cookies), and in-memory database.
*   **Frontend**: HTML, JavaScript & Tailwind CSS (via CDN)
    *   No heavy frontend frameworks, just pure performance.

## Installation and Usage

Follow these steps to run the project locally.

### Prerequisites
*   [Rust & Cargo](https://www.rust-lang.org/tools/install)
*   [Node.js](https://nodejs.org/) (Only needed if you want to modify the CSS build process, otherwise optional)

### Start the Project

1.  **Start the Backend Server:**

    Navigate to the `backend` folder and run:
    ```bash
    cd backend
    cargo run
    ```
    *This will compile the Rust code and start the server.*

2.  **Open the App:**

    When the server says `Rocket has launched...`, open your browser and go to:
    
    👉 **http://127.0.0.1:8000**

### How to Use
1.  Create a new user on the landing page.
2.  Log in with your new user.
3.  Click the big **DRINK ENERGY** button when you finish a can.
4.  Watch your rank rise on the leaderboard!

## Project Structure

*   `backend/src/main.rs`: All backend logic (API endpoints, user structs).
*   `frontend/static/`:
    *   `index.html`: Main page (Dashboard).
    *   `login.html`: Login and registration.
    *   `styles.css`: Tailwind styles.

---
*Note: Data is only stored in the server's memory (RAM). If you restart the server, the counter and users will be reset.*
