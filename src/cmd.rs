use std::{error::Error, str::FromStr};

use clap::{arg, crate_name, crate_version, Arg, ArgAction, Command};
use mlua::Lua;

use crate::config::{load, load_pacx_config};

struct PkgInfo {
    manager: String,
    name: String,
    pub version: Option<String>,
}

impl FromStr for PkgInfo {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // 1. Split the input string `s` by the first colon (`:`)
        let (manager_str, pkg_full_string) =
            s.split_once(':')
                .ok_or("Invalid package string: missing manager separator ':'")?;

        // 2. Initialize `package_name` as `pkg_full_string` and `version` as `None`.
        let mut package_name = pkg_full_string;
        let mut version: Option<String> = None;

        // 3. Scan `pkg_full_string` from right to left to find the first unescaped at-sign (`@`).
        let mut found_at_idx: Option<usize> = None;
        if !pkg_full_string.is_empty() { // Optimization: only scan if there's something to scan
            for (idx, char_candidate) in pkg_full_string.char_indices().rev() {
                if char_candidate == '@' {
                    if idx > 0 {
                        // Check character before it
                        if pkg_full_string.chars().nth(idx - 1) != Some('\\') {
                            found_at_idx = Some(idx);
                            break;
                        }
                        // else, it's escaped, continue searching
                    } else {
                        // '@' is at the beginning of pkg_full_string
                        found_at_idx = Some(idx);
                        break;
                    }
                }
            }
        }

        if let Some(idx) = found_at_idx {
            // 4. If an unescaped `@` is found at `idx`
            if idx == 0 {
                // Special Rule for Leading `@`: part of the package name
                // package_name remains pkg_full_string, version remains None
            } else {
                // Otherwise (if `idx > 0`):
                let (name_part, version_part) = pkg_full_string.split_at(idx);
                package_name = name_part;
                version = Some(version_part[1..].to_string()); // Skip the '@' itself
            }
        }
        // 5. If no unescaped `@` is found: package_name and version remain as initialized.

        // 6. Replace all occurrences of `\@` with `@` in `package_name`.
        let final_package_name = package_name.replace("\\@", "@");

        // 7. Construct and return `Ok(PkgInfo { ... })`.
        Ok(PkgInfo {
            manager: manager_str.to_string(),
            name: final_package_name,
            version,
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
                    None => return Err("No packages specified".into()),
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

                    func.call::<()>((pkg.name.clone(), flags.clone(), pkg.version.clone()))?;
                }
            }
        }
        None => {
            cmd.print_help()?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::PkgInfo;
    use std::str::FromStr;

    #[test]
    fn test_pkginfo_from_str() {
        // 1. No version
        let input1 = "apt:nginx";
        let pkg1 = PkgInfo::from_str(input1).unwrap();
        assert_eq!(pkg1.manager, "apt");
        assert_eq!(pkg1.name, "nginx");
        assert_eq!(pkg1.version, None);

        // 2. With version
        let input2 = "python:requests@==2.28.1";
        let pkg2 = PkgInfo::from_str(input2).unwrap();
        assert_eq!(pkg2.manager, "python");
        assert_eq!(pkg2.name, "requests");
        assert_eq!(pkg2.version, Some("==2.28.1".to_string()));

        // 3. Scoped package, no version
        let input3 = "npm:@scope/my-package";
        let pkg3 = PkgInfo::from_str(input3).unwrap();
        assert_eq!(pkg3.manager, "npm");
        assert_eq!(pkg3.name, "@scope/my-package");
        assert_eq!(pkg3.version, None);

        // 4. Scoped package, with version
        let input4 = "npm:@scope/my-package@1.0.0";
        let pkg4 = PkgInfo::from_str(input4).unwrap();
        assert_eq!(pkg4.manager, "npm");
        assert_eq!(pkg4.name, "@scope/my-package");
        assert_eq!(pkg4.version, Some("1.0.0".to_string()));

        // 5. Package name with escaped `@`, with version
        let input5 = "npm:my-package\\@with-at@1.0.0";
        let pkg5 = PkgInfo::from_str(input5).unwrap();
        assert_eq!(pkg5.manager, "npm");
        assert_eq!(pkg5.name, "my-package@with-at");
        assert_eq!(pkg5.version, Some("1.0.0".to_string()));

        // 6. Package name with escaped `@`, no version
        let input6 = "npm:my-package\\@with-at";
        let pkg6 = PkgInfo::from_str(input6).unwrap();
        assert_eq!(pkg6.manager, "npm");
        assert_eq!(pkg6.name, "my-package@with-at");
        assert_eq!(pkg6.version, None);

        // 7. Invalid input (no colon)
        let input7 = "invalidinput";
        let result7 = PkgInfo::from_str(input7);
        assert!(result7.is_err());

        // 8. Empty package name (but valid manager)
        let input8 = "apt:";
        let pkg8 = PkgInfo::from_str(input8).unwrap();
        assert_eq!(pkg8.manager, "apt");
        assert_eq!(pkg8.name, "");
        assert_eq!(pkg8.version, None);

        // 9. Empty version string
        let input9 = "apt:nginx@";
        let pkg9 = PkgInfo::from_str(input9).unwrap();
        assert_eq!(pkg9.manager, "apt");
        assert_eq!(pkg9.name, "nginx");
        assert_eq!(pkg9.version, Some("".to_string()));
        
        // 10. Version string with escaped @
        let input10 = "manager:package@version\\@1";
        let pkg10 = PkgInfo::from_str(input10).unwrap();
        assert_eq!(pkg10.manager, "manager");
        assert_eq!(pkg10.name, "package");
        assert_eq!(pkg10.version, Some("version\\@1".to_string()));

        // 11. Package name with multiple escaped @ and a version
        let input11 = "npm:my\\@pkg\\@name@1.2.3";
        let pkg11 = PkgInfo::from_str(input11).unwrap();
        assert_eq!(pkg11.manager, "npm");
        assert_eq!(pkg11.name, "my@pkg@name");
        assert_eq!(pkg11.version, Some("1.2.3".to_string()));

        // 12. Package name ending with escaped @, no version
        let input12 = "npm:some-package\\@";
        let pkg12 = PkgInfo::from_str(input12).unwrap();
        assert_eq!(pkg12.manager, "npm");
        assert_eq!(pkg12.name, "some-package@");
        assert_eq!(pkg12.version, None);
    }
}
