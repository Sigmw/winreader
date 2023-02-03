mod memory;
use crate::memory::address::get_memory_address;
use crate::memory::path::get_path_process;
use crate::memory::read::read_process_memory;
use crate::memory::mem::get_process_mem;
use clap::{Arg, Command};

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
    let adress_usize = address.unwrap() as usize;
    let process_path = get_path_process(*pid);
    let mem_alloc = get_process_mem(*pid);
    let mem_alloc = mem_alloc / 1024 / 1024;
    read_process_memory(*pid, adress_usize);

    println!("Informations about {} PID:", *pid);
    println!("Process name: {:?}", process_path.unwrap());
    println!("Memory Adress: {:?}", address.unwrap());
    println!("Allocated Memory: {mem_alloc:?}MiB");
    println!("Memory Buffer saved in WINREADER-DUMP.txt");
}
