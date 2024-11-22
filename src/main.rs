use std::{error::Error, str::FromStr};

use clap::{Parser, Subcommand};
use config::load::load;
use mlua::{Function, Lua};

mod config;

#[derive(Debug, Parser)]
struct Cli {
    #[clap(subcommand)]
    subcommand: SubCommands,
}

#[derive(Debug, Subcommand)]
enum SubCommands {
    /// Show information about packages
    Info { pkgs_info: Vec<PkgInfo> },
    /// Install packages
    #[clap(aliases = &["add", "i"])]
    Install { pkgs_info: Vec<PkgInfo> },
    /// List packages
    #[clap(alias = "ls")]
    List { pkg_managers: Vec<String> },
    /// Show outdated packages
    Outdated { pkg_managers: Vec<String> },
    /// Uninstall packages
    #[clap(aliases = &["delete", "remove", "rm"])]
    Uninstall { pkgs_info: Vec<PkgInfo> },
    /// Upgrade packages
    Upgrade { pkgs_info: Vec<PkgInfo> },
    /// Show why a package is installed
    Why { pkgs_info: Vec<PkgInfo> },
}

#[derive(Clone, Debug)]
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

impl From<String> for PkgInfo {
    fn from(s: String) -> Self {
        match PkgInfo::from_str(&s) {
            Ok(p) => p,
            Err(_) => PkgInfo {
                manager: s,
                name: "".to_string(),
            },
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let lua = Lua::new();
    let cli = Cli::parse();

    match cli.subcommand {
        SubCommands::Info { pkgs_info } => {
            for pkg_info in pkgs_info {
                let config = load(&lua, &pkg_info.manager)?;
                let func: Function = config.config.get("info")?;
                let _ = func.call::<()>(pkg_info.name);
            }
        }
        SubCommands::Install { pkgs_info } => {
            for pkg_info in pkgs_info {
                let config = load(&lua, &pkg_info.manager)?;
                let func: Function = config.config.get("install")?;
                let _ = func.call::<()>(pkg_info.name);
            }
        }
        SubCommands::List { pkg_managers } => {
            for pkg_manager in pkg_managers {
                let config = load(&lua, &pkg_manager)?;
                let func: Function = config.config.get("list")?;
                let _ = func.call::<()>(());
            }
        }
        SubCommands::Outdated { pkg_managers } => {
            for pkg_manager in pkg_managers {
                let config = load(&lua, &pkg_manager)?;
                let func: Function = config.config.get("outdated")?;
                let _ = func.call::<()>(());
            }
        }
        SubCommands::Uninstall { pkgs_info } => {
            for pkg_info in pkgs_info {
                let config = load(&lua, &pkg_info.manager)?;
                let func: Function = config.config.get("uninstall")?;
                let _ = func.call::<()>(pkg_info.name);
            }
        }
        SubCommands::Upgrade { pkgs_info } => {
            for pkg_info in pkgs_info {
                let config = load(&lua, &pkg_info.manager)?;
                let func: Function = config.config.get("upgrade")?;
                let _ = func.call::<()>(pkg_info.name);
            }
        }
        SubCommands::Why { pkgs_info } => {
            for pkg_info in pkgs_info {
                let config = load(&lua, &pkg_info.manager)?;
                let func: Function = config.config.get("why")?;
                let _ = func.call::<()>(pkg_info.name);
            }
        }
    }
    Ok(())
}
