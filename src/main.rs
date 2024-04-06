#![doc = include_str!("../README.md")]

use sailfish::TemplateOnce;
use templates::SailfishResponder;

#[macro_use]
extern crate rocket;

mod templates;

/// Returns Hello World
#[get("/")]
fn index() -> SailfishResponder {
    let template = templates::index::IndexTemplate {};
    SailfishResponder(
        template.render_once().expect("Failed to render template"),
    )
}

/// Start the rocket server
#[rocket::main]
#[allow(clippy::redundant_type_annotations)]
async fn main() {
    let rocket = rocket::build().mount("/", routes![index]);
    rocket.launch().await.expect("Server Crashed");
}
