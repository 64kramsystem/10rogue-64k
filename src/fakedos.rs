use ::libc;
extern "C" {
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    static mut current_drive: libc::c_int;
    static mut last_drive: libc::c_int;
    fn is_alpha(ch: libc::c_char) -> bool;
    fn endblk(str: *mut libc::c_char) -> *mut libc::c_char;
    fn stpblk(str: *mut libc::c_char) -> *mut libc::c_char;
    fn cur_clear();
    fn cursor(ison: bool) -> bool;
    fn wdump();
    fn wrestor();
    fn cur_printw(msg: *const libc::c_char, _: ...);
    fn cur_move(row: libc::c_int, col: libc::c_int) -> libc::c_int;
    fn getinfo(str: *mut libc::c_char, size: libc::c_int) -> libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fakedos() {
    let mut comline: [libc::c_char; 132] = [0; 132];
    let mut savedir: [libc::c_char; 3] = *::core::mem::transmute::<
        &[u8; 3],
        &mut [libc::c_char; 3],
    >(b"a:\0");
    let mut comhead: *mut libc::c_char = 0 as *mut libc::c_char;
    wdump();
    cur_clear();
    cur_move(0 as libc::c_int, 0 as libc::c_int);
    cursor(1 as libc::c_int != 0);
    *savedir.as_mut_ptr() = (current_drive + 'A' as i32) as libc::c_char;
    loop {
        memset(
            comline.as_mut_ptr() as *mut libc::c_void,
            0 as libc::c_int,
            ::core::mem::size_of::<[libc::c_char; 132]>() as libc::c_ulong,
        );
        cur_printw(
            b"\n%c>\0" as *const u8 as *const libc::c_char,
            current_drive + 'A' as i32,
        );
        getinfo(comline.as_mut_ptr(), 130 as libc::c_int);
        comhead = stpblk(comline.as_mut_ptr());
        endblk(comhead);
        if !dodos(comhead) {
            break;
        }
    }
    dodos(savedir.as_mut_ptr());
    cursor(0 as libc::c_int != 0);
    cur_clear();
    wrestor();
}
unsafe extern "C" fn dodos(mut com: *mut libc::c_char) -> bool {
    let mut drv: libc::c_int = 0;
    if !(*com as libc::c_int & !(0x7f as libc::c_int) == 0 as libc::c_int)
        || strcmp(com, b"rogue\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
    {
        return 0 as libc::c_int != 0;
    }
    if *com.offset(1 as libc::c_int as isize) as libc::c_int == ':' as i32
        && *com.offset(2 as libc::c_int as isize) as libc::c_int == 0 as libc::c_int
    {
        drv = (*com as libc::c_int & 0x1f as libc::c_int) - 1 as libc::c_int;
        cur_printw(b"\n\0" as *const u8 as *const libc::c_char);
        if !is_alpha(*com) || drv >= select_drive(drv) {
            cur_printw(
                b"Invalid drive specification\n\0" as *const u8 as *const libc::c_char,
            );
        }
    } else if *com.offset(0 as libc::c_int as isize) != 0 {
        cur_printw(
            b"\nBad command or file name\n\0" as *const u8 as *const libc::c_char,
        );
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn select_drive(mut drv: libc::c_int) -> libc::c_int {
    if drv >= 0 as libc::c_int && drv <= last_drive {
        current_drive = drv;
    }
    return last_drive;
}
