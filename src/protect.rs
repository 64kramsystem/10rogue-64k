use ::libc;
extern "C" {
    static mut goodchk: libc::c_int;
}
#[no_mangle]
pub static mut no_step: libc::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn protect(mut drive: libc::c_int) {
    goodchk = 0xd0d as libc::c_int;
    no_step = 0 as libc::c_int;
}
