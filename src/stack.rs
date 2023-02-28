use std::alloc::Layout;
use std::alloc::{alloc, dealloc};

pub const MIN_STACK_SIZE: usize = 16 * 1024;
pub const STACK_ALIGNMENT: usize = 16;

#[derive(Debug)]
pub struct Stack(pub *mut u8, pub usize);

impl Stack {
    pub fn new(size: usize) -> Self {
        unsafe {
            assert_ne!(size, 0);
            let aligned_size = size & !(STACK_ALIGNMENT - 1);
            let ptr = alloc(Layout::from_size_align_unchecked(aligned_size, STACK_ALIGNMENT));
            Self(ptr, aligned_size)
        }
    }

    #[inline(always)]
    pub fn base(&self) -> *mut u8 {
        unsafe { self.limit().add(self.1) }
    }

    #[inline(always)]
    pub fn limit(&self) -> *mut u8 {
        self.0 as *mut u8
    }
}

impl Drop for Stack {
    #[inline]
    fn drop(&mut self) {
        unsafe { dealloc(self.0, Layout::from_size_align_unchecked(self.1, STACK_ALIGNMENT)) };
    }
}
