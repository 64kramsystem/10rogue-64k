use ::libc;
extern "C" {
    fn __ctype_tolower_loc() -> *mut *const __int32_t;
    fn __ctype_toupper_loc() -> *mut *const __int32_t;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn readchar() -> byte;
    fn csum() -> libc::c_int;
    static mut revno: libc::c_int;
    static mut verno: libc::c_int;
    static mut after: bool;
    static mut again: bool;
    static mut door_stop: bool;
    static mut expert: bool;
    static mut fastmode: bool;
    static mut faststate: bool;
    static mut firstmove: bool;
    static mut running: bool;
    static mut save_msg: bool;
    static mut huh: [libc::c_char; 0];
    static mut runch: libc::c_char;
    static mut typebuf: *mut libc::c_char;
    static mut take: libc::c_char;
    static mut helpcoms: [h_list; 0];
    static mut helpobjs: [h_list; 0];
    static mut count: libc::c_int;
    static mut mpos: libc::c_int;
    static mut no_command: libc::c_int;
    static mut cur_ring: [*mut THING; 0];
    static mut player: THING;
    static mut delta: coord;
    #[link_name = "macro"]
    static mut macro_0: [libc::c_char; 0];
    static mut whoami: [libc::c_char; 0];
    static mut _level: *mut byte;
    static mut _flags: *mut byte;
    fn wear();
    fn take_off();
    fn teleport() -> libc::c_int;
    fn rnd(range: libc::c_int) -> libc::c_int;
    fn search();
    fn do_daemons();
    fn do_fuses();
    fn pick_up(ch: byte);
    fn io_unctrl(ch: byte) -> *mut libc::c_char;
    fn msg(fmt: *const libc::c_char, _: ...);
    fn INDEX(y: libc::c_int, x: libc::c_int) -> libc::c_int;
    fn tr_name(type_0: byte) -> *mut libc::c_char;
    fn get_dir() -> bool;
    fn doctor();
    fn save_game();
    fn addmsg(fmt: *const libc::c_char, _: ...);
    fn do_macro(buf: *mut libc::c_char, sz: libc::c_int);
    fn discovered();
    fn do_zap();
    fn fakedos();
    fn help(helpscr: *mut h_list);
    fn u_level();
    fn d_level();
    fn call();
    fn ring_off();
    fn ring_on();
    fn wield();
    fn eat();
    fn read_scroll();
    fn quaff();
    #[link_name = "drop"]
    fn drop_0();
    fn inventory(list: *mut THING, type_0: libc::c_int, lstr: *mut libc::c_char) -> byte;
    fn quit();
    fn missile(ydelta: libc::c_int, xdelta: libc::c_int);
    fn do_run(ch: byte);
    fn do_move(dy: libc::c_int, dx: libc::c_int);
    fn find_dir(ch: byte, cp: *mut coord) -> bool;
    fn look(wakeup: bool);
    fn status();
    fn cur_refresh();
    fn cur_addstr(s: *mut libc::c_char);
    fn cur_printw(msg_0: *const libc::c_char, _: ...);
    fn cur_move(row: libc::c_int, col: libc::c_int) -> libc::c_int;
    static mut LINES: libc::c_int;
    static mut COLS: libc::c_int;
}
pub type __int32_t = libc::c_int;
pub type byte = libc::c_uchar;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct h_list {
    pub h_chstr: [byte; 6],
    pub h_desc: *mut libc::c_char,
}
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
unsafe extern "C" fn tolower(mut __c: libc::c_int) -> libc::c_int {
    return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
        *(*__ctype_tolower_loc()).offset(__c as isize)
    } else {
        __c
    };
}
#[inline]
unsafe extern "C" fn toupper(mut __c: libc::c_int) -> libc::c_int {
    return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
        *(*__ctype_toupper_loc()).offset(__c as isize)
    } else {
        __c
    };
}
static mut lastcount: libc::c_int = 0;
static mut lastch: byte = 0;
static mut do_take: byte = 0;
static mut lasttake: byte = 0;
#[no_mangle]
pub unsafe extern "C" fn command() {
    let mut ntimes: libc::c_int = 0;
    if player._t._t_flags as libc::c_int & 0x4000 as libc::c_int != 0 as libc::c_int {
        ntimes = rnd(2 as libc::c_int) + 2 as libc::c_int;
    } else {
        ntimes = 1 as libc::c_int;
    }
    loop {
        let fresh0 = ntimes;
        ntimes = ntimes - 1;
        if !(fresh0 != 0) {
            break;
        }
        status();
        if no_command != 0 {
            no_command -= 1;
            if no_command <= 0 as libc::c_int {
                msg(b"you can move again\0" as *const u8 as *const libc::c_char);
                no_command = 0 as libc::c_int;
            }
            cur_refresh();
        } else {
            execcom();
        }
        do_fuses();
        do_daemons();
        ntimes = 0 as libc::c_int;
        while ntimes <= 1 as libc::c_int {
            if !(*cur_ring.as_mut_ptr().offset(ntimes as isize)).is_null() {
                match (**cur_ring.as_mut_ptr().offset(ntimes as isize))
                    ._o
                    ._o_which
                {
                    3 => {
                        search();
                    }
                    11 => {
                        if rnd(50 as libc::c_int) == 17 as libc::c_int {
                            teleport();
                        }
                    }
                    _ => {}
                }
            }
            ntimes += 1;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn com_char() -> byte {
    let mut same: bool = false;
    let mut ch: byte = 0;
    same = fastmode as libc::c_int == faststate as libc::c_int;
    ch = readchar();
    if same {
        fastmode = faststate;
    } else {
        fastmode = !faststate;
    }
    match ch as libc::c_int {
        8 => {
            ch = 'h' as i32 as byte;
        }
        43 => {
            ch = 't' as i32 as byte;
        }
        45 => {
            ch = 'z' as i32 as byte;
        }
        _ => {}
    }
    if mpos != 0 && !running {
        msg(b"\0" as *const u8 as *const libc::c_char);
    }
    return ch;
}
#[no_mangle]
pub unsafe extern "C" fn get_prefix() -> byte {
    let mut junk: libc::c_int = 0;
    let mut retch: byte = 0;
    let mut ch: byte = 0;
    after = 1 as libc::c_int != 0;
    fastmode = faststate;
    look(1 as libc::c_int != 0);
    if !running {
        door_stop = 0 as libc::c_int != 0;
    }
    do_take = 1 as libc::c_int as byte;
    again = 0 as libc::c_int != 0;
    count -= 1;
    if count > 0 as libc::c_int {
        do_take = lasttake;
        retch = lastch;
        fastmode = 0 as libc::c_int != 0;
        cur_refresh();
    } else {
        count = 0 as libc::c_int;
        if running {
            retch = runch as byte;
            do_take = lasttake;
            cur_refresh();
        } else {
            retch = 0 as libc::c_int as byte;
            while retch as libc::c_int == 0 as libc::c_int {
                ch = com_char();
                match ch as libc::c_int {
                    48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 => {
                        junk = count * 10 as libc::c_int;
                        junk += ch as libc::c_int - '0' as i32;
                        if junk > 0 as libc::c_int && junk < 10000 as libc::c_int {
                            count = junk;
                        }
                        show_count();
                    }
                    102 => {
                        fastmode = !fastmode;
                    }
                    103 => {
                        do_take = 0 as libc::c_int as byte;
                    }
                    97 => {
                        retch = lastch;
                        count = lastcount;
                        do_take = lasttake;
                        again = 1 as libc::c_int != 0;
                    }
                    32 => {}
                    27 => {
                        door_stop = 0 as libc::c_int != 0;
                        count = 0 as libc::c_int;
                        show_count();
                    }
                    _ => {
                        retch = ch;
                    }
                }
            }
        }
    }
    if count != 0 {
        fastmode = 0 as libc::c_int != 0;
    }
    match retch as libc::c_int {
        104 | 106 | 107 | 108 | 121 | 117 | 98 | 110 => {
            if fastmode as libc::c_int != 0 && !running {
                if !(player._t._t_flags as libc::c_int & 0x1 as libc::c_int != 0 as libc::c_int) {
                    door_stop = 1 as libc::c_int != 0;
                    firstmove = 1 as libc::c_int != 0;
                }
                retch = ({
                    let mut __res: libc::c_int = 0;
                    if ::core::mem::size_of::<byte>() as libc::c_ulong
                        > 1 as libc::c_int as libc::c_ulong
                    {
                        if 0 != 0 {
                            let mut __c: libc::c_int = retch as libc::c_int;
                            __res = if __c < -(128 as libc::c_int) || __c > 255 as libc::c_int {
                                __c
                            } else {
                                *(*__ctype_toupper_loc()).offset(__c as isize)
                            };
                        } else {
                            __res = toupper(retch as libc::c_int);
                        }
                    } else {
                        __res = *(*__ctype_toupper_loc()).offset(retch as libc::c_int as isize);
                    }
                    __res
                }) as byte;
            }
        }
        72 | 74 | 75 | 76 | 89 | 85 | 66 | 78 | 113 | 114 | 115 | 122 | 116 | 46 => {}
        _ => {
            count = 0 as libc::c_int;
        }
    }
    if count != 0 || lastcount != 0 {
        show_count();
    }
    lastch = retch;
    lastcount = count;
    lasttake = do_take;
    return retch;
}
#[no_mangle]
pub unsafe extern "C" fn show_count() {
    cur_move(LINES - 2 as libc::c_int, COLS - 4 as libc::c_int);
    if count != 0 {
        cur_printw(b"%-4d\0" as *const u8 as *const libc::c_char, count);
    } else {
        cur_addstr(b"    \0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    };
}
#[no_mangle]
pub unsafe extern "C" fn execcom() {
    let mut mv: coord = coord { x: 0, y: 0 };
    let mut ch: libc::c_int = 0;
    loop {
        ch = get_prefix() as libc::c_int;
        match ch {
            104 | 106 | 107 | 108 | 121 | 117 | 98 | 110 => {
                find_dir(ch as byte, &mut mv);
                do_move(mv.y, mv.x);
            }
            72 | 74 | 75 | 76 | 89 | 85 | 66 | 78 => {
                do_run(
                    ({
                        let mut __res: libc::c_int = 0;
                        if ::core::mem::size_of::<libc::c_int>() as libc::c_ulong
                            > 1 as libc::c_int as libc::c_ulong
                        {
                            if 0 != 0 {
                                let mut __c: libc::c_int = ch;
                                __res = if __c < -(128 as libc::c_int) || __c > 255 as libc::c_int {
                                    __c
                                } else {
                                    *(*__ctype_tolower_loc()).offset(__c as isize)
                                };
                            } else {
                                __res = tolower(ch);
                            }
                        } else {
                            __res = *(*__ctype_tolower_loc()).offset(ch as isize);
                        }
                        __res
                    }) as byte,
                );
            }
            116 => {
                if get_dir() {
                    missile(delta.y, delta.x);
                } else {
                    after = 0 as libc::c_int != 0;
                }
            }
            81 => {
                after = 0 as libc::c_int != 0;
                quit();
            }
            105 => {
                after = 0 as libc::c_int != 0;
                inventory(
                    player._t._t_pack,
                    0 as libc::c_int,
                    b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
            }
            100 => {
                drop_0();
            }
            113 => {
                quaff();
            }
            114 => {
                read_scroll();
            }
            101 => {
                eat();
            }
            119 => {
                wield();
            }
            87 => {
                wear();
            }
            84 => {
                take_off();
            }
            80 => {
                ring_on();
            }
            82 => {
                ring_off();
            }
            99 => {
                after = 0 as libc::c_int != 0;
                call();
            }
            62 => {
                after = 0 as libc::c_int != 0;
                d_level();
            }
            60 => {
                after = 0 as libc::c_int != 0;
                u_level();
            }
            47 => {
                after = 0 as libc::c_int != 0;
                help(helpobjs.as_mut_ptr());
            }
            63 => {
                after = 0 as libc::c_int != 0;
                help(helpcoms.as_mut_ptr());
            }
            33 => {
                after = 0 as libc::c_int != 0;
                fakedos();
            }
            115 => {
                search();
            }
            122 => {
                if get_dir() {
                    do_zap();
                } else {
                    after = 0 as libc::c_int != 0;
                }
            }
            68 => {
                after = 0 as libc::c_int != 0;
                discovered();
            }
            20 => {
                after = 0 as libc::c_int != 0;
                // Rust port: Fixed weakly typed conversion
                expert ^= true;
                msg(if expert as libc::c_int != 0 {
                    b"Ok, I'll be brief\0" as *const u8 as *const libc::c_char
                } else {
                    b"Goodie, I can use big words again!\0" as *const u8 as *const libc::c_char
                });
            }
            70 => {
                after = 0 as libc::c_int != 0;
                do_macro(macro_0.as_mut_ptr(), 41 as libc::c_int);
            }
            6 => {
                after = 0 as libc::c_int != 0;
                typebuf = macro_0.as_mut_ptr();
            }
            18 => {
                after = 0 as libc::c_int != 0;
                msg(huh.as_mut_ptr());
            }
            118 => {
                after = 0 as libc::c_int != 0;
                if strcmp(
                    whoami.as_mut_ptr(),
                    b"The Grand Beeking\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    addmsg(b"(%d)\0" as *const u8 as *const libc::c_char, csum());
                }
                msg(
                    b"Rogue version %d.%d (Mr. Mctesq was here)\0" as *const u8
                        as *const libc::c_char,
                    revno,
                    verno,
                );
            }
            83 => {
                after = 0 as libc::c_int != 0;
                save_game();
            }
            46 => {
                doctor();
            }
            94 => {
                after = 0 as libc::c_int != 0;
                if get_dir() {
                    let mut lookat: coord = coord { x: 0, y: 0 };
                    lookat.y = player._t._t_pos.y + delta.y;
                    lookat.x = player._t._t_pos.x + delta.x;
                    if *_level.offset(INDEX(lookat.y, lookat.x) as isize) as libc::c_int
                        != 0x4 as libc::c_int
                    {
                        msg(b"no trap there.\0" as *const u8 as *const libc::c_char);
                    } else {
                        msg(
                            b"you found %s\0" as *const u8 as *const libc::c_char,
                            tr_name(
                                (*_flags.offset(INDEX(lookat.y, lookat.x) as isize) as libc::c_int
                                    & 0x7 as libc::c_int) as byte,
                            ),
                        );
                    }
                }
            }
            111 => {
                after = 0 as libc::c_int != 0;
                msg(b"i don't have any options, oh my!\0" as *const u8 as *const libc::c_char);
            }
            12 => {
                after = 0 as libc::c_int != 0;
                msg(b"the screen looks fine to me (jll was here)\0" as *const u8
                    as *const libc::c_char);
            }
            _ => {
                after = 0 as libc::c_int != 0;
                save_msg = 0 as libc::c_int != 0;
                msg(
                    b"illegal command '%s'\0" as *const u8 as *const libc::c_char,
                    io_unctrl(ch as byte),
                );
                count = 0 as libc::c_int;
                save_msg = 1 as libc::c_int != 0;
            }
        }
        if take as libc::c_int != 0 && do_take as libc::c_int != 0 {
            pick_up(take as byte);
        }
        take = 0 as libc::c_int as libc::c_char;
        if !running {
            door_stop = 0 as libc::c_int != 0;
        }
        if !(after as libc::c_int == 0 as libc::c_int) {
            break;
        }
    }
}
