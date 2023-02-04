use windows_sys::Win32::System::Diagnostics::ToolHelp::CreateToolhelp32Snapshot;
use windows_sys::Win32::{
    Foundation::{CloseHandle, HANDLE, INVALID_HANDLE_VALUE},
    System::{
        Diagnostics::ToolHelp::{Thread32First, Thread32Next, TH32CS_SNAPTHREAD, THREADENTRY32},
        Threading::{OpenProcess, OpenThread, THREAD_ALL_ACCESS},
    },
};

use crate::error::error_fmt::get_last_error_message;

pub fn get_thread_handle_by_pid(pid: u32) -> HANDLE {
    unsafe {
        let process_handle = OpenProcess(THREAD_ALL_ACCESS, 0, pid);

        if process_handle == std::ptr::null::<i32>() as *const i32 as isize {
            let error = get_last_error_message();
            println!("Error: Coudn't open process of {pid} PID: {error:?}");
        }

        let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPTHREAD, 0);

        if snapshot == INVALID_HANDLE_VALUE {
            let error = get_last_error_message();
            CloseHandle(process_handle);
            println!("Error: The thread HANDLE value of {pid} is invalid: {error:?}");
            std::process::exit(1);
        }

        let mut thread_entry = THREADENTRY32 {
            dwSize: std::mem::size_of::<THREADENTRY32>() as u32,
            ..std::mem::zeroed()
        };

        if Thread32First(snapshot, &mut thread_entry) == 0 {
            let error = get_last_error_message();
            CloseHandle(snapshot);
            CloseHandle(process_handle);
            println!("Error: Thread32First failed: {error:?}");
            std::process::exit(1);
        }

        let mut h_thread: isize = std::isize::MIN;

        loop {
            if thread_entry.th32OwnerProcessID == pid {
                h_thread = OpenThread(THREAD_ALL_ACCESS, 0, thread_entry.th32ThreadID);
                break;
            }

            if Thread32Next(snapshot, &mut thread_entry) == 0 {
                break;
            }
        }

        CloseHandle(snapshot);
        CloseHandle(process_handle);
        h_thread
    }
}
