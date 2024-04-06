#![doc = include_str!("../README.md")]

#[macro_use] extern crate rocket;

/// Returns Hello World
#[get("/")]
fn index() -> &'static str {
    "Hello, World!"
}

/// Start the rocket server
#[rocket::main]
async fn main() {
    let rocket = rocket::build()
        .mount("/", routes![index]);
    rocket.launch().await.expect("Server Crashed");
}