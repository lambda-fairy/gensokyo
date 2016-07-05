use core::mem;
use core::ptr;

use {EfiBox, EfiResult, Guid, PhysicalAddress, Protocol, check_status};
use sys;

pub use sys::{ModeInformation, PixelBitmask, PixelFormat};

pub struct GraphicsOutput(sys::GraphicsOutputProtocol);

impl Protocol for GraphicsOutput {
    const GUID: Guid = sys::GRAPHICS_OUTPUT_PROTOCOL_GUID;
}

impl GraphicsOutput {
    pub fn linear_frame_buffer(&self) -> (PhysicalAddress, usize) {
        let mode = self.0.mode;
        unsafe { ((*mode).frame_buffer_base, (*mode).frame_buffer_size) }
    }

    pub fn current_mode(&self) -> ModeNumber {
        unsafe { (*self.0.mode).mode }
    }

    pub fn current_mode_info(&self) -> &ModeInformation {
        unsafe { mem::transmute((*self.0.mode).info) }
    }

    pub fn max_mode(&self) -> ModeNumber {
        unsafe { (*self.0.mode).max_mode }
    }

    pub fn query_mode(&self, mode: ModeNumber) -> EfiResult<EfiBox<ModeInformation>> {
        unsafe {
            let mut size_of_info: usize = 0;
            let mut info = ptr::null_mut() as *mut ModeInformation;
            let status = (self.0.query_mode)(
                &self.0 as *const _ as *mut _,
                mode,
                &mut size_of_info as *mut _,
                &mut info as *mut _);
            check_status(status)?;
            Ok(EfiBox::from_raw(info))
        }
    }

    pub fn set_mode(&self, mode: ModeNumber) -> EfiResult<()> {
        let status = unsafe {
            (self.0.set_mode)(
                &self.0 as *const _ as *mut _,
                mode)
        };
        check_status(status)
    }
}

pub type ModeNumber = u32;

/*
#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ModeNumber(u32);
*/
