use std::path::Path;

use rusqlite::{Connection, OpenFlags};

use crate::result::Result;

use super::db_calls;

pub(crate) fn open(db_path: &Path) -> Result<Connection> {
    let mut conn = match Connection::open_with_flags(db_path, OpenFlags::SQLITE_OPEN_READ_WRITE) {
        Ok(conn) => conn,
        Err(_) => {
            let conn = Connection::open_with_flags(db_path, OpenFlags::SQLITE_OPEN_CREATE | OpenFlags::SQLITE_OPEN_READ_WRITE)?;
            conn.pragma_update(None, "foreign_key", "ON")?;
            conn.pragma_update(None, "user_version", 0)?;
            conn
        }
    };
    run_db_updates(&mut conn)?;
    Ok(conn)
}
    
fn run_db_updates(conn: &mut Connection) -> Result<()> {
    let current_db_version: usize = conn.pragma_query_value(None, "user_version", |r| r.get(0))?;
    if current_db_version == 0 {
        db_init_new(conn)?;
        conn.pragma_update(None, "user_version", 1)?;
    } else {
        
    }
    Ok(())
}

fn db_init_new(conn: &mut Connection) -> Result<()> {
    db_calls::create_tables(conn)?;
    Ok(())
}