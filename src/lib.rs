#![no_std]
#![feature(lang_items)]
#![feature(asm)]

extern crate rlibc;
extern crate uefi_sys;

use uefi_sys::*;

#[no_mangle]
pub extern "win64" fn efi_start(
    _image_handle: *const (),
    system_table: *const EFI_SYSTEM_TABLE) -> EFI_STATUS
{
    let hello = "Hello, world!\r\n";
    let mut buffer = [0u16; 64];
    for (c, d) in hello.bytes().zip(buffer.iter_mut()) {
        *d = c as u16;
    }
    unsafe {
        let con_out = (*system_table).ConOut;
        ((*con_out).OutputString)(con_out, buffer.as_ptr());
    }
    loop {}
}

#[no_mangle] pub fn abort() -> ! { loop {} }
#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] extern fn panic_fmt() -> ! { abort() }
