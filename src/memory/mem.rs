use crate::memory::open::open_process_memory;
use winapi::um::{
    psapi::{GetProcessMemoryInfo, PROCESS_MEMORY_COUNTERS},
};
use crate::error::error_fmt::get_last_error_message;

pub fn get_process_mem(pid: u32) -> u64 {
    unsafe {
        let handle = open_process_memory(pid);

        let mut mem_alloc: PROCESS_MEMORY_COUNTERS = std::mem::zeroed();
        let result = GetProcessMemoryInfo(
            handle,
            &mut mem_alloc,
            std::mem::size_of::<PROCESS_MEMORY_COUNTERS>() as u32,
        );
        if result == 0 {
            let error = get_last_error_message(); 
            println!(
                "Error: Coudn't get process memory information of {pid} PID: {error}"
            );
            std::process::exit(1);
        }
        mem_alloc.WorkingSetSize as u64
    }
}
