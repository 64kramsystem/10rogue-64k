use ::libc;
extern "C" {
    static mut player: THING;
    static mut _level: *mut byte;
    fn start_run(runner: *mut coord);
    fn cansee(y: libc::c_int, x: libc::c_int) -> bool;
    fn msg(fmt: *const libc::c_char, _: ...);
    fn new_item() -> *mut THING;
    fn rnd(range: libc::c_int) -> libc::c_int;
    fn find_obj(y: libc::c_int, x: libc::c_int) -> *mut THING;
    fn step_ok(ch: byte) -> bool;
    fn offmap(y: libc::c_int, x: libc::c_int) -> bool;
    fn winat(y: libc::c_int, x: libc::c_int) -> byte;
    fn INDEX(y: libc::c_int, x: libc::c_int) -> libc::c_int;
    fn new_monster(tp: *mut THING, type_0: byte, cp: *mut coord);
    fn moat(my: libc::c_int, mx: libc::c_int) -> *mut THING;
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
static mut slimy: coord = coord { x: 0, y: 0 };
#[no_mangle]
pub unsafe extern "C" fn slime_split(mut tp: *mut THING) {
    let mut nslime: *mut THING = 0 as *mut THING;
    if !new_slime(tp) || {
        nslime = new_item();
        nslime.is_null()
    } {
        return;
    }
    msg(b"The slime divides.  Ick!\0" as *const u8 as *const libc::c_char);
    new_monster(nslime, 'S' as i32 as byte, &mut slimy);
    if cansee(slimy.y, slimy.x) {
        (*nslime)._t._t_oldch = *_level.offset(INDEX(slimy.y, slimy.x) as isize);
        cur_mvaddch(slimy.y, slimy.x, 'S' as i32 as byte);
    }
    start_run(&mut slimy);
}
unsafe extern "C" fn new_slime(mut tp: *mut THING) -> bool {
    let mut y: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut ty: libc::c_int = 0;
    let mut tx: libc::c_int = 0;
    let mut ret: bool = false;
    let mut ntp: *mut THING = 0 as *mut THING;
    let mut sp: coord = coord { x: 0, y: 0 };
    ret = 0 as libc::c_int != 0;
    (*tp)._t._t_flags = ((*tp)._t._t_flags as libc::c_int | 0x8000 as libc::c_int) as libc::c_short;
    ty = (*tp)._t._t_pos.y;
    tx = (*tp)._t._t_pos.x;
    if !plop_monster(ty, tx, &mut sp) {
        y = ty - 1 as libc::c_int;
        while y <= ty + 1 as libc::c_int {
            x = tx - 1 as libc::c_int;
            while x <= tx + 1 as libc::c_int {
                if winat(y, x) as libc::c_int == 'S' as i32 && {
                    ntp = moat(y, x);
                    !ntp.is_null()
                } {
                    if !((*ntp)._t._t_flags as libc::c_int & 0x8000 as libc::c_int != 0) {
                        if new_slime(ntp) {
                            y = ty + 2 as libc::c_int;
                            x = tx + 2 as libc::c_int;
                        }
                    }
                }
                x += 1;
            }
            y += 1;
        }
    } else {
        ret = 1 as libc::c_int != 0;
        slimy = sp;
    }
    (*tp)._t._t_flags =
        ((*tp)._t._t_flags as libc::c_int & !(0x8000 as libc::c_int)) as libc::c_short;
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn plop_monster(
    mut r: libc::c_int,
    mut c: libc::c_int,
    mut cp: *mut coord,
) -> bool {
    let mut y: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut inv_odds: libc::c_int = 0 as libc::c_int;
    let mut appear: bool = 0 as libc::c_int != 0;
    let mut ch: byte = 0;
    y = r - 1 as libc::c_int;
    while y <= r + 1 as libc::c_int {
        x = c - 1 as libc::c_int;
        while x <= c + 1 as libc::c_int {
            if !(y == player._t._t_pos.y && x == player._t._t_pos.x
                || offmap(y, x) as libc::c_int != 0)
            {
                ch = winat(y, x);
                if step_ok(ch) {
                    if !(ch as libc::c_int == 0xd as libc::c_int
                        && (*find_obj(y, x))._o._o_which == 6 as libc::c_int)
                    {
                        appear = 1 as libc::c_int != 0;
                        inv_odds += 1;
                        if rnd(inv_odds) == 0 as libc::c_int {
                            (*cp).y = y;
                            (*cp).x = x;
                        }
                    }
                }
            }
            x += 1;
        }
        y += 1;
    }
    return appear;
}
