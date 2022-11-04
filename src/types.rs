#![allow(missing_docs)]

use std::fmt::Display;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use url::Url;

use crate::api::{
    common::{Mode, Region, Tier},
    events::EventId,
    games::GameId,
    matches::MatchId,
    players::PlayerId,
    stages::StageId,
    teams::TeamId,
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Event<'a> {
    #[serde(rename = "_id")]
    pub id: EventId<'a>,
    pub slug: String,
    pub name: Option<String>,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub region: Region,
    pub mode: Mode,
    pub prize: Option<Prize>,
    pub tier: Tier,
    pub image: Option<Url>,
    #[serde(default)]
    pub stages: Vec<Stage>,
    #[serde(default)]
    pub groups: Vec<String>,
}

impl<'a> From<Event<'a>> for EventId<'a> {
    fn from(value: Event<'a>) -> Self {
        value.id
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Prize {
    pub amount: f64,
    pub currency: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stage {
    #[serde(rename = "_id")]
    pub id: StageId,
    pub name: String,
    pub format: Option<String>,
    pub region: Option<Region>,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub liquipedia: Option<Url>,
    #[serde(default)]
    pub substages: Vec<Substage>,
    pub prize: Option<Prize>,
    #[serde(default)]
    pub qualifier: bool,
    #[serde(default)]
    pub lan: bool,
    pub location: Option<Location>,
}

impl From<Stage> for StageId {
    fn from(value: Stage) -> Self {
        value.id
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Substage {
    #[serde(rename = "_id")]
    pub id: SubstageId,
    pub name: String,
    pub format: Option<String>,
}

impl From<Substage> for SubstageId {
    fn from(value: Substage) -> Self {
        value.id
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Location {
    pub venue: Option<String>,
    pub city: Option<String>,
    pub country: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Match<'a> {
    #[serde(rename = "_id")]
    pub id: MatchId<'a>,
    pub slug: String,
    #[serde(rename = "octane_id")]
    pub octane_id: Option<String>,
    pub event: Event<'a>,
    pub stage: Stage,
    pub date: Option<DateTime<Utc>>,
    pub format: Option<Format>,
    pub blue: Option<Side<'a>>,
    pub orange: Option<Side<'a>>,
    pub number: Option<i64>,
    #[serde(default)]
    pub games: Vec<GameScore<'a>>,
    pub reverse_sweep_attempt: Option<bool>,
    pub reverse_sweep: Option<bool>,
}

impl<'a> From<Match<'a>> for MatchId<'a> {
    fn from(value: Match<'a>) -> Self {
        value.id
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum Format {
    #[serde(rename = "best")]
    BestOf {
        length: i64,
    },
    Set {
        length: i64,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Side<'a> {
    pub score: Option<i64>,
    #[serde(default)]
    pub winner: bool,
    pub match_winner: Option<bool>,
    pub team: Option<TeamInfo<'a>>,
    #[serde(default)]
    pub players: Vec<PlayerInfo<'a>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TeamInfo<'a> {
    pub team: Team<'a>,
    pub stats: Option<TeamStats>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Team<'a> {
    #[serde(rename = "_id")]
    pub id: TeamId<'a>,
    pub slug: Option<String>,
    pub name: String,
    pub image: Option<Url>,
    pub region: Option<Region>,
    #[serde(default)]
    pub relevant: bool,
}

impl<'a> From<Team<'a>> for TeamId<'a> {
    fn from(value: Team<'a>) -> Self {
        value.id
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TeamStats {
    pub core: CoreStats,
    pub boost: Option<TeamBoostStats>,
    pub ball: Option<BallStats>,
    pub movement: Option<TeamMovementStats>,
    pub positioning: Option<PositioningStats>,
    pub demo: Option<DemoStats>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CoreStats {
    pub shots: i64,
    pub goals: i64,
    pub saves: i64,
    pub assists: i64,
    pub score: f64, /* Only integer values are possible in RL, but there are
                     * errenous float values in the Octane.gg database
                     */
    pub shooting_percentage: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TeamBoostStats {
    pub bpm: i64,
    pub bcpm: f64,
    pub avg_amount: f64,
    pub amount_collected: i64,
    pub amount_stolen: i64,
    pub amount_collected_big: i64,
    pub amount_stolen_big: i64,
    pub amount_collected_small: i64,
    pub amount_stolen_small: i64,
    pub count_collected_big: i64,
    pub count_stolen_big: i64,
    pub count_collected_small: i64,
    pub count_stolen_small: i64,
    pub amount_overfill: i64,
    pub amount_overfill_stolen: i64,
    pub amount_used_while_supersonic: i64,
    pub time_zero_boost: f64,
    pub time_full_boost: f64,
    pub time_boost_0_to_25: f64,
    pub time_boost_25_to_50: f64,
    pub time_boost_50_to_75: f64,
    pub time_boost_75_to_100: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TeamMovementStats {
    pub total_distance: i64,
    pub time_supersonic_speed: f64,
    pub time_boost_speed: f64,
    pub time_slow_speed: f64,
    pub time_ground: f64,
    pub time_low_air: f64,
    pub time_high_air: f64,
    pub time_powerslide: f64,
    pub count_powerslide: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PositioningStats {
    pub time_defensive_third: f64,
    pub time_neutral_third: f64,
    pub time_offensive_third: f64,
    pub time_defensive_half: f64,
    pub time_offensive_half: f64,
    pub time_behind_ball: f64,
    pub time_infront_ball: f64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DemoStats {
    pub inflicted: i64,
    pub taken: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerInfo<'a> {
    pub player: Player<'a>,
    pub stats: PlayerStats,
    pub advanced: AdvancedStats,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Player<'a> {
    #[serde(rename = "_id")]
    pub id: PlayerId<'a>,
    pub slug: Option<String>,
    pub tag: String,
    pub country: Option<String>,
    pub name: Option<String>,
    #[serde(default)]
    pub accounts: Vec<Account>,
    #[serde(default)]
    pub relevant: bool,
    pub team: Option<Team<'a>>,
    #[serde(default)]
    pub substitute: bool,
    #[serde(default)]
    pub coach: bool,
}

impl<'a> From<Player<'a>> for PlayerId<'a> {
    fn from(value: Player<'a>) -> Self {
        value.id
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    pub platform: Option<String>,
    pub id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerStats {
    pub core: CoreStats,
    pub boost: Option<PlayerBoostStats>,
    pub movement: Option<PlayerMovementStats>,
    pub positioning: Option<PlayerPositioningStats>,
    pub demo: Option<DemoStats>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerBoostStats {
    pub bpm: i64,
    pub bcpm: f64,
    pub avg_amount: f64,
    pub amount_collected: i64,
    pub amount_stolen: i64,
    pub amount_collected_big: i64,
    pub amount_stolen_big: i64,
    pub amount_collected_small: i64,
    pub amount_stolen_small: i64,
    pub count_collected_big: i64,
    pub count_stolen_big: i64,
    pub count_collected_small: i64,
    pub count_stolen_small: i64,
    pub amount_overfill: i64,
    pub amount_overfill_stolen: i64,
    pub amount_used_while_supersonic: i64,
    pub time_zero_boost: f64,
    pub percent_zero_boost: f64,
    pub time_full_boost: f64,
    pub percent_full_boost: f64,
    pub time_boost_0_to_25: f64,
    pub time_boost_25_to_50: f64,
    pub time_boost_50_to_75: f64,
    pub time_boost_75_to_100: f64,
    pub percent_boost_0_to_25: f64,
    pub percent_boost_25_to_50: f64,
    pub percent_boost_50_to_75: f64,
    pub percent_boost_75_to_100: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerMovementStats {
    pub avg_speed: i64,
    pub total_distance: i64,
    pub time_supersonic_speed: f64,
    pub time_boost_speed: f64,
    pub time_slow_speed: f64,
    pub time_ground: f64,
    pub time_low_air: f64,
    pub time_high_air: f64,
    pub time_powerslide: f64,
    pub count_powerslide: i64,
    pub avg_powerslide_duration: f64,
    pub avg_speed_percentage: f64,
    pub percent_slow_speed: f64,
    pub percent_boost_speed: f64,
    pub percent_supersonic_speed: f64,
    pub percent_ground: f64,
    pub percent_low_air: f64,
    pub percent_high_air: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerPositioningStats {
    pub avg_distance_to_ball: i64,
    pub avg_distance_to_ball_possession: i64,
    pub avg_distance_to_ball_no_possession: i64,
    pub avg_distance_to_mates: i64,
    pub time_defensive_third: f64,
    pub time_neutral_third: f64,
    pub time_offensive_third: f64,
    pub time_defensive_half: f64,
    pub time_offensive_half: f64,
    pub time_behind_ball: f64,
    pub time_infront_ball: f64,
    pub time_most_back: f64,
    pub time_most_forward: f64,
    pub goals_against_while_last_defender: i64,
    pub time_closest_to_ball: f64,
    pub time_farthest_from_ball: f64,
    pub percent_defensive_third: f64,
    pub percent_offensive_third: f64,
    pub percent_neutral_third: f64,
    pub percent_defensive_half: f64,
    pub percent_offensive_half: f64,
    pub percent_behind_ball: f64,
    pub percent_infront_ball: f64,
    pub percent_most_back: f64,
    pub percent_most_forward: f64,
    pub percent_closest_to_ball: f64,
    pub percent_farthest_from_ball: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdvancedStats {
    pub goal_participation: f64,
    pub rating: Option<f64>,
    #[serde(default)]
    pub mvp: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameScore<'a> {
    #[serde(rename = "_id")]
    pub id: Option<GameId<'a>>,
    pub blue: i64,
    pub orange: i64,
    pub duration: Option<i64>,
    pub ballchasing: Option<String>,
    #[serde(default)]
    pub overtime: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Game<'a> {
    #[serde(rename = "_id")]
    pub id: GameId<'a>,
    #[serde(rename = "octane_id")]
    pub octane_id: Option<String>,
    pub number: i64,
    #[serde(rename = "match")]
    pub match_field: Match<'a>,
    pub map: Option<Map>,
    pub duration: Option<i64>,
    pub date: Option<DateTime<Utc>>,
    pub blue: Side<'a>,
    pub orange: Side<'a>,
    pub ballchasing: Option<String>,
    pub overtime: Option<bool>,
    pub flip_ballchasing: Option<bool>,
}

impl<'a> From<Game<'a>> for GameId<'a> {
    fn from(value: Game<'a>) -> Self {
        value.id
    }
}

// Make an enum with valid maps?
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Map {
    pub name: Option<String>,
    pub id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BallStats {
    pub possession_time: f64,
    pub time_in_side: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Record<'a> {
    pub game: Game<'a>,
    pub team: Team<'a>,
    pub opponent: Team<'a>,
    pub winner: bool,
    pub player: Player<'a>,
    pub stat: f64,
}

/// Represents a substage ID
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Hash)]
pub struct SubstageId(u32);

impl<T> From<T> for SubstageId
where
    T: Into<u32>,
{
    fn from(id: T) -> Self {
        Self(id.into())
    }
}

impl Display for SubstageId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.0)
    }
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq, Serialize)]
pub struct Participant<'a> {
    pub team: Team<'a>,
    pub players: Vec<Player<'a>>,
}
