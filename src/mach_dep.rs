use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type _XDisplay;
    static mut maxrow: libc::c_int;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    static mut stdout: *mut FILE;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn vfprintf(_: *mut FILE, _: *const libc::c_char, _: ::core::ffi::VaList) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn exit(_: libc::c_int) -> !;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn pause() -> libc::c_int;
    fn time(__timer: *mut time_t) -> time_t;
    fn localtime(__timer: *const time_t) -> *mut tm;
    fn nanosleep(__requested_time: *const timespec, __remaining: *mut timespec) -> libc::c_int;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn ioctl(__fd: libc::c_int, __request: libc::c_ulong, _: ...) -> libc::c_int;
    fn quit();
    static mut terse: bool;
    static mut expert: bool;
    static mut typebuf: *mut libc::c_char;
    static mut whoami: [libc::c_char; 0];
    static mut hit_mul: libc::c_int;
    static mut your_na: *mut libc::c_char;
    static mut prbuf: *mut libc::c_char;
    static mut kild_by: *mut libc::c_char;
    static mut goodchk: libc::c_int;
    static mut no_step: libc::c_int;
    fn free_ds();
    static mut count: libc::c_int;
    fn SIG2();
    fn cur_mvaddch(r: libc::c_int, c: libc::c_int, chr: byte);
    static mut scr_type: libc::c_int;
    static mut COLS: libc::c_int;
    fn cur_clrtoeol();
    fn cur_refresh();
    fn xlate_ch(ch: libc::c_int) -> byte;
    static mut is_saved: libc::c_int;
    fn cur_mvaddstr(r: libc::c_int, c: libc::c_int, s: *mut libc::c_char);
    fn cur_endwin();
    fn cur_addch(chr: byte);
    fn cursor(ison: bool) -> bool;
    fn cur_clear();
    fn set_attr(bute: libc::c_int);
    fn cur_box(ul_r: libc::c_int, ul_c: libc::c_int, lr_r: libc::c_int, lr_c: libc::c_int);
    static mut LINES: libc::c_int;
    fn center(row: libc::c_int, string: *mut libc::c_char);
    fn getinfo(str: *mut libc::c_char, size: libc::c_int) -> libc::c_int;
    fn repchr(chr: byte, cnt: libc::c_int);
    fn cur_getch_timeout(msdelay: libc::c_int) -> libc::c_int;
    fn cur_move(row: libc::c_int, col: libc::c_int) -> libc::c_int;
    fn XOpenDisplay(_: *const libc::c_char) -> *mut Display;
    fn XCloseDisplay(_: *mut Display) -> libc::c_int;
    fn XGetKeyboardControl(_: *mut Display, _: *mut XKeyboardState) -> libc::c_int;
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
pub type __uint16_t = libc::c_ushort;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type uint16_t = __uint16_t;
pub type uintptr_t = libc::c_ulong;
pub type size_t = libc::c_ulong;
pub type va_list = __builtin_va_list;
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
pub type time_t = __time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tm {
    pub tm_sec: libc::c_int,
    pub tm_min: libc::c_int,
    pub tm_hour: libc::c_int,
    pub tm_mday: libc::c_int,
    pub tm_mon: libc::c_int,
    pub tm_year: libc::c_int,
    pub tm_wday: libc::c_int,
    pub tm_yday: libc::c_int,
    pub tm_isdst: libc::c_int,
    pub tm_gmtoff: libc::c_long,
    pub tm_zone: *const libc::c_char,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct md_tm {
    pub second: libc::c_int,
    pub minute: libc::c_int,
    pub hour: libc::c_int,
    pub day: libc::c_int,
    pub month: libc::c_int,
    pub year: libc::c_int,
}
pub type TM = md_tm;
pub type intptr = uintptr_t;
pub type dosptr = uint16_t;
pub type byte = libc::c_uchar;
pub type Display = _XDisplay;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XKeyboardState {
    pub key_click_percent: libc::c_int,
    pub bell_percent: libc::c_int,
    pub bell_pitch: libc::c_uint,
    pub bell_duration: libc::c_uint,
    pub led_mask: libc::c_ulong,
    pub global_auto_repeat: libc::c_int,
    pub auto_repeats: [libc::c_char; 32],
}
#[inline]
unsafe extern "C" fn vprintf(
    mut __fmt: *const libc::c_char,
    mut __arg: ::core::ffi::VaList,
) -> libc::c_int {
    return vfprintf(stdout, __fmt, __arg.as_va_list());
}
#[no_mangle]
pub static mut xdisplay: *mut Display = 0 as *const Display as *mut Display;
static mut ocb: libc::c_int = 0;
static mut _treg: sw_regs = sw_regs {
    ax: 0,
    bx: 0,
    cx: 0,
    dx: 0,
    si: 0,
    di: 0,
    ds: 0,
    es: 0,
};
#[no_mangle]
pub static mut regs: *mut sw_regs = unsafe { &_treg as *const sw_regs as *mut sw_regs };
#[no_mangle]
pub static mut current_drive: libc::c_int = 'C' as i32 - 'A' as i32;
#[no_mangle]
pub static mut last_drive: libc::c_int = 'F' as i32 - 'A' as i32;
#[no_mangle]
pub unsafe extern "C" fn swap_bits(
    mut data: byte,
    mut i: libc::c_uint,
    mut j: libc::c_uint,
    mut length: libc::c_uint,
) -> byte {
    let mut x: byte = ((data as libc::c_int >> i ^ data as libc::c_int >> j) as libc::c_uint
        & ((1 as libc::c_uint) << length).wrapping_sub(1 as libc::c_int as libc::c_uint))
        as byte;
    return (data as libc::c_int ^ ((x as libc::c_int) << i | (x as libc::c_int) << j)) as byte;
}
#[no_mangle]
pub unsafe extern "C" fn md_keyboard_leds() -> libc::c_int {
    let mut state: libc::c_int = 0 as libc::c_int;
    let mut fd: libc::c_int = 0;
    let mut kbstate: XKeyboardState = XKeyboardState {
        key_click_percent: 0,
        bell_percent: 0,
        bell_pitch: 0,
        bell_duration: 0,
        led_mask: 0,
        global_auto_repeat: 0,
        auto_repeats: [0; 32],
    };
    if !xdisplay.is_null() {
        XGetKeyboardControl(xdisplay, &mut kbstate);
        state = swap_bits(
            kbstate.led_mask as byte,
            0 as libc::c_int as libc::c_uint,
            2 as libc::c_int as libc::c_uint,
            1 as libc::c_int as libc::c_uint,
        ) as libc::c_int;
    } else {
        fd = open(
            b"/dev/tty\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int | 0o400 as libc::c_int,
        );
        if fd == -(1 as libc::c_int)
            || ioctl(
                fd,
                0x4b64 as libc::c_int as libc::c_ulong,
                &mut state as *mut libc::c_int,
            ) == -(1 as libc::c_int)
        {
            state = 0 as libc::c_int;
        }
        close(fd);
    }
    return state << 4 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn csum() -> libc::c_int {
    return -(1632 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn pokeb(
    mut _offset: libc::c_int,
    mut _segment: libc::c_int,
    mut _value: byte,
) {
}
#[no_mangle]
pub unsafe extern "C" fn peekb(mut _offset: libc::c_int, mut _segment: libc::c_int) -> byte {
    return 0 as libc::c_int as byte;
}
#[no_mangle]
pub unsafe extern "C" fn out(mut _port: libc::c_int, mut _value: byte) {}
#[export_name = "in"]
pub unsafe extern "C" fn in_0(mut _port: libc::c_int) -> byte {
    return 0 as libc::c_int as byte;
}
#[no_mangle]
pub unsafe extern "C" fn dmaout(
    mut _data: *mut libc::c_void,
    mut _wordlength: libc::c_uint,
    mut _segment: libc::c_uint,
    mut _offset: libc::c_uint,
) {
}
#[no_mangle]
pub unsafe extern "C" fn dmain(
    mut _buffer: *mut libc::c_void,
    mut _wordlength: libc::c_uint,
    mut _segment: libc::c_uint,
    mut _offset: libc::c_uint,
) {
}
#[no_mangle]
pub unsafe extern "C" fn _halt() {
    cur_endwin();
    printf(b"HALT!\n\0" as *const u8 as *const libc::c_char);
    pause();
}
#[no_mangle]
pub unsafe extern "C" fn COFF() {
    let mut reg: sw_regs = sw_regs {
        ax: 0,
        bx: 0,
        cx: 0,
        dx: 0,
        si: 0,
        di: 0,
        ds: 0,
        es: 0,
    };
    reg.ax = 0x2523 as libc::c_int;
    reg.ds = 0x33 as libc::c_int;
    reg.dx = ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, intptr>(Some(
        quit as unsafe extern "C" fn() -> (),
    )) as dosptr as libc::c_int;
    swint(0x21 as libc::c_int, &mut reg);
}
#[no_mangle]
pub unsafe extern "C" fn setup() {
    terse = 0 as libc::c_int != 0;
    maxrow = 23 as libc::c_int;
    if COLS == 40 as libc::c_int {
        maxrow = 22 as libc::c_int;
        terse = 1 as libc::c_int != 0;
    }
    expert = terse;
    COFF();
    ocb = set_ctrlb(0 as libc::c_int) as libc::c_int;
    if !(getenv(b"DISPLAY\0" as *const u8 as *const libc::c_char)).is_null() {
        xdisplay = XOpenDisplay(0 as *const libc::c_char);
    }
}
#[no_mangle]
pub unsafe extern "C" fn md_clock() {
    if no_step != 0 && {
        no_step += 1;
        no_step > 20 as libc::c_int
    } {
        _halt();
    }
    if hit_mul != 1 as libc::c_int && goodchk == 0xd0d as libc::c_int {
        kild_by = prbuf;
        your_na = whoami.as_mut_ptr();
        hit_mul = 1 as libc::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn md_time() -> libc::c_long {
    return time(0 as *mut time_t);
}
#[no_mangle]
pub unsafe extern "C" fn md_localtime() -> *mut TM {
    static mut md_local: TM = TM {
        second: 0,
        minute: 0,
        hour: 0,
        day: 0,
        month: 0,
        year: 0,
    };
    let mut secs: time_t = time(0 as *mut time_t);
    let mut local: *mut tm = localtime(&mut secs);
    md_local.second = (*local).tm_sec;
    md_local.minute = (*local).tm_min;
    md_local.hour = (*local).tm_hour;
    md_local.day = (*local).tm_mday;
    md_local.month = (*local).tm_mon;
    md_local.year = (*local).tm_year + 1900 as libc::c_int;
    return &mut md_local;
}
#[no_mangle]
pub unsafe extern "C" fn md_nanosleep(mut nanoseconds: libc::c_long) {
    let mut ts: timespec = {
        let mut init = timespec {
            tv_sec: 0 as libc::c_int as __time_t,
            tv_nsec: nanoseconds,
        };
        init
    };
    nanosleep(&mut ts, 0 as *mut timespec);
}
#[no_mangle]
pub unsafe extern "C" fn md_srand() -> libc::c_int {
    return md_time() as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn flush_type() {
    typebuf = b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn credits() {
    let mut tname: [libc::c_char; 25] = [0; 25];
    cursor(0 as libc::c_int != 0);
    cur_clear();
    if scr_type != 7 as libc::c_int {
        set_attr(5 as libc::c_int);
    }
    cur_box(
        0 as libc::c_int,
        0 as libc::c_int,
        LINES - 1 as libc::c_int,
        COLS - 1 as libc::c_int,
    );
    set_attr(16 as libc::c_int);
    center(
        2 as libc::c_int,
        b"ROGUE:  The Adventure Game\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if scr_type != 7 as libc::c_int {
        set_attr(10 as libc::c_int);
    } else {
        set_attr(12 as libc::c_int);
    }
    center(
        4 as libc::c_int,
        b"The game of Rogue was designed by:\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    set_attr(15 as libc::c_int);
    center(
        6 as libc::c_int,
        b"Michael Toy and Glenn Wichman\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if scr_type != 7 as libc::c_int {
        set_attr(10 as libc::c_int);
    } else {
        set_attr(12 as libc::c_int);
    }
    center(
        9 as libc::c_int,
        b"Various implementations by:\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    set_attr(15 as libc::c_int);
    center(
        11 as libc::c_int,
        b"Ken Arnold, Jon Lane and Michael Toy\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    if scr_type != 7 as libc::c_int {
        set_attr(10 as libc::c_int);
    } else {
        set_attr(12 as libc::c_int);
    }
    center(
        14 as libc::c_int,
        b"Adapted for the IBM PC by:\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    set_attr(15 as libc::c_int);
    center(
        16 as libc::c_int,
        b"A.I. Design\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if scr_type != 7 as libc::c_int {
        set_attr(10 as libc::c_int);
    } else {
        set_attr(12 as libc::c_int);
    }
    if scr_type != 7 as libc::c_int {
        set_attr(11 as libc::c_int);
    }
    center(
        19 as libc::c_int,
        b"(C)Copyright 1985\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    set_attr(15 as libc::c_int);
    center(
        20 as libc::c_int,
        b"Epyx Incorporated\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    set_attr(0 as libc::c_int);
    if scr_type != 7 as libc::c_int {
        set_attr(11 as libc::c_int);
    }
    center(
        21 as libc::c_int,
        b"All Rights Reserved\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if scr_type != 7 as libc::c_int {
        set_attr(5 as libc::c_int);
    }
    cur_move(22 as libc::c_int, 0 as libc::c_int);
    cur_addch(0xcc as libc::c_int as byte);
    repchr(0xcd as libc::c_int as byte, COLS - 2 as libc::c_int);
    cur_addch(0xb9 as libc::c_int as byte);
    set_attr(0 as libc::c_int);
    cur_mvaddstr(
        23 as libc::c_int,
        2 as libc::c_int,
        b"Rogue's Name? \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    is_saved = 1 as libc::c_int;
    set_attr(15 as libc::c_int);
    getinfo(tname.as_mut_ptr(), 23 as libc::c_int);
    if *tname.as_mut_ptr() as libc::c_int != 0
        && *tname.as_mut_ptr() as libc::c_int != 27 as libc::c_int
    {
        strcpy(whoami.as_mut_ptr(), tname.as_mut_ptr());
    }
    is_saved = 0 as libc::c_int;
    cur_move(23 as libc::c_int, 0 as libc::c_int);
    cur_clrtoeol();
    cur_move(24 as libc::c_int, 0 as libc::c_int);
    cur_clrtoeol();
    if scr_type != 7 as libc::c_int {
        set_attr(5 as libc::c_int);
    }
    cur_mvaddch(
        22 as libc::c_int,
        0 as libc::c_int,
        0xc8 as libc::c_int as byte,
    );
    cur_mvaddch(
        22 as libc::c_int,
        COLS - 1 as libc::c_int,
        0xbc as libc::c_int as byte,
    );
    set_attr(0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn readchar() -> byte {
    let mut xch: libc::c_int = 0;
    let mut ch: byte = 0;
    if *typebuf != 0 {
        SIG2();
        cur_refresh();
        let fresh0 = typebuf;
        typebuf = typebuf.offset(1);
        return *fresh0 as byte;
    }
    loop {
        SIG2();
        cur_refresh();
        xch = cur_getch_timeout(250 as libc::c_int);
        if !(xch == -(1 as libc::c_int)) {
            break;
        }
    }
    ch = xlate_ch(xch);
    if ch as libc::c_int == 27 as libc::c_int {
        count = 0 as libc::c_int;
    }
    return ch;
}
#[no_mangle]
pub unsafe extern "C" fn bdos(mut fnum: libc::c_int, mut dxval: libc::c_int) -> libc::c_int {
    let mut saveptr: *mut sw_regs = 0 as *mut sw_regs;
    (*regs).ax = fnum << 8 as libc::c_int;
    (*regs).cx = 0 as libc::c_int;
    (*regs).bx = (*regs).cx;
    (*regs).dx = dxval;
    saveptr = regs;
    swint(0x21 as libc::c_int, regs);
    regs = saveptr;
    return 0xff as libc::c_int & (*regs).ax;
}
#[no_mangle]
pub unsafe extern "C" fn newmem(mut nbytes: libc::c_uint) -> *mut libc::c_char {
    let mut newaddr: *mut libc::c_void = 0 as *mut libc::c_void;
    newaddr = malloc(nbytes as libc::c_ulong) as *mut libc::c_char as *mut libc::c_void;
    if newaddr.is_null() {
        fatal(b"No Memory\0" as *const u8 as *const libc::c_char);
    }
    return newaddr as *mut libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn swint(mut intno: libc::c_int, mut rp: *mut sw_regs) -> libc::c_int {
    let mut _dsval: libc::c_int = 0 as libc::c_int;
    (*rp).es = _dsval;
    (*rp).ds = (*rp).es;
    sysint(intno, rp, rp);
    return (*rp).ax;
}
#[no_mangle]
pub unsafe extern "C" fn sysint(
    mut _intno: libc::c_int,
    mut inregs: *mut sw_regs,
    mut outregs: *mut sw_regs,
) -> libc::c_int {
    (*outregs).ax = 0 as libc::c_int;
    (*outregs).bx = 0 as libc::c_int;
    (*outregs).cx = 0 as libc::c_int;
    (*outregs).dx = 0 as libc::c_int;
    (*outregs).si = (*inregs).si;
    (*outregs).di = (*inregs).di;
    (*outregs).ds = (*inregs).ds;
    (*outregs).es = (*inregs).es;
    return 0xf22a as libc::c_int;
}
// Rust port: Fixed parameter, which was a weakly typed bool.
#[no_mangle]
pub unsafe extern "C" fn set_ctrlb(mut state: libc::c_int) -> bool {
    let mut rg: sw_regs = sw_regs {
        ax: 0,
        bx: 0,
        cx: 0,
        dx: 0,
        si: 0,
        di: 0,
        ds: 0,
        es: 0,
    };
    let mut retcode: libc::c_int = 0;
    rg.ax = 0x3300 as libc::c_int;
    swint(0x21 as libc::c_int, &mut rg);
    retcode = rg.dx & 0xff as libc::c_int;
    rg.ax = 0x3300 as libc::c_int;
    rg.dx = state as libc::c_int;
    swint(0x21 as libc::c_int, &mut rg);
    return retcode != 0;
}
#[no_mangle]
pub unsafe extern "C" fn unsetup() {
    set_ctrlb(ocb);
    if !xdisplay.is_null() {
        XCloseDisplay(xdisplay);
        xdisplay = 0 as *mut Display;
    }
}
#[no_mangle]
pub unsafe extern "C" fn one_tick() {
    md_nanosleep(1000000 as libc::c_long * 27 as libc::c_int as libc::c_long);
    md_clock();
}
#[no_mangle]
pub unsafe extern "C" fn fatal(mut msg: *const libc::c_char, mut args: ...) {
    let mut argp: ::core::ffi::VaListImpl;
    cur_endwin();
    argp = args.clone();
    vprintf(msg, argp.as_va_list());
    md_exit(0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn md_exit(mut status: libc::c_int) {
    cur_endwin();
    unsetup();
    free_ds();
    exit(status);
}
