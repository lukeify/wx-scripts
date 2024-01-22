use chrono::{DateTime, FixedOffset};
use rusqlite::{Connection, params};
use crate::sensors::Sensor;

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
    pub fn insert_sensor(&self, sensor: &Sensor) -> rusqlite::Result<usize> {
        self.cxn.execute("
            CREATE TABLE IF NOT EXISTS sensors(\
            id INTEGER PRIMARY KEY,\
            name STRING NOT NULL\
            )", ())?;

        self.cxn.execute(
            "INSERT INTO sensors (name) VALUES (?1)",
            params![sensor.to_string()]
        )
    }

    pub fn get_sensor_id(&self, sensor: &Sensor) -> Result<Option<u8>, rusqlite::Error> {
        let mut stmt = self.cxn.prepare("SELECT id FROM sensors WHERE name= ?1")?;
        let rows = stmt.query_map([sensor.to_string()], |row| row.get(0))?;

        let mut sensor_ids: Vec<u8> = Vec::new();
        for id_result in rows {
            sensor_ids.push(id_result?);
        }

        Ok(sensor_ids.first().cloned())
    }

    pub fn insert_sensor_arrangement<T: rusqlite::ToSql>(&self, sensor: &Sensor, args: T) -> Result<(), rusqlite::Error> {
        self.cxn.execute("CREATE TABLE IF NOT EXISTS sensor_arrangements(\
            id INTEGER PRIMARY KEY,\
            sensor_id INTEGER NOT NULL,\
            arguments TEXT NOT NULL\
            )", ()).expect("TODO: panic message");

        if let Some(sensor_id) = self.get_sensor_id(sensor)? {
            self.cxn.execute(
                "INSERT INTO sensor_arrangements (sensor_id, arguments) VALUES (?1, ?2)",
                params![sensor_id, args]
            )?;
            Ok(())
        } else {
            Err(rusqlite::Error::QueryReturnedNoRows)
        }
    }

    pub fn insert_sensor_entry<T: rusqlite::ToSql>(&self, for_sensor: &Sensor, timestamp: DateTime<FixedOffset>, data: T) -> rusqlite::Result<usize> {
        self.cxn.execute("
            CREATE TABLE IF NOT EXISTS sensor_entries(\
            id INTEGER PRIMARY_KEY,\
            sensor_id INTEGER NOT NULL,\
            timestamp TEXT NOT NULL,\
            arguments TEXT NULL,\
            data TEXT NULL\
            )", ())?;

        if let Some(sensor_id) = self.get_sensor_id(for_sensor)? {
            self.cxn.execute(
                "INSERT INTO sensor_entries (a, b, c) VALUES (?1, ?2, ?3)",
                params![sensor_id, timestamp.to_rfc3339(), data]
            )
        } else {
            Err(rusqlite::Error::QueryReturnedNoRows)
        }
    }
}
