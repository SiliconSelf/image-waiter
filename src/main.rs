#![doc = include_str!("../README.md")]

use image_actor::ImageActor;
use sailfish::TemplateOnce;
use templates::SailfishResponder;
use rocket::State;

#[macro_use]
extern crate rocket;

mod templates;
mod image_actor;

/// Returns Hello World
#[get("/")]
fn index(rng: &State<ImageActor>) -> SailfishResponder {
    let template = templates::index::IndexTemplate {
        images: Vec::new()
    };
    SailfishResponder(
        template.render_once().expect("Failed to render template"),
    )
}

/// Start the rocket server
#[rocket::main]
#[allow(clippy::redundant_type_annotations)]
async fn main() {
    let rocket = rocket::build()
        .manage(ImageActor::new())
        .mount("/", routes![index]);
    rocket.launch().await.expect("Server Crashed");
}
