extern crate chrono;
use chrono::prelude::DateTime;
use chrono::Utc;
use reqwest::Client as HttpClient;
use reqwest::Response;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use std::time::{Duration, UNIX_EPOCH};

mod urls {
    pub const ASTROS: &str = "http://api.open-notify.org/astros.json";
    pub const ISS_NOW: &str = "http://api.open-notify.org/iss-now.json";
}

#[derive(Debug)]
struct Location {
    latitude: f64,
    longitude: f64,
}
#[derive(Debug)]
pub struct ISSLocation {
    location: Location,
    date_time: DateTime<Utc>,
    message: String,
}
#[derive(Deserialize, Clone, Debug)]
struct __Location {
    pub latitude: String,
    pub longitude: String,
}
#[derive(Deserialize, Clone, Debug)]
struct __ISSLocation {
    pub message: String,
    pub timestamp: u64,
    pub iss_position: __Location,
}
#[derive(Deserialize, Clone, Debug)]
pub struct Person {
    pub name: String,
    pub craft: String,
}
#[derive(Deserialize, Debug)]
pub struct PeopleInSpace {
    pub message: String,
    pub number: u16,
    pub people: Vec<Person>,
}

async fn get(url: &'static str) -> String {
    let http = HttpClient::new();
    http.get(url)
        .send()
        .await
        .expect("Failed to get response")
        .text()
        .await
        .expect("Failed to get data")
}

pub async fn get_iss_location() -> ISSLocation {
    let data = get(urls::ISS_NOW).await;
    let json = serde_json::from_str::<__ISSLocation>(data.as_str()).unwrap();
    let unix = UNIX_EPOCH + Duration::from_secs(json.timestamp);
    ISSLocation {
        location: Location {
            latitude: json.iss_position.latitude.parse().unwrap(),
            longitude: json.iss_position.longitude.parse().unwrap(),
        },
        message: json.message,
        date_time: DateTime::<Utc>::from(unix),
    }
}

pub async fn get_people_in_space() -> PeopleInSpace {
    let data = get(urls::ASTROS).await;
    serde_json::from_str::<PeopleInSpace>(data.as_str()).unwrap()
}

