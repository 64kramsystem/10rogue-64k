use ::libc;
extern "C" {
    fn __ctype_tolower_loc() -> *mut *const __int32_t;
    fn __ctype_toupper_loc() -> *mut *const __int32_t;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn vsnprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ::core::ffi::VaList,
    ) -> libc::c_int;
    fn init_weapon(weap: *mut THING, type_0: byte);
    fn num(n1: libc::c_int, n2: libc::c_int, type_0: libc::c_char) -> *mut libc::c_char;
    fn readchar() -> byte;
    static mut nullstr: [libc::c_char; 0];
    static mut s_guess: [*mut libc::c_char; 0];
    static mut p_guess: [*mut libc::c_char; 0];
    static mut r_guess: [*mut libc::c_char; 0];
    static mut ws_guess: [*mut libc::c_char; 0];
    static mut amulet: bool;
    static mut expert: bool;
    static mut terse: bool;
    static mut p_know: [bool; 0];
    static mut r_know: [bool; 0];
    static mut s_know: [bool; 0];
    static mut ws_know: [bool; 0];
    static mut a_names: [*mut libc::c_char; 0];
    static mut p_colors: [*mut libc::c_char; 0];
    static mut r_stones: [*mut libc::c_char; 0];
    static mut w_names: [*mut libc::c_char; 0];
    static mut ws_made: [*mut libc::c_char; 0];
    static mut ws_type: [*mut libc::c_char; 0];
    static mut a_chances: [libc::c_int; 0];
    static mut a_class: [libc::c_int; 0];
    static mut inpack: libc::c_int;
    static mut no_food: libc::c_int;
    static mut cur_armor: *mut THING;
    static mut cur_ring: [*mut THING; 0];
    static mut cur_weapon: *mut THING;
    static mut lvl_obj: *mut THING;
    static mut player: THING;
    static mut monsters: [monster; 0];
    static mut p_magic: [magic_item; 0];
    static mut r_magic: [magic_item; 0];
    static mut s_magic: [magic_item; 0];
    static mut things: [magic_item; 0];
    static mut ws_magic: [magic_item; 0];
    static mut s_names: [array; 0];
    static mut fruit: [libc::c_char; 0];
    static mut prbuf: *mut libc::c_char;
    static mut _level: *mut byte;
    fn waste_time();
    fn extinguish(func: Option::<unsafe extern "C" fn() -> ()>);
    fn unsee();
    fn msg(fmt: *const libc::c_char, _: ...);
    fn noterse(str: *mut libc::c_char) -> *mut libc::c_char;
    fn new_item() -> *mut THING;
    fn list_detach(list: *mut *mut THING, item: *mut THING);
    fn list_attach(list: *mut *mut THING, item: *mut THING);
    fn rnd(range: libc::c_int) -> libc::c_int;
    fn chg_str(amt: libc::c_int);
    fn vowelstr(str: *mut libc::c_char) -> *mut libc::c_char;
    fn INDEX(y: libc::c_int, x: libc::c_int) -> libc::c_int;
    fn get_item(purpose: *mut libc::c_char, type_0: libc::c_int) -> *mut THING;
    fn ring_num(obj: *mut THING) -> *mut libc::c_char;
    fn fix_stick(cur: *mut THING);
    fn charge_str(obj: *mut THING) -> *mut libc::c_char;
    fn is_lower(ch: libc::c_char) -> bool;
    fn cur_clear();
    fn getrc(rp: *mut libc::c_int, cp: *mut libc::c_int);
    fn cur_addstr(s: *mut libc::c_char);
    fn wdump();
    fn wrestor();
    fn cur_move(row: libc::c_int, col: libc::c_int) -> libc::c_int;
    fn cur_printw(msg_0: *const libc::c_char, _: ...);
    static mut LINES: libc::c_int;
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
pub struct magic_item {
    pub mi_name: *mut libc::c_char,
    pub mi_prob: libc::c_int,
    pub mi_worth: libc::c_short,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct array {
    pub storage: [libc::c_char; 21],
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct monster {
    pub m_name: *mut libc::c_char,
    pub m_carry: libc::c_int,
    pub m_flags: libc::c_ushort,
    pub m_stats: stats,
}
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
#[no_mangle]
pub unsafe extern "C" fn inv_name(
    mut obj: *mut THING,
    mut drop_1: bool,
) -> *mut libc::c_char {
    let mut which: libc::c_int = (*obj)._o._o_which;
    let mut pb: *mut libc::c_char = 0 as *mut libc::c_char;
    pb = prbuf;
    match (*obj)._o._o_type {
        13 => {
            if (*obj)._o._o_count == 1 as libc::c_int {
                strcpy(pb, b"A scroll \0" as *const u8 as *const libc::c_char);
                pb = &mut *prbuf.offset(9 as libc::c_int as isize) as *mut libc::c_char;
            } else {
                sprintf(
                    pb,
                    b"%d scrolls \0" as *const u8 as *const libc::c_char,
                    (*obj)._o._o_count,
                );
                pb = &mut *prbuf
                    .offset(
                        (strlen
                            as unsafe extern "C" fn(
                                *const libc::c_char,
                            ) -> libc::c_ulong)(prbuf) as isize,
                    ) as *mut libc::c_char;
            }
            if *s_know.as_mut_ptr().offset(which as isize) {
                sprintf(
                    pb,
                    b"of %s\0" as *const u8 as *const libc::c_char,
                    (*s_magic.as_mut_ptr().offset(which as isize)).mi_name,
                );
            } else if **s_guess.as_mut_ptr().offset(which as isize) != 0 {
                sprintf(
                    pb,
                    b"called %s\0" as *const u8 as *const libc::c_char,
                    *s_guess.as_mut_ptr().offset(which as isize),
                );
            } else {
                chopmsg(
                    pb,
                    b"titled '%.17s'\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    b"titled '%s'\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    &mut *s_names.as_mut_ptr().offset(which as isize) as *mut array,
                );
            }
        }
        173 => {
            if (*obj)._o._o_count == 1 as libc::c_int {
                strcpy(pb, b"A potion \0" as *const u8 as *const libc::c_char);
                pb = &mut *prbuf.offset(9 as libc::c_int as isize) as *mut libc::c_char;
            } else {
                sprintf(
                    pb,
                    b"%d potions \0" as *const u8 as *const libc::c_char,
                    (*obj)._o._o_count,
                );
                pb = &mut *pb
                    .offset(
                        (strlen
                            as unsafe extern "C" fn(
                                *const libc::c_char,
                            ) -> libc::c_ulong)(prbuf) as isize,
                    ) as *mut libc::c_char;
            }
            if *p_know.as_mut_ptr().offset(which as isize) {
                chopmsg(
                    pb,
                    b"of %s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    b"of %s(%s)\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    (*p_magic.as_mut_ptr().offset(which as isize)).mi_name,
                    *p_colors.as_mut_ptr().offset(which as isize),
                );
            } else if **p_guess.as_mut_ptr().offset(which as isize) != 0 {
                chopmsg(
                    pb,
                    b"called %s\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    b"called %s(%s)\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    *p_guess.as_mut_ptr().offset(which as isize),
                    *p_colors.as_mut_ptr().offset(which as isize),
                );
            } else if (*obj)._o._o_count == 1 as libc::c_int {
                sprintf(
                    prbuf,
                    b"A%s %s potion\0" as *const u8 as *const libc::c_char,
                    vowelstr(*p_colors.as_mut_ptr().offset(which as isize)),
                    *p_colors.as_mut_ptr().offset(which as isize),
                );
            } else {
                sprintf(
                    prbuf,
                    b"%d %s potions\0" as *const u8 as *const libc::c_char,
                    (*obj)._o._o_count,
                    *p_colors.as_mut_ptr().offset(which as isize),
                );
            }
        }
        5 => {
            if which == 1 as libc::c_int {
                if (*obj)._o._o_count == 1 as libc::c_int {
                    sprintf(
                        pb,
                        b"A%s %s\0" as *const u8 as *const libc::c_char,
                        vowelstr(fruit.as_mut_ptr()),
                        fruit.as_mut_ptr(),
                    );
                } else {
                    sprintf(
                        pb,
                        b"%d %ss\0" as *const u8 as *const libc::c_char,
                        (*obj)._o._o_count,
                        fruit.as_mut_ptr(),
                    );
                }
            } else if (*obj)._o._o_count == 1 as libc::c_int {
                strcpy(pb, b"Some food\0" as *const u8 as *const libc::c_char);
            } else {
                sprintf(
                    pb,
                    b"%d rations of food\0" as *const u8 as *const libc::c_char,
                    (*obj)._o._o_count,
                );
            }
        }
        24 => {
            if (*obj)._o._o_count > 1 as libc::c_int {
                sprintf(
                    pb,
                    b"%d \0" as *const u8 as *const libc::c_char,
                    (*obj)._o._o_count,
                );
            } else {
                sprintf(
                    pb,
                    b"A%s \0" as *const u8 as *const libc::c_char,
                    vowelstr(*w_names.as_mut_ptr().offset(which as isize)),
                );
            }
            pb = &mut *prbuf
                .offset(
                    (strlen
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                        ) -> libc::c_ulong)(prbuf) as isize,
                ) as *mut libc::c_char;
            if (*obj)._o._o_flags as libc::c_int & 0x2 as libc::c_int != 0 {
                sprintf(
                    pb,
                    b"%s %s\0" as *const u8 as *const libc::c_char,
                    num(
                        (*obj)._o._o_hplus,
                        (*obj)._o._o_dplus,
                        0x18 as libc::c_int as libc::c_char,
                    ),
                    *w_names.as_mut_ptr().offset(which as isize),
                );
            } else {
                sprintf(
                    pb,
                    b"%s\0" as *const u8 as *const libc::c_char,
                    *w_names.as_mut_ptr().offset(which as isize),
                );
            }
            if (*obj)._o._o_count > 1 as libc::c_int {
                strcat(pb, b"s\0" as *const u8 as *const libc::c_char);
            }
            if (*obj)._o._o_enemy as libc::c_int != 0
                && (*obj)._o._o_flags as libc::c_int & 0x40 as libc::c_int != 0
            {
                strcat(pb, b" of \0" as *const u8 as *const libc::c_char);
                strcat(
                    pb,
                    (*monsters
                        .as_mut_ptr()
                        .offset(
                            ((*obj)._o._o_enemy as libc::c_int - 'A' as i32) as isize,
                        ))
                        .m_name,
                );
                strcat(pb, b" slaying\0" as *const u8 as *const libc::c_char);
            }
        }
        8 => {
            if (*obj)._o._o_flags as libc::c_int & 0x2 as libc::c_int != 0 {
                chopmsg(
                    pb,
                    b"%s %s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    b"%s %s [armor class %d]\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    num(
                        *a_class.as_mut_ptr().offset(which as isize)
                            - (*obj)._o._o_ac as libc::c_int,
                        0 as libc::c_int,
                        0x8 as libc::c_int as libc::c_char,
                    ),
                    *a_names.as_mut_ptr().offset(which as isize),
                    -((*obj)._o._o_ac as libc::c_int - 11 as libc::c_int),
                );
            } else {
                sprintf(
                    pb,
                    b"%s\0" as *const u8 as *const libc::c_char,
                    *a_names.as_mut_ptr().offset(which as isize),
                );
            }
        }
        12 => {
            strcpy(pb, b"The Amulet of Yendor\0" as *const u8 as *const libc::c_char);
        }
        231 => {
            sprintf(
                pb,
                b"A%s %s \0" as *const u8 as *const libc::c_char,
                vowelstr(*ws_type.as_mut_ptr().offset(which as isize)),
                *ws_type.as_mut_ptr().offset(which as isize),
            );
            pb = &mut *prbuf
                .offset(
                    (strlen
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                        ) -> libc::c_ulong)(prbuf) as isize,
                ) as *mut libc::c_char;
            if *ws_know.as_mut_ptr().offset(which as isize) {
                chopmsg(
                    pb,
                    b"of %s%s\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    b"of %s%s(%s)\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    (*ws_magic.as_mut_ptr().offset(which as isize)).mi_name,
                    charge_str(obj),
                    *ws_made.as_mut_ptr().offset(which as isize),
                );
            } else if **ws_guess.as_mut_ptr().offset(which as isize) != 0 {
                chopmsg(
                    pb,
                    b"called %s\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    b"called %s(%s)\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    *ws_guess.as_mut_ptr().offset(which as isize),
                    *ws_made.as_mut_ptr().offset(which as isize),
                );
            } else {
                pb = &mut *prbuf.offset(2 as libc::c_int as isize) as *mut libc::c_char;
                sprintf(
                    pb,
                    b"%s %s\0" as *const u8 as *const libc::c_char,
                    *ws_made.as_mut_ptr().offset(which as isize),
                    *ws_type.as_mut_ptr().offset(which as isize),
                );
            }
        }
        9 => {
            if *r_know.as_mut_ptr().offset(which as isize) {
                chopmsg(
                    pb,
                    b"A%s ring of %s\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    b"A%s ring of %s(%s)\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    ring_num(obj),
                    (*r_magic.as_mut_ptr().offset(which as isize)).mi_name,
                    *r_stones.as_mut_ptr().offset(which as isize),
                );
            } else if **r_guess.as_mut_ptr().offset(which as isize) != 0 {
                chopmsg(
                    pb,
                    b"A ring called %s\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    b"A ring called %s(%s)\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    *r_guess.as_mut_ptr().offset(which as isize),
                    *r_stones.as_mut_ptr().offset(which as isize),
                );
            } else {
                sprintf(
                    pb,
                    b"A%s %s ring\0" as *const u8 as *const libc::c_char,
                    vowelstr(*r_stones.as_mut_ptr().offset(which as isize)),
                    *r_stones.as_mut_ptr().offset(which as isize),
                );
            }
        }
        _ => {}
    }
    if obj == cur_armor {
        strcat(pb, b" (being worn)\0" as *const u8 as *const libc::c_char);
    }
    if obj == cur_weapon {
        strcat(pb, b" (weapon in hand)\0" as *const u8 as *const libc::c_char);
    }
    if obj == *cur_ring.as_mut_ptr().offset(0 as libc::c_int as isize) {
        strcat(pb, b" (on left hand)\0" as *const u8 as *const libc::c_char);
    } else if obj == *cur_ring.as_mut_ptr().offset(1 as libc::c_int as isize) {
        strcat(pb, b" (on right hand)\0" as *const u8 as *const libc::c_char);
    }
    if drop_1 as libc::c_int != 0
        && (*prbuf.offset(0 as libc::c_int as isize) as libc::c_int >= 'A' as i32
            && *prbuf.offset(0 as libc::c_int as isize) as libc::c_int <= 'Z' as i32)
    {
        *prbuf
            .offset(
                0 as libc::c_int as isize,
            ) = ({
            let mut __res: libc::c_int = 0;
            if ::core::mem::size_of::<libc::c_char>() as libc::c_ulong
                > 1 as libc::c_int as libc::c_ulong
            {
                if 0 != 0 {
                    let mut __c: libc::c_int = *prbuf.offset(0 as libc::c_int as isize)
                        as libc::c_int;
                    __res = if __c < -(128 as libc::c_int) || __c > 255 as libc::c_int {
                        __c
                    } else {
                        *(*__ctype_tolower_loc()).offset(__c as isize)
                    };
                } else {
                    __res = tolower(
                        *prbuf.offset(0 as libc::c_int as isize) as libc::c_int,
                    );
                }
            } else {
                __res = *(*__ctype_tolower_loc())
                    .offset(
                        *prbuf.offset(0 as libc::c_int as isize) as libc::c_int as isize,
                    );
            }
            __res
        }) as libc::c_char;
    } else if !drop_1 && is_lower(*prbuf) as libc::c_int != 0 {
        *prbuf = ({
            let mut __res: libc::c_int = 0;
            if ::core::mem::size_of::<libc::c_char>() as libc::c_ulong
                > 1 as libc::c_int as libc::c_ulong
            {
                if 0 != 0 {
                    let mut __c: libc::c_int = *prbuf as libc::c_int;
                    __res = if __c < -(128 as libc::c_int) || __c > 255 as libc::c_int {
                        __c
                    } else {
                        *(*__ctype_toupper_loc()).offset(__c as isize)
                    };
                } else {
                    __res = toupper(*prbuf as libc::c_int);
                }
            } else {
                __res = *(*__ctype_toupper_loc()).offset(*prbuf as libc::c_int as isize);
            }
            __res
        }) as libc::c_char;
    }
    return prbuf;
}
unsafe extern "C" fn chopmsg(
    mut s: *mut libc::c_char,
    mut shmsg: *mut libc::c_char,
    mut lnmsg: *mut libc::c_char,
    mut args: ...
) {
    let mut argp: ::core::ffi::VaListImpl;
    argp = args.clone();
    vsnprintf(
        s,
        80 as libc::c_int as libc::c_ulong,
        if terse as libc::c_int != 0 || expert as libc::c_int != 0 {
            shmsg
        } else {
            lnmsg
        },
        argp.as_va_list(),
    );
}
#[export_name = "drop"]
pub unsafe extern "C" fn drop_0() {
    let mut ch: byte = 0;
    let mut nobj: *mut THING = 0 as *mut THING;
    let mut op: *mut THING = 0 as *mut THING;
    ch = *_level.offset(INDEX(player._t._t_pos.y, player._t._t_pos.x) as isize);
    if ch as libc::c_int != 0xfa as libc::c_int
        && ch as libc::c_int != 0xb1 as libc::c_int
    {
        msg(b"there is something there already\0" as *const u8 as *const libc::c_char);
        return;
    }
    op = get_item(
        b"drop\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    if op.is_null() {
        return;
    }
    if !can_drop(op) {
        return;
    }
    if (*op)._o._o_count >= 2 as libc::c_int && (*op)._o._o_type != 0x18 as libc::c_int {
        nobj = new_item();
        if nobj.is_null() {
            msg(
                b"%sit appears to be stuck in your pack!\0" as *const u8
                    as *const libc::c_char,
                noterse(
                    b"can't drop it, \0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                ),
            );
            return;
        }
        (*op)._o._o_count -= 1;
        memmove(
            nobj as *mut libc::c_void,
            op as *const libc::c_void,
            ::core::mem::size_of::<THING>() as libc::c_ulong,
        );
        (*nobj)._o._o_count = 1 as libc::c_int;
        op = nobj;
        if (*op)._o._o_group != 0 as libc::c_int {
            inpack += 1;
        }
    } else {
        list_detach(&mut player._t._t_pack, op);
    }
    inpack -= 1;
    list_attach(&mut lvl_obj, op);
    *_level
        .offset(
            INDEX(player._t._t_pos.y, player._t._t_pos.x) as isize,
        ) = (*op)._o._o_type as byte;
    memmove(
        &mut (*op)._o._o_pos as *mut coord as *mut libc::c_void,
        &mut player._t._t_pos as *mut coord as *const libc::c_void,
        ::core::mem::size_of::<coord>() as libc::c_ulong,
    );
    if (*op)._o._o_type == 0xc as libc::c_int {
        amulet = 0 as libc::c_int != 0;
    }
    msg(
        b"dropped %s\0" as *const u8 as *const libc::c_char,
        inv_name(op, 1 as libc::c_int != 0),
    );
}
#[no_mangle]
pub unsafe extern "C" fn can_drop(mut op: *mut THING) -> bool {
    if op.is_null() {
        return 1 as libc::c_int != 0;
    }
    if op != cur_armor && op != cur_weapon
        && op != *cur_ring.as_mut_ptr().offset(0 as libc::c_int as isize)
        && op != *cur_ring.as_mut_ptr().offset(1 as libc::c_int as isize)
    {
        return 1 as libc::c_int != 0;
    }
    if (*op)._o._o_flags as libc::c_int & 0x1 as libc::c_int != 0 {
        msg(
            b"you can't.  It appears to be cursed\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    if op == cur_weapon {
        cur_weapon = 0 as *mut THING;
    } else if op == cur_armor {
        waste_time();
        cur_armor = 0 as *mut THING;
    } else {
        let mut hand: libc::c_int = 0;
        hand = 0 as libc::c_int;
        if op != *cur_ring.as_mut_ptr().offset(hand as isize) {
            hand = 1 as libc::c_int;
            if op != *cur_ring.as_mut_ptr().offset(hand as isize) {
                return 1 as libc::c_int != 0;
            }
        }
        let ref mut fresh0 = *cur_ring.as_mut_ptr().offset(hand as isize);
        *fresh0 = 0 as *mut THING;
        match (*op)._o._o_which {
            1 => {
                chg_str(-((*op)._o._o_ac as libc::c_int));
            }
            4 => {
                unsee();
                extinguish(
                    ::core::mem::transmute::<
                        Option::<unsafe extern "C" fn() -> ()>,
                        Option::<unsafe extern "C" fn() -> ()>,
                    >(Some(unsee as unsafe extern "C" fn() -> ())),
                );
            }
            _ => {}
        }
    }
    return 1 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn new_thing() -> *mut THING {
    let mut cur: *mut THING = 0 as *mut THING;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    cur = new_item();
    if cur.is_null() {
        return 0 as *mut THING;
    }
    (*cur)._o._o_dplus = 0 as libc::c_int;
    (*cur)._o._o_hplus = (*cur)._o._o_dplus;
    (*cur)
        ._o
        ._o_hurldmg = b"0d0\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    (*cur)._o._o_damage = (*cur)._o._o_hurldmg;
    (*cur)._o._o_ac = 11 as libc::c_int as libc::c_short;
    (*cur)._o._o_count = 1 as libc::c_int;
    (*cur)._o._o_group = 0 as libc::c_int;
    (*cur)._o._o_flags = 0 as libc::c_int as libc::c_short;
    (*cur)._o._o_enemy = 0 as libc::c_int as libc::c_char;
    match if no_food > 3 as libc::c_int {
        2 as libc::c_int
    } else {
        pick_one(things.as_mut_ptr(), 7 as libc::c_int)
    } {
        0 => {
            (*cur)._o._o_type = 0xad as libc::c_int;
            (*cur)._o._o_which = pick_one(p_magic.as_mut_ptr(), 14 as libc::c_int);
        }
        1 => {
            (*cur)._o._o_type = 0xd as libc::c_int;
            (*cur)._o._o_which = pick_one(s_magic.as_mut_ptr(), 15 as libc::c_int);
        }
        2 => {
            no_food = 0 as libc::c_int;
            (*cur)._o._o_type = 0x5 as libc::c_int;
            if rnd(10 as libc::c_int) != 0 as libc::c_int {
                (*cur)._o._o_which = 0 as libc::c_int;
            } else {
                (*cur)._o._o_which = 1 as libc::c_int;
            }
        }
        3 => {
            (*cur)._o._o_type = 0x18 as libc::c_int;
            (*cur)._o._o_which = rnd(10 as libc::c_int);
            init_weapon(cur, (*cur)._o._o_which as byte);
            k = rnd(100 as libc::c_int);
            if k < 10 as libc::c_int {
                (*cur)
                    ._o
                    ._o_flags = ((*cur)._o._o_flags as libc::c_int | 0x1 as libc::c_int)
                    as libc::c_short;
                (*cur)._o._o_hplus -= rnd(3 as libc::c_int) + 1 as libc::c_int;
            } else if k < 15 as libc::c_int {
                (*cur)._o._o_hplus += rnd(3 as libc::c_int) + 1 as libc::c_int;
            }
        }
        4 => {
            (*cur)._o._o_type = 0x8 as libc::c_int;
            j = 0 as libc::c_int;
            k = rnd(100 as libc::c_int);
            while j < 8 as libc::c_int {
                if k < *a_chances.as_mut_ptr().offset(j as isize) {
                    break;
                }
                j += 1;
            }
            (*cur)._o._o_which = j;
            (*cur)._o._o_ac = *a_class.as_mut_ptr().offset(j as isize) as libc::c_short;
            k = rnd(100 as libc::c_int);
            if k < 20 as libc::c_int {
                (*cur)
                    ._o
                    ._o_flags = ((*cur)._o._o_flags as libc::c_int | 0x1 as libc::c_int)
                    as libc::c_short;
                (*cur)
                    ._o
                    ._o_ac = ((*cur)._o._o_ac as libc::c_int
                    + (rnd(3 as libc::c_int) + 1 as libc::c_int)) as libc::c_short;
            } else if k < 28 as libc::c_int {
                (*cur)
                    ._o
                    ._o_ac = ((*cur)._o._o_ac as libc::c_int
                    - (rnd(3 as libc::c_int) + 1 as libc::c_int)) as libc::c_short;
            }
        }
        5 => {
            (*cur)._o._o_type = 0x9 as libc::c_int;
            (*cur)._o._o_which = pick_one(r_magic.as_mut_ptr(), 14 as libc::c_int);
            match (*cur)._o._o_which {
                1 | 0 | 7 | 8 => {
                    (*cur)._o._o_ac = rnd(3 as libc::c_int) as libc::c_short;
                    if (*cur)._o._o_ac as libc::c_int == 0 as libc::c_int {
                        (*cur)._o._o_ac = -(1 as libc::c_int) as libc::c_short;
                        (*cur)
                            ._o
                            ._o_flags = ((*cur)._o._o_flags as libc::c_int
                            | 0x1 as libc::c_int) as libc::c_short;
                    }
                }
                6 | 11 => {
                    (*cur)
                        ._o
                        ._o_flags = ((*cur)._o._o_flags as libc::c_int
                        | 0x1 as libc::c_int) as libc::c_short;
                }
                _ => {}
            }
        }
        6 => {
            (*cur)._o._o_type = 0xe7 as libc::c_int;
            (*cur)._o._o_which = pick_one(ws_magic.as_mut_ptr(), 14 as libc::c_int);
            fix_stick(cur);
        }
        _ => {}
    }
    return cur;
}
unsafe extern "C" fn pick_one(
    mut magic: *mut magic_item,
    mut nitems: libc::c_int,
) -> libc::c_int {
    let mut end: *mut magic_item = 0 as *mut magic_item;
    let mut i: libc::c_int = 0;
    let mut start: *mut magic_item = 0 as *mut magic_item;
    start = magic;
    end = &mut *magic.offset(nitems as isize) as *mut magic_item;
    i = rnd(100 as libc::c_int);
    while magic < end {
        if i < (*magic).mi_prob {
            break;
        }
        magic = magic.offset(1);
    }
    if magic == end {
        magic = start;
    }
    return magic.offset_from(start) as libc::c_long as libc::c_int;
}
static mut line_cnt: libc::c_int = 0 as libc::c_int;
static mut newpage: bool = 0 as libc::c_int != 0;
static mut lastfmt: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut lastarg: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub unsafe extern "C" fn discovered() {
    print_disc(0xad as libc::c_int as byte);
    add_line(
        nullstr.as_mut_ptr(),
        b" \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    print_disc(0xd as libc::c_int as byte);
    add_line(
        nullstr.as_mut_ptr(),
        b" \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    print_disc(0x9 as libc::c_int as byte);
    add_line(
        nullstr.as_mut_ptr(),
        b" \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    print_disc(0xe7 as libc::c_int as byte);
    end_line(nullstr.as_mut_ptr());
}
unsafe extern "C" fn print_disc(mut type_0: byte) {
    let mut know: *mut bool = 0 as *mut bool;
    let mut guess: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut maxnum: libc::c_int = 0 as libc::c_int;
    let mut num_found: libc::c_int = 0;
    static mut obj: THING = thing {
        _t: C2RustUnnamed_0 {
            _l_next: 0 as *const thing as *mut thing,
            _l_prev: 0 as *const thing as *mut thing,
            _t_pos: coord { x: 0, y: 0 },
            _t_turn: 0,
            _t_type: 0,
            _t_disguise: 0,
            _t_oldch: 0,
            _t_dest: 0 as *const coord as *mut coord,
            _t_flags: 0,
            _t_stats: stats {
                s_str: 0,
                s_exp: 0,
                s_lvl: 0,
                s_arm: 0,
                s_hpt: 0,
                s_dmg: 0 as *const libc::c_char as *mut libc::c_char,
                s_maxhp: 0,
            },
            _t_room: 0 as *const room as *mut room,
            _t_pack: 0 as *const thing as *mut thing,
        },
    };
    static mut order: [libc::c_short; 15] = [0; 15];
    match type_0 as libc::c_int {
        13 => {
            maxnum = 15 as libc::c_int;
            know = s_know.as_mut_ptr();
            guess = s_guess.as_mut_ptr();
        }
        173 => {
            maxnum = 14 as libc::c_int;
            know = p_know.as_mut_ptr();
            guess = p_guess.as_mut_ptr();
        }
        9 => {
            maxnum = 14 as libc::c_int;
            know = r_know.as_mut_ptr();
            guess = r_guess.as_mut_ptr();
        }
        231 => {
            maxnum = 14 as libc::c_int;
            know = ws_know.as_mut_ptr();
            guess = ws_guess.as_mut_ptr();
        }
        _ => {}
    }
    set_order(order.as_mut_ptr(), maxnum);
    obj._o._o_count = 1 as libc::c_int;
    obj._o._o_flags = 0 as libc::c_int as libc::c_short;
    num_found = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < maxnum {
        if *know.offset(order[i as usize] as isize) as libc::c_int != 0
            || **guess.offset(order[i as usize] as isize) as libc::c_int != 0
        {
            obj._o._o_type = type_0 as libc::c_int;
            obj._o._o_which = order[i as usize] as libc::c_int;
            add_line(
                nullstr.as_mut_ptr(),
                b"%s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                inv_name(&mut obj, 0 as libc::c_int != 0),
            );
            num_found += 1;
        }
        i += 1;
    }
    if num_found == 0 as libc::c_int {
        add_line(
            nullstr.as_mut_ptr(),
            nothing(type_0),
            b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
}
unsafe extern "C" fn set_order(
    mut order: *mut libc::c_short,
    mut numthings: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < numthings {
        *order.offset(i as isize) = i as libc::c_short;
        i += 1;
    }
    i = numthings;
    while i > 0 as libc::c_int {
        r = rnd(i);
        t = *order.offset((i - 1 as libc::c_int) as isize) as libc::c_int;
        *order.offset((i - 1 as libc::c_int) as isize) = *order.offset(r as isize);
        *order.offset(r as isize) = t as libc::c_short;
        i -= 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn add_line(
    mut use_0: *mut libc::c_char,
    mut fmt: *mut libc::c_char,
    mut arg: *mut libc::c_char,
) -> byte {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut retchar: byte = ' ' as i32 as byte;
    if line_cnt == 0 as libc::c_int {
        wdump();
        cur_clear();
    }
    if line_cnt >= LINES - 1 as libc::c_int || fmt.is_null() {
        cur_move(LINES - 1 as libc::c_int, 0 as libc::c_int);
        if *use_0 != 0 {
            cur_printw(
                b"-Select item to %s. Esc to cancel-\0" as *const u8
                    as *const libc::c_char,
                use_0,
            );
        } else {
            cur_addstr(
                b"-Press space to continue-\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
        }
        loop {
            retchar = readchar();
            if !(retchar as libc::c_int != 27 as libc::c_int
                && retchar as libc::c_int != ' ' as i32
                && !is_lower(retchar as libc::c_char))
            {
                break;
            }
        }
        cur_clear();
        newpage = 1 as libc::c_int != 0;
        line_cnt = 0 as libc::c_int;
    }
    if !fmt.is_null()
        && !(line_cnt == 0 as libc::c_int && *fmt as libc::c_int == '\0' as i32)
    {
        cur_move(line_cnt, 0 as libc::c_int);
        cur_printw(fmt, arg);
        getrc(&mut x, &mut y);
        if y != 0 as libc::c_int {
            line_cnt = x + 1 as libc::c_int;
        }
        lastfmt = fmt;
        lastarg = arg;
    }
    return retchar;
}
#[no_mangle]
pub unsafe extern "C" fn end_line(mut use_0: *mut libc::c_char) -> byte {
    let mut retchar: libc::c_int = 0;
    retchar = add_line(
        use_0,
        0 as *mut libc::c_char,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as libc::c_int;
    wrestor();
    line_cnt = 0 as libc::c_int;
    newpage = 0 as libc::c_int != 0;
    return retchar as byte;
}
unsafe extern "C" fn nothing(mut type_0: byte) -> *mut libc::c_char {
    let mut sp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tystr: *mut libc::c_char = 0 as *mut libc::c_char;
    sprintf(prbuf, b"Haven't discovered anything\0" as *const u8 as *const libc::c_char);
    if terse {
        sprintf(prbuf, b"Nothing\0" as *const u8 as *const libc::c_char);
    }
    sp = &mut *prbuf
        .offset(
            (strlen as unsafe extern "C" fn(*const libc::c_char) -> libc::c_ulong)(prbuf)
                as isize,
        ) as *mut libc::c_char;
    match type_0 as libc::c_int {
        173 => {
            tystr = b"potion\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
        13 => {
            tystr = b"scroll\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
        9 => {
            tystr = b"ring\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
        231 => {
            tystr = b"stick\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
        _ => {
            tystr = b"item\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
    }
    sprintf(sp, b" about any %ss\0" as *const u8 as *const libc::c_char, tystr);
    return prbuf;
}
