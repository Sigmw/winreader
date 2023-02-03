use std::path::Path;

use crate::memory::open;
use winapi::um::psapi::GetModuleFileNameExW;

pub fn get_path_and_name_process(pid: u32) -> Option<(String, String)> {
    let handle = open::open_process_memory(pid);
    let mut buffer = vec![0u16; 1024];
    let length = unsafe {
        GetModuleFileNameExW(
            handle,
            std::ptr::null_mut(),
            buffer.as_mut_ptr(),
            buffer.len() as u32,
        )
    };
    if length == 0 {
        return None;
    }
    let process_path = String::from_utf16_lossy(&buffer[..length as usize]);
    let path = Path::new(&process_path);
    let process_name = path.file_name().unwrap().to_str().unwrap().to_string();

    Some((process_path, process_name))
}
