use core::fmt;
use core::iter;

use {EfiResult, Guid, Protocol, check_status};
use sys;


/// Provides a simple interface for displaying text.
pub struct SimpleTextOutput(sys::SimpleTextOutputProtocol);

impl Protocol for SimpleTextOutput {
    const GUID: Guid = sys::SIMPLE_TEXT_OUTPUT_GUID;
}

impl SimpleTextOutput {
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
                (self.0.output_string)(&self.0 as _, buffer.as_ptr())
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
    /// let out = bs.stdout();
    /// write!(out, "Hello, world!\r\n").unwrap();
    /// ```
    pub fn write_fmt(&self, args: fmt::Arguments) -> EfiResult<()> {
        struct Writer<'e> {
            inner: &'e SimpleTextOutput,
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
