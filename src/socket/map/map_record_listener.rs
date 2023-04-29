

use crate::{socket::{player::{player_listener::PlayerListener, player_events::PlayerDeathData}, r#match::match_events::MatchEndData, server::server_context::ServerContext}, database::models::{player::{PlayerRecord, ProjectileRecord, FirstBloodRecord}, death::DamageCause, participant::Participant, r#match::Match}, util::time::get_u64_time_millis};
use async_trait::async_trait;

pub struct MapRecordListener {}

#[async_trait]
impl PlayerListener for MapRecordListener {
    type Context = Participant;

    async fn on_kill(
        &self,
        server_context: &mut ServerContext, 
        current_match: &mut Match, 
        context: &mut Self::Context, 
        data: &mut PlayerDeathData, 
        first_blood: bool
    ) { 
        {
            if first_blood {
                let time = get_u64_time_millis() - current_match.started_at.unwrap();
                let record_beat = match current_match.level.records.fastest_first_blood.as_ref() {
                    Some(first_blood_record) => {
                        time < first_blood_record.time
                    },
                    None => { true },
                };
                if record_beat {
                    current_match.level.records.fastest_first_blood = Some(FirstBloodRecord {
                        match_id: current_match.id.clone(),
                        attacker: context.get_simple_player(),
                        victim: data.victim.clone(),
                        time
                    });
                };
            };

            if data.distance.is_some() && current_match.participants.len() >= 6 && data.cause != DamageCause::Fall {
                let record_beat = match current_match.level.records.longest_projectile_kill.as_ref() {
                    Some(projectile_record) => {
                        data.distance.unwrap() > projectile_record.distance
                    }
                    None => {
                        true
                    }
                };

                if record_beat {
                    current_match.level.records.longest_projectile_kill = Some(ProjectileRecord { 
                        match_id: current_match.id.clone(), 
                        player: context.get_simple_player(), 
                        distance: data.distance.unwrap() 
                    });
                };
            };

            server_context.api_state.match_cache.set(&server_context.api_state.database, &current_match.id, &current_match, false).await;
        };
    }

    async fn on_killstreak(
        &self,
        server_context: &mut ServerContext, 
        current_match: &mut Match, 
        context: &mut Self::Context, 
        amount: u32
    ) {
        {
            let current_record = match &current_match.level.records.highest_killstreak {
                Some(ks_record) => { ks_record.value },
                None => { 0 },
            };

            if amount > current_record {
                current_match.level.records.highest_killstreak = Some(PlayerRecord { 
                    match_id: current_match.id.clone(), 
                    player: context.get_simple_player(), 
                    value: amount
                }); 
            };

            server_context.api_state.match_cache.set(&server_context.api_state.database, &current_match.id, &current_match, false).await;
        };
    }

    async fn on_wool_place(
        &self, 
        server_context: &mut ServerContext, 
        current_match: &mut Match, 
        context: &mut Self::Context, 
        held_time: u64, 
    ) {
        let record_time = &current_match.level.records.fastest_wool_capture;
        if record_time.is_none() || held_time < record_time.as_ref().unwrap().value {
            current_match.level.records.fastest_wool_capture = Some(PlayerRecord { 
                match_id: current_match.id.clone(), 
                player: context.get_simple_player(), 
                value: held_time 
            });
        }

        server_context.api_state.match_cache.set(
            &server_context.api_state.database, &current_match.id, &current_match, false
        ).await;
    }

    async fn on_flag_place(
        &self, 
        server_context: &mut ServerContext, 
        current_match: &mut Match, 
        context: &mut Self::Context, 
        held_time: u64, 
    ) {
        let record_time = &current_match.level.records.fastest_flag_capture;
        if record_time.is_none() || held_time < record_time.as_ref().unwrap().value {
            current_match.level.records.fastest_flag_capture = Some(PlayerRecord { 
                match_id: current_match.id.clone(), 
                player: context.get_simple_player(), 
                value: held_time 
            });
        }

        server_context.api_state.match_cache.set(
            &server_context.api_state.database, &current_match.id, &current_match, false
        ).await;
    }

    async fn on_match_end_v2(
        &self,
        server_context: &mut ServerContext, 
        current_match: &mut Match, 
        context: &mut Self::Context, 
        _end_data: &mut MatchEndData
    ) {
        {
            let kills = context.stats.kills;
            let record_kills = 
                if let Some(record) = &current_match.level.records.kills_in_match {
                    record.value
                } else { 0 };
            if kills > record_kills {
                current_match.level.records.kills_in_match = Some(PlayerRecord { 
                    match_id: current_match.id.clone(),
                    player: context.get_simple_player(),
                    value: kills
                });
            };

            let deaths = context.stats.deaths;
            let record_deaths = 
                if let Some(record) = &current_match.level.records.deaths_in_match {
                    record.value
                } else { 0 };
            if deaths > record_deaths {
                current_match.level.records.deaths_in_match = Some(PlayerRecord { 
                    match_id: current_match.id.clone(),
                    player: context.get_simple_player(),
                    value: deaths
                });
            };
            server_context.api_state.match_cache.set(&server_context.api_state.database, &current_match.id, &current_match, false).await;
        };
    }

}
