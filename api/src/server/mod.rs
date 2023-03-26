use crate::{error::Result, config::AppConfig};

mod routes;
mod user;

pub async fn launch_server(config: AppConfig) -> Result<()>{
    let figment = rocket::Config::figment()
        .merge(("port", config.server_port));

    let _rocket = rocket::custom(figment)
        .mount("/api", rocket::routes![
            routes::get_proxer,
            routes::get_anime,
            routes::get_useranime,
            routes::get_users,
        ])
        .manage(config)
        .launch()
        .await?;

    Ok(())
}