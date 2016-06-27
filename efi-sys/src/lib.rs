#![no_std]
#![warn(missing_debug_implementations)]

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

#[derive(Debug)]
#[repr(C)]
pub enum Void { #[doc(hidden)] _Impossible }

pub type Status = usize;
pub type Handle = *const Void;

//
// Appendix A: GUID and Time Formats, p2335
//

#[derive(Debug)]
#[repr(C)]
pub struct Guid {
    // FIXME: use repr(align = "64") instead
    // https://github.com/rust-lang/rust/issues/33626
    pub _align: [u64; 0],
    pub time_low: u32,
    pub time_mid: u16,
    pub time_high_and_version: u16,
    pub clock_seq_high_and_reserved: u8,
    pub clock_seq_low: u8,
    pub node: [u8; 6],
}

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

    /*
    pub create_event: CreateEvent,
    pub set_timer: SetTimer,
    pub wait_for_event: WaitForEvent,
    pub signal_event: SignalEvent,
    pub close_event: CloseEvent,
    pub check_event: CheckEvent,

    pub install_protocol_interface: InstallProtocolInterface,
    pub reinstall_protocol_interface: ReinstallProtocolInterface,
    pub uninstall_protocol_interface: UninstallProtocolInterface,
    pub handle_protocol: HandleProtocol,
    pub reserved: *const Void,
    pub register_protocol_notify: RegisterProtocolNotify,
    pub locate_handle: LocateHandle,
    pub locate_device_path: LocateDevicePath,
    pub install_configuration_table: InstallConfigurationTable,

    pub load_image: ImageLoad,
    pub start_image: ImageStart,
    pub exit: Exit,
    pub unload_image: ImageUnload,
    pub exit_boot_services: ExitBootServices,
    */

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
    pub exit: *const (),
    pub unload_image: *const (),
    pub exit_boot_services: ExitBootServices,

    /*
    pub get_next_monotonic_count: GetNextMonotonicCount,
    pub stall: Stall,
    pub set_watchdog_timer: SetWatchdogTimer,

    pub connect_controller: ConnectController,
    pub disconnect_controller: DisconnectController,

    pub open_protocol: OpenProtocol,
    pub close_protocol: CloseProtocol,
    pub open_protocol_information: OpenProtocolInformation,

    pub protocols_per_handle: ProtocolsPerHandle,
    pub locate_handle_buffer: LocateHandleBuffer,
    pub locate_protocol: LocateProtocol,
    pub install_multiple_protocol_interfaces: InstallMultipleProtocolInterfaces,
    pub uninstall_multiple_protocol_interfaces: UninstallMultipleProtocolInterfaces,

    pub calculate_crc32: CalculateCrc32,

    pub copy_mem: CopyMem,
    pub set_mem: SetMem,
    pub create_event_ex: CreateEventEx,
    */
}

pub type RaiseTpl = extern "win64" fn(Tpl) -> Tpl;
pub type Tpl = usize;
pub type RestoreTpl = extern "win64" fn(Tpl);

pub type AllocatePages = extern "win64" fn(
    AllocateType,
    MemoryType,
    usize,
    *mut PhysicalAddress,
    ) -> Status;
#[derive(Debug)]
#[repr(C, u32)]
pub enum AllocateType {
    AllocateAnyPages,
    AllocateMaxAddress,
    AllocateAddress,
    MaxAllocateType,
}
#[derive(Debug)]
#[repr(C, u32)]
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
pub type PhysicalAddress = u64;
pub type FreePages = extern "win64" fn(
    PhysicalAddress,
    usize,
    ) -> Status;
pub type GetMemoryMap = extern "win64" fn(
    *mut usize,
    *mut MemoryDescriptor,
    *mut usize,
    *mut usize,
    *mut u32,
    ) -> Status;
#[derive(Debug)]
#[repr(C)]
pub struct MemoryDescriptor {
    pub type_: MemoryType,  // = UINT32
    pub physical_start: PhysicalAddress,
    pub virtual_start: VirtualAddress,
    pub number_of_pages: u64,
    pub attribute: u64,
}
pub type VirtualAddress = u64;
pub const MEMORY_DESCRIPTOR_VERSION: u32 = 1;

pub type AllocatePool = extern "win64" fn(
    MemoryType,
    usize,
    *mut *mut Void,
    ) -> Status;
pub type FreePool = extern "win64" fn(*mut Void) -> Status;

pub type ExitBootServices = extern "win64" fn(
    Handle,
    usize,
    );

//
// 4.5 EFI Runtime Services Table, p102
//

#[allow(missing_debug_implementations)]
#[repr(C)]
pub struct RuntimeServices {
    pub hdr: TableHeader,

    /*
    pub get_time: GetTime,
    pub set_time: SetTime,
    pub get_wakeup_time: GetWakeupTime,
    pub set_wakeup_time: SetWakeupTime,

    pub set_virtual_address_map: SetVirtualAddressMap,
    pub convert_pointer: ConvertPointer,

    pub get_variable: GetVariable,
    pub get_next_variable_name: GetNextVariableName,
    pub set_variable: SetVariable,

    pub get_next_high_monotonic_count: GetNextHighMonoCount,
    pub reset_system: ResetSystem,

    pub update_capsule: UpdateCapsule,
    pub query_capsule_capabilities: QueryCapsuleCapabilities,

    pub query_variable_info: QueryVariableInfo,
    */
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
