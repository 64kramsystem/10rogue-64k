use ::libc;
extern "C" {
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn readchar() -> byte;
    static mut amulet: bool;
    static mut after: bool;
    static mut again: bool;
    static mut expert: bool;
    static mut saw_amulet: bool;
    static mut terse: bool;
    static mut inpack: libc::c_int;
    static mut mpos: libc::c_int;
    static mut purse: libc::c_int;
    static mut lvl_obj: *mut THING;
    static mut mlist: *mut THING;
    static mut player: THING;
    static mut s_menu: [libc::c_char; 0];
    static mut _level: *mut byte;
    fn ifterse(tfmt: *const libc::c_char, fmt: *const libc::c_char, _: ...);
    fn msg(fmt: *const libc::c_char, _: ...);
    fn addmsg(fmt: *const libc::c_char, _: ...);
    fn noterse(str: *mut libc::c_char) -> *mut libc::c_char;
    fn list_detach(list: *mut *mut THING, item: *mut THING);
    fn discard(item: *mut THING) -> libc::c_int;
    fn find_obj(y: libc::c_int, x: libc::c_int) -> *mut THING;
    fn INDEX(y: libc::c_int, x: libc::c_int) -> libc::c_int;
    fn end_line(use_0: *mut libc::c_char) -> byte;
    fn inv_name(obj: *mut THING, drop_0: bool) -> *mut libc::c_char;
    fn add_line(
        use_0: *mut libc::c_char,
        fmt: *mut libc::c_char,
        arg: *mut libc::c_char,
    ) -> byte;
    fn cur_mvaddch(r: libc::c_int, c: libc::c_int, chr: byte);
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
unsafe extern "C" fn pack_obj(mut ch: byte, mut chp: *mut byte) -> *mut THING {
    let mut obj: *mut THING = 0 as *mut THING;
    let mut och: byte = 0;
    obj = player._t._t_pack;
    och = 'a' as i32 as byte;
    while !obj.is_null() {
        if ch as libc::c_int == och as libc::c_int {
            return obj;
        }
        obj = (*obj)._t._l_next;
        och = och.wrapping_add(1);
    }
    *chp = och;
    return 0 as *mut THING;
}
#[no_mangle]
pub unsafe extern "C" fn add_pack(mut obj: *mut THING, mut silent: bool) {
    let mut current_block: u64;
    let mut op: *mut THING = 0 as *mut THING;
    let mut lp: *mut THING = 0 as *mut THING;
    let mut exact: bool = false;
    let mut from_floor: bool = false;
    let mut floor: byte = 0;
    if obj.is_null() {
        from_floor = 1 as libc::c_int != 0;
        obj = find_obj(player._t._t_pos.y, player._t._t_pos.x);
        if obj.is_null() {
            return;
        }
    } else {
        from_floor = 0 as libc::c_int != 0;
    }
    floor = (if !(player._t._t_room).is_null()
        && (*player._t._t_room).r_flags as libc::c_int & 0x2 as libc::c_int != 0
    {
        0xb1 as libc::c_int
    } else {
        0xfa as libc::c_int
    }) as byte;
    if (*obj)._o._o_group != 0 {
        op = player._t._t_pack;
        loop {
            if op.is_null() {
                current_block = 15976848397966268834;
                break;
            }
            if (*op)._o._o_group == (*obj)._o._o_group {
                (*op)._o._o_count += (*obj)._o._o_count;
                if from_floor {
                    list_detach(&mut lvl_obj, obj);
                    cur_mvaddch(player._t._t_pos.y, player._t._t_pos.x, floor);
                    *_level
                        .offset(
                            INDEX(player._t._t_pos.y, player._t._t_pos.x) as isize,
                        ) = floor;
                }
                discard(obj);
                obj = op;
                current_block = 17702919769861191155;
                break;
            } else {
                op = (*op)._t._l_next;
            }
        }
    } else {
        current_block = 15976848397966268834;
    }
    match current_block {
        15976848397966268834 => {
            if inpack >= 23 as libc::c_int - 1 as libc::c_int {
                msg(
                    b"you can't carry anything else\0" as *const u8
                        as *const libc::c_char,
                );
                return;
            }
            if (*obj)._o._o_type == 0xd as libc::c_int
                && (*obj)._o._o_which == 6 as libc::c_int
            {
                if (*obj)._o._o_flags as libc::c_int & 0x8 as libc::c_int != 0 {
                    list_detach(&mut lvl_obj, obj);
                    cur_mvaddch(player._t._t_pos.y, player._t._t_pos.x, floor);
                    *_level
                        .offset(
                            INDEX(player._t._t_pos.y, player._t._t_pos.x) as isize,
                        ) = floor;
                    msg(
                        b"the scroll turns to dust%s.\0" as *const u8
                            as *const libc::c_char,
                        noterse(
                            b" as you pick it up\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                        ),
                    );
                    return;
                } else {
                    (*obj)
                        ._o
                        ._o_flags = ((*obj)._o._o_flags as libc::c_int
                        | 0x8 as libc::c_int) as libc::c_short;
                }
            }
            inpack += 1;
            if from_floor {
                list_detach(&mut lvl_obj, obj);
                cur_mvaddch(player._t._t_pos.y, player._t._t_pos.x, floor);
                *_level
                    .offset(
                        INDEX(player._t._t_pos.y, player._t._t_pos.x) as isize,
                    ) = floor;
            }
            exact = 0 as libc::c_int != 0;
            op = player._t._t_pack;
            while !op.is_null() {
                if (*obj)._o._o_type == (*op)._o._o_type {
                    break;
                }
                op = (*op)._t._l_next;
            }
            if op.is_null() {
                op = player._t._t_pack;
                while !op.is_null() {
                    if (*op)._o._o_type != 0x5 as libc::c_int {
                        break;
                    }
                    lp = op;
                    op = (*op)._t._l_next;
                }
            } else {
                while (*op)._o._o_type == (*obj)._o._o_type {
                    if (*op)._o._o_which == (*obj)._o._o_which {
                        exact = 1 as libc::c_int != 0;
                        break;
                    } else {
                        lp = op;
                        op = (*op)._t._l_next;
                        if op.is_null() {
                            break;
                        }
                    }
                }
            }
            if op.is_null() {
                if (player._t._t_pack).is_null() {
                    player._t._t_pack = obj;
                } else {
                    (*lp)._t._l_next = obj;
                    (*obj)._t._l_prev = lp;
                    (*obj)._t._l_next = 0 as *mut thing;
                }
            } else if exact as libc::c_int != 0
                && ((*obj)._o._o_type == 0xad as libc::c_int
                    || (*obj)._o._o_type == 0xd as libc::c_int
                    || (*obj)._o._o_type == 0x5 as libc::c_int
                    || (*obj)._o._o_type == 0xf as libc::c_int)
            {
                (*op)._o._o_count += 1;
                discard(obj);
                obj = op;
            } else {
                (*obj)._t._l_prev = (*op)._t._l_prev;
                if !((*obj)._t._l_prev).is_null() {
                    (*(*obj)._t._l_prev)._t._l_next = obj;
                } else {
                    player._t._t_pack = obj;
                }
                (*obj)._t._l_next = op;
                (*op)._t._l_prev = obj;
            }
        }
        _ => {}
    }
    op = mlist;
    while !op.is_null() {
        if !((*op)._t._t_dest).is_null() && (*(*op)._t._t_dest).x == (*obj)._o._o_pos.x
            && (*(*op)._t._t_dest).y == (*obj)._o._o_pos.y
        {
            (*op)._t._t_dest = &mut player._t._t_pos;
        }
        op = (*op)._t._l_next;
    }
    if (*obj)._o._o_type == 0xc as libc::c_int {
        amulet = 1 as libc::c_int != 0;
        saw_amulet = 1 as libc::c_int != 0;
    }
    if !silent {
        msg(
            b"%s%s (%c)\0" as *const u8 as *const libc::c_char,
            noterse(
                b"you now have \0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            ),
            inv_name(obj, 1 as libc::c_int != 0),
            pack_char(obj) as libc::c_int,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn inventory(
    mut list: *mut THING,
    mut type_0: libc::c_int,
    mut lstr: *mut libc::c_char,
) -> byte {
    let mut ch: byte = 0;
    let mut n_objs: libc::c_int = 0;
    let mut inv_temp: [libc::c_char; 80] = [0; 80];
    n_objs = 0 as libc::c_int;
    ch = 'a' as i32 as byte;
    while !list.is_null() {
        if !(type_0 != 0 && type_0 != (*list)._o._o_type
            && !(type_0 == -(1 as libc::c_int)
                && ((*list)._o._o_type == 0xd as libc::c_int
                    || (*list)._o._o_type == 0xad as libc::c_int
                    || (*list)._o._o_type == 0x9 as libc::c_int
                    || (*list)._o._o_type == 0xe7 as libc::c_int))
            && !(type_0 == 0x18 as libc::c_int
                && (*list)._o._o_type == 0xad as libc::c_int)
            && !(type_0 == 0xe7 as libc::c_int && (*list)._o._o_enemy as libc::c_int != 0
                && (*list)._o._o_ac as libc::c_int != 0))
        {
            n_objs += 1;
            sprintf(
                inv_temp.as_mut_ptr(),
                b"%c) %%s\0" as *const u8 as *const libc::c_char,
                ch as libc::c_int,
            );
            add_line(lstr, inv_temp.as_mut_ptr(), inv_name(list, 0 as libc::c_int != 0));
        }
        ch = ch.wrapping_add(1);
        list = (*list)._t._l_next;
    }
    if n_objs == 0 as libc::c_int {
        msg(
            if type_0 == 0 as libc::c_int {
                b"you are empty handed\0" as *const u8 as *const libc::c_char
            } else {
                b"you don't have anything appropriate\0" as *const u8
                    as *const libc::c_char
            },
        );
        return 0 as libc::c_int as byte;
    }
    return end_line(lstr);
}
#[no_mangle]
pub unsafe extern "C" fn pick_up(mut ch: byte) {
    let mut obj: *mut THING = 0 as *mut THING;
    match ch as libc::c_int {
        15 => {
            obj = find_obj(player._t._t_pos.y, player._t._t_pos.x);
            if obj.is_null() {
                return;
            }
            money((*obj)._o._o_ac as libc::c_int);
            list_detach(&mut lvl_obj, obj);
            discard(obj);
            (*player._t._t_room).r_goldval = 0 as libc::c_int;
        }
        8 | 173 | 5 | 24 | 13 | 12 | 9 | 231 | _ => {
            add_pack(0 as *mut THING, 0 as libc::c_int != 0);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn get_item(
    mut purpose: *mut libc::c_char,
    mut type_0: libc::c_int,
) -> *mut THING {
    let mut obj: *mut THING = 0 as *mut THING;
    let mut ch: byte = 0;
    let mut och: byte = 0;
    static mut lch: byte = 0;
    static mut wasthing: *mut THING = 0 as *const THING as *mut THING;
    let mut gi_state: byte = 0;
    let mut once_only: libc::c_int = 0 as libc::c_int;
    if strncmp(
        s_menu.as_mut_ptr(),
        b"sel\0" as *const u8 as *const libc::c_char,
        3 as libc::c_int as libc::c_ulong,
    ) == 0 && strcmp(purpose, b"eat\0" as *const u8 as *const libc::c_char) != 0
        && strcmp(purpose, b"drop\0" as *const u8 as *const libc::c_char) != 0
        || strcmp(s_menu.as_mut_ptr(), b"on\0" as *const u8 as *const libc::c_char) == 0
    {
        once_only = 1 as libc::c_int;
    }
    gi_state = again as byte;
    if (player._t._t_pack).is_null() {
        msg(b"you aren't carrying anything\0" as *const u8 as *const libc::c_char);
        return 0 as *mut THING;
    } else {
        ch = lch;
        loop {
            if !(gi_state as libc::c_int != 0 && wasthing == pack_obj(ch, &mut och)) {
                if once_only != 0 {
                    ch = '*' as i32 as byte;
                } else {
                    if !terse && !expert {
                        addmsg(
                            b"which object do you want to \0" as *const u8
                                as *const libc::c_char,
                        );
                    }
                    msg(
                        b"%s? (* for list): \0" as *const u8 as *const libc::c_char,
                        purpose,
                    );
                    ch = readchar();
                }
            }
            mpos = 0 as libc::c_int;
            gi_state = 0 as libc::c_int as byte;
            once_only = 0 as libc::c_int;
            if ch as libc::c_int == '*' as i32 {
                ch = inventory(player._t._t_pack, type_0, purpose);
                if ch as libc::c_int == 0 as libc::c_int {
                    after = 0 as libc::c_int != 0;
                    return 0 as *mut THING;
                }
                if ch as libc::c_int == ' ' as i32 {
                    continue;
                }
                lch = ch;
            }
            if ch as libc::c_int == 27 as libc::c_int {
                after = 0 as libc::c_int != 0;
                msg(b"\0" as *const u8 as *const libc::c_char);
                return 0 as *mut THING;
            }
            obj = pack_obj(ch, &mut och);
            if obj.is_null() {
                ifterse(
                    b"range is 'a' to '%c'\0" as *const u8 as *const libc::c_char,
                    b"please specify a letter between 'a' and '%c'\0" as *const u8
                        as *const libc::c_char,
                    och as libc::c_int - 1 as libc::c_int,
                );
            } else {
                if strcmp(purpose, b"identify\0" as *const u8 as *const libc::c_char)
                    != 0
                {
                    lch = ch;
                    wasthing = obj;
                }
                return obj;
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn pack_char(mut obj: *mut THING) -> byte {
    let mut item: *mut THING = 0 as *mut THING;
    let mut c: byte = 0;
    c = 'a' as i32 as byte;
    item = player._t._t_pack;
    while !item.is_null() {
        if item == obj {
            return c
        } else {
            c = c.wrapping_add(1);
        }
        item = (*item)._t._l_next;
    }
    return '?' as i32 as byte;
}
#[no_mangle]
pub unsafe extern "C" fn money(mut value: libc::c_int) {
    let mut floor: byte = 0;
    floor = (if (*player._t._t_room).r_flags as libc::c_int & 0x2 as libc::c_int != 0 {
        0xb1 as libc::c_int
    } else {
        0xfa as libc::c_int
    }) as byte;
    purse += value;
    cur_mvaddch(player._t._t_pos.y, player._t._t_pos.x, floor);
    *_level.offset(INDEX(player._t._t_pos.y, player._t._t_pos.x) as isize) = floor;
    if value > 0 as libc::c_int {
        msg(b"you found %d gold pieces\0" as *const u8 as *const libc::c_char, value);
    }
}
