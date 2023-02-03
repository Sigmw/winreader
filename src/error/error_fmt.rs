use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;
use std::ptr;
use winapi::shared::minwindef::{DWORD};
use winapi::um::errhandlingapi::GetLastError;
use winapi::um::winbase::{
    FormatMessageW, FORMAT_MESSAGE_FROM_SYSTEM, FORMAT_MESSAGE_IGNORE_INSERTS,
};

pub fn get_last_error_message() -> String {
    let error = unsafe { GetLastError() };

    let mut buffer = vec![0u16; 1024];
    let result = unsafe {
        FormatMessageW(
            FORMAT_MESSAGE_FROM_SYSTEM | FORMAT_MESSAGE_IGNORE_INSERTS,
            ptr::null_mut(),
            error,
            0,
            buffer.as_mut_ptr(),
            buffer.len() as DWORD,
            ptr::null_mut(),
        )
    };
    if result == 0 {
        return format!("Failed to retrieve error message, error code: {error}");
    }

    let message = OsString::from_wide(&buffer[..(result as usize) - 2])
        .to_string_lossy()
        .into_owned();
    message
}
