// https://github.com/tianocore/edk2/blob/master/MdePkg/Include/Protocol/SimpleTextOut.h

use Status;

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
