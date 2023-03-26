use rocket::State;

use crate::{config::AppConfig, error::Result, proxer::AnimeDB, server::user::RequestUser};

pub(crate) fn get_routes() -> Vec<rocket::Route> {
    rocket::routes![get_anime]
}

#[rocket::get("/anime", format = "json")]
pub(crate) async fn get_anime(config: &State<AppConfig>, _user: RequestUser) -> Result<String> {
    let ost_db = AnimeDB::new(&config.db_path).await?;
    let anime_list = ost_db.get_all_anime()?;
    let json = serde_json::to_string(&anime_list).unwrap();
    Ok(json)
}