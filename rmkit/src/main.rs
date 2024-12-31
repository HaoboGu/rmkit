use crate::rmk_build::build_rmk;
use anyhow::{anyhow, Result};
use chip::get_chip_options;
use chips::{get_all_chip_info, Chip};
use clap::Parser;
use futures::stream::StreamExt;
use inquire::ui::{Attributes, Color, RenderConfig, StyleSheet, Styled};
use inquire::{Select, Text};
use keyboard_toml::{parse_keyboard_toml, ProjectInfo};
use reqwest::Client;
use std::fs;
use std::fs::File;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use zip::ZipArchive;

mod args;
mod cargo_build;
mod chip;
#[allow(dead_code)]
mod config;
mod keyboard_toml;
mod rmk_build;

#[tokio::main]
async fn main() -> Result<()> {
    inquire::set_global_render_config(get_render_config());
    let args = args::Args::parse();
    match args.command {
        args::Commands::Create {
            keyboard_toml_path,
            vial_json_path,
            target_dir,
        } => create_project(keyboard_toml_path, vial_json_path, target_dir).await,
        args::Commands::Init {
            project_name,
            chip,
            split,
            local_path,
            row2col,
        } => init_project(project_name, chip, split, local_path, row2col).await,
        args::Commands::GetChip { keyboard_toml_path } => {
            let project_info = parse_keyboard_toml(&keyboard_toml_path, None)?;
            println!("{}", project_info.chip);
            Ok(())
        }
        args::Commands::GetProjectName { keyboard_toml_path } => {
            let project_info = parse_keyboard_toml(&keyboard_toml_path, None)?;
            println!("{}", project_info.project_name);
            Ok(())
        }
        args::Commands::Build {
            verbosity,
            keyboard_toml_path,
        } => build_rmk(verbosity, &keyboard_toml_path),
    }
}

async fn create_project(
    keyboard_toml_path: Option<String>,
    vial_json_path: Option<String>,
    target_dir: Option<String>,
) -> Result<()> {
    // Inquire paths interactively is no argument is specified
    let keyboard_toml_path = if keyboard_toml_path.is_none() {
        Text::new("Path to keyboard.toml:")
            .with_default("./keyboard.toml")
            .prompt()?
    } else {
        keyboard_toml_path.unwrap()
    };
    let vial_json_path = if vial_json_path.is_none() {
        Text::new("Path to vial.json")
            .with_default(&"./vial.json")
            .prompt()?
    } else {
        vial_json_path.unwrap()
    };
    // Parse keyboard.toml to get project info
    let project_info = parse_keyboard_toml(&keyboard_toml_path, target_dir)?;

    // Download corresponding project template
    download_project_template(&project_info).await?;

    // Copy keyboard.toml and vial.json to project_dir
    fs::copy(
        &keyboard_toml_path,
        project_info.target_dir.join("keyboard.toml"),
    )?;
    fs::copy(&vial_json_path, project_info.target_dir.join("vial.json"))?;

    // Post-process
    post_process(project_info)?;

    Ok(())
}

/// Postprocessing after generating project
fn post_process(project_info: ProjectInfo) -> Result<()> {
    // Replace {{ project_name }} in toml/json files
    replace_in_folder(
        &project_info,
        "toml",
        "{{ project_name }}",
        &project_info.project_name,
    )?;
    replace_in_folder(
        &project_info,
        "json",
        "{{ project_name }}",
        &project_info.project_name,
    )?;

    // Replace {{ chip_name }} in toml files
    replace_in_folder(
        &project_info,
        "toml",
        "{{ chip_name }}",
        &project_info.chip.to_string(),
    )?;

    // If the keyboard is row2col, update generated Cargo.toml
    if project_info.row2col {
        disable_rmk_default_feature(&project_info.target_dir).map_err(|e| anyhow!(e))?;
    }

    Ok(())
}

fn replace_in_folder(project_info: &ProjectInfo, ext: &str, from: &str, to: &str) -> Result<()> {
    let walker = walkdir::WalkDir::new(&project_info.target_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map_or(false, |e| e == ext));
    for entry in walker {
        let path = entry.path();
        let content = fs::read_to_string(path)?;
        let new_content = content.replace(from, to);
        fs::write(path, new_content)?;
    }
    Ok(())
}

async fn download_project_template(project_info: &ProjectInfo) -> Result<()> {
    let user = "HaoboGu";
    let repo = "rmk-template";
    let branch = "feat/rework";
    let url = format!(
        "https://github.com/{}/{}/archive/refs/heads/{}.zip",
        user, repo, branch
    );
    download_with_progress(&url, &project_info.target_dir, &project_info.remote_folder).await
}

