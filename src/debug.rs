/// Routines for writing to the PC serial port. Used primarily for debugging.

use core::fmt;
use spin::Mutex;
use x86::io::{inb, outb};

pub static COM1: Mutex<SerialOutput> = Mutex::new(unsafe { SerialOutput::new(0x3f8) });

#[derive(Debug)]
pub struct SerialOutput { port: u16 }

impl SerialOutput {
    pub const unsafe fn new(port: u16) -> SerialOutput {
        SerialOutput { port: port }
    }

    pub fn write(&mut self, bytes: &[u8]) {
        unsafe {
            for b in bytes {
                // Wait until the serial port is ready
                while inb(self.port + 5) & 0x20 == 0 {
                    // Do nothing
                }
                outb(self.port, *b);
            }
        }
    }
}

impl fmt::Write for SerialOutput {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write(s.as_bytes());
        Ok(())
    }
}
