#[macro_use] extern crate rocket;

use rocket::fs::{FileServer, relative, NamedFile};
use rocket::http::{Cookie, CookieJar};
use rocket::form::Form;
use rocket::serde::{Serialize, Deserialize};
use rocket::{get, post, routes, launch, Build, Rocket, State};
use std::collections::HashMap;
use tokio::sync::Mutex;
use std::path::Path;

#[derive(FromForm)]
struct Login {
    name: String,
    password: String,
}

#[derive(Serialize, Deserialize, Clone, FromForm)]
struct User {
    name: String,
    password: String,
    poop_count: u32,
}

type UserMap = Mutex<HashMap<String, User>>;

#[get("/")]
async fn index(cookies: &CookieJar<'_>, users: &State<UserMap>) -> Option<NamedFile> {
    if let Some(cookie) = cookies.get("username") {
        let username = cookie.value();
        let users = users.lock().await;
        if users.contains_key(username) {
            return NamedFile::open(Path::new("../frontend/static/index.html")).await.ok();
        }
    }
    NamedFile::open(Path::new("../frontend/static/login.html")).await.ok()
}

#[post("/login", data = "<login>")]
async fn login(cookies: &CookieJar<'_>, login: Form<Login>, users: &State<UserMap>) -> &'static str {
    let users = users.lock().await;
    if let Some(user) = users.get(&login.name) {
        if user.password == login.password {
            cookies.add(Cookie::new("username", user.name.clone()));
            return "Login successful";
        }
    }
    "Invalid name or password"
}

#[post("/poop")]
async fn record_poop(cookies: &CookieJar<'_>, users: &State<UserMap>) -> &'static str {
    if let Some(cookie) = cookies.get("username") {
        let username = cookie.value();
        let mut users = users.lock().await;
        if let Some(user) = users.get_mut(username) {
            user.poop_count += 1;
            return "Poop recorded";
        }
    }
    "User not logged in"
}

#[get("/data")]
async fn get_data(cookies: &CookieJar<'_>, users: &State<UserMap>) -> Option<String> {
    if let Some(cookie) = cookies.get("username") {
        let username = cookie.value();
        let users = users.lock().await;
        if let Some(user) = users.get(username) {
            let daily_avg = user.poop_count as f32 / 7.0;
            let leaderboard: Vec<_> = users.values().cloned().collect();
            let leaderboard = serde_json::to_string(&leaderboard).unwrap();
            let data = serde_json::json!({
                "daily_avg": daily_avg,
                "weekly_poop": user.poop_count,
                "leaderboard": leaderboard,
            });
            return Some(data.to_string());
        }
    }
    None
}

#[post("/add_user", data = "<user>")]
async fn add_user(user: Form<User>, users: &State<UserMap>) -> &'static str {
    let mut users = users.lock().await;
    users.insert(user.name.clone(), user.into_inner());
    "User added"
}

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build()
        .manage(Mutex::new(HashMap::<String, User>::new()))
        .mount("/", routes![index, login, record_poop, get_data, add_user])
        .mount("/static", FileServer::from(relative!("../frontend/static")))
}