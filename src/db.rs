use std::path::PathBuf;
use anyhow::Error;
use log::{error, info};
use rusqlite::{Connection};
use uuid::Uuid;

#[derive(Debug)]
pub struct Project {
    id: String,
    name: String,
    path: String,
    project_prefix: Option<String>,
    enumerator_value: Option<u32>,
}

impl Project {
    pub fn new(name: &str, path: &str, project_prefix: Option<String>, enumerator_value: Option<u32>) -> Project {
        Project {
            id: Uuid::new_v4().to_string(),
            name: name.to_string(),
            path: path.to_string(),
            project_prefix: project_prefix,
            enumerator_value: enumerator_value
        }
    }
}

fn get_db_conn(db_path: &PathBuf) -> Result<Connection, Error> {
    let path = db_path.join("jari.db3");
    match Connection::open(path) {
        Ok(conn) => Ok(conn),
        Err(e) => Err(Error::new(e)),
    }
}

pub fn init_app_db(db_path: &PathBuf) -> Result<(), Error> {
    let conn = get_db_conn(db_path)?;
    match conn.execute(
        "create table if not exists project (
id text primary key,
name text not null,
path text not null,
project_prefix text not null,
enumerator_value integer
)", ()) {
        Ok(_) => info!("created project table"),
        Err(e) => {
            error!("error creating project table");
            return Err(Error::new(e));
        }
    }


   match conn.execute("
create table if not exists task (
id text primary key,
proj_id text not null,
title text not null,
path text not null,
file_name text not null,
created_at text default current_timestamp,
status text not null default TODO,
status_change_date text default current_timestamp,
context text,
description text,
foreign key (proj_id) references project(id)
)", ()) {
        Ok(_) => {
            info!("created task table");
            Ok(())
        },
        Err(e) => {
            error!("error creating task table");
            return Err(Error::new(e));
        }
    }
}

pub fn create_project(db_path: &PathBuf, project: &Project) {
    let conn = get_db_conn(db_path);
}
