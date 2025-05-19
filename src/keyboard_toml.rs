use rmk_config::KeyboardTomlConfig;
use std::{
    env, fs,
    path::{Path, PathBuf},
    process,
};

use crate::chip::get_board_chip_map;

/// All info needed to create a RMK project
#[derive(Debug)]
pub(crate) struct ProjectInfo {
    /// Project name
    pub(crate) project_name: String,
    /// Local directory of created RMK project
    pub(crate) target_dir: PathBuf,
    /// Remote folder name which contains the template
    pub(crate) remote_folder: String,
    /// Chip name
    pub(crate) chip: String,
    /// Key for uf2 generation
    pub(crate) uf2_key: String,
    /// Whether the project is row2col, row2col needs special post-process
    pub(crate) row2col: bool,
}

/// Parse `keyboard.toml`, get all needed project info for creating a new RMK project
pub(crate) fn parse_keyboard_toml(
    keyboard_toml: &String,
    target_dir: Option<String>,
) -> Result<ProjectInfo, Box<dyn std::error::Error>> {
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
        keyboard_toml_config.keyboard.board.as_deref(),
        keyboard_toml_config.keyboard.chip.as_deref(),
    ) {
        (None, None) => {
            Err("Either 'board' or 'chip' must be specified in keyboard.toml".to_string())
        }
        (Some(board), None) => {
            let map = get_board_chip_map();
            map.get(board.to_lowercase().as_str())
                .map(|chip| chip.to_string())
                .ok_or_else(|| format!("Unsupported board '{}'", board))
        }
        (None, Some(chip)) => Ok(chip.to_string().to_lowercase()),
        (Some(_), Some(_)) => {
            Err("'board' and 'chip' cannot both be specified in keyboard.toml".to_string())
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
        (None, None) => {
            Err("Either 'matrix' or 'split' section must be specified in keyboard.toml".to_string())
        }
        (None, Some(_)) => Ok("split".to_string()),
        (Some(_), None) => Ok("normal".to_string()),
        (Some(_), Some(_)) => {
            Err("'matrix' and 'split' cannot both be specified in keyboard.toml".to_string())
        }
    }?;

    let folder = if matrix_type == "split" {
        format!("{}_{}", chip, matrix_type)
    } else {
        chip.clone()
    };

    let uf2_key = if chip.starts_with("stm32") {
        chip[..7].to_string()
    } else {
        chip.clone()
    };

    Ok(ProjectInfo {
        project_name,
        target_dir: project_dir,
        remote_folder: folder,
        chip,
        uf2_key,
        row2col,
    })
}

/// Read the `keyboard.toml` configuration file
pub(crate) fn read_keyboard_toml_config<P: AsRef<Path>>(
    keyboard_toml: P,
) -> Result<KeyboardTomlConfig, String> {
    // Read the keyboard configuration file in the project root
    let s = match fs::read_to_string(keyboard_toml) {
        Ok(s) => s,
        Err(e) => {
            let msg = format!("Failed to read `keyboard.toml` configuration file: {}", e);
            return Err(msg);
        }
    };

    // Parse the configuration file content into a `KeyboardTomlConfig` struct
    match toml::from_str(&s) {
        Ok(c) => Ok(c),
        Err(e) => {
            let msg = format!("Failed to parse `keyboard.toml`: {}", e.message());
            return Err(msg);
        }
    }
}
