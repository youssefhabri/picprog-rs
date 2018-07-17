use std::collections::HashMap;

//const POWER_SEQUENCE: HashMap<&str, i8> = [
//    ("vcc", 0),
//    ("vccvpp1", 1),
//    ("vccvpp2", 2),
//    ("vpp1vcc", 3),
//    ("vpp2vcc", 4),
//    ("vccfastvpp", 1),
//    ("vccfastvpp2", 2)
//].iter().cloned().collect();
//
//static VCC_VPP_DELAY: HashMap<&str, bool> = [
//    ("vcc", false),
//    ("vccvpp1", false),
//    ("vccvpp2", false),
//    ("vpp1vcc", false),
//    ("vpp2vcc", false),
//    ("vccfastvpp1", true),
//    ("vccfastvpp2", true)
//].iter().cloned().collect();
//
//const SOCKET_IMAGE: HashMap<&str, &str> = [
//    ("8pin", "socket pin 13"),
//    ("14pin", "socket pin 13"),
//    ("18pin", "socket pin 2"),
//    ("28Npin", "socket pin 1"),
//    ("40pin", "socket pin 1")
//].iter().cloned().collect();
//
//const BOOLEAN: HashMap<&str, bool> = [
//    ("y", true),
//    ("1", true),
//    ("n", false),
//    ("0", false)
//].iter().cloned().collect();
//
//const CORE_TYPE: HashMap<&str, i8> = [
//    ("bit16_a", 1),
//    ("bit16_b", 2),
//    ("bit14_g", 3),
//    ("bit12_a", 4),
//    ("bit14_a", 5),
//    ("bit14_b", 6),
//    ("bit14_c", 7),
//    ("bit14_d", 8),
//    ("bit14_e", 9),
//    ("bit14_f", 10),
//    ("bit12_b", 11),
//    ("bit14_h", 12),
//    ("bit16_c", 13)
//].iter().cloned().collect();

pub struct FuseOption {
    name: &'static str,
    value: u16
}

// TODO implement reading fuses from file
pub struct Fuse {
    name: &'static str,
    options: Vec<FuseOption>,
    current_option: u16
}

pub struct ChipInfo {
    name: &'static str,
    include: bool,
    socket_image: &'static str,
    erase_mode: u8,
    flash_chip: bool,
    power_sequence: u8,
    power_sequence_str: &'static str,
    program_delay: u8,
    program_tries: u8,
    over_program: u8,
    core_type: u8,
    rom_size: u16,
    eeprom_size: u8,
    fuse_blank: u32,
    cpwarn: bool,
    flag_calibration_value_in_rom: bool,
    flag_band_gap_fuse: bool,
    // flag_18f_single_panel_access_mode
    icsp_only: bool,
    chip_id: u32,
    fuses: Vec<Fuse>
}

