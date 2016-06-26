#![allow(non_camel_case_types, non_snake_case)]
#![no_std]

//! Low-level UEFI definitions.
//!
//! This crate provides the bare minimum to make Akira work, and so is in no way
//! near complete.
//!
//! Comments in the code refer to the UEFI Specification 2.6, available at
//! <http://www.uefi.org/sites/default/files/resources/UEFI%20Spec%202_6.pdf>.

mod protocol;
pub use protocol::*;

//
// 2.3.1 Data Types, p23
//

pub type VOID = ();
pub type STATUS = usize;
pub type HANDLE = *const VOID;

//
// Appendix A: GUID and Time Formats, p2335
//

#[repr(C)]
pub struct GUID {
    // FIXME: use repr(align = "64") instead
    // https://github.com/rust-lang/rust/issues/33626
    pub _Align: [u64; 0],
    pub TimeLow: u32,
    pub TimeMid: u16,
    pub TimeHighAndVersion: u16,
    pub ClockSeqHighAndReserved: u8,
    pub ClockSeqLow: u8,
    pub Node: [u8; 6],
}

//
// Appendix C: Status Codes, p2347
//

pub const SUCCESS: STATUS = 0;
pub const MAX_BIT: usize = !(!0usize >> 1);

//
// 4.2 EFI Table Header, p94
//

#[repr(C)]
pub struct TABLE_HEADER {
    pub Signature: u64,
    pub Revision: u32,
    pub HeaderSize: u32,
    pub CRC32: u32,
    pub Reserved: u32,
}

//
// 4.3 EFI System Table, p96
//

#[repr(C)]
pub struct SYSTEM_TABLE {
    pub Hdr: TABLE_HEADER,
    pub FirmwareVendor: *const u16,
    pub FirmwareRevision: u32,
    pub ConsoleInHandle: HANDLE,
    pub ConIn: *const SIMPLE_TEXT_INPUT_PROTOCOL,
    pub ConsoleOutHandle: HANDLE,
    pub ConOut: *const SIMPLE_TEXT_OUTPUT_PROTOCOL,
    pub StandardErrorHandle: HANDLE,
    pub StdErr: *const SIMPLE_TEXT_OUTPUT_PROTOCOL,
    pub RuntimeServices: *const RUNTIME_SERVICES,
    pub BootServices: *const BOOT_SERVICES,
    pub NumberOfTableEntries: usize,
    pub ConfigurationTable: *const CONFIGURATION_TABLE,
}

//
// 4.4 Boot Services Table, p98
//

#[repr(C)]
pub struct BOOT_SERVICES {
    pub Hdr: TABLE_HEADER,

    pub RaiseTPL: RAISE_TPL,
    pub RestoreTPL: RESTORE_TPL,

    pub AllocatePages: ALLOCATE_PAGES,
    pub FreePages: FREE_PAGES,
    pub GetMemoryMap: GET_MEMORY_MAP,
    pub AllocatePool: ALLOCATE_POOL,
    pub FreePool: FREE_POOL,

    /*
    pub CreateEvent: CREATE_EVENT,
    pub SetTimer: SET_TIMER,
    pub WaitForEvent: WAIT_FOR_EVENT,
    pub SignalEvent: SIGNAL_EVENT,
    pub CloseEvent: CLOSE_EVENT,
    pub CheckEvent: CHECK_EVENT,

    pub InstallProtocolInterface: INSTALL_PROTOCOL_INTERFACE,
    pub ReinstallProtocolInterface: REINSTALL_PROTOCOL_INTERFACE,
    pub UninstallProtocolInterface: UNINSTALL_PROTOCOL_INTERFACE,
    pub HandleProtocol: HANDLE_PROTOCOL,
    pub Reserved: *const VOID,
    pub RegisterProtocolNotify: REGISTER_PROTOCOL_NOTIFY,
    pub LocateHandle: LOCATE_HANDLE,
    pub LocateDevicePath: LOCATE_DEVICE_PATH,
    pub InstallConfigurationTable: INSTALL_CONFIGURATION_TABLE,

    pub LoadImage: IMAGE_LOAD,
    pub StartImage: IMAGE_START,
    pub Exit: EXIT,
    pub UnloadImage: IMAGE_UNLOAD,
    pub ExitBootServices: EXIT_BOOT_SERVICES,

    pub GetNextMonotonicCount: GET_NEXT_MONOTONIC_COUNT,
    pub Stall: STALL,
    pub SetWatchdogTimer: SET_WATCHDOG_TIMER,

    pub ConnectController: CONNECT_CONTROLLER,
    pub DisconnectController: DISCONNECT_CONTROLLER,

    pub OpenProtocol: OPEN_PROTOCOL,
    pub CloseProtocol: CLOSE_PROTOCOL,
    pub OpenProtocolInformation: OPEN_PROTOCOL_INFORMATION,

    pub ProtocolsPerHandle: PROTOCOLS_PER_HANDLE,
    pub LocateHandleBuffer: LOCATE_HANDLE_BUFFER,
    pub LocateProtocol: LOCATE_PROTOCOL,
    pub InstallMultipleProtocolInterfaces: INSTALL_MULTIPLE_PROTOCOL_INTERFACES,
    pub UninstallMultipleProtocolInterfaces: UNINSTALL_MULTIPLE_PROTOCOL_INTERFACES,

    pub CalculateCrc32: CALCULATE_CRC32,

    pub CopyMem: COPY_MEM,
    pub SetMem: SET_MEM,
    pub CreateEventEx: CREATE_EVENT_EX,
    */
}

