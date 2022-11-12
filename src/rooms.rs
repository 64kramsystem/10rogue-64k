use crate::rnd;
use ::libc;
extern "C" {
    fn memmove(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
        -> *mut libc::c_void;
    static mut maxrow: libc::c_int;
    static mut bailout: bool;
    static mut saw_amulet: bool;
    static mut level: libc::c_int;
    static mut max_level: libc::c_int;
    static mut lvl_obj: *mut THING;
    static mut player: THING;
    static mut passages: [room; 0];
    static mut rooms: [room; 0];
    static mut _level: *mut byte;
    static mut _flags: *mut byte;
    fn see_monst(mp: *mut THING) -> bool;
    fn roomin(cp: *mut coord) -> *mut room;
    fn new_item() -> *mut THING;
    fn list_attach(list: *mut *mut THING, item: *mut THING);
    fn draw_maze(rp: *mut room);
    fn winat(y: libc::c_int, x: libc::c_int) -> byte;
    fn INDEX(y: libc::c_int, x: libc::c_int) -> libc::c_int;
    fn randmonster(wander: bool) -> libc::c_char;
    fn new_monster(tp: *mut THING, type_0: byte, cp: *mut coord);
    fn give_pack(tp: *mut THING);
    fn moat(my: libc::c_int, mx: libc::c_int) -> *mut THING;
    fn door_open(rp: *mut room);
    fn rnd_room() -> libc::c_int;
    fn cur_mvinch(r: libc::c_int, c: libc::c_int) -> byte;
    fn cur_addch(chr: byte);
    fn set_attr(bute: libc::c_int);
    fn cur_move(row: libc::c_int, col: libc::c_int) -> libc::c_int;
    static mut COLS: libc::c_int;
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
pub unsafe extern "C" fn do_rooms() {
    let mut i: libc::c_int = 0;
    let mut rm: libc::c_int = 0;
    let mut rp: *mut room = 0 as *mut room;
    let mut tp: *mut THING = 0 as *mut THING;
    let mut left_out: libc::c_int = 0;
    let mut top: coord = coord { x: 0, y: 0 };
    let mut bsze: coord = coord { x: 0, y: 0 };
    let mut mp: coord = coord { x: 0, y: 0 };
    let mut endline: libc::c_int = 0;
    endline = maxrow + 1 as libc::c_int;
    bsze.x = COLS / 3 as libc::c_int;
    bsze.y = endline / 3 as libc::c_int;
    rp = rooms.as_mut_ptr();
    while rp < &mut *rooms.as_mut_ptr().offset(9 as libc::c_int as isize) as *mut room {
        (*rp).r_flags = 0 as libc::c_int as libc::c_short;
        (*rp).r_nexits = (*rp).r_flags as libc::c_int;
        (*rp).r_goldval = (*rp).r_nexits;
        rp = rp.offset(1);
    }
    left_out = rnd(4 as libc::c_int);
    i = 0 as libc::c_int;
    while i < left_out {
        loop {
            rm = (rnd_room as unsafe extern "C" fn() -> libc::c_int)();
            rp = &mut *rooms.as_mut_ptr().offset(rm as isize) as *mut room;
            if !((*rp).r_flags as libc::c_int & 0x4 as libc::c_int != 0) {
                break;
            }
        }
        (*rp).r_flags = ((*rp).r_flags as libc::c_int | 0x2 as libc::c_int) as libc::c_short;
        if rm > 2 as libc::c_int
            && level > 10 as libc::c_int
            && rnd(20 as libc::c_int) < level - 9 as libc::c_int
        {
            (*rp).r_flags = ((*rp).r_flags as libc::c_int | 0x4 as libc::c_int) as libc::c_short;
        }
        i += 1;
    }
    i = 0 as libc::c_int;
    rp = rooms.as_mut_ptr();
    while i < 9 as libc::c_int {
        top.x = i % 3 as libc::c_int * bsze.x + 1 as libc::c_int;
        top.y = i / 3 as libc::c_int * bsze.y;
        if (*rp).r_flags as libc::c_int & 0x2 as libc::c_int != 0 {
            if (*rp).r_flags as libc::c_int & 0x4 as libc::c_int != 0 {
                (*rp).r_pos.x = top.x;
                (*rp).r_pos.y = top.y;
                draw_maze(rp);
            } else {
                loop {
                    (*rp).r_pos.x = top.x + rnd(bsze.x - 2 as libc::c_int) + 1 as libc::c_int;
                    (*rp).r_pos.y = top.y + rnd(bsze.y - 2 as libc::c_int) + 1 as libc::c_int;
                    (*rp).r_max.x = -COLS;
                    (*rp).r_max.x = -endline;
                    if (*rp).r_pos.y > 0 as libc::c_int
                        && (*rp).r_pos.y < endline - 1 as libc::c_int
                    {
                        break;
                    }
                }
            }
        } else {
            if rnd(10 as libc::c_int) < level - 1 as libc::c_int {
                (*rp).r_flags =
                    ((*rp).r_flags as libc::c_int | 0x1 as libc::c_int) as libc::c_short;
            }
            loop {
                (*rp).r_max.x = rnd(bsze.x - 4 as libc::c_int) + 4 as libc::c_int;
                (*rp).r_max.y = rnd(bsze.y - 4 as libc::c_int) + 4 as libc::c_int;
                (*rp).r_pos.x = top.x + rnd(bsze.x - (*rp).r_max.x);
                (*rp).r_pos.y = top.y + rnd(bsze.y - (*rp).r_max.y);
                if !((*rp).r_pos.y == 0 as libc::c_int) {
                    break;
                }
            }
            draw_room(rp);
            if rnd(2 as libc::c_int) == 0 as libc::c_int && (!saw_amulet || level >= max_level) {
                let mut gold: *mut THING = 0 as *mut THING;
                gold = new_item();
                if !gold.is_null() {
                    (*rp).r_goldval =
                        rnd(50 as libc::c_int + 10 as libc::c_int * level) + 2 as libc::c_int;
                    (*gold)._o._o_ac = (*rp).r_goldval as libc::c_short;
                    loop {
                        let mut gch: byte = 0;
                        rnd_pos(rp, &mut (*rp).r_gold);
                        gch = *_level.offset(INDEX((*rp).r_gold.y, (*rp).r_gold.x) as isize);
                        if gch as libc::c_int == 0xfa as libc::c_int
                            || gch as libc::c_int == 0xb1 as libc::c_int
                        {
                            break;
                        }
                    }
                    memmove(
                        &mut (*gold)._o._o_pos as *mut coord as *mut libc::c_void,
                        &mut (*rp).r_gold as *mut coord as *const libc::c_void,
                        ::core::mem::size_of::<coord>() as libc::c_ulong,
                    );
                    (*gold)._o._o_flags = 0x20 as libc::c_int as libc::c_short;
                    (*gold)._o._o_group = 1 as libc::c_int;
                    (*gold)._o._o_type = 0xf as libc::c_int;
                    list_attach(&mut lvl_obj, gold);
                    *_level.offset(INDEX((*rp).r_gold.y, (*rp).r_gold.x) as isize) =
                        0xf as libc::c_int as byte;
                }
            }
            if rnd(100 as libc::c_int)
                < (if (*rp).r_goldval > 0 as libc::c_int {
                    80 as libc::c_int
                } else {
                    25 as libc::c_int
                })
            {
                tp = new_item();
                if !tp.is_null() {
                    let mut mch: byte = 0;
                    loop {
                        rnd_pos(rp, &mut mp);
                        mch = winat(mp.y, mp.x);
                        if mch as libc::c_int == 0xfa as libc::c_int
                            || mch as libc::c_int == 0xb1 as libc::c_int
                        {
                            break;
                        }
                    }
                    new_monster(tp, randmonster(0 as libc::c_int != 0) as byte, &mut mp);
                    give_pack(tp);
                }
            }
        }
        rp = rp.offset(1);
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn draw_room(mut rp: *mut room) {
    let mut y: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    vert(rp, (*rp).r_pos.x);
    vert(rp, (*rp).r_pos.x + (*rp).r_max.x - 1 as libc::c_int);
    horiz(rp, (*rp).r_pos.y);
    horiz(rp, (*rp).r_pos.y + (*rp).r_max.y - 1 as libc::c_int);
    *_level.offset(INDEX((*rp).r_pos.y, (*rp).r_pos.x) as isize) = 0xc9 as libc::c_int as byte;
    *_level.offset(INDEX(
        (*rp).r_pos.y,
        (*rp).r_pos.x + (*rp).r_max.x - 1 as libc::c_int,
    ) as isize) = 0xbb as libc::c_int as byte;
    *_level.offset(INDEX(
        (*rp).r_pos.y + (*rp).r_max.y - 1 as libc::c_int,
        (*rp).r_pos.x,
    ) as isize) = 0xc8 as libc::c_int as byte;
    *_level.offset(INDEX(
        (*rp).r_pos.y + (*rp).r_max.y - 1 as libc::c_int,
        (*rp).r_pos.x + (*rp).r_max.x - 1 as libc::c_int,
    ) as isize) = 0xbc as libc::c_int as byte;
    y = (*rp).r_pos.y + 1 as libc::c_int;
    while y < (*rp).r_pos.y + (*rp).r_max.y - 1 as libc::c_int {
        x = (*rp).r_pos.x + 1 as libc::c_int;
        while x < (*rp).r_pos.x + (*rp).r_max.x - 1 as libc::c_int {
            *_level.offset(INDEX(y, x) as isize) = 0xfa as libc::c_int as byte;
            x += 1;
        }
        y += 1;
    }
}
unsafe extern "C" fn vert(mut rp: *mut room, mut startx: libc::c_int) {
    let mut y: libc::c_int = 0;
    y = (*rp).r_pos.y + 1 as libc::c_int;
    while y <= (*rp).r_max.y + (*rp).r_pos.y - 1 as libc::c_int {
        *_level.offset(INDEX(y, startx) as isize) = 0xba as libc::c_int as byte;
        y += 1;
    }
}
unsafe extern "C" fn horiz(mut rp: *mut room, mut starty: libc::c_int) {
    let mut x: libc::c_int = 0;
    x = (*rp).r_pos.x;
    while x <= (*rp).r_pos.x + (*rp).r_max.x - 1 as libc::c_int {
        *_level.offset(INDEX(starty, x) as isize) = 0xcd as libc::c_int as byte;
        x += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn rnd_pos(mut rp: *mut room, mut cp: *mut coord) {
    (*cp).x = (*rp).r_pos.x + rnd((*rp).r_max.x - 2 as libc::c_int) + 1 as libc::c_int;
    (*cp).y = (*rp).r_pos.y + rnd((*rp).r_max.y - 2 as libc::c_int) + 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn enter_room(mut cp: *mut coord) {
    let mut rp: *mut room = 0 as *mut room;
    let mut y: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut tp: *mut THING = 0 as *mut THING;
    player._t._t_room = roomin(cp);
    rp = player._t._t_room;
    if bailout as libc::c_int != 0
        || (*rp).r_flags as libc::c_int & 0x2 as libc::c_int != 0
            && (*rp).r_flags as libc::c_int & 0x4 as libc::c_int == 0 as libc::c_int
    {
        return;
    }
    door_open(rp);
    if (*rp).r_flags as libc::c_int & 0x1 as libc::c_int == 0
        && !(player._t._t_flags as libc::c_int & 0x1 as libc::c_int != 0 as libc::c_int)
        && (*rp).r_flags as libc::c_int & 0x4 as libc::c_int == 0
    {
        y = (*rp).r_pos.y;
        while y < (*rp).r_max.y + (*rp).r_pos.y {
            cur_move(y, (*rp).r_pos.x);
            x = (*rp).r_pos.x;
            while x < (*rp).r_max.x + (*rp).r_pos.x {
                tp = moat(y, x);
                if tp.is_null() || !see_monst(tp) {
                    cur_addch(*_level.offset(INDEX(y, x) as isize));
                } else {
                    (*tp)._t._t_oldch = *_level.offset(INDEX(y, x) as isize);
                    cur_addch((*tp)._t._t_disguise);
                }
                x += 1;
            }
            y += 1;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn leave_room(mut cp: *mut coord) {
    let mut y: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut rp: *mut room = 0 as *mut room;
    let mut floor: byte = 0;
    let mut ch: byte = 0;
    rp = player._t._t_room;
    player._t._t_room = &mut *passages.as_mut_ptr().offset(
        (*_flags.offset(
            (INDEX as unsafe extern "C" fn(libc::c_int, libc::c_int) -> libc::c_int)(
                (*cp).y,
                (*cp).x,
            ) as isize,
        ) as libc::c_int
            & 0xf as libc::c_int) as isize,
    ) as *mut room;
    floor = (if (*rp).r_flags as libc::c_int & 0x1 as libc::c_int != 0
        && !(player._t._t_flags as libc::c_int & 0x1 as libc::c_int != 0 as libc::c_int)
    {
        ' ' as i32
    } else {
        0xfa as libc::c_int
    }) as byte;
    if (*rp).r_flags as libc::c_int & 0x4 as libc::c_int != 0 {
        floor = 0xb1 as libc::c_int as byte;
    }
    y = (*rp).r_pos.y + 1 as libc::c_int;
    while y < (*rp).r_max.y + (*rp).r_pos.y - 1 as libc::c_int {
        x = (*rp).r_pos.x + 1 as libc::c_int;
        while x < (*rp).r_max.x + (*rp).r_pos.x - 1 as libc::c_int {
            let mut current_block_13: u64;
            ch = cur_mvinch(y, x);
            match ch as libc::c_int {
                32 | 177 | 4 | 240 => {}
                250 => {
                    if floor as libc::c_int == ' ' as i32 {
                        cur_addch(' ' as i32 as byte);
                    }
                }
                _ => {
                    if ch as libc::c_int >= 'A' as i32 && ch as libc::c_int <= 'Z' as i32 {
                        if player._t._t_flags as libc::c_int & 0x2 as libc::c_int
                            != 0 as libc::c_int
                        {
                            set_attr(14 as libc::c_int);
                            cur_addch(ch);
                            set_attr(0 as libc::c_int);
                            current_block_13 = 13242334135786603907;
                        } else {
                            (*moat(y, x))._t._t_oldch = '@' as i32 as byte;
                            current_block_13 = 6009453772311597924;
                        }
                    } else {
                        current_block_13 = 6009453772311597924;
                    }
                    match current_block_13 {
                        13242334135786603907 => {}
                        _ => {
                            cur_addch(floor);
                        }
                    }
                }
            }
            x += 1;
        }
        y += 1;
    }
    door_open(rp);
}
