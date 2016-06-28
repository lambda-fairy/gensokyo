#![feature(coerce_unsized)]
#![feature(question_mark)]
#![feature(unique)]
#![feature(unsize)]
#![no_std]

//! This crate provides a high-level interface to UEFI.

pub extern crate efi_sys as sys;

use core::fmt;
use core::iter;
use core::marker::{PhantomData, Unsize};
use core::mem;
use core::ops::{CoerceUnsized, Deref, DerefMut};
use core::ptr::{self, Unique};


static mut INSTANCE: Option<(sys::Handle, *const sys::SystemTable)> = None;


// TODO: make this more typed
pub type Error = sys::Status;

pub type EfiResult<T> = Result<T, Error>;

/// Converts a low-level `EFI_STATUS` to a high-level `EfiResult`.
///
/// This returns `Ok` if the high (error) bit is not set, and `Err` otherwise.
pub fn check_status(status: sys::Status) -> EfiResult<()> {
    // TODO: handle warnings
    if status & sys::MAX_BIT == 0 {
        Ok(())
    } else {
        Err(status)
    }
}


/// The main UEFI entry point.
pub struct Efi {
    #[allow(dead_code)] image_handle: sys::Handle,
    system_table: *const sys::SystemTable,
}

impl Efi {
    /// Constructs a UEFI wrapper.
    ///
    /// # Safety
    ///
    /// This is unsafe, because the user must ensure that the arguments are
    /// valid and not null.
    pub unsafe fn new(
        image_handle: sys::Handle,
        system_table: *const sys::SystemTable) -> Efi
    {
        if INSTANCE.is_some() {
            panic!("Efi::new() cannot be called more than once");
        }
        INSTANCE = Some((image_handle, system_table));
        Efi {
            image_handle: image_handle,
            system_table: system_table,
        }
    }

    /// Returns a handle to the console output.
    pub fn stdout(&self) -> SimpleTextOutput {
        unsafe { SimpleTextOutput::new((*self.system_table).con_out) }
    }

    /// Returns a handle to the console standard error.
    pub fn stderr(&self) -> SimpleTextOutput {
        unsafe { SimpleTextOutput::new((*self.system_table).std_err) }
    }

    /// Places an object on the UEFI heap.
    ///
    /// Panics if the allocation fails.
    pub fn boxed<T>(&self, value: T) -> EfiBox<T> {
        unsafe {
            let ptr = self.allocate(mem::size_of::<T>()) as *mut T;
            ptr::write(ptr, value);
            EfiBox::from_raw(ptr)
        }
    }

    /// Allocates a block of memory using the UEFI allocator. The memory is of
    /// type `EfiLoaderData`.
    ///
    /// Panics if the allocation fails.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the memory is freed (using `deallocate()`)
    /// before exiting boot services.
    pub unsafe fn allocate(&self, size: usize) -> *mut u8 {
        let mut buffer = ptr::null_mut() as *mut u8;
        let result = ((*(*self.system_table).boot_services).allocate_pool)(
                sys::MemoryType::LoaderData,
                size,
                &mut buffer as *mut _ as *mut _);
        check_status(result).unwrap();
        buffer
    }

    /// Deallocates a block of memory provided by `allocate()`.
    pub unsafe fn deallocate(&self, buffer: *mut u8) {
        // Ignore the result, since nobody checks the result of free() anyway
        let _ = ((*(*self.system_table).boot_services).free_pool)(buffer as *mut _);
    }

    /// Retrieves a copy of the UEFI memory map.
    pub fn memory_map(&self) -> MemoryMap {
        unsafe {
            let mut memory_map_size: usize = 0;
            let mut map_key: usize = 0;
            let mut descriptor_size: usize = 0;
            let mut descriptor_version: u32 = 0;
            // First, make a call with a null buffer to get its size
            let _ = ((*(*self.system_table).boot_services).get_memory_map)(
                &mut memory_map_size as *mut _,
                ptr::null_mut() as *mut _,
                &mut map_key as *mut _,
                &mut descriptor_size as *mut _,
                &mut descriptor_version as *mut _);
            // Add some wiggle room to account for the extra allocation
            // The '4' is completely arbitrary, but it tends to work in practice
            memory_map_size += 4 * descriptor_size;
            // Now call it again with an actual buffer
            let memory_map = self.allocate(memory_map_size) as *mut _;
            let result = ((*(*self.system_table).boot_services).get_memory_map)(
                &mut memory_map_size as *mut _,
                memory_map,
                &mut map_key as *mut _,
                &mut descriptor_size as *mut _,
                &mut descriptor_version as *mut _);
            match check_status(result) {
                Ok(..) => MemoryMap::from_raw(memory_map, memory_map_size, descriptor_size),
                Err(e) => {
                    self.deallocate(memory_map as *mut _);
                    panic!("Could not get memory map (error {:?})", e);
                },
            }
        }
    }

