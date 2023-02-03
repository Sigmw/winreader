use winapi::ctypes::c_void;
use winapi::um::processthreadsapi::OpenProcess;
use winapi::um::winnt::{PROCESS_QUERY_INFORMATION, PROCESS_VM_READ};
use crate::error::error_fmt::get_last_error_message;


pub fn open_process_memory(pid: u32) -> *mut c_void {
    let handle = unsafe { OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, 0, pid) };
    if handle.is_null() {
        let error = get_last_error_message();
        println!("Error: Can't open the process of {pid} PID: {error}");
        std::process::exit(1);
    }
    handle
}
