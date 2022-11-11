use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn one_tick();
    fn fatal(msg: *const libc::c_char, _: ...);
    fn is_space(ch_0: libc::c_char) -> bool;
    fn stccpy(
        s1: *mut libc::c_char,
        s2: *mut libc::c_char,
        count: libc::c_int,
    ) -> *mut libc::c_char;
    fn lcase(str: *mut libc::c_char);
}
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type size_t = libc::c_ulong;
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
pub type byte = libc::c_uchar;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct environment {
    pub e_label: *mut libc::c_char,
    pub e_string: *mut libc::c_char,
    pub strlen: libc::c_int,
}
static mut l_name: [libc::c_char; 5] =
    unsafe { *::core::mem::transmute::<&[u8; 5], &mut [libc::c_char; 5]>(b"name\0") };
static mut l_save: [libc::c_char; 9] =
    unsafe { *::core::mem::transmute::<&[u8; 9], &mut [libc::c_char; 9]>(b"savefile\0") };
static mut l_score: [libc::c_char; 10] =
    unsafe { *::core::mem::transmute::<&[u8; 10], &mut [libc::c_char; 10]>(b"scorefile\0") };
static mut l_macro: [libc::c_char; 6] =
    unsafe { *::core::mem::transmute::<&[u8; 6], &mut [libc::c_char; 6]>(b"macro\0") };
static mut l_fruit: [libc::c_char; 6] =
    unsafe { *::core::mem::transmute::<&[u8; 6], &mut [libc::c_char; 6]>(b"fruit\0") };
static mut l_drive: [libc::c_char; 6] =
    unsafe { *::core::mem::transmute::<&[u8; 6], &mut [libc::c_char; 6]>(b"drive\0") };
static mut l_menu: [libc::c_char; 5] =
    unsafe { *::core::mem::transmute::<&[u8; 5], &mut [libc::c_char; 5]>(b"menu\0") };
static mut l_screen: [libc::c_char; 7] =
    unsafe { *::core::mem::transmute::<&[u8; 7], &mut [libc::c_char; 7]>(b"screen\0") };
#[no_mangle]
pub static mut whoami: [libc::c_char; 24] = unsafe {
    *::core::mem::transmute::<&[u8; 24], &mut [libc::c_char; 24]>(
        b"Rodney\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
    )
};
#[no_mangle]
pub static mut s_score: [libc::c_char; 15] = unsafe {
    *::core::mem::transmute::<&[u8; 15], &mut [libc::c_char; 15]>(b"rogue.scr\0\0\0\0\0\0")
};
#[no_mangle]
pub static mut s_save: [libc::c_char; 15] = unsafe {
    *::core::mem::transmute::<&[u8; 15], &mut [libc::c_char; 15]>(b"rogue.sav\0\0\0\0\0\0")
};
#[export_name = "macro"]
pub static mut macro_0: [libc::c_char; 42] = unsafe {
    *::core::mem::transmute::<&[u8; 42], &mut [libc::c_char; 42]>(
        b"v\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
    )
};
#[no_mangle]
pub static mut fruit: [libc::c_char; 24] = unsafe {
    *::core::mem::transmute::<&[u8; 24], &mut [libc::c_char; 24]>(
        b"Slime Mold\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
    )
};
#[no_mangle]
pub static mut s_drive: [libc::c_char; 2] =
    unsafe { *::core::mem::transmute::<&[u8; 2], &mut [libc::c_char; 2]>(b"?\0") };
#[no_mangle]
pub static mut s_menu: [libc::c_char; 4] =
    unsafe { *::core::mem::transmute::<&[u8; 4], &mut [libc::c_char; 4]>(b"on\0\0") };
#[no_mangle]
pub static mut s_screen: [libc::c_char; 8] =
    unsafe { *::core::mem::transmute::<&[u8; 8], &mut [libc::c_char; 8]>(b"\0w fast\0") };
