use crate::{error::Result, config::AppConfig};

mod routes;
mod user;

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