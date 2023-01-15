use core::sync::atomic::{Ordering, AtomicPtr};

/// Write to raw memory address
pub unsafe fn write<T>(address: *mut T, data: T) { 
    core::ptr::write_volatile(address, data);
}


/// Read from raw memory address
pub unsafe fn read<T: Copy>(address: *mut T) -> T {
    core::ptr::read_volatile(address)
}

bitflags::bitflags! {

    struct PageFlags: u32 {
        const ALLOCATED = 0x80000000;
        const KERNEL_PAGE = 0x40000000;

    }

}

/// Infromation about a page of memory 
#[repr(C)]
struct Page {
    vaddr: u32,
    flags: u32,
    next: *mut Page,
    prev: *mut Page
}


fn get_page_size() -> u32 {
    4 * 1024 * 1024
}

fn get_mem_size() -> u32 {
    1024 * 1024 * 1024 // TODO: The right thing
}

static mut PAGES: AtomicPtr<Page> = AtomicPtr::new(core::ptr::null_mut());

extern "C" {
    static __end: *mut Page;
}


use log::debug;
pub fn init() {
    let mem_size = get_mem_size();
    debug!("Memory: {mem_size}");
    let num_pages = mem_size / get_page_size();
    debug!("Total pages: {}", num_pages);
    let page_array_len = core::mem::size_of::<Page>() as u32 * num_pages;
    unsafe { 
        PAGES.store(__end, Ordering::Relaxed);
        debug!("End? {:p}", PAGES.get_mut());
        core::ptr::write_bytes(PAGES.get_mut(), 0, page_array_len as usize);
        let kernel_pages = __end as u32 / get_page_size();
        debug!("Kernel pages: {}", kernel_pages);
        for i in 0..kernel_pages {
            (*__end.wrapping_offset(i as isize)).vaddr = i * get_page_size();
            (*__end.wrapping_offset(i as isize)).flags = PageFlags::ALLOCATED.union(PageFlags::KERNEL_PAGE).bits();
        }
        for i in kernel_pages..num_pages {
            // TODO: Something
        }
    }
    debug!("Memory iniitialized.");
}
