use crate::memory::open;
use winapi::um::{memoryapi::ReadProcessMemory};
use crate::error::error_fmt::get_last_error_message;


pub fn read_process_stack(pid: u32, address: usize) -> Vec<u8> {
    let handle = open::open_process_memory(pid);
    let buffer = [0u8; 1024];
    let buffer_ptr = buffer.as_ptr() as *mut u8;
    let mut bytes_read: usize = 0;
    let read = unsafe {
        ReadProcessMemory(
            handle,
            address as *const _,
            buffer_ptr as *mut _,
            buffer.len(),
            &mut bytes_read,
        )
    };
    if read == 0 {
        let error = get_last_error_message();
        println!("Error: Can't read the proccess memory of {pid} PID: {error}");
        std::process::exit(1);
    }
    let buffer_slice = unsafe { std::slice::from_raw_parts(buffer_ptr, bytes_read) };
    buffer_slice.to_vec()
}
