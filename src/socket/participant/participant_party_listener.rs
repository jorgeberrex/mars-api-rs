use crate::database::models::participant::Participant;
use crate::database::models::r#match::Match;

use crate::socket::{player::player_listener::PlayerListener, server::server_context::ServerContext};

use crate::util::time::get_u64_time_millis;



pub struct ParticipantPartyListener {}

use async_trait::async_trait;

#[async_trait]
impl PlayerListener for ParticipantPartyListener {
    type Context = Participant;

    async fn on_party_join(
        &self,
        _server_context: &mut ServerContext, 
        _current_match: &mut Match, 
        context: &mut Self::Context, 
        party_name: String
    ) {
        context.party_name = Some(party_name.clone());
        context.last_party_name = Some(party_name.clone());
        context.joined_party_at = Some(get_u64_time_millis());
    }

    async fn on_party_leave(
        &self,
        _server_context: &mut ServerContext, 
        _current_match: &mut Match, 
        context: &mut Self::Context
    ) {
        context.party_name = None;
        context.last_left_party_at = Some(get_u64_time_millis());
        context.joined_party_at = None;
    }
}
