use dirs::config_dir;
use mlua::{Lua, Table};
use std::error::Error;
use std::path::PathBuf;
use std::{env, fs};

pub struct Config {
    pub config: Table,
}

pub fn load<'lua>(lua: &'lua Lua, pkg_manager: &str) -> Result<Config, Box<dyn Error>> {
    let cfg_dir = match env::var_os("XDG_CONFIG_HOME").map(|x| PathBuf::from(x)) {
        Some(p) => p,
        None => match config_dir() {
            Some(p) => p,
            None => return Err("Failed to find config directory".into()),
        },
    }
    .join("pacx");

    let config_file: String = fs::read_to_string(cfg_dir.join(pkg_manager.to_owned() + ".lua"))?;

    let config: Table = lua.load(&config_file).eval()?;
    let lua_runner = Config { config };

    Ok(lua_runner)
}
