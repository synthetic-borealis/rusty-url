#[macro_use]
extern crate rocket;

use std::net::Ipv4Addr;
use std::path::{Path, PathBuf};

use rocket::Config;
use rocket::fs::{NamedFile, relative};
use rocket::http::Status;
use rocket::response::Redirect;

#[get("/app/<_..>")]
async fn app() -> Option<NamedFile> {
    NamedFile::open("static/index.html").await.ok()
}

#[get("/static/<path..>")]
async fn static_dir(path: PathBuf) -> Option<NamedFile> {
    let mut path = Path::new(relative!("static")).join(path);

    if path.is_dir() {
        path.push("index.html");
    }

    NamedFile::open(path).await.ok()
}

#[get("/<url_id>")]
fn redirect_url(url_id: String) -> Result<String, Status> {
    Ok(format!("Hello, {}", url_id))
}

#[get("/")]
fn home() -> Redirect {
    Redirect::to("/app")
}

#[launch]
fn rocket() -> rocket::Rocket<rocket::Build> {
    let config = Config {
        address: Ipv4Addr::new(0, 0, 0, 0).into(),
        ..Config::default()
    };
    rocket::custom(&config)
        .mount("/", routes![home, app, static_dir, redirect_url])
}
