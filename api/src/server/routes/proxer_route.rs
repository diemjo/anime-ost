use std::vec;

use rocket::{State, response::content};

use crate::{config::AppConfig, error::{Result, wrap_ok}, proxer::{AnimeDB, refresh_proxer}};

pub(crate) fn get_routes() -> Vec<rocket::Route> {
    rocket::routes![
        get_proxer
    ]
}

#[rocket::get("/proxer")]
pub(crate) async fn get_proxer(config: &State<AppConfig>) -> Result<content::RawJson<String>> {
    let mut ost_db = AnimeDB::new(&config.db_path).await?;
    let res = refresh_proxer(&mut ost_db, config).await.map(|()| String::from("Success"))?;
    Ok(wrap_ok(res))
}