/// Initialize project from remote url
async fn init_project(
    project_name: Option<String>,
    chip: Option<Chip>,
    split: Option<bool>,
    local_path: Option<String>,
    row2col: Option<bool>,
) -> Result<()> {
    let project_name = if project_name.is_none() {
        Text::new("Project Name:").prompt()?.replace(" ", "_")
    } else {
        project_name.unwrap().replace(" ", "_")
    };
    let split = if split.is_none() {
        Select::new("Choose your keyboard type?", vec!["normal", "split"]).prompt()? == "split"
    } else {
        split.unwrap()
    };
    let chip = if chip.is_none() {
        let mut chips = get_all_chip_info();
        if split {
            chips = chips
                .into_iter()
                .filter(|chip| chip.split_support)
                .collect();
        }
        let chips: Vec<Chip> = chips.into_iter().map(|chip| chip.chip).collect();
        Select::new("Choose your microcontroller", chips).prompt()?
    } else {
        chip.unwrap()
    };
    let row2col = row2col.unwrap_or(false);

    // Get project info from parameters
    let target_dir = PathBuf::from(&project_name);
    fs::create_dir_all(&target_dir)?;

    let remote_folder = if split {
        format!("{}_{}", chip, "split")
    } else {
        chip.to_string()
    };

    let project_info = ProjectInfo {
        project_name,
        target_dir,
        remote_folder,
        chip,
        row2col,
    };

    // Download template
    match local_path {
        Some(p) => {
            // Copy local template to project_info.target_dir
            copy_dir_recursive(Path::new(&p), &project_info.target_dir)?;
        }
        None => {
            // Use remote tempate
            download_project_template(&project_info).await?;
        }
    }

    // Post-process
    post_process(project_info)?;

    Ok(())
}

/// Download code from a GitHub repository link and extract it to the `repo` folder, using asynchronous download and a progress bar
///
/// # Parameters
/// - `download_url`: GitHub repository link
/// - `output_path`: Target extraction path
/// - `folder`: Specific subdirectory to extract
async fn download_with_progress<P>(download_url: &str, output_path: P, folder: &str) -> Result<()>
where
    P: AsRef<Path>,
{
    let output_path = output_path.as_ref();

    // Ensure the output path is clean
    if output_path.exists() {
        fs::remove_dir_all(output_path)?;
    }
    fs::create_dir_all(output_path)?;

    println!("⇣ Download project template for {}...", folder);

    // Send request and get response
    let client = Client::new();
    let response = client.get(download_url).send().await?;
    if !response.status().is_success() {
        return Err(anyhow!("Download failed: {}", response.status()).into());
    }

    // Temporary file to store the downloaded content
    let temp_file_path = output_path.join("temp.zip");
    let mut temp_file = File::create(&temp_file_path)?;

    // Ensure the temporary file is cleaned up on error
    struct TempFileCleanup<'a> {
        path: &'a Path,
    }
    impl<'a> Drop for TempFileCleanup<'a> {
        fn drop(&mut self) {
            if self.path.exists() {
                if let Err(e) = fs::remove_file(self.path) {
                    eprintln!(
                        "Failed to remove temp file '{}': {}",
                        self.path.display(),
                        e
                    );
                }
            }
        }
    }
    let _cleanup_guard = TempFileCleanup {
        path: &temp_file_path,
    };

    // Stream response bytes and write to temp file
    let mut stream = response.bytes_stream();
    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        temp_file.write_all(&chunk)?;
    }

    // Open the downloaded ZIP file and extract
    let zip_file = File::open(&temp_file_path)?;
    let mut zip = ZipArchive::new(zip_file)?;

    let mut folder_found = false;
    for i in 0..zip.len() {
        let mut file = zip.by_index(i)?;
        let file_name = file.enclosed_name().ok_or(anyhow!("Invalid file path"))?;

        // Find the root directory from the ZIP file
        let segments: Vec<_> = file_name.iter().collect();
        if segments.len() > 1 && segments[1] == folder {
            folder_found = true;
            let relative_name = file_name.iter().skip(2).collect::<PathBuf>();
            let out_path = output_path.join(relative_name);

            if file.is_dir() {
                fs::create_dir_all(&out_path)?;
            } else {
                if let Some(parent) = out_path.parent() {
                    fs::create_dir_all(parent)?;
                }
                let mut outfile = File::create(&out_path)?;
                io::copy(&mut file, &mut outfile)?;
            }
        }
    }

    if !folder_found {
        // Check whether the remote_folder starts with stm32, do the second search using `stm32xx` and if there's still no matched template, use `stm32` template
        if folder.starts_with("stm32") {
            // Generate template for stm32
            if folder.len() > 7 {
                // Do the second search, use the stm32's family name
                let stm32_series = &folder[..7];
                for i in 0..zip.len() {
                    let mut file = zip.by_index(i)?;
                    let file_name = file.enclosed_name().ok_or(anyhow!("Invalid file path"))?;

                    // Find the root directory from the ZIP file
                    let segments: Vec<_> = file_name.iter().collect();
                    if segments.len() > 1 && segments[1] == stm32_series {
                        folder_found = true;
                        let relative_name = file_name.iter().skip(2).collect::<PathBuf>();
                        let out_path = output_path.join(relative_name);

                        if file.is_dir() {
                            fs::create_dir_all(&out_path)?;
                        } else {
                            if let Some(parent) = out_path.parent() {
                                fs::create_dir_all(parent)?;
                            }
                            let mut outfile = File::create(&out_path)?;
                            io::copy(&mut file, &mut outfile)?;
                        }
                    }
                }
            }
            if !folder_found {
                println!("️️🚨 There's no template available for [{folder}], using the default stm32 template. You may need to make further edit.");
                // Still not found, use the default stm32 template
                for i in 0..zip.len() {
                    let mut file = zip.by_index(i)?;
                    let file_name = file.enclosed_name().ok_or(anyhow!("Invalid file path"))?;

                    // Find the root directory from the ZIP file
                    let segments: Vec<_> = file_name.iter().collect();
                    if segments.len() > 1 && segments[1] == "stm32" {
                        folder_found = true;
                        let relative_name = file_name.iter().skip(2).collect::<PathBuf>();
                        let out_path = output_path.join(relative_name);

                        if file.is_dir() {
                            fs::create_dir_all(&out_path)?;
                        } else {
                            if let Some(parent) = out_path.parent() {
                                fs::create_dir_all(parent)?;
                            }
                            let mut outfile = File::create(&out_path)?;
                            io::copy(&mut file, &mut outfile)?;
                        }
                    }
                }
            }
        }

        // Check again
        if !folder_found {
            return Err(anyhow!(
                "The specified chip/board '{}' does not exist in the template repo",
                folder
            ));
        }
    }

    println!("✅ Project created, path: {}", output_path.display());
    Ok(())
}

