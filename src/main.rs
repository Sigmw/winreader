mod error;
mod memory;
mod module;
use std::{fs::File};
use std::io::prelude::*;
use crate::memory::address::get_memory_address;
use crate::memory::mem::get_process_mem;
use crate::memory::path_name::get_path_and_name_process;
use crate::memory::stack::read_process_stack;
use crate::module::modules::print_dependencies;
use chrono::prelude::*;
use clap::{Arg, Command};
use winapi::ctypes::c_void;

fn main() {
    let matches = Command::new("winreader")
        .arg_required_else_help(true)
        .version("1.0.0")
        .about("winreader: Read buffer memory in processes on Windows")
        .arg(
            Arg::new("PID")
                .help("Program Process Identifier")
                .value_parser(clap::value_parser!(u32).range(..=4294967295)),
        )
        .get_matches();

    let pid = matches.get_one::<u32>("PID").unwrap();
    let address = get_memory_address(*pid);
    let address_usize = address.unwrap() as usize;
    let (process_path, process_name) = get_path_and_name_process(*pid).unwrap();
    let module_dependencies = print_dependencies(*pid);
    let mem_alloc = get_process_mem(*pid);
    let mem_alloc = mem_alloc / 1024 / 1024;
    let mem_stack = read_process_stack(*pid, address_usize);
    
    create_file_dump(*pid, process_path, process_name, address.unwrap(), mem_alloc, mem_stack, module_dependencies)
}

fn create_file_dump(pid: u32, path: String, name: String, address: *mut c_void, mem_alloc: u64, mem_stack: Vec<u8>, deps: Vec<String>) {
    let local: DateTime<Local> = Local::now();
    let date = local.format("%H:%M:%S - %d/%m/%Y").to_string();

    let mut buffer = format!("----------------------------WINREADER DUMP----------------------------\n\nDATE: {date}\n\nPROCESS PID: {pid}\nPROCESS NAME: {name:?}\nPROCESS PATH: {path}\nMEMORY ADDRESS: {address:?}\nALLOCATED MEMORY (IN PROCESS DUMP REVIEW): {mem_alloc}MiB\n\nMEMORY STACK DUMP: {mem_stack:?}\n\nMODULE DEPENDENCIES USED BY PROCESS:\n");
    for dep in deps {
        buffer.push_str(format!("- {dep}\n").as_str());
    }
    buffer.push_str("----------------------------END DUMP----------------------------");
    
    let path_file = format!("WINREADER-{}.txt", local.format("%H-%M-%d-%m-%Y"));
    let file = File::create(&path_file);
    file.expect("Error:").write_all(buffer.as_bytes()).unwrap();
    println!("Dump saved in {path_file}");
    
}
