use chrono::{DateTime, FixedOffset};
use rusqlite::{Connection, params};

pub struct WxDatabase {
    cxn: Connection,
}

impl WxDatabase {
    pub fn new() -> Self {
        match Connection::open("db/wxscripts.sqlite3") {
            Ok(cxn) => Self { cxn },
            Err(e) => panic!("Connection couldn't be created: {}", e),
        }
    }

    /// Upsert a `Sensor` into the SQLite database. If the `sensors` table does not exist,
    /// create it first. Returns a `rusqlite::Result` that indicates how many rows have been
    /// modified.
    ///
    /// # Arguments
    ///
    /// * `sensor_name` - The string name of the `Sensor` that is being inserted.
    pub fn insert_sensor(&self, sensor_name: &str) -> rusqlite::Result<usize> {
        self.cxn.execute("
            CREATE TABLE IF NOT EXISTS sensors(\
            id INTEGER PRIMARY KEY,\
            name STRING NOT NULL\
            )", ())?;

        self.cxn.execute(
            "INSERT INTO sensors (name) VALUES (?1)",
            params![sensor_name]
        )
    }

    pub fn insert_sensor_entry<T: rusqlite::ToSql>(&self, for_sensor: &str, timestamp: DateTime<FixedOffset>, args: T, data: T) -> rusqlite::Result<usize> {
        self.cxn.execute("
            CREATE TABLE IF NOT EXISTS sensor_entries(\
            id INTEGER PRIMARY_KEY,\
            sensor_id INTEGER NOT NULL,\
            timestamp TEXT NOT NULL,\
            arguments TEXT NULL,\
            data TEXT NULL\
            )", ())?;

        // TODO: Fetch sensor id for sensor
        let sensor_id = 1;

        // TODO: JSON serialize data

        self.cxn.execute(
            "INSERT INTO sensor_entries (a, b, c) VALUES (?1, ?2, ?3)",
            params![sensor_id, timestamp.to_rfc3339(), data]
        )
    }
}
