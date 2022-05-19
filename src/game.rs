use serde::Deserialize;

#[derive(Deserialize)]
pub struct Game {
    #[serde(rename = "gameData")]
    pub game_data: GameData,
}

#[derive(Deserialize)]
pub struct GameData {
    pub teams: Teams,
    pub datetime: DateTime,
}

#[derive(Deserialize)]
pub struct DateTime {
    #[serde(rename = "dateTime")]
    pub date_time: String,
    #[serde(rename = "endDateTime")]
    pub end_date_time: String,
}

#[derive(Deserialize)]
pub struct Teams {
    pub home: Home,
}

#[derive(Deserialize)]
pub struct Home {
    pub venue: Venue,
}

#[derive(Deserialize)]
pub struct Venue {
    #[serde(rename = "timeZone")]
    pub time_zone: TimeZone,
}

#[derive(Deserialize)]
pub struct TimeZone {
    pub offset: i8,
    pub tz: String,
}
