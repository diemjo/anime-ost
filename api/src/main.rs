use crate::config::AppConfig;
use crate::result::Result;

mod proxer;
mod db;
mod config;
mod server;
mod common;
mod models;

pub use common::error;
pub use common::result;

#[rocket::main]
async fn main() -> Result<()> {
    let config = AppConfig::load_config()?;
    //println!("{:?}", config);
    server::launch_server(config).await?;
    Ok(())
}