use ::libc;
extern "C" {
    fn memmove(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
        -> *mut libc::c_void;
    fn flush_type();
    static mut s_guess: [*mut libc::c_char; 0];
    static mut p_guess: [*mut libc::c_char; 0];
    static mut r_guess: [*mut libc::c_char; 0];
    static mut ws_guess: [*mut libc::c_char; 0];
    static mut running: bool;
    static mut p_know: [bool; 0];
    static mut r_know: [bool; 0];
    static mut s_know: [bool; 0];
    static mut ws_know: [bool; 0];
    static mut count: libc::c_int;
    static mut mpos: libc::c_int;
    static mut player: THING;
    static mut rooms: [room; 0];
    fn fuse(func: Option<unsafe extern "C" fn() -> ()>, time: libc::c_int);
    fn lengthen(func: Option<unsafe extern "C" fn() -> ()>, xtime: libc::c_int);
    fn unconfuse();
    fn msg(fmt: *const libc::c_char, _: ...);
    fn rnd(range: libc::c_int) -> libc::c_int;
    fn look(wakeup: bool);
    static mut _level: *mut byte;
    fn step_ok(ch: byte) -> bool;
    fn winat(y: libc::c_int, x: libc::c_int) -> byte;
    fn INDEX(y: libc::c_int, x: libc::c_int) -> libc::c_int;
    fn f_restor();
    static mut no_move: libc::c_int;
    fn rnd_room() -> libc::c_int;
    fn rnd_pos(rp: *mut room, cp: *mut coord);
    fn enter_room(cp: *mut coord);
    fn leave_room(cp: *mut coord);
    fn get_item(purpose: *mut libc::c_char, type_0: libc::c_int) -> *mut THING;
    fn inv_name(obj: *mut THING, drop_0: bool) -> *mut libc::c_char;
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
pub unsafe extern "C" fn whatis() {
    let mut obj: *mut THING = 0 as *mut THING;
    if (player._t._t_pack).is_null() {
        msg(
            b"You don't have anything in your pack to identify\0" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    loop {
        obj = get_item(
            b"identify\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            0 as libc::c_int,
        );
        if !obj.is_null() {
            break;
        }
        msg(b"You must identify something\0" as *const u8 as *const libc::c_char);
        msg(b" \0" as *const u8 as *const libc::c_char);
        mpos = 0 as libc::c_int;
    }
    match (*obj)._o._o_type {
        13 => {
            *s_know.as_mut_ptr().offset((*obj)._o._o_which as isize) = 1 as libc::c_int != 0;
            **s_guess.as_mut_ptr().offset((*obj)._o._o_which as isize) =
                '\0' as i32 as libc::c_char;
        }
        173 => {
            *p_know.as_mut_ptr().offset((*obj)._o._o_which as isize) = 1 as libc::c_int != 0;
            **p_guess.as_mut_ptr().offset((*obj)._o._o_which as isize) =
                '\0' as i32 as libc::c_char;
        }
        231 => {
            *ws_know.as_mut_ptr().offset((*obj)._o._o_which as isize) = 1 as libc::c_int != 0;
            (*obj)._o._o_flags =
                ((*obj)._o._o_flags as libc::c_int | 0x2 as libc::c_int) as libc::c_short;
            **ws_guess.as_mut_ptr().offset((*obj)._o._o_which as isize) =
                '\0' as i32 as libc::c_char;
        }
        24 | 8 => {
            (*obj)._o._o_flags =
                ((*obj)._o._o_flags as libc::c_int | 0x2 as libc::c_int) as libc::c_short;
        }
        9 => {
            *r_know.as_mut_ptr().offset((*obj)._o._o_which as isize) = 1 as libc::c_int != 0;
            (*obj)._o._o_flags =
                ((*obj)._o._o_flags as libc::c_int | 0x2 as libc::c_int) as libc::c_short;
            **r_guess.as_mut_ptr().offset((*obj)._o._o_which as isize) =
                '\0' as i32 as libc::c_char;
        }
        _ => {}
    }
    if (*obj)._o._o_enemy != 0 {
        (*obj)._o._o_flags =
            ((*obj)._o._o_flags as libc::c_int | 0x40 as libc::c_int) as libc::c_short;
    }
    msg(inv_name(obj, 0 as libc::c_int != 0));
}
#[no_mangle]
pub unsafe extern "C" fn teleport() -> libc::c_int {
    let mut rm: libc::c_int = 0;
    let mut c: coord = coord { x: 0, y: 0 };
    cur_mvaddch(
        player._t._t_pos.y,
        player._t._t_pos.x,
        *_level.offset(INDEX(player._t._t_pos.y, player._t._t_pos.x) as isize),
    );
    loop {
        rm = rnd_room();
        rnd_pos(&mut *rooms.as_mut_ptr().offset(rm as isize), &mut c);
        if step_ok(winat(c.y, c.x)) {
            break;
        }
    }
    if &mut *rooms.as_mut_ptr().offset(rm as isize) as *mut room != player._t._t_room {
        leave_room(&mut player._t._t_pos);
        memmove(
            &mut player._t._t_pos as *mut coord as *mut libc::c_void,
            &mut c as *mut coord as *const libc::c_void,
            ::core::mem::size_of::<coord>() as libc::c_ulong,
        );
        enter_room(&mut player._t._t_pos);
    } else {
        memmove(
            &mut player._t._t_pos as *mut coord as *mut libc::c_void,
            &mut c as *mut coord as *const libc::c_void,
            ::core::mem::size_of::<coord>() as libc::c_ulong,
        );
        look(1 as libc::c_int != 0);
    }
    cur_mvaddch(
        player._t._t_pos.y,
        player._t._t_pos.x,
        0x1 as libc::c_int as byte,
    );
    if player._t._t_flags as libc::c_int & 0x80 as libc::c_int != 0 as libc::c_int {
        player._t._t_flags =
            (player._t._t_flags as libc::c_int & !(0x80 as libc::c_int)) as libc::c_short;
        f_restor();
    }
    no_move = 0 as libc::c_int;
    count = 0 as libc::c_int;
    running = 0 as libc::c_int != 0;
    flush_type();
    if player._t._t_flags as libc::c_int & 0x100 as libc::c_int != 0 as libc::c_int {
        lengthen(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn() -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(unconfuse as unsafe extern "C" fn() -> ())),
            rnd(4 as libc::c_int) + 2 as libc::c_int,
        );
    } else {
        fuse(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn() -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(unconfuse as unsafe extern "C" fn() -> ())),
            rnd(4 as libc::c_int) + 2 as libc::c_int,
        );
    }
    player._t._t_flags =
        (player._t._t_flags as libc::c_int | 0x100 as libc::c_int) as libc::c_short;
    return rm;
}
