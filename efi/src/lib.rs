#![feature(question_mark)]
#![no_std]

pub extern crate efi_sys as sys;

use core::fmt;
use core::iter;
use core::marker::PhantomData;
use core::mem;


static mut INSTANCE: Option<(sys::HANDLE, *const sys::SYSTEM_TABLE)> = None;


// TODO: make this more typed
pub type Error = sys::STATUS;

pub type EfiResult<T> = Result<T, Error>;

pub fn check_status(status: sys::STATUS) -> EfiResult<()> {
    if status == sys::SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}


pub struct Efi {
    #[allow(dead_code)] image_handle: sys::HANDLE,
    system_table: *const sys::SYSTEM_TABLE,
}

impl Efi {
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

    pub fn stdout(&self) -> SimpleTextOutput {
        unsafe { SimpleTextOutput::new((*self.system_table).ConOut) }
    }

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


pub struct SimpleTextOutput<'e> {
    out: *const sys::SIMPLE_TEXT_OUTPUT_PROTOCOL,
    _marker: PhantomData<&'e Efi>,
}

impl<'e> SimpleTextOutput<'e> {
    pub unsafe fn new(out: *const sys::SIMPLE_TEXT_OUTPUT_PROTOCOL) -> SimpleTextOutput<'e> {
        SimpleTextOutput {
            out: out,
            _marker: PhantomData,
        }
    }

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
