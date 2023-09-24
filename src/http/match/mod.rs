use rocket::{State, Build, Rocket};
use crate::{database::models::r#match::Match, MarsAPIState, util::{responder::JsonResponder, error::ApiErrorResponder, r#macro::unwrap_helper}};

#[get("/<match_id>")]
pub async fn matches(
    state: &State<MarsAPIState>,
    match_id: &str
) -> Result<JsonResponder<Match>, ApiErrorResponder> {
    let match_id = match_id.to_lowercase();
    let cached_match = 
        unwrap_helper::return_default!(
            state.match_cache.get(&state.database, &match_id).await,
            Err(ApiErrorResponder::validation_error())
        );
    Ok(JsonResponder::ok(cached_match))
}

pub fn mount(rocket_build: Rocket<Build>) -> Rocket<Build> {
    rocket_build.mount("/mc/matches", routes![matches])
}
