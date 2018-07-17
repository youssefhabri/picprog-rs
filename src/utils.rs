use std::thread;
use std::time::Duration;
use std::ops::Shr;

pub fn sleep_ms(milliseconds: u64) {
    thread::sleep(Duration::from_millis(milliseconds))
}

pub fn hi_lo(number: u16) -> (u8, u8) {
    (
        (number & 0xff00).shr(8) as u8,
        (number & 0x00ff).shr(0) as u8
    )
}