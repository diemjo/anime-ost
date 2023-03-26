use std::path::{Path, PathBuf};

use rusqlite::Connection;

use crate::{error::{Result, Error}, models::{Anime, AnimeUserEntry, AnimeUser}};

mod db_init;
mod db_calls;
mod sqlite_statements;

pub(crate) struct AnimeDB {
    conn: Connection
}

impl AnimeDB {
    pub(crate) async fn new(db_path: &Path) -> Result<Self> {
        let mut path_buf = PathBuf::from(db_path);
        if path_buf.exists() && path_buf.is_dir() {
            path_buf.push("anime-ost.db");
        }
        tokio::fs::create_dir_all(path_buf.parent().unwrap()).await
            .or_else(|e| Err(Error::DirectoryCreateError(e)))?;
        let conn = db_init::open(path_buf.as_path())?;
        Ok( AnimeDB { conn })
    }

    pub(crate) fn get_all_users(&self) -> Result<Vec<AnimeUser>> {
        let user_list = db_calls::get_all_users(&self.conn)?;
        Ok(user_list)
    }

    pub(crate) fn get_all_anime(&self) -> Result<Vec<Anime>> {
        let anime_list = db_calls::get_all_anime(&self.conn)?;
        Ok(anime_list)
    }

    pub(crate) fn get_all_user_anime(&self) -> Result<Vec<AnimeUserEntry>> {
        let anime_list = db_calls::get_all_user_anime(&self.conn)?;
        Ok(anime_list)
    }

    pub(crate) fn insert_anime<A: AsRef<AnimeUserEntry>>(&mut self, anime_list: &[A]) -> Result<()> {
        db_calls::insert_anime(&mut self.conn, anime_list)
    }
}