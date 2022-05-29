
use chrono;

#[get("/")]
pub fn datetime() -> String {
    format!("{:?}", chrono::offset::Utc::now())
}
