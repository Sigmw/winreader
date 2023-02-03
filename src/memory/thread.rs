use std::ptr::null_mut;
use winapi::shared::minwindef::DWORD;
use winapi::um::handleapi::{CloseHandle, INVALID_HANDLE_VALUE};
use winapi::um::processthreadsapi::{OpenProcess, OpenThread};
use winapi::um::tlhelp32::{
    CreateToolhelp32Snapshot, Thread32First, Thread32Next, TH32CS_SNAPTHREAD, THREADENTRY32,
};
use winapi::um::winnt::{HANDLE, THREAD_ALL_ACCESS};

pub fn get_thread_handle_by_pid(pid: DWORD) -> HANDLE {
    unsafe {
        let process_handle = OpenProcess(THREAD_ALL_ACCESS, 0, pid);

        if process_handle.is_null() {
            return null_mut();
        }

        let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPTHREAD, 0);

        if snapshot == INVALID_HANDLE_VALUE {
            CloseHandle(process_handle);
            return null_mut();
        }

        let mut thread_entry = THREADENTRY32 {
            dwSize: std::mem::size_of::<THREADENTRY32>() as DWORD,
            ..std::mem::zeroed()
        };

        if Thread32First(snapshot, &mut thread_entry) == 0 {
            CloseHandle(snapshot);
            CloseHandle(process_handle);
            return null_mut();
        }

        let mut h_thread = null_mut();

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
