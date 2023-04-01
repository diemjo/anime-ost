use rocket::{State, response::content};

use crate::{config::AppConfig, result::{Result, wrap_ok}, server::user::RequestUser, proxer::AnimeDB};

pub(crate) fn get_routes() -> Vec<rocket::Route> {
    rocket::routes![
        get_useranime
    ]
}

#[rocket::get("/useranime")]
pub(crate) async fn get_useranime(config: &State<AppConfig>, _user: RequestUser) -> Result<content::RawJson<String>> {
    let ost_db = AnimeDB::new(&config.db_path).await?;
    let anime_list = ost_db.get_all_user_anime()?;
    Ok(wrap_ok(anime_list))
}