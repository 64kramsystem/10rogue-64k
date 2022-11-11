use ::libc;
extern "C" {
    fn memmove(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
        -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    fn md_srand() -> libc::c_int;
    fn one_tick();
    fn csum() -> libc::c_int;
    fn _halt();
    static mut saw_amulet: bool;
    static mut level: libc::c_int;
    static mut max_level: libc::c_int;
    static mut mpos: libc::c_int;
    static mut no_food: libc::c_int;
    static mut ntraps: libc::c_int;
    static mut total: libc::c_int;
    static mut seed: libc::c_long;
    static mut cksum: libc::c_int;
    static mut lvl_obj: *mut THING;
    static mut mlist: *mut THING;
    static mut player: THING;
    static mut oldpos: coord;
    static mut oldrp: *mut room;
    static mut rooms: [room; 0];
    static mut _level: *mut byte;
    static mut _flags: *mut byte;
    fn status();
    fn new_item() -> *mut THING;
    fn list_attach(list: *mut *mut THING, item: *mut THING);
    fn list_free(ptr: *mut *mut THING);
    fn rnd(range: libc::c_int) -> libc::c_int;
    fn winat(y: libc::c_int, x: libc::c_int) -> byte;
    fn INDEX(y: libc::c_int, x: libc::c_int) -> libc::c_int;
    fn randmonster(wander: bool) -> libc::c_char;
    fn new_monster(tp: *mut THING, type_0: byte, cp: *mut coord);
    fn f_restor();
    fn give_pack(tp: *mut THING);
    fn moat(my: libc::c_int, mx: libc::c_int) -> *mut THING;
    fn turn_see(turn_off: bool) -> bool;
    fn enter_room(cp: *mut coord);
    fn rnd_pos(rp: *mut room, cp: *mut coord);
    fn new_thing() -> *mut THING;
    fn do_passages();
    fn do_rooms();
    fn cur_mvaddch(r: libc::c_int, c: libc::c_int, chr: byte);
    fn implode();
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
pub unsafe extern "C" fn new_level() {
    let mut rm: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut tp: *mut THING = 0 as *mut THING;
    let mut fp: *mut byte = 0 as *mut byte;
    let mut index: libc::c_int = 0;
    let mut stairs: coord = coord { x: 0, y: 0 };
    player._t._t_flags =
        (player._t._t_flags as libc::c_int & !(0x80 as libc::c_int)) as libc::c_short;
    if level > max_level {
        max_level = level;
    }
    one_tick();
    if level > 1 as libc::c_int && csum() != cksum {
        _halt();
    }
    memset(
        _level as *mut libc::c_void,
        ' ' as i32,
        ((25 as libc::c_int - 3 as libc::c_int) * 80 as libc::c_int) as libc::c_ulong,
    );
    memset(
        _flags as *mut libc::c_void,
        0x10 as libc::c_int,
        ((25 as libc::c_int - 3 as libc::c_int) * 80 as libc::c_int) as libc::c_ulong,
    );
    tp = mlist;
    while !tp.is_null() {
        list_free(&mut (*tp)._t._t_pack);
        tp = (*tp)._t._l_next;
    }
    list_free(&mut mlist);
    f_restor();
    list_free(&mut lvl_obj);
    do_rooms();
    if max_level > 1 as libc::c_int {
        implode();
    }
    status();
    do_passages();
    no_food += 1;
    put_things();
    i = 0 as libc::c_int;
    loop {
        rm = rnd_room();
        rnd_pos(&mut *rooms.as_mut_ptr().offset(rm as isize), &mut stairs);
        index = INDEX(stairs.y, stairs.x);
        let fresh0 = i;
        i = i + 1;
        if fresh0 > 100 as libc::c_int {
            i = 0 as libc::c_int;
            seed = md_srand() as libc::c_long;
        }
        if *_level.offset(index as isize) as libc::c_int == 0xfa as libc::c_int
            || *_level.offset(index as isize) as libc::c_int == 0xb1 as libc::c_int
        {
            break;
        }
    }
    *_level.offset(index as isize) = 0xf0 as libc::c_int as byte;
    if rnd(10 as libc::c_int) < level {
        ntraps = rnd(level / 4 as libc::c_int) + 1 as libc::c_int;
        if ntraps > 10 as libc::c_int {
            ntraps = 10 as libc::c_int;
        }
        i = ntraps;
        loop {
            let fresh1 = i;
            i = i - 1;
            if !(fresh1 != 0) {
                break;
            }
            loop {
                rm = rnd_room();
                rnd_pos(&mut *rooms.as_mut_ptr().offset(rm as isize), &mut stairs);
                index = INDEX(stairs.y, stairs.x);
                if *_level.offset(index as isize) as libc::c_int == 0xfa as libc::c_int
                    || *_level.offset(index as isize) as libc::c_int == 0xb1 as libc::c_int
                {
                    break;
                }
            }
            fp = &mut *_flags.offset(index as isize) as *mut byte;
            *fp = (*fp as libc::c_int & !(0x10 as libc::c_int)) as byte;
            *fp = (*fp as libc::c_int | rnd(6 as libc::c_int)) as byte;
        }
    }
    loop {
        rm = rnd_room();
        rnd_pos(
            &mut *rooms.as_mut_ptr().offset(rm as isize),
            &mut player._t._t_pos,
        );
        index = INDEX(player._t._t_pos.y, player._t._t_pos.x);
        if (*_level.offset(index as isize) as libc::c_int == 0xfa as libc::c_int
            || *_level.offset(index as isize) as libc::c_int == 0xb1 as libc::c_int)
            && *_flags.offset(index as isize) as libc::c_int & 0x10 as libc::c_int != 0
            && (moat(player._t._t_pos.y, player._t._t_pos.x)).is_null()
        {
            break;
        }
    }
    mpos = 0 as libc::c_int;
    enter_room(&mut player._t._t_pos);
    cur_mvaddch(
        player._t._t_pos.y,
        player._t._t_pos.x,
        0x1 as libc::c_int as byte,
    );
    memmove(
        &mut oldpos as *mut coord as *mut libc::c_void,
        &mut player._t._t_pos as *mut coord as *const libc::c_void,
        ::core::mem::size_of::<coord>() as libc::c_ulong,
    );
    oldrp = player._t._t_room;
    if player._t._t_flags as libc::c_int & 0x2 as libc::c_int != 0 as libc::c_int {
        turn_see(0 as libc::c_int != 0);
    }
}
#[no_mangle]
pub unsafe extern "C" fn rnd_room() -> libc::c_int {
    let mut rm: libc::c_int = 0;
    loop {
        rm = rnd(9 as libc::c_int);
        if (*rooms.as_mut_ptr().offset(rm as isize)).r_flags as libc::c_int & 0x2 as libc::c_int
            == 0 as libc::c_int
            || (*rooms.as_mut_ptr().offset(rm as isize)).r_flags as libc::c_int & 0x4 as libc::c_int
                != 0
        {
            break;
        }
    }
    return rm;
}
#[no_mangle]
pub unsafe extern "C" fn put_things() {
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut cur: *mut THING = 0 as *mut THING;
    let mut rm: libc::c_int = 0;
    let mut tp: coord = coord { x: 0, y: 0 };
    if saw_amulet as libc::c_int != 0 && level < max_level {
        i = 9 as libc::c_int - 1 as libc::c_int;
    } else {
        if level >= 26 as libc::c_int && !saw_amulet {
            cur = new_item();
            if !cur.is_null() {
                list_attach(&mut lvl_obj, cur);
                (*cur)._o._o_dplus = 0 as libc::c_int;
                (*cur)._o._o_hplus = (*cur)._o._o_dplus;
                (*cur)._o._o_hurldmg =
                    b"0d0\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
                (*cur)._o._o_damage = (*cur)._o._o_hurldmg;
                (*cur)._o._o_ac = 11 as libc::c_int as libc::c_short;
                (*cur)._o._o_type = 0xc as libc::c_int;
                loop {
                    rm = rnd_room();
                    rnd_pos(&mut *rooms.as_mut_ptr().offset(rm as isize), &mut tp);
                    if winat(tp.y, tp.x) as libc::c_int == 0xfa as libc::c_int
                        || winat(tp.y, tp.x) as libc::c_int == 0xb1 as libc::c_int
                    {
                        break;
                    }
                }
                *_level.offset(INDEX(tp.y, tp.x) as isize) = 0xc as libc::c_int as byte;
                memmove(
                    &mut (*cur)._o._o_pos as *mut coord as *mut libc::c_void,
                    &mut tp as *mut coord as *const libc::c_void,
                    ::core::mem::size_of::<coord>() as libc::c_ulong,
                );
            }
        }
        if rnd(20 as libc::c_int) == 0 as libc::c_int {
            treas_room();
        }
    }
    while i < 9 as libc::c_int {
        if total < 83 as libc::c_int && rnd(100 as libc::c_int) < 35 as libc::c_int {
            cur = new_thing();
            list_attach(&mut lvl_obj, cur);
            loop {
                rm = rnd_room();
                rnd_pos(&mut *rooms.as_mut_ptr().offset(rm as isize), &mut tp);
                if *_level.offset(INDEX(tp.y, tp.x) as isize) as libc::c_int == 0xfa as libc::c_int
                    || *_level.offset(INDEX(tp.y, tp.x) as isize) as libc::c_int
                        == 0xb1 as libc::c_int
                {
                    break;
                }
            }
            *_level.offset(INDEX(tp.y, tp.x) as isize) = (*cur)._o._o_type as byte;
            memmove(
                &mut (*cur)._o._o_pos as *mut coord as *mut libc::c_void,
                &mut tp as *mut coord as *const libc::c_void,
                ::core::mem::size_of::<coord>() as libc::c_ulong,
            );
        }
        i += 1;
    }
}
unsafe extern "C" fn treas_room() {
    let mut nm: libc::c_int = 0;
    let mut index: libc::c_int = 0;
    let mut tp: *mut THING = 0 as *mut THING;
    let mut rp: *mut room = 0 as *mut room;
    let mut spots: libc::c_int = 0;
    let mut num_monst: libc::c_int = 0;
    let mut mp: coord = coord { x: 0, y: 0 };
    rp = &mut *rooms
        .as_mut_ptr()
        .offset((rnd_room as unsafe extern "C" fn() -> libc::c_int)() as isize)
        as *mut room;
    spots =
        ((*rp).r_max.y - 2 as libc::c_int) * ((*rp).r_max.x - 2 as libc::c_int) - 2 as libc::c_int;
    if spots > 10 as libc::c_int - 2 as libc::c_int {
        spots = 10 as libc::c_int - 2 as libc::c_int;
    }
    nm = rnd(spots) + 2 as libc::c_int;
    num_monst = nm;
    loop {
        let fresh2 = nm;
        nm = nm - 1;
        if !(fresh2 != 0 && total < 83 as libc::c_int) {
            break;
        }
        loop {
            rnd_pos(rp, &mut mp);
            index = INDEX(mp.y, mp.x);
            if *_level.offset(index as isize) as libc::c_int == 0xfa as libc::c_int
                || *_level.offset(index as isize) as libc::c_int == 0xb1 as libc::c_int
            {
                break;
            }
        }
        tp = new_thing();
        memmove(
            &mut (*tp)._o._o_pos as *mut coord as *mut libc::c_void,
            &mut mp as *mut coord as *const libc::c_void,
            ::core::mem::size_of::<coord>() as libc::c_ulong,
        );
        list_attach(&mut lvl_obj, tp);
        *_level.offset(index as isize) = (*tp)._o._o_type as byte;
    }
    nm = rnd(spots) + 2 as libc::c_int;
    if nm < num_monst + 2 as libc::c_int {
        nm = num_monst + 2 as libc::c_int;
    }
    spots = ((*rp).r_max.y - 2 as libc::c_int) * ((*rp).r_max.x - 2 as libc::c_int);
    if nm > spots {
        nm = spots;
    }
    level += 1;
    loop {
        let fresh3 = nm;
        nm = nm - 1;
        if !(fresh3 != 0) {
            break;
        }
        spots = 0 as libc::c_int;
        while spots < 10 as libc::c_int {
            rnd_pos(rp, &mut mp);
            index = INDEX(mp.y, mp.x);
            if (*_level.offset(index as isize) as libc::c_int == 0xfa as libc::c_int
                || *_level.offset(index as isize) as libc::c_int == 0xb1 as libc::c_int)
                && (moat(mp.y, mp.x)).is_null()
            {
                break;
            }
            spots += 1;
        }
        if spots != 10 as libc::c_int {
            tp = new_item();
            if !tp.is_null() {
                new_monster(tp, randmonster(0 as libc::c_int != 0) as byte, &mut mp);
                (*tp)._t._t_flags =
                    ((*tp)._t._t_flags as libc::c_int | 0x20 as libc::c_int) as libc::c_short;
                give_pack(tp);
            }
        }
    }
    level -= 1;
}
