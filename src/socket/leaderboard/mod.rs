use std::sync::Arc;
use mongodb::{bson::doc, Cursor};
use num_traits::cast::FromPrimitive;
use redis::{aio::Connection, ToRedisArgs};
use serde::{Serialize, Deserialize};
use strum_macros::{Display, EnumIter, EnumString};
use strum::IntoEnumIterator;

use chrono::{Month, DateTime, Utc, TimeZone, FixedOffset, Datelike};

use crate::{database::{cache::RedisAdapter, Database, models::player::Player}, util::r#macro::unwrap_helper};

pub mod leaderboard_listener;

fn get_est_datetime() -> DateTime<FixedOffset> {
    let naive_utc_time = Utc::now().naive_utc();
    let fixed_offset = FixedOffset::west(4 * 3600); // UTC-4 for EST
    fixed_offset.from_utc_datetime(&naive_utc_time)
}

pub enum Season {
    Spring,
    Summer,
    Autumn,
    Winter
}

impl Season {
    pub fn of_northern(month: Month) -> Season {
        match month {
            Month::March | Month::April => Season::Spring,
            Month::May | Month::June | Month::July | Month::August  => Season::Summer,
            Month::September | Month::October => Season::Autumn,
            Month::November | Month::December | Month::January | Month::February => Season::Winter,
        }
    }

    pub fn name(&self) -> &'static str {
        match &self {
            Season::Spring => "spring",
            Season::Summer => "summer",
            Season::Autumn => "autumn",
            Season::Winter => "winter",
        }
    }
}

#[derive(EnumIter, EnumString)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum LeaderboardPeriod {
    Daily,
    Weekly,
    Monthly,
    Seasonally,
    Yearly,
    AllTime
}

impl LeaderboardPeriod {
    pub fn get_today_id(&self) -> String {
        let date = get_est_datetime();
        match &self {
            Self::Daily => {
                let day = date.day();
                let month = date.month() - 1; // Java being cringe
                let year = date.year();
                format!("{}:d:{}:{}", year, month, day)
            },
            Self::Weekly => {
                let week = date.iso_week().week();
                let year = date.year();
                format!("{}:w:{}", year, week)
            },
            Self::Monthly => {
                let month = date.month() - 1;
                let year = date.year();
                format!("{}:m:{}", year, month)
            },
            Self::Seasonally => {
                let month = date.month() - 1;
                let season = Season::of_northern(Month::from_u32(month + 1).unwrap_or(Month::January)).name();
                let year = date.year();
                format!("{}:s:{}", year, season)
            },
            Self::Yearly => {
                let year = date.year();
                format!("{}:y", year)
            },
            Self::AllTime => String::from("all"),
        }
    }
}

#[derive(Display, EnumString, Serialize, Deserialize, Clone, Eq, Hash, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum ScoreType {
    Kills,
    Deaths,
    FirstBloods,
    Wins,
    Losses,
    Ties,
    Xp,
    MessagesSent,
    MatchesPlayed,
    ServerPlaytime,
    GamePlaytime,
    CoreLeaks,
    CoreBlockDestroys,
    DestroyableDestroys,
    DestroyableBlockDestroys,
    FlagCaptures,
    FlagDrops,
    FlagPickups,
    FlagDefends,
    FlagHoldTime,
    WoolCaptures,
    WoolDrops,
    WoolPickups,
    WoolDefends,
    ControlPointCaptures,
    HighestKillstreak
}

