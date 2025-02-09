use std::sync::Mutex;
use serde::{Deserialize, Serialize};
use tauri::State;
use rusqlite::{params, Connection, Error};

#[derive(Debug)]
struct Database{
    conn: Connection
}

#[derive(Debug, Serialize, Deserialize)]
struct Record{
    name: String,
    date: String,
    time: String
}

impl Record {
    fn default() -> Record {
        Record { name: "".to_string(), date: "".to_string(), time: "".to_string() }
    }

    fn new(name: String, date: String, time: String) -> Record {
        Record { name ,date, time }
    }
}

impl Database{
    fn new() -> Result<Database, Error>{
        let connection = Connection::open("./database/database.db")?;
        Ok(Database{conn: connection})
    }

    fn initialize(&mut self) -> Result<(), Error>{
        match self.conn.execute("CREATE TABLE IF NOT EXISTS events(
            name TEXT,
            date TEXT,
            time TEXT
        );", []){
            Ok(_) => Ok(()),
            Err(e) => Err(e)
        }
    }

    fn add_record(&mut self, record: Record) -> Result<(), Error>{
        match self.conn.execute("INSERT INTO events(name, date, time) VALUES (?1, ?2, ?3)", params![record.name, record.date, record.time]){
            Ok(_) => Ok(()),
            Err(e) => Err(e)
        }
    }

    fn get_record(&mut self, record: String) -> Result<Record, Error>{
        let mut comm = self.conn.prepare("SELECT name, date, time FROM events WHERE name = ?1")?;
        let ret = comm.query_row([record], |row|{
            Ok(Record{name: row.get(0)?, date: row.get(1)?, time: row.get(2)?})
        })?;
        Ok(ret)
    }

    fn delete_record(&mut self, record: Record) -> Result<Record, Error>{
        match self.conn.execute("DELETE FROM events WHERE name = ?1 AND date = ?2 AND time = ?3", [record.name.clone(), record.date.clone(), record.time.clone()]){
            Ok(_) => Ok(record),
            Err(e) => Err(e)
        }
    }

    fn custom_command(&mut self, command: &str) -> Result<(), Error>{
        match self.conn.execute(command, []){
            Ok(_) => Ok(()),
            Err(e) => Err(e)
        }
    }

    fn get_all_records(&mut self) -> Result<Vec<Record>, Error>{
        let mut comm = self.conn.prepare("SELECT * FROM events")?;
        let mut cols = comm.query([])?;
        let mut vec = Vec::new();
        while let Some(row) = cols.next()? {
            vec.push(Record::new(row.get(0)?, row.get(1)?, row.get(2)?));
        }
        Ok(vec)
    }
}

#[tauri::command]
fn create_database(state: State<'_, Mutex<Option<Database>>>) {
    let mut db = state.lock().unwrap();
    if db.is_none(){
        match Database::new(){
            Ok(database) => {*db = Some(database); println!("db opened correctly")},
            Err(e) => println!("{:?}", e)
        }
        
    } else {
        println!("db already exists");
    }
}

#[tauri::command]
fn initialize_database(state: State<'_, Mutex<Option<Database>>>) {
    let mut db = state.lock().expect("error unpacking mutex");
    if db.is_none(){
        println!("database doesn't exist yet");
    } else {
        if let Some(ref mut database) = *db{
            match database.initialize(){
                Ok(_) => println!("database inizialized"),
                Err(e) =>println!("{:?}", e)
            }
        } else {
            println!("database error");
        }
        
    }
}

#[tauri::command]
fn add_record(record: Record, state: State<'_, Mutex<Option<Database>>>) {
    let mut db = state.lock().expect("error unpacking mutex");
    if db.is_none(){
        println!("database doesn't exist yet");
    } else {
        if let Some(ref mut database) = *db{
            match database.add_record(record){
                Ok(_) => println!("record inserted"),
                Err(e) =>println!("{:?}", e)
            }
        } else {
            println!("database error");
        }
        
    }
}

#[tauri::command]
fn get_record(record: Record, state: State<'_, Mutex<Option<Database>>>) -> Record {
    let mut db: std::sync::MutexGuard<'_, Option<Database>> = state.lock().expect("error unpacking mutex");
    if db.is_none(){
        println!("database doesn't exist yet");
        Record::default()
    } else {
        if let Some(ref mut database) = *db{
            match database.get_record(record.name){
                Ok(record_db) => record_db,
                Err(e) =>{println!("{:?}", e);Record::default()}
            }
        } else {
            println!("database error");
            Record::default()
        }
    }
}

#[tauri::command]
fn delete_record(record: Record, state: State<'_, Mutex<Option<Database>>>) {
    let mut db: std::sync::MutexGuard<'_, Option<Database>> = state.lock().expect("error unpacking mutex");
    if db.is_none(){
        println!("database doesn't exist yet");
    } else {
        if let Some(ref mut database) = *db{
            match database.delete_record(record){
                Ok(rec) => println!("deleted record: {:?}", rec),
                Err(e) =>{println!("{:?}", e)}
            }
        } else {
            println!("database error");
        }
    }
}

#[tauri::command]
fn get_all_records(state: State<'_, Mutex<Option<Database>>>) -> Vec<Record>{
    let mut db = state.lock().expect("error unpacking mutex");
    if db.is_none(){
        println!("database doesn't exist yet");
        Vec::new()
    } else {
        if let Some(ref mut db) = *db {
            match db.get_all_records() {
                Ok(records) => records,
                Err(e) =>{println!("{:?}", e);Vec::new()}
            }
        } else {
            println!("database error");
            Vec::new()
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(Mutex::new(None::<Database>))
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![create_database, initialize_database, get_record, add_record, get_all_records, delete_record])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