    /// Invokes the given callback with a reference to a current live `Efi`
    /// object. Returns the result of the callback.
    ///
    /// If there is no current `Efi` object, the callback is ignored and `None`
    /// is returned instead.
    ///
    /// This method is useful for writing panic handlers, since they don't have
    /// direct access to the system table.
    ///
    /// # Examples
    ///
    /// ```rust
    /// #[lang = "panic_fmt"]
    /// fn panic_fmt(args: core::fmt::Arguments, file: &str, line: u32) -> ! {
    ///     // Log the panic to the console
    ///     let _ = Efi::with_instance(|efi| {
    ///         write!(efi.stderr(), "PANIC: {} {} {}\r\n", args, file, line)
    ///     });
    ///     loop {}
    /// }
    /// ```
    pub fn with_instance<F, T>(callback: F) -> Option<T> where
        F: FnOnce(&Efi) -> T
    {
        unsafe { INSTANCE }.map(|(image_handle, system_table)| {
            let efi = Efi {
                image_handle: image_handle,
                system_table: system_table,
            };
            let result = callback(&efi);
            mem::forget(efi);
            result
        })
    }
}

impl Drop for Efi {
    fn drop(&mut self) {
        unsafe { INSTANCE = None; }
    }
}


pub struct EfiBox<'e, T: ?Sized> {
    _marker: PhantomData<&'e Efi>,
    ptr: Unique<T>,
}

impl<'e, T: ?Sized> EfiBox<'e, T> {
    pub unsafe fn from_raw(ptr: *mut T) -> Self {
        EfiBox { _marker: PhantomData, ptr: Unique::new(ptr) }
    }

    pub fn into_raw(self) -> *mut T {
        *self.ptr
    }
}

impl<'e, T: ?Sized + Unsize<U>, U: ?Sized> CoerceUnsized<EfiBox<'e, U>> for EfiBox<'e, T> {}

impl<'e, T: ?Sized> Deref for EfiBox<'e, T> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe { self.ptr.get() }
    }
}

impl<'e, T: ?Sized> DerefMut for EfiBox<'e, T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { self.ptr.get_mut() }
    }
}

impl<'e, T: ?Sized> Drop for EfiBox<'e, T> {
    fn drop(&mut self) {
        Efi::with_instance(|efi| unsafe { efi.deallocate(*self.ptr as *mut _) });
    }
}

impl<'e, T: fmt::Debug + ?Sized> fmt::Debug for EfiBox<'e, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&**self, f)
    }
}

impl<'e, T: fmt::Display + ?Sized> fmt::Display for EfiBox<'e, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&**self, f)
    }
}


/// Provides a simple interface for displaying text.
pub struct SimpleTextOutput<'e> {
    out: *const sys::SimpleTextOutputProtocol,
    _marker: PhantomData<&'e Efi>,
}

impl<'e> SimpleTextOutput<'e> {
    /// Constructs a `SimpleTextOutput` from a raw protocol handle.
    ///
    /// This is a low-level constructor: you most likely want to use
    /// `Efi::stdout()` or `Efi::stderr()` instead.
    ///
    /// # Safety
    ///
    /// This is unsafe because the user must check that the handle points to a
    /// valid object. Also, the user must ensure that the `SimpleTextOutput` is
    /// dropped before exiting boot services.
    pub unsafe fn new(out: *const sys::SimpleTextOutputProtocol) -> SimpleTextOutput<'e> {
        SimpleTextOutput {
            out: out,
            _marker: PhantomData,
        }
    }

    /// Write a string to the handle.
    pub fn write_str(&self, s: &str) -> EfiResult<()> {
        let mut buffer = [0u16; 128];
        let mut chars = s.chars().peekable();
        while chars.peek().is_some() {
            let chunk = chars.by_ref().take(buffer.len()-1).chain(iter::once('\0'));
            for (d, c) in buffer.iter_mut().zip(chunk) {
                *d = c as u16;  // UCS-2
            }
            let status = unsafe {
                ((*self.out).output_string)(self.out, buffer.as_ptr())
            };
            check_status(status)?;
        }
        Ok(())
    }

    /// Write a formatting object to the handle.
    ///
    /// This method lets you use the `write!` macro to output formatted text.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let out = efi.stdout();
    /// write!(out, "Hello, world!\r\n").unwrap();
    /// ```
    pub fn write_fmt(&'e self, args: fmt::Arguments) -> EfiResult<()> {
        struct Writer<'e> {
            inner: &'e SimpleTextOutput<'e>,
            result: EfiResult<()>,
        }
        impl<'e> fmt::Write for Writer<'e> {
            fn write_str(&mut self, s: &str) -> fmt::Result {
                self.result = self.inner.write_str(s);
                self.result.map_err(|_| fmt::Error)
            }
        }
        let mut writer = Writer { inner: self, result: Ok(()) };
        let _ = fmt::Write::write_fmt(&mut writer, args);
        writer.result
    }
}


