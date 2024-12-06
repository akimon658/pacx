use std::error::Error;

use assert_cmd::Command;
use assert_fs::{
    prelude::{FileWriteStr, PathChild},
    TempDir,
};

#[test]
fn subcommand_with_args() -> Result<(), Box<dyn Error>> {
    let temp_dir = TempDir::new()?;
    let config_dir = temp_dir.child("pacx");

    let pacx_config = config_dir.child("pacx.lua");
    pacx_config.write_str(include_str!("pacx.lua"))?;

    let testpkg_config = config_dir.child("test_manager.lua");
    testpkg_config.write_str(include_str!("test_manager.lua"))?;

    let _ = Command::cargo_bin("pacx")?
        .env(
            "XDG_CONFIG_HOME",
            match temp_dir.path().to_str() {
                Some(p) => p,
                None => return Err("Failed to convert path to string".into()),
            },
        )
        .args(&["subcmd_test", "test_manager:testpkg"])
        .assert()
        .stdout("testpkg\n");

    Ok(())
}
