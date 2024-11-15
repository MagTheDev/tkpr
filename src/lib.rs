use std::{collections::HashMap, path::PathBuf};
use chrono::{DateTime, Duration, Local, Utc};
use rusqlite::{params, Connection};


#[derive(Debug, Clone, Hash)]
pub struct WorkRecord {
    pub id: i32,
    pub date: DateTime<Local>,
    pub description: String,
    pub duration: Duration,
    pub project: String
}

impl WorkRecord {
    pub fn new(id: i32, date: DateTime<Local>, description: String, duration: Duration, project: String) -> Self {
        Self { id, date, description, duration, project }
    }
}



pub struct WorkManager {
    conn: Connection,
    records_cache: HashMap<i32, WorkRecord>,
    active_sessions: HashMap<String, ActiveWorkSession>
}

impl WorkManager {
    pub fn new(location: Option<PathBuf>) -> Self {
        let location = match location {
            Some(location) => location,
            None => "./work.db".into(),
        };
        // This unwrap is fine, because we're handling the location
        let connection = Connection::open(location).unwrap();

        connection.execute(r#"--sql
            CREATE TABLE IF NOT EXISTS work_records (
                id INTEGER PRIMARY KEY,
                date DATETIME NOT NULL,
                description TEXT NOT NULL,
                duration_minutes INTEGER NOT NULL,
                project TEXT NOT NULL
            )
        "#, ()).expect("Error initializing database");
        Self {
            conn: connection,
            records_cache: HashMap::default(),
            active_sessions: HashMap::default()
        }
    }
    pub fn create_work_record(&self, record: WorkRecord) -> Result<usize, rusqlite::Error> {
        self.create_work_record_raw(record.date, record.duration, record.description, record.project)
    }

    pub fn create_work_record_raw(&self, date: DateTime<Local>, duration: Duration, description: String, project: String) -> Result<usize, rusqlite::Error> {
        self.conn.execute(r#"--sql
            INSERT INTO work_records (
                date, description, duration, project
            ) 
            VALUES (
                ?1, ?2, ?3, ?4
            )
        "#, (
            &date.naive_local(),
            description,
            &duration.num_minutes(),
            &project
        ))
    }

    pub fn get_work_record_by_id(&mut self, id: i32) -> Option<WorkRecord> {

        if let Some(record) = self.records_cache.get(&id).cloned() {
            return Some(record)
        }

        let mut record_query = self.conn.prepare_cached(r#"--sql
            SELECT * FROM work_records WHERE id = ?1;
        "#).unwrap();
        let mut record_iter = record_query.query(params![id]).ok()?; 
        let row = record_iter.next().transpose()?.ok()?;

        let record = WorkRecord {
            id: row.get(0).ok()?,
            date: row.get(1).ok()?,
            description: row.get(2).ok()?,
            duration: Duration::seconds(row.get(3).ok()?),
            project: row.get(4).ok()?
        };

        self.records_cache.insert(record.id, record.clone());
        Some(record)
    }

    pub fn get_all_cached_records(&self) -> Vec<WorkRecord> {
        return self.records_cache.values().cloned().collect();
    }

    pub fn get_all_records(&self) -> Vec<WorkRecord> {
        
        let mut record_query = self.conn.prepare_cached(r#"--sql
            SELECT * FROM work_records;
        "#).unwrap();

        let record_iter = record_query.query_map([], |row| {
            Ok(
                WorkRecord {
                    id: row.get(0).unwrap(),
                    date: row.get(1).unwrap(),
                    description: row.get(2).unwrap(),
                    duration: Duration::seconds(row.get(3).unwrap()),
                    project: row.get(4).ok().unwrap()
                }
            )
        }).unwrap();

        let mut records = vec![];
        
        for record in record_iter {
            if let Ok(rec) = record {
                records.push(rec);
            }
        }

        records
    }

    pub fn delete_record_by_id(&self, id: i32) -> Option<()> {

        let mut record_query = self.conn.prepare(r#"--sql
            DELETE FROM work_records WHERE id = ?1
        "#).unwrap();

        record_query.execute([&id]).ok()?;

        Some(())
    }

    pub fn new_session(&mut self, ident: String, project: String) -> ActiveWorkSession {
        let session = ActiveWorkSession::new(ident, None, project);
        self.active_sessions.insert(session.identifier.clone(), session.clone());
        session
    }

    pub fn end_session(&mut self, ident: String, description: Option<String>) -> Option<WorkRecord> {
        let session = self.active_sessions.remove(&ident)?;
        let record = session.end(description);
        self.create_work_record(record.clone()).ok()?;
        Some(record)
    }

    pub fn end_session_without_saving(&mut self, ident: String, description: Option<String>) -> Option<WorkRecord> {
        let session = self.active_sessions.remove(&ident)?;
        let record = session.end(description);
        self.records_cache.insert(record.id, record.clone());
        Some(record)
    }

    pub fn get_active_sessions(&self) -> Vec<ActiveWorkSession> {
        self.active_sessions.values().cloned().collect()
    }
}

#[derive(Debug, Clone)]
pub struct ActiveWorkSession {
    pub identifier: String,
    pub start: DateTime<Local>,
    pub project: String
}

impl ActiveWorkSession {
    pub fn new(identifier: String, start: Option<DateTime<Local>>, project: String) -> Self {
        let start = match start {
            Some(start) => start,
            None => Local::now(),
        };
        Self { identifier, start, project }
    }

    pub fn end(self, description: Option<String>) -> WorkRecord {

        let end_time = Local::now();
        let work_duration = self.start - end_time;

        let description = match description {
            Some(description) => description,
            None => self.identifier,
        };

        WorkRecord::new(-1, self.start, description, work_duration, self.project)
    }
}