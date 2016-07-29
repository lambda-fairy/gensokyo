#![no_std]
#![feature(try_from)]
#![warn(missing_debug_implementations)]

//! Low-level UEFI definitions.
//!
//! This crate provides the bare minimum to make Gensokyo work, and so is in no
//! way near complete.
//!
//! Comments in the code refer to the UEFI Specification 2.6, available at
//! <http://www.uefi.org/sites/default/files/resources/UEFI%20Spec%202_6.pdf>.

#[macro_use] extern crate bitflags;

use core::convert::TryFrom;
use core::fmt;
use core::mem;

mod protocol;
pub use protocol::*;

//
// 2.3.1 Data Types, p23
//

#[derive(Debug)]
#[repr(C)]
pub enum Void { #[doc(hidden)] _Impossible }

pub type Status = usize;
pub type Handle = *mut Void;

// UEFI treats pages as being 4 KiB in size, regardless of the underlying
// implementation
pub const PAGE_SIZE: usize = 4096;

//
// Appendix A: GUID and Time Formats, p2335
//

// FIXME: add repr(align = "64") here
// https://github.com/rust-lang/rust/issues/33626
#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[repr(C)]
pub struct Guid(
    pub u32,
    pub u16,
    pub u16,
    pub [u8; 8],
    );

//
// Appendix D: Status Codes, p2347
//

pub const MAX_BIT: usize = !(!0usize >> 1);

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[repr(usize)]
pub enum KnownStatus {
    // NOTE: if you change this list, be sure to change the TryFrom impl as well
    Success = 0,
    WarnUnknownGlyph = 1,
    WarnDeleteFailure = 2,
    WarnWriteFailure = 3,
    WarnBufferTooSmall = 4,
    WarnStaleData = 5,
    WarnFileSystem = 6,
    LoadError = MAX_BIT | 1,
    InvalidParameter = MAX_BIT | 2,
    Unsupported = MAX_BIT | 3,
    BadBufferSize = MAX_BIT | 4,
    BufferTooSmall = MAX_BIT | 5,
    NotReady = MAX_BIT | 6,
    DeviceError = MAX_BIT | 7,
    WriteProtected = MAX_BIT | 8,
    OutOfResources = MAX_BIT | 9,
    VolumeCorrupted = MAX_BIT | 10,
    VolumeFull = MAX_BIT | 11,
    NoMedia = MAX_BIT | 12,
    MediaChanged = MAX_BIT | 13,
    NotFound = MAX_BIT | 14,
    AccessDenied = MAX_BIT | 15,
    NoResponse = MAX_BIT | 16,
    NoMapping = MAX_BIT | 17,
    Timeout = MAX_BIT | 18,
    NotStarted = MAX_BIT | 19,
    AlreadyStarted = MAX_BIT | 20,
    Aborted = MAX_BIT | 21,
    IcmpError = MAX_BIT | 22,
    TftpError = MAX_BIT | 23,
    ProtocolError = MAX_BIT | 24,
    IncompatibleVersion = MAX_BIT | 25,
    SecurityViolation = MAX_BIT | 26,
    CrcError = MAX_BIT | 27,
    EndOfMedia = MAX_BIT | 28,
    EndOfFile = MAX_BIT | 31,
    InvalidLanguage = MAX_BIT | 32,
    CompromisedData = MAX_BIT | 33,
    IpAddressConflict = MAX_BIT | 34,
    HttpError = MAX_BIT | 35,
}

impl TryFrom<usize> for KnownStatus {
    type Err = usize;
    fn try_from(status: usize) -> Result<Self, usize> {
        if status <= 6 || ((MAX_BIT | 1) <= status && status <= (MAX_BIT | 35)) {
            Ok(unsafe { mem::transmute(status) })
        } else {
            Err(status)
        }
    }
}

impl Into<usize> for KnownStatus {
    fn into(self) -> usize {
        unsafe { mem::transmute(self) }
    }
}

//
// 4.2 EFI Table Header, p94
//

#[derive(Debug)]
#[repr(C)]
pub struct TableHeader {
    pub signature: u64,
    pub revision: u32,
    pub header_size: u32,
    pub crc32: u32,
    pub reserved: u32,
}

//
// 4.3 EFI System Table, p96
//

#[allow(missing_debug_implementations)]
#[repr(C)]
pub struct SystemTable {
    pub hdr: TableHeader,
    pub firmware_vendor: *mut u16,
    pub firmware_revision: u32,
    pub console_in_handle: Handle,
    pub con_in: *mut SimpleTextInputProtocol,
    pub console_out_handle: Handle,
    pub con_out: *mut SimpleTextOutputProtocol,
    pub standard_error_handle: Handle,
    pub std_err: *mut SimpleTextOutputProtocol,
    pub runtime_services: *mut RuntimeServices,
    pub boot_services: *mut BootServices,
    pub number_of_table_entries: usize,
    pub configuration_table: *mut ConfigurationTable,
}

//
// 4.4 Boot Services Table, p98
//

#[allow(missing_debug_implementations)]
#[repr(C)]
pub struct BootServices {
    pub hdr: TableHeader,

