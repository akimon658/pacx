use std::{error::Error, str::FromStr};

use clap::{crate_name, Command};
use config::load::{load, load_pacx_config};
use mlua::{Function, Lua};

mod config;

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

fn main() -> Result<(), Box<dyn Error>> {
    let lua = Lua::new();

    let mut cmd = Command::new(crate_name!());
    let pacx_config = load_pacx_config(&lua)?;

    for subcommand in pacx_config.subcommands {
        cmd = cmd.subcommand(
            Command::new(subcommand.name)
                .about(subcommand.description)
                .aliases(subcommand.aliases),
        );
    }

    match cmd.clone().get_matches().subcommand() {
        Some((subcmd, arg_matches)) => {
            let pkgs: Vec<PkgInfo> =
                match arg_matches.get_many::<String>("") {
                    Some(pkg_matches) => pkg_matches
                        .map(|x| PkgInfo::from_str(x))
                        .collect::<Result<Vec<PkgInfo>, Box<dyn Error>>>()?,
                    None => Err("No packages specified")?,
                };

            for pkg in pkgs {
                let config = load(&lua, &pkg.manager)?;
                let func: Function = config.config.get(subcmd)?;
                let _ = func.call::<()>(pkg.name)?;
            }
        }
        None => {
            cmd.print_help()?;
        }
    }

    Ok(())
}
