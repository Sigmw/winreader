use crate::error::error_fmt::get_last_error_message;
use std::os::raw::c_void;
use std::ptr;
use windows_sys::Win32::System::Memory::VirtualQueryEx;
use windows_sys::Win32::System::Memory::MEMORY_BASIC_INFORMATION;
use windows_sys::Win32::System::Memory::{MEM_COMMIT, MEM_PRIVATE};

use super::open::open_process_memory;

pub fn get_memory_address(pid: u32) -> Option<*mut c_void> {
    let mut info: MEMORY_BASIC_INFORMATION = unsafe { std::mem::zeroed() };
    let mut address = ptr::null_mut();
    loop {
        // Open a handle to the process's memory
        let handle = open_process_memory(pid);

        // Call the VirtualQueryEx function to retrieve information about a range of pages in the virtual address space of the specified process
        let result = unsafe {
            VirtualQueryEx(
                handle,
                address,
                &mut info as *mut MEMORY_BASIC_INFORMATION,
                std::mem::size_of::<MEMORY_BASIC_INFORMATION>(),
            )
        };
        if result == 0 {
            let error = get_last_error_message();
            println!(
                "Couldn't retrieve information about the virtual address space of {pid} PID: {error}"
            );
            std::process::exit(1);
        }
        // Check if the memory is committed (i.e., it is allocated and reserved for the process) and is private
        if info.State == MEM_COMMIT && info.Type == MEM_PRIVATE {
            // Return the base address of the memory region if the criteria are met
            return Some(info.BaseAddress);
        }
        // Move the address to the next memory region to check
        address = (info.BaseAddress as usize + info.RegionSize) as *mut _;
    }
}
