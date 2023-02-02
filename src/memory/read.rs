use crate::memory::open;
use std::fs::File;
use std::io::prelude::*;
use winapi::um::memoryapi::ReadProcessMemory;

pub fn read_process_memory(pid: u32, address: usize) {
    let handle = open::open_process_memory(pid);
    let mut buffer = [0u8; 1024];
    let read = unsafe {
        ReadProcessMemory(
            handle,
            address as *const _,
            buffer.as_mut_ptr() as *mut _,
            buffer.len(),
            std::ptr::null_mut(),
        )
    };
    if read == 0 {
        println!("Error: Can't read the proccess memory of {pid} PID.");
        std::process::exit(1);
    }

    let mut file = match File::create("WINREADER-DUMP.txt") {
        Ok(file) => file,
        Err(e) => {
            println!("Error: Can't create a file for buffer memory dump: {:?}", e);
            std::process::exit(1);
        }
    };
    let binding = buffer.to_vec();
    let binding = String::from_utf8_lossy(&binding);
    match file.write_all(binding.as_bytes()) {
        Ok(_) => return,
        Err(e) => {
            println!(
                "Error: Couldn't write the memory buffer in txt dump file: {:?}",
                e
            );
            std::process::exit(1);
        }
    }
}
