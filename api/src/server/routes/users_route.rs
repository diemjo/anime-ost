use rocket::{State, response::content};

use crate::{config::AppConfig, server::user::RequestUser, error::{Result, wrap_ok}, proxer::AnimeDB};

pub(crate) fn get_routes() -> Vec<rocket::Route> {
    rocket::routes![
        get_users
    ]
}

#[rocket::get("/users", format = "application/json")]
pub(crate) async fn get_users(config: &State<AppConfig>, _user: RequestUser) -> Result<content::RawJson<String>> {
    let ost_db = AnimeDB::new(&config.db_path).await?;
    let user_list = ost_db.get_all_users()?;
    Ok(wrap_ok(user_list))
}