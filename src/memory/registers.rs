use super::thread::get_thread_handle_by_pid;
use crate::error::error_fmt::get_last_error_message;
use windows_sys::Win32::System::Diagnostics::Debug::GetThreadContext;
use windows_sys::Win32::System::Diagnostics::Debug::CONTEXT;

pub struct Registers {
    pub rax: u64,
    pub rbx: u64,
    pub rcx: u64,
    pub rdx: u64,
    pub cs: u16,
    pub ss: u16,
    pub ds: u16,
    pub es: u16,
    pub rip: u64,
    pub rsp: u64,
    pub rsi: u64,
    pub rdi: u64,
    pub eflgs: u32,
    pub rbp: u64,
    pub fs: u16,
    pub gs: u16,
}

// Basically these constants are DWORDs for the context.ContextFlags to receive and assign to the CONTEXT struct.
// Why u32? DWORD = c_ulong = u32 :)
// Thanks to https://github.com/retep998/winapi-rs/blob/0.3/src/um/winnt.rs
const CONTEXT_AMD64: u32 = 0x00100000;
const CONTEXT_CONTROL: u32 = CONTEXT_AMD64 | 0x00000001;
const CONTEXT_INTEGER: u32 = CONTEXT_AMD64 | 0x00000002;
const CONTEXT_FLOATING_POINT: u32 = CONTEXT_AMD64 | 0x00000008;
const CONTEXT_FULL: u32 = CONTEXT_CONTROL | CONTEXT_INTEGER | CONTEXT_FLOATING_POINT;

pub fn get_registers(pid: u32) -> Registers {
    unsafe {
        let h_thread = get_thread_handle_by_pid(pid);
        let error = get_last_error_message();
        println!("{error:?}");
        let mut context: CONTEXT = std::mem::zeroed();
        context.ContextFlags = CONTEXT_FULL;
        let success = GetThreadContext(h_thread, &mut context as *mut CONTEXT);
        if success == 0 {
            panic!("GetThreadContext failed");
        }

        Registers {
            rax: context.Rax,
            rbx: context.Rbx,
            rcx: context.Rcx,
            rdx: context.Rdx,
            cs: context.SegCs,
            ss: context.SegSs,
            ds: context.SegDs,
            es: context.SegEs,
            rip: context.Rip,
            rsp: context.Rsp,
            rsi: context.Rsi,
            rdi: context.Rdi,
            eflgs: context.EFlags,
            rbp: context.Rbp,
            fs: context.SegFs,
            gs: context.SegGs,
        }
    }
}
