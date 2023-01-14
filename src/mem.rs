/// Write to raw memory address
pub unsafe fn write<T>(address: *mut T, data: T) { 
    core::ptr::write_volatile(address, data);
}


/// Read from raw memory address
pub unsafe fn read<T: Copy>(address: *mut T) -> T {
    core::ptr::read_volatile(address)
}
