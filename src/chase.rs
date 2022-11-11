use ::libc;
extern "C" {
    fn abs(_: libc::c_int) -> libc::c_int;
    static mut bailout: bool;
    static mut expert: bool;
    static mut running: bool;
    static mut terse: bool;
    static mut flashmsg: *mut libc::c_char;
    static mut intense: *mut libc::c_char;
    static mut w_names: [*mut libc::c_char; 0];
    static mut cur_weapon: *mut THING;
    static mut lvl_obj: *mut THING;
    static mut mlist: *mut THING;
    static mut player: THING;
    static mut delta: coord;
    static mut passages: [room; 0];
    static mut rooms: [room; 0];
    static mut monsters: [monster; 0];
    static mut _level: *mut byte;
    static mut _flags: *mut byte;
    fn INDEX(y: libc::c_int, x: libc::c_int) -> libc::c_int;
    fn msg(fmt: *const libc::c_char, _: ...);
    fn DISTANCE(
        y1: libc::c_int,
        x1: libc::c_int,
        y2: libc::c_int,
        x2: libc::c_int,
    ) -> libc::c_int;
    fn rnd(range: libc::c_int) -> libc::c_int;
    fn _ce(a: *mut coord, b: *mut coord) -> bool;
    fn list_attach(list: *mut *mut THING, item: *mut THING);
    fn list_detach(list: *mut *mut THING, item: *mut THING);
    fn attack(mp: *mut THING);
    fn step_ok(ch: byte) -> bool;
    fn winat(y: libc::c_int, x: libc::c_int) -> byte;
    fn offmap(y: libc::c_int, x: libc::c_int) -> bool;
    fn rndmove(who: *mut THING, newmv: *mut coord);
    fn fire_bolt(start: *mut coord, dir: *mut coord, name: *mut libc::c_char);
    fn sign(nm: libc::c_int) -> libc::c_int;
    fn moat(my: libc::c_int, mx: libc::c_int) -> *mut THING;
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
#[no_mangle]
pub static mut ch_ret: coord = coord { x: 0, y: 0 };
#[no_mangle]
pub unsafe extern "C" fn runners() {
    let mut tp: *mut THING = 0 as *mut THING;
    let mut dist: libc::c_int = 0;
    tp = mlist;
    while !tp.is_null() {
        if !((*tp)._t._t_flags as libc::c_int & 0x80 as libc::c_int != 0 as libc::c_int)
            && (*tp)._t._t_flags as libc::c_int & 0x4 as libc::c_int != 0 as libc::c_int
        {
            dist = DISTANCE(
                player._t._t_pos.y,
                player._t._t_pos.x,
                (*tp)._t._t_pos.y,
                (*tp)._t._t_pos.x,
            );
            if !((*tp)._t._t_flags as libc::c_int & 0x2000 as libc::c_int
                != 0 as libc::c_int
                || (*tp)._t._t_type as libc::c_int == 'S' as i32
                    && dist > 3 as libc::c_int) || (*tp)._t._t_turn as libc::c_int != 0
            {
                do_chase(tp);
            }
            if (*tp)._t._t_flags as libc::c_int & 0x4000 as libc::c_int
                != 0 as libc::c_int
            {
                do_chase(tp);
            }
            dist = DISTANCE(
                player._t._t_pos.y,
                player._t._t_pos.x,
                (*tp)._t._t_pos.y,
                (*tp)._t._t_pos.x,
            );
            if (*tp)._t._t_flags as libc::c_int & 0x8000 as libc::c_int
                != 0 as libc::c_int && dist > 3 as libc::c_int
            {
                do_chase(tp);
            }
            (*tp)
                ._t
                ._t_turn = ((*tp)._t._t_turn as libc::c_int ^ 1 as libc::c_int)
                as libc::c_char;
        }
        tp = (*tp)._t._l_next;
    }
}
#[no_mangle]
pub unsafe extern "C" fn do_chase(mut th: *mut THING) {
    let mut mindist: libc::c_int = 32767 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut dist: libc::c_int = 0;
    let mut door: bool = false;
    let mut obj: *mut THING = 0 as *mut THING;
    let mut oroom: *mut room = 0 as *mut room;
    let mut rer: *mut room = 0 as *mut room;
    let mut ree: *mut room = 0 as *mut room;
    let mut this: coord = coord { x: 0, y: 0 };
    rer = (*th)._t._t_room;
    if (*th)._t._t_flags as libc::c_int & 0x40 as libc::c_int != 0 as libc::c_int
        && (*rer).r_goldval == 0 as libc::c_int
    {
        (*th)._t._t_dest = &mut player._t._t_pos;
    }
    ree = player._t._t_room;
    if (*th)._t._t_dest != &mut player._t._t_pos as *mut coord {
        ree = roomin((*th)._t._t_dest);
    }
    if ree.is_null() {
        return;
    }
    door = *_level.offset(INDEX((*th)._t._t_pos.y, (*th)._t._t_pos.x) as isize)
        as libc::c_int == 0xce as libc::c_int;
    loop {
        if rer != ree
            && (*rer).r_flags as libc::c_int & 0x4 as libc::c_int == 0 as libc::c_int
        {
            i = 0 as libc::c_int;
            while i < (*rer).r_nexits {
                dist = DISTANCE(
                    (*(*th)._t._t_dest).y,
                    (*(*th)._t._t_dest).x,
                    (*rer).r_exit[i as usize].y,
                    (*rer).r_exit[i as usize].x,
                );
                if dist < mindist {
                    this = (*rer).r_exit[i as usize];
                    mindist = dist;
                }
                i += 1;
            }
            if !door {
                break;
            }
            rer = &mut *passages
                .as_mut_ptr()
                .offset(
                    (*_flags
                        .offset(
                            (INDEX
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    libc::c_int,
                                ) -> libc::c_int)((*th)._t._t_pos.y, (*th)._t._t_pos.x)
                                as isize,
                        ) as libc::c_int & 0xf as libc::c_int) as isize,
                ) as *mut room;
            door = 0 as libc::c_int != 0;
        } else {
            this = *(*th)._t._t_dest;
            if ((*th)._t._t_type as libc::c_int == 'D' as i32
                || (*th)._t._t_type as libc::c_int == 'I' as i32)
                && ((*th)._t._t_pos.y == player._t._t_pos.y
                    || (*th)._t._t_pos.x == player._t._t_pos.x
                    || abs((*th)._t._t_pos.y - player._t._t_pos.y)
                        == abs((*th)._t._t_pos.x - player._t._t_pos.x))
                && {
                    dist = DISTANCE(
                        (*th)._t._t_pos.y,
                        (*th)._t._t_pos.x,
                        player._t._t_pos.y,
                        player._t._t_pos.x,
                    );
                    dist > 2 as libc::c_int
                        && dist <= 6 as libc::c_int * 6 as libc::c_int
                }
                && !((*th)._t._t_flags as libc::c_int & 0x1000 as libc::c_int
                    != 0 as libc::c_int) && rnd(5 as libc::c_int) == 0 as libc::c_int
            {
                running = 0 as libc::c_int != 0;
                delta.y = sign(player._t._t_pos.y - (*th)._t._t_pos.y);
                delta.x = sign(player._t._t_pos.x - (*th)._t._t_pos.x);
                fire_bolt(
                    &mut (*th)._t._t_pos,
                    &mut delta,
                    (if (*th)._t._t_type as libc::c_int == 'D' as i32 {
                        b"flame\0" as *const u8 as *const libc::c_char
                    } else {
                        b"frost\0" as *const u8 as *const libc::c_char
                    }) as *mut libc::c_char,
                );
                return;
            }
            break;
        }
    }
    chase(th, &mut this);
    if _ce(&mut ch_ret, &mut player._t._t_pos) {
        attack(th);
        return;
    } else {
        if _ce(&mut ch_ret, (*th)._t._t_dest) {
            obj = lvl_obj;
            while !obj.is_null() {
                if (*th)._t._t_dest == &mut (*obj)._o._o_pos as *mut coord {
                    let mut oldchar: byte = 0;
                    list_detach(&mut lvl_obj, obj);
                    list_attach(&mut (*th)._t._t_pack, obj);
                    let ref mut fresh0 = *_level
                        .offset(INDEX((*obj)._o._o_pos.y, (*obj)._o._o_pos.x) as isize);
                    *fresh0 = (if (*(*th)._t._t_room).r_flags as libc::c_int
                        & 0x2 as libc::c_int != 0
                    {
                        0xb1 as libc::c_int
                    } else {
                        0xfa as libc::c_int
                    }) as byte;
                    oldchar = *fresh0;
                    if cansee((*obj)._o._o_pos.y, (*obj)._o._o_pos.x) {
                        cur_mvaddch((*obj)._o._o_pos.y, (*obj)._o._o_pos.x, oldchar);
                    }
                    (*th)._t._t_dest = find_dest(th);
                    break;
                } else {
                    obj = (*obj)._t._l_next;
                }
            }
        }
    }
    if (*th)._t._t_type as libc::c_int == 'F' as i32 {
        return;
    }
    if (*th)._t._t_oldch as libc::c_int != '@' as i32 {
        if (*th)._t._t_oldch as libc::c_int == ' ' as i32
            && cansee((*th)._t._t_pos.y, (*th)._t._t_pos.x) as libc::c_int != 0
            && *_level.offset(INDEX((*th)._t._t_pos.y, (*th)._t._t_pos.x) as isize)
                as libc::c_int == 0xfa as libc::c_int
        {
            cur_mvaddch(
                (*th)._t._t_pos.y,
                (*th)._t._t_pos.x,
                0xfa as libc::c_int as byte,
            );
        } else if (*th)._t._t_oldch as libc::c_int == 0xfa as libc::c_int
            && !cansee((*th)._t._t_pos.y, (*th)._t._t_pos.x)
            && !(player._t._t_flags as libc::c_int & 0x2 as libc::c_int
                != 0 as libc::c_int)
        {
            cur_mvaddch((*th)._t._t_pos.y, (*th)._t._t_pos.x, ' ' as i32 as byte);
        } else {
            cur_mvaddch((*th)._t._t_pos.y, (*th)._t._t_pos.x, (*th)._t._t_oldch);
        }
    }
    oroom = (*th)._t._t_room;
    if !_ce(&mut ch_ret, &mut (*th)._t._t_pos) {
        (*th)._t._t_room = roomin(&mut ch_ret);
        if ((*th)._t._t_room).is_null() {
            (*th)._t._t_room = oroom;
            return;
        }
        if oroom != (*th)._t._t_room {
            (*th)._t._t_dest = find_dest(th);
        }
        (*th)._t._t_pos = ch_ret;
    }
    if see_monst(th) {
        if *_flags.offset(INDEX(ch_ret.y, ch_ret.x) as isize) as libc::c_int
            & 0x40 as libc::c_int != 0
        {
            set_attr(14 as libc::c_int);
        }
        (*th)._t._t_oldch = cur_mvinch(ch_ret.y, ch_ret.x);
        cur_mvaddch(ch_ret.y, ch_ret.x, (*th)._t._t_disguise);
    } else if player._t._t_flags as libc::c_int & 0x2 as libc::c_int != 0 as libc::c_int
    {
        set_attr(14 as libc::c_int);
        (*th)._t._t_oldch = cur_mvinch(ch_ret.y, ch_ret.x);
        cur_mvaddch(ch_ret.y, ch_ret.x, (*th)._t._t_type as byte);
    } else {
        (*th)._t._t_oldch = '@' as i32 as byte;
    }
    if (*th)._t._t_oldch as libc::c_int == 0xfa as libc::c_int
        && (*oroom).r_flags as libc::c_int & 0x1 as libc::c_int != 0
    {
        (*th)._t._t_oldch = ' ' as i32 as byte;
    }
    set_attr(0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn see_monst(mut mp: *mut THING) -> bool {
    if player._t._t_flags as libc::c_int & 0x1 as libc::c_int != 0 as libc::c_int {
        return 0 as libc::c_int != 0;
    }
    if (*mp)._t._t_flags as libc::c_int & 0x10 as libc::c_int != 0 as libc::c_int
        && !(player._t._t_flags as libc::c_int & 0x800 as libc::c_int
            != 0 as libc::c_int)
    {
        return 0 as libc::c_int != 0;
    }
    if DISTANCE(
        (*mp)._t._t_pos.y,
        (*mp)._t._t_pos.x,
        player._t._t_pos.y,
        player._t._t_pos.x,
    ) >= 3 as libc::c_int
        && ((*mp)._t._t_room != player._t._t_room
            || (*(*mp)._t._t_room).r_flags as libc::c_int & 0x1 as libc::c_int != 0
            || (*(*mp)._t._t_room).r_flags as libc::c_int & 0x4 as libc::c_int != 0)
    {
        return 0 as libc::c_int != 0;
    }
    if !cur_weapon.is_null()
        && (*mp)._t._t_type as libc::c_int == (*cur_weapon)._o._o_enemy as libc::c_int
        && (*cur_weapon)._o._o_flags as libc::c_int & 0x4 as libc::c_int
            == 0 as libc::c_int
    {
        (*cur_weapon)
            ._o
            ._o_flags = ((*cur_weapon)._o._o_flags as libc::c_int | 0x4 as libc::c_int)
            as libc::c_short;
        msg(
            flashmsg,
            *w_names.as_mut_ptr().offset((*cur_weapon)._o._o_which as isize),
            if terse as libc::c_int != 0 || expert as libc::c_int != 0 {
                b"\0" as *const u8 as *const libc::c_char
            } else {
                intense as *const libc::c_char
            },
        );
    }
    return 1 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn start_run(mut runner: *mut coord) {
    let mut tp: *mut THING = 0 as *mut THING;
    tp = moat((*runner).y, (*runner).x);
    if !tp.is_null() {
        (*tp)
            ._t
            ._t_flags = ((*tp)._t._t_flags as libc::c_int | 0x4 as libc::c_int)
            as libc::c_short;
        (*tp)
            ._t
            ._t_flags = ((*tp)._t._t_flags as libc::c_int & !(0x80 as libc::c_int))
            as libc::c_short;
        (*tp)._t._t_dest = find_dest(tp);
    }
}
#[no_mangle]
pub unsafe extern "C" fn chase(mut tp: *mut THING, mut ee: *mut coord) {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut dist: libc::c_int = 0;
    let mut thisdist: libc::c_int = 0;
    let mut obj: *mut THING = 0 as *mut THING;
    let mut er: *mut coord = 0 as *mut coord;
    let mut ch: byte = 0;
    let mut plcnt: libc::c_int = 1 as libc::c_int;
    er = &mut (*tp)._t._t_pos;
    if (*tp)._t._t_flags as libc::c_int & 0x100 as libc::c_int != 0 as libc::c_int
        && rnd(5 as libc::c_int) != 0 as libc::c_int
        || (*tp)._t._t_type as libc::c_int == 'P' as i32
            && rnd(5 as libc::c_int) == 0 as libc::c_int
        || (*tp)._t._t_type as libc::c_int == 'B' as i32
            && rnd(2 as libc::c_int) == 0 as libc::c_int
    {
        rndmove(tp, &mut ch_ret);
        dist = DISTANCE(ch_ret.y, ch_ret.x, (*ee).y, (*ee).x);
        if rnd(30 as libc::c_int) == 17 as libc::c_int {
            (*tp)
                ._t
                ._t_flags = ((*tp)._t._t_flags as libc::c_int & !(0x100 as libc::c_int))
                as libc::c_short;
        }
    } else {
        let mut ey: libc::c_int = 0;
        let mut ex: libc::c_int = 0;
        dist = DISTANCE((*er).y, (*er).x, (*ee).y, (*ee).x);
        ch_ret = *er;
        ey = (*er).y + 1 as libc::c_int;
        ex = (*er).x + 1 as libc::c_int;
        x = (*er).x - 1 as libc::c_int;
        while x <= ex {
            let mut current_block_26: u64;
            y = (*er).y - 1 as libc::c_int;
            while y <= ey {
                let mut tryp: coord = coord { x: 0, y: 0 };
                tryp.x = x;
                tryp.y = y;
                if !(offmap(y, x) as libc::c_int != 0 || !diag_ok(er, &mut tryp)) {
                    ch = winat(y, x);
                    if step_ok(ch) {
                        if ch as libc::c_int == 0xd as libc::c_int {
                            obj = lvl_obj;
                            while !obj.is_null() {
                                if y == (*obj)._o._o_pos.y && x == (*obj)._o._o_pos.x {
                                    break;
                                }
                                obj = (*obj)._t._l_next;
                            }
                            if !obj.is_null() && (*obj)._o._o_which == 6 as libc::c_int {
                                current_block_26 = 1856101646708284338;
                            } else {
                                current_block_26 = 11298138898191919651;
                            }
                        } else {
                            current_block_26 = 11298138898191919651;
                        }
                        match current_block_26 {
                            1856101646708284338 => {}
                            _ => {
                                thisdist = DISTANCE(y, x, (*ee).y, (*ee).x);
                                if thisdist < dist {
                                    plcnt = 1 as libc::c_int;
                                    ch_ret = tryp;
                                    dist = thisdist;
                                } else if thisdist == dist
                                    && {
                                        plcnt += 1;
                                        rnd(plcnt) == 0 as libc::c_int
                                    }
                                {
                                    ch_ret = tryp;
                                    dist = thisdist;
                                }
                            }
                        }
                    }
                }
                y += 1;
            }
            x += 1;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn roomin(mut cp: *mut coord) -> *mut room {
    let mut rp: *mut room = 0 as *mut room;
    let mut fp: *mut byte = 0 as *mut byte;
    rp = rooms.as_mut_ptr();
    while rp
        <= &mut *rooms
            .as_mut_ptr()
            .offset((9 as libc::c_int - 1 as libc::c_int) as isize) as *mut room
    {
        if (*cp).x < (*rp).r_pos.x + (*rp).r_max.x && (*rp).r_pos.x <= (*cp).x
            && (*cp).y < (*rp).r_pos.y + (*rp).r_max.y && (*rp).r_pos.y <= (*cp).y
        {
            return rp;
        }
        rp = rp.offset(1);
    }
    fp = &mut *_flags
        .offset(
            (INDEX
                as unsafe extern "C" fn(
                    libc::c_int,
                    libc::c_int,
                ) -> libc::c_int)((*cp).y, (*cp).x) as isize,
        ) as *mut byte;
    if *fp as libc::c_int & 0x40 as libc::c_int != 0 {
        return &mut *passages
            .as_mut_ptr()
            .offset((*fp as libc::c_int & 0xf as libc::c_int) as isize) as *mut room;
    }
    bailout = 1 as libc::c_int != 0;
    return 0 as *mut room;
}
#[no_mangle]
pub unsafe extern "C" fn diag_ok(mut sp: *mut coord, mut ep: *mut coord) -> bool {
    if (*ep).x == (*sp).x || (*ep).y == (*sp).y {
        return 1 as libc::c_int != 0;
    }
    return step_ok(*_level.offset(INDEX((*ep).y, (*sp).x) as isize)) as libc::c_int != 0
        && step_ok(*_level.offset(INDEX((*sp).y, (*ep).x) as isize)) as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn cansee(mut y: libc::c_int, mut x: libc::c_int) -> bool {
    let mut rer: *mut room = 0 as *mut room;
    let mut tp: coord = coord { x: 0, y: 0 };
    if player._t._t_flags as libc::c_int & 0x1 as libc::c_int != 0 as libc::c_int {
        return 0 as libc::c_int != 0;
    }
    if DISTANCE(y, x, player._t._t_pos.y, player._t._t_pos.x) < 3 as libc::c_int {
        return 1 as libc::c_int != 0;
    }
    tp.y = y;
    tp.x = x;
    rer = roomin(&mut tp);
    return rer == player._t._t_room
        && (*rer).r_flags as libc::c_int & 0x1 as libc::c_int == 0;
}
#[no_mangle]
pub unsafe extern "C" fn find_dest(mut tp: *mut THING) -> *mut coord {
    let mut obj: *mut THING = 0 as *mut THING;
    let mut prob: libc::c_int = 0;
    let mut rp: *mut room = 0 as *mut room;
    prob = (*monsters
        .as_mut_ptr()
        .offset(((*tp)._t._t_type as libc::c_int - 'A' as i32) as isize))
        .m_carry;
    if prob <= 0 as libc::c_int || (*tp)._t._t_room == player._t._t_room
        || see_monst(tp) as libc::c_int != 0
    {
        return &mut player._t._t_pos;
    }
    rp = (*tp)._t._t_room;
    obj = lvl_obj;
    while !obj.is_null() {
        if !((*obj)._o._o_type == 0xd as libc::c_int
            && (*obj)._o._o_which == 6 as libc::c_int)
        {
            if roomin(&mut (*obj)._o._o_pos) == rp && rnd(100 as libc::c_int) < prob {
                tp = mlist;
                while !tp.is_null() {
                    if (*tp)._t._t_dest == &mut (*obj)._o._o_pos as *mut coord {
                        break;
                    }
                    tp = (*tp)._t._l_next;
                }
                if tp.is_null() {
                    return &mut (*obj)._o._o_pos;
                }
            }
        }
        obj = (*obj)._t._l_next;
    }
    return &mut player._t._t_pos;
}
