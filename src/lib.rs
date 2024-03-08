use std::{
    f64::consts,
    ffi::CString,
    ptr::{null, null_mut},
};

use libc::{c_char, c_int, dlsym, EIO, RTLD_DEFAULT, RTLD_NEXT};
use nix::errno::Errno;

#[no_mangle]
pub unsafe extern "C" fn execve(
    prog: *const c_char,
    argv: *const *const c_char,
    envp: *const *const c_char,
) -> c_int {
    let addr = dlsym(RTLD_NEXT, c"execve".as_ptr());

    eprintln!("addr: {:p}", addr);
    if addr == null_mut() {
        eprintln!("Couldn't find execve!");
        Errno::EIO.set();
        return -1;
    }

    let execve_real: extern "C" fn(
        *const c_char,
        *const *const c_char,
        *const *const c_char,
    ) -> c_int = std::mem::transmute(addr);

    let _prog = CString::from_raw(prog as *mut _);
    eprintln!("called execve! {:?}", _prog);

    execve_real(prog, argv, envp)
}
