use crate::memory::open;
use std::ptr;
use winapi::ctypes::c_void;
use winapi::um::memoryapi::VirtualQueryEx;
use winapi::um::winnt::MEMORY_BASIC_INFORMATION;

pub fn get_memory_address(pid: u32) -> Option<*mut c_void> {
    let mut info: MEMORY_BASIC_INFORMATION = unsafe { std::mem::zeroed() };
    let mut address = ptr::null_mut();
    loop {
        // Call the VirtualQueryEx function to retrieve information about a range of pages in the virtual address space of the specified process
        let result = unsafe {
            VirtualQueryEx(
                open::open_process_memory(pid),
                address,
                &mut info,
                (std::mem::size_of::<MEMORY_BASIC_INFORMATION>() as u32)
                    .try_into()
                    .unwrap(),
            )
        };
        if result == 0 {
            break;
        }
        // Check if the memory is committed (i.e., it is allocated and reserved for the process) and is private
        if info.State == winapi::um::winnt::MEM_COMMIT
            && info.Type == winapi::um::winnt::MEM_PRIVATE
        {
            // Return the base address of the memory region if the criteria are met
            return Some(info.BaseAddress);
        }
        // Move the address to the next memory region to check
        address = (info.BaseAddress as usize + info.RegionSize as usize) as *mut _;
    }
    None
}
