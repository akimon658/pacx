use std::{error::Error, str::FromStr};

use clap::{arg, crate_name, crate_version, Arg, ArgAction, Command};
use mlua::Lua;

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
                String::new()
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
                .arg(
                    Arg::new("dry-run")
                        .long("dry-run")
                        .short('n')
                        .help("Print the source of the function without executing")
                        .action(ArgAction::SetTrue),
                )
                .arg(arg!([package] ... "Packages to operate on"))
                .arg(arg!([flag] ... "Flags to pass to the package manager").last(true)),
        );
    }

    match cmd.clone().get_matches().subcommand() {
        Some((subcmd, arg_matches)) => {
            let dry_run = arg_matches.get_flag("dry-run");

            let pkgs: Vec<PkgInfo> =
                match arg_matches.get_many::<String>("package") {
                    Some(pkg_matches) => pkg_matches
                        .map(|x| PkgInfo::from_str(x))
                        .collect::<Result<Vec<PkgInfo>, Box<dyn Error>>>()?,
                    None => Err("No packages specified")?,
                };

            let flags = match arg_matches.get_many::<String>("flag") {
                Some(flag_matches) => flag_matches.cloned().collect::<Vec<String>>().join(" "),
                None => String::new(),
            };

            for pkg in pkgs {
                let config = load(&lua, &pkg.manager)?;

                if dry_run {
                    print!("{}", config.get_function_src(subcmd)?);
                } else {
                    let func = config.get_function(subcmd)?;

                    func.call::<()>((pkg.name, flags.clone()))?;
                }
            }
        }
        None => {
            cmd.print_help()?;
        }
    }

    Ok(())
}
