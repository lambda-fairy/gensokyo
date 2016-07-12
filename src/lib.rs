#![no_std]
#![feature(lang_items)]
#![feature(asm)]

extern crate efi;
extern crate rlibc;

use efi::{sys, BootServices};

#[no_mangle]
pub extern "win64" fn efi_start(
    image_handle: sys::Handle,
    system_table: *mut sys::SystemTable) -> sys::Status
{
    let (bs, _rs) = unsafe { efi::init(image_handle, system_table) };
    let (memory_map, _map_key) = bs.memory_map();
    let stdout = bs.stdout();
    for desc in &memory_map {
        write!(stdout, "{:?}\r\n", desc).unwrap();
    }
    abort();
}

#[no_mangle]
pub fn abort() -> ! {
    loop {
        unsafe { asm!("hlt" :::: "volatile"); }
    }
}

#[lang = "panic_fmt"]
extern fn panic_fmt(args: core::fmt::Arguments, file: &str, line: u32) -> ! {
    unsafe {
        if let Some(bs) = BootServices::get_instance() {
            let _ = write!(bs.stdout(), "\r
\r
===================== PANIC ======================\r
{args}\r
    at {file}:{line}\r
==================================================\r
\r
", args = args, file = file, line = line);
        }
    }
    abort();
}
