use winapi::ctypes::c_void;
use winapi::um::processthreadsapi::OpenProcess;
use winapi::um::winnt::{PROCESS_QUERY_INFORMATION, PROCESS_VM_READ};

pub fn open_process_memory(pid: u32) -> *mut c_void {
    let handle = unsafe { OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, 0, pid) };
    if handle == std::ptr::null_mut() {
        println!("Error: Can't open the process of {pid} PID.");
        std::process::exit(1);
    }
    handle
}
