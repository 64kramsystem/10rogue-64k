use ::libc;
extern "C" {
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    static mut maxrow: libc::c_int;
    static mut s_guess: [*mut libc::c_char; 0];
    static mut expert: bool;
    static mut terse: bool;
    static mut s_know: [bool; 0];
    static mut flashmsg: *mut libc::c_char;
    static mut intense: *mut libc::c_char;
    static mut w_names: [*mut libc::c_char; 0];
    static mut inpack: libc::c_int;
    static mut no_command: libc::c_int;
    static mut cur_armor: *mut THING;
    static mut cur_ring: [*mut THING; 0];
    static mut cur_weapon: *mut THING;
    static mut lvl_obj: *mut THING;
    static mut player: THING;
    static mut s_menu: [libc::c_char; 0];
    static mut _level: *mut byte;
    static mut _flags: *mut byte;
    fn ifterse(tfmt: *const libc::c_char, fmt: *const libc::c_char, _: ...);
    fn msg(fmt: *const libc::c_char, _: ...);
    fn more(msg_0: *mut libc::c_char);
    fn status();
    fn new_item() -> *mut THING;
    fn list_detach(list: *mut *mut THING, item: *mut THING);
    fn discard(item: *mut THING) -> libc::c_int;
    fn rnd(range: libc::c_int) -> libc::c_int;
    fn look(wakeup: bool);
    fn aggravate();
    fn call_it(know: bool, guess: *mut *mut libc::c_char);
    fn spread(nm: libc::c_int) -> libc::c_int;
    fn INDEX(y: libc::c_int, x: libc::c_int) -> libc::c_int;
    fn randmonster(wander: bool) -> libc::c_char;
    fn pick_mons() -> libc::c_char;
    fn new_monster(tp: *mut THING, type_0: byte, cp: *mut coord);
    fn moat(my: libc::c_int, mx: libc::c_int) -> *mut THING;
    fn get_item(purpose: *mut libc::c_char, type_0: libc::c_int) -> *mut THING;
    fn teleport() -> libc::c_int;
    fn whatis();
    fn plop_monster(r: libc::c_int, c: libc::c_int, cp: *mut coord) -> bool;
    fn cur_mvaddch(r: libc::c_int, c: libc::c_int, chr: byte);
    fn set_attr(bute: libc::c_int);
    static mut COLS: libc::c_int;
    fn cur_inch() -> byte;
    fn cur_move(row: libc::c_int, col: libc::c_int) -> libc::c_int;
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
pub static mut laugh: *mut libc::c_char = b"you hear maniacal laughter%s.\0" as *const u8
    as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut in_dist: *mut libc::c_char = b" in the distance\0" as *const u8
    as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub unsafe extern "C" fn read_scroll() {
    let mut obj: *mut THING = 0 as *mut THING;
    let mut y: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut ch: byte = 0;
    let mut op: *mut THING = 0 as *mut THING;
    let mut index: libc::c_int = 0;
    let mut discardit: bool = 0 as libc::c_int != 0;
    obj = get_item(
        b"read\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0xd as libc::c_int,
    );
    if obj.is_null() {
        return;
    }
    if (*obj)._o._o_type != 0xd as libc::c_int {
        msg(b"there is nothing on it to read\0" as *const u8 as *const libc::c_char);
        return;
    }
    ifterse(
        b"the scroll vanishes\0" as *const u8 as *const libc::c_char,
        b"as you read the scroll, it vanishes\0" as *const u8 as *const libc::c_char,
    );
    if obj == cur_weapon {
        cur_weapon = 0 as *mut THING;
    }
    match (*obj)._o._o_which {
        0 => {
            player
                ._t
                ._t_flags = (player._t._t_flags as libc::c_int | 0x400 as libc::c_int)
                as libc::c_short;
            msg(b"your hands begin to glow red\0" as *const u8 as *const libc::c_char);
        }
        4 => {
            if !cur_armor.is_null() {
                (*cur_armor)._o._o_ac -= 1;
                (*cur_armor)
                    ._o
                    ._o_flags = ((*cur_armor)._o._o_flags as libc::c_int
                    & !(0x1 as libc::c_int)) as libc::c_short;
                ifterse(
                    b"your armor glows faintly\0" as *const u8 as *const libc::c_char,
                    b"your armor glows faintly for a moment\0" as *const u8
                        as *const libc::c_char,
                );
            }
        }
        2 => {
            x = player._t._t_pos.x - 3 as libc::c_int;
            while x <= player._t._t_pos.x + 3 as libc::c_int {
                if x >= 0 as libc::c_int && x < COLS {
                    y = player._t._t_pos.y - 3 as libc::c_int;
                    while y <= player._t._t_pos.y + 3 as libc::c_int {
                        if y > 0 as libc::c_int && y < maxrow
                            && {
                                op = moat(y, x);
                                !op.is_null()
                            }
                        {
                            (*op)
                                ._t
                                ._t_flags = ((*op)._t._t_flags as libc::c_int
                                & !(0x4 as libc::c_int)) as libc::c_short;
                            (*op)
                                ._t
                                ._t_flags = ((*op)._t._t_flags as libc::c_int
                                | 0x80 as libc::c_int) as libc::c_short;
                        }
                        y += 1;
                    }
                }
                x += 1;
            }
        }
        3 => {
            *s_know
                .as_mut_ptr()
                .offset(3 as libc::c_int as isize) = 1 as libc::c_int != 0;
            no_command += rnd(spread(5 as libc::c_int)) + 4 as libc::c_int;
            player
                ._t
                ._t_flags = (player._t._t_flags as libc::c_int & !(0x4 as libc::c_int))
                as libc::c_short;
            msg(b"you fall asleep\0" as *const u8 as *const libc::c_char);
        }
        10 => {
            let mut mp: coord = coord { x: 0, y: 0 };
            if plop_monster(player._t._t_pos.y, player._t._t_pos.x, &mut mp)
                as libc::c_int != 0
                && {
                    op = new_item();
                    !op.is_null()
                }
            {
                new_monster(op, randmonster(0 as libc::c_int != 0) as byte, &mut mp);
            } else {
                ifterse(
                    b"you hear a faint cry of anguish\0" as *const u8
                        as *const libc::c_char,
                    b"you hear a faint cry of anguish in the distance\0" as *const u8
                        as *const libc::c_char,
                );
            }
        }
        5 => {
            *s_know
                .as_mut_ptr()
                .offset(5 as libc::c_int as isize) = 1 as libc::c_int != 0;
            msg(
                b"this scroll is an identify scroll\0" as *const u8
                    as *const libc::c_char,
            );
            if strcmp(s_menu.as_mut_ptr(), b"on\0" as *const u8 as *const libc::c_char)
                == 0
                || strcmp(
                    s_menu.as_mut_ptr(),
                    b"sel\0" as *const u8 as *const libc::c_char,
                ) == 0
            {
                more(
                    b" More \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
            }
            whatis();
        }
        1 => {
            *s_know
                .as_mut_ptr()
                .offset(1 as libc::c_int as isize) = 1 as libc::c_int != 0;
            msg(
                b"oh, now this scroll has a map on it\0" as *const u8
                    as *const libc::c_char,
            );
            y = 1 as libc::c_int;
            while y < maxrow {
                x = 0 as libc::c_int;
                while x < COLS {
                    index = INDEX(y, x);
                    let mut current_block_51: u64;
                    ch = *_level.offset(index as isize);
                    match ch as libc::c_int {
                        186 | 205 | 201 | 187 | 200 | 188 => {
                            if *_flags.offset(index as isize) as libc::c_int
                                & 0x10 as libc::c_int == 0
                            {
                                let ref mut fresh0 = *_level.offset(index as isize);
                                *fresh0 = 0xce as libc::c_int as byte;
                                ch = *fresh0;
                                let ref mut fresh1 = *_flags.offset(index as isize);
                                *fresh1 = (*fresh1 as libc::c_int & !(0x10 as libc::c_int))
                                    as byte;
                            }
                            current_block_51 = 17404231275461091374;
                        }
                        206 | 177 | 240 => {
                            current_block_51 = 17404231275461091374;
                        }
                        _ => {
                            ch = ' ' as i32 as byte;
                            current_block_51 = 10930818133215224067;
                        }
                    }
                    match current_block_51 {
                        17404231275461091374 => {
                            op = moat(y, x);
                            if !op.is_null() {
                                if (*op)._t._t_oldch as libc::c_int == ' ' as i32 {
                                    (*op)._t._t_oldch = ch;
                                }
                            }
                        }
                        _ => {}
                    }
                    if ch as libc::c_int == 0xce as libc::c_int {
                        cur_move(y, x);
                        if cur_inch() as libc::c_int != 0xce as libc::c_int {
                            set_attr(14 as libc::c_int);
                        }
                    }
                    if ch as libc::c_int != ' ' as i32 {
                        cur_mvaddch(y, x, ch);
                    }
                    set_attr(0 as libc::c_int);
                    x += 1;
                }
                y += 1;
            }
        }
        7 => {
            ch = 0 as libc::c_int as byte;
            op = lvl_obj;
            while !op.is_null() {
                if (*op)._o._o_type == 0x5 as libc::c_int {
                    ch = 1 as libc::c_int as byte;
                    set_attr(14 as libc::c_int);
                    cur_mvaddch(
                        (*op)._o._o_pos.y,
                        (*op)._o._o_pos.x,
                        0x5 as libc::c_int as byte,
                    );
                    set_attr(0 as libc::c_int);
                } else if (*op)._o._o_type == 0xc as libc::c_int {
                    ch = 1 as libc::c_int as byte;
                    set_attr(14 as libc::c_int);
                    cur_mvaddch(
                        (*op)._o._o_pos.y,
                        (*op)._o._o_pos.x,
                        0xc as libc::c_int as byte,
                    );
                    set_attr(0 as libc::c_int);
                }
                op = (*op)._t._l_next;
            }
            if ch != 0 {
                *s_know
                    .as_mut_ptr()
                    .offset(7 as libc::c_int as isize) = 1 as libc::c_int != 0;
                msg(
                    b"your nose tingles as you sense food\0" as *const u8
                        as *const libc::c_char,
                );
            } else {
                ifterse(
                    b"you hear a growling noise close by\0" as *const u8
                        as *const libc::c_char,
                    b"you hear a growling noise very close to you\0" as *const u8
                        as *const libc::c_char,
                );
            }
        }
        8 => {
            let mut cur_room: *mut room = 0 as *mut room;
            cur_room = player._t._t_room;
            teleport();
            if cur_room != player._t._t_room {
                *s_know
                    .as_mut_ptr()
                    .offset(8 as libc::c_int as isize) = 1 as libc::c_int != 0;
            }
        }
        9 => {
            if cur_weapon.is_null() || (*cur_weapon)._o._o_type != 0x18 as libc::c_int {
                msg(
                    b"you feel a strange sense of loss\0" as *const u8
                        as *const libc::c_char,
                );
            } else {
                (*cur_weapon)
                    ._o
                    ._o_flags = ((*cur_weapon)._o._o_flags as libc::c_int
                    & !(0x1 as libc::c_int)) as libc::c_short;
                if rnd(2 as libc::c_int) == 0 as libc::c_int {
                    (*cur_weapon)._o._o_hplus += 1;
                } else {
                    (*cur_weapon)._o._o_dplus += 1;
                }
                ifterse(
                    b"your %s glows blue\0" as *const u8 as *const libc::c_char,
                    b"your %s glows blue for a moment\0" as *const u8
                        as *const libc::c_char,
                    *w_names.as_mut_ptr().offset((*cur_weapon)._o._o_which as isize),
                );
            }
        }
        6 => {
            msg(
                laugh,
                if terse as libc::c_int != 0 || expert as libc::c_int != 0 {
                    b"\0" as *const u8 as *const libc::c_char
                } else {
                    in_dist as *const libc::c_char
                },
            );
        }
        11 => {
            if !cur_armor.is_null() {
                (*cur_armor)
                    ._o
                    ._o_flags = ((*cur_armor)._o._o_flags as libc::c_int
                    & !(0x1 as libc::c_int)) as libc::c_short;
            }
            if !cur_weapon.is_null() {
                (*cur_weapon)
                    ._o
                    ._o_flags = ((*cur_weapon)._o._o_flags as libc::c_int
                    & !(0x1 as libc::c_int)) as libc::c_short;
            }
            if !(*cur_ring.as_mut_ptr().offset(0 as libc::c_int as isize)).is_null() {
                let ref mut fresh2 = (**cur_ring
                    .as_mut_ptr()
                    .offset(0 as libc::c_int as isize))
                    ._o
                    ._o_flags;
                *fresh2 = (*fresh2 as libc::c_int & !(0x1 as libc::c_int))
                    as libc::c_short;
            }
            if !(*cur_ring.as_mut_ptr().offset(1 as libc::c_int as isize)).is_null() {
                let ref mut fresh3 = (**cur_ring
                    .as_mut_ptr()
                    .offset(1 as libc::c_int as isize))
                    ._o
                    ._o_flags;
                *fresh3 = (*fresh3 as libc::c_int & !(0x1 as libc::c_int))
                    as libc::c_short;
            }
            ifterse(
                b"somebody is watching over you\0" as *const u8 as *const libc::c_char,
                b"you feel as if somebody is watching over you\0" as *const u8
                    as *const libc::c_char,
            );
        }
        12 => {
            aggravate();
            ifterse(
                b"you hear a humming noise\0" as *const u8 as *const libc::c_char,
                b"you hear a high pitched humming noise\0" as *const u8
                    as *const libc::c_char,
            );
        }
        13 => {
            msg(b"this scroll seems to be blank\0" as *const u8 as *const libc::c_char);
        }
        14 => {
            if cur_weapon.is_null() || (*cur_weapon)._o._o_type != 0x18 as libc::c_int {
                msg(
                    laugh,
                    if terse as libc::c_int != 0 || expert as libc::c_int != 0 {
                        b"\0" as *const u8 as *const libc::c_char
                    } else {
                        in_dist as *const libc::c_char
                    },
                );
            } else if (*cur_weapon)._o._o_enemy as libc::c_int != 0 as libc::c_int {
                msg(
                    b"your %s vanishes in a puff of smoke\0" as *const u8
                        as *const libc::c_char,
                    *w_names.as_mut_ptr().offset((*cur_weapon)._o._o_which as isize),
                );
                list_detach(&mut player._t._t_pack, cur_weapon);
                discard(cur_weapon);
                cur_weapon = 0 as *mut THING;
            } else {
                (*cur_weapon)._o._o_enemy = pick_mons();
                (*cur_weapon)._o._o_hplus += 1;
                (*cur_weapon)._o._o_dplus += 1;
                (*cur_weapon)._o._o_ac = 1 as libc::c_int as libc::c_short;
                msg(
                    flashmsg,
                    *w_names.as_mut_ptr().offset((*cur_weapon)._o._o_which as isize),
                    if terse as libc::c_int != 0 || expert as libc::c_int != 0 {
                        b"\0" as *const u8 as *const libc::c_char
                    } else {
                        intense as *const libc::c_char
                    },
                );
            }
        }
        _ => {
            msg(b"what a puzzling scroll!\0" as *const u8 as *const libc::c_char);
            return;
        }
    }
    look(1 as libc::c_int != 0);
    status();
    inpack -= 1;
    if (*obj)._o._o_count > 1 as libc::c_int {
        (*obj)._o._o_count -= 1;
    } else {
        list_detach(&mut player._t._t_pack, obj);
        discardit = 1 as libc::c_int != 0;
    }
    call_it(
        *s_know.as_mut_ptr().offset((*obj)._o._o_which as isize),
        &mut *s_guess.as_mut_ptr().offset((*obj)._o._o_which as isize),
    );
    if discardit {
        discard(obj);
    }
}
