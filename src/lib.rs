use crate::game::Game;
use crate::schedule::Schedule;
use crate::teams::Team;
use crate::timezone::Timezone;
use std::ops::Sub;
use time::format_description::well_known;
use time::{OffsetDateTime, UtcOffset};

mod game;
mod schedule;
pub mod teams;
pub mod timezone;

pub async fn get_nhl_times(date: &str, timezone: Timezone, team: Team) -> Vec<String> {
    let json_raw = reqwest::get(format!(
        "https://statsapi.web.nhl.com/api/v1/schedule?language=en&date={}&hydrate=game,broadcasts",
        date
    ))
    .await
    .unwrap()
    .text()
    .await
    .unwrap();

    let mut output = Vec::new();

    let schedule = serde_json::from_str::<Schedule>(&json_raw).unwrap();
    for schedule_date in schedule.dates {
        for schedule_game in schedule_date.games {
            let away: Team = schedule_game.teams.away.team.id.try_into().unwrap();
            let home: Team = schedule_game.teams.home.team.id.try_into().unwrap();
            if team == Team::All || team == away || team == home {
                if schedule_game.status.detailed_state != *"Final" {
                    output.push(format!(
                        "{} at {},{},{},{},{},{},{},{},{}",
                        away,
                        home,
                        schedule_game.status.detailed_state,
                        " ",
                        " ",
                        " ",
                        " ",
                        " ",
                        " ",
                        " ",
                    ));
                    continue;
                }

                let json_raw = reqwest::get(format!(
                    "https://statsapi.web.nhl.com/api/v1/game/{}/feed/live",
                    schedule_game.game_pk
                ))
                .await
                .unwrap()
                .text()
                .await
                .unwrap();

                if let Ok(game) = serde_json::from_str::<Game>(&json_raw) {
                    let start_time = OffsetDateTime::parse(
                        &game.game_data.datetime.date_time,
                        &well_known::Rfc3339,
                    )
                    .unwrap()
                    .to_offset(UtcOffset::from_hms(timezone.into(), 0, 0).unwrap());
                    if start_time.date().to_string() != *date {
                        continue;
                    }
                    let end_time = OffsetDateTime::parse(
                        &game.game_data.datetime.end_date_time,
                        &well_known::Rfc3339,
                    )
                    .unwrap()
                    .to_offset(UtcOffset::from_hms(timezone.into(), 0, 0).unwrap());
                    let game_duration = end_time.sub(start_time);
                    let venue_start_time = start_time.to_offset(
                        UtcOffset::from_hms(game.game_data.teams.home.venue.time_zone.offset, 0, 0)
                            .unwrap(),
                    );
                    let venue_end_time = end_time.to_offset(
                        UtcOffset::from_hms(game.game_data.teams.home.venue.time_zone.offset, 0, 0)
                            .unwrap(),
                    );
                    let broadcasts = schedule_game
                        .broadcasts
                        .iter()
                        .map(|b| format!("{} ({})", b.name, b.broadcast_type))
                        .collect::<Vec<String>>()
                        .join(". ");
                    output.push(format!(
                        "{} at {},{},{},{},{},{},{},{}",
                        away,
                        home,
                        format_args!("{}", start_time.date()),
                        format_args!(
                            "{:0>2}:{:0>2} {}",
                            venue_start_time.hour(),
                            venue_start_time.minute(),
                            game.game_data.teams.home.venue.time_zone.tz
                        ),
                        format_args!(
                            "{:0>2}:{:0>2} {}",
                            venue_end_time.hour(),
                            venue_end_time.minute(),
                            game.game_data.teams.home.venue.time_zone.tz
                        ),
                        format_args!(
                            "{}:{:0>2}",
                            game_duration.whole_hours(),
                            game_duration.whole_minutes() % 60
                        ),
                        format_args!(
                            "{:0>2}:{:0>2} {}",
                            start_time.hour(),
                            start_time.minute(),
                            timezone
                        ),
                        format_args!(
                            "{:0>2}:{:0>2} {}",
                            end_time.hour(),
                            end_time.minute(),
                            timezone
                        ),
                        broadcasts
                    ));
                }
            }
        }
    }
    output
}
