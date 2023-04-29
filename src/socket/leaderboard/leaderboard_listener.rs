use crate::{socket::{player::{player_listener::PlayerListener, player_events::PlayerDeathData}, participant::participant_context::{PlayerMatchResult}, r#match::match_events::{MatchEndData}, server::server_context::ServerContext}, database::models::{participant::Participant, r#match::Match}};

pub struct LeaderboardListener {}

#[async_trait]
impl PlayerListener for LeaderboardListener {
    type Context = Participant;

    async fn on_match_end_v2(
        &self,
        server_context: &mut ServerContext, 
        current_match: &mut Match, 
        context: &mut Self::Context, 
        end_data: &mut MatchEndData
    ) { 
        {
            if !current_match.is_tracking_stats() {
                return;
            }

            let match_result = current_match.get_participant_match_result(&context, end_data);

            match match_result {
                PlayerMatchResult::Win => {
                    server_context.api_state.leaderboards.wins.increment(&context.get_id_name(), Some(1)).await;
                },
                PlayerMatchResult::Lose => {
                    server_context.api_state.leaderboards.losses.increment(&context.get_id_name(), Some(1)).await;
                },
                PlayerMatchResult::Tie => {
                    server_context.api_state.leaderboards.ties.increment(&context.get_id_name(), Some(1)).await;
                },
                _ => {} 
            }

            server_context.api_state.leaderboards.matches_played.increment(&context.get_id_name(), Some(1)).await;
            server_context.api_state.leaderboards.messages_sent.increment(
                &context.get_id_name(), 
                Some(context.stats.messages.total())
            ).await;
            server_context.api_state.leaderboards.game_playtime.increment(
                &context.get_id_name(), 
                Some(u32::try_from(context.stats.game_playtime).unwrap_or(0))
            ).await;
        };
    }


    async fn on_kill(
        &self,
        server_context: &mut ServerContext, 
        current_match: &mut Match, 
        context: &mut Self::Context, 
        _data: &mut PlayerDeathData, 
        first_blood: bool
    ) { 
        {
            if !current_match.is_tracking_stats() {
                return;
            };

            server_context.api_state.leaderboards.kills.increment(&context.get_id_name(), Some(1)).await;
            if first_blood {
                server_context.api_state.leaderboards.first_bloods.increment(&context.get_id_name(), Some(1)).await;
            };
        }
    }

    async fn on_death(
        &self,
        server_context: &mut ServerContext, 
        current_match: &mut Match, 
        context: &mut Self::Context, 
        _data: &mut PlayerDeathData, 
        _first_blood: bool
    ) { 
        {
            if !current_match.is_tracking_stats() {
                return;
            };

            server_context.api_state.leaderboards.deaths.increment(&context.get_id_name(), Some(1)).await;
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
            if !current_match.is_tracking_stats() {
                return;
            };
            server_context.api_state.leaderboards.highest_killstreak.set_if_higher(&context.get_id_name(), amount).await;
        };
    }

    async fn on_destroyable_destroy(
        &self, 
        server_context: &mut ServerContext, 
        current_match: &mut Match, 
        context: &mut Self::Context, 
        _percentage: f32, 
        block_count: u32
    ) {
        if !current_match.is_tracking_stats() {
            return;
        };
        server_context.api_state.leaderboards.destroyable_destroys.increment(&context.get_id_name(), Some(1)).await;
        server_context.api_state.leaderboards.destroyable_block_destroys.increment(&context.get_id_name(), Some(block_count)).await;
    }

    async fn on_core_leak(
        &self, 
        server_context: &mut ServerContext, 
        current_match: &mut Match, 
        context: &mut Self::Context, 
        _percentage: f32, 
        _block_count: u32
    ) {
        if !current_match.is_tracking_stats() {
            return;
        };
        server_context.api_state.leaderboards.core_leaks.increment(&context.get_id_name(), Some(1)).await;
        server_context.api_state.leaderboards.core_block_destroys.increment(&context.get_id_name(), Some(1)).await;
    }

    async fn on_flag_place(
        &self, 
        server_context: &mut ServerContext, 
        current_match: &mut Match, 
        context: &mut Self::Context, 
        held_time: u64, 
    ) {
        if !current_match.is_tracking_stats() {
            return;
        };
        server_context.api_state.leaderboards.flag_captures.increment(&context.get_id_name(), Some(1)).await;
        server_context.api_state.leaderboards.flag_hold_time.increment(&context.get_id_name(), Some(u32::try_from(held_time).unwrap())).await;
    }

    async fn on_flag_pickup(
        &self, 
        server_context: &mut ServerContext, 
        current_match: &mut Match, 
        context: &mut Self::Context
    ) {
        if !current_match.is_tracking_stats() {
            return;
        };
        server_context.api_state.leaderboards.flag_pickups.increment(&context.get_id_name(), Some(1)).await;
    }

    async fn on_flag_drop(
        &self, 
        server_context: &mut ServerContext, 
        current_match: &mut Match, 
        context: &mut Self::Context, 
        held_time: u64, 
    ) {
        if !current_match.is_tracking_stats() {
            return;
        };
        server_context.api_state.leaderboards.flag_drops.increment(&context.get_id_name(), Some(1)).await;
        server_context.api_state.leaderboards.flag_hold_time.increment(&context.get_id_name(), Some(u32::try_from(held_time).unwrap())).await;
    }

    async fn on_flag_defend(
        &self, 
        server_context: &mut ServerContext, 
        current_match: &mut Match, 
        context: &mut Self::Context
    ) {
        if !current_match.is_tracking_stats() {
            return;
        };
        server_context.api_state.leaderboards.flag_defends.increment(&context.get_id_name(), Some(1)).await;
    }

    async fn on_wool_place(
        &self, 
        server_context: &mut ServerContext, 
        current_match: &mut Match, 
        context: &mut Self::Context, 
        _held_time: u64, 
    ) {
        if !current_match.is_tracking_stats() {
            return;
        };
        server_context.api_state.leaderboards.wool_captures.increment(&context.get_id_name(), Some(1)).await;
    }

    async fn on_wool_pickup(
        &self, 
        server_context: &mut ServerContext, 
        current_match: &mut Match, 
        context: &mut Self::Context
    ) {
        if !current_match.is_tracking_stats() {
            return;
        };
        server_context.api_state.leaderboards.wool_pickups.increment(&context.get_id_name(), Some(1)).await;
    }

    async fn on_wool_drop(
        &self, 
        server_context: &mut ServerContext, 
        current_match: &mut Match, 
        context: &mut Self::Context, 
        _held_time: u64, 
    ) {
        if !current_match.is_tracking_stats() {
            return;
        };
        server_context.api_state.leaderboards.wool_drops.increment(&context.get_id_name(), Some(1)).await;
    }

    async fn on_wool_defend(
        &self, 
        server_context: &mut ServerContext, 
        current_match: &mut Match, 
        context: &mut Self::Context
    ) {
        if !current_match.is_tracking_stats() {
            return;
        };
        server_context.api_state.leaderboards.wool_defends.increment(&context.get_id_name(), Some(1)).await;
    }

    async fn on_control_point_capture(
        &self, 
        server_context: &mut ServerContext, 
        current_match: &mut Match, 
        context: &mut Self::Context, 
        _contributors: u32, 
    ) {
        if !current_match.is_tracking_stats() {
            return;
        };

        server_context.api_state.leaderboards.control_point_captures.increment(&context.get_id_name(), Some(1)).await;
    }
}