impl ScoreType {
    pub fn to_leaderboard<'a>(&self, lbs: &'a MarsLeaderboards) -> &'a Leaderboard {
        match self {
            ScoreType::Kills => &lbs.kills,
            ScoreType::Deaths => &lbs.deaths,
            ScoreType::FirstBloods => &lbs.first_bloods,
            ScoreType::Wins => &lbs.wins,
            ScoreType::Losses => &lbs.losses,
            ScoreType::Ties => &lbs.ties,
            ScoreType::Xp => &lbs.xp,
            ScoreType::MessagesSent => &lbs.messages_sent,
            ScoreType::MatchesPlayed => &lbs.matches_played,
            ScoreType::ServerPlaytime => &lbs.server_playtime,
            ScoreType::GamePlaytime => &lbs.game_playtime,
            ScoreType::CoreLeaks => &lbs.core_leaks,
            ScoreType::CoreBlockDestroys => &lbs.core_block_destroys,
            ScoreType::DestroyableDestroys => &lbs.destroyable_destroys,
            ScoreType::DestroyableBlockDestroys => &lbs.destroyable_block_destroys,
            ScoreType::FlagCaptures => &lbs.flag_captures,
            ScoreType::FlagDrops => &lbs.flag_drops,
            ScoreType::FlagPickups => &lbs.flag_pickups,
            ScoreType::FlagDefends => &lbs.flag_defends,
            ScoreType::FlagHoldTime => &lbs.flag_hold_time,
            ScoreType::WoolCaptures => &lbs.wool_captures,
            ScoreType::WoolDrops => &lbs.wool_drops,
            ScoreType::WoolPickups => &lbs.wool_pickups,
            ScoreType::WoolDefends => &lbs.wool_defends,
            ScoreType::ControlPointCaptures => &lbs.control_point_captures,
            ScoreType::HighestKillstreak => &lbs.highest_killstreak,
        }
    }
}

pub struct Leaderboard {
    pub score_type: ScoreType,
    pub database: Arc<Database>,
    pub cache: Arc<RedisAdapter>
}


impl Leaderboard {
    async fn zadd_entries<T: ToRedisArgs, K: ToRedisArgs, V: ToRedisArgs>(&self, key: &T, items: &Vec<(K, V)>) {
        let _ = self.cache.submit(|mut conn| async move {
            let _ = redis::cmd("ZADD")
                .arg(key)
                .arg(items)
                .query_async::<Connection, ()>(&mut conn).await;
        }).await;
    }

    pub async fn populate_all_time(&self) {
        let cursor : Cursor<Player> = match self.database.players.find(doc! {}, None).await {
            Ok(player_cursor) => player_cursor,
            Err(_) => return
        };
        let players = {
            let mut players = Database::consume_cursor_into_owning_vec(cursor).await;
            players.sort_by(|a, b| {
                b.stats.get_score(&self.score_type).cmp(&a.stats.get_score(&self.score_type))
            });
            players
        };
        let members = {
            let mut members : Vec<(String, u64)> = Vec::new();
            for player in players.iter() {
                members.push((player.id_name(), player.stats.get_score(&self.score_type) as u64));
            };
            members
        };
        self.zadd_entries(&self.get_id(&LeaderboardPeriod::AllTime), &members).await;
    }

    pub async fn set(&self, id: &String, score: u32) {
        let u64_score = score as u64;
        let _ = self.cache.submit(|mut conn| async move {
            for period in LeaderboardPeriod::iter() {
                let _ = redis::cmd("ZADD").arg(&self.get_id(&period)).arg(u64_score).arg(id).query_async::<Connection, ()>(&mut conn).await;
            };
        }).await;
    }

    pub async fn increment(&self, id: &String, incr: Option<u32>) {
        let u64_incr = incr.unwrap_or(1) as u64;
        let _ = self.cache.submit(|mut conn| async move {
            for period in LeaderboardPeriod::iter() {
                let _ = redis::cmd("ZINCRBY").arg(&self.get_id(&period)).arg(u64_incr).arg(id).query_async::<Connection, ()>(&mut conn).await;
            };
        }).await;
    }

    fn strings_as_leaderboard_entries(raw: Vec<String>) -> Vec<LeaderboardEntry> {
        let mut entries : Vec<LeaderboardEntry> = Vec::new();
        if raw.len() <= 1 || raw.len() % 2 == 1 {
            return entries;
        };
        for i in (0..=(raw.len() - 2)).step_by(2) {
            let id_name = raw[i].clone();
            let score = raw[i + 1].parse::<u32>().unwrap_or(0);
            let (id, name) = {
                let mut parts = id_name.split("/");
                let id = unwrap_helper::continue_default!(parts.next());
                let name = unwrap_helper::continue_default!(parts.next());
                (id, name)
            };
            entries.push(LeaderboardEntry { id: id.to_owned(), name: name.to_owned(), score });
        }
        entries
    }

    pub async fn fetch_top(&self, period: &LeaderboardPeriod, limit: u32) -> Vec<LeaderboardEntry> {
        let lb_top = self.cache.submit(|mut conn| async move {
            let top : Option<Vec<String>> = match redis::cmd("ZRANGE").arg(&self.get_id(period)).arg(0u32).arg(limit - 1).arg("REV").arg("WITHSCORES").query_async::<Connection, Vec<String>>(&mut conn).await {
                Ok(res) => Some(res),
                Err(_) => None
            };
            top.unwrap_or(Vec::new())
        }).await.unwrap_or(Vec::new());
        Self::strings_as_leaderboard_entries(lb_top)
    }

