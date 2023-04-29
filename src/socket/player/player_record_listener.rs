use crate::{database::models::{player::{PlayerRecord, FirstBloodRecord, ProjectileRecord, Player}, death::DamageCause, r#match::Match}, socket::{server::server_context::ServerContext, r#match::match_events::MatchEndData}, util::time::get_u64_time_millis};

use super::{player_listener::PlayerListener, player_events::PlayerDeathData};

pub struct PlayerRecordListener {}

#[async_trait]
impl PlayerListener for PlayerRecordListener {
    type Context = Player;

    async fn on_kill(
        &self,
        _server_context: &mut ServerContext, 
        current_match: &mut Match, 
        context: &mut Self::Context, 
        data: &mut PlayerDeathData, 
        first_blood: bool
    ) { 
        {
            if !current_match.is_tracking_stats() {
                return;
            };

            if first_blood {
                let time = get_u64_time_millis() - current_match.started_at.unwrap();
                let record_beat = match context.stats.records.fastest_first_blood.as_ref() {
                    Some(first_blood_record) => {
                        time < first_blood_record.time
                    },
                    None => { true },
                };
                if record_beat {
                    context.stats.records.fastest_first_blood = Some(FirstBloodRecord {
                        match_id: current_match.id.clone(),
                        attacker: context.to_simple(),
                        victim: data.victim.clone(),
                        time
                    });
                };
            };

            if data.distance.is_some() && current_match.participants.len() >= 6 && data.cause != DamageCause::Fall {
                let record_beat = match context.stats.records.longest_projectile_kill.as_ref() {
                    Some(projectile_record) => {
                        data.distance.unwrap() > projectile_record.distance
                    }
                    None => {
                        true
                    }
                };

                if record_beat {
                    context.stats.records.longest_projectile_kill = Some(ProjectileRecord { 
                        match_id: current_match.id.clone(), 
                        player: context.to_simple(), 
                        distance: data.distance.unwrap() 
                    });
                };
            };
        };
    }

    async fn on_wool_place(
        &self, 
        _server_context: &mut ServerContext, 
        current_match: &mut Match, 
        context: &mut Self::Context, 
        held_time: u64, 
    ) {
        if !current_match.is_tracking_stats() {
            return;
        }

        let record_time = &context.stats.records.fastest_wool_capture;
        if record_time.is_none() || held_time < record_time.as_ref().unwrap().value {
            context.stats.records.fastest_wool_capture = Some(PlayerRecord { 
                match_id: current_match.id.clone(), 
                player: context.to_simple(), 
                value: held_time 
            });
        }
    }

    async fn on_flag_place(
        &self, 
        _server_context: &mut ServerContext, 
        current_match: &mut Match, 
        context: &mut Self::Context, 
        held_time: u64, 
    ) {
        if !current_match.is_tracking_stats() {
            return;
        }

        let record_time = &context.stats.records.fastest_flag_capture;
        if record_time.is_none() || held_time < record_time.as_ref().unwrap().value {
            context.stats.records.fastest_flag_capture = Some(PlayerRecord { 
                match_id: current_match.id.clone(), 
                player: context.to_simple(), 
                value: held_time 
            });
        }
    }

    async fn on_match_end_v2(
        &self,
        _server_context: &mut ServerContext, 
        current_match: &mut Match, 
        context: &mut Self::Context, 
        _end_data: &mut MatchEndData
    ) { 
        {
            if !current_match.is_tracking_stats() {
                return;
            }

            let participant = current_match.get_participant(&context.id);

            let kills = participant.stats.kills;
            let record_kills = match context.stats.records.kills_in_match.clone() {
                Some(kills_in_match_record) => { kills_in_match_record.value },
                None => { 0 }
            };
            if kills > record_kills {
                context.stats.records.kills_in_match = Some(PlayerRecord {
                    match_id: current_match.id.clone(),
                    player: context.to_simple(),
                    value: kills,
                });
            };

            let deaths = participant.stats.deaths;
            let record_deaths = match context.stats.records.deaths_in_match.clone() {
                Some(deaths_in_match_record) => { deaths_in_match_record.value },
                None => { 0 }
            };
            if deaths > record_deaths {
                context.stats.records.deaths_in_match = Some(PlayerRecord {
                    match_id: current_match.id.clone(),
                    player: context.to_simple(),
                    value: deaths,
                });
            };
        };
    }
}
