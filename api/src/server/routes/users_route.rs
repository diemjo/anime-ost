use rocket::{State};

use crate::{config::AppConfig, server::user::RequestUser, error::Result, proxer::AnimeDB};

#[rocket::get("/users")]
pub(crate) async fn get_users(config: &State<AppConfig>, _user: RequestUser) -> Result<String> {
    let ost_db = AnimeDB::new(&config.db_path).await?;
    let user_list = ost_db.get_all_users()?;
    let json = serde_json::to_string(&user_list).unwrap();
    Ok(json)
}