impl ChipInfo {
    pub fn new() -> ChipInfo {
        ChipInfo {
            name: "16F877A",
            include: true,
            socket_image: "socket pin 1",
            erase_mode: 5,
            flash_chip: true,
            power_sequence: 1,
            power_sequence_str: "VccVpp1",
            program_delay: 10,
            program_tries: 1,
            over_program: 1,
            core_type: 9,
            rom_size: 00002000,
            eeprom_size: 00000100,
            fuse_blank: 0x3FFF,
            cpwarn: false,
            flag_calibration_value_in_rom: false,
            flag_band_gap_fuse: false,
            icsp_only: false,
            chip_id: 0x0E20,
            fuses: vec![
                Fuse { name: "WDT", options: vec![ FuseOption { name: "Enabled", value: 0x3FFF }, FuseOption { name: "Disabled", value: 0x3FFB } ], current_option: 0 },
                Fuse { name: "PWRTE", options: vec![ FuseOption { name: "Enabled", value: 0x3FBF }, FuseOption { name: "Disabled", value: 0x3FFF } ], current_option: 0 },
                Fuse { name: "BODEN", options: vec![ FuseOption { name: "Enabled", value: 0x3FFF }, FuseOption { name: "Disabled", value: 0x3FBF } ], current_option: 0 },
                Fuse { name: "LVP", options: vec![ FuseOption { name: "Enabled", value: 0x3FFF }, FuseOption { name: "Disabled", value: 0x3F7F } ], current_option: 0 },
                Fuse { name: "CPD", options: vec![ FuseOption { name: "Enabled", value: 0x3EFF }, FuseOption { name: "Disabled", value: 0x3FFF } ], current_option: 0 },
                Fuse { name: "WRT Enable", options: vec![
                    FuseOption { name: "Enabled", value: 0x3FFF },
                    FuseOption { name: "WRT_256", value: 0x3DFF },
                    FuseOption { name: "WRT_1FOURTH", value: 0x3BFF },
                    FuseOption { name: "WRT_HALF", value: 0x39FF },
                ], current_option: 0 },
                Fuse { name: "DEBUG", options: vec![ FuseOption { name: "Enabled", value: 0x37FF }, FuseOption { name: "Disabled", value: 0x3FFF } ], current_option: 0 },
                Fuse { name: "Oscillator", options: vec![
                    FuseOption { name: "RC", value: 0x3FFF },
                    FuseOption { name: "HS", value: 0x3FFE },
                    FuseOption { name: "XT", value: 0x3FFD },
                    FuseOption { name: "LP", value: 0x3FFC },
                ], current_option: 0 },
                Fuse { name: "Code Protect", options: vec![ FuseOption { name: "Enabled", value: 0x1FFF }, FuseOption { name: "Disabled", value: 0x3FFF } ], current_option: 0 },
            ]
        }
    }

    pub fn name(&self) -> &str {
        self.name
    }

    pub fn erase_mode(&self) -> u8 {
        self.erase_mode
    }

    pub fn flash_chip(&self) -> bool {
        self.flash_chip
    }

    pub fn power_sequence(&self) -> u8 {
        self.power_sequence
    }

    pub fn power_sequence_str(&self) -> &str {
        self.power_sequence_str
    }

    pub fn program_delay(&self) -> u8 {
        self.program_delay
    }

    pub fn program_tries(&self) -> u8 {
        self.program_tries
    }

    pub fn over_program(&self) -> u8 {
        self.over_program
    }

    pub fn core_type(&self) -> u8 {
        self.core_type
    }

    pub fn core_bits(&self) -> u32 {
        match self.core_type {
            1 | 2 => 16,
            3 | 5 | 6 | 7 | 8 | 9 | 10 => 14,
            4 => 16,
            _ => 0
        }
    }

    pub fn rom_size(&self) -> u16 {
        self.rom_size
    }

    pub fn eeprom_size(&self) -> u8 {
        self.eeprom_size
    }

    pub fn fuse_blank(&self) -> u32 {
        self.fuse_blank
    }

    pub fn cpwarn(&self) -> bool {
        self.cpwarn
    }

    pub fn flag_calibration_value_in_rom(&self) -> bool {
        self.flag_calibration_value_in_rom
    }

    pub fn flag_band_gap_fuse(&self) -> bool {
        self.flag_band_gap_fuse
    }

    pub fn icsp_only(&self) -> bool {
        self.icsp_only
    }

    pub fn flag_18f_single_panel_access_mode(&self) -> bool {
        self.core_type == 1
    }

    pub fn chip_id(&self) -> u32 {
        self.chip_id
    }

    pub fn fuses(&self) -> &Vec<Fuse> {
        &self.fuses
    }

    pub fn flag_vcc_vpp_delay(&self) -> bool {
        match self.power_sequence_str.to_lowercase().as_ref() {
            "vcc" => false,
            "vccvpp1" => false,
            "vccvpp2" => false,
            "vpp1vcc" => false,
            "vpp2vcc" => false,
            "vccfastvpp1" => true,
            "vccfastvpp2" => true,
            _ => false
        }
    }
}
