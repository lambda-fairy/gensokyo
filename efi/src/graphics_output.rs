use {EfiResult, Guid, PhysicalAddress, Protocol, check_status};
use sys;

pub struct GraphicsOutput(sys::GraphicsOutputProtocol);

impl Protocol for GraphicsOutput {
    const GUID: Guid = sys::GRAPHICS_OUTPUT_PROTOCOL_GUID;
}

impl GraphicsOutput {
    pub fn linear_frame_buffer(&self) -> (PhysicalAddress, usize) {
        let mode = self.0.mode;
        ((*mode).frame_buffer_base, (*mode).frame_buffer_size)
    }
}
