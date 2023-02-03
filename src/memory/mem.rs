use winapi::um::psapi::{GetProcessMemoryInfo, PROCESS_MEMORY_COUNTERS};
use crate::memory::open::open_process_memory;

pub fn get_process_mem(pid: u32) -> u64 {
    unsafe {
        let handle = open_process_memory(pid);

        let mut mem_alloc: PROCESS_MEMORY_COUNTERS = std::mem::zeroed();
        GetProcessMemoryInfo(handle, &mut mem_alloc, std::mem::size_of::<PROCESS_MEMORY_COUNTERS>() as u32);
        mem_alloc.WorkingSetSize as u64
    }
}