use home;
use serde_derive::{Deserialize, Serialize};
use std::fs::read_to_string;
use std::io::Write;
use std::{
    fs::{self, File},
    path::Path,
};
use toml;

#[derive(Deserialize, Serialize)]
pub struct ProConfig {
    pub project_path: String,
    pub code_editor: String,
    pub editor_flags: Vec<String>,
}

fn home_dir() -> Result<String, ()> {
    let home_dir_path = home::home_dir().ok_or_else(|| {
        eprintln!("Can't get your home directory");
    })?;
    Ok(home_dir_path.to_str().unwrap().to_owned())
}

pub fn at_home(subpath: &str) -> Result<String, ()> {
    let home_path = home_dir()?;
    Ok(format!("{home_path}/{subpath}"))
}

pub fn file_exists(filepath: String) -> bool {
    let ref_path: &str = filepath.as_ref();
    let path = Path::new(ref_path);
    path.exists()
}

pub fn config_path() -> Result<String, ()> {
    Ok(at_home(".config/pro/config.toml")?)
}

pub fn create_config_file() -> Result<(), ()> {
    let config_path_string = &config_path()?;
    let config_path = Path::new(config_path_string);
    let prefix = config_path.parent().unwrap();
    fs::create_dir_all(prefix).map_err(|e| {
        eprintln!("Can't create prefix folder because of:\n{e}");
    })?;
    File::create(config_path).map_err(|e| {
        eprintln!("Can't create config because of:\n{e}");
    })?;
    Ok(())
}

pub fn write_config(config: &ProConfig) -> Result<(), ()> {
    let mut config_file = File::options()
        .write(true)
        .open(config_path()?)
        .map_err(|e| eprintln!("Can't open config file because of: {e}\n"))?;
    let default_toml_config = toml::to_string(config).map_err(|e| {
        eprintln!("Can't serialize `ProConfig` to toml because of:\n {e}");
    })?;
    write!(config_file, "{default_toml_config}").map_err(|e| eprintln!("Can't write default config because of: {e}"))?;
    Ok(())
}

pub fn read_config() -> Result<ProConfig, ()> {
    let file_contents = read_to_string(config_path()?).map_err(|e| {
        eprintln!("Can't read config contents because of:\n{e}");
    })?;
    let config: ProConfig = toml::from_str(file_contents.as_ref()).map_err(|e| {
        eprintln!("Can't deserialize config file content because of:\n{e}");
    })?;
    Ok(config)
}
