use rmk_config::KeyboardTomlConfig;
use std::{env, fs, path::PathBuf, process};

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
    /// List of disabled default features
    pub(crate) disabled_default_feature: Vec<String>,
}

/// Parse `keyboard.toml`, get all needed project info for creating a new RMK project
pub(crate) fn parse_keyboard_toml(
    keyboard_toml: &String,
    target_dir: Option<String>,
) -> Result<ProjectInfo, Box<dyn std::error::Error>> {
    let keyboard_toml_config = KeyboardTomlConfig::new_from_toml_str(keyboard_toml);

    let project_name = keyboard_toml_config
        .keyboard
        .clone()
        .expect("[keyboard] section is required")
        .name
        .replace(" ", "_");
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

    let mut default_feature_config = vec![];

    // Check keyboard.toml
    let row2col = if let Some(m) = keyboard_toml_config.clone().matrix {
        m.row2col
    } else {
        if let Some(s) = keyboard_toml_config.clone().split {
            s.central.matrix.row2col
        } else {
            false
        }
    };
    if row2col {
        default_feature_config.push("col2row".to_string());
    }

    // Storage config
    let storage_config = keyboard_toml_config.get_storage_config();
    if !storage_config.enabled {
        default_feature_config.push("storage".to_string());
    }

    // Defmt config
    let dep_config = keyboard_toml_config.get_dependency_config();
    if !dep_config.defmt_log {
        default_feature_config.push("defmt".to_string());
    }

    let board_config = keyboard_toml_config.get_board_config().unwrap();
    let matrix_type = match board_config {
        rmk_config::BoardConfig::Split(_) => "split".to_string(),
        rmk_config::BoardConfig::UniBody(_) => "normal".to_string(),
    };

    let chip_model = keyboard_toml_config.get_chip_model().unwrap();
    let chip_or_board = if let Some(board) = chip_model.board {
        board
    } else {
        chip_model.chip.clone()
    };
    let folder = if matrix_type == "split" {
        format!("{}_{}", chip_or_board, matrix_type)
    } else {
        chip_or_board.clone()
    };

    let uf2_key = if chip_model.chip.starts_with("stm32") {
        chip_model.chip[..7].to_string()
    } else {
        chip_model.chip.clone()
    };

    Ok(ProjectInfo {
        project_name,
        target_dir: project_dir,
        remote_folder: folder,
        chip: chip_or_board,
        uf2_key,
        disabled_default_feature: default_feature_config,
    })
}
