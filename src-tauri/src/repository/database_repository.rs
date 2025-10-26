use rusqlite::{params, Connection, Error};
use shared::models::record::Record;
use tauri::{AppHandle, Manager};
use uuid::Uuid;

#[derive(Debug)]
pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new(app: AppHandle) -> Result<Database, Error> {
        let mut path = app.path().app_data_dir().unwrap();
        path.push("database");
        std::fs::create_dir_all(&path).expect("Failed to create app data dir");
        path.push("database.db");
        let connection = Connection::open(path)?;
        Ok(Database { conn: connection })
    }

    pub fn initialize(&mut self) -> Result<(), Error> {
        match self.conn.execute(
            "CREATE TABLE IF NOT EXISTS events(
            uuid TEXT,
            name TEXT,
            date TEXT,
            time TEXT,
            notified_at TEXT
        );",
            [],
        ) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    pub fn add_record(&mut self, record: Record) -> Result<(), Error> {
        match self.conn.execute(
            "INSERT INTO events(uuid, name, date, time, notified_at) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                Uuid::new_v4().to_string(),
                record.name,
                record.date,
                record.time,
                record.notified_at
            ],
        ) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    pub fn get_record(&mut self, record: String) -> Result<Record, Error> {
        let mut comm = self
            .conn
            .prepare("SELECT name, date, time, notified_at FROM events WHERE uuid = ?1")?;
        let ret = comm.query_row([record], |row| {
            Ok(Record::new(
                row.get(0)?,
                row.get(1)?,
                row.get(2)?,
                row.get(3)?,
                row.get(4)?,
            ))
        })?;
        Ok(ret)
    }

    pub fn delete_record(&mut self, record: Record) -> Result<Record, Error> {
        match self
            .conn
            .execute("DELETE FROM events WHERE uuid = ?1", [record.uuid.clone()])
        {
            Ok(_) => Ok(record),
            Err(e) => Err(e),
        }
    }

    pub fn update_record(&mut self, record: Record) -> Result<(), Error> {
        match self.conn.execute(
            "UPDATE events SET name=?1, date=?2, time=?3, notified_at=?5 WHERE uuid = ?4",
            [
                record.name,
                record.date,
                record.time,
                record.uuid,
                record.notified_at,
            ],
        ) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    pub fn get_all_records(&mut self) -> Result<Vec<Record>, Error> {
        let mut comm = self.conn.prepare("SELECT * FROM events")?;
        let mut cols = comm.query([])?;
        let mut vec = Vec::new();
        while let Some(row) = cols.next()? {
            vec.push(Record::new(
                row.get(0)?,
                row.get(1)?,
                row.get(2)?,
                row.get(3)?,
                row.get(4)?,
            ));
        }
        Ok(vec)
    }
}
