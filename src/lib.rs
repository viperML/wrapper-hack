#![no_std]

use core::{mem, ptr::null_mut};

use libc::{c_char, c_int, dlsym, RTLD_NEXT};
use nix::errno::Errno;

use libc_print::std_name::eprintln;

#[no_mangle]
pub unsafe extern "C" fn execve(
    prog: *const c_char,
    argv: *const *const c_char,
    envp: *const *const c_char,
) -> c_int {
    let addr = dlsym(RTLD_NEXT, c"execve".as_ptr());

    if addr == null_mut() {
        eprintln!("Couldn't find original execve");
        Errno::EIO.set();
        return -1;
    }

    let execve_real: extern "C" fn(
        *const c_char,
        *const *const c_char,
        *const *const c_char,
    ) -> c_int = mem::transmute(addr);

    eprintln!("Hello! 🦀");

    execve_real(prog, argv, envp)
}
