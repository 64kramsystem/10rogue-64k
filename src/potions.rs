use ::libc;
extern "C" {
    static mut p_guess: [*mut libc::c_char; 0];
    static mut p_know: [bool; 0];
    static mut inpack: libc::c_int;
    static mut no_command: libc::c_int;
    static mut cur_ring: [*mut THING; 0];
    static mut cur_weapon: *mut THING;
    static mut lvl_obj: *mut THING;
    static mut mlist: *mut THING;
    static mut player: THING;
    fn see_monst(mp: *mut THING) -> bool;
    fn fuse(func: Option::<unsafe extern "C" fn() -> ()>, time: libc::c_int);
    fn lengthen(func: Option::<unsafe extern "C" fn() -> ()>, xtime: libc::c_int);
    fn unconfuse();
    fn unsee();
    fn sight();
    fn is_magic(obj: *mut THING) -> bool;
    fn raise_level();
    fn msg(fmt: *const libc::c_char, _: ...);
    static mut max_stats: stats;
    static mut monsters: [monster; 0];
    static mut fruit: [libc::c_char; 0];
    fn status();
    fn goodch(obj: *mut THING) -> libc::c_char;
    fn spread(nm: libc::c_int) -> libc::c_int;
    fn noterse(str: *mut libc::c_char) -> *mut libc::c_char;
    fn get_item(purpose: *mut libc::c_char, type_0: libc::c_int) -> *mut THING;
    fn add_haste(potion: bool) -> bool;
    fn call_it(know: bool, guess: *mut *mut libc::c_char);
    fn add_str(sp: *mut str_t, amt: libc::c_int);
    fn chg_str(amt: libc::c_int);
    fn look(wakeup: bool);
    fn roll(number: libc::c_int, sides: libc::c_int) -> libc::c_int;
    fn rnd(range: libc::c_int) -> libc::c_int;
    fn list_detach(list: *mut *mut THING, item: *mut THING);
    fn discard(item: *mut THING) -> libc::c_int;
    fn cur_addch(chr: byte);
    fn set_attr(bute: libc::c_int);
    fn cur_mvaddch(r: libc::c_int, c: libc::c_int, chr: byte);
    fn cur_move(row: libc::c_int, col: libc::c_int) -> libc::c_int;
    fn cur_inch() -> byte;
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
unsafe extern "C" fn turn_see_off() {
    turn_see(1 as libc::c_int != 0);
}
#[no_mangle]
pub unsafe extern "C" fn quaff() {
    let mut obj: *mut THING = 0 as *mut THING;
    let mut th: *mut THING = 0 as *mut THING;
    let mut discardit: bool = 0 as libc::c_int != 0;
    obj = get_item(
        b"quaff\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0xad as libc::c_int,
    );
    if obj.is_null() {
        return;
    }
    if (*obj)._o._o_type != 0xad as libc::c_int {
        msg(
            b"yuk! Why would you want to drink that?\0" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    if obj == cur_weapon {
        cur_weapon = 0 as *mut THING;
    }
    let mut current_block_106: u64;
    match (*obj)._o._o_which {
        0 => {
            *p_know
                .as_mut_ptr()
                .offset(0 as libc::c_int as isize) = 1 as libc::c_int != 0;
            if !(player._t._t_flags as libc::c_int & 0x100 as libc::c_int
                != 0 as libc::c_int)
            {
                if player._t._t_flags as libc::c_int & 0x100 as libc::c_int
                    != 0 as libc::c_int
                {
                    lengthen(
                        ::core::mem::transmute::<
                            Option::<unsafe extern "C" fn() -> ()>,
                            Option::<unsafe extern "C" fn() -> ()>,
                        >(Some(unconfuse as unsafe extern "C" fn() -> ())),
                        rnd(8 as libc::c_int) + spread(20 as libc::c_int),
                    );
                } else {
                    fuse(
                        ::core::mem::transmute::<
                            Option::<unsafe extern "C" fn() -> ()>,
                            Option::<unsafe extern "C" fn() -> ()>,
                        >(Some(unconfuse as unsafe extern "C" fn() -> ())),
                        rnd(8 as libc::c_int) + spread(20 as libc::c_int),
                    );
                }
                player
                    ._t
                    ._t_flags = (player._t._t_flags as libc::c_int
                    | 0x100 as libc::c_int) as libc::c_short;
                msg(
                    b"wait, what's going on? Huh? What? Who?\0" as *const u8
                        as *const libc::c_char,
                );
            }
        }
        2 => {
            let mut sick: *mut libc::c_char = b"you feel %s sick.\0" as *const u8
                as *const libc::c_char as *mut libc::c_char;
            *p_know
                .as_mut_ptr()
                .offset(2 as libc::c_int as isize) = 1 as libc::c_int != 0;
            if !(!(*cur_ring.as_mut_ptr().offset(0 as libc::c_int as isize)).is_null()
                && (**cur_ring.as_mut_ptr().offset(0 as libc::c_int as isize))
                    ._o
                    ._o_which == 2 as libc::c_int
                || !(*cur_ring.as_mut_ptr().offset(1 as libc::c_int as isize)).is_null()
                    && (**cur_ring.as_mut_ptr().offset(1 as libc::c_int as isize))
                        ._o
                        ._o_which == 2 as libc::c_int)
            {
                chg_str(-(rnd(3 as libc::c_int) + 1 as libc::c_int));
                msg(sick, b"very\0" as *const u8 as *const libc::c_char);
            } else {
                msg(sick, b"momentarily\0" as *const u8 as *const libc::c_char);
            }
        }
        5 => {
            *p_know
                .as_mut_ptr()
                .offset(5 as libc::c_int as isize) = 1 as libc::c_int != 0;
            player._t._t_stats.s_hpt += roll(player._t._t_stats.s_lvl, 4 as libc::c_int);
            if player._t._t_stats.s_hpt > player._t._t_stats.s_maxhp {
                player._t._t_stats.s_maxhp += 1;
                player._t._t_stats.s_hpt = player._t._t_stats.s_maxhp;
            }
            sight();
            msg(b"you begin to feel better\0" as *const u8 as *const libc::c_char);
        }
        3 => {
            *p_know
                .as_mut_ptr()
                .offset(3 as libc::c_int as isize) = 1 as libc::c_int != 0;
            chg_str(1 as libc::c_int);
            msg(
                b"you feel stronger. What bulging muscles!\0" as *const u8
                    as *const libc::c_char,
            );
        }
        6 => {
            fuse(
                ::core::mem::transmute::<
                    Option::<unsafe extern "C" fn() -> ()>,
                    Option::<unsafe extern "C" fn() -> ()>,
                >(Some(turn_see_off as unsafe extern "C" fn() -> ())),
                spread(20 as libc::c_int),
            );
            if mlist.is_null() {
                msg(
                    b"you have a strange feeling%s.\0" as *const u8
                        as *const libc::c_char,
                    noterse(
                        b" for a moment\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    ),
                );
            } else {
                if turn_see(0 as libc::c_int != 0) {
                    *p_know
                        .as_mut_ptr()
                        .offset(6 as libc::c_int as isize) = 1 as libc::c_int != 0;
                }
                msg(b"\0" as *const u8 as *const libc::c_char);
            }
        }
        7 => {
            if !lvl_obj.is_null() {
                let mut tp: *mut THING = 0 as *mut THING;
                let mut show: bool = false;
                show = 0 as libc::c_int != 0;
                tp = lvl_obj;
                while !tp.is_null() {
                    if is_magic(tp) {
                        show = 1 as libc::c_int != 0;
                        cur_mvaddch(
                            (*tp)._o._o_pos.y,
                            (*tp)._o._o_pos.x,
                            goodch(tp) as byte,
                        );
                        *p_know
                            .as_mut_ptr()
                            .offset(7 as libc::c_int as isize) = 1 as libc::c_int != 0;
                    }
                    tp = (*tp)._t._l_next;
                }
                th = mlist;
                while !th.is_null() {
                    tp = (*th)._t._t_pack;
                    while !tp.is_null() {
                        if is_magic(tp) {
                            show = 1 as libc::c_int != 0;
                            cur_mvaddch(
                                (*th)._t._t_pos.y,
                                (*th)._t._t_pos.x,
                                '$' as i32 as byte,
                            );
                            *p_know
                                .as_mut_ptr()
                                .offset(7 as libc::c_int as isize) = 1 as libc::c_int != 0;
                        }
                        tp = (*tp)._t._l_next;
                    }
                    th = (*th)._t._l_next;
                }
                if show {
                    msg(
                        b"You sense the presence of magic.\0" as *const u8
                            as *const libc::c_char,
                    );
                    current_block_106 = 14141370668937312244;
                } else {
                    current_block_106 = 12829669402821218572;
                }
            } else {
                current_block_106 = 12829669402821218572;
            }
            match current_block_106 {
                14141370668937312244 => {}
                _ => {
                    msg(
                        b"you have a strange feeling for a moment%s.\0" as *const u8
                            as *const libc::c_char,
                        noterse(
                            b", then it passes\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                        ),
                    );
                }
            }
        }
        1 => {
            *p_know
                .as_mut_ptr()
                .offset(1 as libc::c_int as isize) = 1 as libc::c_int != 0;
            no_command = spread(2 as libc::c_int);
            player
                ._t
                ._t_flags = (player._t._t_flags as libc::c_int & !(0x4 as libc::c_int))
                as libc::c_short;
            msg(b"you can't move\0" as *const u8 as *const libc::c_char);
        }
        4 => {
            if !(player._t._t_flags as libc::c_int & 0x800 as libc::c_int
                != 0 as libc::c_int)
            {
                fuse(
                    ::core::mem::transmute::<
                        Option::<unsafe extern "C" fn() -> ()>,
                        Option::<unsafe extern "C" fn() -> ()>,
                    >(Some(unsee as unsafe extern "C" fn() -> ())),
                    spread(300 as libc::c_int),
                );
                look(0 as libc::c_int != 0);
                invis_on();
            }
            sight();
            msg(
                b"this potion tastes like %s juice\0" as *const u8
                    as *const libc::c_char,
                fruit.as_mut_ptr(),
            );
        }
        8 => {
            *p_know
                .as_mut_ptr()
                .offset(8 as libc::c_int as isize) = 1 as libc::c_int != 0;
            msg(
                b"you suddenly feel much more skillful\0" as *const u8
                    as *const libc::c_char,
            );
            raise_level();
        }
        9 => {
            *p_know
                .as_mut_ptr()
                .offset(9 as libc::c_int as isize) = 1 as libc::c_int != 0;
            player._t._t_stats.s_hpt += roll(player._t._t_stats.s_lvl, 8 as libc::c_int);
            if player._t._t_stats.s_hpt > player._t._t_stats.s_maxhp {
                if player._t._t_stats.s_hpt
                    > player._t._t_stats.s_maxhp + player._t._t_stats.s_lvl
                        + 1 as libc::c_int
                {
                    player._t._t_stats.s_maxhp += 1;
                }
                player._t._t_stats.s_maxhp += 1;
                player._t._t_stats.s_hpt = player._t._t_stats.s_maxhp;
            }
            sight();
            msg(b"you begin to feel much better\0" as *const u8 as *const libc::c_char);
        }
        10 => {
            *p_know
                .as_mut_ptr()
                .offset(10 as libc::c_int as isize) = 1 as libc::c_int != 0;
            if add_haste(1 as libc::c_int != 0) {
                msg(
                    b"you feel yourself moving much faster\0" as *const u8
                        as *const libc::c_char,
                );
            }
        }
        11 => {
            if !(*cur_ring.as_mut_ptr().offset(0 as libc::c_int as isize)).is_null()
                && (**cur_ring.as_mut_ptr().offset(0 as libc::c_int as isize))
                    ._o
                    ._o_which == 1 as libc::c_int
            {
                add_str(
                    &mut player._t._t_stats.s_str,
                    -((**cur_ring.as_mut_ptr().offset(0 as libc::c_int as isize))
                        ._o
                        ._o_ac as libc::c_int),
                );
            }
            if !(*cur_ring.as_mut_ptr().offset(1 as libc::c_int as isize)).is_null()
                && (**cur_ring.as_mut_ptr().offset(1 as libc::c_int as isize))
                    ._o
                    ._o_which == 1 as libc::c_int
            {
                add_str(
                    &mut player._t._t_stats.s_str,
                    -((**cur_ring.as_mut_ptr().offset(1 as libc::c_int as isize))
                        ._o
                        ._o_ac as libc::c_int),
                );
            }
            if player._t._t_stats.s_str < max_stats.s_str {
                player._t._t_stats.s_str = max_stats.s_str;
            }
            if !(*cur_ring.as_mut_ptr().offset(0 as libc::c_int as isize)).is_null()
                && (**cur_ring.as_mut_ptr().offset(0 as libc::c_int as isize))
                    ._o
                    ._o_which == 1 as libc::c_int
            {
                add_str(
                    &mut player._t._t_stats.s_str,
                    (**cur_ring.as_mut_ptr().offset(0 as libc::c_int as isize))._o._o_ac
                        as libc::c_int,
                );
            }
            if !(*cur_ring.as_mut_ptr().offset(1 as libc::c_int as isize)).is_null()
                && (**cur_ring.as_mut_ptr().offset(1 as libc::c_int as isize))
                    ._o
                    ._o_which == 1 as libc::c_int
            {
                add_str(
                    &mut player._t._t_stats.s_str,
                    (**cur_ring.as_mut_ptr().offset(1 as libc::c_int as isize))._o._o_ac
                        as libc::c_int,
                );
            }
            msg(
                b"%syou feel warm all over\0" as *const u8 as *const libc::c_char,
                noterse(
                    b"hey, this tastes great.  It makes \0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                ),
            );
        }
        12 => {
            *p_know
                .as_mut_ptr()
                .offset(12 as libc::c_int as isize) = 1 as libc::c_int != 0;
            if !(player._t._t_flags as libc::c_int & 0x1 as libc::c_int
                != 0 as libc::c_int)
            {
                player
                    ._t
                    ._t_flags = (player._t._t_flags as libc::c_int | 0x1 as libc::c_int)
                    as libc::c_short;
                fuse(
                    ::core::mem::transmute::<
                        Option::<unsafe extern "C" fn() -> ()>,
                        Option::<unsafe extern "C" fn() -> ()>,
                    >(Some(sight as unsafe extern "C" fn() -> ())),
                    spread(300 as libc::c_int),
                );
                look(0 as libc::c_int != 0);
            }
            msg(
                b"a cloak of darkness falls around you\0" as *const u8
                    as *const libc::c_char,
            );
        }
        13 => {
            msg(
                b"this potion tastes extremely dull\0" as *const u8
                    as *const libc::c_char,
            );
        }
        _ => {
            msg(b"what an odd tasting potion!\0" as *const u8 as *const libc::c_char);
            return;
        }
    }
    status();
    inpack -= 1;
    if (*obj)._o._o_count > 1 as libc::c_int {
        (*obj)._o._o_count -= 1;
    } else {
        list_detach(&mut player._t._t_pack, obj);
        discardit = 1 as libc::c_int != 0;
    }
    call_it(
        *p_know.as_mut_ptr().offset((*obj)._o._o_which as isize),
        &mut *p_guess.as_mut_ptr().offset((*obj)._o._o_which as isize),
    );
    if discardit {
        discard(obj);
    }
}
#[no_mangle]
pub unsafe extern "C" fn invis_on() {
    let mut th: *mut THING = 0 as *mut THING;
    player
        ._t
        ._t_flags = (player._t._t_flags as libc::c_int | 0x800 as libc::c_int)
        as libc::c_short;
    th = mlist;
    while !th.is_null() {
        if (*th)._t._t_flags as libc::c_int & 0x10 as libc::c_int != 0 as libc::c_int
            && see_monst(th) as libc::c_int != 0
        {
            cur_mvaddch((*th)._t._t_pos.y, (*th)._t._t_pos.x, (*th)._t._t_disguise);
        }
        th = (*th)._t._l_next;
    }
}
#[no_mangle]
pub unsafe extern "C" fn turn_see(mut turn_off: bool) -> bool {
    let mut mp: *mut THING = 0 as *mut THING;
    let mut can_see: bool = false;
    let mut add_new: bool = false;
    let mut was_there: byte = cur_inch();
    add_new = 0 as libc::c_int != 0;
    mp = mlist;
    while !mp.is_null() {
        cur_move((*mp)._t._t_pos.y, (*mp)._t._t_pos.x);
        can_see = see_monst(mp) as libc::c_int != 0
            || {
                was_there = cur_inch();
                was_there as libc::c_int == (*mp)._t._t_type as libc::c_int
            };
        if turn_off {
            if !see_monst(mp) && (*mp)._t._t_oldch as libc::c_int != '@' as i32 {
                cur_addch((*mp)._t._t_oldch);
            }
        } else {
            if !can_see {
                set_attr(14 as libc::c_int);
                (*mp)._t._t_oldch = was_there;
            }
            cur_addch((*mp)._t._t_type as byte);
            if !can_see {
                set_attr(0 as libc::c_int);
                add_new = 1 as libc::c_int != 0;
            }
        }
        mp = (*mp)._t._l_next;
    }
    player
        ._t
        ._t_flags = (player._t._t_flags as libc::c_int | 0x2 as libc::c_int)
        as libc::c_short;
    if turn_off {
        player
            ._t
            ._t_flags = (player._t._t_flags as libc::c_int & !(0x2 as libc::c_int))
            as libc::c_short;
    }
    return add_new;
}
#[no_mangle]
pub unsafe extern "C" fn th_effect(mut obj: *mut THING, mut tp: *mut THING) {
    match (*obj)._o._o_which {
        0 | 12 => {
            (*tp)
                ._t
                ._t_flags = ((*tp)._t._t_flags as libc::c_int | 0x100 as libc::c_int)
                as libc::c_short;
            msg(
                b"the %s appears confused\0" as *const u8 as *const libc::c_char,
                (*monsters
                    .as_mut_ptr()
                    .offset(((*tp)._t._t_type as libc::c_int - 'A' as i32) as isize))
                    .m_name,
            );
        }
        1 => {
            (*tp)
                ._t
                ._t_flags = ((*tp)._t._t_flags as libc::c_int & !(0x4 as libc::c_int))
                as libc::c_short;
            (*tp)
                ._t
                ._t_flags = ((*tp)._t._t_flags as libc::c_int | 0x80 as libc::c_int)
                as libc::c_short;
        }
        5 | 9 => {
            (*tp)._t._t_stats.s_hpt += rnd(8 as libc::c_int);
            if (*tp)._t._t_stats.s_hpt > (*tp)._t._t_stats.s_maxhp {
                (*tp)._t._t_stats.s_maxhp += 1;
                (*tp)._t._t_stats.s_hpt = (*tp)._t._t_stats.s_maxhp;
            }
        }
        8 => {
            (*tp)._t._t_stats.s_hpt += 8 as libc::c_int;
            (*tp)._t._t_stats.s_maxhp += 8 as libc::c_int;
            (*tp)._t._t_stats.s_lvl += 1;
        }
        10 => {
            (*tp)
                ._t
                ._t_flags = ((*tp)._t._t_flags as libc::c_int | 0x4000 as libc::c_int)
                as libc::c_short;
        }
        _ => {}
    }
    msg(b"the flask shatters.\0" as *const u8 as *const libc::c_char);
}
