use crate::{
    error::error_fmt::get_last_error_message,
    memory::{open::open_process_memory, path_name::get_path_and_name_process},
};
use std::{ffi::OsString, os::windows::prelude::OsStringExt};
use windows_sys::Win32::{
    Foundation::CloseHandle,
    System::ProcessStatus::{K32EnumProcessModulesEx, K32GetModuleFileNameExW},
};

pub fn print_dependencies(pid: u32) -> Vec<String> {
    let process = open_process_memory(pid);

    let mut module_count = 0;
    let mut modules: Vec<isize> = vec![0; 1024];
    unsafe {
        let result = K32EnumProcessModulesEx(
            process,
            modules.as_mut_ptr(),
            (1024 * std::mem::size_of::<isize>()) as u32,
            &mut module_count,
            0x03,
        );
        if result == 0 {
            let error = get_last_error_message();
            println!("Error: Failed to enumerate process modules of {pid} PID: {error}");
            std::process::exit(1);
        }
    }
    modules.resize(module_count as usize, 0);

    let mut modules_file_name: Vec<String> = Vec::new();
    let (process_name, _) = get_path_and_name_process(pid).unwrap();
    for i in 0..module_count {
        let handle = modules[i as usize];
        let mut file_name = [0u16; 1024];
        let length =
            unsafe { K32GetModuleFileNameExW(process, handle, file_name.as_mut_ptr(), 1024_u32) };
        if length == 0 {
            continue;
        }
        let file_name = OsString::from_wide(&file_name[0..length as usize])
            .to_string_lossy()
            .into_owned();
        if file_name == process_name {
            continue;
        }
        modules_file_name.push(file_name.to_string())
    }
    unsafe { CloseHandle(process) };

    modules_file_name
}
