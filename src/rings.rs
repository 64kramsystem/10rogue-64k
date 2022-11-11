use ::libc;
extern "C" {
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn readchar() -> byte;
    static mut after: bool;
    static mut mpos: libc::c_int;
    static mut cur_ring: [*mut THING; 0];
    static mut ring_buf: *mut libc::c_char;
    fn msg(fmt: *const libc::c_char, _: ...);
    fn noterse(str: *mut libc::c_char) -> *mut libc::c_char;
    fn rnd(range: libc::c_int) -> libc::c_int;
    fn chg_str(amt: libc::c_int);
    fn aggravate();
    fn is_current(obj: *mut THING) -> bool;
    fn get_item(purpose: *mut libc::c_char, type_0: libc::c_int) -> *mut THING;
    fn pack_char(obj: *mut THING) -> byte;
    fn invis_on();
    fn inv_name(obj: *mut THING, drop_0: bool) -> *mut libc::c_char;
    fn can_drop(op: *mut THING) -> bool;
    fn num(n1: libc::c_int, n2: libc::c_int, type_0: libc::c_char) -> *mut libc::c_char;
}
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
#[no_mangle]
pub unsafe extern "C" fn ring_on() {
    let mut current_block: u64;
    let mut obj: *mut THING = 0 as *mut THING;
    let mut ring: libc::c_int = -(1 as libc::c_int);
    obj = get_item(
        b"put on\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0x9 as libc::c_int,
    );
    if !obj.is_null() {
        if (*obj)._o._o_type != 0x9 as libc::c_int {
            msg(
                b"you can't put that on your finger\0" as *const u8
                    as *const libc::c_char,
            );
        } else if !is_current(obj) {
            if (*cur_ring.as_mut_ptr().offset(0 as libc::c_int as isize)).is_null() {
                ring = 0 as libc::c_int;
            }
            if (*cur_ring.as_mut_ptr().offset(1 as libc::c_int as isize)).is_null() {
                ring = 1 as libc::c_int;
            }
            if (*cur_ring.as_mut_ptr().offset(0 as libc::c_int as isize)).is_null()
                && (*cur_ring.as_mut_ptr().offset(1 as libc::c_int as isize)).is_null()
            {
                ring = gethand();
                if ring < 0 as libc::c_int {
                    current_block = 16077025435993566754;
                } else {
                    current_block = 1394248824506584008;
                }
            } else {
                current_block = 1394248824506584008;
            }
            match current_block {
                16077025435993566754 => {}
                _ => {
                    if ring < 0 as libc::c_int {
                        msg(
                            b"you already have a ring on each hand\0" as *const u8
                                as *const libc::c_char,
                        );
                    } else {
                        let ref mut fresh0 = *cur_ring
                            .as_mut_ptr()
                            .offset(ring as isize);
                        *fresh0 = obj;
                        match (*obj)._o._o_which {
                            1 => {
                                chg_str((*obj)._o._o_ac as libc::c_int);
                            }
                            4 => {
                                invis_on();
                            }
                            6 => {
                                aggravate();
                            }
                            _ => {}
                        }
                        msg(
                            b"%swearing %s (%c)\0" as *const u8 as *const libc::c_char,
                            noterse(
                                b"you are now \0" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char,
                            ),
                            inv_name(obj, 1 as libc::c_int != 0),
                            pack_char(obj) as libc::c_int,
                        );
                        return;
                    }
                }
            }
        }
    }
    after = 0 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn ring_off() {
    let mut ring: libc::c_int = 0;
    let mut obj: *mut THING = 0 as *mut THING;
    let mut packchar: libc::c_char = 0;
    if (*cur_ring.as_mut_ptr().offset(0 as libc::c_int as isize)).is_null()
        && (*cur_ring.as_mut_ptr().offset(1 as libc::c_int as isize)).is_null()
    {
        msg(b"you aren't wearing any rings\0" as *const u8 as *const libc::c_char);
        after = 0 as libc::c_int != 0;
        return;
    } else {
        if (*cur_ring.as_mut_ptr().offset(0 as libc::c_int as isize)).is_null() {
            ring = 1 as libc::c_int;
        } else if (*cur_ring.as_mut_ptr().offset(1 as libc::c_int as isize)).is_null() {
            ring = 0 as libc::c_int;
        } else {
            ring = gethand();
            if ring < 0 as libc::c_int {
                return;
            }
        }
    }
    mpos = 0 as libc::c_int;
    obj = *cur_ring.as_mut_ptr().offset(ring as isize);
    if obj.is_null() {
        msg(b"not wearing such a ring\0" as *const u8 as *const libc::c_char);
        after = 0 as libc::c_int != 0;
        return;
    }
    packchar = pack_char(obj) as libc::c_char;
    if can_drop(obj) {
        msg(
            b"was wearing %s(%c)\0" as *const u8 as *const libc::c_char,
            inv_name(obj, 1 as libc::c_int != 0),
            packchar as libc::c_int,
        );
    }
}
unsafe extern "C" fn gethand() -> libc::c_int {
    let mut c: libc::c_int = 0;
    loop {
        msg(b"left hand or right hand? \0" as *const u8 as *const libc::c_char);
        c = readchar() as libc::c_int;
        if c == 27 as libc::c_int {
            after = 0 as libc::c_int != 0;
            return -(1 as libc::c_int);
        }
        mpos = 0 as libc::c_int;
        if c == 'l' as i32 || c == 'L' as i32 {
            return 0 as libc::c_int
        } else {
            if c == 'r' as i32 || c == 'R' as i32 {
                return 1 as libc::c_int;
            }
        }
        msg(b"please type L or R\0" as *const u8 as *const libc::c_char);
    };
}
#[no_mangle]
pub unsafe extern "C" fn ring_eat(mut hand: libc::c_int) -> libc::c_int {
    if (*cur_ring.as_mut_ptr().offset(hand as isize)).is_null() {
        return 0 as libc::c_int;
    }
    match (**cur_ring.as_mut_ptr().offset(hand as isize))._o._o_which {
        9 => return 2 as libc::c_int,
        2 | 13 | 0 | 1 | 12 => return 1 as libc::c_int,
        3 => return (rnd(5 as libc::c_int) == 0 as libc::c_int) as libc::c_int,
        7 | 8 => return (rnd(3 as libc::c_int) == 0 as libc::c_int) as libc::c_int,
        10 => return -rnd(2 as libc::c_int),
        4 => return (rnd(5 as libc::c_int) == 0 as libc::c_int) as libc::c_int,
        _ => return 0 as libc::c_int,
    };
}
#[no_mangle]
pub unsafe extern "C" fn ring_num(mut obj: *mut THING) -> *mut libc::c_char {
    if (*obj)._o._o_flags as libc::c_int & 0x2 as libc::c_int == 0 {
        return b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    match (*obj)._o._o_which {
        0 | 1 | 8 | 7 => {
            *ring_buf.offset(0 as libc::c_int as isize) = ' ' as i32 as libc::c_char;
            strcpy(
                &mut *ring_buf.offset(1 as libc::c_int as isize),
                num(
                    (*obj)._o._o_ac as libc::c_int,
                    0 as libc::c_int,
                    0x9 as libc::c_int as libc::c_char,
                ),
            );
        }
        _ => return b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    }
    return ring_buf;
}
