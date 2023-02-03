use std::ptr;
use winapi::um::psapi::{EnumProcessModulesEx, GetModuleFileNameExW};
use winapi::um::handleapi::{CloseHandle};
use winapi::shared::minwindef::HMODULE;
use crate::memory::open::open_process_memory;
use crate::memory::path::get_path_process;

pub fn print_dependencies(pid: u32) -> Vec<String> {
    let process = open_process_memory(pid);

    let mut module_count = 0;
    let mut modules: Vec<HMODULE> = vec![ptr::null_mut(); 1024];
    unsafe {
        EnumProcessModulesEx(process, modules.as_mut_ptr(), (1024 * std::mem::size_of::<HMODULE>()) as u32, &mut module_count, 0x03);
    }
    modules.resize(module_count as usize, ptr::null_mut());

    let mut modules_file_name: Vec<String> = Vec::new();
    let process_name = get_path_process(pid);
    let process_name = process_name.unwrap();
    for i in 0..module_count {
        let handle = modules[i as usize];
        let mut file_name = [0u16; 1024];
        let length = unsafe { GetModuleFileNameExW(process, handle, file_name.as_mut_ptr(), 1024_u32) };
        if length == 0 {
            continue;
        }
        let file_name = String::from_utf16_lossy(&file_name[0..length as usize]);
        if file_name == process_name {
            continue;
        }
        modules_file_name.push(file_name.to_string())
    }
    unsafe { CloseHandle(process) };

    modules_file_name
}