    pub raise_tpl: RaiseTpl,
    pub restore_tpl: RestoreTpl,

    pub allocate_pages: AllocatePages,
    pub free_pages: FreePages,
    pub get_memory_map: GetMemoryMap,
    pub allocate_pool: AllocatePool,
    pub free_pool: FreePool,

    pub create_event: *mut (),
    pub set_timer: *mut (),
    pub wait_for_event: *mut (),
    pub signal_event: *mut (),
    pub close_event: *mut (),
    pub check_event: *mut (),

    pub install_protocol_interface: *mut (),
    pub reinstall_protocol_interface: *mut (),
    pub uninstall_protocol_interface: *mut (),
    pub handle_protocol: *mut (),
    pub reserved: *mut Void,
    pub register_protocol_notify: *mut (),
    pub locate_handle: *mut (),
    pub locate_device_path: *mut (),
    pub install_configuration_table: *mut (),

    pub load_image: *mut (),
    pub start_image: *mut (),
    pub exit: Exit,
    pub unload_image: *mut (),
    pub exit_boot_services: ExitBootServices,

    pub get_next_monotonic_count: GetNextMonotonicCount,
    pub stall: Stall,
    pub set_watchdog_timer: SetWatchdogTimer,

    pub connect_controller: *mut (),
    pub disconnect_controller: *mut (),

    pub open_protocol: OpenProtocol,
    pub close_protocol: CloseProtocol,
    pub open_protocol_information: OpenProtocolInformation,

    pub protocols_per_handle: ProtocolsPerHandle,
    pub locate_handle_buffer: LocateHandleBuffer,
    pub locate_protocol: LocateProtocol,
    pub install_multiple_protocol_interfaces: *mut (),
    pub uninstall_multiple_protocol_interfaces: *mut (),

    pub calculate_crc32: *mut (),

