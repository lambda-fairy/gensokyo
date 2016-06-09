#![no_std]
#![allow(non_camel_case_types, non_snake_case)]

pub type EFI_STATUS = usize;

#[repr(C)]
pub struct EFI_SYSTEM_TABLE {
    pub Hdr: EFI_TABLE_HEADER,
    pub FirmwareVendor: *const u16,
    pub FirmwareRevision: u32,
    pub ConsoleInHandle: *const (),
    pub ConIn: *const EFI_SIMPLE_TEXT_INPUT_PROTOCOL,
    pub ConsoleOutHandle: *const (),
    pub ConOut: *const EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL,
    // ...
}

#[repr(C)]
pub struct EFI_TABLE_HEADER {
    pub Signature: u64,
    pub Revision: u32,
    pub HeaderSize: u32,
    pub CRC32: u32,
    pub Reserved: u32,
}

#[repr(C)]
pub struct EFI_SIMPLE_TEXT_INPUT_PROTOCOL {
    // ...
}

#[repr(C)]
pub struct EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL {
    pub Reset: EFI_TEXT_RESET,
    pub OutputString: EFI_TEXT_STRING,
    // ...
}

pub type EFI_TEXT_RESET = unsafe extern "win64" fn(
    *const EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL,
    bool,
    ) -> EFI_STATUS;

pub type EFI_TEXT_STRING = unsafe extern "win64" fn(
    *const EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL,
    *const u16,
    ) -> EFI_STATUS;
