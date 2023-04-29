use crate::{socket::{r#match::match_events::{MatchEndData}, server::server_context::ServerContext}, database::models::r#match::{DestroyableGoal, Match}};

use super::player_events::{PlayerDeathData, PlayerChatData};
use async_trait::async_trait;

#[async_trait]
pub trait PlayerListener : Sync {
    type Context;

    async fn on_kill(
        &self, 
        _server_context: &mut ServerContext, 
        _current_match: &mut Match, 
        _context: &mut Self::Context, 
        _data: &mut PlayerDeathData, 
        _first_blood: bool
    ) {}

    async fn on_death(
        &self, 
        _server_context: &mut ServerContext, 
        _current_match: &mut Match, 
        _context: &mut Self::Context, 
        _data: &mut PlayerDeathData, 
        _first_blood: bool
    ) {}

    async fn on_chat(
        &self, 
        _server_context: &mut ServerContext, 
        _current_match: &mut Match, 
        _context: &mut Self::Context, 
        _data: &mut PlayerChatData
    ) {}

    async fn on_killstreak(
        &self, 
        _server_context: &mut ServerContext, 
        _current_match: &mut Match, 
        _context: &mut Self::Context, 
        _amount: u32
    ) {}

    async fn on_killstreak_end(
        &self, 
        _server_context: &mut ServerContext, 
        _current_match: &mut Match, 
        _context: &mut Self::Context, 
        _amount: u32
    ) {}

    async fn on_party_join(
        &self, 
        _server_context: &mut ServerContext, 
        _current_match: &mut Match,
        _context: &mut Self::Context, 
        _party_name: String
    ) {}

    async fn on_party_leave(
        &self, 
        _server_context: &mut ServerContext, 
        _current_context: &mut Match, 
        _context: &mut Self::Context
    ) {}

    async fn on_match_end_v2(
        &self, 
        _server_context: &mut ServerContext, 
        _current_match: &mut Match, 
        _context: &mut Self::Context, 
        _end_data: &mut MatchEndData
    ) {}

    async fn on_destroyable_damage(
        &self, 
        _server_context: &mut ServerContext, 
        _current_match: &mut Match, 
        _context: &mut Self::Context, 
        _destroyable: &DestroyableGoal, 
        _block_count: u32
    ) {}

    async fn on_destroyable_destroy(
        &self, 
        _server_context: &mut ServerContext, 
        _current_match: &mut Match, 
        _context: &mut Self::Context, 
        _percentage: f32, 
        _block_count: u32
    ) {}

    async fn on_core_leak(
        &self, 
        _server_context: &mut ServerContext, 
        _current_match: &mut Match, 
        _context: &mut Self::Context, 
        _percentage: f32, 
        _block_count: u32
    ) {}

    async fn on_control_point_capture(
        &self, 
        _server_context: &mut ServerContext, 
        _current_match: &mut Match, 
        _context: &mut Self::Context, 
        _contributors: u32, 
    ) {}

    async fn on_flag_place(
        &self, 
        _server_context: &mut ServerContext, 
        _current_match: &mut Match, 
        _context: &mut Self::Context, 
        _held_time: u64, 
    ) {}

    async fn on_flag_pickup(
        &self, 
        _server_context: &mut ServerContext, 
        _current_match: &mut Match, 
        _context: &mut Self::Context
    ) {}

    async fn on_flag_drop(
        &self, 
        _server_context: &mut ServerContext, 
        _current_match: &mut Match, 
        _context: &mut Self::Context,
        _held_time: u64, 
    ) {}

    async fn on_flag_defend(
        &self, 
        _server_context: &mut ServerContext, 
        _current_match: &mut Match, 
        _context: &mut Self::Context
    ) {}

    async fn on_wool_place(
        &self, 
        _server_context: &mut ServerContext, 
        _current_match: &mut Match, 
        _context: &mut Self::Context, 
        _held_time: u64, 
    ) {}

    async fn on_wool_pickup(
        &self, 
        _server_context: &mut ServerContext, 
        _current_match: &mut Match, 
        _context: &mut Self::Context
    ) {}

    async fn on_wool_drop(
        &self, 
        _server_context: &mut ServerContext, 
        _current_match: &mut Match, 
        _context: &mut Self::Context,
        _held_time: u64, 
    ) {}

    async fn on_wool_defend(
        &self, 
        _server_context: &mut ServerContext, 
        _current_match: &mut Match, 
        _context: &mut Self::Context
    ) {}
}
