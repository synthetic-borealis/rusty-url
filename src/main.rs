#[macro_use] extern crate rocket;

use rocket::http::Status;

#[get("/")]
fn hello() -> Result<String, Status> {
    Ok("Hello, World!".to_string())
}

#[launch]
fn rocket() -> rocket::Rocket<rocket::Build> {
    rocket::build().mount("/", routes![hello])
}

