
#[macro_use]
extern crate rocket;

use chrono;

use rocket::http::Status;
use rocket::response::status;

#[get("/")]
fn index() -> &'static str {
    "hello world"
}

#[get("/")]
fn datetime() -> String {
    format!("{:?}", chrono::offset::Utc::now())
}

const DEFINE_EN_API_URL: &'static str = "https://api.dictionaryapi.dev/api/v2/entries/en";
#[get("/")]
async fn define() -> Status {
    define_en_handler("poop").await;
    Status::Ok
}

async fn define_en_handler(word: &str) -> Result<(), reqwest::Error> {

    println!("en handler");
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_millis(100))
        .build()?;
    let resp = client
        .get(format!("{}/{}", DEFINE_EN_API_URL, word))
        .send().await?;
    println!("response {:?}", resp);
    Ok(())
}


#[launch]
fn launch() -> _ {
    rocket::build()
        .mount("/datetime", routes![datetime])
        .mount("/define", routes![define])
}

