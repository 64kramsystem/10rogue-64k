use ::libc;
extern "C" {
    fn __ctype_toupper_loc() -> *mut *const __int32_t;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn flush_type();
    static mut it: *mut libc::c_char;
    static mut you: *mut libc::c_char;
    static mut expert: bool;
    static mut running: bool;
    static mut terse: bool;
    static mut he_man: [*mut libc::c_char; 0];
    static mut w_names: [*mut libc::c_char; 0];
    static mut a_class: [libc::c_int; 0];
    static mut count: libc::c_int;
    static mut fung_hit: libc::c_int;
    static mut inpack: libc::c_int;
    static mut level: libc::c_int;
    static mut max_level: libc::c_int;
    static mut no_command: libc::c_int;
    static mut purse: libc::c_int;
    static mut quiet: libc::c_int;
    static mut hit_mul: libc::c_int;
    static mut cur_armor: *mut THING;
    static mut cur_ring: [*mut THING; 0];
    static mut cur_weapon: *mut THING;
    static mut mlist: *mut THING;
    static mut player: THING;
    static mut monsters: [monster; 0];
    static mut tbuf: *mut libc::c_char;
    static mut _level: *mut byte;
    static mut e_levels: *mut libc::c_long;
    fn start_run(runner: *mut coord);
    fn cansee(y: libc::c_int, x: libc::c_int) -> bool;
    fn slime_split(tp: *mut THING);
    fn rnd(range: libc::c_int) -> libc::c_int;
    fn msg(fmt: *const libc::c_char, _: ...);
    fn addmsg(fmt: *const libc::c_char, _: ...);
    fn roll(number: libc::c_int, sides: libc::c_int) -> libc::c_int;
    fn discard(item: *mut THING) -> libc::c_int;
    fn list_detach(list: *mut *mut THING, item: *mut THING);
    fn INDEX(y: libc::c_int, x: libc::c_int) -> libc::c_int;
    fn fall(obj: *mut THING, pr: bool);
    fn list_attach(list: *mut *mut THING, item: *mut THING);
    fn new_item() -> *mut THING;
    fn f_restor();
    fn th_effect(obj: *mut THING, tp: *mut THING);
    fn moat(my: libc::c_int, mx: libc::c_int) -> *mut THING;
    fn status();
    fn death(monst: libc::c_char);
    fn inv_name(obj: *mut THING, drop_0: bool) -> *mut libc::c_char;
    fn noterse(str: *mut libc::c_char) -> *mut libc::c_char;
    fn chg_str(amt: libc::c_int);
    fn cur_mvaddch(r: libc::c_int, c: libc::c_int, chr: byte);
    fn set_attr(bute: libc::c_int);
}
pub type __int32_t = libc::c_int;
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
#[inline]
unsafe extern "C" fn toupper(mut __c: libc::c_int) -> libc::c_int {
    return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
        *(*__ctype_toupper_loc()).offset(__c as isize)
    } else {
        __c
    };
}
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fight(
    mut mp: *mut coord,
    mut mn: libc::c_char,
    mut weap: *mut THING,
    mut thrown: bool,
) -> bool {
    let mut tp: *mut THING = 0 as *mut THING;
    let mut mname: *mut libc::c_char = 0 as *mut libc::c_char;
    tp = moat((*mp).y, (*mp).x);
    if tp.is_null() {
        return 0 as libc::c_int != 0;
    }
    quiet = 0 as libc::c_int;
    count = quiet;
    start_run(mp);
    if (*tp)._t._t_type as libc::c_int == 'X' as i32
        && (*tp)._t._t_disguise as libc::c_int != 'X' as i32
        && !(player._t._t_flags as libc::c_int & 0x1 as libc::c_int != 0 as libc::c_int)
    {
        (*tp)._t._t_disguise = 'X' as i32 as byte;
        mn = (*tp)._t._t_disguise as libc::c_char;
        if thrown {
            return 0 as libc::c_int != 0;
        }
        msg(b"wait! That's a Xeroc!\0" as *const u8 as *const libc::c_char);
    }
    mname = (*monsters.as_mut_ptr().offset((mn as libc::c_int - 'A' as i32) as isize))
        .m_name;
    if player._t._t_flags as libc::c_int & 0x1 as libc::c_int != 0 as libc::c_int {
        mname = it;
    }
    if roll_em(&mut player, tp, weap, thrown) as libc::c_int != 0
        || !weap.is_null() && (*weap)._o._o_type == 0xad as libc::c_int
    {
        let mut did_huh: bool = 0 as libc::c_int != 0;
        if thrown {
            thunk(
                weap,
                mname,
                b"hits\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                b"hit\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
        } else {
            hit(0 as *mut libc::c_char, mname);
        }
        if !weap.is_null() && (*weap)._o._o_type == 0xad as libc::c_int {
            th_effect(weap, tp);
            if !thrown {
                if (*weap)._o._o_count > 1 as libc::c_int {
                    (*weap)._o._o_count -= 1;
                } else {
                    list_detach(&mut player._t._t_pack, weap);
                    discard(weap);
                }
                cur_weapon = 0 as *mut THING;
            }
        }
        if player._t._t_flags as libc::c_int & 0x400 as libc::c_int != 0 as libc::c_int {
            did_huh = 1 as libc::c_int != 0;
            (*tp)
                ._t
                ._t_flags = ((*tp)._t._t_flags as libc::c_int | 0x100 as libc::c_int)
                as libc::c_short;
            player
                ._t
                ._t_flags = (player._t._t_flags as libc::c_int & !(0x400 as libc::c_int))
                as libc::c_short;
            msg(b"your hands stop glowing red\0" as *const u8 as *const libc::c_char);
        }
        if (*tp)._t._t_stats.s_hpt <= 0 as libc::c_int {
            killed(tp, 1 as libc::c_int != 0);
        } else if did_huh as libc::c_int != 0
            && !(player._t._t_flags as libc::c_int & 0x1 as libc::c_int
                != 0 as libc::c_int)
        {
            msg(b"the %s appears confused\0" as *const u8 as *const libc::c_char, mname);
        }
        return 1 as libc::c_int != 0;
    }
    if thrown {
        thunk(
            weap,
            mname,
            b"misses\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"missed\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    } else {
        miss(0 as *mut libc::c_char, mname);
    }
    if (*tp)._t._t_type as libc::c_int == 'S' as i32
        && rnd(100 as libc::c_int) > 25 as libc::c_int
    {
        slime_split(tp);
    }
    return 0 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn attack(mut mp: *mut THING) {
    let mut mname: *mut libc::c_char = 0 as *mut libc::c_char;
    running = 0 as libc::c_int != 0;
    quiet = 0 as libc::c_int;
    count = quiet;
    if (*mp)._t._t_type as libc::c_int == 'X' as i32
        && !(player._t._t_flags as libc::c_int & 0x1 as libc::c_int != 0 as libc::c_int)
    {
        (*mp)._t._t_disguise = 'X' as i32 as byte;
    }
    mname = (*monsters
        .as_mut_ptr()
        .offset(((*mp)._t._t_type as libc::c_int - 'A' as i32) as isize))
        .m_name;
    if player._t._t_flags as libc::c_int & 0x1 as libc::c_int != 0 as libc::c_int {
        mname = it;
    }
    if roll_em(mp, &mut player, 0 as *mut THING, 0 as libc::c_int != 0) {
        hit(mname, 0 as *mut libc::c_char);
        if player._t._t_stats.s_hpt <= 0 as libc::c_int {
            death((*mp)._t._t_type);
        }
        if !((*mp)._t._t_flags as libc::c_int & 0x1000 as libc::c_int
            != 0 as libc::c_int)
        {
            match (*mp)._t._t_type as libc::c_int {
                65 => {
                    if !cur_armor.is_null()
                        && ((*cur_armor)._o._o_ac as libc::c_int) < 9 as libc::c_int
                        && (*cur_armor)._o._o_which != 0 as libc::c_int
                    {
                        if !(*cur_ring.as_mut_ptr().offset(0 as libc::c_int as isize))
                            .is_null()
                            && (**cur_ring
                                .as_mut_ptr()
                                .offset(0 as libc::c_int as isize))
                                ._o
                                ._o_which == 13 as libc::c_int
                            || !(*cur_ring
                                .as_mut_ptr()
                                .offset(1 as libc::c_int as isize))
                                .is_null()
                                && (**cur_ring
                                    .as_mut_ptr()
                                    .offset(1 as libc::c_int as isize))
                                    ._o
                                    ._o_which == 13 as libc::c_int
                        {
                            msg(
                                b"the rust vanishes instantly\0" as *const u8
                                    as *const libc::c_char,
                            );
                        } else {
                            msg(
                                b"your armor weakens, oh my!\0" as *const u8
                                    as *const libc::c_char,
                            );
                            (*cur_armor)._o._o_ac += 1;
                        }
                    }
                }
                73 => {
                    if no_command > 1 as libc::c_int {
                        no_command -= 1;
                    }
                }
                82 => {
                    if !save(0 as libc::c_int) {
                        if !(!(*cur_ring.as_mut_ptr().offset(0 as libc::c_int as isize))
                            .is_null()
                            && (**cur_ring
                                .as_mut_ptr()
                                .offset(0 as libc::c_int as isize))
                                ._o
                                ._o_which == 2 as libc::c_int
                            || !(*cur_ring
                                .as_mut_ptr()
                                .offset(1 as libc::c_int as isize))
                                .is_null()
                                && (**cur_ring
                                    .as_mut_ptr()
                                    .offset(1 as libc::c_int as isize))
                                    ._o
                                    ._o_which == 2 as libc::c_int)
                        {
                            chg_str(-(1 as libc::c_int));
                            msg(
                                b"you feel a bite in your leg%s\0" as *const u8
                                    as *const libc::c_char,
                                noterse(
                                    b" and now feel weaker\0" as *const u8
                                        as *const libc::c_char as *mut libc::c_char,
                                ),
                            );
                        } else {
                            msg(
                                b"a bite momentarily weakens you\0" as *const u8
                                    as *const libc::c_char,
                            );
                        }
                    }
                }
                87 | 86 => {
                    if rnd(100 as libc::c_int)
                        < (if (*mp)._t._t_type as libc::c_int == 'W' as i32 {
                            15 as libc::c_int
                        } else {
                            30 as libc::c_int
                        })
                    {
                        let mut fewer: libc::c_int = 0;
                        if (*mp)._t._t_type as libc::c_int == 'W' as i32 {
                            if player._t._t_stats.s_exp
                                == 0 as libc::c_int as libc::c_long
                            {
                                death('W' as i32 as libc::c_char);
                            }
                            player._t._t_stats.s_lvl -= 1;
                            if player._t._t_stats.s_lvl == 0 as libc::c_int {
                                player._t._t_stats.s_exp = 0 as libc::c_int as libc::c_long;
                                player._t._t_stats.s_lvl = 1 as libc::c_int;
                            } else {
                                player
                                    ._t
                                    ._t_stats
                                    .s_exp = *e_levels
                                    .offset(
                                        (player._t._t_stats.s_lvl - 1 as libc::c_int) as isize,
                                    ) + 1 as libc::c_int as libc::c_long;
                            }
                            fewer = roll(1 as libc::c_int, 10 as libc::c_int);
                        } else {
                            fewer = roll(1 as libc::c_int, 5 as libc::c_int);
                        }
                        player._t._t_stats.s_hpt -= fewer;
                        player._t._t_stats.s_maxhp -= fewer;
                        if player._t._t_stats.s_hpt < 1 as libc::c_int {
                            player._t._t_stats.s_hpt = 1 as libc::c_int;
                        }
                        if player._t._t_stats.s_maxhp < 1 as libc::c_int {
                            death((*mp)._t._t_type);
                        }
                        msg(
                            b"you suddenly feel weaker\0" as *const u8
                                as *const libc::c_char,
                        );
                    }
                }
                70 => {
                    player
                        ._t
                        ._t_flags = (player._t._t_flags as libc::c_int
                        | 0x80 as libc::c_int) as libc::c_short;
                    fung_hit += 1;
                    sprintf(
                        (*mp)._t._t_stats.s_dmg,
                        b"%dd1\0" as *const u8 as *const libc::c_char,
                        fung_hit,
                    );
                }
                76 => {
                    let mut lastpurse: libc::c_long = 0;
                    lastpurse = purse as libc::c_long;
                    purse
                        -= rnd(50 as libc::c_int + 10 as libc::c_int * level)
                            + 2 as libc::c_int;
                    if !save(0o3 as libc::c_int) {
                        purse
                            -= rnd(50 as libc::c_int + 10 as libc::c_int * level)
                                + 2 as libc::c_int
                                + (rnd(50 as libc::c_int + 10 as libc::c_int * level)
                                    + 2 as libc::c_int)
                                + (rnd(50 as libc::c_int + 10 as libc::c_int * level)
                                    + 2 as libc::c_int)
                                + (rnd(50 as libc::c_int + 10 as libc::c_int * level)
                                    + 2 as libc::c_int);
                    }
                    if purse < 0 as libc::c_int {
                        purse = 0 as libc::c_int;
                    }
                    remove_monster(&mut (*mp)._t._t_pos, mp, 0 as libc::c_int != 0);
                    if purse as libc::c_long != lastpurse {
                        msg(
                            b"your purse feels lighter\0" as *const u8
                                as *const libc::c_char,
                        );
                    }
                }
                78 => {
                    let mut obj: *mut THING = 0 as *mut THING;
                    let mut steal: *mut THING = 0 as *mut THING;
                    let mut nobj: libc::c_int = 0;
                    let mut she_stole: *mut libc::c_char = b"she stole %s!\0"
                        as *const u8 as *const libc::c_char as *mut libc::c_char;
                    steal = 0 as *mut THING;
                    nobj = 0 as libc::c_int;
                    obj = player._t._t_pack;
                    while !obj.is_null() {
                        if obj != cur_armor && obj != cur_weapon
                            && obj
                                != *cur_ring.as_mut_ptr().offset(0 as libc::c_int as isize)
                            && obj
                                != *cur_ring.as_mut_ptr().offset(1 as libc::c_int as isize)
                            && is_magic(obj) as libc::c_int != 0
                            && {
                                nobj += 1;
                                rnd(nobj) == 0 as libc::c_int
                            }
                        {
                            steal = obj;
                        }
                        obj = (*obj)._t._l_next;
                    }
                    if !steal.is_null() {
                        remove_monster(&mut (*mp)._t._t_pos, mp, 0 as libc::c_int != 0);
                        inpack -= 1;
                        if (*steal)._o._o_count > 1 as libc::c_int
                            && (*steal)._o._o_group == 0 as libc::c_int
                        {
                            let mut oc: libc::c_int = 0;
                            let fresh0 = (*steal)._o._o_count;
                            (*steal)._o._o_count = (*steal)._o._o_count - 1;
                            oc = fresh0;
                            (*steal)._o._o_count = 1 as libc::c_int;
                            msg(she_stole, inv_name(steal, 1 as libc::c_int != 0));
                            (*steal)._o._o_count = oc;
                        } else {
                            list_detach(&mut player._t._t_pack, steal);
                            discard(steal);
                            msg(she_stole, inv_name(steal, 1 as libc::c_int != 0));
                        }
                    }
                }
                _ => {}
            }
        }
    } else if (*mp)._t._t_type as libc::c_int != 'I' as i32 {
        if (*mp)._t._t_type as libc::c_int == 'F' as i32 {
            player._t._t_stats.s_hpt -= fung_hit;
            if player._t._t_stats.s_hpt <= 0 as libc::c_int {
                death((*mp)._t._t_type);
            }
        }
        miss(mname, 0 as *mut libc::c_char);
    }
    flush_type();
    count = 0 as libc::c_int;
    status();
}
#[no_mangle]
pub unsafe extern "C" fn swing(
    mut at_lvl: libc::c_int,
    mut op_arm: libc::c_int,
    mut wplus: libc::c_int,
) -> bool {
    let mut res: libc::c_int = rnd(20 as libc::c_int);
    let mut need: libc::c_int = 20 as libc::c_int - at_lvl - op_arm;
    return res + wplus >= need;
}
#[no_mangle]
pub unsafe extern "C" fn check_level() {
    let mut i: libc::c_int = 0;
    let mut add: libc::c_int = 0;
    let mut olevel: libc::c_int = 0;
    i = 0 as libc::c_int;
    while *e_levels.offset(i as isize) != 0 as libc::c_int as libc::c_long {
        if *e_levels.offset(i as isize) > player._t._t_stats.s_exp {
            break;
        }
        i += 1;
    }
    i += 1;
    olevel = player._t._t_stats.s_lvl;
    player._t._t_stats.s_lvl = i;
    if i > olevel {
        add = roll(i - olevel, 10 as libc::c_int);
        player._t._t_stats.s_maxhp += add;
        player._t._t_stats.s_hpt += add;
        if player._t._t_stats.s_hpt > player._t._t_stats.s_maxhp {
            player._t._t_stats.s_hpt = player._t._t_stats.s_maxhp;
        }
        msg(
            b"and achieve the rank of \"%s\"\0" as *const u8 as *const libc::c_char,
            *he_man.as_mut_ptr().offset((i - 1 as libc::c_int) as isize),
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn roll_em(
    mut thatt: *mut THING,
    mut thdef: *mut THING,
    mut weap: *mut THING,
    mut hurl: bool,
) -> bool {
    let mut att: *mut stats = 0 as *mut stats;
    let mut def: *mut stats = 0 as *mut stats;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ndice: libc::c_int = 0;
    let mut nsides: libc::c_int = 0;
    let mut def_arm: libc::c_int = 0;
    let mut did_hit: bool = 0 as libc::c_int != 0;
    let mut hplus: libc::c_int = 0;
    let mut dplus: libc::c_int = 0;
    let mut damage: libc::c_int = 0;
    att = &mut (*thatt)._t._t_stats;
    def = &mut (*thdef)._t._t_stats;
    if weap.is_null() {
        cp = (*att).s_dmg;
        dplus = 0 as libc::c_int;
        hplus = 0 as libc::c_int;
    } else {
        hplus = (*weap)._o._o_hplus;
        dplus = (*weap)._o._o_dplus;
        if (*thdef)._t._t_type as libc::c_int == (*weap)._o._o_enemy as libc::c_int {
            hplus += 4 as libc::c_int;
            dplus += 4 as libc::c_int;
        }
        if weap == cur_weapon {
            if !(*cur_ring.as_mut_ptr().offset(0 as libc::c_int as isize)).is_null()
                && (**cur_ring.as_mut_ptr().offset(0 as libc::c_int as isize))
                    ._o
                    ._o_which == 8 as libc::c_int
            {
                dplus
                    += (**cur_ring.as_mut_ptr().offset(0 as libc::c_int as isize))
                        ._o
                        ._o_ac as libc::c_int;
            } else if !(*cur_ring.as_mut_ptr().offset(0 as libc::c_int as isize))
                .is_null()
                && (**cur_ring.as_mut_ptr().offset(0 as libc::c_int as isize))
                    ._o
                    ._o_which == 7 as libc::c_int
            {
                hplus
                    += (**cur_ring.as_mut_ptr().offset(0 as libc::c_int as isize))
                        ._o
                        ._o_ac as libc::c_int;
            }
            if !(*cur_ring.as_mut_ptr().offset(1 as libc::c_int as isize)).is_null()
                && (**cur_ring.as_mut_ptr().offset(1 as libc::c_int as isize))
                    ._o
                    ._o_which == 8 as libc::c_int
            {
                dplus
                    += (**cur_ring.as_mut_ptr().offset(1 as libc::c_int as isize))
                        ._o
                        ._o_ac as libc::c_int;
            } else if !(*cur_ring.as_mut_ptr().offset(1 as libc::c_int as isize))
                .is_null()
                && (**cur_ring.as_mut_ptr().offset(1 as libc::c_int as isize))
                    ._o
                    ._o_which == 7 as libc::c_int
            {
                hplus
                    += (**cur_ring.as_mut_ptr().offset(1 as libc::c_int as isize))
                        ._o
                        ._o_ac as libc::c_int;
            }
        }
        cp = (*weap)._o._o_damage;
        if hurl as libc::c_int != 0
            && (*weap)._o._o_flags as libc::c_int & 0x10 as libc::c_int != 0
            && !cur_weapon.is_null()
            && (*cur_weapon)._o._o_which == (*weap)._o._o_launch as libc::c_int
        {
            cp = (*weap)._o._o_hurldmg;
            hplus += (*cur_weapon)._o._o_hplus;
            dplus += (*cur_weapon)._o._o_dplus;
        }
        if (*weap)._o._o_type == 0xe7 as libc::c_int
            && (*weap)._o._o_which == 1 as libc::c_int
            && {
                (*weap)._o._o_ac -= 1;
                ((*weap)._o._o_ac as libc::c_int) < 0 as libc::c_int
            }
        {
            (*weap)
                ._o
                ._o_damage = b"0d0\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
            cp = (*weap)._o._o_damage;
            (*weap)._o._o_dplus = 0 as libc::c_int;
            (*weap)._o._o_hplus = (*weap)._o._o_dplus;
            (*weap)._o._o_ac = 0 as libc::c_int as libc::c_short;
        }
    }
    if cp.is_null() {
        return 0 as libc::c_int != 0;
    }
    if !((*thdef)._t._t_flags as libc::c_int & 0x4 as libc::c_int != 0 as libc::c_int) {
        hplus += 4 as libc::c_int;
    }
    def_arm = (*def).s_arm;
    if def == &mut player._t._t_stats as *mut stats {
        if !cur_armor.is_null() {
            def_arm = (*cur_armor)._o._o_ac as libc::c_int;
        }
        if !(*cur_ring.as_mut_ptr().offset(0 as libc::c_int as isize)).is_null()
            && (**cur_ring.as_mut_ptr().offset(0 as libc::c_int as isize))._o._o_which
                == 0 as libc::c_int
        {
            def_arm
                -= (**cur_ring.as_mut_ptr().offset(0 as libc::c_int as isize))._o._o_ac
                    as libc::c_int;
        }
        if !(*cur_ring.as_mut_ptr().offset(1 as libc::c_int as isize)).is_null()
            && (**cur_ring.as_mut_ptr().offset(1 as libc::c_int as isize))._o._o_which
                == 0 as libc::c_int
        {
            def_arm
                -= (**cur_ring.as_mut_ptr().offset(1 as libc::c_int as isize))._o._o_ac
                    as libc::c_int;
        }
    }
    loop {
        ndice = atoi(cp);
        cp = strchr(cp, 'd' as i32);
        if cp.is_null() {
            break;
        }
        cp = cp.offset(1);
        nsides = atoi(cp);
        if swing((*att).s_lvl, def_arm, hplus + str_plus((*att).s_str)) {
            let mut proll: libc::c_int = 0;
            proll = roll(ndice, nsides);
            damage = dplus + proll + add_dam((*att).s_str);
            if thdef == &mut player as *mut THING && max_level == 1 as libc::c_int {
                damage = (damage + 1 as libc::c_int) / 2 as libc::c_int;
            }
            if thdef == &mut player as *mut THING {
                damage *= hit_mul;
            }
            (*def).s_hpt
                -= if 0 as libc::c_int > damage { 0 as libc::c_int } else { damage };
            did_hit = 1 as libc::c_int != 0;
        }
        cp = strchr(cp, '/' as i32);
        if cp.is_null() {
            break;
        }
        cp = cp.offset(1);
    }
    return did_hit;
}
#[no_mangle]
pub unsafe extern "C" fn prname(
    mut who: *mut libc::c_char,
    mut upper: bool,
) -> *mut libc::c_char {
    *tbuf = '\0' as i32 as libc::c_char;
    if who.is_null() {
        strcpy(tbuf, you);
    } else if player._t._t_flags as libc::c_int & 0x1 as libc::c_int != 0 as libc::c_int
    {
        strcpy(tbuf, it);
    } else {
        strcpy(tbuf, b"the \0" as *const u8 as *const libc::c_char);
        strcat(tbuf, who);
    }
    if upper {
        *tbuf = ({
            let mut __res: libc::c_int = 0;
            if ::core::mem::size_of::<libc::c_char>() as libc::c_ulong
                > 1 as libc::c_int as libc::c_ulong
            {
                if 0 != 0 {
                    let mut __c: libc::c_int = *tbuf as libc::c_int;
                    __res = if __c < -(128 as libc::c_int) || __c > 255 as libc::c_int {
                        __c
                    } else {
                        *(*__ctype_toupper_loc()).offset(__c as isize)
                    };
                } else {
                    __res = toupper(*tbuf as libc::c_int);
                }
            } else {
                __res = *(*__ctype_toupper_loc()).offset(*tbuf as libc::c_int as isize);
            }
            __res
        }) as libc::c_char;
    }
    return tbuf;
}
#[no_mangle]
pub unsafe extern "C" fn hit(mut er: *mut libc::c_char, mut ee: *mut libc::c_char) {
    let mut s: *mut libc::c_char = b"\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char;
    addmsg(prname(er, 1 as libc::c_int != 0));
    match if terse as libc::c_int != 0 || expert as libc::c_int != 0 {
        1 as libc::c_int
    } else {
        rnd(4 as libc::c_int)
    } {
        0 => {
            s = b" scored an excellent hit on \0" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
        }
        1 => {
            s = b" hit \0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
        2 => {
            s = (if er.is_null() {
                b" have injured \0" as *const u8 as *const libc::c_char
            } else {
                b" has injured \0" as *const u8 as *const libc::c_char
            }) as *mut libc::c_char;
        }
        3 => {
            s = (if er.is_null() {
                b" swing and hit \0" as *const u8 as *const libc::c_char
            } else {
                b" swings and hits \0" as *const u8 as *const libc::c_char
            }) as *mut libc::c_char;
        }
        _ => {}
    }
    msg(
        b"%s%s\0" as *const u8 as *const libc::c_char,
        s,
        prname(ee, 0 as libc::c_int != 0),
    );
}
#[no_mangle]
pub unsafe extern "C" fn miss(mut er: *mut libc::c_char, mut ee: *mut libc::c_char) {
    let mut s: *mut libc::c_char = b"\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char;
    addmsg(prname(er, 1 as libc::c_int != 0));
    match if terse as libc::c_int != 0 || expert as libc::c_int != 0 {
        1 as libc::c_int
    } else {
        rnd(4 as libc::c_int)
    } {
        0 => {
            s = (if er.is_null() {
                b" swing and miss\0" as *const u8 as *const libc::c_char
            } else {
                b" swings and misses\0" as *const u8 as *const libc::c_char
            }) as *mut libc::c_char;
        }
        1 => {
            s = (if er.is_null() {
                b" miss\0" as *const u8 as *const libc::c_char
            } else {
                b" misses\0" as *const u8 as *const libc::c_char
            }) as *mut libc::c_char;
        }
        2 => {
            s = (if er.is_null() {
                b" barely miss\0" as *const u8 as *const libc::c_char
            } else {
                b" barely misses\0" as *const u8 as *const libc::c_char
            }) as *mut libc::c_char;
        }
        3 => {
            s = (if er.is_null() {
                b" don't hit\0" as *const u8 as *const libc::c_char
            } else {
                b" doesn't hit\0" as *const u8 as *const libc::c_char
            }) as *mut libc::c_char;
        }
        _ => {}
    }
    msg(
        b"%s %s\0" as *const u8 as *const libc::c_char,
        s,
        prname(ee, 0 as libc::c_int != 0),
    );
}
#[no_mangle]
pub unsafe extern "C" fn save_throw(mut which: libc::c_int, mut tp: *mut THING) -> bool {
    let mut need: libc::c_int = 0;
    need = 14 as libc::c_int + which - (*tp)._t._t_stats.s_lvl / 2 as libc::c_int;
    return roll(1 as libc::c_int, 20 as libc::c_int) >= need;
}
#[no_mangle]
pub unsafe extern "C" fn save(mut which: libc::c_int) -> bool {
    if which == 0o3 as libc::c_int {
        if !(*cur_ring.as_mut_ptr().offset(0 as libc::c_int as isize)).is_null()
            && (**cur_ring.as_mut_ptr().offset(0 as libc::c_int as isize))._o._o_which
                == 0 as libc::c_int
        {
            which
                -= (**cur_ring.as_mut_ptr().offset(0 as libc::c_int as isize))._o._o_ac
                    as libc::c_int;
        }
        if !(*cur_ring.as_mut_ptr().offset(1 as libc::c_int as isize)).is_null()
            && (**cur_ring.as_mut_ptr().offset(1 as libc::c_int as isize))._o._o_which
                == 0 as libc::c_int
        {
            which
                -= (**cur_ring.as_mut_ptr().offset(1 as libc::c_int as isize))._o._o_ac
                    as libc::c_int;
        }
    }
    return save_throw(which, &mut player);
}
#[no_mangle]
pub unsafe extern "C" fn str_plus(mut str: str_t) -> libc::c_int {
    let mut add: libc::c_int = 4 as libc::c_int;
    if str < 8 as libc::c_int as libc::c_uint {
        return str.wrapping_sub(7 as libc::c_int as libc::c_uint) as libc::c_int;
    }
    if str < 31 as libc::c_int as libc::c_uint {
        add -= 1;
    }
    if str < 21 as libc::c_int as libc::c_uint {
        add -= 1;
    }
    if str < 19 as libc::c_int as libc::c_uint {
        add -= 1;
    }
    if str < 17 as libc::c_int as libc::c_uint {
        add -= 1;
    }
    return add;
}
#[no_mangle]
pub unsafe extern "C" fn add_dam(mut str: str_t) -> libc::c_int {
    let mut add: libc::c_int = 6 as libc::c_int;
    if str < 8 as libc::c_int as libc::c_uint {
        return str.wrapping_sub(7 as libc::c_int as libc::c_uint) as libc::c_int;
    }
    if str < 31 as libc::c_int as libc::c_uint {
        add -= 1;
    }
    if str < 22 as libc::c_int as libc::c_uint {
        add -= 1;
    }
    if str < 20 as libc::c_int as libc::c_uint {
        add -= 1;
    }
    if str < 18 as libc::c_int as libc::c_uint {
        add -= 1;
    }
    if str < 17 as libc::c_int as libc::c_uint {
        add -= 1;
    }
    if str < 16 as libc::c_int as libc::c_uint {
        add -= 1;
    }
    return add;
}
#[no_mangle]
pub unsafe extern "C" fn raise_level() {
    player
        ._t
        ._t_stats
        .s_exp = *e_levels.offset((player._t._t_stats.s_lvl - 1 as libc::c_int) as isize)
        + 1 as libc::c_long;
    check_level();
}
#[no_mangle]
pub unsafe extern "C" fn thunk(
    mut weap: *mut THING,
    mut mname: *mut libc::c_char,
    mut does: *mut libc::c_char,
    mut did: *mut libc::c_char,
) {
    if (*weap)._o._o_type == 0x18 as libc::c_int {
        addmsg(
            b"the %s %s \0" as *const u8 as *const libc::c_char,
            *w_names.as_mut_ptr().offset((*weap)._o._o_which as isize),
            does,
        );
    } else {
        addmsg(b"you %s \0" as *const u8 as *const libc::c_char, did);
    }
    if player._t._t_flags as libc::c_int & 0x1 as libc::c_int != 0 as libc::c_int {
        msg(it);
    } else {
        msg(b"the %s\0" as *const u8 as *const libc::c_char, mname);
    };
}
#[no_mangle]
pub unsafe extern "C" fn remove_monster(
    mut mp: *mut coord,
    mut tp: *mut THING,
    mut waskill: bool,
) {
    let mut obj: *mut THING = 0 as *mut THING;
    let mut nexti: *mut THING = 0 as *mut THING;
    if tp.is_null() {
        return;
    }
    obj = (*tp)._t._t_pack;
    while !obj.is_null() {
        nexti = (*obj)._t._l_next;
        memmove(
            &mut (*obj)._o._o_pos as *mut coord as *mut libc::c_void,
            &mut (*tp)._t._t_pos as *mut coord as *const libc::c_void,
            ::core::mem::size_of::<coord>() as libc::c_ulong,
        );
        list_detach(&mut (*tp)._t._t_pack, obj);
        if waskill {
            fall(obj, 0 as libc::c_int != 0);
        } else {
            discard(obj);
        }
        obj = nexti;
    }
    if *_level.offset(INDEX((*mp).y, (*mp).x) as isize) as libc::c_int
        == 0xb1 as libc::c_int
    {
        set_attr(14 as libc::c_int);
    }
    if (*tp)._t._t_oldch as libc::c_int == 0xfa as libc::c_int
        && !cansee((*mp).y, (*mp).x)
    {
        cur_mvaddch((*mp).y, (*mp).x, ' ' as i32 as byte);
    } else if (*tp)._t._t_oldch as libc::c_int != '@' as i32 {
        cur_mvaddch((*mp).y, (*mp).x, (*tp)._t._t_oldch);
    }
    set_attr(0 as libc::c_int);
    list_detach(&mut mlist, tp);
    discard(tp);
}
#[no_mangle]
pub unsafe extern "C" fn is_magic(mut obj: *mut THING) -> bool {
    match (*obj)._o._o_type {
        8 => {
            return (*obj)._o._o_ac as libc::c_int
                != *a_class.as_mut_ptr().offset((*obj)._o._o_which as isize);
        }
        24 => {
            return (*obj)._o._o_hplus != 0 as libc::c_int
                || (*obj)._o._o_dplus != 0 as libc::c_int;
        }
        173 | 13 | 231 | 9 | 12 => return 1 as libc::c_int != 0,
        _ => {}
    }
    return 0 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn killed(mut tp: *mut THING, mut pr: bool) {
    player._t._t_stats.s_exp += (*tp)._t._t_stats.s_exp;
    match (*tp)._t._t_type as libc::c_int {
        70 => {
            player
                ._t
                ._t_flags = (player._t._t_flags as libc::c_int & !(0x80 as libc::c_int))
                as libc::c_short;
            f_restor();
        }
        76 => {
            let mut gold: *mut THING = 0 as *mut THING;
            gold = new_item();
            if gold.is_null() {
                return;
            }
            (*gold)._o._o_type = 0xf as libc::c_int;
            (*gold)
                ._o
                ._o_ac = (rnd(50 as libc::c_int + 10 as libc::c_int * level)
                + 2 as libc::c_int) as libc::c_short;
            if save(0o3 as libc::c_int) {
                (*gold)
                    ._o
                    ._o_ac = ((*gold)._o._o_ac as libc::c_int
                    + (rnd(50 as libc::c_int + 10 as libc::c_int * level)
                        + 2 as libc::c_int
                        + (rnd(50 as libc::c_int + 10 as libc::c_int * level)
                            + 2 as libc::c_int)
                        + (rnd(50 as libc::c_int + 10 as libc::c_int * level)
                            + 2 as libc::c_int)
                        + (rnd(50 as libc::c_int + 10 as libc::c_int * level)
                            + 2 as libc::c_int))) as libc::c_short;
            }
            list_attach(&mut (*tp)._t._t_pack, gold);
        }
        _ => {}
    }
    remove_monster(&mut (*tp)._t._t_pos, tp, 1 as libc::c_int != 0);
    if pr {
        addmsg(b"you have defeated \0" as *const u8 as *const libc::c_char);
        if player._t._t_flags as libc::c_int & 0x1 as libc::c_int != 0 as libc::c_int {
            msg(it);
        } else {
            msg(
                b"the %s\0" as *const u8 as *const libc::c_char,
                (*monsters
                    .as_mut_ptr()
                    .offset(((*tp)._t._t_type as libc::c_int - 'A' as i32) as isize))
                    .m_name,
            );
        }
    }
    check_level();
}
