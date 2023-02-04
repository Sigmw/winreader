use crate::error::error_fmt::get_last_error_message;
use windows_sys::Win32::System::Threading::{
    OpenProcess, PROCESS_QUERY_INFORMATION, PROCESS_VM_READ,
};

pub fn open_process_memory(pid: u32) -> isize {
    let handle = unsafe { OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, 0, pid) };
    if handle == std::ptr::null::<i32>() as *const i32 as isize {
        let error = get_last_error_message();
        println!("Error: Can't open the process of {pid} PID: {error}");
        std::process::exit(1);
    }
    handle
}
