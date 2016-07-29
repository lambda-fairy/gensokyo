use core::slice;

use bitmap::Bitmap;

/// A simple allocator that uses a bitmap to track pages.
pub struct BitmapAllocator {
    bitmap: Bitmap<'static>,
    page_size: usize,
}

impl BitmapAllocator {
    /// Creates a new allocator.
    ///
    /// * `memory_size` is the size of the physical address space, in bytes.
    ///   This can be derived from the memory map.
    ///
    /// * `page_size` is the size of a page. This must be a power of two, and
    ///   is usually 4096.
    ///
    /// * `allocate` is a callback that allocates memory. It takes a single
    ///   parameter—the number of bytes to allocate—and returns a buffer of that
    ///   size. The buffer must be aligned to `usize` (8 bytes on 64-bit
    ///   platforms, 4 bytes on 32-bit) to allow for future optimizations.
    pub fn new<F>(memory_size: usize, page_size: usize, allocate: F) -> BitmapAllocator where
        F: FnOnce(usize) -> *mut u8,
    {
        assert!(memory_size % page_size == 0, "memory size is a multiple of page size");
        let n_pages = memory_size / page_size;
        let n_bytes = divide_ceiling(n_pages, 8);
        let buffer = unsafe { slice::from_raw_parts_mut(allocate(n_bytes), n_bytes) };
        let mut alloc = BitmapAllocator {
            bitmap: Bitmap::new(buffer),
            page_size: page_size,
        };
        for page in 0 .. alloc.bitmap.len() {
            // If the number of pages is not a multiple of 8, then the bitmap
            // will have some extra space at the end. Mark this space as "used"
            // so that we don't accidentally allocate it.
            alloc.bitmap.set(page, page >= n_pages);
        }
        alloc
    }

    /// Converts an address to a page number.
    fn addr_to_page(&self, addr: usize) -> usize {
        addr / self.page_size
    }

    /// Calculates the minimum number of pages needed to store `len` bytes.
    fn len_to_pages(&self, len: usize) -> usize {
        divide_ceiling(len, self.page_size)
    }

    /// Converts a page number to an address.
    fn page_to_addr(&self, page: usize) -> usize {
        page * self.page_size
    }

    /// Determines whether the page pointed to by `addr` is in use.
    #[allow(dead_code)]  // TODO remove this
    pub fn is_allocated(&self, addr: usize) -> bool {
        let page = self.addr_to_page(addr);
        self.bitmap.get(page)
    }

    /// Mark the pages covered by the region `[addr, addr+len)` as used.
    pub fn mark_as_used(&mut self, addr: usize, len: usize) {
        let page_start = self.addr_to_page(addr);
        let page_len = self.len_to_pages(len);
        for page in page_start .. page_start + page_len {
            self.bitmap.set(page, true);
        }
    }

    /// Allocates a single page. Returns the address of the page. Panics if
    /// there are no free pages left.
    pub fn allocate(&mut self) -> usize {
        let page = self.bitmap.find_zero().expect("out of memory");
        self.bitmap.set(page, true);
        self.page_to_addr(page)
    }

    /// Deallocates a page. Panics if the page has already been freed.
    #[allow(dead_code)]  // TODO remove this
    pub fn deallocate(&mut self, addr: usize) {
        let page = self.addr_to_page(addr);
        if !self.bitmap.get(page) {
            panic!("attempt to free unallocated memory at {:#x}", addr);
        }
        self.bitmap.set(page, false);
    }
}

/// Divides `value` by `base`, rounding toward positive infinity.
fn divide_ceiling(value: usize, base: usize) -> usize {
    (value + (base - 1)) / base
}
