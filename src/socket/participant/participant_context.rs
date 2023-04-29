

use serde::{Serialize, Deserialize};


use crate::{database::models::{participant::Participant, r#match::Match}, socket::{player::player_context::PlayerContext}, MarsAPIState};

pub struct ParticipantContext<'a> {
    pub profile: Participant,
    pub current_match: &'a mut Match
}

impl<'a> ParticipantContext<'a> {
    pub async fn get_player_context(&'a mut self, state: &MarsAPIState) -> PlayerContext<'a> {
        let player = self.profile.get_player(state).await;
        PlayerContext { profile: player, current_match: self.current_match }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PlayerMatchResult {
    Win,
    Lose,
    Tie,
    Intermediate
}
