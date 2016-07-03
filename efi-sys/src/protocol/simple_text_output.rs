// https://github.com/tianocore/edk2/blob/master/MdePkg/Include/Protocol/SimpleTextOut.h

use {Guid, Status};


pub const SIMPLE_TEXT_OUTPUT_GUID: Guid = Guid(
    0x387477c2, 0x69c7, 0x11d2,
    [0x8e, 0x39, 0x00, 0xa0, 0xc9, 0x69, 0x72, 0x3b]);

#[allow(missing_debug_implementations)]
#[repr(C)]
pub struct SimpleTextOutputProtocol {
    pub reset: TextReset,
    pub output_string: TextString,
    // ...
}

pub type TextReset = unsafe extern "win64" fn(
    *const SimpleTextOutputProtocol,
    bool,
    ) -> Status;

pub type TextString = unsafe extern "win64" fn(
    *const SimpleTextOutputProtocol,
    *const u16,
    ) -> Status;
