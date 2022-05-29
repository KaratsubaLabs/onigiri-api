
#[macro_use]
extern crate rocket;

use rocket::Request;
use rocket::serde::{Serialize, Deserialize};
use rocket::serde::json::Json;

use reqwest::StatusCode;

use chrono;

#[get("/")]
fn index() -> &'static str {
    "hello world"
}

#[get("/")]
fn datetime() -> String {
    format!("{:?}", chrono::offset::Utc::now())
}

const DEFINE_EN_API_URL: &'static str = "https://api.dictionaryapi.dev/api/v2/entries/en";
#[get("/en/<word>")]
async fn define(word: &str) -> Json<Vec<DictionaryEntry>> {
    let definitions = define_en_handler(word).await.unwrap();
    Json(definitions)
}

// TODO make macro to auto make structs from json
#[derive(Debug, Serialize, Deserialize)]
struct DictionaryEntry {
    word: String,
    meanings: Vec<Meaning>,
}
#[derive(Debug, Serialize, Deserialize)]
struct Meaning {
    partOfSpeech: String,
    definitions: Vec<Definition>,
}
#[derive(Debug, Serialize, Deserialize)]
struct Definition {
    definition: String,
}

async fn define_en_handler(word: &str) -> Result<Vec<DictionaryEntry>, reqwest::Error> {

    let resp = reqwest::get(format!("{}/{}", DEFINE_EN_API_URL, word)).await?;
    if resp.status() != StatusCode::OK {
        return Ok(Vec::new());
    }
    let definitions: Vec<DictionaryEntry> = resp.json().await?;
    Ok(definitions)
}

#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("not found :(")
}

#[launch]
fn launch() -> _ {
    rocket::build()
        .mount("/datetime", routes![datetime])
        .mount("/define", routes![define])
        .register("/", catchers![not_found])
}

