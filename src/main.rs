extern crate serialport;
extern crate byteorder;
#[macro_use]
extern crate structure;

mod utils;
mod protocol;
mod chipinfo;

use std::time::{Duration};
use serialport::*;

use protocol::ProtocolInterface;

fn main() {
    let port_settings = SerialPortSettings {
        baud_rate: 19200,
        data_bits: DataBits::Eight,
        flow_control: FlowControl::None,
        parity: Parity::None,
        stop_bits: StopBits::One,
        timeout: Duration::from_millis(300),
    };

    let port_name = "/dev/ttyUSB0";

    let port = match serialport::open_with_settings(port_name, &port_settings) {
        Ok(port) => port,
        Err(err) => panic!("Unable to open serial port {}: {}.", port_name, err.description)
    };

    println!("Receiving data on {} at {} baud:", &port_name, 19200);

    let mut pi = ProtocolInterface::new(port);
    let _ = pi.reset();
    pi.init_programming_vars();

//    let rom = pi.read_rom();
//    println!("ROM: {:?}", rom);
//
//    let eeprom = pi.read_eeprom();
//    println!("EEPROM: {:?}", eeprom);
//
//    let config = pi.read_config();
//    println!("CONFIG: {:?}", config);

    println!("BLANK ROM: {:?}", pi.rom_is_blank(0xFF));
    println!("BLANK EEPROM: {:?}", pi.eeprom_is_blank());
}