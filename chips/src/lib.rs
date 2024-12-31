use std::fmt::Display;

use clap::ValueEnum;
use once_cell::sync::Lazy;
use serde_derive::{Deserialize, Serialize};
use strum::{EnumIter, EnumString, IntoEnumIterator};

/// All known boards
#[derive(Clone, Debug, Serialize, Deserialize, EnumIter, EnumString)]
#[serde(rename_all = "kebab-case")]
#[strum(serialize_all = "kebab-case")]
pub enum Board {
    NrfMicro,
    BlueMicro840,
    PuchiBle,
    NiceNano,
    NiceNanoV2,
    XiaoBle,
    Liatris,
    EliteC,
    ProMicro,
}
static BOARD_VARIANTS: Lazy<Vec<Board>> = Lazy::new(|| Board::iter().collect());

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string_value = serde_json::to_string(self).expect("Enum serialization failed");
        write!(f, "{}", string_value.trim_matches('"'))
    }
}

impl ValueEnum for Board {
    fn value_variants<'a>() -> &'a [Self] {
        &BOARD_VARIANTS
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        Some(clap::builder::PossibleValue::new(self.to_string()))
    }
}

/// All known chip families
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, EnumIter, EnumString)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum Chip {
    AT32F415,
    ATMEGA32,
    BK7231N,
    BK7231U,
    BK7251,
    BL602,
    CH32V,
    CSK4,
    CSK6,
    ESP32,
    ESP32C2,
    ESP32C3,
    ESP32C5,
    ESP32C6,
    ESP32C61,
    ESP32H2,
    ESP32P4,
    ESP32S2,
    ESP32S3,
    ESP8266,
    FX2,
    GD32F350,
    GD32VF103,
    KL32L2,
    LPC55,
    M0SENSE,
    MIMXRT10XX,
    MaixPlayU4,
    NRF52,
    NRF52832xxAA,
    NRF52832xxAB,
    NRF52833,
    NRF52840,
    RA4M1,
    RP2040,
    Rp2350ArmNs,
    Rp2350ArmS,
    Rp2350Riscv,
    Rp2xxxAbsolute,
    Rp2xxxData,
    RTL8710A,
    RTL8710B,
    RTL8720C,
    RTL8720D,
    RZA1LU,
    SAMD21,
    SAMD51,
    SAML21,
    STM32F0,
    STM32F1,
    STM32F2,
    STM32F3,
    STM32F4,
    STM32F407,
    STM32F407VG,
    STM32F411xC,
    STM32F411xE,
    STM32F7,
    STM32G0,
    STM32G4,
    STM32H7,
    STM32L0,
    STM32L1,
    STM32L4,
    STM32L5,
    STM32WB,
    STM32WL,
    XR809,
}
static CHIP_VARIANTS: Lazy<Vec<Chip>> = Lazy::new(|| Chip::iter().collect());

pub fn get_all_chip_info() -> Vec<ChipInfo> {
    CHIP_VARIANTS
        .iter()
        .map(|variant| get_info(&variant))
        .collect()
}

impl ValueEnum for Chip {
    fn value_variants<'a>() -> &'a [Self] {
        &CHIP_VARIANTS
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        Some(clap::builder::PossibleValue::new(self.to_string()))
    }
}

impl Display for Chip {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string_value = serde_json::to_string(self).expect("Enum serialization failed");
        write!(f, "{}", string_value.trim_matches('"'))
    }
}

#[derive(Debug, Clone)]
pub struct ChipInfo {
    pub family_id: u32,
    pub name: String,
    pub firmware_formats: Vec<FirmwareFormat>,
    pub split_support: bool,
    pub chip: Chip,
}

#[derive(Debug, Clone)]
pub enum FirmwareFormat {
    Bin,
    Elf,
    Hex,
    Uf2,
}

pub fn get_chip(board: &Board) -> Chip {
    match board {
        Board::NrfMicro
        | Board::BlueMicro840
        | Board::PuchiBle
        | Board::NiceNano
        | Board::NiceNanoV2
        | Board::XiaoBle => Chip::NRF52840,
        Board::Liatris => Chip::RP2040,
        Board::EliteC | Board::ProMicro => Chip::ATMEGA32,
    }
}