fn get_render_config() -> RenderConfig<'static> {
    let mut render_config = RenderConfig::default();
    render_config.prompt_prefix = Styled::new("?").with_fg(Color::LightRed);

    render_config.error_message = render_config
        .error_message
        .with_prefix(Styled::new("❌").with_fg(Color::LightRed));

    render_config.answer = StyleSheet::new()
        .with_attr(Attributes::ITALIC)
        .with_fg(Color::LightGreen);

    render_config
}

fn copy_dir_recursive(src: &Path, dest: &Path) -> io::Result<()> {
    if !src.is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Source is not a directory",
        ));
    }

    // Create the target folder
    fs::create_dir_all(dest)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        let src_path = entry.path();
        let dest_path = dest.join(entry.file_name());

        if file_type.is_dir() {
            // Recursively process
            copy_dir_recursive(&src_path, &dest_path)?;
        } else {
            // Copy file
            fs::copy(&src_path, &dest_path)?;
        }
    }
    Ok(())
}

/// 更新指定路径下 Cargo.toml 文件中的 rmk 依赖配置
/// 将 rmk = { version = "...", features = ["..."] } 替换为
/// rmk = { version = "...", default-features = false, features = ["..."] }
///
/// # Arguments
/// * `target_dir` - 包含 Cargo.toml 的目标目录路径
///
/// # Returns
/// * `Result<(), String>` - 成功返回 Ok，失败返回 Err
fn disable_rmk_default_feature(target_dir: &PathBuf) -> std::result::Result<(), String> {
    // 定义 Cargo.toml 的路径
    let cargo_toml_path = Path::new(target_dir).join("Cargo.toml");

    // 使用 cargo_toml 解析为 Manifest
    let mut manifest =
        cargo_toml::Manifest::from_path(&cargo_toml_path).map_err(|e| e.to_string())?;

    // 获取 dependencies 并修改 rmk 配置
    if let Some(cargo_toml::Dependency::Detailed(rmk_dep)) = manifest.dependencies.get_mut("rmk") {
        // 设置 default-features = false，并保留原始 version 和 features
        rmk_dep.default_features = false
    } else {
        return Err("No valid rmk dependency found".to_string());
    }

    // 将修改后的 Manifest 转换为字符串
    let updated_toml = toml::to_string(&manifest)
        .map_err(|e| format!("Failed to serialize updated Cargo.toml: {}", e))?;

    // 将更新后的内容写回文件
    fs::write(&cargo_toml_path, updated_toml)
        .map_err(|e| format!("Failed to write updated Cargo.toml: {}", e))?;

    Ok(())
}
