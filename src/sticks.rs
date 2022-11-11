use ::libc;
extern "C" {
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    static mut after: bool;
    static mut ws_know: [bool; 0];
    static mut w_names: [*mut libc::c_char; 0];
    static mut ws_type: [*mut libc::c_char; 0];
    static mut no_command: libc::c_int;
    static mut cur_weapon: *mut THING;
    static mut mlist: *mut THING;
    static mut player: THING;
    static mut delta: coord;
    static mut passages: [room; 0];
    static mut rooms: [room; 0];
    static mut monsters: [monster; 0];
    static mut _level: *mut byte;
    static mut _flags: *mut byte;
    fn start_run(runner: *mut coord);
    fn see_monst(mp: *mut THING) -> bool;
    fn fight(mp: *mut coord, mn: libc::c_char, weap: *mut THING, thrown: bool) -> bool;
    fn save_throw(which: libc::c_int, tp: *mut THING) -> bool;
    fn save(which: libc::c_int) -> bool;
    fn killed(tp: *mut THING, pr: bool);
    fn msg(fmt: *const libc::c_char, _: ...);
    fn noterse(str: *mut libc::c_char) -> *mut libc::c_char;
    fn list_detach(list: *mut *mut THING, item: *mut THING);
    fn rnd(range: libc::c_int) -> libc::c_int;
    fn roll(number: libc::c_int, sides: libc::c_int) -> libc::c_int;
    fn step_ok(ch: byte) -> bool;
    fn _ce(a: *mut coord, b: *mut coord) -> bool;
    fn winat(y: libc::c_int, x: libc::c_int) -> byte;
    fn spread(nm: libc::c_int) -> libc::c_int;
    fn INDEX(y: libc::c_int, x: libc::c_int) -> libc::c_int;
    fn new_monster(tp: *mut THING, type_0: byte, cp: *mut coord);
    fn moat(my: libc::c_int, mx: libc::c_int) -> *mut THING;
    fn rnd_room() -> libc::c_int;
    fn get_item(purpose: *mut libc::c_char, type_0: libc::c_int) -> *mut THING;
    fn death(monst: libc::c_char);
    fn rnd_pos(rp: *mut room, cp: *mut coord);
    fn enter_room(cp: *mut coord);
    fn tick_pause();
    fn hit_monster(y: libc::c_int, x: libc::c_int, obj: *mut THING) -> bool;
    fn do_motion(obj: *mut THING, ydelta: libc::c_int, xdelta: libc::c_int);
    fn cur_mvaddch(r: libc::c_int, c: libc::c_int, chr: byte);
    fn cur_mvinch(r: libc::c_int, c: libc::c_int) -> byte;
    fn set_attr(bute: libc::c_int);
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct monster {
    pub m_name: *mut libc::c_char,
    pub m_carry: libc::c_int,
    pub m_flags: libc::c_ushort,
    pub m_stats: stats,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub s_pos: coord,
    pub s_under: byte,
}
#[no_mangle]
pub unsafe extern "C" fn fix_stick(mut cur: *mut THING) {
    if strcmp(
        *ws_type.as_mut_ptr().offset((*cur)._o._o_which as isize),
        b"staff\0" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int
    {
        (*cur)
            ._o
            ._o_damage = b"2d3\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char;
    } else {
        (*cur)
            ._o
            ._o_damage = b"1d1\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char;
    }
    (*cur)
        ._o
        ._o_hurldmg = b"1d1\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    (*cur)._o._o_ac = (3 as libc::c_int + rnd(5 as libc::c_int)) as libc::c_short;
    match (*cur)._o._o_which {
        1 => {
            (*cur)._o._o_hplus = 100 as libc::c_int;
            (*cur)._o._o_dplus = 3 as libc::c_int;
            (*cur)
                ._o
                ._o_damage = b"1d8\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
        }
        0 => {
            (*cur)
                ._o
                ._o_ac = (10 as libc::c_int + rnd(10 as libc::c_int)) as libc::c_short;
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn do_zap() {
    let mut obj: *mut THING = 0 as *mut THING;
    let mut tp: *mut THING = 0 as *mut THING;
    let mut y: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut which_one: libc::c_int = 0;
    obj = get_item(
        b"zap with\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0xe7 as libc::c_int,
    );
    if obj.is_null() {
        return;
    }
    which_one = (*obj)._o._o_which;
    if (*obj)._o._o_type != 0xe7 as libc::c_int {
        if (*obj)._o._o_enemy as libc::c_int != 0 && (*obj)._o._o_ac as libc::c_int != 0
        {
            which_one = 14 as libc::c_int;
        } else {
            msg(b"you can't zap with that!\0" as *const u8 as *const libc::c_char);
            after = 0 as libc::c_int != 0;
            return;
        }
    }
    if (*obj)._o._o_ac as libc::c_int == 0 as libc::c_int {
        msg(b"nothing happens\0" as *const u8 as *const libc::c_char);
        return;
    }
    match which_one {
        0 => {
            if player._t._t_flags as libc::c_int & 0x1 as libc::c_int != 0 as libc::c_int
            {
                msg(
                    b"you feel a warm glow around you\0" as *const u8
                        as *const libc::c_char,
                );
            } else {
                *ws_know
                    .as_mut_ptr()
                    .offset(0 as libc::c_int as isize) = 1 as libc::c_int != 0;
                if (*player._t._t_room).r_flags as libc::c_int & 0x2 as libc::c_int != 0
                {
                    msg(
                        b"the corridor glows and then fades\0" as *const u8
                            as *const libc::c_char,
                    );
                } else {
                    msg(
                        b"the room is lit by a shimmering blue light\0" as *const u8
                            as *const libc::c_char,
                    );
                }
            }
            if (*player._t._t_room).r_flags as libc::c_int & 0x2 as libc::c_int == 0 {
                (*player._t._t_room)
                    .r_flags = ((*player._t._t_room).r_flags as libc::c_int
                    & !(0x1 as libc::c_int)) as libc::c_short;
                enter_room(&mut player._t._t_pos);
            }
        }
        9 => {
            if player._t._t_stats.s_hpt < 2 as libc::c_int {
                msg(b"you are too weak to use it\0" as *const u8 as *const libc::c_char);
                return;
            } else {
                drain();
            }
        }
        5 | 11 | 12 | 13 | 14 => {
            let mut monster: byte = 0;
            let mut oldch: byte = 0;
            let mut rm: libc::c_int = 0;
            let mut new_yx: coord = coord { x: 0, y: 0 };
            y = player._t._t_pos.y;
            x = player._t._t_pos.x;
            while step_ok(winat(y, x)) {
                y += delta.y;
                x += delta.x;
            }
            tp = moat(y, x);
            if !tp.is_null() {
                let mut omonst: byte = 0;
                monster = (*tp)._t._t_type as byte;
                omonst = monster;
                if monster as libc::c_int == 'F' as i32 {
                    player
                        ._t
                        ._t_flags = (player._t._t_flags as libc::c_int
                        & !(0x80 as libc::c_int)) as libc::c_short;
                }
                if which_one == 14 as libc::c_int {
                    if monster as libc::c_int == (*obj)._o._o_enemy as libc::c_int {
                        msg(
                            b"the %s vanishes in a puff of smoke\0" as *const u8
                                as *const libc::c_char,
                            (*monsters
                                .as_mut_ptr()
                                .offset((monster as libc::c_int - 'A' as i32) as isize))
                                .m_name,
                        );
                        killed(tp, 0 as libc::c_int != 0);
                    } else {
                        msg(
                            b"you hear a maniacal chuckle in the distance.\0"
                                as *const u8 as *const libc::c_char,
                        );
                    }
                } else if which_one == 5 as libc::c_int {
                    let mut pp: *mut THING = 0 as *mut THING;
                    pp = (*tp)._t._t_pack;
                    list_detach(&mut mlist, tp);
                    if see_monst(tp) {
                        cur_mvaddch(y, x, *_level.offset(INDEX(y, x) as isize));
                    }
                    oldch = (*tp)._t._t_oldch;
                    delta.y = y;
                    delta.x = x;
                    monster = (rnd(26 as libc::c_int) + 'A' as i32) as byte;
                    new_monster(tp, monster, &mut delta);
                    if see_monst(tp) {
                        cur_mvaddch(y, x, monster);
                    }
                    (*tp)._t._t_oldch = oldch;
                    (*tp)._t._t_pack = pp;
                    let ref mut fresh0 = *ws_know
                        .as_mut_ptr()
                        .offset(5 as libc::c_int as isize);
                    // Rust port: Fix weakly typed booleans
                    *fresh0 = *fresh0 | (monster != omonst);
                } else if which_one == 13 as libc::c_int {
                    (*tp)
                        ._t
                        ._t_flags = ((*tp)._t._t_flags as libc::c_int
                        | 0x1000 as libc::c_int) as libc::c_short;
                    (*tp)
                        ._t
                        ._t_flags = ((*tp)._t._t_flags as libc::c_int
                        & !(0x10 as libc::c_int | 0x400 as libc::c_int))
                        as libc::c_short;
                    (*tp)._t._t_disguise = (*tp)._t._t_type as byte;
                } else {
                    if see_monst(tp) {
                        cur_mvaddch(y, x, (*tp)._t._t_oldch);
                    }
                    if which_one == 11 as libc::c_int {
                        (*tp)._t._t_oldch = '@' as i32 as byte;
                        loop {
                            rm = rnd_room();
                            new_yx = (*tp)._t._t_pos;
                            rnd_pos(
                                &mut *rooms.as_mut_ptr().offset(rm as isize),
                                &mut new_yx,
                            );
                            if winat(new_yx.y, new_yx.x) as libc::c_int
                                == 0xfa as libc::c_int
                                || winat(new_yx.y, new_yx.x) as libc::c_int
                                    == 0xb1 as libc::c_int
                            {
                                break;
                            }
                        }
                        (*tp)._t._t_pos = new_yx;
                        if see_monst(tp) {
                            cur_mvaddch(
                                (*tp)._t._t_pos.y,
                                (*tp)._t._t_pos.x,
                                (*tp)._t._t_disguise,
                            );
                        } else if player._t._t_flags as libc::c_int & 0x2 as libc::c_int
                            != 0 as libc::c_int
                        {
                            set_attr(14 as libc::c_int);
                            cur_mvaddch(
                                (*tp)._t._t_pos.y,
                                (*tp)._t._t_pos.x,
                                (*tp)._t._t_disguise,
                            );
                            set_attr(0 as libc::c_int);
                        }
                    } else {
                        (*tp)._t._t_pos.y = player._t._t_pos.y + delta.y;
                        (*tp)._t._t_pos.x = player._t._t_pos.x + delta.x;
                    }
                    if (*tp)._t._t_type as libc::c_int == 'F' as i32 {
                        player
                            ._t
                            ._t_flags = (player._t._t_flags as libc::c_int
                            & !(0x80 as libc::c_int)) as libc::c_short;
                    }
                    if (*tp)._t._t_pos.y != y || (*tp)._t._t_pos.x != x {
                        (*tp)
                            ._t
                            ._t_oldch = cur_mvinch((*tp)._t._t_pos.y, (*tp)._t._t_pos.x);
                    }
                }
                (*tp)._t._t_dest = &mut player._t._t_pos;
                (*tp)
                    ._t
                    ._t_flags = ((*tp)._t._t_flags as libc::c_int | 0x4 as libc::c_int)
                    as libc::c_short;
            }
        }
        6 => {
            let mut bolt: THING = thing {
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
            *ws_know
                .as_mut_ptr()
                .offset(6 as libc::c_int as isize) = 1 as libc::c_int != 0;
            bolt._o._o_type = '*' as i32;
            bolt
                ._o
                ._o_hurldmg = b"1d8\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
            bolt._o._o_hplus = 1000 as libc::c_int;
            bolt._o._o_dplus = 1 as libc::c_int;
            bolt._o._o_flags = 0x10 as libc::c_int as libc::c_short;
            if !cur_weapon.is_null() {
                bolt._o._o_launch = (*cur_weapon)._o._o_which as libc::c_char;
            }
            do_motion(&mut bolt, delta.y, delta.x);
            tp = moat(bolt._o._o_pos.y, bolt._o._o_pos.x);
            if !tp.is_null() && !save_throw(0o3 as libc::c_int, tp) {
                hit_monster(bolt._o._o_pos.y, bolt._o._o_pos.x, &mut bolt);
            } else {
                msg(
                    b"the missle vanishes with a puff of smoke\0" as *const u8
                        as *const libc::c_char,
                );
            }
        }
        1 => {
            delta.y += player._t._t_pos.y;
            delta.x += player._t._t_pos.x;
            tp = moat(delta.y, delta.x);
            if !tp.is_null() {
                if rnd(20 as libc::c_int) == 0 as libc::c_int {
                    (*obj)
                        ._o
                        ._o_damage = b"3d8\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char;
                    (*obj)._o._o_dplus = 9 as libc::c_int;
                } else {
                    (*obj)
                        ._o
                        ._o_damage = b"2d8\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char;
                    (*obj)._o._o_dplus = 4 as libc::c_int;
                }
                fight(&mut delta, (*tp)._t._t_type, obj, 0 as libc::c_int != 0);
            }
        }
        7 | 8 => {
            y = player._t._t_pos.y;
            x = player._t._t_pos.x;
            while step_ok(winat(y, x)) {
                y += delta.y;
                x += delta.x;
            }
            tp = moat(y, x);
            if !tp.is_null() {
                if which_one == 7 as libc::c_int {
                    if (*tp)._t._t_flags as libc::c_int & 0x2000 as libc::c_int
                        != 0 as libc::c_int
                    {
                        (*tp)
                            ._t
                            ._t_flags = ((*tp)._t._t_flags as libc::c_int
                            & !(0x2000 as libc::c_int)) as libc::c_short;
                    } else {
                        (*tp)
                            ._t
                            ._t_flags = ((*tp)._t._t_flags as libc::c_int
                            | 0x4000 as libc::c_int) as libc::c_short;
                    }
                } else {
                    if (*tp)._t._t_flags as libc::c_int & 0x4000 as libc::c_int
                        != 0 as libc::c_int
                    {
                        (*tp)
                            ._t
                            ._t_flags = ((*tp)._t._t_flags as libc::c_int
                            & !(0x4000 as libc::c_int)) as libc::c_short;
                    } else {
                        (*tp)
                            ._t
                            ._t_flags = ((*tp)._t._t_flags as libc::c_int
                            | 0x2000 as libc::c_int) as libc::c_short;
                    }
                    (*tp)._t._t_turn = 1 as libc::c_int as libc::c_char;
                }
                delta.y = y;
                delta.x = x;
                start_run(&mut delta);
            }
        }
        2 | 3 | 4 => {
            if which_one == 2 as libc::c_int {
                name = b"bolt\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char;
            } else if which_one == 3 as libc::c_int {
                name = b"flame\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char;
            } else {
                name = b"ice\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
            }
            fire_bolt(&mut player._t._t_pos, &mut delta, name);
            *ws_know.as_mut_ptr().offset(which_one as isize) = 1 as libc::c_int != 0;
        }
        _ => {}
    }
    (*obj)._o._o_ac -= 1;
    if ((*obj)._o._o_ac as libc::c_int) < 0 as libc::c_int {
        (*obj)._o._o_ac = 0 as libc::c_int as libc::c_short;
    }
}
#[no_mangle]
pub unsafe extern "C" fn drain() {
    let mut mp: *mut THING = 0 as *mut THING;
    let mut cnt: libc::c_int = 0;
    let mut corp: *mut room = 0 as *mut room;
    let mut dp: *mut *mut THING = 0 as *mut *mut THING;
    let mut inpass: bool = false;
    let mut drainee: [*mut THING; 40] = [0 as *mut THING; 40];
    cnt = 0 as libc::c_int;
    if *_level.offset(INDEX(player._t._t_pos.y, player._t._t_pos.x) as isize)
        as libc::c_int == 0xce as libc::c_int
    {
        corp = &mut *passages
            .as_mut_ptr()
            .offset(
                (*_flags
                    .offset(
                        (INDEX
                            as unsafe extern "C" fn(
                                libc::c_int,
                                libc::c_int,
                            ) -> libc::c_int)(player._t._t_pos.y, player._t._t_pos.x)
                            as isize,
                    ) as libc::c_int & 0xf as libc::c_int) as isize,
            ) as *mut room;
    } else {
        corp = 0 as *mut room;
    }
    inpass = (*player._t._t_room).r_flags as libc::c_int & 0x2 as libc::c_int != 0;
    dp = drainee.as_mut_ptr();
    mp = mlist;
    while !mp.is_null() {
        if (*mp)._t._t_room == player._t._t_room || (*mp)._t._t_room == corp
            || inpass as libc::c_int != 0
                && *_level.offset(INDEX((*mp)._t._t_pos.y, (*mp)._t._t_pos.x) as isize)
                    as libc::c_int == 0xce as libc::c_int
                && &mut *passages
                    .as_mut_ptr()
                    .offset(
                        (*_flags
                            .offset(
                                (INDEX
                                    as unsafe extern "C" fn(
                                        libc::c_int,
                                        libc::c_int,
                                    ) -> libc::c_int)((*mp)._t._t_pos.y, (*mp)._t._t_pos.x)
                                    as isize,
                            ) as libc::c_int & 0xf as libc::c_int) as isize,
                    ) as *mut room == player._t._t_room
        {
            let fresh1 = dp;
            dp = dp.offset(1);
            *fresh1 = mp;
        }
        mp = (*mp)._t._l_next;
    }
    cnt = dp.offset_from(drainee.as_mut_ptr()) as libc::c_long as libc::c_int;
    if cnt == 0 as libc::c_int {
        msg(b"you have a tingling feeling\0" as *const u8 as *const libc::c_char);
        return;
    }
    *dp = 0 as *mut THING;
    player._t._t_stats.s_hpt /= 2 as libc::c_int;
    cnt = player._t._t_stats.s_hpt / cnt + 1 as libc::c_int;
    dp = drainee.as_mut_ptr();
    while !(*dp).is_null() {
        mp = *dp;
        (*mp)._t._t_stats.s_hpt -= cnt;
        if (*mp)._t._t_stats.s_hpt <= 0 as libc::c_int {
            killed(mp, see_monst(mp));
        } else {
            start_run(&mut (*mp)._t._t_pos);
        }
        dp = dp.offset(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn fire_bolt(
    mut start: *mut coord,
    mut dir: *mut coord,
    mut name: *mut libc::c_char,
) {
    let mut dirch: byte = 0 as libc::c_int as byte;
    let mut ch: byte = 0;
    let mut tp: *mut THING = 0 as *mut THING;
    let mut hit_hero: bool = false;
    let mut used: bool = false;
    let mut changed: bool = false;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut pos: coord = coord { x: 0, y: 0 };
    let mut spotpos: [C2RustUnnamed_1; 12] = [C2RustUnnamed_1 {
        s_pos: coord { x: 0, y: 0 },
        s_under: 0,
    }; 12];
    let mut bolt: THING = thing {
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
    let mut is_frost: bool = false;
    is_frost = strcmp(name, b"frost\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int;
    bolt._o._o_type = 0x18 as libc::c_int;
    bolt._o._o_which = 10 as libc::c_int;
    bolt
        ._o
        ._o_hurldmg = b"6d6\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    bolt._o._o_damage = bolt._o._o_hurldmg;
    bolt._o._o_hplus = 30 as libc::c_int;
    bolt._o._o_dplus = 0 as libc::c_int;
    let ref mut fresh2 = *w_names.as_mut_ptr().offset(10 as libc::c_int as isize);
    *fresh2 = name;
    match (*dir).y + (*dir).x {
        0 => {
            dirch = '/' as i32 as byte;
        }
        1 | -1 => {
            dirch = (if (*dir).y == 0 as libc::c_int { '-' as i32 } else { '|' as i32 })
                as byte;
        }
        2 | -2 => {
            dirch = '\\' as i32 as byte;
        }
        _ => {}
    }
    pos = *start;
    hit_hero = start != &mut player._t._t_pos as *mut coord;
    used = 0 as libc::c_int != 0;
    changed = 0 as libc::c_int != 0;
    i = 0 as libc::c_int;
    while i < 6 as libc::c_int && !used {
        pos.y += (*dir).y;
        pos.x += (*dir).x;
        ch = winat(pos.y, pos.x);
        spotpos[i as usize].s_pos = pos;
        spotpos[i as usize].s_under = cur_mvinch(pos.y, pos.x);
        if spotpos[i as usize].s_under as libc::c_int == dirch as libc::c_int {
            spotpos[i as usize].s_under = 0 as libc::c_int as byte;
        }
        match ch as libc::c_int {
            206 | 205 | 186 | 201 | 187 | 200 | 188 | 32 => {
                if !changed {
                    hit_hero = !hit_hero;
                }
                changed = 0 as libc::c_int != 0;
                (*dir).y = -(*dir).y;
                (*dir).x = -(*dir).x;
                i -= 1;
                msg(b"the %s bounces\0" as *const u8 as *const libc::c_char, name);
            }
            _ => {
                if !hit_hero
                    && {
                        tp = moat(pos.y, pos.x);
                        !tp.is_null()
                    }
                {
                    hit_hero = 1 as libc::c_int != 0;
                    changed = !changed;
                    if (*tp)._t._t_oldch as libc::c_int != '@' as i32 {
                        (*tp)._t._t_oldch = *_level.offset(INDEX(pos.y, pos.x) as isize);
                    }
                    if !save_throw(0o3 as libc::c_int, tp)
                        || is_frost as libc::c_int != 0
                    {
                        bolt._o._o_pos = pos;
                        used = 1 as libc::c_int != 0;
                        if (*tp)._t._t_type as libc::c_int == 'D' as i32
                            && strcmp(
                                name,
                                b"flame\0" as *const u8 as *const libc::c_char,
                            ) == 0 as libc::c_int
                        {
                            msg(
                                b"the flame bounces off the dragon\0" as *const u8
                                    as *const libc::c_char,
                            );
                        } else {
                            hit_monster(pos.y, pos.x, &mut bolt);
                            if cur_mvinch(pos.y, pos.x) as libc::c_int
                                != dirch as libc::c_int
                            {
                                spotpos[i as usize].s_under = cur_mvinch(pos.y, pos.x);
                            }
                        }
                    } else if ch as libc::c_int != 'X' as i32
                        || (*tp)._t._t_disguise as libc::c_int == 'X' as i32
                    {
                        if start == &mut player._t._t_pos as *mut coord {
                            start_run(&mut pos);
                        }
                        msg(
                            b"the %s whizzes past the %s\0" as *const u8
                                as *const libc::c_char,
                            name,
                            (*monsters
                                .as_mut_ptr()
                                .offset((ch as libc::c_int - 'A' as i32) as isize))
                                .m_name,
                        );
                    }
                } else if hit_hero as libc::c_int != 0
                    && _ce(&mut pos, &mut player._t._t_pos) as libc::c_int != 0
                {
                    hit_hero = 0 as libc::c_int != 0;
                    changed = !changed;
                    if !save(0o3 as libc::c_int) {
                        if is_frost {
                            msg(
                                b"You are frozen by a blast of frost%s.\0" as *const u8
                                    as *const libc::c_char,
                                noterse(
                                    b" from the Ice Monster\0" as *const u8
                                        as *const libc::c_char as *mut libc::c_char,
                                ),
                            );
                            if no_command < 20 as libc::c_int {
                                no_command += spread(7 as libc::c_int);
                            }
                        } else {
                            player._t._t_stats.s_hpt
                                -= roll(6 as libc::c_int, 6 as libc::c_int);
                            if player._t._t_stats.s_hpt <= 0 as libc::c_int {
                                if start == &mut player._t._t_pos as *mut coord {
                                    death('b' as i32 as libc::c_char);
                                } else {
                                    death((*moat((*start).y, (*start).x))._t._t_type);
                                }
                            }
                        }
                        used = 1 as libc::c_int != 0;
                        if !is_frost {
                            msg(
                                b"you are hit by the %s\0" as *const u8
                                    as *const libc::c_char,
                                name,
                            );
                        }
                    } else {
                        msg(
                            b"the %s whizzes by you\0" as *const u8
                                as *const libc::c_char,
                            name,
                        );
                    }
                }
                if is_frost {
                    set_attr(13 as libc::c_int);
                } else {
                    set_attr(3 as libc::c_int);
                }
                tick_pause();
                cur_mvaddch(pos.y, pos.x, dirch);
                set_attr(0 as libc::c_int);
            }
        }
        i += 1;
    }
    j = 0 as libc::c_int;
    while j < i {
        tick_pause();
        if spotpos[j as usize].s_under != 0 {
            cur_mvaddch(
                spotpos[j as usize].s_pos.y,
                spotpos[j as usize].s_pos.x,
                spotpos[j as usize].s_under,
            );
        }
        j += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn charge_str(mut obj: *mut THING) -> *mut libc::c_char {
    static mut buf: [libc::c_char; 20] = [0; 20];
    if (*obj)._o._o_flags as libc::c_int & 0x2 as libc::c_int == 0 {
        buf[0 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
    } else {
        sprintf(
            buf.as_mut_ptr(),
            b" [%d charges]\0" as *const u8 as *const libc::c_char,
            (*obj)._o._o_ac as libc::c_int,
        );
    }
    return buf.as_mut_ptr();
}
