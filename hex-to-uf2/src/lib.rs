use std::{
    fs::File,
    io::{BufRead, BufReader, Write},
    path::Path,
};

use anyhow::{anyhow, Result};
use chips::ChipInfo;

use crate::block::Block;

pub mod block;

const ADDRESS_MASK: u32 = 0xff;
const INVERTED_ADDRESS_MASK: u32 = !ADDRESS_MASK;

pub fn hex_to_uf2(
    hex_lines: impl Iterator<Item = impl AsRef<str>>,
    chip_info: &Option<ChipInfo>,
) -> Result<Vec<u8>> {
    let mut upper: u32 = 0;
    let mut app_start_address: Option<u32> = None;

    let mut current_block: Option<Block> = None;

    let mut blocks: Vec<Block> = vec![];

    for hex_line in hex_lines {
        let hex_line = hex_line.as_ref();
        if !hex_line.starts_with(':') {
            continue;
        }

        let mut binary_line = vec![];
        let mut i = 1;

        while i < hex_line.len() - 1 {
            let byte = u8::from_str_radix(&hex_line[i..i + 2], 16).unwrap_or(0);
            binary_line.push(byte);
            i += 2;
        }

        let byte_count = binary_line[0] as usize;
        if binary_line.len() - 5 != byte_count {
            return Err(anyhow!("Invalid HEX line: mismatched byte count"));
        }

        if binary_line.len() < 4 {
            return Err(anyhow!(
                "Line with less than 4 encoded hex chars + 1 byte leads to undefined behaviour!"
            ));
        }

        match binary_line[3] {
            0 => {
                let mut address =
                    upper + (((binary_line[1] as u32) << 8) | (binary_line[2] as u32));
                if app_start_address.is_none() {
                    app_start_address = Some(address);
                }
                // Skip first 4 and last item
                for byte in binary_line.iter().take(binary_line.len() - 1).skip(4) {
                    if current_block.is_none()
                        || (current_block.as_ref().unwrap().address & INVERTED_ADDRESS_MASK)
                            != (address & INVERTED_ADDRESS_MASK)
                    {
                        if let Some(block) = current_block.take() {
                            blocks.push(block);
                        }
                        current_block = Some(Block::new(address & INVERTED_ADDRESS_MASK));
                    }
                    let current_mut_block = current_block
                        .as_mut()
                        .expect("current_block should always be some here");

                    current_mut_block.bytes[(address & ADDRESS_MASK) as usize] = *byte;
                    address += 1;
                }
            }
            1 => break, // End of file
            2 => {
                upper = (((binary_line[4] as u32) << 8) | (binary_line[5] as u32)) << 4;
            }
            4 => {
                upper = (((binary_line[4] as u32) << 8) | (binary_line[5] as u32)) << 16;
            }
            _ => {
                // do nothing
            }
        }
    }

    if let Some(block) = current_block.take() {
        // Add last block
        blocks.push(block);
    }

    let number_of_blocks = blocks.len() as u32;

    let family_id = chip_info.as_ref().map(|info| info.family_id);

    Ok(blocks
        .iter()
        .enumerate()
        .flat_map(|(i, block)| {
            block
                .encode(i as u32, number_of_blocks, family_id)
                .into_iter()
        })
        .collect())
}

pub fn hex_to_uf2_file(
    hex_file: &Path,
    output_path: &Path,
    family: &Option<ChipInfo>,
) -> Result<()> {
    let binary_buffer = BufReader::new(File::open(hex_file).expect("Couldn't open input file!"));

    let uf2_buffer = hex_to_uf2(
        binary_buffer
            .lines()
            .map(|line| line.expect("error reading line")),
        family,
    )
    .expect("Error converting hex to uf2");

    let mut file =
        File::create(output_path).expect("Error creating or overwriting destination file");

    file.write_all(&uf2_buffer)
        .expect("Error writing to destination");

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::io::Read;

    use chips::{get_info, Chip};

    use super::*;

    #[test]
    fn static_string() {
        let static_string = ":020000041000EA
:1000000000B5324B212058609868022188439860DF
:10001000D860186158612E4B002199600221596106
:100020000121F02299502B49196001219960352056
:1000300000F044F80222904214D00621196600F024
:1000400034F8196E01211966002018661A6600F04E
:100050002CF8196E196E196E052000F02FF8012189
:100060000842F9D1002199601B49196000215960AB";

        let uf2_bytes = hex_to_uf2(static_string.lines(), &None);

        println!("{uf2_bytes:X?}");
    }

    fn compare_to_python(
        input: &Path,
        output: &Path,
        python_out: &Path,
        family: &Option<ChipInfo>,
    ) {
        hex_to_uf2_file(input, output, family).unwrap();

        let rust_reader = BufReader::new(File::open(output).unwrap());
        let python_reader = BufReader::new(File::open(python_out).unwrap());

        let mut rust_bytes = rust_reader.bytes();
        let mut python_bytes = python_reader.bytes();

        let mut position: usize = 0;

        loop {
            match (rust_bytes.next(), python_bytes.next()) {
                (None, None) => break,
                (Some(_), None) => {
                    assert!(false, "rust_bytes is longer than python_bytes");
                    break;
                }
                (None, Some(_)) => {
                    assert!(false, "python_bytes is longer than rust_bytes");
                    break;
                }
                (Some(rust_byte), Some(python_byte)) => {
                    assert_eq!(
                        rust_byte.unwrap(),
                        python_byte.unwrap(),
                        "Difference at {position}, hex: {position:X}"
                    );
                }
            }

            position += 1;
        }
    }

    #[test]
    fn no_family_should_be_same() {
        compare_to_python(
            Path::new("./test/rmk.hex"),
            Path::new("./test/rmk.uf2"),
            Path::new("./test/rmk_py.uf2"),
            &None,
        );
    }

    #[test]
    fn family_should_be_same() {
        let chip_info = get_info(&Chip::RP2040);
        compare_to_python(
            Path::new("./test/rmk.hex"),
            Path::new("./test/rmk_id.uf2"),
            Path::new("./test/rmk_py_id.uf2"),
            &Some(chip_info),
        );
    }
}
