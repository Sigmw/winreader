use super::open::open_process_memory;
use windows_sys::Win32::System::Diagnostics::Debug::ReadProcessMemory;

pub fn read_process_stack(pid: u32, address: usize) -> Vec<u8> {
    let handle = open_process_memory(pid);
    let buffer = [0u8; 1024];
    let buffer_ptr = buffer.as_ptr() as *mut u8;
    let mut bytes_read: usize = 0;
    let result = unsafe {
        ReadProcessMemory(
            handle,
            address as *const _,
            buffer_ptr as *mut _,
            buffer.len(),
            &mut bytes_read,
        )
    };
    if result == 0 {
        println!("Error: Could not read process memory of PID {pid}.");
        std::process::exit(1);
    }
    let buffer_slice = unsafe { std::slice::from_raw_parts(buffer_ptr, bytes_read) };
    buffer_slice.to_vec()
}
