use std::time::{Duration, Instant};
use std::ops::Add;
use std::cmp::Ordering;
use std::str;
use std::io;
use byteorder::WriteBytesExt;
use serialport::*;

use utils;
use chipinfo::{ChipInfo, Fuse};

const FIRMWARE: [&str; 4] = [
    "K128", "K149-A", "K149-B", "K150"
];

pub struct ProtocolInterface {
    port: Box<SerialPort>,
    vars_set: bool,
    fuses_set: bool,
    vars: ChipInfo,
}

impl ProtocolInterface {
    pub fn new(port: Box<SerialPort>) -> ProtocolInterface {
        let mut pi = ProtocolInterface {
            port,
            vars_set: false,
            fuses_set: false,
            vars: ChipInfo::new()
        };

        match pi.port.set_timeout(Duration::from_millis(100)) {
            Err(err) => panic!(err),
            _ => ()
        }

        return pi
    }

    fn read(&mut self, mut count: usize, timeout: u64) -> Vec<u8> {
        let mut result: Vec<u8> = vec![];
        let init_time = Instant::now();
        let end_time = init_time.add(Duration::from_secs(timeout));

        // TODO check if timeout works correctly
        let mut serial_buf: Vec<u8> = vec![0; 255];
        while count > 0 && (timeout == 0 || Instant::now().cmp(&end_time) == Ordering::Less) {
            match self.port.read(serial_buf.as_mut_slice()) {
                Ok(n) => {
                    for v in &serial_buf[..n] {
                        result.push(*v);
                    }
                    count = count - n
                },
                Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
                Err(e) => eprintln!("{:?}", e),
            }
        }

        return result
    }

    fn expect(&mut self, expected: Vec<u8>, timeout: u64) {
        let response = self.read(expected.len(), timeout);
        if response != expected {
            panic!("expected {:?}, received {:?}.", expected, response);
        }
    }

    pub fn reset(&mut self) -> bool {
        match self.port.write_data_terminal_ready(true) {
            Err(err) => panic!(err),
            _ => ()
        }
        utils::sleep_ms(100);

        match self.port.flush() {
            Err(err) => panic!(err),
            _ => ()
        }

        match self.port.write_data_terminal_ready(false) {
            Err(err) => panic!(err),
            _ => ()
        }
        utils::sleep_ms(100);

        let mut response = self.read(2, 300);
        if response.len() == 0 {
            match self.port.write_data_terminal_ready(true) {
                Err(err) => panic!(err),
                _ => ()
            }
            utils::sleep_ms(100);
            response = self.read(2, 300)
        }

        let mut result = false;
        if response.len() >= 1 {
            result = response[0] == 'B' as u8;
        }

        if result && response.len() == 2 {
            println!("Firmware: {}", FIRMWARE[response[1] as usize]);
        }

        return result
    }

    fn command_start(&mut self, command: u8) -> bool {
        self.port.write_u8(b'\x01');
        self.expect(vec![b'Q'], 10);

        self.port.write_u8(b'P');

        let ack = self.read(1, 5);
        let result = ack[0] == b'P';
        if !result {
            panic!("no acknowledgement for command start.")
        }

        if command > 0 {
            self.port.write_u8(command);
        }

        return result
    }

    fn null_command(&mut self) {
        self.port.write_u8(0);
    }

    fn command_end(&mut self) -> bool {
        self.port.write_u8(1);
        let ack = self.read(1, 10);
        let result = ack[0] == b'Q';

        if !result {
            if ack.len() != 0 {
                panic!("unexpected response ({:?}) in command end.", ack)
            } else {
                panic!("no acknowledgement for command end.")
            }
        }

        return result
    }

    pub fn echo(&mut self, message: &str) -> Vec<u8> {
        let cmd = 2;
        let _ = self.command_start(0);
        let mut result: Vec<u8> = vec![];
        for c in message.chars() {
            self.port.write_u8(cmd);
            self.port.write_u8(c as u8);
            let mut response = self.read(1, 5);
            result.append(&mut response);
        }
        let _ = self.command_end();

        return result
    }

    pub fn init_programming_vars(&mut self) -> bool {
        let cmd = 3;
        self.command_start(cmd);

        let flags = (self.vars.flag_calibration_value_in_rom() as u8 & 1)
                    | (self.vars.flag_band_gap_fuse() as u8 & 2)
                    | (self.vars.flag_18f_single_panel_access_mode() as u8 & 4)
                    | (self.vars.flag_vcc_vpp_delay() as u8 & 8);

        let (rs_hi, rs_lo) = utils::hi_lo(self.vars.rom_size());
        let (ers_hi, ers_lo) = utils::hi_lo(self.vars.eeprom_size() as u16);

        let mut command_payload = vec![
            rs_hi, rs_lo,
            ers_hi, ers_lo,
            self.vars.core_type(),
            flags,
            self.vars.program_delay(),
            self.vars.power_sequence(),
            self.vars.erase_mode(),
            self.vars.program_tries(),
            self.vars.over_program(),
        ];

//        let mut command_payload = structure!(">HHBBBBBBB").pack(
//            self.vars.rom_size(),
//            self.vars.eeprom_size().into(),
//            self.vars.core_type(),
//            flags,
//            self.vars.program_delay(),
//            self.vars.power_sequence(),
//            self.vars.erase_mode(),
//            self.vars.program_tries(),
//            self.vars.over_program(),
//        );

        self.port.write(command_payload.as_mut_slice());
        let response = self.read(1, 5);
        self.command_end();

        let result = response[0] == b'I';

        if result {
            self.vars_set = true;
        }

        return result
    }

