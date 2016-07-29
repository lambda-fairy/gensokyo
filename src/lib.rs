#![no_std]
#![feature(asm)]
#![feature(const_fn)]
#![feature(lang_items)]

extern crate efi;
extern crate rlibc;
extern crate spin;

use core::mem;
use efi::{sys, BootServices, GraphicsOutput, MapKey, MemoryMap, MemoryType};
use spin::{Once, RwLock};

mod bitmap;
mod bitmap_alloc;

use bitmap_alloc::BitmapAllocator;

static PHYSICAL_ALLOC: Once<RwLock<BitmapAllocator>> = Once::new();

macro_rules! efi_log {
    ($out:expr, $fmt:expr) => (write!($out, concat!($fmt, "\r\n")).unwrap());
    ($out:expr, $fmt:expr, $($arg:tt)*) => (write!($out, concat!($fmt, "\r\n"), $($arg)*).unwrap());
}

#[no_mangle]
pub extern "win64" fn efi_start(
    image_handle: sys::Handle,
    system_table: *mut sys::SystemTable) -> sys::Status
{
    let (bs, _rs) = unsafe { efi::init(image_handle, system_table) };
    let (map_key, _stack) = initialize(&bs);
    bs.exit_boot_services(map_key).map_err(|(status, _)| status).unwrap();
    abort();
}

/// Performs any UEFI-specific initialization.
///
/// Returns a memory map key, plus a pointer to a region of memory to use as a
/// stack.
///
/// This is in a separate function because we can't exit boot services while
/// the `stdout` handle is alive. There are three possible solutions to this
/// problem:
///
/// 1. Put all the initialization code in a `{}`-block. This adds more
///    indentation and sadness
///
/// 2. Refactor the initialization code into a separate function
///
/// 3. Wait for non-lexical borrows to land, so we can just `mem::drop` the
///    handle and avoid this rigmarole in the first place
fn initialize(bs: &BootServices) -> (MapKey, *mut u8) {
    let stdout = bs.stdout();
    {
        let graphics_output = bs.locate_protocol::<GraphicsOutput>()
            .expect("could not find graphics adapter");
        for mode in 0 .. graphics_output.max_mode() {
            let info = graphics_output.query_mode(mode);
            efi_log!(stdout, "");
            efi_log!(stdout, "Mode #{}:", mode);
            efi_log!(stdout, "{:?}", info);
        }
        efi_log!(stdout, "");
        efi_log!(stdout, "Current mode is: {}", graphics_output.current_mode());
    }
    // Construct the physical memory allocator
    let memory_size = {
        let (memory_map, _) = bs.memory_map();
        calculate_physical_memory_size(&memory_map)
    };
    efi_log!(stdout, "Found {} bytes of memory", memory_size);
    let physical_alloc = BitmapAllocator::new(
        memory_size, efi::PAGE_SIZE, |size| unsafe { bs.allocate(size) });
    PHYSICAL_ALLOC.call_once(|| RwLock::new(physical_alloc));
    // Query the memory map again, since allocating the bitmap would have
    // changed it
    let (memory_map, map_key) = bs.memory_map();
    for desc in &memory_map {
        if !is_usable_memory_type(desc.type_) {
            PHYSICAL_ALLOC.try().unwrap().write().mark_as_used(
                desc.physical_start.0 as usize,
                efi::PAGE_SIZE * desc.number_of_pages as usize);
        }
    }
    // The current call stack is in boot services memory, which will be
    // overwritten at the end of the boot process
    // So we must allocate a new stack and hop into it
    // FIXME: 4k stack lol are you kidding me
    let stack = PHYSICAL_ALLOC.try().unwrap().write().allocate() as *mut u8;
    // We can't deallocate the memory map, since that may change the memory map,
    // invalidating the map key
    mem::forget(memory_map);
    (map_key, stack)
}

fn calculate_physical_memory_size(memory_map: &MemoryMap) -> usize {
    let physical_end = memory_map.iter().map(|desc| desc.physical_end()).max().unwrap();
    physical_end.0 as usize
}

fn is_usable_memory_type(type_: MemoryType) -> bool {
    use efi::MemoryType::*;
    match type_ {
        BootServicesCode | BootServicesData | ConventionalMemory => true,
        _ => false,
    }
}

#[no_mangle]
pub fn abort() -> ! {
    loop {
        unsafe { asm!("hlt" :::: "volatile"); }
    }
}

#[lang = "panic_fmt"]
extern fn panic_fmt(args: core::fmt::Arguments, file: &str, line: u32) -> ! {
    unsafe {
        if let Some(bs) = BootServices::get_instance() {
            let _ = write!(bs.stdout(), "\r
\r
===================== PANIC ======================\r
{args}\r
    at {file}:{line}\r
==================================================\r
\r
", args = args, file = file, line = line);
        }
    }
    abort();
}
