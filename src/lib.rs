#![no_std]
#![feature(lang_items)]
#![feature(asm)]

extern crate efi;
extern crate rlibc;

use efi::{sys, Efi};

#[no_mangle]
pub extern "win64" fn efi_start(
    image_handle: sys::HANDLE,
    system_table: *const sys::SYSTEM_TABLE) -> sys::STATUS
{
    let efi = unsafe { Efi::new(image_handle, system_table) };
    let out = efi.stdout();
    write!(out, "Hello, world!\r\n").unwrap();
    loop {}
}

#[no_mangle]
pub fn abort() -> ! {
    loop {
        unsafe { asm!("hlt" :::: "volatile"); }
    }
}

#[lang = "eh_personality"] extern fn eh_personality() {}

#[lang = "panic_fmt"]
extern fn panic_fmt(args: core::fmt::Arguments, file: &str, line: u32) -> ! {
    let _ = Efi::with_instance(|efi| {
        write!(efi.stdout(), "\r
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
