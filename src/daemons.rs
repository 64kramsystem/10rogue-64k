use ::libc;
extern "C" {
    static mut running: bool;
    static mut terse: bool;
    static mut count: libc::c_int;
    static mut food_left: libc::c_int;
    static mut hungry_state: libc::c_int;
    static mut no_command: libc::c_int;
    static mut quiet: libc::c_int;
    static mut cur_ring: [*mut THING; 0];
    static mut mlist: *mut THING;
    static mut player: THING;
    fn see_monst(mp: *mut THING) -> bool;
    fn start_daemon(func: Option<unsafe extern "C" fn() -> ()>);
    fn fuse(func: Option<unsafe extern "C" fn() -> ()>, time: libc::c_int);
    fn extinguish(func: Option<unsafe extern "C" fn() -> ()>);
    fn rnd(range: libc::c_int) -> libc::c_int;
    fn spread(nm: libc::c_int) -> libc::c_int;
    fn wanderer();
    fn roll(number: libc::c_int, sides: libc::c_int) -> libc::c_int;
    fn msg(fmt: *const libc::c_char, _: ...);
    fn enter_room(cp: *mut coord);
    // Rust port: Fixed missing parameter
    fn ring_eat(p0: libc::c_int) -> libc::c_int;
    fn noterse(str: *mut libc::c_char) -> *mut libc::c_char;
    fn death(monst: libc::c_char);
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
#[no_mangle]
pub unsafe extern "C" fn doctor() {
    let mut lv: libc::c_int = 0;
    let mut ohp: libc::c_int = 0;
    lv = player._t._t_stats.s_lvl;
    ohp = player._t._t_stats.s_hpt;
    quiet += 1;
    if lv < 8 as libc::c_int {
        if quiet + (lv << 1 as libc::c_int) > 20 as libc::c_int {
            player._t._t_stats.s_hpt += 1;
        }
    } else if quiet >= 3 as libc::c_int {
        player._t._t_stats.s_hpt += rnd(lv - 7 as libc::c_int) + 1 as libc::c_int;
    }
    if !(*cur_ring.as_mut_ptr().offset(0 as libc::c_int as isize)).is_null()
        && (**cur_ring.as_mut_ptr().offset(0 as libc::c_int as isize))
            ._o
            ._o_which
            == 9 as libc::c_int
    {
        player._t._t_stats.s_hpt += 1;
    }
    if !(*cur_ring.as_mut_ptr().offset(1 as libc::c_int as isize)).is_null()
        && (**cur_ring.as_mut_ptr().offset(1 as libc::c_int as isize))
            ._o
            ._o_which
            == 9 as libc::c_int
    {
        player._t._t_stats.s_hpt += 1;
    }
    if ohp != player._t._t_stats.s_hpt {
        if player._t._t_stats.s_hpt > player._t._t_stats.s_maxhp {
            player._t._t_stats.s_hpt = player._t._t_stats.s_maxhp;
        }
        quiet = 0 as libc::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn swander() {
    start_daemon(::core::mem::transmute::<
        Option<unsafe extern "C" fn() -> ()>,
        Option<unsafe extern "C" fn() -> ()>,
    >(Some(rollwand as unsafe extern "C" fn() -> ())));
}
#[no_mangle]
pub unsafe extern "C" fn rollwand() {
    static mut between: libc::c_int = 0 as libc::c_int;
    between += 1;
    if between >= 3 as libc::c_int + rnd(3 as libc::c_int) {
        if roll(1 as libc::c_int, 6 as libc::c_int) == 4 as libc::c_int {
            wanderer();
            extinguish(::core::mem::transmute::<
                Option<unsafe extern "C" fn() -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                rollwand as unsafe extern "C" fn() -> (),
            )));
            fuse(
                ::core::mem::transmute::<
                    Option<unsafe extern "C" fn() -> ()>,
                    Option<unsafe extern "C" fn() -> ()>,
                >(Some(swander as unsafe extern "C" fn() -> ())),
                spread(70 as libc::c_int),
            );
        }
        between = 0 as libc::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn unconfuse() {
    player._t._t_flags =
        (player._t._t_flags as libc::c_int & !(0x100 as libc::c_int)) as libc::c_short;
    msg(b"you feel less confused now\0" as *const u8 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn unsee() {
    let mut th: *mut THING = 0 as *mut THING;
    th = mlist;
    while !th.is_null() {
        if (*th)._t._t_flags as libc::c_int & 0x10 as libc::c_int != 0 as libc::c_int
            && see_monst(th) as libc::c_int != 0
            && (*th)._t._t_oldch as libc::c_int != '@' as i32
        {
            cur_mvaddch((*th)._t._t_pos.y, (*th)._t._t_pos.x, (*th)._t._t_oldch);
        }
        th = (*th)._t._l_next;
    }
    player._t._t_flags =
        (player._t._t_flags as libc::c_int & !(0x800 as libc::c_int)) as libc::c_short;
}
#[no_mangle]
pub unsafe extern "C" fn sight() {
    if player._t._t_flags as libc::c_int & 0x1 as libc::c_int != 0 as libc::c_int {
        extinguish(::core::mem::transmute::<
            Option<unsafe extern "C" fn() -> ()>,
            Option<unsafe extern "C" fn() -> ()>,
        >(Some(sight as unsafe extern "C" fn() -> ())));
        player._t._t_flags =
            (player._t._t_flags as libc::c_int & !(0x1 as libc::c_int)) as libc::c_short;
        if (*player._t._t_room).r_flags as libc::c_int & 0x2 as libc::c_int == 0 {
            enter_room(&mut player._t._t_pos);
        }
        msg(b"the veil of darkness lifts\0" as *const u8 as *const libc::c_char);
    }
}
#[no_mangle]
pub unsafe extern "C" fn nohaste() {
    player._t._t_flags =
        (player._t._t_flags as libc::c_int & !(0x4000 as libc::c_int)) as libc::c_short;
    msg(b"you feel yourself slowing down\0" as *const u8 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn stomach() {
    let mut oldfood: libc::c_int = 0;
    let mut deltafood: libc::c_int = 0;
    if food_left <= 0 as libc::c_int {
        let fresh0 = food_left;
        food_left = food_left - 1;
        if fresh0 < -(850 as libc::c_int) {
            death('s' as i32 as libc::c_char);
        }
        if no_command != 0 || rnd(5 as libc::c_int) != 0 as libc::c_int {
            return;
        }
        no_command += rnd(8 as libc::c_int) + 4 as libc::c_int;
        player._t._t_flags =
            (player._t._t_flags as libc::c_int & !(0x4 as libc::c_int)) as libc::c_short;
        running = 0 as libc::c_int != 0;
        count = 0 as libc::c_int;
        hungry_state = 3 as libc::c_int;
        msg(
            b"%syou faint from lack of food\0" as *const u8 as *const libc::c_char,
            noterse(
                b"you feel very weak. \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ),
        );
    } else {
        oldfood = food_left;
        deltafood = ring_eat(0 as libc::c_int) + ring_eat(1 as libc::c_int) + 1 as libc::c_int;
        if terse {
            deltafood *= 2 as libc::c_int;
        }
        food_left -= deltafood;
        if food_left < 150 as libc::c_int && oldfood >= 150 as libc::c_int {
            hungry_state = 2 as libc::c_int;
            msg(b"you are starting to feel weak\0" as *const u8 as *const libc::c_char);
        } else if food_left < 2 as libc::c_int * 150 as libc::c_int
            && oldfood >= 2 as libc::c_int * 150 as libc::c_int
        {
            hungry_state = 1 as libc::c_int;
            msg(b"you are starting to get hungry\0" as *const u8 as *const libc::c_char);
        }
    };
}
