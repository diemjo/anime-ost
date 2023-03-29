use rusqlite::{Connection, named_params};

use crate::{error::Result, models::{Anime, AnimeUserEntry, AnimeUser, AnimeOst, OstType}};

use super::sqlite_statements;

// ##########################################################################################################
// INIT
// ##########################################################################################################

pub(crate) fn create_tables(conn: &mut Connection) -> Result<()> {
    conn.execute_batch(sqlite_statements::DB_INIT_TABLES)?;
    Ok(())
}

// ##########################################################################################################
// SELECT
// ##########################################################################################################

pub(crate) fn get_all_users(conn: &Connection) -> Result<Vec<AnimeUser>> {
    let mut stmt = conn.prepare(sqlite_statements::SELECT_ALL_USERS)?;
    let rows = stmt.query_map([], row_to_anime_user)?.collect::<std::result::Result<Vec<AnimeUser>, rusqlite::Error>>()?;
    Ok(rows)
}

pub(crate) fn get_all_anime(conn: &Connection) -> Result<Vec<Anime>> {
    let mut stmt = conn.prepare(sqlite_statements::SELECT_ALL_ANIME)?;
    let rows = stmt.query_map([], row_to_anime)
        ?.collect::<std::result::Result<Vec<Anime>, rusqlite::Error>>()?;
    Ok(rows)
}

pub(crate) fn get_anime(conn: &Connection, proxer_id: u32) -> Result<Option<Anime>> {
    let mut stmt = conn.prepare(sqlite_statements::SELECT_ANIME)?;
    println!("get_anime: proxer_id={}", proxer_id);
    let row = stmt.query_row(
        named_params! { ":proxer_id": proxer_id},
        row_to_anime
    );
    println!("get_anime: row={:?}", row);
    let res = match row {
        Ok(row) => Ok(Some(row)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(e)
    }?;
    println!("get_anime: res={:?}", res);
    Ok(res)
}

pub(crate) fn get_all_user_anime(conn: &Connection) -> Result<Vec<AnimeUserEntry>> {
    let mut stmt = conn.prepare(sqlite_statements::SELECT_ALL_USER_ANIME)?;
    let rows = stmt.query_map([], row_to_anime_user_entry)
        ?.collect::<std::result::Result<Vec<AnimeUserEntry>, rusqlite::Error>>()?;
    Ok(rows)
}

pub(crate) fn get_all_ost(conn: &Connection) -> Result<Vec<AnimeOst>> {
    let mut stmt = conn.prepare(sqlite_statements::SELECT_ALL_OST)?;
    let rows = stmt.query_map([], row_to_anime_ost)?.collect::<std::result::Result<Vec<AnimeOst>, rusqlite::Error>>()?;
    Ok(rows)
}

pub(crate) fn get_ost(conn: &Connection, proxer_id: u32, ost_type: OstType, number: u32) -> Result<Option<AnimeOst>> {
    let mut stmt = conn.prepare(sqlite_statements::SELECT_OST)?;
    let row = stmt.query_row(
        named_params! { ":proxer_id": proxer_id, ":ost_type": ost_type, ":number": number },
        row_to_anime_ost
    );
    let res = match row {
        Ok(row) => Ok(Some(row)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(e)
    }?;
    Ok(res)
}

// ##########################################################################################################
// INSERT
// ##########################################################################################################

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

pub(crate) fn insert_ost(conn: &mut Connection, proxer_id: u32, ost_type: OstType, number: u32, name: Option<String>, artist: Option<String>, video_url: Option<String>) -> Result<()> {
    let transaction = conn.transaction()?;

    {
        let mut stmt = transaction.prepare(sqlite_statements::INSERT_OST)?;
        stmt.execute(named_params! {
            ":proxer_id": proxer_id,
            ":ost_type": ost_type,
            ":number": number,
            ":name": name,
            ":artist": artist,
            ":video_url": video_url,
        })?;
    }

    transaction.commit()?;
    Ok(())
}

// ##########################################################################################################
// UPDATE
// ##########################################################################################################

pub(crate) fn update_ost(conn: &mut Connection, proxer_id: u32, ost_type: OstType, number: u32, name: Option<String>, artist: Option<String>, video_url: Option<String>) -> Result<()> {
    let transaction = conn.transaction()?;

    {
        let mut stmt = transaction.prepare(sqlite_statements::UPDATE_OST)?;
        stmt.execute(named_params! {
            ":proxer_id": proxer_id,
            ":ost_type": ost_type,
            ":number": number,
            ":name": name,
            ":artist": artist,
            ":video_url": video_url,
        })?;
    }

    transaction.commit()?;
    Ok(())
}

// ##########################################################################################################
// DELETE
// ##########################################################################################################

pub(crate) fn delete_ost(conn: &mut Connection, proxer_id: u32, ost_type: OstType, number: u32) -> Result<()> {
    let transaction = conn.transaction()?;

    {
        let mut stmt = transaction.prepare(sqlite_statements::DELETE_OST)?;
        stmt.execute(named_params! {
            ":proxer_id": proxer_id,
            ":ost_type": ost_type,
            ":number": number,
        })?;
    }

    transaction.commit()?;
    Ok(())
}

// ##########################################################################################################
// HELPERS
// ##########################################################################################################

fn row_to_anime_user(row: &rusqlite::Row) -> rusqlite::Result<AnimeUser> {
    Ok(AnimeUser::new(
        row.get(0)?,
        row.get(1)?,
    ))
}

fn row_to_anime(row: &rusqlite::Row) -> rusqlite::Result<Anime> {
    Ok(Anime::new(
        row.get(0)?,
        row.get(1)?,
        row.get(2)?,
        row.get(3)?,
    ))
}

fn row_to_anime_user_entry(row: &rusqlite::Row) -> rusqlite::Result<AnimeUserEntry> {
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
}

fn row_to_anime_ost(row: &rusqlite::Row) -> rusqlite::Result<AnimeOst> {
    Ok(AnimeOst::new(
        row.get(0)?,
        row.get(1)?,
        row.get(2)?,
        row.get(3)?,
        row.get(4)?,
        row.get(5)?,
    ))
}