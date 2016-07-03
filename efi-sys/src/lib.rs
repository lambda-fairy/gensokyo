#![no_std]
#![warn(missing_debug_implementations)]

//! Low-level UEFI definitions.
//!
//! This crate provides the bare minimum to make Akira work, and so is in no way
//! near complete.
//!
//! Comments in the code refer to the UEFI Specification 2.6, available at
//! <http://www.uefi.org/sites/default/files/resources/UEFI%20Spec%202_6.pdf>.

#[macro_use] extern crate bitflags;

use core::fmt;

mod protocol;
pub use protocol::*;

//
// 2.3.1 Data Types, p23
//

#[derive(Debug)]
#[repr(C)]
pub enum Void { #[doc(hidden)] _Impossible }

pub type Status = usize;
pub type Handle = *const Void;

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
// Appendix C: Status Codes, p2347
//

pub const SUCCESS: Status = 0;
pub const MAX_BIT: usize = !(!0usize >> 1);

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
    pub firmware_vendor: *const u16,
    pub firmware_revision: u32,
    pub console_in_handle: Handle,
    pub con_in: *const SimpleTextInputProtocol,
    pub console_out_handle: Handle,
    pub con_out: *const SimpleTextOutputProtocol,
    pub standard_error_handle: Handle,
    pub std_err: *const SimpleTextOutputProtocol,
    pub runtime_services: *const RuntimeServices,
    pub boot_services: *const BootServices,
    pub number_of_table_entries: usize,
    pub configuration_table: *const ConfigurationTable,
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

    pub create_event: *const (),
    pub set_timer: *const (),
    pub wait_for_event: *const (),
    pub signal_event: *const (),
    pub close_event: *const (),
    pub check_event: *const (),

    pub install_protocol_interface: *const (),
    pub reinstall_protocol_interface: *const (),
    pub uninstall_protocol_interface: *const (),
    pub handle_protocol: *const (),
    pub reserved: *const Void,
    pub register_protocol_notify: *const (),
    pub locate_handle: *const (),
    pub locate_device_path: *const (),
    pub install_configuration_table: *const (),

    pub load_image: *const (),
    pub start_image: *const (),
    pub exit: Exit,
    pub unload_image: *const (),
    pub exit_boot_services: ExitBootServices,

    pub get_next_monotonic_count: GetNextMonotonicCount,
    pub stall: Stall,
    pub set_watchdog_timer: SetWatchdogTimer,

    pub connect_controller: *const (),
    pub disconnect_controller: *const (),

    pub open_protocol: OpenProtocol,
    pub close_protocol: CloseProtocol,
    pub open_protocol_information: OpenProtocolInformation,

    pub protocols_per_handle: ProtocolsPerHandle,
    pub locate_handle_buffer: LocateHandleBuffer,
    pub locate_protocol: LocateProtocol,
    pub install_multiple_protocol_interfaces: *const (),
    pub uninstall_multiple_protocol_interfaces: *const (),

    pub calculate_crc32: *const (),

    pub copy_mem: *const (),
    pub set_mem: *const (),
    pub create_event_ex: *const (),
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
    *const u16,
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
    *const u16,
    ) -> Status;

pub type OpenProtocol = unsafe extern "win64" fn(
    Handle,
    *const Guid,
    *mut *const Void,
    Handle,
    Handle,
    OpenProtocolAttribute,
    ) -> Status;
pub type CloseProtocol = unsafe extern "win64" fn(
    Handle,
    *const Guid,
    Handle,
    Handle,
    ) -> Status;
pub type OpenProtocolInformation = unsafe extern "win64" fn(
    Handle,
    *const Guid,
    *mut *const OpenProtocolInformationEntry,
    *mut usize,
    ) -> Status;

pub type ProtocolsPerHandle = unsafe extern "win64" fn(
    Handle,
    *mut *const *const Guid,
    *mut usize,
    ) -> Status;
pub type LocateHandleBuffer = unsafe extern "win64" fn(
    LocateSearchType,
    *const Guid,
    *const Void,
    *mut usize,
    *mut *const Handle,
    ) -> Status;
pub type LocateProtocol = unsafe extern "win64" fn(
    *const Guid,
    *const Void,
    *mut *const Void,
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

    pub get_variable: *const (),
    pub get_next_variable_name: *const (),
    pub set_variable: *const (),

    pub get_next_high_monotonic_count: *const (),
    pub reset_system: ResetSystem,

    pub update_capsule: *const (),
    pub query_capsule_capabilities: *const (),

    pub query_variable_info: *const (),
}

//
// 7 Services -- Runtime Services, p233
//

pub type GetTime = unsafe extern "win64" fn(*mut Time, *mut TimeCapabilities) -> Status;
pub type SetTime = unsafe extern "win64" fn(*const Time) -> Status;
pub type GetWakeupTime = unsafe extern "win64" fn(
    *mut bool,
    *mut bool,
    *mut Time,
    ) -> Status;
pub type SetWakeupTime = unsafe extern "win64" fn(bool, *const Time) -> Status;

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
    *const MemoryDescriptor,
    ) -> Status;
pub type ConvertPointer = unsafe extern "win64" fn(
    DebugDisposition,
    *mut *const Void,
    ) -> Status;

bitflags! { pub flags DebugDisposition: usize {
    const OPTIONAL_PTR = 0x00000001,
}}

pub type ResetSystem = unsafe extern "win64" fn(
    ResetType,
    Status,
    usize,
    *const Void,
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
    pub vendor_table: *const Void,
}