static mut element: [environment; 8] = unsafe {
    [
        {
            let mut init = environment {
                e_label: l_name.as_ptr() as *mut _,
                e_string: whoami.as_ptr() as *mut _,
                strlen: 23 as libc::c_int,
            };
            init
        },
        {
            let mut init = environment {
                e_label: l_score.as_ptr() as *mut _,
                e_string: s_score.as_ptr() as *mut _,
                strlen: 14 as libc::c_int,
            };
            init
        },
        {
            let mut init = environment {
                e_label: l_save.as_ptr() as *mut _,
                e_string: s_save.as_ptr() as *mut _,
                strlen: 14 as libc::c_int,
            };
            init
        },
        {
            let mut init = environment {
                e_label: l_macro.as_ptr() as *mut _,
                e_string: macro_0.as_ptr() as *mut _,
                strlen: 40 as libc::c_int,
            };
            init
        },
        {
            let mut init = environment {
                e_label: l_fruit.as_ptr() as *mut _,
                e_string: fruit.as_ptr() as *mut _,
                strlen: 23 as libc::c_int,
            };
            init
        },
        {
            let mut init = environment {
                e_label: l_drive.as_ptr() as *mut _,
                e_string: s_drive.as_ptr() as *mut _,
                strlen: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = environment {
                e_label: l_menu.as_ptr() as *mut _,
                e_string: s_menu.as_ptr() as *mut _,
                strlen: 3 as libc::c_int,
            };
            init
        },
        {
            let mut init = environment {
                e_label: l_screen.as_ptr() as *mut _,
                e_string: s_screen.as_ptr() as *mut _,
                strlen: 7 as libc::c_int,
            };
            init
        },
    ]
};
static mut file: *mut FILE = 0 as *const FILE as *mut FILE;
static mut ch: byte = 0;
static mut pstate: libc::c_int = 0;
static mut blabel: [libc::c_char; 11] = [0; 11];
static mut bstring: [libc::c_char; 25] = [0; 25];
static mut plabel: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut pstring: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub unsafe extern "C" fn setenv_from_file(mut envfile: *mut libc::c_char) -> bool {
    let mut pc: libc::c_char = 0;
    one_tick();
    file = fopen(envfile, b"r\0" as *const u8 as *const libc::c_char);
    if file.is_null() {
        return 0 as libc::c_int != 0;
    }
    loop {
        pstate = 0 as libc::c_int;
        plabel = blabel.as_mut_ptr();
        pstring = bstring.as_mut_ptr();
        while is_space(peekc() as libc::c_char) {}
        if ch as libc::c_int == 0 as libc::c_int {
            fclose(file);
            return 1 as libc::c_int != 0;
        }
        pstate = 3 as libc::c_int;
        if ch as libc::c_int == '#' as i32 {
            while peekc() as libc::c_int != '\n' as i32 {}
        } else {
            pstate = 1 as libc::c_int;
            *plabel = ch as libc::c_char;
            loop {
                pc = peekc() as libc::c_char;
                if !(pc as libc::c_int != '=' as i32 && pc as libc::c_int != '-' as i32) {
                    break;
                }
                if !is_space(*plabel) || !is_space(ch as libc::c_char) {
                    plabel = plabel.offset(1);
                    *plabel = ch as libc::c_char;
                }
            }
            if !is_space(*plabel) {
                plabel = plabel.offset(1);
            }
            *plabel = 0 as libc::c_int as libc::c_char;
            while is_space(peekc() as libc::c_char) {}
            pstate = 2 as libc::c_int;
            *pstring = ch as libc::c_char;
            while peekc() as libc::c_int != '\n' as i32 {
                if !is_space(*pstring) || !is_space(ch as libc::c_char) {
                    pstring = pstring.offset(1);
                    *pstring = ch as libc::c_char;
                }
            }
            if !is_space(*pstring) {
                pstring = pstring.offset(1);
            }
            *pstring = 0 as libc::c_int as libc::c_char;
            lcase(blabel.as_mut_ptr());
            putenv_struct(blabel.as_mut_ptr(), bstring.as_mut_ptr());
        }
    }
}
unsafe extern "C" fn peekc() -> byte {
    ch = 0 as libc::c_int as byte;
    if plabel > &mut *blabel.as_mut_ptr().offset(10 as libc::c_int as isize) as *mut libc::c_char {
        plabel = &mut *blabel.as_mut_ptr().offset(10 as libc::c_int as isize) as *mut libc::c_char;
    }
    if pstring > &mut *bstring.as_mut_ptr().offset(24 as libc::c_int as isize) as *mut libc::c_char
    {
        pstring =
            &mut *bstring.as_mut_ptr().offset(24 as libc::c_int as isize) as *mut libc::c_char;
    }
    if fread(
        &mut ch as *mut byte as *mut libc::c_void,
        1 as libc::c_int as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        file,
    ) == 0
        && pstate != 0 as libc::c_int
    {
        if pstate >= 2 as libc::c_int {
            return '\n' as i32 as byte;
        }
        fatal(b"rogue.opt: incorrect file format\n\0" as *const u8 as *const libc::c_char);
    }
    if ch as libc::c_int == 26 as libc::c_int {
        ch = '\n' as i32 as byte;
    }
    return ch;
}
unsafe extern "C" fn putenv_struct(mut label: *mut libc::c_char, mut string: *mut libc::c_char) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        if strcmp(label, element[i as usize].e_label) == 0 as libc::c_int {
            stccpy(
                element[i as usize].e_string,
                string,
                element[i as usize].strlen,
            );
        }
        i += 1;
    }
}
