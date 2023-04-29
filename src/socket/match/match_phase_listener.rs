use std::collections::HashMap;

use uuid::Uuid;

use crate::{socket::{server::{server_context::ServerContext, server_events::MatchLoadData}, socket_router::SocketError}, database::{Database, models::{r#match::{Match, Party, MatchState}, participant::Participant}}, util::{r#macro::unwrap_helper, time::get_u64_time_millis}};

use super::match_events::{MatchStartData, MatchEndData};

pub struct MatchPhaseListener<'a> {
    pub server: &'a mut ServerContext
}

impl MatchPhaseListener<'_> {
    pub async fn on_load(&mut self, data: MatchLoadData) -> Result<(), SocketError> {
        let mut level = unwrap_helper::return_default!(Database::find_by_id(&self.server.api_state.database.levels, &data.map_id).await, Err(SocketError::InvalidMatchState));
        let time_millis = get_u64_time_millis();
        let match_id = Uuid::new_v4().to_string();
        level.goals = Some(data.goals);
        level.last_match_id = Some(match_id.clone());

        let mut parties : HashMap<String, Party> = HashMap::new();
        for party in data.parties {
            parties.insert(party.name.clone(), Party { name: party.name, alias: party.alias, color: party.color, min: party.min, max: party.max });
        }

        let new_match = Match {
            id: match_id,
            loaded_at: time_millis,
            started_at: None,
            ended_at: None,
            level,
            parties,
            participants: HashMap::new(),
            server_id: self.server.id.clone(),
            first_blood: None
        };


        self.server.api_state.match_cache.set(&self.server.api_state.database, &new_match.id, &new_match, true).await;
        self.server.set_current_match_id(&new_match.id).await;
        info!("({}) Match loaded: {}", self.server.id, new_match.id);
        Ok(())
    }

    pub fn on_start(&self, data: MatchStartData, mut current_match: Match) -> Result<Match, SocketError> {
        if MatchState::Pre != current_match.get_state() {
            return Err(SocketError::InvalidMatchState)
        };

        current_match.started_at = Some(get_u64_time_millis());

        let participants : Vec<Participant> = data.participants.into_iter().map(|p| { Participant::from_simple(p) }).collect();
        current_match.save_participants(participants);

        info!("({}) Match started: {}", self.server.id, current_match.id);
        Ok(current_match)
    }

    pub fn on_end(&self, _data: &MatchEndData, mut current_match: Match) -> Result<Match, SocketError> {
        if MatchState::InProgress != current_match.get_state() {
            return Err(SocketError::InvalidMatchState)
        };
        current_match.ended_at = Some(get_u64_time_millis());
        info!("({}) Match ended: {}", self.server.id, current_match.id);
        Ok(current_match)
    }
}
