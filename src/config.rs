use dirs::config_dir;
use mlua::{FromLua, Function, Lua, Table, Value};
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::PathBuf;
use std::{env, fs};

pub struct Config {
    lua_config: Table,
    manager_name: String,
}

impl Config {
    pub fn get_function(&self, name: &str) -> Result<Function, Box<dyn Error>> {
        let func = match self.lua_config.get(name) {
            Ok(f) => f,
            Err(mlua::Error::FromLuaConversionError { from, .. }) if from == "nil" => {
                return Err(format!(
                    "function \"{}\" is not defined for {}",
                    name, self.manager_name
                )
                .into());
            }
            Err(e) => return Err(e.into()),
        };

        Ok(func)
    }

    pub fn get_function_src(&self, name: &str) -> Result<String, Box<dyn Error>> {
        let func_info = self.get_function(name)?.info();
        let line_start = match func_info.line_defined {
            Some(l) => l,
            None => return Err("failed to get start line".into()),
        };
        let line_end = match func_info.last_line_defined {
            Some(l) => l,
            None => return Err("failed to get end line".into()),
        };
        let lines = BufReader::new(File::open(get_config_path(&self.manager_name)?)?).lines();
        let mut src = String::new();
        let mut line_num = 0;

        for line in lines {
            line_num += 1;

            if line_num < line_start {
                continue;
            }

            if line_num > line_end {
                break;
            }

            src.push_str(&line?);
            src.push('\n');
        }

        Ok(src)
    }
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

    Ok(lua.convert(lua_config.lua_config)?)
}

pub fn load(lua: &Lua, pkg_manager: &str) -> Result<Config, Box<dyn Error>> {
    let cfg_dir = get_config_dir()?;
    let cfg_path = get_config_path(pkg_manager)?;

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

    let lua_config: Table = lua.load(&config_file).eval()?;
    let config = Config {
        lua_config,
        manager_name: pkg_manager.to_string(),
    };

    Ok(config)
}

fn get_config_dir() -> Result<PathBuf, Box<dyn Error>> {
    let config_dir = match env::var_os("XDG_CONFIG_HOME").map(|x| PathBuf::from(x)) {
        Some(p) => p,
        None => match config_dir() {
            Some(p) => p,
            None => return Err("Failed to find config directory".into()),
        },
    }
    .join("pacx");

    Ok(config_dir)
}

fn get_config_path(pkg_manager: &str) -> Result<PathBuf, Box<dyn Error>> {
    let config_path = get_config_dir()?.join(pkg_manager.to_owned() + ".lua");

    Ok(config_path)
}
