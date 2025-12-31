use std::{env::current_dir, fs::{DirBuilder, exists}, path::{Path, PathBuf}};
use crate::{cli::InitArgs, db::{Project, init_app_db}};
use anyhow::{Error, Result};
use log::{error, info};

/// prvo proveri da li je first run ili ne
/// kreiraj bazu u xdg_data
/// kreiraj globalni config u xdg_config
/// proveri da li je vec inizijalizovan ovaj projekat
/// inicijalizuj projekat (kreiraj red u tabeli)
pub fn run(args: &InitArgs) -> Result<()> {
    let app_data_path = get_app_data_path(args);

//proveri da li je first run i inicijalizuj aplikaciju
    if !is_app_initialized(app_data_path.as_ref().unwrap()).is_ok() {
        init_app(&app_data_path.unwrap())?;
        //kreiraj globalni konfig
        info!("App initialized");
    }
    let project_name = get_project_name(args);
    let current_dir = get_current_dir_path();
    let project = Project::new(&project_name, current_dir, None, None);
    Ok(())
}

fn get_project_name(args: &InitArgs) -> String {
    match &args.name {
        Some(name) => if name == "CURRENT_DIR_NAME" {
            get_current_dir_path()
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string()
        } else {
            String::from(name)
        },
        None => unreachable!(),
    }
}

fn get_current_dir_path() -> PathBuf {
    current_dir()
        .ok()
        .unwrap()
}

fn get_app_data_path(args: &InitArgs) -> Result<PathBuf, Error> {
    if args.local {
        match &args.dir {
            Some(dir) => {
                if dir == "CURRENT_DIR_PATH" {
                    match current_dir() {
                        Ok(dir) => Ok(dir),
                        Err(e) => Err(Error::new(e))
                    }
                } else {
                    Ok(Path::new(&dir).to_path_buf())
                }
            }
            None => unreachable!(),
        }
    } else {
        match dirs::data_dir() {
            Some(dir) => Ok(dir),
            None => {
                error!("$XDG_DATA_HOME not defined");
                return Err(Error::new(std::io::Error::new(std::io::ErrorKind::NotFound, "$XDG_DATA_HOME not defined")));
            }
        }
    }
}

fn init_app(data_dir: &PathBuf) -> Result<()> {
    create_app_data_dir(&data_dir)?;
    let db_init_res = init_app_db(&data_dir);
    match db_init_res {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

fn create_app_data_dir(data_dir: &PathBuf) -> Result<()> {
    Ok(DirBuilder::new().create(data_dir.join("jari"))?)
}

fn is_app_initialized(data_dir: &PathBuf) -> std::io::Result<bool> {
    exists(Path::new(&data_dir).join("jari"))
}
