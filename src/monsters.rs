use crate::rnd;
use ::libc;
extern "C" {
    fn memmove(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
        -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    static mut f_damage: [libc::c_char; 0];
    static mut fung_hit: libc::c_int;
    static mut level: libc::c_int;
    static mut rooms: [room; 0];
    static mut monsters: [monster; 0];
    fn start_run(runner: *mut coord);
    fn roomin(cp: *mut coord) -> *mut room;
    fn fuse(func: Option<unsafe extern "C" fn() -> ()>, time: libc::c_int);
    fn lengthen(func: Option<unsafe extern "C" fn() -> ()>, xtime: libc::c_int);
    fn unconfuse();
    fn save(which: libc::c_int) -> bool;
    static mut total: libc::c_int;
    static mut cur_ring: [*mut THING; 0];
    static mut mlist: *mut THING;
    static mut player: THING;
    fn new_item() -> *mut THING;
    fn list_attach(list: *mut *mut THING, item: *mut THING);
    fn roll(number: libc::c_int, sides: libc::c_int) -> libc::c_int;
    fn step_ok(ch: byte) -> bool;
    fn winat(y: libc::c_int, x: libc::c_int) -> byte;
    fn DISTANCE(y1: libc::c_int, x1: libc::c_int, y2: libc::c_int, x2: libc::c_int) -> libc::c_int;
    fn spread(nm: libc::c_int) -> libc::c_int;
    fn rnd_pos(rp: *mut room, cp: *mut coord);
    fn rnd_room() -> libc::c_int;
    fn msg(fmt: *const libc::c_char, _: ...);
    fn new_thing() -> *mut THING;
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
static mut vorp_mons: *mut libc::c_char =
    b"KEBHISORZLCAQNYTWFPUGMXVJD\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
static mut lvl_mons: *mut libc::c_char =
    b"K BHISOR LCA NYTWFP GMXVJD\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
static mut wand_mons: *mut libc::c_char =
    b"KEBHISORZ CAQ YTW PUGM VJ \0" as *const u8 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub unsafe extern "C" fn randmonster(mut wander: bool) -> libc::c_char {
    let mut d: libc::c_int = 0;
    let mut mons: *mut libc::c_char = 0 as *mut libc::c_char;
    mons = if wander as libc::c_int != 0 {
        wand_mons
    } else {
        lvl_mons
    };
    loop {
        let mut r10: libc::c_int = rnd(5 as libc::c_int) + rnd(6 as libc::c_int);
        d = level + (r10 - 5 as libc::c_int);
        if d < 1 as libc::c_int {
            d = rnd(5 as libc::c_int) + 1 as libc::c_int;
        }
        if d > 26 as libc::c_int {
            d = rnd(5 as libc::c_int) + 22 as libc::c_int;
        }
        d -= 1;
        if !(*mons.offset(d as isize) as libc::c_int == ' ' as i32) {
            break;
        }
    }
    return *mons.offset(d as isize);
}
#[no_mangle]
pub unsafe extern "C" fn new_monster(mut tp: *mut THING, mut type_0: byte, mut cp: *mut coord) {
    let mut mp: *mut monster = 0 as *mut monster;
    let mut lev_add: libc::c_int = 0;
    lev_add = level - 26 as libc::c_int;
    if lev_add < 0 as libc::c_int {
        lev_add = 0 as libc::c_int;
    }
    list_attach(&mut mlist, tp);
    (*tp)._t._t_type = type_0 as libc::c_char;
    (*tp)._t._t_disguise = type_0;
    memmove(
        &mut (*tp)._t._t_pos as *mut coord as *mut libc::c_void,
        cp as *const libc::c_void,
        ::core::mem::size_of::<coord>() as libc::c_ulong,
    );
    (*tp)._t._t_oldch = '@' as i32 as byte;
    (*tp)._t._t_room = roomin(cp);
    mp = &mut *monsters
        .as_mut_ptr()
        .offset(((*tp)._t._t_type as libc::c_int - 'A' as i32) as isize) as *mut monster;
    (*tp)._t._t_stats.s_lvl = (*mp).m_stats.s_lvl + lev_add;
    (*tp)._t._t_stats.s_hpt = roll((*tp)._t._t_stats.s_lvl, 8 as libc::c_int);
    (*tp)._t._t_stats.s_maxhp = (*tp)._t._t_stats.s_hpt;
    (*tp)._t._t_stats.s_arm = (*mp).m_stats.s_arm - lev_add;
    (*tp)._t._t_stats.s_dmg = (*mp).m_stats.s_dmg;
    (*tp)._t._t_stats.s_str = (*mp).m_stats.s_str;
    (*tp)._t._t_stats.s_exp = (*mp).m_stats.s_exp
        + (lev_add * 10 as libc::c_int) as libc::c_long
        + exp_add(tp) as libc::c_long;
    (*tp)._t._t_flags = (*mp).m_flags as libc::c_short;
    (*tp)._t._t_turn = 1 as libc::c_int as libc::c_char;
    (*tp)._t._t_pack = 0 as *mut thing;
    if !(*cur_ring.as_mut_ptr().offset(0 as libc::c_int as isize)).is_null()
        && (**cur_ring.as_mut_ptr().offset(0 as libc::c_int as isize))
            ._o
            ._o_which
            == 6 as libc::c_int
        || !(*cur_ring.as_mut_ptr().offset(1 as libc::c_int as isize)).is_null()
            && (**cur_ring.as_mut_ptr().offset(1 as libc::c_int as isize))
                ._o
                ._o_which
                == 6 as libc::c_int
    {
        start_run(cp);
    }
    if type_0 as libc::c_int == 'F' as i32 {
        (*tp)._t._t_stats.s_dmg = f_damage.as_mut_ptr();
    }
    if type_0 as libc::c_int == 'X' as i32 {
        match rnd(if level > 25 as libc::c_int {
            9 as libc::c_int
        } else {
            8 as libc::c_int
        }) {
            0 => {
                (*tp)._t._t_disguise = 0xf as libc::c_int as byte;
            }
            1 => {
                (*tp)._t._t_disguise = 0xad as libc::c_int as byte;
            }
            2 => {
                (*tp)._t._t_disguise = 0xd as libc::c_int as byte;
            }
            3 => {
                (*tp)._t._t_disguise = 0xf0 as libc::c_int as byte;
            }
            4 => {
                (*tp)._t._t_disguise = 0x18 as libc::c_int as byte;
            }
            5 => {
                (*tp)._t._t_disguise = 0x8 as libc::c_int as byte;
            }
            6 => {
                (*tp)._t._t_disguise = 0x9 as libc::c_int as byte;
            }
            7 => {
                (*tp)._t._t_disguise = 0xe7 as libc::c_int as byte;
            }
            8 => {
                (*tp)._t._t_disguise = 0xc as libc::c_int as byte;
            }
            _ => {}
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn f_restor() {
    let mut mp: *mut monster = &mut *monsters
        .as_mut_ptr()
        .offset(('F' as i32 - 'A' as i32) as isize) as *mut monster;
    fung_hit = 0 as libc::c_int;
    strcpy(f_damage.as_mut_ptr(), (*mp).m_stats.s_dmg);
}
unsafe extern "C" fn exp_add(mut tp: *mut THING) -> libc::c_int {
    let mut mod_0: libc::c_int = 0;
    if (*tp)._t._t_stats.s_lvl == 1 as libc::c_int {
        mod_0 = (*tp)._t._t_stats.s_maxhp / 8 as libc::c_int;
    } else {
        mod_0 = (*tp)._t._t_stats.s_maxhp / 6 as libc::c_int;
    }
    if (*tp)._t._t_stats.s_lvl > 9 as libc::c_int {
        mod_0 *= 20 as libc::c_int;
    } else if (*tp)._t._t_stats.s_lvl > 6 as libc::c_int {
        mod_0 *= 4 as libc::c_int;
    }
    return mod_0;
}
#[no_mangle]
pub unsafe extern "C" fn wanderer() {
    let mut i: libc::c_int = 0;
    let mut rp: *mut room = 0 as *mut room;
    let mut tp: *mut THING = 0 as *mut THING;
    let mut cp: coord = coord { x: 0, y: 0 };
    tp = new_item();
    if tp.is_null() {
        return;
    }
    loop {
        i = rnd_room();
        rp = &mut *rooms.as_mut_ptr().offset(i as isize) as *mut room;
        if !(rp == player._t._t_room) {
            rnd_pos(rp, &mut cp);
        }
        if rp != player._t._t_room && step_ok(winat(cp.y, cp.x)) as libc::c_int != 0 {
            break;
        }
    }
    new_monster(tp, randmonster(1 as libc::c_int != 0) as byte, &mut cp);
    start_run(&mut (*tp)._t._t_pos);
}
#[no_mangle]
pub unsafe extern "C" fn wake_monster(mut y: libc::c_int, mut x: libc::c_int) -> *mut THING {
    let mut tp: *mut THING = 0 as *mut THING;
    let mut rp: *mut room = 0 as *mut room;
    let mut ch: byte = 0;
    let mut dst: libc::c_int = 0;
    tp = moat(y, x);
    if tp.is_null() {
        return tp;
    }
    ch = (*tp)._t._t_type as byte;
    if !((*tp)._t._t_flags as libc::c_int & 0x4 as libc::c_int != 0 as libc::c_int)
        && rnd(3 as libc::c_int) != 0 as libc::c_int
        && (*tp)._t._t_flags as libc::c_int & 0x20 as libc::c_int != 0 as libc::c_int
        && !((*tp)._t._t_flags as libc::c_int & 0x80 as libc::c_int != 0 as libc::c_int)
        && !(!(*cur_ring.as_mut_ptr().offset(0 as libc::c_int as isize)).is_null()
            && (**cur_ring.as_mut_ptr().offset(0 as libc::c_int as isize))
                ._o
                ._o_which
                == 12 as libc::c_int
            || !(*cur_ring.as_mut_ptr().offset(1 as libc::c_int as isize)).is_null()
                && (**cur_ring.as_mut_ptr().offset(1 as libc::c_int as isize))
                    ._o
                    ._o_which
                    == 12 as libc::c_int)
    {
        (*tp)._t._t_dest = &mut player._t._t_pos;
        (*tp)._t._t_flags =
            ((*tp)._t._t_flags as libc::c_int | 0x4 as libc::c_int) as libc::c_short;
    }
    if ch as libc::c_int == 'M' as i32
        && !(player._t._t_flags as libc::c_int & 0x1 as libc::c_int != 0 as libc::c_int)
        && !((*tp)._t._t_flags as libc::c_int & 0x8 as libc::c_int != 0 as libc::c_int)
        && !((*tp)._t._t_flags as libc::c_int & 0x1000 as libc::c_int != 0 as libc::c_int)
        && (*tp)._t._t_flags as libc::c_int & 0x4 as libc::c_int != 0 as libc::c_int
    {
        rp = player._t._t_room;
        dst = DISTANCE(y, x, player._t._t_pos.y, player._t._t_pos.x);
        if !rp.is_null() && (*rp).r_flags as libc::c_int & 0x1 as libc::c_int == 0
            || dst < 3 as libc::c_int
        {
            (*tp)._t._t_flags =
                ((*tp)._t._t_flags as libc::c_int | 0x8 as libc::c_int) as libc::c_short;
            if !save(0o3 as libc::c_int) {
                if player._t._t_flags as libc::c_int & 0x100 as libc::c_int != 0 as libc::c_int {
                    lengthen(
                        ::core::mem::transmute::<
                            Option<unsafe extern "C" fn() -> ()>,
                            Option<unsafe extern "C" fn() -> ()>,
                        >(Some(unconfuse as unsafe extern "C" fn() -> ())),
                        rnd(20 as libc::c_int) + spread(20 as libc::c_int),
                    );
                } else {
                    fuse(
                        ::core::mem::transmute::<
                            Option<unsafe extern "C" fn() -> ()>,
                            Option<unsafe extern "C" fn() -> ()>,
                        >(Some(unconfuse as unsafe extern "C" fn() -> ())),
                        rnd(20 as libc::c_int) + spread(20 as libc::c_int),
                    );
                }
                player._t._t_flags =
                    (player._t._t_flags as libc::c_int | 0x100 as libc::c_int) as libc::c_short;
                msg(b"the medusa's gaze has confused you\0" as *const u8 as *const libc::c_char);
            }
        }
    }
    if (*tp)._t._t_flags as libc::c_int & 0x40 as libc::c_int != 0 as libc::c_int
        && !((*tp)._t._t_flags as libc::c_int & 0x4 as libc::c_int != 0 as libc::c_int)
    {
        (*tp)._t._t_flags =
            ((*tp)._t._t_flags as libc::c_int | 0x4 as libc::c_int) as libc::c_short;
        if (*player._t._t_room).r_goldval != 0 {
            (*tp)._t._t_dest = &mut (*player._t._t_room).r_gold;
        } else {
            (*tp)._t._t_dest = &mut player._t._t_pos;
        }
    }
    return tp;
}
#[no_mangle]
pub unsafe extern "C" fn give_pack(mut tp: *mut THING) {
    if total < 83 as libc::c_int
        && rnd(100 as libc::c_int)
            < (*monsters
                .as_mut_ptr()
                .offset(((*tp)._t._t_type as libc::c_int - 'A' as i32) as isize))
            .m_carry
    {
        list_attach(&mut (*tp)._t._t_pack, new_thing());
    }
}
#[no_mangle]
pub unsafe extern "C" fn pick_mons() -> libc::c_char {
    let mut cp: *mut libc::c_char = vorp_mons.offset(strlen(vorp_mons) as isize);
    loop {
        cp = cp.offset(-1);
        if !(cp >= vorp_mons && rnd(10 as libc::c_int) != 0) {
            break;
        }
    }
    if cp < vorp_mons {
        return 'M' as i32 as libc::c_char;
    }
    return *cp;
}
#[no_mangle]
pub unsafe extern "C" fn moat(mut my: libc::c_int, mut mx: libc::c_int) -> *mut THING {
    let mut tp: *mut THING = 0 as *mut THING;
    tp = mlist;
    while !tp.is_null() {
        if (*tp)._t._t_pos.x == mx && (*tp)._t._t_pos.y == my {
            return tp;
        }
        tp = (*tp)._t._l_next;
    }
    return 0 as *mut THING;
}
