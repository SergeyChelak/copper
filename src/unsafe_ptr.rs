#[derive(Debug, Clone, Copy)]
pub struct UnsafePointer(usize);

impl UnsafePointer {
    pub fn new(address: usize) -> Self {
        Self(address)
    }

    pub fn add(&self, offset: usize) -> Self {
        Self::new(self.0 + offset)
    }

    pub unsafe fn read_volatile<T>(&self) -> T {
        unsafe { core::ptr::read_volatile((*self).into()) }
    }

    pub unsafe fn write_volatile<T>(&self, value: T) {
        unsafe { core::ptr::write_volatile((*self).into(), value) }
    }
}

impl<T> From<*const T> for UnsafePointer {
    fn from(value: *const T) -> Self {
        UnsafePointer::new(value as usize)
    }
}

impl<T> From<UnsafePointer> for *mut T {
    fn from(value: UnsafePointer) -> Self {
        value.0 as *mut T
    }
}

impl<T> From<UnsafePointer> for *const T {
    fn from(value: UnsafePointer) -> Self {
        value.0 as *const T
    }
}
