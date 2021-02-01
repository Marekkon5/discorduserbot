use rusqlite::{params, Connection, Row};
use std::error::Error;

pub struct Filters {
    db: Connection
}

impl Filters {
    //Open
    pub fn load() -> Result<Filters, Box<dyn Error>> {
        let conn = Connection::open("filters.db")?;
        
        //Create table
        conn.execute(
            "CREATE TABLE IF NOT EXISTS filters (
                id          INTEGER PRIMARY KEY AUTOINCREMENT,
                channelId   INTEGER,
                search      TEXT NOT NULL,
                replace     TEXT NOT NULL,
                filename    TEXT,
                data        BLOB
            );",
            params![]
        )?;

        Ok(Filters {
            db: conn
        })
    }

    //Create new filter
    pub fn create_new(&self, channel_id: i64, search: &str, replace: String, filename: Option<String>, url: Option<String>) -> Result<(), Box<dyn Error>> {
        //File
        if url.is_some() {
            let data = reqwest::blocking::get(&url.unwrap())?.bytes()?.to_vec();
            self.db.execute(
                "INSERT INTO filters (channelId, search, replace, filename, data) VALUES (?1, ?2, ?3, ?4, ?5)",
                params![channel_id, search, replace, filename, data]
            )?;
            return Ok(());
        }
        
        //Save filter to DB
        self.db.execute(
            "INSERT INTO filters (channelId, search, replace) VALUES (?1, ?2, ?3)",
            params![channel_id, search, replace]
        )?;

        Ok(())
    }

    //Match message filter
    pub fn match_filter(&self, channel_id: i64, message: &str) -> Result<Filter, Box<dyn Error>> {
        let result = self.db.query_row(
            "SELECT id, channelId, search, replace, filename, data, instr(?1, search) pos FROM filters WHERE pos > 0 AND channelId = ?2",
            params![message, channel_id],
            Filter::from_sql
        )?;
        Ok(result)
    }

    //Get all filters for channel
    pub fn get_all(&self, channel_id: i64) -> Result<Vec<Filter>, Box<dyn Error>> {
        let mut stmt = self.db.prepare("SELECT id, channelId, search, replace, filename, data FROM filters WHERE channelId = ?")?;
        let results = stmt.query_map(params![channel_id], |row| Filter::from_sql(row))?;
        let filters = results.map(|r| r.unwrap()).collect();
        Ok(filters)
    }
}

#[derive(Debug)]
pub struct Filter {
    pub id: i32,
    pub channel_id: i64,
    pub search: String,
    pub replace: String,
    pub filename: Option<String>,
    pub data: Option<Vec<u8>>
}

impl Filter {
    pub fn from_sql(row: &Row) -> Result<Filter, rusqlite::Error> {
        Ok(Filter {
            id: row.get(0)?,
            channel_id: row.get(1)?,
            search: row.get(2)?,
            replace: row.get(3)?,
            filename: row.get(4)?,
            data: row.get(5)?
        })
    }
}