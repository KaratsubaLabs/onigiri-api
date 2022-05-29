
#[macro_use] extern crate rocket;

mod handler;
mod services;

use services::{
    anime::anime,
    datetime::datetime,
    define::define,
    weather::weather,
};

#[launch]
fn launch() -> _ {
    rocket::build()
        .mount("/anime",routes![anime])
        .mount("/datetime", routes![datetime])
        .mount("/define", routes![define])
        .mount("/weather",routes![weather])
        .register("/", catchers![handler::not_found])
}

