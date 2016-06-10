#![allow(non_camel_case_types, non_snake_case)]
#![no_std]

//! Low-level UEFI definitions.
//!
//! This crate provides the bare minimum to make Akira work, and so is in no way
//! near complete.

mod protocol;
pub use protocol::*;

pub type STATUS = usize;
pub const SUCCESS: STATUS = 0;

pub enum VOID {}
pub type HANDLE = *const VOID;

//
// https://github.com/tianocore/edk2/blob/master/MdePkg/Include/Base.h
//

#[repr(C)]
pub struct GUID {
    // FIXME: use repr(align = "64") instead
    // https://github.com/rust-lang/rust/issues/33626
    pub _align: [u64; 0],
    pub Data1: u32,
    pub Data2: u16,
    pub Data3: u16,
    pub Data4: [u8; 8],
}

//
// https://github.com/tianocore/edk2/blob/master/MdePkg/Include/Uefi/UefiSpec.h
//

#[repr(C)]
pub struct TABLE_HEADER {
    pub Signature: u64,
    pub Revision: u32,
    pub HeaderSize: u32,
    pub CRC32: u32,
    pub Reserved: u32,
}

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

#[repr(C)]
pub struct BOOT_SERVICES {
    pub Hdr: TABLE_HEADER,

    /*
    pub RaiseTPL: RAISE_TPL,
    pub RestoreTPL: RESTORE_TPL,

    pub AllocatePages: ALLOCATE_PAGES,
    pub FreePages: FREE_PAGES,
    pub GetMemoryMap: GET_MEMORY_MAP,
    pub AllocatePool: ALLOCATE_POOL,
    pub FreePool: FREE_POOL,

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
    VOID *Reserved,
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

#[repr(C)]
pub struct CONFIGURATION_TABLE {
    pub VendorGuid: GUID,
    pub VendorTable: *const VOID,
}

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
