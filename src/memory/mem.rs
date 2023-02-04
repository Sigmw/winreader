use super::open::open_process_memory;
use crate::error::error_fmt::get_last_error_message;
use windows_sys::Win32::System::ProcessStatus::K32GetProcessMemoryInfo;
use windows_sys::Win32::System::ProcessStatus::PROCESS_MEMORY_COUNTERS;

pub fn get_process_mem(pid: u32) -> u64 {
    unsafe {
        let handle = open_process_memory(pid);

        let mut mem_alloc: PROCESS_MEMORY_COUNTERS = std::mem::zeroed();
        let result = K32GetProcessMemoryInfo(
            handle,
            &mut mem_alloc,
            std::mem::size_of::<PROCESS_MEMORY_COUNTERS>() as u32,
        );
        if result == 0 {
            let error = get_last_error_message();
            println!(
                "Error: Could not retrieve memory information for process with PID {pid}: {error}"
            );
            std::process::exit(1);
        }
        mem_alloc.WorkingSetSize as u64
    }
}
