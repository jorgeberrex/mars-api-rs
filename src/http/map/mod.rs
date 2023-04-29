use futures::future::join_all;
use mongodb::bson::doc;
use rocket::{Rocket, Build, State, serde::json::Json};

use crate::{MarsAPIState, http::map::payload::MapLoadOneRequest, util::{auth::AuthorizationToken, time::get_u64_time_millis, r#macro::unwrap_helper, error::ApiErrorResponder}, database::{models::level::{Level, LevelRecords}, Database}};

mod payload;

#[post("/", format = "json", data = "<maps>")]
async fn add_maps(
    state: &State<MarsAPIState>,
    maps: Json<Vec<MapLoadOneRequest>>,
    _auth_guard: AuthorizationToken
) -> Json<Vec<Level>> {
    let map_list = maps.0;
    let map_list_length = map_list.len();
    let time_millis = get_u64_time_millis();
    let mut maps_to_save : Vec<Level> = Vec::new();
    
    let query_tasks : Vec<_> = map_list.iter().map(|map| {
        state.database.find_by_name::<Level>(&map.name)
    }).collect();
    let level_docs = join_all(query_tasks).await;
    for (map, level_opt) in map_list.into_iter().zip(level_docs) {
        maps_to_save.push(if let Some(mut existing_map) = level_opt {
            existing_map.name = map.name;
            existing_map.name_lower = existing_map.name.to_lowercase();
            existing_map.version = map.version;
            existing_map.gamemodes = map.gamemodes;
            existing_map.authors = map.authors;
            existing_map.updated_at = time_millis;
            existing_map.contributors = map.contributors;
            existing_map
        } else {
            // do this before the move
            let lowercase_map_name = map.name.to_lowercase();
            Level {
                id: map.id,
                name: map.name,
                name_lower: lowercase_map_name,
                version: map.version,
                gamemodes: map.gamemodes,
                loaded_at: time_millis,
                updated_at: time_millis,
                authors: map.authors,
                contributors: map.contributors,
                records: LevelRecords::default(),
                goals: None,
                last_match_id: None
            }
        });
    }

    let save_tasks : Vec<_> = maps_to_save.iter().map(|map| { state.database.save(map) }).collect();
    join_all(save_tasks).await;

    info!("Received {} maps. Updating {} maps.", map_list_length, maps_to_save.len());
    Json(state.database.get_all_documents().await)
}

#[get("/")]
async fn get_all_maps(state: &State<MarsAPIState>) -> Json<Vec<Level>> {
    Json(state.database.get_all_documents::<Level>().await)
}

#[get("/<map_id>")]
async fn get_map_by_id(state: &State<MarsAPIState>, map_id: &str) -> Result<Json<Level>, ApiErrorResponder> {
    let map = unwrap_helper::return_default!(Database::find_by_id(&state.database.levels, map_id).await, Err(ApiErrorResponder::missing_map()));
    Ok(Json(map))
}

pub fn mount(build: Rocket<Build>) -> Rocket<Build> {
    build.mount("/mc/maps", routes![add_maps, get_all_maps, get_map_by_id])
}
