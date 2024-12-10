use std::{error::Error, str::FromStr};

use clap::{arg, crate_name, crate_version, Command};
use mlua::{Function, Lua};

use crate::config::{load, load_pacx_config};

struct PkgInfo {
    manager: String,
    name: String,
}

impl FromStr for PkgInfo {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(':').collect();
        if parts.len() == 0 {
            return Err("Invalid package info".into());
        }
        Ok(PkgInfo {
            manager: parts[0].to_string(),
            name: if parts.len() >= 2 {
                parts[1..].join(":")
            } else {
                "".to_string()
            },
        })
    }
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let lua = Lua::new();
    let pacx_config = load_pacx_config(&lua)?;

    let mut cmd = Command::new(crate_name!()).version(crate_version!());

    for subcommand in pacx_config.subcommands {
        cmd = cmd.subcommand(
            Command::new(subcommand.name)
                .about(subcommand.description)
                .aliases(subcommand.aliases)
                .arg(arg!([package] ... "Packages to operate on")),
        );
    }

    match cmd.clone().get_matches().subcommand() {
        Some((subcmd, arg_matches)) => {
            let pkgs: Vec<PkgInfo> =
                match arg_matches.get_many::<String>("package") {
                    Some(pkg_matches) => pkg_matches
                        .map(|x| PkgInfo::from_str(x))
                        .collect::<Result<Vec<PkgInfo>, Box<dyn Error>>>()?,
                    None => Err("No packages specified")?,
                };

            for pkg in pkgs {
                let config = load(&lua, &pkg.manager)?;
                let func: Function = match config.config.get(subcmd) {
                    Ok(func) => func,
                    Err(mlua::Error::FromLuaConversionError { from, .. }) if from == "nil" => Err(
                        format!("function \"{}\" is not defined for {}", subcmd, pkg.manager),
                    )?,
                    Err(e) => Err(e)?,
                };
                let _ = func.call::<()>(pkg.name)?;
            }
        }
        None => {
            cmd.print_help()?;
        }
    }

    Ok(())
}
