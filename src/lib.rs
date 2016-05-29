#![no_std]
#![feature(lang_items)]
#![feature(asm)]

extern crate rlibc;

#[repr(C)]
pub struct SystemTable {
    hdr: TableHeader,
    firmware_vendor: *const u16,
    firmware_revision: u32,
    console_in_handle: *const (),
    con_in: *const SimpleTextInputProtocol,
    console_out_handle: *const (),
    con_out: *const SimpleTextOutputProtocol,
    // ...
}

#[repr(C)]
struct TableHeader {
    signature: u64,
    revision: u32,
    header_size: u32,
    crc32: u32,
    reserved: u32,
}

#[repr(C)]
pub struct SimpleTextInputProtocol {
    // ...
}

#[repr(C)]
pub struct SimpleTextOutputProtocol {
    reset: *const (),
    output_string: unsafe extern "win64" fn(*const SimpleTextOutputProtocol, *const u16),
    // ...
}

#[no_mangle]
pub extern "win64" fn efi_start(_image_handle: *const (), system_table: *const SystemTable) -> isize {
    let hello = "Hello, world!\r\n";
    let mut buffer = [0u16; 64];
    for (c, d) in hello.bytes().zip(buffer.iter_mut()) {
        *d = c as u16;
    }
    unsafe {
        let con_out = (*system_table).con_out;
        ((*con_out).output_string)(con_out, buffer.as_ptr());
    }
    0
}

#[no_mangle] pub fn abort() -> ! { loop {} }
#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] extern fn panic_fmt() -> ! { abort() }
