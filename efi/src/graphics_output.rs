use core::mem;
use core::ptr;

use {EfiBox, EfiResult, Guid, PhysicalAddress, Protocol, check_status};
use sys;

pub use sys::{BltPixel, ModeInformation, PixelBitmask, PixelFormat};

pub struct GraphicsOutput(sys::GraphicsOutputProtocol);

impl Protocol for GraphicsOutput {
    const GUID: Guid = sys::GRAPHICS_OUTPUT_PROTOCOL_GUID;
}

impl GraphicsOutput {
    /// Returns the physical address and size of the linear frame buffer.
    pub fn linear_frame_buffer(&self) -> (PhysicalAddress, usize) {
        let mode = self.0.mode;
        unsafe { ((*mode).frame_buffer_base, (*mode).frame_buffer_size) }
    }

    /// Returns the index of the current mode.
    pub fn current_mode(&self) -> ModeNumber {
        unsafe { (*self.0.mode).mode }
    }

    /// Returns information pertaining to the current mode.
    pub fn current_mode_info(&self) -> &ModeInformation {
        unsafe { mem::transmute((*self.0.mode).info) }
    }

    /// Returns the number of modes supported by this device. All mode numbers
    /// are in the range `[0, max_mode)`.
    pub fn max_mode(&self) -> ModeNumber {
        unsafe { (*self.0.mode).max_mode }
    }

    /// Queries information on the specified mode.
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

    /// Sets the mode of this device.
    pub fn set_mode(&self, mode: ModeNumber) -> EfiResult<()> {
        let status = unsafe {
            (self.0.set_mode)(
                &self.0 as *const _ as *mut _,
                mode)
        };
        check_status(status)
    }

    /// Fills the rectangle with a single color.
    pub fn fill(
        &self,
        x: usize, y: usize,
        w: usize, h: usize,
        pixel: BltPixel) -> EfiResult<()>
    {
        let status = unsafe {
            (self.0.blt)(
                &self.0 as *const _ as *mut _,
                &pixel as *const _ as *mut _,
                sys::BltOperation::VideoFill,
                0, 0,
                x, y,
                w, h,
                0)
        };
        check_status(status)
    }

    /// Copies a pixel buffer to the screen.
    pub fn copy_buffer_to_video(
        &self,
        sx: usize, sy: usize,
        x: usize, y: usize,
        w: usize, h: usize,
        buffer: &[BltPixel],
        row_len: usize) -> EfiResult<()> {
        {
            // Bounds check! ^_^
            let row_len = if row_len == 0 { w } else { row_len };
            assert!(buffer.len() >= (sy + h) * row_len);
        }
        let delta = row_len * mem::size_of::<BltPixel>();
        let status = unsafe {
            (self.0.blt)(
                &self.0 as *const _ as *mut _,
                buffer.as_ptr() as *mut _,
                sys::BltOperation::BufferToVideo,
                sx, sy,
                x, y,
                w, h,
                delta)
        };
        check_status(status)
    }
}

pub type ModeNumber = u32;

/*
#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ModeNumber(u32);
*/