pub type RAISE_TPL = extern "win64" fn(TPL) -> TPL;
pub type TPL = usize;
pub type RESTORE_TPL = extern "win64" fn(TPL);

pub type ALLOCATE_PAGES = extern "win64" fn(
    ALLOCATE_TYPE,
    MEMORY_TYPE,
    usize,
    *mut PHYSICAL_ADDRESS,
    ) -> STATUS;
#[repr(C, u32)]
pub enum ALLOCATE_TYPE {
    AllocateAnyPages,
    AllocateMaxAddress,
    AllocateAddress,
    MaxAllocateType,
}
#[repr(C, u32)]
pub enum MEMORY_TYPE {
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
pub type PHYSICAL_ADDRESS = u64;
pub type FREE_PAGES = extern "win64" fn(
    PHYSICAL_ADDRESS,
    usize,
    ) -> STATUS;
pub type GET_MEMORY_MAP = extern "win64" fn(
    *mut usize,
    *mut MEMORY_DESCRIPTOR,
    *mut usize,
    *mut usize,
    *mut u32,
    ) -> STATUS;
#[repr(C)]
pub struct MEMORY_DESCRIPTOR {
    pub Type: u32,
    pub PhysicalStart: PHYSICAL_ADDRESS,
    pub VirtualStart: VIRTUAL_ADDRESS,
    pub NumberOfPages: u64,
    pub Attribute: u64,
}
pub type VIRTUAL_ADDRESS = u64;
pub const MEMORY_DESCRIPTOR_VERSION: u32 = 1;

pub type ALLOCATE_POOL = extern "win64" fn(
    MEMORY_TYPE,
    usize,
    *mut *mut VOID,
    ) -> STATUS;
pub type FREE_POOL = extern "win64" fn(*mut VOID) -> STATUS;

//
// 4.5 EFI Runtime Services Table, p102
//

#[repr(C)]
pub struct RUNTIME_SERVICES {
    pub Hdr: TABLE_HEADER,

    /*
    pub GetTime: GET_TIME,
    pub SetTime: SET_TIME,
    pub GetWakeupTime: GET_WAKEUP_TIME,
    pub SetWakeupTime: SET_WAKEUP_TIME,

    pub SetVirtualAddressMap: SET_VIRTUAL_ADDRESS_MAP,
    pub ConvertPointer: CONVERT_POINTER,

    pub GetVariable: GET_VARIABLE,
    pub GetNextVariableName: GET_NEXT_VARIABLE_NAME,
    pub SetVariable: SET_VARIABLE,

    pub GetNextHighMonotonicCount: GET_NEXT_HIGH_MONO_COUNT,
    pub ResetSystem: RESET_SYSTEM,

    pub UpdateCapsule: UPDATE_CAPSULE,
    pub QueryCapsuleCapabilities: QUERY_CAPSULE_CAPABILITIES,

    pub QueryVariableInfo: QUERY_VARIABLE_INFO,
    */
}

//
// 4.6 EFI Configuration Table & Properties Table, p104
//

#[repr(C)]
pub struct CONFIGURATION_TABLE {
    pub VendorGuid: GUID,
    pub VendorTable: *const VOID,
}
