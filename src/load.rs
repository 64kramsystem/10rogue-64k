use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn fseek(__stream: *mut FILE, __off: libc::c_long, __whence: libc::c_int) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    // Rust port: Fixed missing parameters
    fn peekb(p0: libc::c_int, p1: libc::c_int) -> byte;
    // Rust port: Fixed missing parameters
    fn pokeb(p0: libc::c_int, p1: libc::c_int, p2: i32);
    // Rust port: Fixed missing parameters
    fn out(p0: libc::c_int, p1: i32);
    // Rust port: Fixed missing parameters
    fn dmaout(p0: *mut i8, p1: u32, p2: u32, p3: u32);
    static mut current_drive: libc::c_int;
    static mut s_drive: [libc::c_char; 0];
    fn is_alpha(ch: libc::c_char) -> bool;
    fn is_upper(ch: libc::c_char) -> bool;
    fn video_mode(type_0: libc::c_int) -> byte;
    fn cur_getch_timeout(msdelay: libc::c_int) -> libc::c_int;
    fn get_mode() -> byte;
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
static mut store: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut blksize: libc::c_int = 0x4000 as libc::c_int;
static mut file: *mut FILE = 0 as *const FILE as *mut FILE;
#[no_mangle]
pub unsafe extern "C" fn epyx_yuck() {
    let mut type_0: libc::c_int = get_mode() as libc::c_int;
    if type_0 == 7 as libc::c_int || {
        file = fopen(
            b"rogue.pic\0" as *const u8 as *const libc::c_char,
            b"r\0" as *const u8 as *const libc::c_char,
        );
        file.is_null()
    } {
        return;
    }
    loop {
        store = malloc(blksize as libc::c_ulong) as *mut libc::c_char;
        if !store.is_null() {
            break;
        }
        blksize /= 2 as libc::c_int;
    }
    video_mode(4 as libc::c_int);
    scr_load();
    fclose(file);
    cur_getch_timeout(1000 as libc::c_int * 60 as libc::c_int * 5 as libc::c_int);
    video_mode(type_0);
    free(store as *mut libc::c_void);
}
unsafe extern "C" fn scr_load() {
    let mut palette: libc::c_int = 0;
    let mut background: libc::c_int = 0;
    let mut mode: libc::c_int = 0;
    let mut burst: libc::c_int = 0;
    bload(0xb800 as libc::c_int as libc::c_uint);
    palette = peekb(8012 as libc::c_int, 0xb800 as libc::c_int) as libc::c_int;
    background = peekb(8013 as libc::c_int, 0xb800 as libc::c_int) as libc::c_int;
    if palette >= 3 as libc::c_int {
        background |= 0x10 as libc::c_int;
    }
    burst = 0 as libc::c_int;
    let mut current_block_8: u64;
    match palette {
        2 | 5 => {
            burst = 1 as libc::c_int;
            current_block_8 = 11507105467092190330;
        }
        0 | 3 => {
            current_block_8 = 11507105467092190330;
        }
        1 | 4 => {
            palette = 0 as libc::c_int;
            current_block_8 = 13109137661213826276;
        }
        _ => {
            current_block_8 = 13109137661213826276;
        }
    }
    match current_block_8 {
        11507105467092190330 => {
            palette = 1 as libc::c_int;
        }
        _ => {}
    }
    out(0x3d9 as libc::c_int, background);
    mode = peekb(0x65 as libc::c_int, 0x40 as libc::c_int) as libc::c_int & !(0x4 as libc::c_int);
    if burst == 1 as libc::c_int {
        mode = mode | 0x4 as libc::c_int;
    }
    pokeb(0x65 as libc::c_int, 0x40 as libc::c_int, mode);
    out(0x3d8 as libc::c_int, mode);
}
unsafe extern "C" fn bload(mut segment: libc::c_uint) {
    let mut offset: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut rdcnt: libc::c_uint = 0;
    if fread(
        store as *mut libc::c_void,
        7 as libc::c_int as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        file,
    ) == 0
    {
        fseek(file, 7 as libc::c_long, 0 as libc::c_int);
    }
    loop {
        rdcnt = fread(
            store as *mut libc::c_void,
            blksize as libc::c_ulong,
            1 as libc::c_int as libc::c_ulong,
            file,
        ) as libc::c_uint;
        if !(rdcnt != 0) {
            break;
        }
        dmaout(
            store,
            rdcnt.wrapping_div(2 as libc::c_int as libc::c_uint),
            segment,
            offset,
        );
        offset = offset.wrapping_add(rdcnt);
        if offset >= 16384 as libc::c_int as libc::c_uint {
            break;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn find_drive() -> libc::c_int {
    let mut drive: libc::c_int = current_drive;
    let mut spec: libc::c_char = *s_drive.as_mut_ptr().offset(0 as libc::c_int as isize);
    if is_alpha(spec) {
        if is_upper(spec) {
            drive = spec as libc::c_int - 'A' as i32;
        } else {
            drive = spec as libc::c_int - 'a' as i32;
        }
    }
    return drive;
}
