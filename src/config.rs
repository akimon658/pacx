use dirs::config_dir;
use mlua::{FromLua, Lua, Table, Value};
use std::error::Error;
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use std::{env, fs};

pub struct Config {
    pub config: Table,
}

pub struct SubCommand {
    pub name: String,
    pub description: String,
    pub aliases: Vec<String>,
}

impl FromLua for SubCommand {
    fn from_lua(value: mlua::Value, _: &Lua) -> mlua::Result<Self> {
        match value {
            Value::Table(t) => Ok(SubCommand {
                name: t.get("name")?,
                description: if t.contains_key("description")? {
                    t.get("description")?
                } else {
                    String::new()
                },
                aliases: if t.contains_key("aliases")? {
                    t.get("aliases")?
                } else {
                    Vec::new()
                },
            }),
            _ => Err(mlua::Error::FromLuaConversionError {
                from: value.type_name(),
                to: "SubCommand".to_string(),
                message: Some("expected table".to_string()),
            }),
        }
    }
}

pub struct PacxConfig {
    pub subcommands: Vec<SubCommand>,
}

impl FromLua for PacxConfig {
    fn from_lua(value: mlua::Value, _: &Lua) -> mlua::Result<Self> {
        match value {
            Value::Table(t) => Ok(PacxConfig {
                subcommands: t.get("subcommands")?,
            }),
            _ => Err(mlua::Error::FromLuaConversionError {
                from: value.type_name(),
                to: "PacxConfig".to_string(),
                message: Some("expected table".to_string()),
            }),
        }
    }
}

pub fn load_pacx_config(lua: &Lua) -> Result<PacxConfig, Box<dyn Error>> {
    let lua_config = load(lua, "pacx")?;

    Ok(lua.convert(lua_config.config)?)
}

pub fn load(lua: &Lua, pkg_manager: &str) -> Result<Config, Box<dyn Error>> {
    let cfg_dir = match env::var_os("XDG_CONFIG_HOME").map(|x| PathBuf::from(x)) {
        Some(p) => p,
        None => match config_dir() {
            Some(p) => p,
            None => return Err("Failed to find config directory".into()),
        },
    }
    .join("pacx");

    let cfg_path = cfg_dir.join(pkg_manager.to_owned() + ".lua");
    let config_file: String = if pkg_manager == "pacx" && !cfg_path.exists() {
        let content = include_str!("./default.lua");
        fs::create_dir_all(&cfg_dir)?;
        let mut writer = BufWriter::new(fs::File::create(&cfg_path)?);

        writer.write_all(content.as_bytes())?;
        writer.flush()?;
        println!("Default configuration file created at {:?}", cfg_path);

        content.to_string()
    } else {
        match fs::read_to_string(&cfg_path) {
            Ok(c) => c,
            Err(e) => return Err(format!("Failed to open {}: {}", cfg_path.display(), e).into()),
        }
    };

    let config: Table = lua.load(&config_file).eval()?;
    let lua_runner = Config { config };

    Ok(lua_runner)
}
