use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use rocket::{response::{self, Response, Responder}, Request, http::{Status, ContentType}, serde::json::Json};

use crate::{database::models::{player::{SimplePlayer, Player}, punishment::Punishment, session::Session}, socket::leaderboard::ScoreType};

#[derive(Deserialize, Serialize)]
pub struct PlayerPreLoginRequest {
    pub player: SimplePlayer,
    pub ip: String
}

pub type PlayerLoginRequest = PlayerPreLoginRequest;

pub struct PlayerPreLoginResponder {
    pub response: PlayerPreLoginResponse
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerPreLoginResponse {
    pub new: bool,
    pub allowed: bool,
    pub player: Player,
    pub active_punishments: Vec<Punishment>
}

impl<'r> Responder<'r, 'static> for PlayerPreLoginResponder {
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'static> {
        let is_new = self.response.new;
        let data = Json(self.response);
        Response::build_from(data.respond_to(req)?)
            .header(ContentType::JSON)
            .status(if is_new { Status::Created } else { Status::Ok })
            .ok()
    }
}

pub struct PlayerLoginResponder {
    pub response: PlayerLoginResponse
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerLoginResponse {
    pub active_session: Session
}

impl<'r> Responder<'r, 'static> for PlayerLoginResponder {
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'static> {
        let data = Json(self.response);
        Response::build_from(data.respond_to(req)?)
            .header(ContentType::JSON)
            .status(Status::Created)
            .ok()
    }
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerLogoutRequest {
    pub player: SimplePlayer,
    pub session_id: String,
    pub playtime: u64
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerLookupResponse {
    pub player: Player,
    pub alts: Vec<PlayerAltResponse>
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerAltResponse {
    pub player: Player,
    pub punishments: Vec<Punishment>
}


#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerProfileResponse {
    pub player: Player,
    pub leaderboard_positions: HashMap<ScoreType, u64>
}

pub enum PlayerProfileResponder {
    RawProfile(Player),
    ProfileWithLeaderboardPositions(PlayerProfileResponse)
}

impl <'r> Responder<'r, 'static> for PlayerProfileResponder {
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'static> {
        match &self {
            PlayerProfileResponder::RawProfile(profile) => {
                let data = Json(profile);
                Response::build_from(data.respond_to(req)?)
                    .header(ContentType::JSON)
                    .status(Status::Ok)
                    .ok()
            },
            PlayerProfileResponder::ProfileWithLeaderboardPositions(wrapped) => {
                let data = Json(wrapped);
                Response::build_from(data.respond_to(req)?)
                    .header(ContentType::JSON)
                    .status(Status::Ok)
                    .ok()
            },
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct PlayerAddNoteRequest {
    pub author: SimplePlayer,
    pub content: String
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerSetActiveTagRequest {
    pub active_tag_id: Option<String>
}
