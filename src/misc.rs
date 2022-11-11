use ::libc;
extern "C" {
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn flush_type();
    fn readchar() -> byte;
    static mut maxrow: libc::c_int;
    static mut s_guess: [*mut libc::c_char; 0];
    static mut p_guess: [*mut libc::c_char; 0];
    static mut r_guess: [*mut libc::c_char; 0];
    static mut ws_guess: [*mut libc::c_char; 0];
    static mut amulet: bool;
    static mut again: bool;
    static mut door_stop: bool;
    static mut firstmove: bool;
    static mut running: bool;
    static mut terse: bool;
    static mut was_trapped: libc::c_uchar;
    static mut p_know: [bool; 0];
    static mut r_know: [bool; 0];
    static mut s_know: [bool; 0];
    static mut ws_know: [bool; 0];
    static mut p_colors: [*mut libc::c_char; 0];
    static mut r_stones: [*mut libc::c_char; 0];
    static mut runch: libc::c_char;
    static mut ws_made: [*mut libc::c_char; 0];
    static mut a_class: [libc::c_int; 0];
    static mut count: libc::c_int;
    static mut food_left: libc::c_int;
    static mut hungry_state: libc::c_int;
    static mut inpack: libc::c_int;
    static mut level: libc::c_int;
    static mut no_command: libc::c_int;
    static mut cur_armor: *mut THING;
    static mut cur_ring: [*mut THING; 0];
    static mut cur_weapon: *mut THING;
    static mut lvl_obj: *mut THING;
    static mut mlist: *mut THING;
    static mut player: THING;
    static mut delta: coord;
    static mut oldpos: coord;
    static mut oldrp: *mut room;
    static mut max_stats: stats;
    static mut s_names: [array; 0];
    static mut fruit: [libc::c_char; 0];
    static mut prbuf: *mut libc::c_char;
    static mut _level: *mut byte;
    static mut _flags: *mut byte;
    fn start_run(runner: *mut coord);
    fn see_monst(mp: *mut THING) -> bool;
    fn fuse(func: Option::<unsafe extern "C" fn() -> ()>, time: libc::c_int);
    fn extinguish(func: Option::<unsafe extern "C" fn() -> ()>);
    fn nohaste();
    fn check_level();
    fn msg(fmt: *const libc::c_char, _: ...);
    fn noterse(str: *mut libc::c_char) -> *mut libc::c_char;
    fn list_detach(list: *mut *mut THING, item: *mut THING);
    fn discard(item: *mut THING) -> libc::c_int;
    fn rnd(range: libc::c_int) -> libc::c_int;
    fn wake_monster(y: libc::c_int, x: libc::c_int) -> *mut THING;
    fn moat(my: libc::c_int, mx: libc::c_int) -> *mut THING;
    fn get_item(purpose: *mut libc::c_char, type_0: libc::c_int) -> *mut THING;
    fn new_level();
    fn total_winner();
    fn cur_clear();
    fn cur_mvaddstr(r: libc::c_int, c: libc::c_int, s: *mut libc::c_char);
    fn cur_addch(chr: byte);
    fn cur_addstr(s: *mut libc::c_char);
    fn set_attr(bute: libc::c_int);
    fn wdump();
    fn wrestor();
    fn cur_beep();
    fn cur_move(row: libc::c_int, col: libc::c_int) -> libc::c_int;
    fn cur_inch() -> byte;
    fn getinfo(str: *mut libc::c_char, size: libc::c_int) -> libc::c_int;
    static mut COLS: libc::c_int;
}
pub type byte = libc::c_uchar;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct h_list {
    pub h_chstr: [byte; 6],
    pub h_desc: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct coord {
    pub x: libc::c_int,
    pub y: libc::c_int,
}
pub type str_t = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct array {
    pub storage: [libc::c_char; 21],
}
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
pub unsafe extern "C" fn tr_name(mut type_0: byte) -> *mut libc::c_char {
    match type_0 as libc::c_int {
        0 => {
            return b"a trapdoor\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
        }
        3 => {
            return b"a beartrap\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
        }
        2 => {
            return b"a sleeping gas trap\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
        }
        1 => {
            return b"an arrow trap\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
        }
        4 => {
            return b"a teleport trap\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
        }
        5 => {
            return b"a poison dart trap\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
        }
        _ => {}
    }
    msg(b"wierd trap: %d\0" as *const u8 as *const libc::c_char, type_0 as libc::c_int);
    return 0 as *mut libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn look(mut wakeup: bool) {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut ch: byte = 0;
    let mut pch: byte = 0;
    let mut index: libc::c_int = 0;
    let mut tp: *mut THING = 0 as *mut THING;
    let mut rp: *mut room = 0 as *mut room;
    let mut ey: libc::c_int = 0;
    let mut ex: libc::c_int = 0;
    let mut passcount: libc::c_int = 0 as libc::c_int;
    let mut pfl: byte = 0;
    let mut fp: *mut byte = 0 as *mut byte;
    let mut sy: libc::c_int = 0;
    let mut sx: libc::c_int = 0;
    let mut sumhero: libc::c_int = 0 as libc::c_int;
    let mut diffhero: libc::c_int = 0 as libc::c_int;
    rp = player._t._t_room;
    index = INDEX(player._t._t_pos.y, player._t._t_pos.x);
    pfl = *_flags.offset(index as isize);
    pch = *_level.offset(index as isize);
    if !_ce(&mut oldpos, &mut player._t._t_pos) {
        if !(player._t._t_flags as libc::c_int & 0x1 as libc::c_int != 0 as libc::c_int)
        {
            x = oldpos.x - 1 as libc::c_int;
            while x <= oldpos.x + 1 as libc::c_int {
                y = oldpos.y - 1 as libc::c_int;
                while y <= oldpos.y + 1 as libc::c_int {
                    if !(y == player._t._t_pos.y && x == player._t._t_pos.x
                        || offmap(y, x) as libc::c_int != 0)
                    {
                        cur_move(y, x);
                        ch = cur_inch();
                        if ch as libc::c_int == 0xfa as libc::c_int {
                            if (*oldrp).r_flags as libc::c_int
                                & (0x2 as libc::c_int | 0x1 as libc::c_int)
                                == 0x1 as libc::c_int
                            {
                                cur_addch(' ' as i32 as byte);
                            }
                        } else {
                            fp = &mut *_flags
                                .offset(
                                    (INDEX
                                        as unsafe extern "C" fn(
                                            libc::c_int,
                                            libc::c_int,
                                        ) -> libc::c_int)(y, x) as isize,
                                ) as *mut byte;
                            if (*fp as libc::c_int & 0x20 as libc::c_int != 0
                                || *fp as libc::c_int & 0x40 as libc::c_int != 0)
                                && ch as libc::c_int != 0xb1 as libc::c_int
                                && ch as libc::c_int != 0xf0 as libc::c_int
                                && *fp as libc::c_int & 0xf as libc::c_int
                                    == pfl as libc::c_int & 0xf as libc::c_int
                            {
                                cur_addch(0xb1 as libc::c_int as byte);
                            }
                        }
                    }
                    y += 1;
                }
                x += 1;
            }
        }
        oldpos = player._t._t_pos;
        oldrp = rp;
    }
    ey = player._t._t_pos.y + 1 as libc::c_int;
    ex = player._t._t_pos.x + 1 as libc::c_int;
    sx = player._t._t_pos.x - 1 as libc::c_int;
    sy = player._t._t_pos.y - 1 as libc::c_int;
    if door_stop as libc::c_int != 0 && !firstmove && running as libc::c_int != 0 {
        sumhero = player._t._t_pos.y + player._t._t_pos.x;
        diffhero = player._t._t_pos.y - player._t._t_pos.x;
    }
    y = sy;
    while y <= ey {
        if y > 0 as libc::c_int && y < maxrow {
            let mut current_block_57: u64;
            x = sx;
            while x <= ex {
                if !(x <= 0 as libc::c_int || x >= COLS) {
                    if !(player._t._t_flags as libc::c_int & 0x1 as libc::c_int
                        != 0 as libc::c_int)
                    {
                        if y == player._t._t_pos.y && x == player._t._t_pos.x {
                            current_block_57 = 18377268871191777778;
                        } else {
                            current_block_57 = 5529461102203738653;
                        }
                    } else if y != player._t._t_pos.y || x != player._t._t_pos.x {
                        current_block_57 = 18377268871191777778;
                    } else {
                        current_block_57 = 5529461102203738653;
                    }
                    match current_block_57 {
                        18377268871191777778 => {}
                        _ => {
                            index = INDEX(y, x);
                            fp = &mut *_flags.offset(index as isize) as *mut byte;
                            ch = *_level.offset(index as isize);
                            if pch as libc::c_int != 0xce as libc::c_int
                                && ch as libc::c_int != 0xce as libc::c_int
                            {
                                if pfl as libc::c_int & 0x40 as libc::c_int
                                    != *fp as libc::c_int & 0x40 as libc::c_int
                                {
                                    if pfl as libc::c_int & 0x20 as libc::c_int == 0
                                        && *fp as libc::c_int & 0x20 as libc::c_int == 0
                                    {
                                        current_block_57 = 18377268871191777778;
                                    } else {
                                        current_block_57 = 9441801433784995173;
                                    }
                                } else if *fp as libc::c_int & 0x40 as libc::c_int != 0
                                    && *fp as libc::c_int & 0xf as libc::c_int
                                        != pfl as libc::c_int & 0xf as libc::c_int
                                {
                                    current_block_57 = 18377268871191777778;
                                } else {
                                    current_block_57 = 9441801433784995173;
                                }
                            } else {
                                current_block_57 = 9441801433784995173;
                            }
                            match current_block_57 {
                                18377268871191777778 => {}
                                _ => {
                                    tp = moat(y, x);
                                    if !tp.is_null() {
                                        if player._t._t_flags as libc::c_int & 0x2 as libc::c_int
                                            != 0 as libc::c_int
                                            && (*tp)._t._t_flags as libc::c_int & 0x10 as libc::c_int
                                                != 0 as libc::c_int
                                        {
                                            if door_stop as libc::c_int != 0 && !firstmove {
                                                running = 0 as libc::c_int != 0;
                                            }
                                            current_block_57 = 18377268871191777778;
                                        } else {
                                            if wakeup {
                                                wake_monster(y, x);
                                            }
                                            if (*tp)._t._t_oldch as libc::c_int != ' ' as i32
                                                || (*rp).r_flags as libc::c_int & 0x1 as libc::c_int == 0
                                                    && !(player._t._t_flags as libc::c_int & 0x1 as libc::c_int
                                                        != 0 as libc::c_int)
                                            {
                                                (*tp)._t._t_oldch = *_level.offset(index as isize);
                                            }
                                            if see_monst(tp) {
                                                ch = (*tp)._t._t_disguise;
                                            }
                                            current_block_57 = 17233182392562552756;
                                        }
                                    } else {
                                        current_block_57 = 17233182392562552756;
                                    }
                                    match current_block_57 {
                                        18377268871191777778 => {}
                                        _ => {
                                            if ch as libc::c_int != 0xb1 as libc::c_int
                                                && *fp as libc::c_int
                                                    & (0x40 as libc::c_int | 0x20 as libc::c_int) != 0
                                            {
                                                if ch as libc::c_int != 0x8 as libc::c_int {
                                                    set_attr(14 as libc::c_int);
                                                }
                                            }
                                            cur_move(y, x);
                                            cur_addch(ch);
                                            set_attr(0 as libc::c_int);
                                            if door_stop as libc::c_int != 0 && !firstmove
                                                && running as libc::c_int != 0
                                            {
                                                match runch as libc::c_int {
                                                    104 => {
                                                        if x == ex {
                                                            current_block_57 = 18377268871191777778;
                                                        } else {
                                                            current_block_57 = 11796148217846552555;
                                                        }
                                                    }
                                                    106 => {
                                                        if y == sy {
                                                            current_block_57 = 18377268871191777778;
                                                        } else {
                                                            current_block_57 = 11796148217846552555;
                                                        }
                                                    }
                                                    107 => {
                                                        if y == ey {
                                                            current_block_57 = 18377268871191777778;
                                                        } else {
                                                            current_block_57 = 11796148217846552555;
                                                        }
                                                    }
                                                    108 => {
                                                        if x == sx {
                                                            current_block_57 = 18377268871191777778;
                                                        } else {
                                                            current_block_57 = 11796148217846552555;
                                                        }
                                                    }
                                                    121 => {
                                                        if y + x - sumhero >= 1 as libc::c_int {
                                                            current_block_57 = 18377268871191777778;
                                                        } else {
                                                            current_block_57 = 11796148217846552555;
                                                        }
                                                    }
                                                    117 => {
                                                        if y - x - diffhero >= 1 as libc::c_int {
                                                            current_block_57 = 18377268871191777778;
                                                        } else {
                                                            current_block_57 = 11796148217846552555;
                                                        }
                                                    }
                                                    110 => {
                                                        if y + x - sumhero <= -(1 as libc::c_int) {
                                                            current_block_57 = 18377268871191777778;
                                                        } else {
                                                            current_block_57 = 11796148217846552555;
                                                        }
                                                    }
                                                    98 => {
                                                        if y - x - diffhero <= -(1 as libc::c_int) {
                                                            current_block_57 = 18377268871191777778;
                                                        } else {
                                                            current_block_57 = 11796148217846552555;
                                                        }
                                                    }
                                                    _ => {
                                                        current_block_57 = 11796148217846552555;
                                                    }
                                                }
                                                match current_block_57 {
                                                    18377268871191777778 => {}
                                                    _ => {
                                                        match ch as libc::c_int {
                                                            206 => {
                                                                if x == player._t._t_pos.x || y == player._t._t_pos.y {
                                                                    running = 0 as libc::c_int != 0;
                                                                }
                                                            }
                                                            177 => {
                                                                if x == player._t._t_pos.x || y == player._t._t_pos.y {
                                                                    passcount += 1;
                                                                }
                                                            }
                                                            250 | 186 | 205 | 201 | 187 | 200 | 188 | 32 => {}
                                                            _ => {
                                                                running = 0 as libc::c_int != 0;
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                x += 1;
            }
        }
        y += 1;
    }
    if door_stop as libc::c_int != 0 && !firstmove && passcount > 1 as libc::c_int {
        running = 0 as libc::c_int != 0;
    }
    cur_move(player._t._t_pos.y, player._t._t_pos.x);
    if *_flags.offset(INDEX(player._t._t_pos.y, player._t._t_pos.x) as isize)
        as libc::c_int & 0x40 as libc::c_int != 0
        || was_trapped as libc::c_int > 1 as libc::c_int
        || *_flags.offset(INDEX(player._t._t_pos.y, player._t._t_pos.x) as isize)
            as libc::c_int & 0x20 as libc::c_int != 0
    {
        set_attr(14 as libc::c_int);
    }
    cur_addch(0x1 as libc::c_int as byte);
    set_attr(0 as libc::c_int);
    if was_trapped != 0 {
        cur_beep();
        was_trapped = 0 as libc::c_int as libc::c_uchar;
    }
}
#[no_mangle]
pub unsafe extern "C" fn find_obj(mut y: libc::c_int, mut x: libc::c_int) -> *mut THING {
    let mut op: *mut THING = 0 as *mut THING;
    op = lvl_obj;
    while !op.is_null() {
        if (*op)._o._o_pos.y == y && (*op)._o._o_pos.x == x {
            return op;
        }
        op = (*op)._t._l_next;
    }
    return 0 as *mut THING;
}
#[no_mangle]
pub unsafe extern "C" fn eat() {
    let mut obj: *mut THING = 0 as *mut THING;
    obj = get_item(
        b"eat\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0x5 as libc::c_int,
    );
    if obj.is_null() {
        return;
    }
    if (*obj)._o._o_type != 0x5 as libc::c_int {
        msg(
            b"ugh, you would get ill if you ate that\0" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    inpack -= 1;
    (*obj)._o._o_count -= 1;
    if (*obj)._o._o_count < 1 as libc::c_int {
        list_detach(&mut player._t._t_pack, obj);
        discard(obj);
    }
    if food_left < 0 as libc::c_int {
        food_left = 0 as libc::c_int;
    }
    if food_left > 2000 as libc::c_int - 20 as libc::c_int {
        no_command += 2 as libc::c_int + rnd(5 as libc::c_int);
    }
    food_left
        += spread(1300 as libc::c_int) - 200 as libc::c_int + rnd(400 as libc::c_int);
    if food_left > 2000 as libc::c_int {
        food_left = 2000 as libc::c_int;
    }
    hungry_state = 0 as libc::c_int;
    if obj == cur_weapon {
        cur_weapon = 0 as *mut THING;
    }
    if (*obj)._o._o_which == 1 as libc::c_int {
        msg(
            b"my, that was a yummy %s\0" as *const u8 as *const libc::c_char,
            fruit.as_mut_ptr(),
        );
    } else if rnd(100 as libc::c_int) > 70 as libc::c_int {
        player._t._t_stats.s_exp += 1;
        msg(b"yuk, this food tastes awful\0" as *const u8 as *const libc::c_char);
        check_level();
    } else {
        msg(b"yum, that tasted good\0" as *const u8 as *const libc::c_char);
    }
    if no_command != 0 {
        msg(b"You feel bloated and fall asleep\0" as *const u8 as *const libc::c_char);
    }
}
#[no_mangle]
pub unsafe extern "C" fn chg_str(mut amt: libc::c_int) {
    let mut comp: str_t = 0;
    if amt == 0 as libc::c_int {
        return;
    }
    add_str(&mut player._t._t_stats.s_str, amt);
    comp = player._t._t_stats.s_str;
    if !(*cur_ring.as_mut_ptr().offset(0 as libc::c_int as isize)).is_null()
        && (**cur_ring.as_mut_ptr().offset(0 as libc::c_int as isize))._o._o_which
            == 1 as libc::c_int
    {
        add_str(
            &mut comp,
            -((**cur_ring.as_mut_ptr().offset(0 as libc::c_int as isize))._o._o_ac
                as libc::c_int),
        );
    }
    if !(*cur_ring.as_mut_ptr().offset(1 as libc::c_int as isize)).is_null()
        && (**cur_ring.as_mut_ptr().offset(1 as libc::c_int as isize))._o._o_which
            == 1 as libc::c_int
    {
        add_str(
            &mut comp,
            -((**cur_ring.as_mut_ptr().offset(1 as libc::c_int as isize))._o._o_ac
                as libc::c_int),
        );
    }
    if comp > max_stats.s_str {
        max_stats.s_str = comp;
    }
}
#[no_mangle]
pub unsafe extern "C" fn add_str(mut sp: *mut str_t, mut amt: libc::c_int) {
    *sp = (*sp as libc::c_uint).wrapping_add(amt as libc::c_uint) as str_t as str_t;
    if *sp < 3 as libc::c_int as libc::c_uint {
        *sp = 3 as libc::c_int as str_t;
    } else if *sp > 31 as libc::c_int as libc::c_uint {
        *sp = 31 as libc::c_int as str_t;
    }
}
#[no_mangle]
pub unsafe extern "C" fn add_haste(mut potion: bool) -> bool {
    if player._t._t_flags as libc::c_int & 0x4000 as libc::c_int != 0 as libc::c_int {
        no_command += rnd(8 as libc::c_int);
        player
            ._t
            ._t_flags = (player._t._t_flags as libc::c_int & !(0x4 as libc::c_int))
            as libc::c_short;
        extinguish(
            ::core::mem::transmute::<
                Option::<unsafe extern "C" fn() -> ()>,
                Option::<unsafe extern "C" fn() -> ()>,
            >(Some(nohaste as unsafe extern "C" fn() -> ())),
        );
        player
            ._t
            ._t_flags = (player._t._t_flags as libc::c_int & !(0x4000 as libc::c_int))
            as libc::c_short;
        msg(b"you faint from exhaustion\0" as *const u8 as *const libc::c_char);
        return 0 as libc::c_int != 0;
    } else {
        player
            ._t
            ._t_flags = (player._t._t_flags as libc::c_int | 0x4000 as libc::c_int)
            as libc::c_short;
        if potion {
            fuse(
                ::core::mem::transmute::<
                    Option::<unsafe extern "C" fn() -> ()>,
                    Option::<unsafe extern "C" fn() -> ()>,
                >(Some(nohaste as unsafe extern "C" fn() -> ())),
                rnd(4 as libc::c_int) + 10 as libc::c_int,
            );
        }
        return 1 as libc::c_int != 0;
    };
}
#[no_mangle]
pub unsafe extern "C" fn aggravate() {
    let mut mi: *mut THING = 0 as *mut THING;
    mi = mlist;
    while !mi.is_null() {
        start_run(&mut (*mi)._t._t_pos);
        mi = (*mi)._t._l_next;
    }
}
#[no_mangle]
pub unsafe extern "C" fn vowelstr(mut str: *mut libc::c_char) -> *mut libc::c_char {
    match *str as libc::c_int {
        97 | 65 | 101 | 69 | 105 | 73 | 111 | 79 | 117 | 85 => {
            return b"n\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
        _ => return b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    };
}
#[no_mangle]
pub unsafe extern "C" fn is_current(mut obj: *mut THING) -> bool {
    if obj.is_null() {
        return 0 as libc::c_int != 0;
    }
    if obj == cur_armor || obj == cur_weapon
        || obj == *cur_ring.as_mut_ptr().offset(0 as libc::c_int as isize)
        || obj == *cur_ring.as_mut_ptr().offset(1 as libc::c_int as isize)
    {
        msg(b"That's already in use\0" as *const u8 as *const libc::c_char);
        return 1 as libc::c_int != 0;
    }
    return 0 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn get_dir() -> bool {
    let mut ch: libc::c_int = 0;
    if again {
        return 1 as libc::c_int != 0;
    }
    msg(b"which direction? \0" as *const u8 as *const libc::c_char);
    loop {
        ch = readchar() as libc::c_int;
        if ch == 27 as libc::c_int {
            msg(b"\0" as *const u8 as *const libc::c_char);
            return 0 as libc::c_int != 0;
        }
        if !(find_dir(ch as byte, &mut delta) as libc::c_int == 0 as libc::c_int) {
            break;
        }
    }
    msg(b"\0" as *const u8 as *const libc::c_char);
    if player._t._t_flags as libc::c_int & 0x100 as libc::c_int != 0 as libc::c_int
        && rnd(5 as libc::c_int) == 0 as libc::c_int
    {
        loop {
            delta.y = rnd(3 as libc::c_int) - 1 as libc::c_int;
            delta.x = rnd(3 as libc::c_int) - 1 as libc::c_int;
            if !(delta.y == 0 as libc::c_int && delta.x == 0 as libc::c_int) {
                break;
            }
        }
    }
    return 1 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn find_dir(mut ch: byte, mut cp: *mut coord) -> bool {
    let mut gotit: bool = false;
    gotit = 1 as libc::c_int != 0;
    match ch as libc::c_int {
        104 | 72 => {
            (*cp).y = 0 as libc::c_int;
            (*cp).x = -(1 as libc::c_int);
        }
        106 | 74 => {
            (*cp).y = 1 as libc::c_int;
            (*cp).x = 0 as libc::c_int;
        }
        107 | 75 => {
            (*cp).y = -(1 as libc::c_int);
            (*cp).x = 0 as libc::c_int;
        }
        108 | 76 => {
            (*cp).y = 0 as libc::c_int;
            (*cp).x = 1 as libc::c_int;
        }
        121 | 89 => {
            (*cp).y = -(1 as libc::c_int);
            (*cp).x = -(1 as libc::c_int);
        }
        117 | 85 => {
            (*cp).y = -(1 as libc::c_int);
            (*cp).x = 1 as libc::c_int;
        }
        98 | 66 => {
            (*cp).y = 1 as libc::c_int;
            (*cp).x = -(1 as libc::c_int);
        }
        110 | 78 => {
            (*cp).y = 1 as libc::c_int;
            (*cp).x = 1 as libc::c_int;
        }
        _ => {
            gotit = 0 as libc::c_int != 0;
        }
    }
    return gotit;
}
#[no_mangle]
pub unsafe extern "C" fn sign(mut nm: libc::c_int) -> libc::c_int {
    if nm < 0 as libc::c_int {
        return -(1 as libc::c_int)
    } else {
        return (nm > 0 as libc::c_int) as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn spread(mut nm: libc::c_int) -> libc::c_int {
    return nm - nm / 10 as libc::c_int + rnd(nm / 5 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn call_it(mut know: bool, mut guess: *mut *mut libc::c_char) {
    if know as libc::c_int != 0 && **guess as libc::c_int != 0 {
        **guess = '\0' as i32 as libc::c_char;
    } else if !know && **guess as libc::c_int == '\0' as i32 {
        msg(
            b"%scall it? \0" as *const u8 as *const libc::c_char,
            noterse(
                b"what do you want to \0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            ),
        );
        getinfo(prbuf, 20 as libc::c_int);
        if *prbuf as libc::c_int != 27 as libc::c_int {
            strcpy(*guess, prbuf);
        }
        msg(b"\0" as *const u8 as *const libc::c_char);
    }
}
#[no_mangle]
pub unsafe extern "C" fn step_ok(mut ch: byte) -> bool {
    match ch as libc::c_int {
        32 | 186 | 205 | 201 | 187 | 200 | 188 => return 0 as libc::c_int != 0,
        _ => return !(ch as libc::c_int >= 'A' as i32 && ch as libc::c_int <= 'Z' as i32),
    };
}
#[no_mangle]
pub unsafe extern "C" fn goodch(mut obj: *mut THING) -> libc::c_char {
    let mut ch: libc::c_char = '$' as i32 as libc::c_char;
    if (*obj)._o._o_flags as libc::c_int & 0x1 as libc::c_int != 0 {
        ch = '~' as i32 as libc::c_char;
    }
    match (*obj)._o._o_type {
        8 => {
            if (*obj)._o._o_ac as libc::c_int
                > *a_class.as_mut_ptr().offset((*obj)._o._o_which as isize)
            {
                ch = '~' as i32 as libc::c_char;
            }
        }
        24 => {
            if (*obj)._o._o_hplus < 0 as libc::c_int
                || (*obj)._o._o_dplus < 0 as libc::c_int
            {
                ch = '~' as i32 as libc::c_char;
            }
        }
        13 => {
            match (*obj)._o._o_which {
                3 | 10 | 12 => {
                    ch = '~' as i32 as libc::c_char;
                }
                _ => {}
            }
        }
        173 => {
            match (*obj)._o._o_which {
                0 | 1 | 2 | 12 => {
                    ch = '~' as i32 as libc::c_char;
                }
                _ => {}
            }
        }
        231 => {
            match (*obj)._o._o_which {
                7 | 12 => {
                    ch = '~' as i32 as libc::c_char;
                }
                _ => {}
            }
        }
        9 => {
            match (*obj)._o._o_which {
                0 | 1 | 8 | 7 => {
                    if ((*obj)._o._o_ac as libc::c_int) < 0 as libc::c_int {
                        ch = '~' as i32 as libc::c_char;
                    }
                }
                6 | 11 => {
                    ch = '~' as i32 as libc::c_char;
                }
                _ => {}
            }
        }
        _ => {}
    }
    return ch;
}
#[no_mangle]
pub unsafe extern "C" fn help(mut helpscr: *mut h_list) {
    let mut hcount: libc::c_int = 0 as libc::c_int;
    let mut hrow: libc::c_int = 0;
    let mut hcol: libc::c_int = 0;
    let mut isfull: libc::c_int = 0;
    let mut answer: byte = 0 as libc::c_int as byte;
    wdump();
    while *(*helpscr).h_desc as libc::c_int != 0
        && answer as libc::c_int != 27 as libc::c_int
    {
        isfull = 0 as libc::c_int;
        if hcount
            % (if terse as libc::c_int != 0 {
                23 as libc::c_int
            } else {
                46 as libc::c_int
            }) == 0 as libc::c_int
        {
            cur_clear();
        }
        hcol = 0 as libc::c_int;
        if terse {
            hrow = hcount % 23 as libc::c_int;
            if hrow == 22 as libc::c_int {
                isfull = 1 as libc::c_int;
            }
        } else {
            hrow = hcount % 46 as libc::c_int / 2 as libc::c_int;
            if hcount % 2 as libc::c_int != 0 {
                hcol = 40 as libc::c_int;
            }
            if hrow == 22 as libc::c_int && hcol == 40 as libc::c_int {
                isfull = 1 as libc::c_int;
            }
        }
        cur_move(hrow, hcol);
        cur_addstr(((*helpscr).h_chstr).as_mut_ptr() as *mut libc::c_char);
        cur_addstr((*helpscr).h_desc);
        helpscr = helpscr.offset(1);
        if *(*helpscr).h_desc as libc::c_int == 0 as libc::c_int || isfull != 0 {
            if *(*helpscr).h_desc as libc::c_int == 0 as libc::c_int {
                cur_mvaddstr(
                    24 as libc::c_int,
                    0 as libc::c_int,
                    b"--press space to continue--\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                );
            } else if terse {
                cur_mvaddstr(
                    24 as libc::c_int,
                    0 as libc::c_int,
                    b"--Space for more, Esc to continue--\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                );
            } else {
                cur_mvaddstr(
                    24 as libc::c_int,
                    0 as libc::c_int,
                    b"--Press space for more, Esc to continue--\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                );
            }
            loop {
                answer = readchar();
                if !(answer as libc::c_int != ' ' as i32
                    && answer as libc::c_int != 27 as libc::c_int)
                {
                    break;
                }
            }
        }
        hcount += 1;
    }
    wrestor();
}
#[no_mangle]
pub unsafe extern "C" fn DISTANCE(
    mut y1: libc::c_int,
    mut x1: libc::c_int,
    mut y2: libc::c_int,
    mut x2: libc::c_int,
) -> libc::c_int {
    let mut dx: libc::c_int = 0;
    let mut dy: libc::c_int = 0;
    dx = x1 - x2;
    dy = y1 - y2;
    return dx * dx + dy * dy;
}
#[no_mangle]
pub unsafe extern "C" fn _ce(mut a: *mut coord, mut b: *mut coord) -> bool {
    return (*a).x == (*b).x && (*a).y == (*b).y;
}
#[no_mangle]
pub unsafe extern "C" fn INDEX(mut y: libc::c_int, mut x: libc::c_int) -> libc::c_int {
    return x * (maxrow - 1 as libc::c_int) + y - 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn offmap(mut y: libc::c_int, mut x: libc::c_int) -> bool {
    return y < 1 as libc::c_int || y >= maxrow || x < 0 as libc::c_int || x >= COLS;
}
#[no_mangle]
pub unsafe extern "C" fn winat(mut y: libc::c_int, mut x: libc::c_int) -> byte {
    return (if !(moat(y, x)).is_null() {
        (*moat(y, x))._t._t_disguise as libc::c_int
    } else {
        *_level.offset(INDEX(y, x) as isize) as libc::c_int
    }) as byte;
}
#[no_mangle]
pub unsafe extern "C" fn search() {
    let mut y: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut fp: *mut byte = 0 as *mut byte;
    let mut ey: libc::c_int = 0;
    let mut ex: libc::c_int = 0;
    if player._t._t_flags as libc::c_int & 0x1 as libc::c_int != 0 as libc::c_int {
        return;
    }
    ey = player._t._t_pos.y + 1 as libc::c_int;
    ex = player._t._t_pos.x + 1 as libc::c_int;
    y = player._t._t_pos.y - 1 as libc::c_int;
    while y <= ey {
        x = player._t._t_pos.x - 1 as libc::c_int;
        while x <= ex {
            if !(y == player._t._t_pos.y && x == player._t._t_pos.x
                || offmap(y, x) as libc::c_int != 0)
            {
                // Rust port: Fixed likely C2Rust confusion about the macro
                fp = &mut *_flags.offset(INDEX(y, x) as isize) as *mut byte;
                if *fp as libc::c_int & 0x10 as libc::c_int == 0 {
                    match *_level.offset(INDEX(y, x) as isize) as libc::c_int {
                        186 | 205 | 201 | 187 | 200 | 188 => {
                            if !(rnd(5 as libc::c_int) != 0 as libc::c_int) {
                                *_level
                                    .offset(INDEX(y, x) as isize) = 0xce as libc::c_int as byte;
                                *fp = (*fp as libc::c_int | 0x10 as libc::c_int) as byte;
                                running = 0 as libc::c_int != 0;
                                count = running as libc::c_int;
                            }
                        }
                        250 => {
                            if !(rnd(2 as libc::c_int) != 0 as libc::c_int) {
                                *_level
                                    .offset(INDEX(y, x) as isize) = 0x4 as libc::c_int as byte;
                                *fp = (*fp as libc::c_int | 0x10 as libc::c_int) as byte;
                                running = 0 as libc::c_int != 0;
                                count = running as libc::c_int;
                                msg(
                                    b"you found %s\0" as *const u8 as *const libc::c_char,
                                    tr_name((*fp as libc::c_int & 0x7 as libc::c_int) as byte),
                                );
                            }
                        }
                        _ => {}
                    }
                }
            }
            x += 1;
        }
        y += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn d_level() {
    if *_level.offset(INDEX(player._t._t_pos.y, player._t._t_pos.x) as isize)
        as libc::c_int != 0xf0 as libc::c_int
    {
        msg(b"I see no way down\0" as *const u8 as *const libc::c_char);
    } else {
        level += 1;
        new_level();
    };
}
#[no_mangle]
pub unsafe extern "C" fn u_level() {
    if *_level.offset(INDEX(player._t._t_pos.y, player._t._t_pos.x) as isize)
        as libc::c_int == 0xf0 as libc::c_int
    {
        if amulet {
            level -= 1;
            if level == 0 as libc::c_int {
                total_winner();
            }
            new_level();
            msg(
                b"you feel a wrenching sensation in your gut\0" as *const u8
                    as *const libc::c_char,
            );
        } else {
            msg(b"your way is magically blocked\0" as *const u8 as *const libc::c_char);
        }
    } else {
        msg(b"I see no way up\0" as *const u8 as *const libc::c_char);
    };
}
#[no_mangle]
pub unsafe extern "C" fn call() {
    let mut obj: *mut THING = 0 as *mut THING;
    let mut guess: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut elsewise: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut know: *mut bool = 0 as *mut bool;
    obj = get_item(
        b"call\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        -(1 as libc::c_int),
    );
    if obj.is_null() {
        return;
    }
    match (*obj)._o._o_type {
        9 => {
            guess = r_guess.as_mut_ptr();
            know = r_know.as_mut_ptr();
            elsewise = if **guess.offset((*obj)._o._o_which as isize) as libc::c_int
                != '\0' as i32
            {
                *guess.offset((*obj)._o._o_which as isize)
            } else {
                *r_stones.as_mut_ptr().offset((*obj)._o._o_which as isize)
            };
        }
        173 => {
            guess = p_guess.as_mut_ptr();
            know = p_know.as_mut_ptr();
            elsewise = if **guess.offset((*obj)._o._o_which as isize) as libc::c_int
                != '\0' as i32
            {
                *guess.offset((*obj)._o._o_which as isize)
            } else {
                *p_colors.as_mut_ptr().offset((*obj)._o._o_which as isize)
            };
        }
        13 => {
            guess = s_guess.as_mut_ptr();
            know = s_know.as_mut_ptr();
            elsewise = if **guess.offset((*obj)._o._o_which as isize) as libc::c_int
                != '\0' as i32
            {
                *guess.offset((*obj)._o._o_which as isize)
            } else {
                ((*s_names.as_mut_ptr().offset((*obj)._o._o_which as isize)).storage)
                    .as_mut_ptr()
            };
        }
        231 => {
            guess = ws_guess.as_mut_ptr();
            know = ws_know.as_mut_ptr();
            elsewise = if **guess.offset((*obj)._o._o_which as isize) as libc::c_int
                != '\0' as i32
            {
                *guess.offset((*obj)._o._o_which as isize)
            } else {
                *ws_made.as_mut_ptr().offset((*obj)._o._o_which as isize)
            };
        }
        _ => {
            msg(b"you can't call that anything\0" as *const u8 as *const libc::c_char);
            return;
        }
    }
    if *know.offset((*obj)._o._o_which as isize) {
        msg(b"that has already been identified\0" as *const u8 as *const libc::c_char);
        return;
    }
    msg(b"Was called \"%s\"\0" as *const u8 as *const libc::c_char, elsewise);
    msg(b"what do you want to call it? \0" as *const u8 as *const libc::c_char);
    getinfo(prbuf, 20 as libc::c_int);
    if *prbuf as libc::c_int != 0 && *prbuf as libc::c_int != 27 as libc::c_int {
        strcpy(*guess.offset((*obj)._o._o_which as isize), prbuf);
    }
    msg(b"\0" as *const u8 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn do_macro(mut buf: *mut libc::c_char, mut sz: libc::c_int) {
    let mut cp: *mut libc::c_char = prbuf;
    msg(b"F9 was %s, enter new macro: \0" as *const u8 as *const libc::c_char, buf);
    if getinfo(prbuf, sz - 1 as libc::c_int) != 27 as libc::c_int {
        loop {
            if *cp as libc::c_int != 'F' as i32 & 0o37 as libc::c_int {
                let fresh0 = buf;
                buf = buf.offset(1);
                *fresh0 = *cp;
            }
            let fresh1 = cp;
            cp = cp.offset(1);
            if !(*fresh1 != 0) {
                break;
            }
        }
    }
    msg(b"\0" as *const u8 as *const libc::c_char);
    flush_type();
}