    pub async fn set_if_higher(&self, id: &String, new: u32) {
        let _ = self.cache.submit(|mut conn| async move {
            for period in LeaderboardPeriod::iter() {
                let current = match redis::cmd("ZSCORE").arg(&self.get_id(&period)).arg(id).query_async::<Connection, String>(&mut conn).await {
                    Ok(res) => { res.parse::<u32>().unwrap() },
                    Err(_) => { 0u32 }
                };
                if new > current {
                    redis::cmd("ZADD").arg(&self.get_id(&period)).arg(new as f64).arg(id).query_async::<Connection, ()>(&mut conn).await;
                };
            };
        }).await;
    }

    pub async fn get_position(&self, id: &String, period: &LeaderboardPeriod) -> Option<u64> {
        self.cache.submit(|mut conn| async move {
            let rank : Option<u64> = match redis::cmd("ZREVRANK").arg(&self.get_id(period)).arg(id).query_async::<Connection, u64>(&mut conn).await {
                Ok(res) => Some(res),
                Err(_) => None // this error occurs when redis encounters an issue executing the query
            };
            rank
        }).await.unwrap_or(None) // this unwrap occurs if a connection can't be obtained
    }

    fn get_id(&self, period: &LeaderboardPeriod) -> String {
        format!("lb:{}:{}", self.score_type, period.get_today_id())
    }
}

pub struct MarsLeaderboards {
    pub kills: Leaderboard,
    pub deaths: Leaderboard,
    pub first_bloods: Leaderboard,
    pub wins: Leaderboard,
    pub losses: Leaderboard,
    pub ties: Leaderboard,
    pub xp: Leaderboard,
    pub messages_sent: Leaderboard,
    pub matches_played: Leaderboard,
    pub server_playtime: Leaderboard,
    pub game_playtime: Leaderboard,
    pub core_leaks: Leaderboard,
    pub core_block_destroys: Leaderboard,
    pub destroyable_destroys: Leaderboard,
    pub destroyable_block_destroys: Leaderboard,
    pub flag_captures: Leaderboard,
    pub flag_drops: Leaderboard,
    pub flag_pickups: Leaderboard,
    pub flag_defends: Leaderboard,
    pub flag_hold_time: Leaderboard,
    pub wool_captures: Leaderboard,
    pub wool_drops: Leaderboard,
    pub wool_pickups: Leaderboard,
    pub wool_defends: Leaderboard,
    pub control_point_captures: Leaderboard,
    pub highest_killstreak: Leaderboard
}

