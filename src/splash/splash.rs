use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn vsnprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ::core::ffi::VaList,
    ) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn epyx_yeah(path: *const libc::c_char) -> libc::c_int;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type va_list = __builtin_va_list;
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
static mut PICFILE: *const libc::c_char = b"../rogue.pic\0" as *const u8
    as *const libc::c_char;
static mut PROGNAME: *const libc::c_char = 0 as *const libc::c_char;
#[no_mangle]
pub unsafe extern "C" fn usage(mut stream: *mut FILE) {
    fprintf(
        stream,
        b"Usage: %s [-h|--help] [PICFILE]\n\0" as *const u8 as *const libc::c_char,
        PROGNAME,
    );
}
#[no_mangle]
pub unsafe extern "C" fn fatal(mut fmt: *const libc::c_char, mut args: ...) {
    let mut msg: [libc::c_char; 1000] = [0; 1000];
    let mut argp: ::core::ffi::VaListImpl;
    argp = args.clone();
    vsnprintf(
        msg.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 1000]>() as libc::c_ulong,
        fmt,
        argp.as_va_list(),
    );
    fprintf(
        stderr,
        b"%s: %s\n\0" as *const u8 as *const libc::c_char,
        PROGNAME,
        msg.as_mut_ptr(),
    );
    usage(stderr);
    exit(1 as libc::c_int);
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut arg: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut path: *const libc::c_char = 0 as *const libc::c_char;
    PROGNAME = *argv.offset(0 as libc::c_int as isize);
    loop {
        argc -= 1;
        if !(argc != 0) {
            break;
        }
        argv = argv.offset(1);
        arg = *argv;
        if strcmp(b"-h\0" as *const u8 as *const libc::c_char, arg) == 0
            || strcmp(b"--help\0" as *const u8 as *const libc::c_char, arg) == 0
        {
            printf(
                b"Display a 320x200 PIC image in BSAVE format using SDL\n\0" as *const u8
                    as *const libc::c_char,
            );
            printf(
                b"With CGA colors, palette 1i (Black / Cyan / Magenta / White)\n\0"
                    as *const u8 as *const libc::c_char,
            );
            printf(
                b"Default image path: %s\n\0" as *const u8 as *const libc::c_char,
                PICFILE,
            );
            usage(stdout);
            return 0 as libc::c_int;
        }
        if strcmp(b"--\0" as *const u8 as *const libc::c_char, arg) == 0 {
            argc -= 1;
            break;
        } else {
            if *arg.offset(0 as libc::c_int as isize) as libc::c_int == '-' as i32 {
                fatal(b"invalid option: %s\0" as *const u8 as *const libc::c_char, arg);
            }
            if !path.is_null() {
                fatal(
                    b"too many arguments: %s\0" as *const u8 as *const libc::c_char,
                    arg,
                );
            }
            path = arg;
        }
    }
    if path.is_null() {
        if argc != 0 {
            argv = argv.offset(1);
            path = *argv;
            argc -= 1;
        } else {
            path = PICFILE;
        }
    }
    if argc != 0 {
        argv = argv.offset(1);
        fatal(b"too many arguments: %s\0" as *const u8 as *const libc::c_char, *argv);
    }
    if argc == 0 && !path.is_null() {} else {
        __assert_fail(
            b"!argc && path\0" as *const u8 as *const libc::c_char,
            b"splash.c\0" as *const u8 as *const libc::c_char,
            74 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 23],
                &[libc::c_char; 23],
            >(b"int main(int, char **)\0"))
                .as_ptr(),
        );
    };
    if epyx_yeah(path) == 0 {
        exit(1 as libc::c_int);
    }
    return 0;
}
pub fn main() {
    let mut args: Vec::<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::core::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0(
                (args.len() - 1) as libc::c_int,
                args.as_mut_ptr() as *mut *mut libc::c_char,
            ) as i32,
        )
    }
}
