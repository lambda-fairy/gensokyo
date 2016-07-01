#![no_std]
#![feature(lang_items)]
#![feature(asm)]

extern crate efi;
extern crate rlibc;

use efi::{sys, BootServices};

#[no_mangle]
pub extern "win64" fn efi_start(
    image_handle: sys::Handle,
    system_table: *const sys::SystemTable) -> sys::Status
{
    let (bs, _rs) = unsafe { efi::init(image_handle, system_table) };
    let (memory_map, _map_key) = bs.memory_map();
    for desc in &memory_map {
        write!(bs.stdout(), "{:?}\r\n", desc).unwrap();
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
    let _ = BootServices::with_instance(|bs| {
        write!(bs.stderr(), "\r
\r
===================== PANIC ======================\r
{args}\r
    at {file}:{line}\r
==================================================\r
\r
", args = args, file = file, line = line)
    });
    abort();
}