    pub fn set_programming_voltages(&mut self, on: bool) -> bool {
        let cmd_on = 4;
        let cmd_off = 5;
        let expect: u8;
        // self.need_vars()
        if on {
            self.port.write_u8(cmd_on);
            expect = b'V';
        } else {
            self.port.write_u8(cmd_off);
            expect = b'v';
        }
        let response = self.read(1, 5);

        return response[0] == expect
    }

    fn cycle_programming_voltages(&mut self) -> bool {
        let cmd = 6;
        // self.need_vars()
        self.command_start(cmd);
        let response = self.read(1, 5);
        self.command_end();

        return response[0] == b'V'
    }

    pub fn program_rom(&mut self, data: Vec<u8>) -> bool {
        let cmd = 7;
        // self.need_vars();

        let word_count = data.len();
        if self.vars.rom_size() < word_count as u16 {
            panic!("Data too large for PIC ROM");
        }

        if (word_count * 2) % 32 != 0 {
            panic!("ROM data must be a multiple of 32 bytes in size.")
        }

        self.command_start(0);
        self.set_programming_voltages(true);
        self.port.write_u8(cmd);

        let word_count_message = word_count as u8;
        self.port.write_u8(word_count_message);

        self.expect(vec![b'Y'], 5);

        // TODO finish this function
        // TODO find a way to translate the python try-catch to the rust functional way

        for i in (0..(word_count * 2)).step_by(32) {
            self.port.write_all(&data[i..(i + 32)]);
            self.expect(vec![b'Y'], 5);
        }
        self.expect(vec![b'P'], 5);
        // TODO handle try-catch and flushInput

        self.set_programming_voltages(false);
        self.command_end();

        return true

    }

    fn program_eeprom(&mut self, data: Vec<u8>) {}

    fn program_id_fuses(&mut self) {}

    fn program_calibration(&mut self) {}

    pub fn read_rom(&mut self) -> Vec<u8> {
        let cmd = 11;
        // self.need_vars()

        // self.vars.rom_size() is in words. Multiply by 2 to get bytes
        let rom_size = self.vars.rom_size() * 2;

        self.command_start(0);
        self.set_programming_voltages(true);
        self.port.write_u8(cmd);

        let response = self.read(rom_size as usize, 5);

        self.set_programming_voltages(false);
        self.command_end();

        return response
    }

    pub fn read_eeprom(&mut self) -> Vec<u8> {
        let cmd = 12;
        // self.need_vars()

        let eeprom_size = self.vars.eeprom_size();

        self.command_start(0);
        self.set_programming_voltages(true);

        self.port.write_u8(cmd);

        let response = self.read(eeprom_size.into(), 5);

        self.set_programming_voltages(false);
        self.command_end();

        return response
    }

    pub fn read_configuration(&mut self) -> Vec<u8> {
        let cmd = 13;

        self.command_start(0);
        self.set_programming_voltages(true);

        self.port.write_u8(cmd);
        let ack = self.read(1, 5);
        if ack[0] != b'C' {
            panic!("No acknowledgement from read_config()")
        }

        let response = self.read(26, 5);

        self.set_programming_voltages(false);
        self.command_end();

        return response
    }

    fn erase_chip(&mut self) -> bool {
        let cmd = 14;
        // self.need_vars();

        self.command_start(cmd);
        let response = self.read(1, 5);
        self.command_end();

        return response[0] == b'Y'
    }

    pub fn rom_is_blank(&mut self, high_byte: u8) -> bool {
        let cmd = 15;
        // self.need_vars();

        let expected_b_bytes = (self.vars.rom_size() / 256) - 1;
        self.command_start(cmd);
        self.port.write_u8(high_byte);

        loop {
            let response = self.read(1, 5);
            match response[0] {
                b'Y' => {
                    self.command_end();
                    return true
                },
                b'N' | b'C' => {
                    self.command_end();
                    return false
                },
                b'B' => {
                    if expected_b_bytes <= 0 {
                        panic!("Received wrong number of 'B' bytes in rom_is_blank()")
                    }
                    return false
                },
                _ => panic!("Unexpected byte in rom_is_blank(): {:?}", response)
            }
        }
    }

    pub fn eeprom_is_blank(&mut self) -> bool {
        let cmd = 16;
        // self.need_vars();

        self.command_start(cmd);
        let response = self.read(1, 5);
        self.command_end();

        if response[0] != b'Y' && response[0] != b'N' {
            panic!("Unexpected response in eeprom_is_blank(): {:?}", response)
        }

        return response[0] == b'Y'
    }
}