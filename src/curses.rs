use ::libc;
extern "C" {
    pub type ldat;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn vsnprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ::core::ffi::VaList,
    ) -> libc::c_int;
    static mut bwflag: libc::c_int;
    fn fatal(msg: *const libc::c_char, _: ...);
    fn md_nanosleep(nanoseconds: libc::c_long);
    fn swap_bits(data: byte, i: libc::c_uint, j: libc::c_uint, width: libc::c_uint) -> byte;
    // Rust port: Fixed missing parameters.
    fn swint(p0: libc::c_int, p1: &mut sw_regs) -> libc::c_int;
    fn setenv(
        __name: *const libc::c_char,
        __value: *const libc::c_char,
        __replace: libc::c_int,
    ) -> libc::c_int;
    fn beep() -> libc::c_int;
    fn can_change_color() -> bool;
    fn cbreak() -> libc::c_int;
    fn curs_set(_: libc::c_int) -> libc::c_int;
    fn endwin() -> libc::c_int;
    fn flushinp() -> libc::c_int;
    fn has_colors() -> bool;
    fn initscr() -> *mut WINDOW;
    fn init_color(
        _: libc::c_short,
        _: libc::c_short,
        _: libc::c_short,
        _: libc::c_short,
    ) -> libc::c_int;
    fn init_pair(_: libc::c_short, _: libc::c_short, _: libc::c_short) -> libc::c_int;
    fn keypad(_: *mut WINDOW, _: bool) -> libc::c_int;
    fn nodelay(_: *mut WINDOW, _: bool) -> libc::c_int;
    fn noecho() -> libc::c_int;
    fn start_color() -> libc::c_int;
    fn waddch(_: *mut WINDOW, _: chtype) -> libc::c_int;
    fn wattrset(_: *mut WINDOW, _: libc::c_int) -> libc::c_int;
    fn wclear(_: *mut WINDOW) -> libc::c_int;
    fn wclrtoeol(_: *mut WINDOW) -> libc::c_int;
    fn wdelch(_: *mut WINDOW) -> libc::c_int;
    fn wgetch(_: *mut WINDOW) -> libc::c_int;
    fn whline(_: *mut WINDOW, _: chtype, _: libc::c_int) -> libc::c_int;
    fn winsch(_: *mut WINDOW, _: chtype) -> libc::c_int;
    fn wmove(_: *mut WINDOW, _: libc::c_int, _: libc::c_int) -> libc::c_int;
    fn wrefresh(_: *mut WINDOW) -> libc::c_int;
    fn wtimeout(_: *mut WINDOW, _: libc::c_int);
    fn wvline(_: *mut WINDOW, _: chtype, _: libc::c_int) -> libc::c_int;
    fn define_key(_: *const libc::c_char, _: libc::c_int) -> libc::c_int;
    fn key_defined(_: *const libc::c_char) -> libc::c_int;
    fn resizeterm(_: libc::c_int, _: libc::c_int) -> libc::c_int;
    fn use_default_colors() -> libc::c_int;
    static mut stdscr: *mut WINDOW;
    static mut COLORS: libc::c_int;
    static mut COLS: libc::c_int;
    static mut LINES: libc::c_int;
    fn getcchar(
        _: *const cchar_t,
        _: *mut wchar_t,
        _: *mut attr_t,
        _: *mut libc::c_short,
        _: *mut libc::c_void,
    ) -> libc::c_int;
    fn setcchar(
        _: *mut cchar_t,
        _: *const wchar_t,
        _: attr_t,
        _: libc::c_short,
        _: *const libc::c_void,
    ) -> libc::c_int;
    fn wadd_wch(_: *mut WINDOW, _: *const cchar_t) -> libc::c_int;
    fn wadd_wchnstr(_: *mut WINDOW, _: *const cchar_t, _: libc::c_int) -> libc::c_int;
    fn wget_wch(_: *mut WINDOW, _: *mut wint_t) -> libc::c_int;
    fn whline_set(_: *mut WINDOW, _: *const cchar_t, _: libc::c_int) -> libc::c_int;
    fn win_wch(_: *mut WINDOW, _: *mut cchar_t) -> libc::c_int;
    fn win_wchnstr(_: *mut WINDOW, _: *mut cchar_t, _: libc::c_int) -> libc::c_int;
    fn wvline_set(_: *mut WINDOW, _: *const cchar_t, _: libc::c_int) -> libc::c_int;
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
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
pub type va_list = __builtin_va_list;
pub type wchar_t = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sw_regs {
    pub ax: libc::c_int,
    pub bx: libc::c_int,
    pub cx: libc::c_int,
    pub dx: libc::c_int,
    pub si: libc::c_int,
    pub di: libc::c_int,
    pub ds: libc::c_int,
    pub es: libc::c_int,
}
pub type byte = libc::c_uchar;
pub type chtype = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _win_st {
    pub _cury: libc::c_short,
    pub _curx: libc::c_short,
    pub _maxy: libc::c_short,
    pub _maxx: libc::c_short,
    pub _begy: libc::c_short,
    pub _begx: libc::c_short,
    pub _flags: libc::c_short,
    pub _attrs: attr_t,
    pub _bkgd: chtype,
    pub _notimeout: bool,
    pub _clear: bool,
    pub _leaveok: bool,
    pub _scroll: bool,
    pub _idlok: bool,
    pub _idcok: bool,
    pub _immed: bool,
    pub _sync: bool,
    pub _use_keypad: bool,
    pub _delay: libc::c_int,
    pub _line: *mut ldat,
    pub _regtop: libc::c_short,
    pub _regbottom: libc::c_short,
    pub _parx: libc::c_int,
    pub _pary: libc::c_int,
    pub _parent: *mut WINDOW,
    pub _pad: pdat,
    pub _yoffset: libc::c_short,
    pub _bkgrnd: cchar_t,
    pub _color: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cchar_t {
    pub attr: attr_t,
    pub chars: [wchar_t; 5],
    pub ext_color: libc::c_int,
}
pub type attr_t = chtype;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pdat {
    pub _pad_y: libc::c_short,
    pub _pad_x: libc::c_short,
    pub _pad_top: libc::c_short,
    pub _pad_left: libc::c_short,
    pub _pad_bottom: libc::c_short,
    pub _pad_right: libc::c_short,
}
pub type WINDOW = _win_st;
pub type wint_t = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xlate {
    pub keycode: libc::c_int,
    pub keyis: byte,
}
pub type CCODE = charcode;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct charcode {
    pub ascii: byte,
    pub unicode: *mut wchar_t,
    pub dos: byte,
}
pub type TTYSEQ = ttykeys;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ttykeys {
    pub def: *mut libc::c_char,
    pub dest: libc::c_int,
}
#[no_mangle]
pub static mut is_saved: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut scr_type: libc::c_int = -(1 as libc::c_int);
#[no_mangle]
pub static mut cur_LINES: libc::c_int = 0;
#[no_mangle]
pub static mut cur_COLS: libc::c_int = 0;
#[no_mangle]
pub static mut init_curses: bool = 0 as libc::c_int != 0;
#[no_mangle]
pub static mut charset: libc::c_int = 3 as libc::c_int;
#[no_mangle]
pub static mut colors: libc::c_int = 0;
#[no_mangle]
pub static mut change_colors: bool = 1 as libc::c_int != 0;
#[no_mangle]
pub static mut use_terminal_fgbg: bool = 1 as libc::c_int != 0;
#[no_mangle]
pub static mut tab_size: libc::c_int = 8 as libc::c_int;
static mut ch_attr: libc::c_int = 0x7 as libc::c_int;
static mut curtain: [[cchar_t; 81]; 25] = [[cchar_t {
    attr: 0,
    chars: [0; 5],
    ext_color: 0,
}; 81]; 25];
static mut cctemp: cchar_t = cchar_t {
    attr: 0,
    chars: [0; 5],
    ext_color: 0,
};
static mut KEY_MASK: libc::c_int = 0;
static mut ccunicode: [wchar_t; 2] =
    unsafe { *::core::mem::transmute::<&[u8; 8], &mut [wchar_t; 2]>(b" \0\0\0\0\0\0\0") };
static mut ccode: CCODE = unsafe {
    {
        let mut init = charcode {
            ascii: '\0' as i32 as byte,
            unicode: ccunicode.as_ptr() as *mut _,
            dos: '\0' as i32 as byte,
        };
        init
    }
};
static mut colors_changed: bool = 0 as libc::c_int != 0;
#[no_mangle]
pub static mut savewin: [[cchar_t; 81]; 25] = [[cchar_t {
    attr: 0,
    chars: [0; 5],
    ext_color: 0,
}; 81]; 25];
static mut color_attr: [byte; 18] = [
    0x7 as libc::c_int as byte,
    0x2 as libc::c_int as byte,
    (0x1 as libc::c_int | 0x2 as libc::c_int) as byte,
    0x4 as libc::c_int as byte,
    (0x1 as libc::c_int | 0x4 as libc::c_int) as byte,
    (0x2 as libc::c_int | 0x4 as libc::c_int) as byte,
    (0x8 as libc::c_int | 0 as libc::c_int) as byte,
    (0x8 as libc::c_int | 0x1 as libc::c_int) as byte,
    (0x8 as libc::c_int | 0x2 as libc::c_int) as byte,
    (0x8 as libc::c_int | 0x4 as libc::c_int) as byte,
    (0x8 as libc::c_int | 0x1 as libc::c_int | 0x4 as libc::c_int) as byte,
    (0x8 as libc::c_int | 0x2 as libc::c_int | 0x4 as libc::c_int) as byte,
    (0x8 as libc::c_int | 0x7 as libc::c_int) as byte,
    0x1 as libc::c_int as byte,
    ((0x7 as libc::c_int & 7 as libc::c_int) << 4 as libc::c_int) as byte,
    (0x8 as libc::c_int | 0x7 as libc::c_int) as byte,
    ((0x7 as libc::c_int & 7 as libc::c_int) << 4 as libc::c_int) as byte,
    0 as libc::c_int as byte,
];
static mut monoc_attr: [byte; 18] = [
    0x7 as libc::c_int as byte,
    0x7 as libc::c_int as byte,
    0x7 as libc::c_int as byte,
    0x7 as libc::c_int as byte,
    0x7 as libc::c_int as byte,
    0x7 as libc::c_int as byte,
    0x7 as libc::c_int as byte,
    0x7 as libc::c_int as byte,
    0x7 as libc::c_int as byte,
    0x7 as libc::c_int as byte,
    0x7 as libc::c_int as byte,
    0x7 as libc::c_int as byte,
    (1 as libc::c_int | (1 as libc::c_int & 7 as libc::c_int) << 4 as libc::c_int) as byte,
    0x7 as libc::c_int as byte,
    ((0x7 as libc::c_int & 7 as libc::c_int) << 4 as libc::c_int | 0x8 as libc::c_int) as byte,
    0x7 as libc::c_int as byte,
    ((0x7 as libc::c_int & 7 as libc::c_int) << 4 as libc::c_int | 0x8 as libc::c_int) as byte,
    0 as libc::c_int as byte,
];
static mut at_table: *mut byte = 0 as *const byte as *mut byte;
static mut ctab: [CCODE; 31] = unsafe {
    [
        {
            let mut init = charcode {
                ascii: '@' as i32 as byte,
                unicode: (*::core::mem::transmute::<&[u8; 8], &[libc::c_int; 2]>(b":&\0\0\0\0\0\0"))
                    .as_ptr() as *mut wchar_t,
                dos: 0x1 as libc::c_int as byte,
            };
            init
        },
        {
            let mut init = charcode {
                ascii: '^' as i32 as byte,
                unicode: (*::core::mem::transmute::<&[u8; 8], &[libc::c_int; 2]>(b"f&\0\0\0\0\0\0"))
                    .as_ptr() as *mut wchar_t,
                dos: 0x4 as libc::c_int as byte,
            };
            init
        },
        {
            let mut init = charcode {
                ascii: ':' as i32 as byte,
                unicode: (*::core::mem::transmute::<&[u8; 8], &[libc::c_int; 2]>(b"c&\0\0\0\0\0\0"))
                    .as_ptr() as *mut wchar_t,
                dos: 0x5 as libc::c_int as byte,
            };
            init
        },
        {
            let mut init = charcode {
                ascii: ']' as i32 as byte,
                unicode: (*::core::mem::transmute::<&[u8; 8], &[libc::c_int; 2]>(
                    b"\xD8%\0\0\0\0\0\0",
                ))
                .as_ptr() as *mut wchar_t,
                dos: 0x8 as libc::c_int as byte,
            };
            init
        },
        {
            let mut init = charcode {
                ascii: '=' as i32 as byte,
                unicode: (*::core::mem::transmute::<&[u8; 8], &[libc::c_int; 2]>(
                    b"\xCB%\0\0\0\0\0\0",
                ))
                .as_ptr() as *mut wchar_t,
                dos: 0x9 as libc::c_int as byte,
            };
            init
        },
        {
            let mut init = charcode {
                ascii: '&' as i32 as byte,
                unicode: (*::core::mem::transmute::<&[u8; 8], &[libc::c_int; 2]>(b"@&\0\0\0\0\0\0"))
                    .as_ptr() as *mut wchar_t,
                dos: 0xc as libc::c_int as byte,
            };
            init
        },
        {
            let mut init = charcode {
                ascii: '?' as i32 as byte,
                unicode: (*::core::mem::transmute::<&[u8; 8], &[libc::c_int; 2]>(b"j&\0\0\0\0\0\0"))
                    .as_ptr() as *mut wchar_t,
                dos: 0xd as libc::c_int as byte,
            };
            init
        },
        {
            let mut init = charcode {
                ascii: '*' as i32 as byte,
                unicode: (*::core::mem::transmute::<&[u8; 8], &[libc::c_int; 2]>(b"<&\0\0\0\0\0\0"))
                    .as_ptr() as *mut wchar_t,
                dos: 0xf as libc::c_int as byte,
            };
            init
        },
        {
            let mut init = charcode {
                ascii: ')' as i32 as byte,
                unicode: (*::core::mem::transmute::<&[u8; 8], &[libc::c_int; 2]>(
                    b"\x91!\0\0\0\0\0\0",
                ))
                .as_ptr() as *mut wchar_t,
                dos: 0x18 as libc::c_int as byte,
            };
            init
        },
        {
            let mut init = charcode {
                ascii: '!' as i32 as byte,
                unicode: (*::core::mem::transmute::<&[u8; 8], &[libc::c_int; 2]>(
                    b"\xA1\0\0\0\0\0\0\0",
                ))
                .as_ptr() as *mut wchar_t,
                dos: 0xad as libc::c_int as byte,
            };
            init
        },
        {
            let mut init = charcode {
                ascii: '#' as i32 as byte,
                unicode: (*::core::mem::transmute::<&[u8; 8], &[libc::c_int; 2]>(
                    b"\x92%\0\0\0\0\0\0",
                ))
                .as_ptr() as *mut wchar_t,
                dos: 0xb1 as libc::c_int as byte,
            };
            init
        },
        {
            let mut init = charcode {
                ascii: '+' as i32 as byte,
                unicode: (*::core::mem::transmute::<&[u8; 8], &[libc::c_int; 2]>(b"l%\0\0\0\0\0\0"))
                    .as_ptr() as *mut wchar_t,
                dos: 0xce as libc::c_int as byte,
            };
            init
        },
        {
            let mut init = charcode {
                ascii: '/' as i32 as byte,
                unicode: (*::core::mem::transmute::<&[u8; 8], &[libc::c_int; 2]>(
                    b"\xC4\x03\0\0\0\0\0\0",
                ))
                .as_ptr() as *mut wchar_t,
                dos: 0xe7 as libc::c_int as byte,
            };
            init
        },
        {
            let mut init = charcode {
                ascii: '.' as i32 as byte,
                unicode: (*::core::mem::transmute::<&[u8; 8], &[libc::c_int; 2]>(
                    b"\xB7\0\0\0\0\0\0\0",
                ))
                .as_ptr() as *mut wchar_t,
                dos: 0xfa as libc::c_int as byte,
            };
            init
        },
        {
            let mut init = charcode {
                ascii: '%' as i32 as byte,
                unicode: (*::core::mem::transmute::<&[u8; 8], &[libc::c_int; 2]>(
                    b"a\"\0\0\0\0\0\0",
                ))
                .as_ptr() as *mut wchar_t,
                dos: 0xf0 as libc::c_int as byte,
            };
            init
        },
        {
            let mut init = charcode {
                ascii: '|' as i32 as byte,
                unicode: (*::core::mem::transmute::<&[u8; 8], &[libc::c_int; 2]>(b"Q%\0\0\0\0\0\0"))
                    .as_ptr() as *mut wchar_t,
                dos: 0xba as libc::c_int as byte,
            };
            init
        },
        {
            let mut init = charcode {
                ascii: '-' as i32 as byte,
                unicode: (*::core::mem::transmute::<&[u8; 8], &[libc::c_int; 2]>(b"P%\0\0\0\0\0\0"))
                    .as_ptr() as *mut wchar_t,
                dos: 0xcd as libc::c_int as byte,
            };
            init
        },
        {
            let mut init = charcode {
                ascii: '-' as i32 as byte,
                unicode: (*::core::mem::transmute::<&[u8; 8], &[libc::c_int; 2]>(b"T%\0\0\0\0\0\0"))
                    .as_ptr() as *mut wchar_t,
                dos: 0xc9 as libc::c_int as byte,
            };
            init
        },
        {
            let mut init = charcode {
                ascii: '-' as i32 as byte,
                unicode: (*::core::mem::transmute::<&[u8; 8], &[libc::c_int; 2]>(b"W%\0\0\0\0\0\0"))
                    .as_ptr() as *mut wchar_t,
                dos: 0xbb as libc::c_int as byte,
            };
            init
        },
        {
            let mut init = charcode {
                ascii: '-' as i32 as byte,
                unicode: (*::core::mem::transmute::<&[u8; 8], &[libc::c_int; 2]>(b"Z%\0\0\0\0\0\0"))
                    .as_ptr() as *mut wchar_t,
                dos: 0xc8 as libc::c_int as byte,
            };
            init
        },
        {
            let mut init = charcode {
                ascii: '-' as i32 as byte,
                unicode: (*::core::mem::transmute::<&[u8; 8], &[libc::c_int; 2]>(b"]%\0\0\0\0\0\0"))
                    .as_ptr() as *mut wchar_t,
                dos: 0xbc as libc::c_int as byte,
            };
            init
        },
        {
            let mut init = charcode {
                ascii: 'X' as i32 as byte,
                unicode: (*::core::mem::transmute::<&[u8; 8], &[libc::c_int; 2]>(b"c%\0\0\0\0\0\0"))
                    .as_ptr() as *mut wchar_t,
                dos: 0xb9 as libc::c_int as byte,
            };
            init
        },
        {
            let mut init = charcode {
                ascii: 'X' as i32 as byte,
                unicode: (*::core::mem::transmute::<&[u8; 8], &[libc::c_int; 2]>(b"`%\0\0\0\0\0\0"))
                    .as_ptr() as *mut wchar_t,
                dos: 0xcc as libc::c_int as byte,
            };
            init
        },
        {
            let mut init = charcode {
                ascii: '<' as i32 as byte,
                unicode: (*::core::mem::transmute::<&[u8; 8], &[libc::c_int; 2]>(
                    b"\xC4%\0\0\0\0\0\0",
                ))
                .as_ptr() as *mut wchar_t,
                dos: 0x11 as libc::c_int as byte,
            };
            init
        },
        {
            let mut init = charcode {
                ascii: '/' as i32 as byte,
                unicode: (*::core::mem::transmute::<&[u8; 8], &[libc::c_int; 2]>(
                    b"\x18%\0\0\0\0\0\0",
                ))
                .as_ptr() as *mut wchar_t,
                dos: 0xd9 as libc::c_int as byte,
            };
            init
        },
        {
            let mut init = charcode {
                ascii: '^' as i32 as byte,
                unicode: (*::core::mem::transmute::<&[u8; 8], &[libc::c_int; 2]>(
                    b"\x91!\0\0\0\0\0\0",
                ))
                .as_ptr() as *mut wchar_t,
                dos: 0x18 as libc::c_int as byte,
            };
            init
        },
        {
            let mut init = charcode {
                ascii: 'v' as i32 as byte,
                unicode: (*::core::mem::transmute::<&[u8; 8], &[libc::c_int; 2]>(
                    b"\x93!\0\0\0\0\0\0",
                ))
                .as_ptr() as *mut wchar_t,
                dos: 0x19 as libc::c_int as byte,
            };
            init
        },
        {
            let mut init = charcode {
                ascii: '>' as i32 as byte,
                unicode: (*::core::mem::transmute::<&[u8; 8], &[libc::c_int; 2]>(
                    b"\x92!\0\0\0\0\0\0",
                ))
                .as_ptr() as *mut wchar_t,
                dos: 0x1a as libc::c_int as byte,
            };
            init
        },
        {
            let mut init = charcode {
                ascii: '<' as i32 as byte,
                unicode: (*::core::mem::transmute::<&[u8; 8], &[libc::c_int; 2]>(
                    b"\x90!\0\0\0\0\0\0",
                ))
                .as_ptr() as *mut wchar_t,
                dos: 0x1b as libc::c_int as byte,
            };
            init
        },
        {
            let mut init = charcode {
                ascii: '#' as i32 as byte,
                unicode: (*::core::mem::transmute::<&[u8; 8], &[libc::c_int; 2]>(
                    b"\x93%\0\0\0\0\0\0",
                ))
                .as_ptr() as *mut wchar_t,
                dos: 0xb2 as libc::c_int as byte,
            };
            init
        },
        {
            let mut init = charcode {
                ascii: '`' as i32 as byte,
                unicode: (*::core::mem::transmute::<&[u8; 8], &[libc::c_int; 2]>(
                    b"`\0\0\0\0\0\0\0",
                ))
                .as_ptr() as *mut wchar_t,
                dos: 0 as libc::c_int as byte,
            };
            init
        },
    ]
};
static mut btab: [CCODE; 14] = unsafe {
    [
        {
            let mut init = charcode {
                ascii: '|' as i32 as byte,
                unicode: (*::core::mem::transmute::<&[u8; 8], &[libc::c_int; 2]>(
                    b"\x02%\0\0\0\0\0\0",
                ))
                .as_ptr() as *mut wchar_t,
                dos: 0xb3 as libc::c_int as byte,
            };
            init
        },
        {
            let mut init = charcode {
                ascii: '-' as i32 as byte,
                unicode: (*::core::mem::transmute::<&[u8; 8], &[libc::c_int; 2]>(
                    b"\0%\0\0\0\0\0\0",
                ))
                .as_ptr() as *mut wchar_t,
                dos: 0xc4 as libc::c_int as byte,
            };
            init
        },
        {
            let mut init = charcode {
                ascii: '.' as i32 as byte,
                unicode: (*::core::mem::transmute::<&[u8; 8], &[libc::c_int; 2]>(
                    b"\x0C%\0\0\0\0\0\0",
                ))
                .as_ptr() as *mut wchar_t,
                dos: 0xda as libc::c_int as byte,
            };
            init
        },
        {
            let mut init = charcode {
                ascii: '.' as i32 as byte,
                unicode: (*::core::mem::transmute::<&[u8; 8], &[libc::c_int; 2]>(
                    b"\x10%\0\0\0\0\0\0",
                ))
                .as_ptr() as *mut wchar_t,
                dos: 0xbf as libc::c_int as byte,
            };
            init
        },
        {
            let mut init = charcode {
                ascii: '`' as i32 as byte,
                unicode: (*::core::mem::transmute::<&[u8; 8], &[libc::c_int; 2]>(
                    b"\x14%\0\0\0\0\0\0",
                ))
                .as_ptr() as *mut wchar_t,
                dos: 0xc0 as libc::c_int as byte,
            };
            init
        },
        {
            let mut init = charcode {
                ascii: '\'' as i32 as byte,
                unicode: (*::core::mem::transmute::<&[u8; 8], &[libc::c_int; 2]>(
                    b"\x18%\0\0\0\0\0\0",
                ))
                .as_ptr() as *mut wchar_t,
                dos: 0xd9 as libc::c_int as byte,
            };
            init
        },
        {
            let mut init = charcode {
                ascii: 'H' as i32 as byte,
                unicode: (*::core::mem::transmute::<&[u8; 8], &[libc::c_int; 2]>(b"Q%\0\0\0\0\0\0"))
                    .as_ptr() as *mut wchar_t,
                dos: 0xba as libc::c_int as byte,
            };
            init
        },
        {
            let mut init = charcode {
                ascii: '=' as i32 as byte,
                unicode: (*::core::mem::transmute::<&[u8; 8], &[libc::c_int; 2]>(b"P%\0\0\0\0\0\0"))
                    .as_ptr() as *mut wchar_t,
                dos: 0xcd as libc::c_int as byte,
            };
            init
        },
        {
            let mut init = charcode {
                ascii: '#' as i32 as byte,
                unicode: (*::core::mem::transmute::<&[u8; 8], &[libc::c_int; 2]>(b"T%\0\0\0\0\0\0"))
                    .as_ptr() as *mut wchar_t,
                dos: 0xc9 as libc::c_int as byte,
            };
            init
        },
        {
            let mut init = charcode {
                ascii: '#' as i32 as byte,
                unicode: (*::core::mem::transmute::<&[u8; 8], &[libc::c_int; 2]>(b"W%\0\0\0\0\0\0"))
                    .as_ptr() as *mut wchar_t,
                dos: 0xbb as libc::c_int as byte,
            };
            init
        },
        {
            let mut init = charcode {
                ascii: '#' as i32 as byte,
                unicode: (*::core::mem::transmute::<&[u8; 8], &[libc::c_int; 2]>(b"Z%\0\0\0\0\0\0"))
                    .as_ptr() as *mut wchar_t,
                dos: 0xc8 as libc::c_int as byte,
            };
            init
        },
        {
            let mut init = charcode {
                ascii: '#' as i32 as byte,
                unicode: (*::core::mem::transmute::<&[u8; 8], &[libc::c_int; 2]>(b"]%\0\0\0\0\0\0"))
                    .as_ptr() as *mut wchar_t,
                dos: 0xbc as libc::c_int as byte,
            };
            init
        },
        {
            let mut init = charcode {
                ascii: '#' as i32 as byte,
                unicode: (*::core::mem::transmute::<&[u8; 8], &[libc::c_int; 2]>(
                    b"\x92%\0\0\0\0\0\0",
                ))
                .as_ptr() as *mut wchar_t,
                dos: 0xb1 as libc::c_int as byte,
            };
            init
        },
        {
            let mut init = charcode {
                ascii: '\0' as i32 as byte,
                unicode: (*::core::mem::transmute::<&[u8; 4], &[libc::c_int; 1]>(b"\0\0\0\0"))
                    .as_ptr() as *mut wchar_t,
                dos: 0 as libc::c_int as byte,
            };
            init
        },
    ]
};
static mut ttymap: [TTYSEQ; 9] = [
    {
        let mut init = ttykeys {
            def: b"\x1BOj\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            dest: '*' as i32,
        };
        init
    },
    {
        let mut init = ttykeys {
            def: b"\x1BOk\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            dest: '+' as i32,
        };
        init
    },
    {
        let mut init = ttykeys {
            def: b"\x1BOm\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            dest: '-' as i32,
        };
        init
    },
    {
        let mut init = ttykeys {
            def: b"\x1BOn\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            dest: '.' as i32,
        };
        init
    },
    {
        let mut init = ttykeys {
            def: b"\x1BOo\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            dest: '/' as i32,
        };
        init
    },
    {
        let mut init = ttykeys {
            def: b"\x1BOM\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            dest: 0o527 as libc::c_int,
        };
        init
    },
    {
        let mut init = ttykeys {
            def: b"\x1B[E\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            dest: 0o536 as libc::c_int,
        };
        init
    },
    {
        let mut init = ttykeys {
            def: b"\x1B[1~\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            dest: 0o406 as libc::c_int,
        };
        init
    },
    {
        let mut init = ttykeys {
            def: b"\x1B[4~\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            dest: 0o550 as libc::c_int,
        };
        init
    },
];
static mut dbl_box: [byte; 7] = [
    0xc9 as libc::c_int as byte,
    0xbb as libc::c_int as byte,
    0xc8 as libc::c_int as byte,
    0xbc as libc::c_int as byte,
    0xba as libc::c_int as byte,
    0xcd as libc::c_int as byte,
    0xcd as libc::c_int as byte,
];
static mut sng_box: [byte; 7] = [
    0xda as libc::c_int as byte,
    0xbf as libc::c_int as byte,
    0xc0 as libc::c_int as byte,
    0xd9 as libc::c_int as byte,
    0xb3 as libc::c_int as byte,
    0xc4 as libc::c_int as byte,
    0xc4 as libc::c_int as byte,
];
static mut spc_box: [byte; 7] = [
    0x20 as libc::c_int as byte,
    0x20 as libc::c_int as byte,
    0x20 as libc::c_int as byte,
    0x20 as libc::c_int as byte,
    0x20 as libc::c_int as byte,
    0x20 as libc::c_int as byte,
    0x20 as libc::c_int as byte,
];
static mut xtab: [xlate; 29] = [
    {
        let mut init = xlate {
            keycode: 0o527 as libc::c_int,
            keyis: '\n' as i32 as byte,
        };
        init
    },
    {
        let mut init = xlate {
            keycode: 0o406 as libc::c_int,
            keyis: 'y' as i32 as byte,
        };
        init
    },
    {
        let mut init = xlate {
            keycode: 0o552 as libc::c_int,
            keyis: 'y' as i32 as byte,
        };
        init
    },
    {
        let mut init = xlate {
            keycode: 0o534 as libc::c_int,
            keyis: 'y' as i32 as byte,
        };
        init
    },
    {
        let mut init = xlate {
            keycode: 0o403 as libc::c_int,
            keyis: 'k' as i32 as byte,
        };
        init
    },
    {
        let mut init = xlate {
            keycode: 0o523 as libc::c_int,
            keyis: 'u' as i32 as byte,
        };
        init
    },
    {
        let mut init = xlate {
            keycode: 0o535 as libc::c_int,
            keyis: 'u' as i32 as byte,
        };
        init
    },
    {
        let mut init = xlate {
            keycode: 0o407 as libc::c_int,
            keyis: 'h' as i32 as byte,
        };
        init
    },
    {
        let mut init = xlate {
            keycode: 0o404 as libc::c_int,
            keyis: 'h' as i32 as byte,
        };
        init
    },
    {
        let mut init = xlate {
            keycode: 0o405 as libc::c_int,
            keyis: 'l' as i32 as byte,
        };
        init
    },
    {
        let mut init = xlate {
            keycode: 0o550 as libc::c_int,
            keyis: 'b' as i32 as byte,
        };
        init
    },
    {
        let mut init = xlate {
            keycode: 0o601 as libc::c_int,
            keyis: 'b' as i32 as byte,
        };
        init
    },
    {
        let mut init = xlate {
            keycode: 0o537 as libc::c_int,
            keyis: 'b' as i32 as byte,
        };
        init
    },
    {
        let mut init = xlate {
            keycode: 0o402 as libc::c_int,
            keyis: 'j' as i32 as byte,
        };
        init
    },
    {
        let mut init = xlate {
            keycode: 0o522 as libc::c_int,
            keyis: 'n' as i32 as byte,
        };
        init
    },
    {
        let mut init = xlate {
            keycode: 0o540 as libc::c_int,
            keyis: 'n' as i32 as byte,
        };
        init
    },
    {
        let mut init = xlate {
            keycode: 0o513 as libc::c_int,
            keyis: '>' as i32 as byte,
        };
        init
    },
    {
        let mut init = xlate {
            keycode: 0o512 as libc::c_int,
            keyis: 's' as i32 as byte,
        };
        init
    },
    {
        let mut init = xlate {
            keycode: 0o410 as libc::c_int + 1 as libc::c_int,
            keyis: '?' as i32 as byte,
        };
        init
    },
    {
        let mut init = xlate {
            keycode: 0o410 as libc::c_int + 2 as libc::c_int,
            keyis: '/' as i32 as byte,
        };
        init
    },
    {
        let mut init = xlate {
            keycode: 0o410 as libc::c_int + 3 as libc::c_int,
            keyis: 'a' as i32 as byte,
        };
        init
    },
    {
        let mut init = xlate {
            keycode: 0o410 as libc::c_int + 4 as libc::c_int,
            keyis: ('R' as i32 & 0o37 as libc::c_int) as byte,
        };
        init
    },
    {
        let mut init = xlate {
            keycode: 0o410 as libc::c_int + 5 as libc::c_int,
            keyis: 'c' as i32 as byte,
        };
        init
    },
    {
        let mut init = xlate {
            keycode: 0o410 as libc::c_int + 6 as libc::c_int,
            keyis: 'D' as i32 as byte,
        };
        init
    },
    {
        let mut init = xlate {
            keycode: 0o410 as libc::c_int + 7 as libc::c_int,
            keyis: 'i' as i32 as byte,
        };
        init
    },
    {
        let mut init = xlate {
            keycode: 0o410 as libc::c_int + 8 as libc::c_int,
            keyis: '^' as i32 as byte,
        };
        init
    },
    {
        let mut init = xlate {
            keycode: 0o410 as libc::c_int + 9 as libc::c_int,
            keyis: ('F' as i32 & 0o37 as libc::c_int) as byte,
        };
        init
    },
    {
        let mut init = xlate {
            keycode: 0o410 as libc::c_int + 10 as libc::c_int,
            keyis: '!' as i32 as byte,
        };
        init
    },
    {
        let mut init = xlate {
            keycode: 0o410 as libc::c_int + 57 as libc::c_int,
            keyis: 'F' as i32 as byte,
        };
        init
    },
];
#[no_mangle]
pub unsafe extern "C" fn cur_beep() {
    beep();
}
#[no_mangle]
pub unsafe extern "C" fn cur_getch_timeout(mut msdelay: libc::c_int) -> libc::c_int {
    let mut ch: libc::c_int = 0 as libc::c_int;
    wtimeout(stdscr, msdelay);
    let mut wchi: wint_t = 0;
    let mut ret: libc::c_int = 0;
    ret = wget_wch(stdscr, &mut wchi);
    if ret == -(1 as libc::c_int)
        || ret == 0 as libc::c_int
            && !(wchi & !(0x7f as libc::c_int) as libc::c_uint == 0 as libc::c_int as libc::c_uint)
    {
        ch = -(1 as libc::c_int);
    } else {
        ch = wchi as libc::c_int;
    }
    if ch != -(1 as libc::c_int) {
        ch = KEY_MASK & ch;
    }
    if ch == 0o632 as libc::c_int {
        resize_screen();
        ch = -(1 as libc::c_int);
    }
    nodelay(stdscr, 0 as libc::c_int != 0);
    return ch;
}
#[no_mangle]
pub unsafe extern "C" fn xlate_ch(mut ch: libc::c_int) -> byte {
    let mut x: *mut xlate = 0 as *mut xlate;
    x = xtab.as_mut_ptr();
    while x < xtab.as_mut_ptr().offset(
        (::core::mem::size_of::<[xlate; 29]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<xlate>() as libc::c_ulong) as isize,
    ) {
        if ch == (*x).keycode {
            ch = (*x).keyis as libc::c_int;
            break;
        } else {
            x = x.offset(1);
        }
    }
    return ch as byte;
}
#[no_mangle]
pub unsafe extern "C" fn cur_move(mut row: libc::c_int, mut col: libc::c_int) -> libc::c_int {
    return wmove(stdscr, row, col);
}
#[no_mangle]
pub unsafe extern "C" fn cur_inch() -> byte {
    let mut chd: byte = 0 as libc::c_int as byte;
    let mut ccp: *mut CCODE = 0 as *mut CCODE;
    let mut wch: wchar_t = 0;
    let mut wcr: wchar_t = 0;
    let mut cch: cchar_t = cchar_t {
        attr: 0,
        chars: [0; 5],
        ext_color: 0,
    };
    let mut wcha: [wchar_t; 6] = [0; 6];
    let mut dummya: attr_t = 0;
    let mut dummyc: libc::c_short = 0;
    win_wch(stdscr, &mut cch);
    getcchar(
        &mut cch,
        wcha.as_mut_ptr(),
        &mut dummya,
        &mut dummyc,
        0 as *mut libc::c_void,
    );
    wch = wcha[0 as libc::c_int as usize];
    chd = wch as byte;
    if charset == 2 as libc::c_int {
        return chd;
    }
    ccp = ctab.as_mut_ptr();
    while (*ccp).dos != 0 {
        match charset {
            1 => {
                wcr = (*ccp).ascii as wchar_t;
            }
            3 => {
                wcr = *(*ccp).unicode;
            }
            _ => {
                wcr = '\0' as i32;
            }
        }
        if wch == wcr {
            chd = (*ccp).dos;
            break;
        } else {
            ccp = ccp.offset(1);
        }
    }
    return chd;
}
#[no_mangle]
pub unsafe extern "C" fn cur_clear() {
    wclear(stdscr);
}
#[no_mangle]
pub unsafe extern "C" fn cursor(mut ison: bool) -> bool {
    let mut oldstate: libc::c_int = curs_set(ison as libc::c_int);
    if oldstate == 0 as libc::c_int || oldstate == -(1 as libc::c_int) {
        return 0 as libc::c_int != 0;
    } else {
        return 1 as libc::c_int != 0;
    };
}
#[no_mangle]
pub unsafe extern "C" fn getrc(mut rp: *mut libc::c_int, mut cp: *mut libc::c_int) {
    *rp = (if !(stdscr as *const libc::c_void).is_null() {
        (*stdscr)._cury as libc::c_int
    } else {
        -(1 as libc::c_int)
    });
    *cp = (if !(stdscr as *const libc::c_void).is_null() {
        (*stdscr)._curx as libc::c_int
    } else {
        -(1 as libc::c_int)
    });
}
#[no_mangle]
pub unsafe extern "C" fn cur_refresh() {
    wrefresh(stdscr);
}
#[no_mangle]
pub unsafe extern "C" fn cur_clrtoeol() {
    wclrtoeol(stdscr);
}
#[no_mangle]
pub unsafe extern "C" fn cur_mvaddstr(
    mut r: libc::c_int,
    mut c: libc::c_int,
    mut s: *mut libc::c_char,
) {
    cur_move(r, c);
    cur_addstr(s);
}
#[no_mangle]
pub unsafe extern "C" fn cur_mvaddch(mut r: libc::c_int, mut c: libc::c_int, mut chr: byte) {
    cur_move(r, c);
    cur_addch(chr);
}
#[no_mangle]
pub unsafe extern "C" fn cur_mvinch(mut r: libc::c_int, mut c: libc::c_int) -> byte {
    cur_move(r, c);
    return cur_inch();
}
#[no_mangle]
pub unsafe extern "C" fn cur_addch(mut chr: byte) {
    let mut old_attr: byte = 0;
    old_attr = ch_attr as byte;
    if at_table == color_attr.as_mut_ptr() {
        if ch_attr == 0x7 as libc::c_int {
            match chr as libc::c_int {
                206 | 186 | 205 | 201 | 187 | 200 | 188 => {
                    ch_attr = 0x2 as libc::c_int | 0x4 as libc::c_int;
                }
                250 => {
                    ch_attr = 0x2 as libc::c_int | 0x8 as libc::c_int;
                }
                240 => {
                    ch_attr = 0 as libc::c_int
                        | (0x2 as libc::c_int & 7 as libc::c_int) << 4 as libc::c_int
                        | 0x80 as libc::c_int;
                }
                4 => {
                    ch_attr = 0x1 as libc::c_int | 0x4 as libc::c_int;
                }
                15 | 1 => {
                    ch_attr = 0x2 as libc::c_int | 0x4 as libc::c_int | 0x8 as libc::c_int;
                }
                173 | 13 | 231 | 8 | 12 | 9 | 24 => {
                    ch_attr = 0x1 as libc::c_int | 0x8 as libc::c_int;
                }
                5 => {
                    ch_attr = 0x4 as libc::c_int;
                }
                _ => {}
            }
        } else if ch_attr == (0x7 as libc::c_int & 7 as libc::c_int) << 4 as libc::c_int {
            match chr as libc::c_int {
                5 => {
                    ch_attr = 0x4 as libc::c_int
                        | (0x7 as libc::c_int & 7 as libc::c_int) << 4 as libc::c_int;
                }
                15 | 1 => {
                    ch_attr = 0x2 as libc::c_int
                        | 0x4 as libc::c_int
                        | 0x8 as libc::c_int
                        | (0x7 as libc::c_int & 7 as libc::c_int) << 4 as libc::c_int;
                }
                173 | 13 | 231 | 8 | 12 | 9 | 24 => {
                    ch_attr = 0x1 as libc::c_int
                        | (0x7 as libc::c_int & 7 as libc::c_int) << 4 as libc::c_int;
                }
                _ => {}
            }
        } else if ch_attr == 0x8 as libc::c_int | 0x7 as libc::c_int
            && chr as libc::c_int == 0xf0 as libc::c_int
        {
            ch_attr = 0 as libc::c_int
                | (0x2 as libc::c_int & 7 as libc::c_int) << 4 as libc::c_int
                | 0x80 as libc::c_int;
        }
    }
    let mut current_block_23: u64;
    match charset {
        2 => {
            current_block_23 = 10686804971910976035;
        }
        3 => {
            wadd_wch(
                stdscr,
                unicode_from_dos(chr, ch_attr as byte, ctab.as_mut_ptr()),
            );
            current_block_23 = 15597372965620363352;
        }
        1 | _ => {
            chr = ascii_from_dos(chr, ctab.as_mut_ptr());
            current_block_23 = 10686804971910976035;
        }
    }
    match current_block_23 {
        10686804971910976035 => {
            waddch(stdscr, chr as libc::c_uint | attr_from_dos(ch_attr as byte));
        }
        _ => {}
    }
    ch_attr = old_attr as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn cur_addstr(mut s: *mut libc::c_char) {
    while *s != 0 {
        let fresh0 = s;
        s = s.offset(1);
        cur_addch(*fresh0 as byte);
    }
}
#[no_mangle]
pub unsafe extern "C" fn unicode_from_dos(
    mut chd: byte,
    mut dos_attr: byte,
    mut mapping: *mut CCODE,
) -> *mut cchar_t {
    let mut color: libc::c_short = 0;
    let mut attrs: attr_t = 0;
    let mut ccp: *mut CCODE = charcode_from_dos(chd, mapping);
    attrw_from_dos(dos_attr, &mut attrs, &mut color);
    setcchar(
        &mut cctemp,
        (*ccp).unicode,
        attrs,
        color,
        0 as *const libc::c_void,
    );
    return &mut cctemp;
}
#[no_mangle]
pub unsafe extern "C" fn define_keys() {
    let mut i: libc::c_int = 0;
    let mut shift: libc::c_int = 0;
    let mut ptr: *mut TTYSEQ = 0 as *mut TTYSEQ;
    let mut max: *mut TTYSEQ = 0 as *mut TTYSEQ;
    i = 0o777 as libc::c_int;
    shift = 1 as libc::c_int;
    loop {
        i >>= 1 as libc::c_int;
        if !(i != 0) {
            break;
        }
        shift += 1;
    }
    KEY_MASK = ((1 as libc::c_int) << shift) - 1 as libc::c_int;
    i = 8 as libc::c_int;
    ptr = ttymap.as_mut_ptr();
    max = ttymap.as_mut_ptr().offset(
        (::core::mem::size_of::<[TTYSEQ; 9]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<TTYSEQ>() as libc::c_ulong) as isize,
    );
    while ptr < max {
        if key_defined((*ptr).def) == 0 {
            let fresh1 = i;
            i = i + 1;
            define_key((*ptr).def, fresh1 << shift | (*ptr).dest);
        }
        ptr = ptr.offset(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn ascii_from_dos(mut chd: byte, mut mapping: *mut CCODE) -> byte {
    return (*charcode_from_dos(chd, mapping)).ascii;
}
#[no_mangle]
pub unsafe extern "C" fn charcode_from_dos(mut chd: byte, mut mapping: *mut CCODE) -> *mut CCODE {
    let mut ccp: *mut CCODE = 0 as *mut CCODE;
    if chd as libc::c_int == '\0' as i32
        || chd as libc::c_int == '\n' as i32
        || chd as libc::c_int & !(0x7f as libc::c_int) == 0 as libc::c_int
            && *(*__ctype_b_loc()).offset(chd as libc::c_int as isize) as libc::c_int
                & _ISprint as libc::c_int as libc::c_ushort as libc::c_int
                != 0
    {
        *ccode.unicode = chd as wchar_t;
        ccode.dos = *ccode.unicode as byte;
        ccode.ascii = ccode.dos;
        return &mut ccode;
    }
    ccp = mapping;
    while (*ccp).dos != 0 {
        if chd as libc::c_int == (*ccp).dos as libc::c_int {
            return ccp;
        }
        ccp = ccp.offset(1);
    }
    return ccp;
}
#[no_mangle]
pub unsafe extern "C" fn color_from_dos(mut dos_attr: byte, mut fg: bool) -> libc::c_short {
    let mut color: byte = (dos_attr as libc::c_int
        >> (if fg as libc::c_int != 0 {
            0 as libc::c_int
        } else {
            4 as libc::c_int
        })
        & 7 as libc::c_int) as byte;
    return swap_bits(
        color,
        0 as libc::c_int as libc::c_uint,
        2 as libc::c_int as libc::c_uint,
        1 as libc::c_int as libc::c_uint,
    ) as libc::c_short;
}
#[no_mangle]
pub unsafe extern "C" fn attr_from_dos(mut dos_attr: byte) -> chtype {
    let mut attr: chtype = (1 as libc::c_uint).wrapping_sub(1 as libc::c_uint)
        | (0 as libc::c_int as chtype) << 0 as libc::c_int + 8 as libc::c_int
            & ((1 as libc::c_uint) << 8 as libc::c_int).wrapping_sub(1 as libc::c_uint)
                << 0 as libc::c_int + 8 as libc::c_int;
    let mut fg: libc::c_short = 0;
    let mut bg: libc::c_short = 0;
    if dos_attr as libc::c_int == 0x7 as libc::c_int {
        return attr;
    }
    if dos_attr as libc::c_int & 0x80 as libc::c_int != 0 {
        attr |= (1 as libc::c_uint) << 11 as libc::c_int + 8 as libc::c_int;
    }
    fg = color_from_dos(dos_attr, 1 as libc::c_int != 0);
    bg = color_from_dos(dos_attr, 0 as libc::c_int != 0);
    if dos_attr as libc::c_int & 0x8 as libc::c_int != 0 {
        if colors < 16 as libc::c_int {
            attr |= (1 as libc::c_uint) << 13 as libc::c_int + 8 as libc::c_int;
        } else {
            fg = (fg as libc::c_int + 8 as libc::c_int) as libc::c_short;
        }
    }
    if dos_attr as libc::c_int & (0x7 as libc::c_int & 7 as libc::c_int) << 4 as libc::c_int
        == (0x7 as libc::c_int & 7 as libc::c_int) << 4 as libc::c_int
        && colors != 8 as libc::c_int
        && use_terminal_fgbg as libc::c_int != 0
    {
        attr |= (1 as libc::c_uint) << 10 as libc::c_int + 8 as libc::c_int;
        let mut tmp: libc::c_short = bg;
        bg = fg;
        fg = tmp;
    }
    if colors > 0 as libc::c_int {
        attr |= ((bg as libc::c_int * colors + fg as libc::c_int + 1 as libc::c_int) as chtype)
            << 0 as libc::c_int + 8 as libc::c_int
            & ((1 as libc::c_uint) << 8 as libc::c_int).wrapping_sub(1 as libc::c_uint)
                << 0 as libc::c_int + 8 as libc::c_int;
    }
    return attr;
}
#[no_mangle]
pub unsafe extern "C" fn attrw_from_dos(
    mut dos_attr: byte,
    mut attrs: *mut attr_t,
    mut color_pair: *mut libc::c_short,
) {
    let mut fg: libc::c_short = 0;
    let mut bg: libc::c_short = 0;
    *attrs = (1 as libc::c_uint).wrapping_sub(1 as libc::c_uint);
    *color_pair = 0 as libc::c_int as libc::c_short;
    if dos_attr as libc::c_int == 0x7 as libc::c_int {
        return;
    }
    if dos_attr as libc::c_int & 0x80 as libc::c_int != 0 {
        *attrs |= (1 as libc::c_uint) << 11 as libc::c_int + 8 as libc::c_int;
    }
    fg = color_from_dos(dos_attr, 1 as libc::c_int != 0);
    bg = color_from_dos(dos_attr, 0 as libc::c_int != 0);
    if dos_attr as libc::c_int & 0x8 as libc::c_int != 0 {
        if colors < 16 as libc::c_int {
            *attrs |= (1 as libc::c_uint) << 13 as libc::c_int + 8 as libc::c_int;
        } else {
            fg = (fg as libc::c_int + 8 as libc::c_int) as libc::c_short;
        }
    }
    if dos_attr as libc::c_int & (0x7 as libc::c_int & 7 as libc::c_int) << 4 as libc::c_int
        == (0x7 as libc::c_int & 7 as libc::c_int) << 4 as libc::c_int
        && colors != 8 as libc::c_int
        && use_terminal_fgbg as libc::c_int != 0
    {
        *attrs |= (1 as libc::c_uint) << 10 as libc::c_int + 8 as libc::c_int;
        let mut tmp: libc::c_short = bg;
        bg = fg;
        fg = tmp;
    }
    if colors > 0 as libc::c_int {
        *color_pair =
            (bg as libc::c_int * colors + fg as libc::c_int + 1 as libc::c_int) as libc::c_short;
    }
}
#[no_mangle]
pub unsafe extern "C" fn init_curses_colors() {
    let mut fg: libc::c_int = 0;
    let mut dos_fg: libc::c_int = 0;
    let mut dfg: libc::c_int = 0;
    let mut bg: libc::c_int = 0;
    let mut dos_bg: libc::c_int = 0;
    let mut dbg: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    let mut g: libc::c_int = 0;
    let mut b: libc::c_int = 0;
    let mut cube: libc::c_int = 0;
    let mut colormode: libc::c_int = 0;
    let mut cmap: [libc::c_int; 16] = [0; 16];
    if !has_colors() || COLORS < 8 as libc::c_int {
        colors = 0 as libc::c_int;
        return;
    }
    colors = 16 as libc::c_int;
    if COLORS >= 256 as libc::c_int {
        colormode = 256 as libc::c_int;
    } else if COLORS >= 88 as libc::c_int {
        colormode = 88 as libc::c_int;
    } else if COLORS >= 16 as libc::c_int {
        colormode = 16 as libc::c_int;
    } else {
        colors = 8 as libc::c_int;
        colormode = colors;
    }
    match colormode {
        8 | 16 => {
            cube = 0 as libc::c_int;
            if can_change_color() as libc::c_int != 0 && change_colors as libc::c_int != 0 {
                colors_changed = 1 as libc::c_int != 0;
            }
        }
        88 => {
            cube = 4 as libc::c_int;
        }
        256 => {
            cube = 6 as libc::c_int;
        }
        _ => {}
    }
    if cube != 0 {
        i = 0 as libc::c_int;
        while i < colors {
            r = ((cube - 1 as libc::c_int) as libc::c_double
                * (((i & 1 as libc::c_int != 0) as libc::c_int * 2 as libc::c_int)
                    as libc::c_double
                    / 3.0f64
                    + ((i & 8 as libc::c_int != 0) as libc::c_int * 1 as libc::c_int)
                        as libc::c_double
                        / 3.0f64)) as libc::c_int;
            g = ((cube - 1 as libc::c_int) as libc::c_double
                * ((((i & 2 as libc::c_int != 0) as libc::c_int * 2 as libc::c_int)
                    as libc::c_double
                    / 3.0f64
                    + ((i & 8 as libc::c_int != 0) as libc::c_int * 1 as libc::c_int)
                        as libc::c_double
                        / 3.0f64)
                    / (if i == 3 as libc::c_int {
                        2 as libc::c_int
                    } else {
                        1 as libc::c_int
                    }) as libc::c_double)) as libc::c_int;
            b = ((cube - 1 as libc::c_int) as libc::c_double
                * (((i & 4 as libc::c_int != 0) as libc::c_int * 2 as libc::c_int)
                    as libc::c_double
                    / 3.0f64
                    + ((i & 8 as libc::c_int != 0) as libc::c_int * 1 as libc::c_int)
                        as libc::c_double
                        / 3.0f64)) as libc::c_int;
            cmap[i as usize] = 16 as libc::c_int + cube * cube * r + cube * g + b;
            i += 1;
        }
    } else if colors_changed {
        i = 0 as libc::c_int;
        while i < colors {
            init_color(
                i as libc::c_short,
                (1000 as libc::c_int as libc::c_double
                    * (((i & 1 as libc::c_int != 0) as libc::c_int * 2 as libc::c_int)
                        as libc::c_double
                        / 3.0f64
                        + ((i & 8 as libc::c_int != 0) as libc::c_int * 1 as libc::c_int)
                            as libc::c_double
                            / 3.0f64)) as libc::c_short,
                (1000 as libc::c_int as libc::c_double
                    * ((((i & 2 as libc::c_int != 0) as libc::c_int * 2 as libc::c_int)
                        as libc::c_double
                        / 3.0f64
                        + ((i & 8 as libc::c_int != 0) as libc::c_int * 1 as libc::c_int)
                            as libc::c_double
                            / 3.0f64)
                        / (if i == 3 as libc::c_int {
                            2 as libc::c_int
                        } else {
                            1 as libc::c_int
                        }) as libc::c_double)) as libc::c_short,
                (1000 as libc::c_int as libc::c_double
                    * (((i & 4 as libc::c_int != 0) as libc::c_int * 2 as libc::c_int)
                        as libc::c_double
                        / 3.0f64
                        + ((i & 8 as libc::c_int != 0) as libc::c_int * 1 as libc::c_int)
                            as libc::c_double
                            / 3.0f64)) as libc::c_short,
            );
            cmap[i as usize] = i;
            i += 1;
        }
    } else {
        i = 0 as libc::c_int;
        while i < colors {
            cmap[i as usize] = i;
            i += 1;
        }
    }
    dos_fg = color_from_dos(0x7 as libc::c_int as byte, 1 as libc::c_int != 0) as libc::c_int;
    dos_bg = color_from_dos(0x7 as libc::c_int as byte, 0 as libc::c_int != 0) as libc::c_int;
    if 0x7 as libc::c_int & 0x8 as libc::c_int != 0 && colors > 8 as libc::c_int {
        dos_fg += 8 as libc::c_int;
    }
    dfg = cmap[7 as libc::c_int as usize];
    dbg = cmap[0 as libc::c_int as usize];
    if use_terminal_fgbg {
        use_default_colors();
        dbg = -(1 as libc::c_int);
        dfg = dbg;
    }
    bg = 0 as libc::c_int;
    while bg < colors {
        fg = colors
            - (if bg != 0 {
                2 as libc::c_int
            } else {
                1 as libc::c_int
            });
        while fg >= 0 as libc::c_int {
            init_pair(
                (bg * colors + fg + 1 as libc::c_int) as libc::c_short,
                (if fg == dos_fg { dfg } else { cmap[fg as usize] }) as libc::c_short,
                (if bg == dos_bg { dbg } else { cmap[bg as usize] }) as libc::c_short,
            );
            fg -= 1;
        }
        bg += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn resize_screen() {
    if LINES != cur_LINES || COLS != cur_COLS {
        if resizeterm(cur_LINES, cur_COLS) == 0 as libc::c_int {
            flushinp();
        } else {
            fatal(
                b"Could not resize resize terminal to %u x %u\n\0" as *const u8
                    as *const libc::c_char,
                cur_COLS,
                cur_LINES,
            );
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn set_attr(mut bute: libc::c_int) {
    if bute < 17 as libc::c_int {
        ch_attr = *at_table.offset(bute as isize) as libc::c_int;
    } else {
        ch_attr = bute;
    }
    wattrset(stdscr, attr_from_dos(ch_attr as byte) as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn winit() {
    if init_curses {
        return;
    }
    scr_type = 3 as libc::c_int;
    setenv(
        b"ESCDELAY\0" as *const u8 as *const libc::c_char,
        b"25\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
    );
    initscr();
    init_curses = 1 as libc::c_int != 0;
    if LINES < cur_LINES || COLS < cur_COLS {
        fatal(
            b"%u-column mode requires at least a %u x %u screen\nYour terminal size is %u x %u\n\0"
                as *const u8 as *const libc::c_char,
            cur_COLS,
            cur_COLS,
            cur_LINES,
            COLS,
            LINES,
        );
    }
    start_color();
    cbreak();
    noecho();
    nodelay(stdscr, 0 as libc::c_int != 0);
    keypad(stdscr, 1 as libc::c_int != 0);
    resize_screen();
    define_keys();
    init_curses_colors();
    at_table = if colors != 0 {
        color_attr.as_mut_ptr()
    } else {
        monoc_attr.as_mut_ptr()
    };
    if bwflag != 0 {
        at_table = monoc_attr.as_mut_ptr();
    }
}
#[no_mangle]
pub unsafe extern "C" fn wdump() {
    let mut line: libc::c_int = 0;
    let mut c_row: libc::c_int = 0;
    let mut c_col: libc::c_int = 0;
    c_row = (if !(stdscr as *const libc::c_void).is_null() {
        (*stdscr)._cury as libc::c_int
    } else {
        -(1 as libc::c_int)
    });
    c_col = (if !(stdscr as *const libc::c_void).is_null() {
        (*stdscr)._curx as libc::c_int
    } else {
        -(1 as libc::c_int)
    });
    line = 0 as libc::c_int;
    while line < LINES {
        if wmove(stdscr, line, 0 as libc::c_int) == -(1 as libc::c_int) {
        } else {
            win_wchnstr(stdscr, (savewin[line as usize]).as_mut_ptr(), COLS);
        };
        line += 1;
    }
    wmove(stdscr, c_row, c_col);
    is_saved = 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn wrestor() {
    let mut line: libc::c_int = 0;
    let mut c_row: libc::c_int = 0;
    let mut c_col: libc::c_int = 0;
    c_row = (if !(stdscr as *const libc::c_void).is_null() {
        (*stdscr)._cury as libc::c_int
    } else {
        -(1 as libc::c_int)
    });
    c_col = (if !(stdscr as *const libc::c_void).is_null() {
        (*stdscr)._curx as libc::c_int
    } else {
        -(1 as libc::c_int)
    });
    line = 0 as libc::c_int;
    while line < LINES {
        if wmove(stdscr, line, 0 as libc::c_int) == -(1 as libc::c_int) {
        } else {
            wadd_wchnstr(stdscr, (savewin[line as usize]).as_mut_ptr(), COLS);
        };
        line += 1;
    }
    wmove(stdscr, c_row, c_col);
    wrefresh(stdscr);
    is_saved = 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn cur_endwin() {
    if init_curses {
        endwin();
        init_curses = 0 as libc::c_int != 0;
    }
}
#[no_mangle]
pub unsafe extern "C" fn cur_line(
    mut chd: byte,
    mut length: libc::c_int,
    mut orientation: bool,
) -> libc::c_int {
    let mut ch: chtype = 0;
    let mut cch: *mut cchar_t = 0 as *mut cchar_t;
    let mut current_block_13: u64;
    match charset {
        2 => {
            current_block_13 = 12229348757182927719;
        }
        3 => {
            cch = unicode_from_dos(chd, ch_attr as byte, btab.as_mut_ptr());
            if (*cch).chars[0 as libc::c_int as usize] == '\0' as i32 {
                cch = unicode_from_dos(chd, ch_attr as byte, ctab.as_mut_ptr());
            }
            if orientation as libc::c_int == 0 as libc::c_int {
                wvline_set(stdscr, cch, length);
            } else {
                whline_set(stdscr, cch, length);
            }
            current_block_13 = 12599329904712511516;
        }
        1 | _ => {
            ch = ascii_from_dos(chd, btab.as_mut_ptr()) as chtype;
            if ch == '\0' as i32 as libc::c_uint {
                ch = ascii_from_dos(chd, ctab.as_mut_ptr()) as chtype;
            }
            chd = ch as byte;
            current_block_13 = 12229348757182927719;
        }
    }
    match current_block_13 {
        12229348757182927719 => {
            ch = chd as libc::c_uint | attr_from_dos(ch_attr as byte);
            if orientation as libc::c_int == 0 as libc::c_int {
                wvline(stdscr, ch, length);
            } else {
                whline(stdscr, ch, length);
            }
        }
        _ => {}
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn cur_box(
    mut ul_r: libc::c_int,
    mut ul_c: libc::c_int,
    mut lr_r: libc::c_int,
    mut lr_c: libc::c_int,
) {
    vbox(dbl_box.as_mut_ptr(), ul_r, ul_c, lr_r, lr_c);
}
#[no_mangle]
pub unsafe extern "C" fn vbox(
    mut box_0: *mut byte,
    mut ul_r: libc::c_int,
    mut ul_c: libc::c_int,
    mut lr_r: libc::c_int,
    mut lr_c: libc::c_int,
) {
    let mut wason: bool = false;
    let mut i: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    wason = cursor(0 as libc::c_int != 0);
    getrc(&mut r, &mut c);
    i = lr_c - ul_c - 1 as libc::c_int;
    if cur_move(ul_r, ul_c + 1 as libc::c_int) == -(1 as libc::c_int) {
    } else {
        cur_line(
            *box_0.offset(5 as libc::c_int as isize),
            i,
            1 as libc::c_int != 0,
        );
    };
    if cur_move(lr_r, ul_c + 1 as libc::c_int) == -(1 as libc::c_int) {
    } else {
        cur_line(
            *box_0.offset(6 as libc::c_int as isize),
            i,
            1 as libc::c_int != 0,
        );
    };
    i = lr_r - ul_r - 1 as libc::c_int;
    if cur_move(ul_r + 1 as libc::c_int, ul_c) == -(1 as libc::c_int) {
    } else {
        cur_line(
            *box_0.offset(4 as libc::c_int as isize),
            i,
            0 as libc::c_int != 0,
        );
    };
    if cur_move(ul_r + 1 as libc::c_int, lr_c) == -(1 as libc::c_int) {
    } else {
        cur_line(
            *box_0.offset(4 as libc::c_int as isize),
            i,
            0 as libc::c_int != 0,
        );
    };
    if cur_move(ul_r, ul_c) == -(1 as libc::c_int) {
    } else {
        cur_line(
            *box_0.offset(0 as libc::c_int as isize),
            1 as libc::c_int,
            1 as libc::c_int != 0,
        );
    };
    if cur_move(ul_r, lr_c) == -(1 as libc::c_int) {
    } else {
        cur_line(
            *box_0.offset(1 as libc::c_int as isize),
            1 as libc::c_int,
            1 as libc::c_int != 0,
        );
    };
    if cur_move(lr_r, ul_c) == -(1 as libc::c_int) {
    } else {
        cur_line(
            *box_0.offset(2 as libc::c_int as isize),
            1 as libc::c_int,
            1 as libc::c_int != 0,
        );
    };
    if cur_move(lr_r, lr_c) == -(1 as libc::c_int) {
    } else {
        cur_line(
            *box_0.offset(3 as libc::c_int as isize),
            1 as libc::c_int,
            1 as libc::c_int != 0,
        );
    };
    cur_move(r, c);
    cursor(wason);
}
#[no_mangle]
pub unsafe extern "C" fn center(mut row: libc::c_int, mut string: *mut libc::c_char) {
    cur_mvaddstr(
        row,
        (COLS as libc::c_ulong)
            .wrapping_sub(strlen(string))
            .wrapping_div(2 as libc::c_int as libc::c_ulong) as libc::c_int,
        string,
    );
}
#[no_mangle]
pub unsafe extern "C" fn cur_printw(mut msg: *const libc::c_char, mut args: ...) {
    let mut pwbuf: [libc::c_char; 132] = [0; 132];
    let mut argp: ::core::ffi::VaListImpl;
    argp = args.clone();
    vsnprintf(
        pwbuf.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 132]>() as libc::c_ulong,
        msg,
        argp.as_va_list(),
    );
    cur_addstr(pwbuf.as_mut_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn repchr(mut chr: byte, mut cnt: libc::c_int) {
    let mut c_row: libc::c_int = 0;
    let mut c_col: libc::c_int = 0;
    c_row = (if !(stdscr as *const libc::c_void).is_null() {
        (*stdscr)._cury as libc::c_int
    } else {
        -(1 as libc::c_int)
    });
    c_col = (if !(stdscr as *const libc::c_void).is_null() {
        (*stdscr)._curx as libc::c_int
    } else {
        -(1 as libc::c_int)
    });
    cur_line(chr, cnt, 1 as libc::c_int != 0);
    wmove(stdscr, c_row, c_col + cnt);
}
#[no_mangle]
pub unsafe extern "C" fn implode() {
    let mut j: libc::c_int = 0;
    let mut delay: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut cinc: libc::c_int = COLS / 10 as libc::c_int / 2 as libc::c_int;
    let mut er: libc::c_int = 0;
    let mut ec: libc::c_int = 0;
    er = if COLS == 80 as libc::c_int {
        LINES - 3 as libc::c_int
    } else {
        LINES - 4 as libc::c_int
    };
    delay = 50 as libc::c_int;
    r = 0 as libc::c_int;
    c = 0 as libc::c_int;
    ec = COLS - 1 as libc::c_int;
    while r < 10 as libc::c_int {
        vbox(sng_box.as_mut_ptr(), r, c, er, ec);
        wrefresh(stdscr);
        md_nanosleep(1000000 as libc::c_long * delay as libc::c_long);
        j = r + 1 as libc::c_int;
        while j <= er - 1 as libc::c_int {
            if cur_move(j, c + 1 as libc::c_int) == -(1 as libc::c_int) {
            } else {
                cur_line(
                    ' ' as i32 as byte,
                    cinc - 1 as libc::c_int,
                    1 as libc::c_int != 0,
                );
            };
            if cur_move(j, ec - cinc + 1 as libc::c_int) == -(1 as libc::c_int) {
            } else {
                cur_line(
                    ' ' as i32 as byte,
                    cinc - 1 as libc::c_int,
                    1 as libc::c_int != 0,
                );
            };
            j += 1;
        }
        vbox(spc_box.as_mut_ptr(), r, c, er, ec);
        r += 1;
        c += cinc;
        er -= 1;
        ec -= cinc;
    }
    wrefresh(stdscr);
}
#[no_mangle]
pub unsafe extern "C" fn drop_curtain() {
    let mut r: libc::c_int = 0;
    let mut delay: libc::c_int = 1500 as libc::c_int / LINES;
    cursor(0 as libc::c_int != 0);
    set_attr(1 as libc::c_int);
    vbox(
        sng_box.as_mut_ptr(),
        0 as libc::c_int,
        0 as libc::c_int,
        LINES - 1 as libc::c_int,
        COLS - 1 as libc::c_int,
    );
    if wmove(stdscr, 0 as libc::c_int, 0 as libc::c_int) == -(1 as libc::c_int) {
    } else {
        win_wchnstr(
            stdscr,
            (curtain[0 as libc::c_int as usize]).as_mut_ptr(),
            COLS,
        );
    };
    wrefresh(stdscr);
    md_nanosleep(1000000 as libc::c_long * delay as libc::c_long);
    set_attr(11 as libc::c_int);
    r = 1 as libc::c_int;
    while r < LINES - 1 as libc::c_int {
        if cur_move(r, 1 as libc::c_int) == -(1 as libc::c_int) {
        } else {
            cur_line(
                0xb1 as libc::c_int as byte,
                COLS - 2 as libc::c_int,
                1 as libc::c_int != 0,
            );
        };
        if wmove(stdscr, r, 0 as libc::c_int) == -(1 as libc::c_int) {
        } else {
            win_wchnstr(stdscr, (curtain[r as usize]).as_mut_ptr(), COLS);
        };
        wrefresh(stdscr);
        md_nanosleep(1000000 as libc::c_long * delay as libc::c_long);
        r += 1;
    }
    if wmove(stdscr, LINES - 1 as libc::c_int, 0 as libc::c_int) == -(1 as libc::c_int) {
    } else {
        win_wchnstr(
            stdscr,
            (curtain[(LINES - 1 as libc::c_int) as usize]).as_mut_ptr(),
            COLS,
        );
    };
    md_nanosleep(1000000 as libc::c_long * delay as libc::c_long);
    cur_move(0 as libc::c_int, 0 as libc::c_int);
    set_attr(0 as libc::c_int);
    wclear(stdscr);
}
#[no_mangle]
pub unsafe extern "C" fn raise_curtain() {
    let mut line: libc::c_int = 0;
    let mut c_row: libc::c_int = 0;
    let mut c_col: libc::c_int = 0;
    let mut delay: libc::c_int = 1500 as libc::c_int / LINES;
    c_row = (if !(stdscr as *const libc::c_void).is_null() {
        (*stdscr)._cury as libc::c_int
    } else {
        -(1 as libc::c_int)
    });
    c_col = (if !(stdscr as *const libc::c_void).is_null() {
        (*stdscr)._curx as libc::c_int
    } else {
        -(1 as libc::c_int)
    });
    wdump();
    line = 0 as libc::c_int;
    while line < LINES {
        if wmove(stdscr, line, 0 as libc::c_int) == -(1 as libc::c_int) {
        } else {
            wadd_wchnstr(stdscr, (curtain[line as usize]).as_mut_ptr(), COLS);
        };
        line += 1;
    }
    line = LINES - 1 as libc::c_int;
    while line >= 0 as libc::c_int {
        if wmove(stdscr, line, 0 as libc::c_int) == -(1 as libc::c_int) {
        } else {
            wadd_wchnstr(stdscr, (savewin[line as usize]).as_mut_ptr(), COLS);
        };
        wrefresh(stdscr);
        md_nanosleep(1000000 as libc::c_long * delay as libc::c_long);
        line -= 1;
    }
    wmove(stdscr, c_row, c_col);
    is_saved = 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn get_mode() -> byte {
    let mut regs: sw_regs = sw_regs {
        ax: 0,
        bx: 0,
        cx: 0,
        dx: 0,
        si: 0,
        di: 0,
        ds: 0,
        es: 0,
    };
    regs.ax = 0xf00 as libc::c_int;
    swint(0x10 as libc::c_int, &mut regs);
    return (0xff as libc::c_int & regs.ax) as byte;
}
#[no_mangle]
pub unsafe extern "C" fn video_mode(mut type_0: libc::c_int) -> byte {
    let mut regs: sw_regs = sw_regs {
        ax: 0,
        bx: 0,
        cx: 0,
        dx: 0,
        si: 0,
        di: 0,
        ds: 0,
        es: 0,
    };
    regs.ax = type_0;
    swint(0x10 as libc::c_int, &mut regs);
    return regs.ax as byte;
}
#[no_mangle]
pub unsafe extern "C" fn getinfo(mut str: *mut libc::c_char, mut size: libc::c_int) -> libc::c_int {
    let mut retstr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ch: libc::c_int = 0;
    let mut readcnt: libc::c_int = 0 as libc::c_int;
    let mut wason: libc::c_int = 0;
    let mut ret: libc::c_int = 1 as libc::c_int;
    retstr = str;
    *str = 0 as libc::c_int as libc::c_char;
    wason = cursor(1 as libc::c_int != 0) as libc::c_int;
    while ret == 1 as libc::c_int {
        loop {
            ch = wgetch(stdscr);
            if !(ch == -(1 as libc::c_int)) {
                break;
            }
        }
        if ch > 0o401 as libc::c_int {
            ch = KEY_MASK & ch;
        }
        match ch {
            27 => {
                while str != retstr {
                    backspace();
                    readcnt -= 1;
                    str = str.offset(-1);
                }
                let fresh2 = str;
                str = str.offset(1);
                *fresh2 = 27 as libc::c_int as libc::c_char;
                ret = *fresh2 as libc::c_int;
                *str = 0 as libc::c_int as libc::c_char;
                cursor(wason != 0);
            }
            410 => {
                resize_screen();
            }
            263 | 8 => {
                if str != retstr {
                    backspace();
                    readcnt -= 1;
                    str = str.offset(-1);
                }
            }
            343 | 10 => {
                *str = 0 as libc::c_int as libc::c_char;
                cursor(wason != 0);
                ret = ch;
            }
            _ => {
                if readcnt >= size {
                    beep();
                } else if !(*(*__ctype_b_loc()).offset(ch as isize) as libc::c_int
                    & _ISprint as libc::c_int as libc::c_ushort as libc::c_int
                    == 0)
                {
                    readcnt += 1;
                    waddch(stdscr, ch as chtype);
                    let fresh3 = str;
                    str = str.offset(1);
                    *fresh3 = ch as libc::c_char;
                }
            }
        }
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn backspace() {
    let mut r: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    r = (if !(stdscr as *const libc::c_void).is_null() {
        (*stdscr)._cury as libc::c_int
    } else {
        -(1 as libc::c_int)
    });
    c = (if !(stdscr as *const libc::c_void).is_null() {
        (*stdscr)._curx as libc::c_int
    } else {
        -(1 as libc::c_int)
    });
    if c > 0 as libc::c_int {
        wmove(stdscr, r, c - 1 as libc::c_int);
    }
    wdelch(stdscr);
    winsch(stdscr, ' ' as i32 as chtype);
}
unsafe extern "C" fn run_static_initializers() {
    cur_LINES = if (25 as libc::c_int) < 25 as libc::c_int {
        25 as libc::c_int
    } else {
        25 as libc::c_int
    };
    cur_COLS = if (80 as libc::c_int) < 80 as libc::c_int {
        80 as libc::c_int
    } else {
        80 as libc::c_int
    };
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
