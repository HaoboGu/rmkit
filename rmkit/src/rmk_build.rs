use std::path::Path;

use crate::{
    cargo_build::cargo_build, cargo_objcopy::cargo_objcopy, config::KeyboardTomlConfig,
    keyboard_toml::read_keyboard_toml_config,
};
use anyhow::Result;
use chips::{ChipInfo, FirmwareFormat};

pub fn build_rmk(verbosity: u64, keyboard_toml_path: &str) -> Result<()> {
    let keyboard_toml = read_keyboard_toml_config(keyboard_toml_path)?;
    let chip_info = keyboard_toml.keyboard.get_chip_info()?;

    if keyboard_toml.split.is_some() {
        internal_build_rmk(verbosity, &chip_info, &keyboard_toml, Some("central"))?;
        internal_build_rmk(verbosity, &chip_info, &keyboard_toml, Some("peripheral"))?;
    } else {
        internal_build_rmk(verbosity, &chip_info, &keyboard_toml, None)?;
    }

    Ok(())
}

fn internal_build_rmk(
    verbosity: u64,
    chip_info: &ChipInfo,
    keyboard_toml: &KeyboardTomlConfig,
    binary: Option<&str>,
) -> Result<()> {
    let name = &keyboard_toml.keyboard.name;
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

    let objcopy_format = match keyboard_toml.keyboard.firmware_format {
        FirmwareFormat::Bin => Some(FirmwareFormat::Bin),
        FirmwareFormat::Elf => None,
        FirmwareFormat::Hex | FirmwareFormat::Uf2 => Some(FirmwareFormat::Hex),
    };

    let Some(objcopy_format) = objcopy_format else {
        return Ok(());
    };

    let firmware_file = cargo_objcopy(file, &name, verbosity, objcopy_format)?;

    if !(keyboard_toml.keyboard.firmware_format == FirmwareFormat::Uf2) {
        return Ok(());
    }

    let uf2_output = format!("{name}.uf2");

    hex_to_uf2::hex_to_uf2_file(
        Path::new(&firmware_file),
        Path::new(&uf2_output),
        &Some(chip_info.clone()),
    )?;

    Ok(())
}
