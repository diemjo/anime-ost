use rocket::{State, serde::json::Json, response::content};
use serde::Deserialize;

use crate::{config::AppConfig, result::{Result, wrap_ok}, error::Error, proxer::AnimeDB, server::{user::RequestUser, deserialize_error_on_missing}, models::OstType};

#[derive(Debug, Deserialize)]
pub(crate) struct OstInput {
    #[serde(deserialize_with = "deserialize_error_on_missing")]
    name: Option<String>,
    #[serde(deserialize_with = "deserialize_error_on_missing")]
    artist: Option<String>,
    #[serde(deserialize_with = "deserialize_error_on_missing")]
    video_url: Option<String>,
}

pub(crate) fn get_routes() -> Vec<rocket::Route> {
    rocket::routes![
        get_ost,
        get_ost_single,
        post_ost_single,
        delete_ost_single
    ]
}

#[rocket::get("/ost")]
pub(crate) async fn get_ost(config: &State<AppConfig>, _user: RequestUser) -> Result<content::RawJson<String>> {
    let ost_db = AnimeDB::new(&config.db_path).await?;
    let ost_list = ost_db.get_all_ost()?;
    Ok(wrap_ok(ost_list))
}

#[rocket::get("/ost/<proxer_id>/<ost_type>/<number>")]
pub(crate) async fn get_ost_single(config: &State<AppConfig>, _user: RequestUser, proxer_id: u32, ost_type: OstType, number: u32) -> Result<content::RawJson<String>> {
    let ost_db = AnimeDB::new(&config.db_path).await?;
    match ost_db.get_ost(proxer_id, ost_type, number)? {
        Some(ost) => Ok(wrap_ok(ost)),
        None => Err(Error::MissingOstError(proxer_id, ost_type, number))
    }
}

#[rocket::post("/ost/<proxer_id>/<ost_type>/<number>", format = "application/json", data = "<input>")]
pub(crate) async fn post_ost_single(config: &State<AppConfig>, _user: RequestUser, proxer_id: u32, ost_type: OstType, number: u32, input: Json<OstInput>) -> Result<content::RawJson<String>> {
    let mut ost_db = AnimeDB::new(&config.db_path).await?;
    let _anime = match ost_db.get_anime(proxer_id)? {
        Some(anime) => Ok(anime),
        None => Err(Error::MissingAnimeError(proxer_id))
    }?;
    let _ = match ost_db.get_ost(proxer_id, ost_type, number)? {
        Some(_) => ost_db.update_ost(proxer_id, ost_type, number, input.name.clone(), input.artist.clone(), input.video_url.clone()),
        None => ost_db.insert_ost(proxer_id, ost_type, number, input.name.clone(), input.artist.clone(), input.video_url.clone())
    }?;
    Ok(wrap_ok("Success".to_string()))
}

#[rocket::delete("/ost/<proxer_id>/<ost_type>/<number>")]
pub(crate) async fn delete_ost_single(config: &State<AppConfig>, _user: RequestUser, proxer_id: u32, ost_type: OstType, number: u32) -> Result<content::RawJson<String>> {
    let mut ost_db = AnimeDB::new(&config.db_path).await?;
    match ost_db.get_ost(proxer_id, ost_type, number)? {
        Some(_ost) => ost_db.delete_ost(proxer_id, ost_type, number),
        None => Err(Error::MissingOstError(proxer_id, ost_type, number))
    }?;
    Ok(wrap_ok("Success".to_string()))
}