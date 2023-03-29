use std::path::{Path, PathBuf};

use rusqlite::Connection;

use crate::{error::{Result, Error}, models::{Anime, AnimeUserEntry, AnimeUser, AnimeOst, OstType}};

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

    // ##########################################################################################################
    // SELECT
    // ##########################################################################################################

    pub(crate) fn get_all_users(&self) -> Result<Vec<AnimeUser>> {
        let user_list = db_calls::get_all_users(&self.conn)?;
        Ok(user_list)
    }

    pub(crate) fn get_all_anime(&self) -> Result<Vec<Anime>> {
        let anime_list = db_calls::get_all_anime(&self.conn)?;
        Ok(anime_list)
    }

    pub(crate) fn get_anime(&self, proxer_id: u32) -> Result<Option<Anime>> {
        let anime = db_calls::get_anime(&self.conn, proxer_id)?;
        Ok(anime)
    }

    pub(crate) fn get_all_user_anime(&self) -> Result<Vec<AnimeUserEntry>> {
        let anime_list = db_calls::get_all_user_anime(&self.conn)?;
        Ok(anime_list)
    }

    pub(crate) fn get_all_ost(&self) -> Result<Vec<AnimeOst>> {
        let ost_list = db_calls::get_all_ost(&self.conn)?;
        Ok(ost_list)
    }

    pub(crate) fn get_ost(&self, proxer_id: u32, ost_type: OstType, number: u32) -> Result<Option<AnimeOst>> {
        let ost = db_calls::get_ost(&self.conn, proxer_id, ost_type, number)?;
        Ok(ost)
    }

    // ##########################################################################################################
    // INSERT
    // ##########################################################################################################
    
    pub(crate) fn insert_anime<A: AsRef<AnimeUserEntry>>(&mut self, anime_list: &[A]) -> Result<()> {
        db_calls::insert_anime(&mut self.conn, anime_list)
    }

    pub(crate) fn insert_ost(&mut self, proxer_id: u32, ost_type: OstType, number: u32, name: Option<String>, artist: Option<String>, video_url: Option<String>) -> Result<()>{
        db_calls::insert_ost(&mut self.conn, proxer_id, ost_type, number, name, artist, video_url)
    }

    // ##########################################################################################################
    // UPDATE
    // ##########################################################################################################

    pub(crate) fn update_ost(&mut self, proxer_id: u32, ost_type: OstType, number: u32, name: Option<String>, artist: Option<String>, video_url: Option<String>) -> Result<()>{
        db_calls::update_ost(&mut self.conn, proxer_id, ost_type, number, name, artist, video_url)
    }

    // ##########################################################################################################
    // DELTETE
    // ##########################################################################################################

    pub(crate) fn delete_ost(&mut self, proxer_id: u32, ost_type: OstType, number: u32) -> Result<()>{
        db_calls::delete_ost(&mut self.conn, proxer_id, ost_type, number)
    }
}