impl MarsLeaderboards {
    pub fn new(redis: Arc<RedisAdapter>, database: Arc<Database>) -> Self {
        MarsLeaderboards {
            kills: Leaderboard { score_type: ScoreType::Kills, cache: Arc::clone(&redis), database: Arc::clone(&database) },
            deaths: Leaderboard { score_type: ScoreType::Deaths, cache: Arc::clone(&redis), database: Arc::clone(&database) },
            first_bloods: Leaderboard { score_type: ScoreType::FirstBloods, cache: Arc::clone(&redis), database: Arc::clone(&database) },
            wins: Leaderboard { score_type: ScoreType::Wins, cache: Arc::clone(&redis), database: Arc::clone(&database) },
            losses: Leaderboard { score_type: ScoreType::Losses, cache: Arc::clone(&redis), database: Arc::clone(&database) },
            ties: Leaderboard { score_type: ScoreType::Ties, cache: Arc::clone(&redis), database: Arc::clone(&database) },
            xp: Leaderboard { score_type: ScoreType::Xp, cache: Arc::clone(&redis), database: Arc::clone(&database) },
            messages_sent: Leaderboard { score_type: ScoreType::MessagesSent, cache: Arc::clone(&redis), database: Arc::clone(&database) },
            matches_played: Leaderboard { score_type: ScoreType::MatchesPlayed, cache: Arc::clone(&redis), database: Arc::clone(&database) },
            server_playtime: Leaderboard { score_type: ScoreType::ServerPlaytime, cache: Arc::clone(&redis), database: Arc::clone(&database) },
            game_playtime: Leaderboard { score_type: ScoreType::GamePlaytime, cache: Arc::clone(&redis), database: Arc::clone(&database) },
            core_leaks: Leaderboard { score_type: ScoreType::CoreLeaks, cache: Arc::clone(&redis), database: Arc::clone(&database) },
            core_block_destroys: Leaderboard { score_type: ScoreType::CoreBlockDestroys, cache: Arc::clone(&redis), database: Arc::clone(&database) },
            destroyable_destroys: Leaderboard { score_type: ScoreType::DestroyableDestroys, cache: Arc::clone(&redis), database: Arc::clone(&database) },
            destroyable_block_destroys: Leaderboard { score_type: ScoreType::DestroyableBlockDestroys, cache: Arc::clone(&redis), database: Arc::clone(&database) },
            flag_captures: Leaderboard { score_type: ScoreType::FlagCaptures, cache: Arc::clone(&redis), database: Arc::clone(&database) },
            flag_drops: Leaderboard { score_type: ScoreType::FlagDrops, cache: Arc::clone(&redis), database: Arc::clone(&database) },
            flag_pickups: Leaderboard { score_type: ScoreType::FlagPickups, cache: Arc::clone(&redis), database: Arc::clone(&database) },
            flag_defends: Leaderboard { score_type: ScoreType::FlagDefends, cache: Arc::clone(&redis), database: Arc::clone(&database) },
            flag_hold_time: Leaderboard { score_type: ScoreType::FlagHoldTime, cache: Arc::clone(&redis), database: Arc::clone(&database) },
            wool_captures: Leaderboard { score_type: ScoreType::WoolCaptures, cache: Arc::clone(&redis), database: Arc::clone(&database) },
            wool_drops: Leaderboard { score_type: ScoreType::WoolDrops, cache: Arc::clone(&redis), database: Arc::clone(&database) },
            wool_pickups: Leaderboard { score_type: ScoreType::WoolPickups, cache: Arc::clone(&redis), database: Arc::clone(&database) },
            wool_defends: Leaderboard { score_type: ScoreType::WoolDefends, cache: Arc::clone(&redis), database: Arc::clone(&database) },
            control_point_captures: Leaderboard { score_type: ScoreType::ControlPointCaptures, cache: Arc::clone(&redis), database: Arc::clone(&database) },
            highest_killstreak: Leaderboard { score_type: ScoreType::HighestKillstreak, cache: Arc::clone(&redis), database: Arc::clone(&database) }
        }
    }

    pub fn from_score_type(&self, score_type: ScoreType) -> &Leaderboard {
        match score_type {
            ScoreType::Kills => &self.kills,
            ScoreType::Deaths => &self.deaths,
            ScoreType::FirstBloods => &self.first_bloods,
            ScoreType::Wins => &self.wins,
            ScoreType::Losses => &self.losses,
            ScoreType::Ties => &self.ties,
            ScoreType::Xp => &self.xp,
            ScoreType::MessagesSent => &self.messages_sent,
            ScoreType::MatchesPlayed => &self.matches_played,
            ScoreType::ServerPlaytime => &self.server_playtime,
            ScoreType::GamePlaytime => &self.game_playtime,
            ScoreType::CoreLeaks => &self.core_leaks,
            ScoreType::CoreBlockDestroys => &self.core_block_destroys,
            ScoreType::DestroyableDestroys => &self.destroyable_destroys,
            ScoreType::DestroyableBlockDestroys => &self.destroyable_block_destroys,
            ScoreType::FlagCaptures => &self.flag_captures,
            ScoreType::FlagDrops => &self.flag_drops,
            ScoreType::FlagPickups => &self.flag_pickups,
            ScoreType::FlagDefends => &self.flag_defends,
            ScoreType::FlagHoldTime => &self.flag_hold_time,
            ScoreType::WoolCaptures => &self.wool_captures,
            ScoreType::WoolDrops => &self.wool_drops,
            ScoreType::WoolPickups => &self.wool_pickups,
            ScoreType::WoolDefends => &self.wool_defends,
            ScoreType::ControlPointCaptures => &self.control_point_captures,
            ScoreType::HighestKillstreak => &self.highest_killstreak
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LeaderboardEntry {
    pub id: String,
    pub name: String,
    pub score: u32
}
