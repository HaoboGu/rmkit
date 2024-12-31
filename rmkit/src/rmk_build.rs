use crate::{
    cargo_build::cargo_build, config::KeyboardTomlConfig, keyboard_toml::read_keyboard_toml_config,
};
use anyhow::Result;

pub fn build_rmk(verbosity: u64, keyboard_toml_path: &str) -> Result<()> {
    let keyboard_toml = read_keyboard_toml_config(&keyboard_toml_path)?;

    if keyboard_toml.split.is_some() {
        internal_build_rmk(verbosity, &keyboard_toml, Some("central"))?;
        internal_build_rmk(verbosity, &keyboard_toml, Some("peripheral"))?;
    } else {
        internal_build_rmk(verbosity, &keyboard_toml, None)?;
    }

    Ok(())
}

fn internal_build_rmk(
    verbosity: u64,
    keyboard_toml: &KeyboardTomlConfig,
    binary: Option<&str>,
) -> Result<()> {
    cargo_build(binary, verbosity)?;
    Ok(())
}
