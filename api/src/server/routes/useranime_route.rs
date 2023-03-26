use rocket::State;

use crate::{config::AppConfig, error::Result, server::user::RequestUser, proxer::AnimeDB};

pub(crate) fn get_routes() -> Vec<rocket::Route> {
    rocket::routes![get_useranime]
}

#[rocket::get("/useranime", format = "json")]
pub(crate) async fn get_useranime(config: &State<AppConfig>, _user: RequestUser) -> Result<String> {
    let ost_db = AnimeDB::new(&config.db_path).await?;
    let anime_list = ost_db.get_all_user_anime()?;
    let json = serde_json::to_string(&anime_list).unwrap();
    Ok(json)
}