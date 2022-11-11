use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn remove(__filename: *const libc::c_char) -> libc::c_int;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn __errno_location() -> *mut libc::c_int;
    fn md_srand() -> libc::c_int;
    fn readchar() -> byte;
    fn fatal(msg_0: *const libc::c_char, _: ...);
    fn md_exit(status: libc::c_int);
    static mut regs: *mut sw_regs;
    static mut revno: libc::c_int;
    static mut verno: libc::c_int;
    static mut terse: bool;
    static mut dnum: libc::c_int;
    static mut mpos: libc::c_int;
    static mut s_save: [libc::c_char; 0];
    static mut s_drive: [libc::c_char; 0];
    static mut whoami: [libc::c_char; 0];
    fn init_ds();
    fn ifterse(tfmt: *const libc::c_char, fmt: *const libc::c_char, _: ...);
    fn msg(fmt: *const libc::c_char, _: ...);
    fn wait_for(ch: byte);
    fn noterse(str: *mut libc::c_char) -> *mut libc::c_char;
    fn cur_clrtoeol();
    fn cur_addstr(s: *mut libc::c_char);
    fn winit();
    fn wdump();
    fn wrestor();
    fn cur_endwin();
    fn cur_printw(msg_0: *const libc::c_char, _: ...);
    fn cur_move(row: libc::c_int, col: libc::c_int) -> libc::c_int;
    fn getinfo(str: *mut libc::c_char, size: libc::c_int) -> libc::c_int;
    static mut COLS: libc::c_int;
    static mut scr_type: libc::c_int;
    static mut savewin: [libc::c_char; 0];
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
#[no_mangle]
pub static mut dummy: [libc::c_char; 1234] = [0; 1234];
#[no_mangle]
pub static mut _lowmem: *mut libc::c_char = unsafe { dummy.as_ptr() as *mut _ };
#[no_mangle]
pub static mut _Uend: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut dummier: [libc::c_char; 4321] = [0; 4321];
#[no_mangle]
pub static mut end_sb: *mut libc::c_char = unsafe { dummier.as_ptr() as *mut _ };
#[no_mangle]
pub static mut startmem: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut msaveid: *mut libc::c_char =
    b"AI Design\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub unsafe extern "C" fn save_game() {
    let mut retcode: libc::c_int = 0;
    let mut savename: [libc::c_char; 20] = [0; 20];
    cur_printw(
        b"Sorry, saving games is disabled. Patches are welcome!\0" as *const u8
            as *const libc::c_char,
    );
}
unsafe extern "C" fn save_ds(mut savename: *mut libc::c_char) -> libc::c_int {
    let mut file: *mut FILE = 0 as *mut FILE;
    let mut answer: libc::c_char = 0;
    file = fopen(savename, b"r\0" as *const u8 as *const libc::c_char);
    if !file.is_null() {
        fclose(file);
        msg(
            b"%s %sexists, overwrite (y/n) ?\0" as *const u8 as *const libc::c_char,
            savename,
            noterse(b"already \0" as *const u8 as *const libc::c_char as *mut libc::c_char),
        );
        answer = readchar() as libc::c_char;
        msg(b"\0" as *const u8 as *const libc::c_char);
        if answer as libc::c_int != 'y' as i32 && answer as libc::c_int != 'Y' as i32 {
            return -(2 as libc::c_int);
        }
    }
    file = fopen(savename, b"w\0" as *const u8 as *const libc::c_char);
    if file.is_null() {
        msg(
            b"Could not create %s\0" as *const u8 as *const libc::c_char,
            savename,
        );
        return -(2 as libc::c_int);
    }
    mpos = 0 as libc::c_int;
    *__errno_location() = 1 as libc::c_int;
    if !(fwrite(
        msaveid as *const libc::c_void,
        10 as libc::c_int as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        file,
    ) == 0
        || fwrite(
            &mut _lowmem as *mut *mut libc::c_char as *const libc::c_void,
            (&mut _Uend as *mut *mut libc::c_char).offset_from(&mut _lowmem) as libc::c_long
                as libc::c_ulong,
            1 as libc::c_int as libc::c_ulong,
            file,
        ) == 0
        || fwrite(
            end_sb as *const libc::c_void,
            startmem.offset_from(end_sb) as libc::c_long as libc::c_ulong,
            1 as libc::c_int as libc::c_ulong,
            file,
        ) == 0)
    {
        wdump();
        if fwrite(
            savewin.as_mut_ptr() as *const libc::c_void,
            4000 as libc::c_int as libc::c_ulong,
            1 as libc::c_int as libc::c_ulong,
            file,
        ) != 0
        {
            *__errno_location() = 0 as libc::c_int;
        }
        wrestor();
    }
    fclose(file);
    match *__errno_location() {
        0 => {
            cur_move(24 as libc::c_int, 0 as libc::c_int);
            cur_clrtoeol();
            cur_move(23 as libc::c_int, 0 as libc::c_int);
            return 1 as libc::c_int;
        }
        _ => {
            msg(b"Could not write savefile to disk!\0" as *const u8 as *const libc::c_char);
            return -(1 as libc::c_int);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn restore(mut savefile: *mut libc::c_char) {
    let mut current_block: u64;
    fatal(
        b"Sorry, restoring games is disabled. Patches are welcome!\n\0" as *const u8
            as *const libc::c_char,
    );
    let mut oldrev: libc::c_int = 0;
    let mut oldver: libc::c_int = 0;
    let mut oldcols: libc::c_int = 0;
    let mut file: *mut FILE = 0 as *mut FILE;
    let mut errbuf: [libc::c_char; 11] = [0; 11];
    let mut save_name: [libc::c_char; 80] = [0; 80];
    let mut read_error: *mut libc::c_char =
        b"Read Error\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    let mut oregs: *mut sw_regs = 0 as *mut sw_regs;
    let mut nbytes: libc::c_uint = 0;
    let mut idbuf: [libc::c_char; 10] = [0; 10];
    oregs = regs;
    winit();
    strcpy(errbuf.as_mut_ptr(), read_error);
    oldrev = revno;
    oldver = verno;
    if strcmp(
        s_drive.as_mut_ptr(),
        b"?\0" as *const u8 as *const libc::c_char,
    ) == 0
    {
        let mut ot: libc::c_int = scr_type;
        cur_printw(b"Press space to restart game\0" as *const u8 as *const libc::c_char);
        scr_type = -(1 as libc::c_int);
        wait_for(' ' as i32 as byte);
        scr_type = ot;
        cur_addstr(b"\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    }
    file = fopen(savefile, b"r\0" as *const u8 as *const libc::c_char);
    if file.is_null() {
        fatal(
            b"%s not found\n\0" as *const u8 as *const libc::c_char,
            savefile,
        );
    } else {
        cur_printw(
            b"Restoring %s\0" as *const u8 as *const libc::c_char,
            savefile,
        );
    }
    strcpy(save_name.as_mut_ptr(), savefile);
    nbytes = (&mut _Uend as *mut *mut libc::c_char).offset_from(&mut _lowmem) as libc::c_long
        as libc::c_uint;
    if fread(
        idbuf.as_mut_ptr() as *mut libc::c_void,
        10 as libc::c_int as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        file,
    ) != 0
        || strcmp(idbuf.as_mut_ptr(), msaveid) != 0
    {
        cur_addstr(
            b"\nNot a savefile\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        current_block = 13797916685926291137;
    } else {
        if fread(
            &mut _lowmem as *mut *mut libc::c_char as *mut libc::c_void,
            nbytes as libc::c_ulong,
            1 as libc::c_int as libc::c_ulong,
            file,
        ) != 0
        {
            if fread(
                end_sb as *mut libc::c_void,
                startmem.offset_from(end_sb) as libc::c_long as libc::c_ulong,
                1 as libc::c_int as libc::c_ulong,
                file,
            ) != 0
            {
                current_block = 2869969976696497411;
            } else {
                current_block = 15904375183555213903;
            }
        } else {
            current_block = 15904375183555213903;
        }
        match current_block {
            2869969976696497411 => {}
            _ => {
                cur_addstr(errbuf.as_mut_ptr());
                current_block = 13797916685926291137;
            }
        }
    }
    match current_block {
        13797916685926291137 => {
            fclose(file);
            md_exit(1 as libc::c_int);
        }
        _ => {}
    }
    regs = oregs;
    if revno != oldrev || verno != oldver {
        fclose(file);
        md_exit(1 as libc::c_int);
    }
    oldcols = COLS;
    init_ds();
    cur_endwin();
    winit();
    if oldcols != COLS {
        fclose(file);
        fatal(b"Restore Error: new screen size\n\0" as *const u8 as *const libc::c_char);
    }
    wdump();
    if fread(
        savewin.as_mut_ptr() as *mut libc::c_void,
        4000 as libc::c_int as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        file,
    ) == 0
    {
        fclose(file);
        fatal(b"Serious restore error\0" as *const u8 as *const libc::c_char);
    }
    wrestor();
    fclose(file);
    mpos = 0 as libc::c_int;
    ifterse(
        b"%s, Welcome back!\0" as *const u8 as *const libc::c_char,
        b"Hello %s, Welcome back to the Dungeons of Doom!\0" as *const u8 as *const libc::c_char,
        whoami.as_mut_ptr(),
    );
    dnum = md_srand();
    remove(save_name.as_mut_ptr());
}
unsafe extern "C" fn run_static_initializers() {
    _Uend = dummy
        .as_mut_ptr()
        .offset(::core::mem::size_of::<[libc::c_char; 1234]>() as libc::c_ulong as isize);
    startmem = dummier
        .as_mut_ptr()
        .offset(::core::mem::size_of::<[libc::c_char; 4321]>() as libc::c_ulong as isize);
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
