#![feature(question_mark)]
#![no_std]

//! This crate provides a high-level interface to UEFI.

pub extern crate efi_sys as sys;

use core::fmt;
use core::iter;
use core::marker::PhantomData;
use core::mem;


static mut INSTANCE: Option<(sys::HANDLE, *const sys::SYSTEM_TABLE)> = None;


// TODO: make this more typed
pub type Error = sys::STATUS;

pub type EfiResult<T> = Result<T, Error>;

/// Converts a low-level `EFI_STATUS` to a high-level `EfiResult`.
///
/// This returns `Ok` if the high (error) bit is not set, and `Err` otherwise.
pub fn check_status(status: sys::STATUS) -> EfiResult<()> {
    // TODO: handle warnings
    if status & sys::MAX_BIT == 0 {
        Ok(())
    } else {
        Err(status)
    }
}


/// The main UEFI entry point.
pub struct Efi {
    #[allow(dead_code)] image_handle: sys::HANDLE,
    system_table: *const sys::SYSTEM_TABLE,
}

impl Efi {
    /// Constructs a UEFI wrapper.
    ///
    /// # Safety
    ///
    /// This is unsafe, because the user must ensure that the arguments are
    /// valid and not null.
    pub unsafe fn new(
        image_handle: sys::HANDLE,
        system_table: *const sys::SYSTEM_TABLE) -> Efi
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
        unsafe { SimpleTextOutput::new((*self.system_table).ConOut) }
    }

    /// Returns a handle to the console standard error.
    pub fn stderr(&self) -> SimpleTextOutput {
        unsafe { SimpleTextOutput::new((*self.system_table).StdErr) }
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


/// Provides a simple interface for displaying text.
pub struct SimpleTextOutput<'e> {
    out: *const sys::SIMPLE_TEXT_OUTPUT_PROTOCOL,
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
    pub unsafe fn new(out: *const sys::SIMPLE_TEXT_OUTPUT_PROTOCOL) -> SimpleTextOutput<'e> {
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
                ((*self.out).OutputString)(self.out, buffer.as_ptr())
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
