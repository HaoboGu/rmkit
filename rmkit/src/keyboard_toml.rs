use anyhow::{anyhow, Result};
use chips::{get_chip, Chip};
use std::{
    env, fs,
    path::{Path, PathBuf},
    process,
};

use crate::{chip::get_board_chip_map, config::KeyboardTomlConfig};

/// All info needed to create a RMK project
#[derive(Debug)]
pub(crate) struct ProjectInfo {
    /// Project name
    pub(crate) project_name: String,
    /// Local directory of created RMK project
    pub(crate) target_dir: PathBuf,
    /// Remote folder name which contains the template
    pub(crate) remote_folder: String,
    /// Chip
    pub(crate) chip: Chip,
    /// Whether the project is row2col, row2col needs special post-process
    pub(crate) row2col: bool,
}

/// Parse `keyboard.toml`, get all needed project info for creating a new RMK project
pub(crate) fn parse_keyboard_toml(
    keyboard_toml: &String,
    target_dir: Option<String>,
) -> Result<ProjectInfo> {
    let keyboard_toml_config = read_keyboard_toml_config(keyboard_toml)?;

    let project_name = keyboard_toml_config.keyboard.name.replace(" ", "_");
    let target_dir = if target_dir.is_none() {
        project_name.clone()
    } else {
        target_dir.unwrap()
    };
    let project_dir = env::current_dir()?.join(&target_dir);

    if let Err(e) = fs::create_dir_all(&project_dir) {
        eprintln!("Failed to create project directory {}: {}", project_name, e);
        process::exit(1);
    }

    // Check keyboard.toml
    let chip = match (
        keyboard_toml_config.keyboard.board.as_ref(),
        keyboard_toml_config.keyboard.chip.as_ref(),
    ) {
        (None, None) => Err(anyhow!(
            "Either 'board' or 'chip' must be specified in keyboard.toml"
        )),
        (Some(board), None) => Ok(get_chip(&board)),
        (None, Some(chip)) => Ok(chip.clone()),
        (Some(board), Some(chip)) => {
            let board_chip = get_chip(&board);
            if chip == &board_chip {
                Ok(chip.clone())
            } else {
                Err(anyhow!(
                    "The board '{board:?} usually uses the chip '{board_chip:?}', but you specified the chip '{chip:?}'. Consider removing the board config from keyboard.toml."
                ))
            }
        }
    }?;

    let row2col = if let Some(m) = keyboard_toml_config.clone().matrix {
        m.row2col
    } else {
        if let Some(s) = keyboard_toml_config.clone().split {
            s.central.matrix.row2col
        } else {
            false
        }
    };

    let matrix_type = match (keyboard_toml_config.matrix, keyboard_toml_config.split) {
        (None, None) => Err(anyhow!(
            "Either 'matrix' or 'split' section must be specified in keyboard.toml"
        )),
        (None, Some(_)) => Ok("split".to_string()),
        (Some(_), None) => Ok("normal".to_string()),
        (Some(_), Some(_)) => Err(anyhow!(
            "'matrix' and 'split' cannot both be specified in keyboard.toml"
        )),
    }?;

    let chip_name = chip.to_string();

    let folder = if matrix_type == "split" {
        format!("{}_{}", &chip_name, matrix_type)
    } else {
        chip_name.clone()
    };

    Ok(ProjectInfo {
        project_name,
        target_dir: project_dir,
        remote_folder: folder,
        chip,
        row2col,
    })
}

/// Read the `keyboard.toml` configuration file
pub(crate) fn read_keyboard_toml_config<P: AsRef<Path>>(
    keyboard_toml: P,
) -> Result<KeyboardTomlConfig> {
    // Read the keyboard configuration file in the project root
    let s = match fs::read_to_string(keyboard_toml) {
        Ok(s) => s,
        Err(e) => {
            let msg = anyhow!("Failed to read `keyboard.toml` configuration file: {}", e);
            return Err(msg);
        }
    };

    // Parse the configuration file content into a `KeyboardTomlConfig` struct
    match toml::from_str(&s) {
        Ok(c) => Ok(c),
        Err(e) => {
            let msg = anyhow!("Failed to parse `keyboard.toml`: {}", e.message());
            return Err(msg);
        }
    }
}
