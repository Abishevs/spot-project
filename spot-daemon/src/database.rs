use std::path::PathBuf;

use crate::utils::get_db_path;
// use rusqlite::{Connection, Result, params};
use rusqlite::{params, Connection, Result, Error as RusqliteError, ErrorCode};
use spot_lib::models::Project;

pub struct DbConnection {
    conn: Connection,
}

impl DbConnection {
    pub fn new(config_path: &PathBuf) -> Result<Self> {
        let db_path = get_db_path(&config_path);
        let conn = Connection::open(db_path)?;
        // Initialize connection
        let _ = Self::init_db(&conn);
        Ok(Self { conn } )
    }

    // Private init func
    fn init_db(conn: &Connection) -> Result<()> {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS tags (
                id INTEGER PRIMARY KEY,
                name TEXT UNIQUE NOT NULL
                )",
            [],
            )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS projects (
                id INTEGER PRIMARY KEY,
                name TEXT UNIQUE NOT NULL,
                cumulative_time INTEGER DEFAULT 0,
                description TEXT
                )",
            [],
            )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS sessions (
                id INTEGER PRIMARY KEY,
                project_id INTEGER NOT NULL,
                start_time TIMESTAMP NOT NULL,
                end_time TIMESTAMP,
                FOREIGN KEY (project_id) REFERENCES projects(id)
                )",
            [],
            )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS project_tags (
                project_id INTEGER NOT NULL,
                tag_id INTEGER NOT NULL,
                PRIMARY KEY (project_id, tag_id),
                FOREIGN KEY (project_id) REFERENCES projects(id),
                FOREIGN KEY (tag_id) REFERENCES tags(id)
                )",
            [],
            )?;

        Ok(())
    }

    // Project managment
    pub fn create_project(&self, project: &Project) -> Result<()> {
        match self.conn.execute(
            "INSERT INTO projects (name, description)
             VALUES (?, ?)",
            params![&project.name.to_lowercase(), 
                    &project.description
            ],
        ) {
            Ok(_) => Ok(()),
            Err(RusqliteError::SqliteFailure(err, Some(_msg))) if err.code == ErrorCode::ConstraintViolation => {
                Err(RusqliteError::SqliteFailure(err, Some(format!("Project with name '{}' already exists.", &project.name))))
            },
            Err(e) => Err(e),
        }
    }

    pub fn update_project(&self, project: &Project) {
    }

    pub fn delete_project(&self, project: &Project) {
    }

    pub fn get_project(&self, project: &Project) {
    }

    pub fn list_projects(&self) -> Result<Vec<Project>>{
        let mut stmt = self.conn.prepare("SELECT * FROM projects")?;
        let project_iter = stmt.query_map([], |row| {
            Ok(Project {
                id: row.get(0)?,
                name: row.get(1)?,
                cumulative_time: row.get(2)?,
                description: row.get(3)?,
            })
        })?;

        let mut projects = Vec::new();
        for project in project_iter {
            projects.push(project?);
        }
        Ok(projects)
    }

    // Session management
    pub fn start_session(&self, project_id: i32) {
    }

    pub fn end_session(&self, session_id: i32){
    }

    pub fn get_session(&self, session_id: i32) {
    }

    pub fn list_sessions(&self, project_id: i32) {
    }

    // Tag managment
    pub fn create_tag(&self, name: String) {
    }

    pub fn delete_tag(&self, id: i32) {
    }

    pub fn assign_tag_to_project(&self, tag_id: i32, project_id: i32){
    }

    pub fn remove_tag_from_project(&self, tag_id: i32, project_id: i32) {
    }

    pub fn list_tags(&self) {
    }

}
