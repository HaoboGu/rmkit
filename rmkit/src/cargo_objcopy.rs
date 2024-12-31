use std::process::Command;

use anyhow::{anyhow, bail, Result};
use cargo_metadata::camino::Utf8PathBuf;
use chips::FirmwareFormat;

pub fn cargo_objcopy(
    file: &Utf8PathBuf,
    out_name: &str,
    verbosity: u64,
    format: FirmwareFormat,
) -> Result<String> {
    let (format, ending) = match format {
        FirmwareFormat::Bin => Ok(("ihex", "hex")),
        FirmwareFormat::Hex => Ok(("binary", "bin")),
        FirmwareFormat::Elf | FirmwareFormat::Uf2 => {
            Err(anyhow!("Firmware Format not supported in objcopy"))
        }
    }?;

    let output_path = format!("{}.{}", out_name, ending);
    let mut objcopy = Command::new("llvm-objcopy");
    objcopy.arg(file);
    objcopy.arg("-O");
    objcopy.arg(format);
    objcopy.arg(&output_path);

    if verbosity > 1 {
        objcopy.arg(format!("-{}", "v".repeat((verbosity - 1) as usize)));
        eprintln!("{objcopy:?}");
    }

    let mut child = objcopy.spawn()?;

    let status = child.wait()?;
    if !status.success() {
        bail!("Failed to convert to {format}");
    }

    Ok(output_path)
}
