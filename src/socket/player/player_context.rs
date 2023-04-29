





use crate::{database::models::{player::{Player}, r#match::Match}, socket::{server::server_context::ServerContext, event_type::EventType}};

use super::player_events::MessageData;

pub struct PlayerContext<'a> {
    pub profile: Player,
    pub current_match: &'a mut Match
}

impl<'a> PlayerContext<'a> {
    // pub async fn get_participant(&mut self) -> Participant {
    //     let mut current_match = self.current_match.lock().await;
    //     current_match.participants.get_mut(&self.profile.id).expect("Participant should exist").clone()
    // }

    pub async fn send_message(&self, server_context: &mut ServerContext, message: &str, sound: Option<String>) {
        let message_data = MessageData {
            message: message.to_owned(),
            sound,
            player_ids: vec![self.profile.id.clone()],
        };
        server_context.call(&EventType::Message, message_data).await;
    }
}

pub async fn send_message_to_player(server_context: &mut ServerContext, player: &Player, message: &str, sound: Option<String>) {
    let message_data = MessageData {
        message: message.to_owned(),
        sound,
        player_ids: vec![player.id.clone()],
    };
    server_context.call(&EventType::Message, message_data).await;
}
