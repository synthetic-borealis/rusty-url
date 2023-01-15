#[macro_use]
extern crate rocket;

use rocket::http::Status;
use rocket_okapi::{openapi, openapi_get_routes, rapidoc::*, swagger_ui::*};

#[openapi]
#[get("/<url_id>")]
fn redirect_url(url_id: String) -> Result<String, Status> {
    Ok(format!("Hello, {}", url_id))
}

#[openapi]
#[get("/")]
fn home() -> Result<String, Status> {
    Ok("Hello, World!".to_string())
}

#[launch]
fn rocket() -> rocket::Rocket<rocket::Build> {
    rocket::build()
        .mount("/", openapi_get_routes![home, redirect_url])
        .mount(
            "/swagger-ui/",
            make_swagger_ui(&SwaggerUIConfig {
                url: "../openapi.json".to_owned(),
                ..Default::default()
            }),
        )
}
