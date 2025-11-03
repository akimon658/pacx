use std::{
    env::{self, temp_dir},
    error::Error,
    fs::{create_dir_all, File},
    io::{BufWriter, Write},
    path::PathBuf,
    time::SystemTime,
};

use assert_cmd::cargo::cargo_bin_cmd;

fn setup() -> Result<(), Box<dyn Error>> {
    let timestamp = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(d) => d.as_secs(),
        Err(e) => return Err(e.into()),
    };

    let xdg_dir_buf = temp_dir().join(format!("pacx-test-{}", timestamp));
    let xdg_dir_str = match xdg_dir_buf.to_str() {
        Some(p) => p,
        None => return Err("Could not find temp dir".into()),
    };

    let config_dir: PathBuf = [xdg_dir_str, "pacx"].iter().collect();
    create_dir_all(&config_dir)?;

    let mut pacx_cfg_writer = BufWriter::new(File::create(config_dir.join("pacx.lua"))?);
    pacx_cfg_writer.write_all(include_bytes!("pacx.lua"))?;
    pacx_cfg_writer.flush()?;

    let mut test_manager_cfg_writer =
        BufWriter::new(File::create(config_dir.join("test_manager.lua"))?);
    test_manager_cfg_writer.write_all(include_bytes!("test_manager.lua"))?;
    test_manager_cfg_writer.flush()?;

    env::set_var("XDG_CONFIG_HOME", xdg_dir_str);

    Ok(())
}

#[test]
fn dry_run() -> Result<(), Box<dyn Error>> {
    setup()?;

    cargo_bin_cmd!("pacx")
        .args(&["subcmd_test", "test_manager", "--dry-run"])
        .assert()
        .stdout(
            r#"local function subcmd_test(pkg)
  print(pkg)
end
"#,
        );

    Ok(())
}

#[test]
fn subcommand_with_args() -> Result<(), Box<dyn Error>> {
    setup()?;

    cargo_bin_cmd!("pacx")
        .args(&["subcmd_test", "test_manager:testpkg"])
        .assert()
        .stdout("testpkg\n");

    Ok(())
}

#[test]
fn subcommand_with_arg_and_flags() -> Result<(), Box<dyn Error>> {
    setup()?;

    cargo_bin_cmd!("pacx")
        .args(&[
            "subcmd_test_with_arg_and_flags",
            "test_manager:testpkg",
            "--",
            "--testflag",
        ])
        .assert()
        .stdout("testpkg --testflag\n");

    Ok(())
}

#[test]
fn subcommand_with_flags() -> Result<(), Box<dyn Error>> {
    setup()?;

    cargo_bin_cmd!("pacx")
        .args(&["subcmd_test_with_flags", "test_manager", "--", "--testflag"])
        .assert()
        .stdout("--testflag\n");

    Ok(())
}
