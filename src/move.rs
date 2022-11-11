use ::libc;
extern "C" {
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn flush_type();
    static mut maxrow: libc::c_int;
    static mut bailout: bool;
    static mut after: bool;
    static mut firstmove: bool;
    static mut running: bool;
    static mut was_trapped: libc::c_uchar;
    static mut runch: libc::c_char;
    static mut take: libc::c_char;
    static mut count: libc::c_int;
    static mut level: libc::c_int;
    static mut no_command: libc::c_int;
    static mut no_move: libc::c_int;
    static mut cur_ring: [*mut THING; 0];
    static mut cur_weapon: *mut THING;
    static mut lvl_obj: *mut THING;
    static mut player: THING;
    static mut oldpos: coord;
    static mut _level: *mut byte;
    static mut _flags: *mut byte;
    fn diag_ok(sp: *mut coord, ep: *mut coord) -> bool;
    fn fight(mp: *mut coord, mn: libc::c_char, weap: *mut THING, thrown: bool) -> bool;
    fn swing(at_lvl: libc::c_int, op_arm: libc::c_int, wplus: libc::c_int) -> bool;
    fn save(which: libc::c_int) -> bool;
    fn msg(fmt: *const libc::c_char, _: ...);
    fn noterse(str: *mut libc::c_char) -> *mut libc::c_char;
    fn new_item() -> *mut THING;
    fn rnd(range: libc::c_int) -> libc::c_int;
    fn roll(number: libc::c_int, sides: libc::c_int) -> libc::c_int;
    fn chg_str(amt: libc::c_int);
    fn step_ok(ch: byte) -> bool;
    fn _ce(a: *mut coord, b: *mut coord) -> bool;
    fn offmap(y: libc::c_int, x: libc::c_int) -> bool;
    fn winat(y: libc::c_int, x: libc::c_int) -> byte;
    fn spread(nm: libc::c_int) -> libc::c_int;
    fn INDEX(y: libc::c_int, x: libc::c_int) -> libc::c_int;
    fn wake_monster(y: libc::c_int, x: libc::c_int) -> *mut THING;
    fn moat(my: libc::c_int, mx: libc::c_int) -> *mut THING;
    fn enter_room(cp: *mut coord);
    fn leave_room(cp: *mut coord);
    fn death(monst: libc::c_char);
    fn teleport() -> libc::c_int;
    fn fall(obj: *mut THING, pr: bool);
    fn init_weapon(weap: *mut THING, type_0: byte);
    fn new_level();
    fn cur_mvaddch(r: libc::c_int, c: libc::c_int, chr: byte);
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
static mut nh: coord = coord { x: 0, y: 0 };
#[no_mangle]
pub unsafe extern "C" fn do_run(mut ch: byte) {
    running = 1 as libc::c_int != 0;
    after = 0 as libc::c_int != 0;
    runch = ch as libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn do_move(mut dy: libc::c_int, mut dx: libc::c_int) {
    let mut current_block: u64;
    let mut ch: byte = 0;
    let mut fl: libc::c_int = 0;
    firstmove = 0 as libc::c_int != 0;
    if bailout {
        bailout = 0 as libc::c_int != 0;
        msg(b"the crack widens ... \0" as *const u8 as *const libc::c_char);
        descend(b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
        return;
    }
    if no_move != 0 {
        no_move -= 1;
        msg(
            b"you are still stuck in the bear trap\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    if player._t._t_flags as libc::c_int & 0x100 as libc::c_int != 0 as libc::c_int
        && rnd(5 as libc::c_int) != 0 as libc::c_int
    {
        rndmove(&mut player, &mut nh);
        current_block = 7651349459974463963;
    } else {
        current_block = 17303099617230331132;
    }
    loop {
        match current_block {
            17303099617230331132 => {
                nh.y = player._t._t_pos.y + dy;
                nh.x = player._t._t_pos.x + dx;
                current_block = 7651349459974463963;
            }
            _ => {
                if !offmap(nh.y, nh.x) {
                    if !diag_ok(&mut player._t._t_pos, &mut nh) {
                        after = 0 as libc::c_int != 0;
                        running = 0 as libc::c_int != 0;
                        return;
                    }
                    if running as libc::c_int != 0
                        && _ce(&mut player._t._t_pos, &mut nh) as libc::c_int != 0
                    {
                        running = 0 as libc::c_int != 0;
                        after = running;
                    }
                    fl = *_flags.offset(INDEX(nh.y, nh.x) as isize) as libc::c_int;
                    ch = winat(nh.y, nh.x);
                    if *_level
                        .offset(INDEX(player._t._t_pos.y, player._t._t_pos.x) as isize)
                        as libc::c_int == 0xce as libc::c_int
                        && ch as libc::c_int == 0xfa as libc::c_int
                    {
                        running = 0 as libc::c_int != 0;
                    }
                    if fl & 0x10 as libc::c_int == 0
                        && ch as libc::c_int == 0xfa as libc::c_int
                    {
                        ch = 0x4 as libc::c_int as byte;
                        *_level.offset(INDEX(nh.y, nh.x) as isize) = ch;
                        let ref mut fresh0 = *_flags.offset(INDEX(nh.y, nh.x) as isize);
                        *fresh0 = (*fresh0 as libc::c_int | 0x10 as libc::c_int) as byte;
                    } else if player._t._t_flags as libc::c_int & 0x80 as libc::c_int
                        != 0 as libc::c_int && ch as libc::c_int != 'F' as i32
                    {
                        msg(b"you are being held\0" as *const u8 as *const libc::c_char);
                        return;
                    }
                    match ch as libc::c_int {
                        32 | 186 | 205 | 201 | 187 | 200 | 188 => {}
                        206 => {
                            current_block = 9565166327568304938;
                            match current_block {
                                488614886186827655 => {
                                    if fl & 0x10 as libc::c_int == 0 {
                                        be_trapped(&mut player._t._t_pos);
                                    }
                                    current_block = 13910360459000981292;
                                    break;
                                }
                                6010056518000876263 => {
                                    ch = be_trapped(&mut nh);
                                    if ch as libc::c_int == 0 as libc::c_int
                                        || ch as libc::c_int == 0o4 as libc::c_int
                                    {
                                        return;
                                    }
                                    current_block = 13910360459000981292;
                                    break;
                                }
                                9100868043760823246 => {
                                    running = 0 as libc::c_int != 0;
                                    if ch as libc::c_int >= 'A' as i32
                                        && ch as libc::c_int <= 'Z' as i32
                                        || !(moat(nh.y, nh.x)).is_null()
                                    {
                                        current_block = 3392087639489470149;
                                        break;
                                    } else {
                                        current_block = 5807581744382915773;
                                        break;
                                    }
                                }
                                _ => {
                                    running = 0 as libc::c_int != 0;
                                    if *_flags
                                        .offset(
                                            INDEX(player._t._t_pos.y, player._t._t_pos.x) as isize,
                                        ) as libc::c_int & 0x40 as libc::c_int != 0
                                    {
                                        enter_room(&mut nh);
                                    }
                                    current_block = 13910360459000981292;
                                    break;
                                }
                            }
                        }
                        4 => {
                            current_block = 6010056518000876263;
                            match current_block {
                                488614886186827655 => {
                                    if fl & 0x10 as libc::c_int == 0 {
                                        be_trapped(&mut player._t._t_pos);
                                    }
                                    current_block = 13910360459000981292;
                                    break;
                                }
                                6010056518000876263 => {
                                    ch = be_trapped(&mut nh);
                                    if ch as libc::c_int == 0 as libc::c_int
                                        || ch as libc::c_int == 0o4 as libc::c_int
                                    {
                                        return;
                                    }
                                    current_block = 13910360459000981292;
                                    break;
                                }
                                9100868043760823246 => {
                                    running = 0 as libc::c_int != 0;
                                    if ch as libc::c_int >= 'A' as i32
                                        && ch as libc::c_int <= 'Z' as i32
                                        || !(moat(nh.y, nh.x)).is_null()
                                    {
                                        current_block = 3392087639489470149;
                                        break;
                                    } else {
                                        current_block = 5807581744382915773;
                                        break;
                                    }
                                }
                                _ => {
                                    running = 0 as libc::c_int != 0;
                                    if *_flags
                                        .offset(
                                            INDEX(player._t._t_pos.y, player._t._t_pos.x) as isize,
                                        ) as libc::c_int & 0x40 as libc::c_int != 0
                                    {
                                        enter_room(&mut nh);
                                    }
                                    current_block = 13910360459000981292;
                                    break;
                                }
                            }
                        }
                        177 => {
                            current_block = 13910360459000981292;
                            break;
                        }
                        250 => {
                            current_block = 488614886186827655;
                            match current_block {
                                488614886186827655 => {
                                    if fl & 0x10 as libc::c_int == 0 {
                                        be_trapped(&mut player._t._t_pos);
                                    }
                                    current_block = 13910360459000981292;
                                    break;
                                }
                                6010056518000876263 => {
                                    ch = be_trapped(&mut nh);
                                    if ch as libc::c_int == 0 as libc::c_int
                                        || ch as libc::c_int == 0o4 as libc::c_int
                                    {
                                        return;
                                    }
                                    current_block = 13910360459000981292;
                                    break;
                                }
                                9100868043760823246 => {
                                    running = 0 as libc::c_int != 0;
                                    if ch as libc::c_int >= 'A' as i32
                                        && ch as libc::c_int <= 'Z' as i32
                                        || !(moat(nh.y, nh.x)).is_null()
                                    {
                                        current_block = 3392087639489470149;
                                        break;
                                    } else {
                                        current_block = 5807581744382915773;
                                        break;
                                    }
                                }
                                _ => {
                                    running = 0 as libc::c_int != 0;
                                    if *_flags
                                        .offset(
                                            INDEX(player._t._t_pos.y, player._t._t_pos.x) as isize,
                                        ) as libc::c_int & 0x40 as libc::c_int != 0
                                    {
                                        enter_room(&mut nh);
                                    }
                                    current_block = 13910360459000981292;
                                    break;
                                }
                            }
                        }
                        _ => {
                            current_block = 9100868043760823246;
                            match current_block {
                                488614886186827655 => {
                                    if fl & 0x10 as libc::c_int == 0 {
                                        be_trapped(&mut player._t._t_pos);
                                    }
                                    current_block = 13910360459000981292;
                                    break;
                                }
                                6010056518000876263 => {
                                    ch = be_trapped(&mut nh);
                                    if ch as libc::c_int == 0 as libc::c_int
                                        || ch as libc::c_int == 0o4 as libc::c_int
                                    {
                                        return;
                                    }
                                    current_block = 13910360459000981292;
                                    break;
                                }
                                9100868043760823246 => {
                                    running = 0 as libc::c_int != 0;
                                    if ch as libc::c_int >= 'A' as i32
                                        && ch as libc::c_int <= 'Z' as i32
                                        || !(moat(nh.y, nh.x)).is_null()
                                    {
                                        current_block = 3392087639489470149;
                                        break;
                                    } else {
                                        current_block = 5807581744382915773;
                                        break;
                                    }
                                }
                                _ => {
                                    running = 0 as libc::c_int != 0;
                                    if *_flags
                                        .offset(
                                            INDEX(player._t._t_pos.y, player._t._t_pos.x) as isize,
                                        ) as libc::c_int & 0x40 as libc::c_int != 0
                                    {
                                        enter_room(&mut nh);
                                    }
                                    current_block = 13910360459000981292;
                                    break;
                                }
                            }
                        }
                    }
                }
                if !(running as libc::c_int != 0
                    && ((*player._t._t_room).r_flags as libc::c_int & 0x2 as libc::c_int
                        != 0
                        && (*player._t._t_room).r_flags as libc::c_int
                            & 0x4 as libc::c_int == 0 as libc::c_int)
                    && !(player._t._t_flags as libc::c_int & 0x1 as libc::c_int
                        != 0 as libc::c_int))
                {
                    current_block = 7990025728955927862;
                    break;
                }
                let mut b1: bool = false;
                let mut b2: bool = false;
                match runch as libc::c_int {
                    104 | 108 => {
                        b1 = player._t._t_pos.y > 1 as libc::c_int
                            && (*_flags
                                .offset(
                                    INDEX(
                                        player._t._t_pos.y - 1 as libc::c_int,
                                        player._t._t_pos.x,
                                    ) as isize,
                                ) as libc::c_int & 0x40 as libc::c_int != 0
                                || *_level
                                    .offset(
                                        INDEX(
                                            player._t._t_pos.y - 1 as libc::c_int,
                                            player._t._t_pos.x,
                                        ) as isize,
                                    ) as libc::c_int == 0xce as libc::c_int);
                        b2 = player._t._t_pos.y < maxrow - 1 as libc::c_int
                            && (*_flags
                                .offset(
                                    INDEX(
                                        player._t._t_pos.y + 1 as libc::c_int,
                                        player._t._t_pos.x,
                                    ) as isize,
                                ) as libc::c_int & 0x40 as libc::c_int != 0
                                || *_level
                                    .offset(
                                        INDEX(
                                            player._t._t_pos.y + 1 as libc::c_int,
                                            player._t._t_pos.x,
                                        ) as isize,
                                    ) as libc::c_int == 0xce as libc::c_int);
                        if b1 as libc::c_int ^ b2 as libc::c_int == 0 {
                            current_block = 7990025728955927862;
                            break;
                        }
                        if b1 {
                            runch = 'k' as i32 as libc::c_char;
                            dy = -(1 as libc::c_int);
                        } else {
                            runch = 'j' as i32 as libc::c_char;
                            dy = 1 as libc::c_int;
                        }
                        dx = 0 as libc::c_int;
                        current_block = 17303099617230331132;
                    }
                    106 | 107 => {
                        b1 = player._t._t_pos.x > 1 as libc::c_int
                            && (*_flags
                                .offset(
                                    INDEX(
                                        player._t._t_pos.y,
                                        player._t._t_pos.x - 1 as libc::c_int,
                                    ) as isize,
                                ) as libc::c_int & 0x40 as libc::c_int != 0
                                || *_level
                                    .offset(
                                        INDEX(
                                            player._t._t_pos.y,
                                            player._t._t_pos.x - 1 as libc::c_int,
                                        ) as isize,
                                    ) as libc::c_int == 0xce as libc::c_int);
                        b2 = player._t._t_pos.x < COLS - 2 as libc::c_int
                            && (*_flags
                                .offset(
                                    INDEX(
                                        player._t._t_pos.y,
                                        player._t._t_pos.x + 1 as libc::c_int,
                                    ) as isize,
                                ) as libc::c_int & 0x40 as libc::c_int != 0
                                || *_level
                                    .offset(
                                        INDEX(
                                            player._t._t_pos.y,
                                            player._t._t_pos.x + 1 as libc::c_int,
                                        ) as isize,
                                    ) as libc::c_int == 0xce as libc::c_int);
                        if b1 as libc::c_int ^ b2 as libc::c_int == 0 {
                            current_block = 7990025728955927862;
                            break;
                        }
                        if b1 {
                            runch = 'h' as i32 as libc::c_char;
                            dx = -(1 as libc::c_int);
                        } else {
                            runch = 'l' as i32 as libc::c_char;
                            dx = 1 as libc::c_int;
                        }
                        dy = 0 as libc::c_int;
                        current_block = 17303099617230331132;
                    }
                    _ => {
                        current_block = 7990025728955927862;
                        break;
                    }
                }
            }
        }
    }
    match current_block {
        5807581744382915773 => {
            running = 0 as libc::c_int != 0;
            if ch as libc::c_int != 0xf0 as libc::c_int {
                take = ch as libc::c_char;
            }
            current_block = 13910360459000981292;
        }
        7990025728955927862 => {
            running = 0 as libc::c_int != 0;
            after = running;
            current_block = 7419121793134201633;
        }
        3392087639489470149 => {
            fight(&mut nh, ch as libc::c_char, cur_weapon, 0 as libc::c_int != 0);
            current_block = 7419121793134201633;
        }
        _ => {}
    }
    match current_block {
        13910360459000981292 => {
            cur_mvaddch(
                player._t._t_pos.y,
                player._t._t_pos.x,
                *_level.offset(INDEX(player._t._t_pos.y, player._t._t_pos.x) as isize),
            );
            if fl & 0x40 as libc::c_int != 0
                && (*_level.offset(INDEX(oldpos.y, oldpos.x) as isize) as libc::c_int
                    == 0xce as libc::c_int
                    || *_flags.offset(INDEX(oldpos.y, oldpos.x) as isize) as libc::c_int
                        & 0x20 as libc::c_int != 0)
            {
                leave_room(&mut nh);
            }
            if fl & 0x20 as libc::c_int != 0
                && *_flags.offset(INDEX(oldpos.y, oldpos.x) as isize) as libc::c_int
                    & 0x20 as libc::c_int == 0 as libc::c_int
            {
                enter_room(&mut nh);
            }
            memmove(
                &mut player._t._t_pos as *mut coord as *mut libc::c_void,
                &mut nh as *mut coord as *const libc::c_void,
                ::core::mem::size_of::<coord>() as libc::c_ulong,
            );
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn door_open(mut rp: *mut room) {
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut ch: byte = 0;
    let mut item: *mut THING = 0 as *mut THING;
    if (*rp).r_flags as libc::c_int & 0x2 as libc::c_int == 0
        && !(player._t._t_flags as libc::c_int & 0x1 as libc::c_int != 0 as libc::c_int)
    {
        j = (*rp).r_pos.y;
        while j < (*rp).r_pos.y + (*rp).r_max.y {
            k = (*rp).r_pos.x;
            while k < (*rp).r_pos.x + (*rp).r_max.x {
                ch = winat(j, k);
                if ch as libc::c_int >= 'A' as i32 && ch as libc::c_int <= 'Z' as i32 {
                    item = wake_monster(j, k);
                    if !item.is_null() {
                        if (*item)._t._t_oldch as libc::c_int == ' ' as i32
                            && (*rp).r_flags as libc::c_int & 0x1 as libc::c_int == 0
                            && !(player._t._t_flags as libc::c_int & 0x1 as libc::c_int
                                != 0 as libc::c_int)
                        {
                            (*item)._t._t_oldch = *_level.offset(INDEX(j, k) as isize);
                        }
                    }
                }
                k += 1;
            }
            j += 1;
        }
    }
}
unsafe extern "C" fn be_trapped(mut tc: *mut coord) -> byte {
    let mut tr: byte = 0;
    let mut index: libc::c_int = 0;
    running = 0 as libc::c_int != 0;
    count = running as libc::c_int;
    index = INDEX((*tc).y, (*tc).x);
    *_level.offset(index as isize) = 0x4 as libc::c_int as byte;
    tr = (*_flags.offset(index as isize) as libc::c_int & 0x7 as libc::c_int) as byte;
    was_trapped = 1 as libc::c_int as libc::c_uchar;
    match tr as libc::c_int {
        0 => {
            descend(
                b"you fell into a trap!\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
        }
        3 => {
            no_move += spread(3 as libc::c_int);
            msg(b"you are caught in a bear trap\0" as *const u8 as *const libc::c_char);
        }
        2 => {
            no_command += spread(5 as libc::c_int);
            player
                ._t
                ._t_flags = (player._t._t_flags as libc::c_int & !(0x4 as libc::c_int))
                as libc::c_short;
            msg(
                b"a %smist envelops you and you fall asleep\0" as *const u8
                    as *const libc::c_char,
                noterse(
                    b"strange white \0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                ),
            );
        }
        1 => {
            if swing(
                player._t._t_stats.s_lvl - 1 as libc::c_int,
                player._t._t_stats.s_arm,
                1 as libc::c_int,
            ) {
                player._t._t_stats.s_hpt -= roll(1 as libc::c_int, 6 as libc::c_int);
                if player._t._t_stats.s_hpt <= 0 as libc::c_int {
                    msg(b"an arrow killed you\0" as *const u8 as *const libc::c_char);
                    death('a' as i32 as libc::c_char);
                } else {
                    msg(
                        b"oh no! An arrow shot you\0" as *const u8 as *const libc::c_char,
                    );
                }
            } else {
                let mut arrow: *mut THING = 0 as *mut THING;
                arrow = new_item();
                if !arrow.is_null() {
                    (*arrow)._o._o_type = 0x18 as libc::c_int;
                    (*arrow)._o._o_which = 3 as libc::c_int;
                    init_weapon(arrow, 3 as libc::c_int as byte);
                    (*arrow)._o._o_count = 1 as libc::c_int;
                    memmove(
                        &mut (*arrow)._o._o_pos as *mut coord as *mut libc::c_void,
                        &mut player._t._t_pos as *mut coord as *const libc::c_void,
                        ::core::mem::size_of::<coord>() as libc::c_ulong,
                    );
                    fall(arrow, 0 as libc::c_int != 0);
                }
                msg(b"an arrow shoots past you\0" as *const u8 as *const libc::c_char);
            }
        }
        4 => {
            teleport();
            cur_mvaddch((*tc).y, (*tc).x, 0x4 as libc::c_int as byte);
            was_trapped = was_trapped.wrapping_add(1);
        }
        5 => {
            if swing(
                player._t._t_stats.s_lvl + 1 as libc::c_int,
                player._t._t_stats.s_arm,
                1 as libc::c_int,
            ) {
                player._t._t_stats.s_hpt -= roll(1 as libc::c_int, 4 as libc::c_int);
                if player._t._t_stats.s_hpt <= 0 as libc::c_int {
                    msg(
                        b"a poisoned dart killed you\0" as *const u8
                            as *const libc::c_char,
                    );
                    death('d' as i32 as libc::c_char);
                }
                if !(!(*cur_ring.as_mut_ptr().offset(0 as libc::c_int as isize))
                    .is_null()
                    && (**cur_ring.as_mut_ptr().offset(0 as libc::c_int as isize))
                        ._o
                        ._o_which == 2 as libc::c_int
                    || !(*cur_ring.as_mut_ptr().offset(1 as libc::c_int as isize))
                        .is_null()
                        && (**cur_ring.as_mut_ptr().offset(1 as libc::c_int as isize))
                            ._o
                            ._o_which == 2 as libc::c_int) && !save(0 as libc::c_int)
                {
                    chg_str(-(1 as libc::c_int));
                }
                msg(
                    b"a dart just hit you in the shoulder\0" as *const u8
                        as *const libc::c_char,
                );
            } else {
                msg(
                    b"a dart whizzes by your ear and vanishes\0" as *const u8
                        as *const libc::c_char,
                );
            }
        }
        _ => {}
    }
    flush_type();
    return tr;
}
#[no_mangle]
pub unsafe extern "C" fn descend(mut mesg: *mut libc::c_char) {
    level += 1;
    if *mesg as libc::c_int == 0 as libc::c_int {
        msg(b" \0" as *const u8 as *const libc::c_char);
    }
    new_level();
    msg(b"\0" as *const u8 as *const libc::c_char);
    msg(mesg);
    if !save(0o1 as libc::c_int) {
        msg(b"you are damaged by the fall\0" as *const u8 as *const libc::c_char);
        player._t._t_stats.s_hpt -= roll(1 as libc::c_int, 8 as libc::c_int);
        if player._t._t_stats.s_hpt <= 0 as libc::c_int {
            death('f' as i32 as libc::c_char);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn rndmove(mut who: *mut THING, mut newmv: *mut coord) {
    let mut current_block: u64;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut ch: byte = 0;
    let mut obj: *mut THING = 0 as *mut THING;
    (*newmv).y = (*who)._t._t_pos.y + rnd(3 as libc::c_int) - 1 as libc::c_int;
    y = (*newmv).y;
    (*newmv).x = (*who)._t._t_pos.x + rnd(3 as libc::c_int) - 1 as libc::c_int;
    x = (*newmv).x;
    if y == (*who)._t._t_pos.y && x == (*who)._t._t_pos.x {
        return;
    }
    if !(y < 1 as libc::c_int || y >= maxrow || (x < 0 as libc::c_int || x >= COLS)) {
        if diag_ok(&mut (*who)._t._t_pos, newmv) {
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
                        current_block = 16501597112000590545;
                    } else {
                        current_block = 4166486009154926805;
                    }
                } else {
                    current_block = 4166486009154926805;
                }
                match current_block {
                    16501597112000590545 => {}
                    _ => return,
                }
            }
        }
    }
    memmove(
        newmv as *mut libc::c_void,
        &mut (*who)._t._t_pos as *mut coord as *const libc::c_void,
        ::core::mem::size_of::<coord>() as libc::c_ulong,
    );
}
