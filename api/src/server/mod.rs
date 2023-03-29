use serde::{Deserialize, Deserializer};

use crate::{error::Result, config::AppConfig};

mod routes;
mod user;

fn deserialize_error_on_missing<'de, D, T>(deserializer: D) -> std::result::Result<T, D::Error>
where
    T: Default + Deserialize<'de>,
    D: Deserializer<'de> {
        let opt = Option::deserialize(deserializer)?;
        Ok(opt.unwrap_or_default())
    }

pub async fn launch_server(config: AppConfig) -> Result<()>{
    let figment = rocket::Config::figment()
        .merge(("port", config.server_port));

    let _rocket = rocket::custom(figment)
        .mount("/api", routes::get_routes())
        .manage(config)
        .launch()
        .await?;

    Ok(())
}