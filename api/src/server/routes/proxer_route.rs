use rocket::State;

use crate::{config::AppConfig, error::Result, proxer::{AnimeDB, refresh_proxer}};

#[rocket::get("/proxer")]
pub(crate) async fn get_proxer(config: &State<AppConfig>) -> Result<String> {
    let mut ost_db = AnimeDB::new(&config.db_path).await?;
    refresh_proxer(&mut ost_db, config).await.map(|()| String::from("Success"))
}