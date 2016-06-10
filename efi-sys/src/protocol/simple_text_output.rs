// https://github.com/tianocore/edk2/blob/master/MdePkg/Include/Protocol/SimpleTextOut.h

use STATUS;

#[repr(C)]
pub struct SIMPLE_TEXT_OUTPUT_PROTOCOL {
    pub Reset: TEXT_RESET,
    pub OutputString: TEXT_STRING,
    // ...
}

pub type TEXT_RESET = unsafe extern "win64" fn(
    *const SIMPLE_TEXT_OUTPUT_PROTOCOL,
    bool,
    ) -> STATUS;

pub type TEXT_STRING = unsafe extern "win64" fn(
    *const SIMPLE_TEXT_OUTPUT_PROTOCOL,
    *const u16,
    ) -> STATUS;
