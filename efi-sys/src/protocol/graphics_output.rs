use {Guid, PhysicalAddress, Status};

pub const GRAPHICS_OUTPUT_PROTOCOL_GUID: Guid = Guid(
    0x9042a9de, 0x23dc, 0x4a38,
    [0x96, 0xfb, 0x7a, 0xde, 0xd0, 0x80, 0x51, 0x6a]);

#[allow(missing_debug_implementations)]
#[repr(C)]
pub struct GraphicsOutputProtocol {
    pub query_mode: QueryMode,
    pub set_mode: SetMode,
    pub blt: Blt,
    pub mode: *mut Mode,
}

pub type QueryMode = unsafe extern "win64" fn(
    *mut GraphicsOutputProtocol,
    u32,
    *mut usize,
    *mut *mut ModeInformation,
    ) -> Status;

pub type SetMode = unsafe extern "win64" fn(
    *mut GraphicsOutputProtocol,
    u32,
    ) -> Status;

pub type Blt = unsafe extern "win64" fn(
    *mut GraphicsOutputProtocol,
    *mut BltPixel,
    BltOperation,
    usize,
    usize,
    usize,
    usize,
    usize,
    usize,
    usize,
    ) -> Status;

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[repr(C)]
pub struct PixelBitmask {
    pub red_mask: u32,
    pub green_mask: u32,
    pub blue_mask: u32,
    pub reserved_mask: u32,
}

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[repr(C)]
pub enum PixelFormat {
    RedGreenBlueReserved8BitPerColor,
    BlueGreenRedReserved8BitPerColor,
    BitMask,
    BltOnly,
    // PixelFormatMax
}

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[repr(C)]
pub struct ModeInformation {
    pub version: u32,
    pub horizontal_resolution: u32,
    pub vertical_resolution: u32,
    pub pixel_format: PixelFormat,
    pub pixel_information: PixelBitmask,
    pub pixels_per_scan_line: u32,
}

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[repr(C)]
pub struct Mode {
    pub max_mode: u32,
    pub mode: u32,
    pub info: *mut ModeInformation,
    pub frame_buffer_base: PhysicalAddress,
    pub frame_buffer_size: usize,
}

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[repr(C)]
pub struct BltPixel {
    pub blue: u8,
    pub green: u8,
    pub red: u8,
    pub reserved: u8,
}

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[repr(C)]
pub enum BltOperation {
    VideoFill,
    VideoToBltBuffer,
    BufferToVideo,
    VideoToVideo,
    // GraphicsOutputBltOperationMax
}
