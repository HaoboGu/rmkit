use anyhow::{bail, Result};
use cargo_metadata::{Artifact, Message, Metadata, MetadataCommand, TargetKind};
use std::{
    env,
    io::BufReader,
    process::{Command, Stdio},
};

fn get_metadata() -> Result<Metadata> {
    let metadata_command = MetadataCommand::new();

    let metadata = metadata_command.exec()?;
    if metadata.workspace_members.is_empty() {
        bail!("Unable to find workspace members");
    }

    Ok(metadata)
}

pub fn cargo_build(bin: Option<&str>, verbosity: u64) -> Result<Option<Artifact>> {
    let metadata = get_metadata()?;
    let cargo = env::var_os("CARGO").unwrap_or_else(|| "cargo".into());
    let mut cargo = Command::new(cargo);
    cargo.arg("build");
    cargo.arg("--release");
    if let Some(bin) = bin {
        cargo.arg("--bin");
        cargo.arg(bin);
    }

    cargo.arg("--message-format=json");
    cargo.stdout(Stdio::piped());

    if verbosity > 1 {
        cargo.arg(format!("-{}", "v".repeat((verbosity - 1) as usize)));
        eprintln!("{cargo:?}");
    }

    let mut child = cargo.spawn()?;
    let stdout = BufReader::new(child.stdout.take().expect("Pipe to cargo process failed"));

    // Note: We call `collect` to ensure we don't block stdout which could prevent the process from exiting
    let messages = Message::parse_stream(stdout).collect::<Vec<_>>();

    let status = child.wait()?;
    if !status.success() {
        bail!("Failed to parse crate metadata");
    }

    let mut target_artifact: Option<Artifact> = None;
    for message in messages {
        match message? {
            Message::CompilerArtifact(artifact) => {
                if (metadata.workspace_members.contains(&artifact.package_id)
                    && artifact.target.name == bin.unwrap_or("no binary")
                    && artifact.executable.is_some())
                    || artifact.target.kind.iter().any(|s| *s == TargetKind::Bin)
                {
                    if target_artifact.is_some() {
                        bail!("Can only have one matching artifact but found several");
                    }

                    target_artifact = Some(artifact);
                }
            }
            Message::CompilerMessage(msg) => {
                if verbosity > 1 {
                    if let Some(rendered) = msg.message.rendered {
                        print!("{rendered}");
                    }
                }
            }
            _ => (),
        }
    }

    if target_artifact.is_none() {
        bail!("Could not determine the wanted artifact");
    }

    Ok(target_artifact)
}
