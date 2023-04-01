use rocket::{State, response::content};

use crate::{config::AppConfig, result::{Result, wrap_ok}, proxer::AnimeDB, server::user::RequestUser};

pub(crate) fn get_routes() -> Vec<rocket::Route> {
    rocket::routes![
        get_anime
    ]
}

#[rocket::get("/anime")]
pub(crate) async fn get_anime(config: &State<AppConfig>, _user: RequestUser) -> Result<content::RawJson<String>> {
    let ost_db = AnimeDB::new(&config.db_path).await?;
    let anime_list = ost_db.get_all_anime()?;
    Ok(wrap_ok(anime_list))
}