    pub copy_mem: *mut (),
    pub set_mem: *mut (),
    pub create_event_ex: *mut (),
}

//
// 6 Services -- Boot Services, p127
//

pub type RaiseTpl = unsafe extern "win64" fn(Tpl) -> Tpl;
pub type Tpl = usize;
pub type RestoreTpl = unsafe extern "win64" fn(Tpl);

pub type AllocatePages = unsafe extern "win64" fn(
    AllocateType,
    MemoryType,
    usize,
    *mut PhysicalAddress,
    ) -> Status;
pub type FreePages = unsafe extern "win64" fn(
    PhysicalAddress,
    usize,
    ) -> Status;
pub type GetMemoryMap = unsafe extern "win64" fn(
    *mut usize,
    *mut MemoryDescriptor,
    *mut usize,
    *mut usize,
    *mut u32,
    ) -> Status;
pub type AllocatePool = unsafe extern "win64" fn(
    MemoryType,
    usize,
    *mut *mut Void,
    ) -> Status;
pub type FreePool = unsafe extern "win64" fn(*mut Void) -> Status;

pub type Exit = unsafe extern "win64" fn(
    Handle,
    Status,
    usize,
    *mut u16,
    ) -> Status;
pub type ExitBootServices = unsafe extern "win64" fn(
    Handle,
    usize,
    ) -> Status;

pub type GetNextMonotonicCount = unsafe extern "win64" fn(*mut u64) -> Status;
pub type Stall = unsafe extern "win64" fn(usize) -> Status;
pub type SetWatchdogTimer = unsafe extern "win64" fn(
    usize,
    u64,
    usize,
    *mut u16,
    ) -> Status;

pub type OpenProtocol = unsafe extern "win64" fn(
    Handle,
    *mut Guid,
    *mut *mut Void,
    Handle,
    Handle,
    OpenProtocolAttribute,
    ) -> Status;
pub type CloseProtocol = unsafe extern "win64" fn(
    Handle,
    *mut Guid,
    Handle,
    Handle,
    ) -> Status;
pub type OpenProtocolInformation = unsafe extern "win64" fn(
    Handle,
    *mut Guid,
    *mut *mut OpenProtocolInformationEntry,
    *mut usize,
    ) -> Status;

pub type ProtocolsPerHandle = unsafe extern "win64" fn(
    Handle,
    *mut *mut *mut Guid,
    *mut usize,
    ) -> Status;
pub type LocateHandleBuffer = unsafe extern "win64" fn(
    LocateSearchType,
    *mut Guid,
    *mut Void,
    *mut usize,
    *mut *mut Handle,
    ) -> Status;
pub type LocateProtocol = unsafe extern "win64" fn(
    *mut Guid,
    *mut Void,
    *mut *mut Void,
    ) -> Status;

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[repr(u32)]
pub enum AllocateType {
    AllocateAnyPages,
    AllocateMaxAddress,
    AllocateAddress,
    MaxAllocateType,
}

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[repr(u32)]
pub enum MemoryType {
    ReservedMemoryType,
    LoaderCode,
    LoaderData,
    BootServicesCode,
    BootServicesData,
    RuntimeServicesCode,
    RuntimeServicesData,
    ConventionalMemory,
    UnusableMemory,
    ACPIReclaimMemory,
    ACPIMemoryNVS,
    MemoryMappedIO,
    MemoryMappedIOPortSpace,
    PalCode,
    PersistentMemory,
    MaxMemoryType,
}

/// Represents a UEFI memory descriptor.
#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[repr(C)]
pub struct MemoryDescriptor {
    pub type_: MemoryType,  // = UINT32
    pub physical_start: PhysicalAddress,
    pub virtual_start: VirtualAddress,
    pub number_of_pages: u64,
    pub attribute: MemoryAttribute,
}

pub const MEMORY_DESCRIPTOR_VERSION: u32 = 1;

impl MemoryDescriptor {
    pub fn physical_end(&self) -> PhysicalAddress {
        let start = self.physical_start.0;
        PhysicalAddress(start + PAGE_SIZE as u64 * self.number_of_pages)
    }
}

#[derive(Copy, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct PhysicalAddress(pub u64);

impl fmt::Debug for PhysicalAddress {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "0x{:016x}", self.0)
    }
}

#[derive(Copy, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct VirtualAddress(pub u64);

impl fmt::Debug for VirtualAddress {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "0x{:016x}", self.0)
    }
}

bitflags! { pub flags MemoryAttribute: u64 {
    const MEMORY_UC = 0x0000000000000001,
    const MEMORY_WC = 0x0000000000000002,
    const MEMORY_WT = 0x0000000000000004,
    const MEMORY_WB = 0x0000000000000008,
    const MEMORY_UCE = 0x0000000000000010,
    const MEMORY_WP = 0x0000000000001000,
    const MEMORY_RP = 0x0000000000002000,
    const MEMORY_XP = 0x0000000000004000,
    const MEMORY_NV = 0x0000000000008000,
    const MEMORY_MORE_RELIABLE = 0x0000000000010000,
    const MEMORY_RO = 0x0000000000020000,
    const MEMORY_RUNTIME = 0x8000000000000000,
}}

