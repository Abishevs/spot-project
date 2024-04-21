use std::{i64, path::PathBuf};

use crate::utils::get_db_path;
use chrono::Utc;
use rusqlite::{params, Connection, Error as RusqliteError, ErrorCode, Result, Transaction};
use spot_lib::models::{Project, Session, Tag};
use std::io;

pub struct DbConnection {
    pub conn: Connection,
}

//Test db
// let project = Project{ 
//     id: None,
//     name: "projecting 1".to_string(),
//     cumulative_time: 0,
//     description: Some("This is my first test project".to_string()),
// };
//
// let res = db_connection.create_project(&project);
// match res {
//     Ok(_) => println!("Project created successfully."),
//     Err(e) => println!("Failed to create project: {}", e),
// }
impl DbConnection {
    pub fn new(config_path: &PathBuf) -> Result<Self> {
        let db_path = get_db_path(&config_path);
        let mut conn = Connection::open(db_path)?;
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
    fn create_project(project: &Project, tx: &mut Transaction) -> Result<Project> {
        match tx.execute(
            "INSERT INTO projects (name, description)
             VALUES (?, ?)",
            params![&project.name.to_lowercase(), 
                    &project.description
            ],
        ) {
            Ok(_) => {
                let id = tx.last_insert_rowid();
                Ok(Project {
                    id: Some(id),
                    name: project.name.clone(),
                    description: project.description.clone(),
                    cumulative_time: 0,
                })
            },
            Err(RusqliteError::SqliteFailure(err, Some(_msg))) if err.code == ErrorCode::ConstraintViolation => {
                Err(RusqliteError::SqliteFailure(err, Some(format!("Project with name '{}' already exists.", &project.name))))
            },
            Err(e) => Err(e),
        }
    }

    pub fn create_project_with_tags(&mut self, project: Project, tags: Vec<Tag>) -> io::Result<String> {
        let mut tx = self.conn.transaction().map_err(|e| {
            io::Error::new(io::ErrorKind::Other, format!("Failed to start transaction: {}", e))
        })?;

        let mut new_tags = Vec::new();
        for tag in tags.iter() {
            match Self::create_tag(&tag, &mut tx) {
                Ok(tag) => new_tags.push(tag),
                Err(_) => {
                    let existing_tag = Self::get_tag_by_name(&tag.name, &mut tx)?;
                    new_tags.push(existing_tag); 
                }
            }
        }
        let new_project = match Self::create_project(&project, &mut tx) {
            Ok(project) => project,
            Err(e) => {
                let _ = tx.rollback(); 
                return Err(io::Error::new(io::ErrorKind::Other, 
                                          format!("Failed to create project: {}", e)));
            }
        };

        for tag in new_tags {
            if let Err(e) = Self::assign_tag_to_project(&tag, &new_project, &mut tx) {
                let _ = tx.rollback(); 
                return Err(io::Error::new(io::ErrorKind::Other,
                                          format!("Failed to assign tag '{}' to project '{}'. Error: {}",
                                                  tag.name, new_project.name, e)));
            }
        }

        tx.commit().map_err(|e| {
            io::Error::new(io::ErrorKind::Other, format!("Failed to commit transaction: {}", e))
        })?;

        Ok(format!("Project '{}' created successfully with tags.", new_project.name))
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
    pub fn start_session(&self, project: &Project) -> Result<Session> {
        let start_time = Utc::now().naive_utc();

        self.conn.execute(
            "INSERT INTO sessions (project_id, start_time) VALUES (?1, ?2)",
            params![project.id, start_time],
            )?;

        let last_id = self.conn.last_insert_rowid();

        Ok(Session {
            id: Some(last_id),
            project_id: project.id.expect("Project has no id, panicking"),
            start_time,
            end_time: None,
        })
    }

    pub fn end_session(&self, session: &Session) -> Result<()>{
        let end_time = Utc::now().naive_utc();

        self.conn.execute(
            "SELECT sessions SET end_time = ? WHERE id = ?",
            params![end_time, session.id]
            )?;

        Ok(())
    }

    pub fn get_session(&self, session: &Session) {
    }

    pub fn list_sessions(&self, project: &Project) {
    }

    // Tag managment
    fn create_tag(tag: &Tag, tx: &mut Transaction) -> Result<Tag> {
        match tx.execute(
            "INSERT INTO tags (name)
             VALUES (?)",
            params![&tag.name.to_lowercase(), 
            ],
        ) {
            Ok(_) => {
                let id = tx.last_insert_rowid();
                Ok(Tag {
                    id: Some(id),
                    name: tag.name.clone(),
                })
            },
            Err(RusqliteError::SqliteFailure(err, 
                                             Some(_msg))) if err.code == ErrorCode::ConstraintViolation => {
                Err(RusqliteError::SqliteFailure(err, 
                                                 Some(format!("Tag with name '{}' already exists.",
                                                              &tag.name.to_lowercase()))))
            },
            Err(e) => Err(e),
        }
    }

    pub fn delete_tag(&self, tag: &Tag) {
    }

    fn assign_tag_to_project(tag: &Tag, project: &Project, tx:&mut Transaction) -> Result<()>{
        let sql = "INSERT INTO project_tags (project_id, tag_id) VALUES (?1, ?2)";
        tx.execute(sql, params![project.id, tag.id])?;
        Ok(())
    }

    pub fn remove_tag_from_project(&self, tag: &Tag, project: &Project) {
    }

    fn get_tag_by_name(name: &str, tx: &mut Transaction) -> Result<Tag, io::Error> {
        tx.query_row("SELECT id, name FROM tags WHERE name = ?", &[name], |row| {
            Ok(Tag {
                id: row.get(0)?,
                name: row.get(1)?,
            })
        }).map_err(|e| {
            io::Error::new(io::ErrorKind::Other, format!("Failed to fetch tag '{}': {}", name, e))
        })
    }

    pub fn list_tags(&self) -> Result<Vec<Tag>>{
        let mut stmt = self.conn.prepare("SELECT * FROM tags")?;
        let tags_iter = stmt.query_map([], |row| {
            Ok(Tag {
                id: row.get(0)?,
                name: row.get(1)?,
            })
        })?;

        let mut tags = Vec::new();
        for tag in tags_iter {
            tags.push(tag?);
        }
        Ok(tags)
    }

}