/// returns the id of a given family
pub fn get_info(family: &Chip) -> ChipInfo {
    match family {
        Chip::ATMEGA32 => ChipInfo {
            // Microchip (Atmel) ATmega32
            family_id: 0x16573617,
            name: Chip::ATMEGA32.to_string(),
            firmware_formats: vec![],
            split_support: false,
            chip: Chip::ATMEGA32,
        },
        Chip::SAML21 => ChipInfo {
            // Microchip (Atmel) SAML21
            family_id: 0x1851780a,
            name: Chip::SAML21.to_string(),
            firmware_formats: vec![],
            split_support: false,
            chip: Chip::SAML21,
        },
        Chip::NRF52 => ChipInfo {
            // Nordic NRF52
            family_id: 0x1b57745f,
            name: Chip::NRF52.to_string(),
            firmware_formats: vec![],
            split_support: false,
            chip: Chip::NRF52,
        },
        Chip::ESP32 => ChipInfo {
            // ESP32
            family_id: 0x1c5f21b0,
            name: Chip::ESP32.to_string(),
            firmware_formats: vec![],
            split_support: false,
            chip: Chip::ESP32,
        },
        Chip::STM32L1 => ChipInfo {
            // ST STM32L1xx
            family_id: 0x1e1f432d,
            name: Chip::STM32L1.to_string(),
            firmware_formats: vec![],
            split_support: false,
            chip: Chip::STM32L1,
        },
        Chip::STM32L0 => ChipInfo {
            // ST STM32L0xx
            family_id: 0x202e3a91,
            name: Chip::STM32L0.to_string(),
            firmware_formats: vec![],
            split_support: false,
            chip: Chip::STM32L0,
        },
        Chip::STM32WL => ChipInfo {
            // ST STM32WLxx
            family_id: 0x21460ff0,
            name: Chip::STM32WL.to_string(),
            firmware_formats: vec![],
            split_support: false,
            chip: Chip::STM32WL,
        },
        Chip::RTL8710B => ChipInfo {
            // Realtek AmebaZ RTL8710B
            family_id: 0x22e0d6fc,
            name: Chip::RTL8710B.to_string(),
            firmware_formats: vec![],
            split_support: false,
            chip: Chip::RTL8710B,
        },
        Chip::LPC55 => ChipInfo {
            // NXP LPC55xx
            family_id: 0x2abc77ec,
            name: Chip::LPC55.to_string(),
            firmware_formats: vec![],
            split_support: false,
            chip: Chip::LPC55,
        },
        Chip::STM32G0 => ChipInfo {
            // ST STM32G0xx
            family_id: 0x300f5633,
            name: Chip::STM32G0.to_string(),
            firmware_formats: vec![],
            split_support: false,
            chip: Chip::STM32G0,
        },
        Chip::GD32F350 => ChipInfo {
            // GD32F350
            family_id: 0x31d228c6,
            name: Chip::GD32F350.to_string(),
            firmware_formats: vec![],
            split_support: false,
            chip: Chip::GD32F350,
        },
        Chip::RTL8720D => ChipInfo {
            // Realtek AmebaD RTL8720D
            family_id: 0x3379CFE2,
            name: Chip::RTL8720D.to_string(),
            firmware_formats: vec![],
            split_support: false,
            chip: Chip::RTL8720D,
        },
        Chip::STM32L5 => ChipInfo {
            // ST STM32L5xx
            family_id: 0x04240bdf,
            name: Chip::STM32L5.to_string(),
            firmware_formats: vec![],
            split_support: false,
            chip: Chip::STM32L5,
        },
        Chip::STM32G4 => ChipInfo {
            // ST STM32G4xx
            family_id: 0x4c71240a,
            name: Chip::STM32G4.to_string(),
            firmware_formats: vec![],
            split_support: false,
            chip: Chip::STM32G4,
        },
        Chip::MIMXRT10XX => ChipInfo {
            // NXP i.MX RT10XX
            family_id: 0x4fb2d5bd,
            name: Chip::MIMXRT10XX.to_string(),
            firmware_formats: vec![],
            split_support: false,
            chip: Chip::MIMXRT10XX,
        },
        Chip::XR809 => ChipInfo {
            // Xradiotech 809
            family_id: 0x51e903a8,
            name: Chip::XR809.to_string(),
            firmware_formats: vec![],
            split_support: false,
            chip: Chip::XR809,
        },
        Chip::STM32F7 => ChipInfo {
            // ST STM32F7xx
            family_id: 0x53b80f00,
            name: Chip::STM32F7.to_string(),
            firmware_formats: vec![],
            split_support: false,
            chip: Chip::STM32F7,
        },
        Chip::SAMD51 => ChipInfo {
            // Microchip (Atmel) SAMD51
            family_id: 0x55114460,
            name: Chip::SAMD51.to_string(),
            firmware_formats: vec![],
            split_support: false,
            chip: Chip::SAMD51,
        },
        Chip::STM32F4 => ChipInfo {
            // ST STM32F4xx
            family_id: 0x57755a57,
            name: Chip::STM32F4.to_string(),
            firmware_formats: vec![],
            split_support: false,
            chip: Chip::STM32F4,
        },
        Chip::FX2 => ChipInfo {
            // Cypress FX2
            family_id: 0x5a18069b,
            name: Chip::FX2.to_string(),
            firmware_formats: vec![],
            split_support: false,
            chip: Chip::FX2,
        },
        Chip::STM32F2 => ChipInfo {
            // ST STM32F2xx
            family_id: 0x5d1a0a2e,
            name: Chip::STM32F2.to_string(),
            firmware_formats: vec![],
            split_support: false,
            chip: Chip::STM32F2,
        },
        Chip::STM32F1 => ChipInfo {
            // ST STM32F103
            family_id: 0x5ee21072,
            name: Chip::STM32F1.to_string(),
            firmware_formats: vec![],
            split_support: false,
            chip: Chip::STM32F1,
        },
        Chip::NRF52833 => ChipInfo {
            // Nordic NRF52833
            family_id: 0x621e937a,
            name: Chip::NRF52833.to_string(),
            firmware_formats: vec![],
            split_support: false,
            chip: Chip::NRF52833,
        },
        Chip::STM32F0 => ChipInfo {
            // ST STM32F0xx
            family_id: 0x647824b6,
            name: Chip::STM32F0.to_string(),
            firmware_formats: vec![],
            split_support: false,
            chip: Chip::STM32F0,
        },
        Chip::BK7231U => ChipInfo {
            // Beken 7231U/7231T
            family_id: 0x675a40b0,
            name: Chip::BK7231U.to_string(),
            firmware_formats: vec![],
            split_support: false,
            chip: Chip::BK7231U,
        },
        Chip::SAMD21 => ChipInfo {
            // Microchip (Atmel) SAMD21
            family_id: 0x68ed2b88,
            name: Chip::SAMD21.to_string(),
            firmware_formats: vec![],
            split_support: false,
            chip: Chip::SAMD21,
        },
        Chip::BK7251 => ChipInfo {
            // Beken 7251/7252
            family_id: 0x6a82cc42,
            name: Chip::BK7251.to_string(),
            firmware_formats: vec![],
            split_support: false,
            chip: Chip::BK7251,
        },
        Chip::STM32F3 => ChipInfo {
            // ST STM32F3xx
            family_id: 0x6b846188,
            name: Chip::STM32F3.to_string(),
            firmware_formats: vec![],
            split_support: false,
            chip: Chip::STM32F3,
        },
        Chip::STM32F407 => ChipInfo {
            // ST STM32F407
            family_id: 0x6d0922fa,
            name: Chip::STM32F407.to_string(),
            firmware_formats: vec![],
            split_support: false,
            chip: Chip::STM32F407,
        },
        Chip::STM32H7 => ChipInfo {
            // ST STM32H7xx
            family_id: 0x6db66082,
            name: Chip::STM32H7.to_string(),
            firmware_formats: vec![],
            split_support: false,
            chip: Chip::STM32H7,
        },
        Chip::STM32WB => ChipInfo {
            // ST STM32WBxx
            family_id: 0x70d16653,
            name: Chip::STM32WB.to_string(),
            firmware_formats: vec![],
            split_support: false,
            chip: Chip::STM32WB,
        },
        Chip::BK7231N => ChipInfo {
            // Beken 7231N
            family_id: 0x7b3ef230,
            name: Chip::BK7231N.to_string(),
            firmware_formats: vec![],
            split_support: false,
            chip: Chip::BK7231N,
        },
        Chip::ESP8266 => ChipInfo {
            // ESP8266
            family_id: 0x7eab61ed,
            name: Chip::ESP8266.to_string(),
            firmware_formats: vec![],
            split_support: false,
            chip: Chip::ESP8266,
        },
        Chip::KL32L2 => ChipInfo {
            // NXP KL32L2x
            family_id: 0x7f83e793,
            name: Chip::KL32L2.to_string(),
            firmware_formats: vec![],
            split_support: false,
            chip: Chip::KL32L2,
        },
        Chip::STM32F407VG => ChipInfo {
            // ST STM32F407VG
            family_id: 0x8fb060fe,
            name: Chip::STM32F407VG.to_string(),
            firmware_formats: vec![],
            split_support: false,
            chip: Chip::STM32F407VG,
        },
        Chip::RTL8710A => ChipInfo {
            // Realtek Ameba1 RTL8710A
            family_id: 0x9fffd543,
            name: Chip::RTL8710A.to_string(),
            firmware_formats: vec![],
            split_support: false,
            chip: Chip::RTL8710A,
        },
        Chip::NRF52840 => ChipInfo {
            // Nordic NRF52840
            family_id: 0xada52840,
            name: Chip::NRF52840.to_string(),
            firmware_formats: vec![],
            split_support: true,
            chip: Chip::NRF52840,
        },
        Chip::ESP32S2 => ChipInfo {
            // ESP32-S2
            family_id: 0xbfdd4eee,
            name: Chip::ESP32S2.to_string(),
            firmware_formats: vec![],
            split_support: false,
            chip: Chip::ESP32S2,
        },
        Chip::ESP32S3 => ChipInfo {
            // ESP32-S3
            family_id: 0xc47e5767,
            name: Chip::ESP32S3.to_string(),
            firmware_formats: vec![],
            split_support: false,
            chip: Chip::ESP32S3,
        },
        Chip::ESP32C3 => ChipInfo {
            // ESP32-C3
            family_id: 0xd42ba06c,
            name: Chip::ESP32C3.to_string(),
            firmware_formats: vec![],
            split_support: false,
            chip: Chip::ESP32C3,
        },
        Chip::ESP32C2 => ChipInfo {
            // ESP32-C2
            family_id: 0x2b88d29c,
            name: Chip::ESP32C2.to_string(),
            firmware_formats: vec![],
            split_support: false,
            chip: Chip::ESP32C2,
        },
        Chip::ESP32H2 => ChipInfo {
            // ESP32-H2
            family_id: 0x332726f6,
            name: Chip::ESP32H2.to_string(),
            firmware_formats: vec![],
            split_support: false,
            chip: Chip::ESP32H2,
        },
        Chip::ESP32C6 => ChipInfo {
            // ESP32-C6
            family_id: 0x540ddf62,
            name: Chip::ESP32C6.to_string(),
            firmware_formats: vec![],
            split_support: false,
            chip: Chip::ESP32C6,
        },
        Chip::ESP32P4 => ChipInfo {
            // ESP32-P4
            family_id: 0x3d308e94,
            name: Chip::ESP32P4.to_string(),
            firmware_formats: vec![],
            split_support: false,
            chip: Chip::ESP32P4,
        },
        Chip::ESP32C5 => ChipInfo {
            // ESP32-C5
            family_id: 0xf71c0343,
            name: Chip::ESP32C5.to_string(),
            firmware_formats: vec![],
            split_support: false,
            chip: Chip::ESP32C5,
        },
        Chip::ESP32C61 => ChipInfo {
            // ESP32-C61
            family_id: 0x77d850c4,
            name: Chip::ESP32C61.to_string(),
            firmware_formats: vec![],
            split_support: false,
            chip: Chip::ESP32C61,
        },
        Chip::BL602 => ChipInfo {
            // Boufallo 602
            family_id: 0xde1270b7,
            name: Chip::BL602.to_string(),
            firmware_formats: vec![],
            split_support: false,
            chip: Chip::BL602,
        },
        Chip::RTL8720C => ChipInfo {
            // Realtek AmebaZ2 RTL8720C
            family_id: 0xe08f7564,
            name: Chip::RTL8720C.to_string(),
            firmware_formats: vec![],
            split_support: false,
            chip: Chip::RTL8720C,
        },
        Chip::RP2040 => ChipInfo {
            // Raspberry Pi RP2040
            family_id: 0xe48bff56,
            name: Chip::RP2040.to_string(),
            firmware_formats: vec![],
            split_support: true,
            chip: Chip::RP2040,
        },
        Chip::Rp2xxxAbsolute => ChipInfo {
            // Raspberry Pi Microcontrollers: Absolute (unpartitioned) download
            family_id: 0xe48bff57,
            name: Chip::Rp2xxxAbsolute.to_string(),
            firmware_formats: vec![],
            split_support: false,
            chip: Chip::Rp2xxxAbsolute,
        },
        Chip::Rp2xxxData => ChipInfo {
            // Raspberry Pi Microcontrollers: Data partition download
            family_id: 0xe48bff58,
            name: Chip::Rp2xxxData.to_string(),
            firmware_formats: vec![],
            split_support: false,
            chip: Chip::Rp2xxxData,
        },
        Chip::Rp2350ArmS => ChipInfo {
            // Raspberry Pi RP2350, Secure Arm image
            family_id: 0xe48bff59,
            name: Chip::Rp2350ArmS.to_string(),
            firmware_formats: vec![],
            split_support: false,
            chip: Chip::Rp2350ArmS,
        },
        Chip::Rp2350Riscv => ChipInfo {
            // Raspberry Pi RP2350, RISC-V image
            family_id: 0xe48bff5a,
            name: Chip::Rp2350Riscv.to_string(),
            firmware_formats: vec![],
            split_support: false,
            chip: Chip::Rp2350Riscv,
        },
        Chip::Rp2350ArmNs => ChipInfo {
            // Raspberry Pi RP2350, Non-secure Arm image
            family_id: 0xe48bff5b,
            name: Chip::Rp2350ArmNs.to_string(),
            firmware_formats: vec![],
            split_support: false,
            chip: Chip::Rp2350ArmNs,
        },
        Chip::STM32L4 => ChipInfo {
            // ST STM32L4xx
            family_id: 0x00ff6919,
            name: Chip::STM32L4.to_string(),
            firmware_formats: vec![],
            split_support: false,
            chip: Chip::STM32L4,
        },
        Chip::GD32VF103 => ChipInfo {
            // GigaDevice GD32VF103
            family_id: 0x9af03e33,
            name: Chip::GD32VF103.to_string(),
            firmware_formats: vec![],
            split_support: false,
            chip: Chip::GD32VF103,
        },
        Chip::CSK4 => ChipInfo {
            // LISTENAI CSK300x/400x
            family_id: 0x4f6ace52,
            name: Chip::CSK4.to_string(),
            firmware_formats: vec![],
            split_support: false,
            chip: Chip::CSK4,
        },
        Chip::CSK6 => ChipInfo {
            // LISTENAI CSK60xx
            family_id: 0x6e7348a8,
            name: Chip::CSK6.to_string(),
            firmware_formats: vec![],
            split_support: false,
            chip: Chip::CSK6,
        },
        Chip::M0SENSE => ChipInfo {
            // M0SENSE BL702
            family_id: 0x11de784a,
            name: Chip::M0SENSE.to_string(),
            firmware_formats: vec![],
            split_support: false,
            chip: Chip::M0SENSE,
        },
        Chip::MaixPlayU4 => ChipInfo {
            // Sipeed MaixPlay-U4(BL618)
            family_id: 0x4b684d71,
            name: Chip::MaixPlayU4.to_string(),
            firmware_formats: vec![],
            split_support: false,
            chip: Chip::MaixPlayU4,
        },
        Chip::RZA1LU => ChipInfo {
            // Renesas RZ/A1LU (R7S7210xx)
            family_id: 0x9517422f,
            name: Chip::RZA1LU.to_string(),
            firmware_formats: vec![],
            split_support: false,
            chip: Chip::RZA1LU,
        },
        Chip::STM32F411xE => ChipInfo {
            // ST STM32F411xE
            family_id: 0x2dc309c5,
            name: Chip::STM32F411xE.to_string(),
            firmware_formats: vec![],
            split_support: false,
            chip: Chip::STM32F411xE,
        },
        Chip::STM32F411xC => ChipInfo {
            // ST STM32F411xC
            family_id: 0x06d1097b,
            name: Chip::STM32F411xC.to_string(),
            firmware_formats: vec![],
            split_support: false,
            chip: Chip::STM32F411xC,
        },
        Chip::NRF52832xxAA => ChipInfo {
            // Nordic NRF52832xxAA
            family_id: 0x72721d4e,
            name: Chip::NRF52832xxAA.to_string(),
            firmware_formats: vec![],
            split_support: false,
            chip: Chip::NRF52832xxAA,
        },
        Chip::NRF52832xxAB => ChipInfo {
            // Nordic NRF52832xxAB
            family_id: 0x6f752678,
            name: Chip::NRF52832xxAB.to_string(),
            firmware_formats: vec![],
            split_support: false,
            chip: Chip::NRF52832xxAB,
        },
        Chip::AT32F415 => ChipInfo {
            // ArteryTek AT32F415
            family_id: 0xa0c97b8e,
            name: Chip::AT32F415.to_string(),
            firmware_formats: vec![],
            split_support: false,
            chip: Chip::AT32F415,
        },
        Chip::CH32V => ChipInfo {
            // WCH CH32V2xx and CH32V3xx
            family_id: 0x699b62ec,
            name: Chip::CH32V.to_string(),
            firmware_formats: vec![],
            split_support: false,
            chip: Chip::CH32V,
        },
        Chip::RA4M1 => ChipInfo {
            // Renesas RA4M1
            family_id: 0x7be8976d,
            name: Chip::RA4M1.to_string(),
            firmware_formats: vec![],
            split_support: false,
            chip: Chip::RA4M1,
        },
    }
}
