mod error;
mod memory;
mod module;
use crate::memory::address::get_memory_address;
use crate::memory::mem::get_process_mem;
use crate::memory::path_name::get_path_and_name_process;
use crate::memory::registers::get_registers;
use crate::memory::stack::read_process_stack;
use crate::module::modules::print_dependencies;
use chrono::prelude::*;
use clap::{Arg, Command};
use memory::registers::Registers;
use std::fs::File;
use std::io::prelude::*;
use std::os::raw::c_void;

struct Params {
    pid: u32,
    path: String,
    name: String,
    address: *mut c_void,
    mem_alloc: u64,
    mem_stack: Vec<u8>,
    deps: Vec<String>,
    registers: Registers,
}

fn main() {
    let matches = Command::new("winreader")
        .arg_required_else_help(true)
        .version("2.0.0")
        .about("winreader: Read buffer memory in processes on Windows")
        .arg(
            Arg::new("PID")
                .help("Program Process Identifier")
                .value_parser(clap::value_parser!(u32).range(..=4294967295)),
        )
        .get_matches();

    let pid = matches.get_one::<u32>("PID").unwrap();

    let address = get_memory_address(*pid).unwrap();
    let address_usize = address as usize;
    let process_info = get_path_and_name_process(*pid).unwrap();
    let process_path = process_info.0;
    let process_name = process_info.1;
    let module_dependencies = print_dependencies(*pid);
    let mem_alloc_kb = get_process_mem(*pid);
    const MB_FACTOR: u64 = 1024 * 1024;
    let mem_alloc_mb = mem_alloc_kb / MB_FACTOR;
    let mem_stack = read_process_stack(*pid, address_usize);
    let registers_frame = get_registers(*pid);
    create_file_dump(Params {
        pid: *pid,
        path: process_path,
        name: process_name,
        address,
        mem_alloc: mem_alloc_mb,
        mem_stack,
        deps: module_dependencies,
        registers: registers_frame,
    });
}

fn create_file_dump(params: Params) {
    let local: DateTime<Local> = Local::now();
    let date = local.format("%H:%M:%S - %d/%m/%Y").to_string();

    let mut buffer = format!("----------------------------WINREADER DUMP----------------------------\n\nDATE: {date}\n\nPROCESS PID: {}\nPROCESS NAME: {:?}\nPROCESS PATH: {}\nMEMORY ADDRESS: {:?}\nALLOCATED MEMORY (IN PROCESS DUMP REVIEW): {}MiB\n\nMEMORY REGISTERS VALUE:\nRAX={} CS={} RIP={} EFLGS={}\nRBX={} SS={} RSP={} RBP={}\nRCX={} DS={} RSI={} FS={}\nRDX={} ES={} RDI={} GS={}\n\nMEMORY STACK DUMP: {:?}\n\nMODULE DEPENDENCIES USED BY PROCESS:\n", params.pid, params.name, params.path, params.address, params.mem_alloc, params.registers.rax, params.registers.cs, params.registers.rip, params.registers.eflgs, params.registers.rbx, params.registers.ss, params.registers.rsp, params.registers.rbp, params.registers.rcx, params.registers.ds, params.registers.rsi, params.registers.fs, params.registers.rdx, params.registers.es, params.registers.rdi, params.registers.gs, params.mem_stack);
    for dep in params.deps {
        buffer.push_str(format!("- {dep}\n").as_str());
    }
    buffer.push_str("----------------------------END DUMP----------------------------");

    let path_file = format!("WINREADER-{}.txt", local.format("%H-%M-%d-%m-%Y"));
    let file = File::create(&path_file);
    file.expect("Error:").write_all(buffer.as_bytes()).unwrap();
    println!("Dump saved in {path_file}");
}
