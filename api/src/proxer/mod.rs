use crate::models::{Anime, AnimeUserEntry};
use crate::{config::AppConfig, error::Result};
pub(crate) use crate::db::AnimeDB;

use proxer_client::ProxerClient;

pub mod proxer_client;
pub mod html;


pub(crate) async fn refresh_proxer(ost_db: &mut AnimeDB, config: &AppConfig) -> Result<()> {
    let mut client = ProxerClient::new(&config.proxer_username, &config.proxer_password)?;
    for user in config.proxer_users.iter() {
        println!("Querying anime for user {}", user);
        let user_anime_list = client.search_anime(*user).await?;
        ost_db.insert_anime(&user_anime_list)?;
    }
    Ok(())
}