bitflags! { pub flags OpenProtocolAttribute: u32 {
    const BY_HANDLE_PROTOCOL = 0x00000001,
    const GET_PROTOCOL = 0x00000002,
    const TEST_PROTOCOL = 0x00000004,
    const BY_CHILD_CONTROLLER = 0x00000008,
    const BY_DRIVER = 0x00000010,
    const EXCLUSIVE = 0x00000020,
}}

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[repr(C)]
pub struct OpenProtocolInformationEntry {
    pub agent_handle: Handle,
    pub controller_handle: Handle,
    pub attributes: u32,  // TODO: Should this be OpenProtocolAttribute?
    pub open_count: u32,
}

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[repr(C, u32)]
pub enum LocateSearchType {
    AllHandles,
    ByRegisterNotify,
    ByProtocol,
}

//
// 4.5 EFI Runtime Services Table, p102
//

#[allow(missing_debug_implementations)]
#[repr(C)]
pub struct RuntimeServices {
    pub hdr: TableHeader,

    pub get_time: GetTime,
    pub set_time: SetTime,
    pub get_wakeup_time: GetWakeupTime,
    pub set_wakeup_time: SetWakeupTime,

    pub set_virtual_address_map: SetVirtualAddressMap,
    pub convert_pointer: ConvertPointer,

    pub get_variable: *mut (),
    pub get_next_variable_name: *mut (),
    pub set_variable: *mut (),

    pub get_next_high_monotonic_count: *mut (),
    pub reset_system: ResetSystem,

    pub update_capsule: *mut (),
    pub query_capsule_capabilities: *mut (),

    pub query_variable_info: *mut (),
}

//
// 7 Services -- Runtime Services, p233
//

pub type GetTime = unsafe extern "win64" fn(*mut Time, *mut TimeCapabilities) -> Status;
pub type SetTime = unsafe extern "win64" fn(*mut Time) -> Status;
pub type GetWakeupTime = unsafe extern "win64" fn(
    *mut bool,
    *mut bool,
    *mut Time,
    ) -> Status;
pub type SetWakeupTime = unsafe extern "win64" fn(bool, *mut Time) -> Status;

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[repr(C)]
pub struct Time {
    pub year: u16,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
    pub pad1: u8,
    pub nanosecond: u32,
    pub time_zone: i16,
    pub daylight: Daylight,
    pub pad2: u8,
}

bitflags! { pub flags Daylight: u8 {
    const ADJUST_DAYLIGHT = 0x01,
    const IN_DAYLIGHT = 0x02,
}}

pub const UNSPECIFIED_TIMEZONE: i16 = 0x07ff;

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[repr(C)]
pub struct TimeCapabilities {
    resolution: u32,
    accuracy: u32,
    sets_to_zero: bool,
}

pub type SetVirtualAddressMap = unsafe extern "win64" fn(
    usize,
    usize,
    u32,
    *mut MemoryDescriptor,
    ) -> Status;
pub type ConvertPointer = unsafe extern "win64" fn(
    DebugDisposition,
    *mut *mut Void,
    ) -> Status;

bitflags! { pub flags DebugDisposition: usize {
    const OPTIONAL_PTR = 0x00000001,
}}

pub type ResetSystem = unsafe extern "win64" fn(
    ResetType,
    Status,
    usize,
    *mut Void,
    ) -> !;

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[repr(C, u32)]
pub enum ResetType {
    Cold,
    Warm,
    Shutdown,
    PlatformSpecific,
}

//
// 4.6 EFI Configuration Table & Properties Table, p104
//

#[derive(Debug)]
#[repr(C)]
pub struct ConfigurationTable {
    pub vendor_guid: Guid,
    pub vendor_table: *mut Void,
}
