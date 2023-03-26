use rusqlite::{Connection, named_params};

use crate::{error::Result, models::{Anime, AnimeUserEntry, AnimeUser}};

use super::sqlite_statements;

pub(crate) fn create_tables(conn: &mut Connection) -> Result<()> {
    conn.execute_batch(sqlite_statements::DB_INIT_TABLES)?;
    Ok(())
}

pub(crate) fn get_all_users(conn: &Connection) -> Result<Vec<AnimeUser>> {
    let mut stmt = conn.prepare(sqlite_statements::SELECT_ALL_USERS)?;
    let rows = stmt.query_map([], |row| {
        Ok(AnimeUser::new(
            row.get(0)?,
            row.get(1)?,
        ))
    })?.collect::<std::result::Result<Vec<AnimeUser>, rusqlite::Error>>()?;
    Ok(rows)
}

pub(crate) fn get_all_anime(conn: &Connection) -> Result<Vec<Anime>> {
    let mut stmt = conn.prepare(sqlite_statements::SELECT_ALL_ANIME)?;
    let rows = stmt.query_map([], |row| {
        Ok(Anime::new(
            row.get(0)?,
            row.get(1)?,
            row.get(2)?,
            row.get(3)?,
        ))
    })?.collect::<std::result::Result<Vec<Anime>, rusqlite::Error>>()?;
    Ok(rows)
}

pub(crate) fn get_all_user_anime(conn: &Connection) -> Result<Vec<AnimeUserEntry>> {
    let mut stmt = conn.prepare(sqlite_statements::SELECT_ALL_USER_ANIME)?;
    let rows = stmt.query_map([], |row| {
        Ok(AnimeUserEntry::new(
            Anime::new(
                row.get(3)?,
                row.get(4)?,
                row.get(5)?,
                row.get(6)?,
            ),
            AnimeUser::new(
                row.get(0)?,
                row.get(1)?,
            ),
            row.get(2)?,
        ))
    })?.collect::<std::result::Result<Vec<AnimeUserEntry>, rusqlite::Error>>()?;
    Ok(rows)
}

pub(crate) fn insert_anime<A: AsRef<AnimeUserEntry>>(conn: &mut Connection, anime_list: &[A]) -> Result<()> {
    let transaction = conn.transaction()?;
    
    {
        if anime_list.is_empty() {
            return Ok(());
        }

        let mut user_stmt = transaction.prepare(sqlite_statements::INSERT_USER)?;
        let mut user_anime_stmt = transaction.prepare(sqlite_statements::INSERT_USER_ANIME)?;
        let mut anime_stmt = transaction.prepare(sqlite_statements::INSERT_ANIME)?;
        
        let user = anime_list.first().unwrap().as_ref().user();
        user_stmt.execute(named_params! {
            ":user_id": user.user_id(),
            ":user_name": user.user_name(),
        })?;

        let now = chrono::Utc::now().naive_utc().format("%Y-%m-%d-%H-%M-%S").to_string();
        for user_anime in anime_list {
            let user_anime = user_anime.as_ref();
            let anime = user_anime.anime();
            anime_stmt.execute(named_params!{
                ":proxer_id": anime.proxer_id(),
                ":proxer_name": anime.proxer_name(),
                ":mal_id": anime.mal_id(),
                ":episode_count": anime.episode_count(),
            })?;
            user_anime_stmt.execute(named_params! {
                ":user_id": user_anime.user().user_id(),
                ":episode_progress": user_anime.progress(),
                ":proxer_id": anime.proxer_id(),
                ":date_added": &now,
            })?;
        }
    }

    transaction.commit()?;
    Ok(())
}