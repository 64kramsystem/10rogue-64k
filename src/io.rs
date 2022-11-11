use ::libc;
extern "C" {
    fn __ctype_toupper_loc() -> *mut *const __int32_t;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strpbrk(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn vsnprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ::core::ffi::VaList,
    ) -> libc::c_int;
    fn flush_type();
    fn readchar() -> byte;
    fn md_keyboard_leds() -> libc::c_int;
    fn md_time() -> libc::c_long;
    fn md_localtime() -> *mut TM;
    static mut reinit: libc::c_int;
    static mut nullstr: [libc::c_char; 0];
    static mut expert: bool;
    static mut faststate: bool;
    static mut running: bool;
    static mut save_msg: bool;
    static mut terse: bool;
    static mut he_man: [*mut libc::c_char; 0];
    static mut huh: [libc::c_char; 0];
    static mut count: libc::c_int;
    static mut hungry_state: libc::c_int;
    static mut level: libc::c_int;
    static mut mpos: libc::c_int;
    static mut purse: libc::c_int;
    static mut cur_armor: *mut THING;
    static mut cur_ring: [*mut THING; 0];
    static mut player: THING;
    static mut max_stats: stats;
    static mut msgbuf: *mut libc::c_char;
    fn show_count();
    fn is_lower(ch: libc::c_char) -> bool;
    fn look(wakeup: bool);
    fn is_print(ch: libc::c_char) -> bool;
    fn is_space(ch: libc::c_char) -> bool;
    static mut LINES: libc::c_int;
    static mut COLS: libc::c_int;
    static mut is_saved: libc::c_int;
    static mut scr_type: libc::c_int;
    fn cur_printw(msg_0: *const libc::c_char, _: ...);
    fn cur_mvaddstr(r: libc::c_int, c: libc::c_int, s: *mut libc::c_char);
    fn cursor(ison: bool) -> bool;
    fn cur_addstr(s: *mut libc::c_char);
    fn cur_addch(chr: byte);
    fn set_attr(bute: libc::c_int);
    fn cur_clrtoeol();
    fn getrc(rp: *mut libc::c_int, cp: *mut libc::c_int);
    fn cur_inch() -> byte;
    fn cur_move(row: libc::c_int, col: libc::c_int) -> libc::c_int;
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
pub type __int32_t = libc::c_int;
pub type va_list = __builtin_va_list;
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
pub type byte = libc::c_uchar;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct coord {
    pub x: libc::c_int,
    pub y: libc::c_int,
}
pub type str_t = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct room {
    pub r_pos: coord,
    pub r_max: coord,
    pub r_gold: coord,
    pub r_goldval: libc::c_int,
    pub r_flags: libc::c_short,
    pub r_nexits: libc::c_int,
    pub r_exit: [coord; 12],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stats {
    pub s_str: str_t,
    pub s_exp: libc::c_long,
    pub s_lvl: libc::c_int,
    pub s_arm: libc::c_int,
    pub s_hpt: libc::c_int,
    pub s_dmg: *mut libc::c_char,
    pub s_maxhp: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union thing {
    pub _t: C2RustUnnamed_0,
    pub _o: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub _l_next: *mut thing,
    pub _l_prev: *mut thing,
    pub _o_type: libc::c_int,
    pub _o_pos: coord,
    pub _o_text: *mut libc::c_char,
    pub _o_launch: libc::c_char,
    pub _o_damage: *mut libc::c_char,
    pub _o_hurldmg: *mut libc::c_char,
    pub _o_count: libc::c_int,
    pub _o_which: libc::c_int,
    pub _o_hplus: libc::c_int,
    pub _o_dplus: libc::c_int,
    pub _o_ac: libc::c_short,
    pub _o_flags: libc::c_short,
    pub _o_enemy: libc::c_char,
    pub _o_group: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub _l_next: *mut thing,
    pub _l_prev: *mut thing,
    pub _t_pos: coord,
    pub _t_turn: libc::c_char,
    pub _t_type: libc::c_char,
    pub _t_disguise: byte,
    pub _t_oldch: byte,
    pub _t_dest: *mut coord,
    pub _t_flags: libc::c_short,
    pub _t_stats: stats,
    pub _t_room: *mut room,
    pub _t_pack: *mut thing,
}
pub type THING = thing;
#[inline]
unsafe extern "C" fn toupper(mut __c: libc::c_int) -> libc::c_int {
    return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
        *(*__ctype_toupper_loc()).offset(__c as isize)
    } else {
        __c
    };
}
static mut newpos: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn ifterse(
    mut tfmt: *const libc::c_char,
    mut fmt: *const libc::c_char,
    mut args: ...
) {
    let mut argp: ::core::ffi::VaListImpl;
    argp = args.clone();
    if expert {
        vmsg(tfmt, argp.as_va_list());
    } else {
        vmsg(fmt, argp.as_va_list());
    };
}
#[no_mangle]
pub unsafe extern "C" fn vmsg(mut fmt: *const libc::c_char, mut argp: ::core::ffi::VaList) {
    if *fmt as libc::c_int == '\0' as i32 {
        cur_move(0 as libc::c_int, 0 as libc::c_int);
        cur_clrtoeol();
        mpos = 0 as libc::c_int;
        return;
    }
    doadd(fmt, argp.as_va_list());
    endmsg();
}
#[no_mangle]
pub unsafe extern "C" fn msg(mut fmt: *const libc::c_char, mut args: ...) {
    let mut argp: ::core::ffi::VaListImpl;
    argp = args.clone();
    vmsg(fmt, argp.as_va_list());
}
#[no_mangle]
pub unsafe extern "C" fn addmsg(mut fmt: *const libc::c_char, mut args: ...) {
    let mut argp: ::core::ffi::VaListImpl;
    argp = args.clone();
    doadd(fmt, argp.as_va_list());
}
#[no_mangle]
pub unsafe extern "C" fn endmsg() {
    if save_msg {
        strcpy(huh.as_mut_ptr(), msgbuf);
    }
    if mpos != 0 {
        look(0 as libc::c_int != 0);
        cur_move(0 as libc::c_int, mpos);
        more(b" More \0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    }
    if is_lower(*msgbuf.offset(0 as libc::c_int as isize)) as libc::c_int != 0
        && *msgbuf.offset(1 as libc::c_int as isize) as libc::c_int != ')' as i32
    {
        *msgbuf.offset(0 as libc::c_int as isize) = ({
            let mut __res: libc::c_int = 0;
            if ::core::mem::size_of::<libc::c_char>() as libc::c_ulong
                > 1 as libc::c_int as libc::c_ulong
            {
                if 0 != 0 {
                    let mut __c: libc::c_int =
                        *msgbuf.offset(0 as libc::c_int as isize) as libc::c_int;
                    __res = if __c < -(128 as libc::c_int) || __c > 255 as libc::c_int {
                        __c
                    } else {
                        *(*__ctype_toupper_loc()).offset(__c as isize)
                    };
                } else {
                    __res = toupper(*msgbuf.offset(0 as libc::c_int as isize) as libc::c_int);
                }
            } else {
                __res = *(*__ctype_toupper_loc())
                    .offset(*msgbuf.offset(0 as libc::c_int as isize) as libc::c_int as isize);
            }
            __res
        }) as libc::c_char;
    }
    putmsg(0 as libc::c_int, msgbuf);
    mpos = newpos;
    newpos = 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn more(mut msg_0: *mut libc::c_char) {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut msz: libc::c_int = 0;
    let mut mbuf: [libc::c_char; 80] = [0; 80];
    let mut morethere: libc::c_int = 1 as libc::c_int;
    let mut covered: libc::c_int = 0 as libc::c_int;
    msz = strlen(msg_0) as libc::c_int;
    getrc(&mut x, &mut y);
    if x != 0 as libc::c_int {
        x = 0 as libc::c_int;
        y = COLS;
    }
    if y + msz > COLS {
        y = COLS - msz;
        cur_move(x, y);
        covered = 1 as libc::c_int;
    }
    i = 0 as libc::c_int;
    while i < msz {
        mbuf[i as usize] = cur_inch() as libc::c_char;
        if i + y < COLS - 2 as libc::c_int {
            cur_move(x, y + i + 1 as libc::c_int);
        }
        mbuf[(i + 1 as libc::c_int) as usize] = 0 as libc::c_int as libc::c_char;
        i += 1;
    }
    cur_move(x, y);
    set_attr(14 as libc::c_int);
    cur_addstr(msg_0);
    set_attr(0 as libc::c_int);
    while readchar() as libc::c_int != ' ' as i32 {
        if covered != 0 && morethere != 0 {
            cur_move(x, y);
            cur_addstr(mbuf.as_mut_ptr());
            morethere = 0 as libc::c_int;
        } else if covered != 0 {
            cur_move(x, y);
            set_attr(14 as libc::c_int);
            cur_addstr(msg_0);
            set_attr(0 as libc::c_int);
            morethere = 1 as libc::c_int;
        }
    }
    cur_move(x, y);
    cur_addstr(mbuf.as_mut_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn doadd(mut fmt: *const libc::c_char, mut argp: ::core::ffi::VaList) {
    vsnprintf(
        &mut *msgbuf.offset(newpos as isize),
        (128 as libc::c_int - newpos) as libc::c_ulong,
        fmt,
        argp.as_va_list(),
    );
    newpos = strlen(msgbuf) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn putmsg(mut msgline: libc::c_int, mut msg_0: *mut libc::c_char) {
    let mut curmsg: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut lastmsg: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmpmsg: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut curlen: libc::c_int = 0;
    curmsg = msg_0;
    loop {
        scrlmsg(msgline, lastmsg, curmsg);
        curlen = strlen(curmsg) as libc::c_int;
        newpos = curlen;
        if curlen > COLS {
            more(b" Cont \0" as *const u8 as *const libc::c_char as *mut libc::c_char);
            lastmsg = curmsg;
            loop {
                tmpmsg = strpbrk(curmsg, b" \0" as *const u8 as *const libc::c_char);
                if (tmpmsg.is_null()
                    || tmpmsg >= &mut *lastmsg.offset(COLS as isize) as *mut libc::c_char)
                    && lastmsg == curmsg
                {
                    curmsg = &mut *lastmsg.offset(COLS as isize) as *mut libc::c_char;
                    break;
                } else {
                    if tmpmsg >= lastmsg.offset(COLS as isize)
                        || (strlen(curmsg) as libc::c_int) < COLS
                    {
                        break;
                    }
                    curmsg = tmpmsg.offset(1 as libc::c_int as isize);
                }
            }
        }
        if !(curlen > COLS) {
            break;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn scrlmsg(
    mut msgline: libc::c_int,
    mut str1: *mut libc::c_char,
    mut str2: *mut libc::c_char,
) {
    let mut fmt: *mut libc::c_char = 0 as *mut libc::c_char;
    if COLS > 40 as libc::c_int {
        fmt = b"%.80s\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    } else {
        fmt = b"%.40s\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    if str1.is_null() {
        cur_move(msgline, 0 as libc::c_int);
        if (strlen(str2) as libc::c_int) < COLS {
            cur_clrtoeol();
        }
        cur_printw(fmt, str2);
    } else {
        while str1 <= str2 {
            cur_move(msgline, 0 as libc::c_int);
            let fresh0 = str1;
            str1 = str1.offset(1);
            cur_printw(fmt, fresh0);
            if (strlen(str1) as libc::c_int) < COLS - 1 as libc::c_int {
                cur_clrtoeol();
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn io_unctrl(mut ch: byte) -> *mut libc::c_char {
    static mut chstr: [libc::c_char; 9] = [0; 9];
    if is_space(ch as libc::c_char) {
        strcpy(
            chstr.as_mut_ptr(),
            b" \0" as *const u8 as *const libc::c_char,
        );
    } else if !is_print(ch as libc::c_char) {
        if (ch as libc::c_int) < ' ' as i32 {
            sprintf(
                chstr.as_mut_ptr(),
                b"^%c\0" as *const u8 as *const libc::c_char,
                ch as libc::c_int + '@' as i32,
            );
        } else {
            sprintf(
                chstr.as_mut_ptr(),
                b"\\x%x\0" as *const u8 as *const libc::c_char,
                ch as libc::c_int,
            );
        }
    } else {
        chstr[0 as libc::c_int as usize] = ch as libc::c_char;
        chstr[1 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    }
    return chstr.as_mut_ptr();
}
#[no_mangle]
pub unsafe extern "C" fn status() {
    let mut oy: libc::c_int = 0;
    let mut ox: libc::c_int = 0;
    static mut s_hungry: libc::c_int = 0;
    static mut s_lvl: libc::c_int = 0;
    static mut s_pur: libc::c_int = -(1 as libc::c_int);
    static mut s_hp: libc::c_int = 0;
    static mut s_ac: libc::c_int = 0 as libc::c_int;
    static mut s_str: str_t = 0;
    static mut s_elvl: libc::c_int = 0 as libc::c_int;
    static mut state_name: [*mut libc::c_char; 5] = [
        b"      \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"Hungry\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"Weak\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"Faint\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"?\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ];
    SIG2();
    getrc(&mut oy, &mut ox);
    if scr_type != 7 as libc::c_int {
        set_attr(11 as libc::c_int);
    }
    if s_lvl != level {
        s_lvl = level;
        cur_move(
            if COLS == 40 as libc::c_int {
                22 as libc::c_int
            } else {
                23 as libc::c_int
            },
            0 as libc::c_int,
        );
        cur_printw(b"Level:%-4d\0" as *const u8 as *const libc::c_char, level);
    }
    if s_hp != player._t._t_stats.s_hpt {
        s_hp = player._t._t_stats.s_hpt;
        cur_move(
            if COLS == 40 as libc::c_int {
                22 as libc::c_int
            } else {
                23 as libc::c_int
            },
            12 as libc::c_int,
        );
        cur_printw(
            b"Hits:%d(%d) \0" as *const u8 as *const libc::c_char,
            player._t._t_stats.s_hpt,
            player._t._t_stats.s_maxhp,
        );
        if player._t._t_stats.s_hpt < 100 as libc::c_int {
            cur_addch(' ' as i32 as byte);
        }
    }
    if player._t._t_stats.s_str != s_str {
        s_str = player._t._t_stats.s_str;
        cur_move(
            if COLS == 40 as libc::c_int {
                22 as libc::c_int
            } else {
                23 as libc::c_int
            },
            26 as libc::c_int,
        );
        cur_printw(
            b"Str:%d(%d) \0" as *const u8 as *const libc::c_char,
            player._t._t_stats.s_str,
            max_stats.s_str,
        );
    }
    if s_pur != purse {
        s_pur = purse;
        cur_move(
            23 as libc::c_int,
            if COLS == 40 as libc::c_int {
                0 as libc::c_int
            } else {
                40 as libc::c_int
            },
        );
        cur_printw(b"Gold:%-5u\0" as *const u8 as *const libc::c_char, purse);
    }
    if s_ac
        != (if !cur_armor.is_null() {
            (*cur_armor)._o._o_ac as libc::c_int
        } else {
            player._t._t_stats.s_arm
        })
    {
        s_ac = if !cur_armor.is_null() {
            (*cur_armor)._o._o_ac as libc::c_int
        } else {
            player._t._t_stats.s_arm
        };
        if !(*cur_ring.as_mut_ptr().offset(0 as libc::c_int as isize)).is_null()
            && (**cur_ring.as_mut_ptr().offset(0 as libc::c_int as isize))
                ._o
                ._o_which
                == 0 as libc::c_int
        {
            s_ac -= (**cur_ring.as_mut_ptr().offset(0 as libc::c_int as isize))
                ._o
                ._o_ac as libc::c_int;
        }
        if !(*cur_ring.as_mut_ptr().offset(1 as libc::c_int as isize)).is_null()
            && (**cur_ring.as_mut_ptr().offset(1 as libc::c_int as isize))
                ._o
                ._o_which
                == 0 as libc::c_int
        {
            s_ac -= (**cur_ring.as_mut_ptr().offset(1 as libc::c_int as isize))
                ._o
                ._o_ac as libc::c_int;
        }
        cur_move(
            23 as libc::c_int,
            if COLS == 40 as libc::c_int {
                12 as libc::c_int
            } else {
                52 as libc::c_int
            },
        );
        cur_printw(
            b"Armor:%-2d\0" as *const u8 as *const libc::c_char,
            -((if !cur_armor.is_null() {
                (*cur_armor)._o._o_ac as libc::c_int
            } else {
                player._t._t_stats.s_arm
            }) - 11 as libc::c_int),
        );
    }
    if s_elvl != player._t._t_stats.s_lvl {
        s_elvl = player._t._t_stats.s_lvl;
        cur_move(
            23 as libc::c_int,
            if COLS == 40 as libc::c_int {
                22 as libc::c_int
            } else {
                62 as libc::c_int
            },
        );
        cur_printw(
            b"%-12s\0" as *const u8 as *const libc::c_char,
            *he_man
                .as_mut_ptr()
                .offset((s_elvl - 1 as libc::c_int) as isize),
        );
    }
    if s_hungry != hungry_state {
        s_hungry = hungry_state;
        cur_move(
            24 as libc::c_int,
            if COLS == 40 as libc::c_int {
                28 as libc::c_int
            } else {
                58 as libc::c_int
            },
        );
        cur_addstr(state_name[0 as libc::c_int as usize]);
        cur_move(
            24 as libc::c_int,
            if COLS == 40 as libc::c_int {
                28 as libc::c_int
            } else {
                58 as libc::c_int
            },
        );
        if hungry_state != 0 {
            set_attr(16 as libc::c_int);
            cur_addstr(state_name[hungry_state as usize]);
            set_attr(0 as libc::c_int);
        }
    }
    if scr_type != 7 as libc::c_int {
        set_attr(0 as libc::c_int);
    }
    cur_move(oy, ox);
}
#[no_mangle]
pub unsafe extern "C" fn wait_for(mut ch: byte) {
    while readchar() as libc::c_int != ch as libc::c_int {}
}
#[no_mangle]
pub unsafe extern "C" fn wait_msg(mut msg_0: *const libc::c_char) {
    set_attr(0 as libc::c_int);
    cur_move(LINES - 1 as libc::c_int, 0 as libc::c_int);
    cursor(1 as libc::c_int != 0);
    if *msg_0 != 0 {
        cur_printw(
            b"[Press Enter to %s]\0" as *const u8 as *const libc::c_char,
            msg_0,
        );
    } else {
        cur_printw(b"[Press Enter]\0" as *const u8 as *const libc::c_char);
    }
    flush_type();
    wait_for('\n' as i32 as byte);
    cur_move(LINES - 1 as libc::c_int, 0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn show_win(mut message: *mut libc::c_char) {
    cur_mvaddstr(0 as libc::c_int, 0 as libc::c_int, message);
    cur_move(player._t._t_pos.y, player._t._t_pos.x);
    wait_for(' ' as i32 as byte);
}
#[no_mangle]
pub unsafe extern "C" fn str_attr(mut str: *mut libc::c_char) {
    while *str != 0 {
        if *str as libc::c_int == '%' as i32 {
            str = str.offset(1);
            set_attr(14 as libc::c_int);
        }
        let fresh1 = str;
        str = str.offset(1);
        cur_addch(*fresh1 as byte);
        set_attr(0 as libc::c_int);
    }
}
#[no_mangle]
pub unsafe extern "C" fn SIG2() {
    static mut key_init: libc::c_int = 1 as libc::c_int;
    static mut numl: libc::c_int = 0;
    static mut capsl: libc::c_int = 0;
    static mut nspot: libc::c_int = 0;
    static mut cspot: libc::c_int = 0;
    static mut tspot: libc::c_int = 0;
    let mut new_numl: libc::c_int = 0;
    let mut new_capsl: libc::c_int = 0;
    let mut new_fmode: libc::c_int = 0;
    static mut bighand: libc::c_int = 0;
    static mut littlehand: libc::c_int = 0;
    let mut showtime: libc::c_int = 0 as libc::c_int;
    let mut spare: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    static mut cur_time: libc::c_long = 0 as libc::c_int as libc::c_long;
    let mut new_time: libc::c_long = md_time();
    if is_saved != 0 || scr_type < 0 as libc::c_int {
        return;
    }
    new_numl = md_keyboard_leds();
    new_capsl = new_numl & 0x40 as libc::c_int;
    new_fmode = new_numl & 0x10 as libc::c_int;
    new_numl &= 0x20 as libc::c_int;
    if new_time - cur_time >= 60 as libc::c_int as libc::c_long {
        let mut local: *mut TM = md_localtime();
        bighand = (*local).hour % 12 as libc::c_int;
        littlehand = (*local).minute;
        cur_time = new_time - (*local).second as libc::c_long;
        showtime = 1 as libc::c_int;
    }
    if key_init != 0 || reinit != 0 {
        key_init = 0 as libc::c_int;
        reinit = key_init;
        if COLS == 40 as libc::c_int {
            nspot = 10 as libc::c_int;
            cspot = 19 as libc::c_int;
            tspot = 35 as libc::c_int;
        } else {
            nspot = 20 as libc::c_int;
            cspot = 39 as libc::c_int;
            tspot = 75 as libc::c_int;
        }
        numl = (new_numl == 0) as libc::c_int;
        capsl = (new_capsl == 0) as libc::c_int;
        showtime += 1;
        faststate = new_fmode == 0;
    }
    getrc(&mut x, &mut y);
    if faststate as libc::c_int != new_fmode {
        faststate = new_fmode != 0;
        count = 0 as libc::c_int;
        show_count();
        running = 0 as libc::c_int != 0;
        cur_move(LINES - 1 as libc::c_int, 0 as libc::c_int);
        if faststate {
            set_attr(16 as libc::c_int);
            cur_addstr(b"Fast Play\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
            set_attr(0 as libc::c_int);
        } else {
            cur_addstr(b"         \0" as *const u8 as *const libc::c_char as *mut libc::c_char);
        }
    }
    if numl != new_numl {
        numl = new_numl;
        count = 0 as libc::c_int;
        show_count();
        running = 0 as libc::c_int != 0;
        cur_move(24 as libc::c_int, nspot);
        if numl != 0 {
            set_attr(16 as libc::c_int);
            cur_addstr(b"NUM LOCK\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
            set_attr(0 as libc::c_int);
        } else {
            cur_addstr(b"        \0" as *const u8 as *const libc::c_char as *mut libc::c_char);
        }
    }
    if capsl != new_capsl {
        capsl = new_capsl;
        cur_move(24 as libc::c_int, cspot);
        if capsl != 0 {
            set_attr(16 as libc::c_int);
            cur_addstr(b"CAP LOCK\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
            set_attr(0 as libc::c_int);
        } else {
            cur_addstr(b"        \0" as *const u8 as *const libc::c_char as *mut libc::c_char);
        }
    }
    if showtime != 0 {
        showtime = 0 as libc::c_int;
        spare = littlehand % 10 as libc::c_int;
        cur_move(24 as libc::c_int, tspot);
        set_attr(16 as libc::c_int);
        cur_printw(
            b"%2d:%1d%1d\0" as *const u8 as *const libc::c_char,
            if bighand != 0 {
                bighand
            } else {
                12 as libc::c_int
            },
            littlehand / 10 as libc::c_int,
            spare,
        );
        set_attr(0 as libc::c_int);
    }
    cur_move(x, y);
}
#[no_mangle]
pub unsafe extern "C" fn noterse(mut str: *mut libc::c_char) -> *mut libc::c_char {
    return if terse as libc::c_int != 0 || expert as libc::c_int != 0 {
        nullstr.as_mut_ptr()
    } else {
        str
    };
}
