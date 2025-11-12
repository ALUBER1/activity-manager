use std::fs::{self, File};

use crate::{
    models::record::Record,
    schema::schema::events::{self, dsl::*},
};
use ::uuid::Uuid;
use diesel::{result::Error, Connection, *};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use tauri::{AppHandle, Manager};

pub struct Database {
    conn: SqliteConnection,
}

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("../migrations");

impl Database {
    pub fn new(app: AppHandle) -> Result<Database, ConnectionError> {
        let mut path = app.path().app_data_dir().unwrap();
        path.push("database");
        fs::create_dir_all(&path).expect("Failed to create app data dir");
        path.push("database.db");

        let mut connection = SqliteConnection::establish(path.to_str().unwrap())?;
        match connection.run_pending_migrations(MIGRATIONS) {
            Ok(_) => println!("all ok"),
            Err(e) => println!("problem applying migration: {}", e),
        }
        Ok(Database { conn: connection })
    }

    pub fn add_record(&mut self, mut record: Record) -> Result<Record, Error> {
        record.uuid = Uuid::new_v4().to_string();
        diesel::insert_into(events::table)
            .values(&record)
            .returning(Record::as_returning())
            .get_result(&mut self.conn)
    }

    pub fn delete_record(&mut self, record: Record) -> Result<Record, Error> {
        diesel::delete(events::table.filter(uuid.eq(record.uuid.clone())))
            .execute(&mut self.conn)?;
        Ok(record)
    }

    pub fn update_record(&mut self, record: Record) -> Result<(), Error> {
        diesel::update(events::table.filter(uuid.eq(record.uuid.clone())))
            .set(record)
            .execute(&mut self.conn)?;
        Ok(())
    }

    pub fn get_all_records(&mut self) -> Result<Vec<Record>, Error> {
        events
            .select(Record::as_select())
            .order_by((date.desc(), time.desc()))
            .load(&mut self.conn)
    }
}