#[derive(Debug)]
pub struct MemoryMap<'e> {
    _marker: PhantomData<&'e Efi>,
    ptr: *mut sys::MemoryDescriptor,
    memory_map_size: usize,
    descriptor_size: usize,
}

impl<'e> MemoryMap<'e> {
    pub unsafe fn from_raw(
        ptr: *mut sys::MemoryDescriptor,
        memory_map_size: usize,
        descriptor_size: usize) -> Self
    {
        assert!(!ptr.is_null());
        assert!(descriptor_size > 0);
        assert!(memory_map_size % descriptor_size == 0);
        MemoryMap {
            _marker: PhantomData,
            ptr: ptr,
            memory_map_size: memory_map_size,
            descriptor_size: descriptor_size,
        }
    }

    pub fn len(&self) -> usize {
        self.memory_map_size / self.descriptor_size
    }

    pub fn iter<'a>(&'a self) -> MemoryMapIter<'a, 'e> {
        MemoryMapIter {
            _marker: PhantomData,
            ptr: self.ptr,
            memory_map_size: self.memory_map_size,
            descriptor_size: self.descriptor_size,
        }
    }

    pub fn iter_mut<'a>(&'a mut self) -> MemoryMapMutIter<'a, 'e> {
        MemoryMapMutIter {
            _marker: PhantomData,
            ptr: self.ptr,
            memory_map_size: self.memory_map_size,
            descriptor_size: self.descriptor_size,
        }
    }
}

impl<'e> Drop for MemoryMap<'e> {
    fn drop(&mut self) {
        Efi::with_instance(|efi| unsafe { efi.deallocate(self.ptr as *mut _) });
    }
}

impl<'a, 'e: 'a> IntoIterator for &'a MemoryMap<'e> {
    type Item = &'a MemoryDescriptor;
    type IntoIter = MemoryMapIter<'a, 'e>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, 'e: 'a> IntoIterator for &'a mut MemoryMap<'e> {
    type Item = &'a mut MemoryDescriptor;
    type IntoIter = MemoryMapMutIter<'a, 'e>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

pub use sys::{MemoryDescriptor, MemoryType, PhysicalAddress, VirtualAddress, MemoryAttribute};

#[derive(Debug)]
pub struct MemoryMapIter<'a, 'e: 'a> {
    _marker: PhantomData<&'a MemoryMap<'e>>,
    ptr: *mut sys::MemoryDescriptor,
    memory_map_size: usize,
    descriptor_size: usize,
}

impl<'a, 'e> Iterator for MemoryMapIter<'a, 'e> {
    type Item = &'a MemoryDescriptor;
    fn next(&mut self) -> Option<Self::Item> {
        if self.memory_map_size == 0 {
            None
        } else {
            unsafe {
                let result = mem::transmute(self.ptr);
                self.ptr = (self.ptr as *mut u8).offset(self.descriptor_size as isize) as *mut _;
                self.memory_map_size -= self.descriptor_size;
                Some(result)
            }
        }
    }
}

#[derive(Debug)]
pub struct MemoryMapMutIter<'a, 'e: 'a> {
    _marker: PhantomData<&'a MemoryMap<'e>>,
    ptr: *mut sys::MemoryDescriptor,
    memory_map_size: usize,
    descriptor_size: usize,
}

impl<'a, 'e> Iterator for MemoryMapMutIter<'a, 'e> {
    type Item = &'a mut MemoryDescriptor;
    fn next(&mut self) -> Option<Self::Item> {
        if self.memory_map_size == 0 {
            None
        } else {
            unsafe {
                let result = mem::transmute(self.ptr);
                self.ptr = (self.ptr as *mut u8).offset(self.descriptor_size as isize) as *mut _;
                self.memory_map_size -= self.descriptor_size;
                Some(result)
            }
        }
    }
}
