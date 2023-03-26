use crate::error::Result;
use crate::config::AppConfig;

mod proxer;
mod db;
mod config;
mod server;
mod error;
mod models;

#[rocket::main]
async fn main() -> Result<()> {
    let config = AppConfig::load_config()?;
    //println!("{:?}", config);
    server::launch_server(config).await?;
    Ok(())
}