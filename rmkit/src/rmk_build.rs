use std::path::Path;

use crate::{
    cargo_build::cargo_build, cargo_objcopy::cargo_objcopy,
    keyboard_toml::read_keyboard_toml_config,
};
use anyhow::Result;
use chips::{ChipInfo, FirmwareFormat};

pub fn build_rmk(verbosity: u64, keyboard_toml_path: &str) -> Result<()> {
    let keyboard_toml = read_keyboard_toml_config(&keyboard_toml_path)?;
    let chip_info = keyboard_toml.keyboard.get_chip_info()?;

    let name = keyboard_toml.keyboard.name;

    if keyboard_toml.split.is_some() {
        internal_build_rmk(verbosity, &chip_info, &name, Some("central"))?;
        internal_build_rmk(verbosity, &chip_info, &name, Some("peripheral"))?;
    } else {
        internal_build_rmk(verbosity, &chip_info, &name, None)?;
    }

    Ok(())
}

fn internal_build_rmk(
    verbosity: u64,
    chip_info: &ChipInfo,
    name: &str,
    binary: Option<&str>,
) -> Result<()> {
    let artifact = cargo_build(binary, verbosity)?.expect("Cargo build failed");

    let file = match &artifact.executable {
        // Example and bins have an executable
        Some(val) => val,
        // Libs have an rlib and an rmeta. We want the rlib, which always
        // comes first in the filenames array after some quick testing.
        //
        // We could instead look for files ending in .rlib, but that would
        // fail for cdylib and other fancy crate kinds.
        None => &artifact.filenames[0],
    };

    let name = match binary {
        Some(binary) => format!("{name}_{binary}"),
        None => name.to_string(),
    };

    let firmware_file = cargo_objcopy(file, &name, verbosity, FirmwareFormat::Hex)?;

    let uf2_output = format!("{name}.uf2");

    hex_to_uf2::hex_to_uf2_file(
        Path::new(&firmware_file),
        Path::new(&uf2_output),
        &Some(chip_info.clone()),
    )?;

    Ok(())
}
