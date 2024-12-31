#[cfg(feature = "clap")]
use clap::ValueEnum;

/// All known chip families
#[derive(Clone, Debug)]
#[cfg_attr(feature = "clap", derive(ValueEnum))]
pub enum ChipFamily {
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

/// returns the id of a given family
pub fn get_family_id(family: ChipFamily) -> u32 {
    match family {
        ChipFamily::ATMEGA32 => 0x16573617, // Microchip (Atmel) ATmega32
        ChipFamily::SAML21 => 0x1851780a,   // Microchip (Atmel) SAML21
        ChipFamily::NRF52 => 0x1b57745f,    // Nordic NRF52
        ChipFamily::ESP32 => 0x1c5f21b0,    // ESP32
        ChipFamily::STM32L1 => 0x1e1f432d,  // ST STM32L1xx
        ChipFamily::STM32L0 => 0x202e3a91,  // ST STM32L0xx
        ChipFamily::STM32WL => 0x21460ff0,  // ST STM32WLxx
        ChipFamily::RTL8710B => 0x22e0d6fc, // Realtek AmebaZ RTL8710B
        ChipFamily::LPC55 => 0x2abc77ec,    // NXP LPC55xx
        ChipFamily::STM32G0 => 0x300f5633,  // ST STM32G0xx
        ChipFamily::GD32F350 => 0x31d228c6, // GD32F350
        ChipFamily::RTL8720D => 0x3379CFE2, // Realtek AmebaD RTL8720D
        ChipFamily::STM32L5 => 0x04240bdf,  // ST STM32L5xx
        ChipFamily::STM32G4 => 0x4c71240a,  // ST STM32G4xx
        ChipFamily::MIMXRT10XX => 0x4fb2d5bd, // NXP i.MX RT10XX
        ChipFamily::XR809 => 0x51e903a8,    // Xradiotech 809
        ChipFamily::STM32F7 => 0x53b80f00,  // ST STM32F7xx
        ChipFamily::SAMD51 => 0x55114460,   // Microchip (Atmel) SAMD51
        ChipFamily::STM32F4 => 0x57755a57,  // ST STM32F4xx
        ChipFamily::FX2 => 0x5a18069b,      // Cypress FX2
        ChipFamily::STM32F2 => 0x5d1a0a2e,  // ST STM32F2xx
        ChipFamily::STM32F1 => 0x5ee21072,  // ST STM32F103
        ChipFamily::NRF52833 => 0x621e937a, // Nordic NRF52833
        ChipFamily::STM32F0 => 0x647824b6,  // ST STM32F0xx
        ChipFamily::BK7231U => 0x675a40b0,  // Beken 7231U/7231T
        ChipFamily::SAMD21 => 0x68ed2b88,   // Microchip (Atmel) SAMD21
        ChipFamily::BK7251 => 0x6a82cc42,   // Beken 7251/7252
        ChipFamily::STM32F3 => 0x6b846188,  // ST STM32F3xx
        ChipFamily::STM32F407 => 0x6d0922fa, // ST STM32F407
        ChipFamily::STM32H7 => 0x6db66082,  // ST STM32H7xx
        ChipFamily::STM32WB => 0x70d16653,  // ST STM32WBxx
        ChipFamily::BK7231N => 0x7b3ef230,  // Beken 7231N
        ChipFamily::ESP8266 => 0x7eab61ed,  // ESP8266
        ChipFamily::KL32L2 => 0x7f83e793,   // NXP KL32L2x
        ChipFamily::STM32F407VG => 0x8fb060fe, // ST STM32F407VG
        ChipFamily::RTL8710A => 0x9fffd543, // Realtek Ameba1 RTL8710A
        ChipFamily::NRF52840 => 0xada52840, // Nordic NRF52840
        ChipFamily::ESP32S2 => 0xbfdd4eee,  // ESP32-S2
        ChipFamily::ESP32S3 => 0xc47e5767,  // ESP32-S3
        ChipFamily::ESP32C3 => 0xd42ba06c,  // ESP32-C3
        ChipFamily::ESP32C2 => 0x2b88d29c,  // ESP32-C2
        ChipFamily::ESP32H2 => 0x332726f6,  // ESP32-H2
        ChipFamily::ESP32C6 => 0x540ddf62,  // ESP32-C6
        ChipFamily::ESP32P4 => 0x3d308e94,  // ESP32-P4
        ChipFamily::ESP32C5 => 0xf71c0343,  // ESP32-C5
        ChipFamily::ESP32C61 => 0x77d850c4, // ESP32-C61
        ChipFamily::BL602 => 0xde1270b7,    // Boufallo 602
        ChipFamily::RTL8720C => 0xe08f7564, // Realtek AmebaZ2 RTL8720C
        ChipFamily::RP2040 => 0xe48bff56,   // Raspberry Pi RP2040
        ChipFamily::Rp2xxxAbsolute => 0xe48bff57, // Raspberry Pi Microcontrollers: Absolute (unpartitioned) download
        ChipFamily::Rp2xxxData => 0xe48bff58, // Raspberry Pi Microcontrollers: Data partition download
        ChipFamily::Rp2350ArmS => 0xe48bff59, // Raspberry Pi RP2350, Secure Arm image
        ChipFamily::Rp2350Riscv => 0xe48bff5a, // Raspberry Pi RP2350, RISC-V image
        ChipFamily::Rp2350ArmNs => 0xe48bff5b, // Raspberry Pi RP2350, Non-secure Arm image
        ChipFamily::STM32L4 => 0x00ff6919,    // ST STM32L4xx
        ChipFamily::GD32VF103 => 0x9af03e33,  // GigaDevice GD32VF103
        ChipFamily::CSK4 => 0x4f6ace52,       // LISTENAI CSK300x/400x
        ChipFamily::CSK6 => 0x6e7348a8,       // LISTENAI CSK60xx
        ChipFamily::M0SENSE => 0x11de784a,    // M0SENSE BL702
        ChipFamily::MaixPlayU4 => 0x4b684d71, // Sipeed MaixPlay-U4(BL618)
        ChipFamily::RZA1LU => 0x9517422f,     // Renesas RZ/A1LU (R7S7210xx)
        ChipFamily::STM32F411xE => 0x2dc309c5, // ST STM32F411xE
        ChipFamily::STM32F411xC => 0x06d1097b, // ST STM32F411xC
        ChipFamily::NRF52832xxAA => 0x72721d4e, // Nordic NRF52832xxAA
        ChipFamily::NRF52832xxAB => 0x6f752678, // Nordic NRF52832xxAB
        ChipFamily::AT32F415 => 0xa0c97b8e,   // ArteryTek AT32F415
        ChipFamily::CH32V => 0x699b62ec,      // WCH CH32V2xx and CH32V3xx
        ChipFamily::RA4M1 => 0x7be8976d,      // Renesas RA4M1
    }
}
