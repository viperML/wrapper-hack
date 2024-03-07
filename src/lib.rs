use std::{f64::consts, ffi::CString, ptr::{null, null_mut}};

use libc::{c_char, c_int, dlsym, EIO, RTLD_DEFAULT, RTLD_NEXT};

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

#[no_mangle]
pub unsafe extern "C" fn execve(
    prog: *const c_char,
    argv: *const *const c_char,
    envp: *const *const c_char,
) -> c_int {
    let name = CString::new("execve").unwrap().as_ptr();
    let addr = dlsym(RTLD_DEFAULT, name);

    eprintln!("addr: {:p}", addr);
    if addr == null_mut() {
        eprintln!("Couldn't find execve!");
        return EIO;
    }

    let execve_real: extern "C" fn(
        *const c_char,
        *const *const c_char,
        *const *const c_char,
    ) -> c_int = std::mem::transmute(addr);
    let _prog = CString::from_raw(prog as *mut _);
    println!("called execve! {:?}", _prog);
    execve_real(prog, argv, envp)
}
