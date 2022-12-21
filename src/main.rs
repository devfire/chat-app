#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Rust-based chat app."
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}