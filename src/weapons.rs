use ::libc;
extern "C" {
    fn memmove(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
        -> *mut libc::c_void;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn md_nanosleep(nanoseconds: libc::c_long);
    static mut after: bool;
    static mut a_names: [*mut libc::c_char; 0];
    static mut w_names: [*mut libc::c_char; 0];
    static mut group: libc::c_int;
    static mut inpack: libc::c_int;
    static mut cur_weapon: *mut THING;
    static mut lvl_obj: *mut THING;
    static mut player: THING;
    static mut _level: *mut byte;
    static mut _flags: *mut byte;
    fn cansee(y: libc::c_int, x: libc::c_int) -> bool;
    fn fight(mp: *mut coord, mn: libc::c_char, weap: *mut THING, thrown: bool) -> bool;
    fn ifterse(tfmt: *const libc::c_char, fmt: *const libc::c_char, _: ...);
    fn msg(fmt: *const libc::c_char, _: ...);
    fn noterse(str: *mut libc::c_char) -> *mut libc::c_char;
    fn new_item() -> *mut THING;
    fn list_detach(list: *mut *mut THING, item: *mut THING);
    fn list_attach(list: *mut *mut THING, item: *mut THING);
    fn discard(item: *mut THING) -> libc::c_int;
    fn rnd(range: libc::c_int) -> libc::c_int;
    fn find_obj(y: libc::c_int, x: libc::c_int) -> *mut THING;
    fn is_current(obj: *mut THING) -> bool;
    fn step_ok(ch: byte) -> bool;
    fn _ce(a: *mut coord, b: *mut coord) -> bool;
    fn offmap(y: libc::c_int, x: libc::c_int) -> bool;
    fn winat(y: libc::c_int, x: libc::c_int) -> byte;
    fn INDEX(y: libc::c_int, x: libc::c_int) -> libc::c_int;
    fn moat(my: libc::c_int, mx: libc::c_int) -> *mut THING;
    fn get_item(purpose: *mut libc::c_char, type_0: libc::c_int) -> *mut THING;
    fn pack_char(obj: *mut THING) -> byte;
    fn inv_name(obj: *mut THING, drop_0: bool) -> *mut libc::c_char;
    fn can_drop(op: *mut THING) -> bool;
    fn cur_refresh();
    fn cur_mvaddch(r: libc::c_int, c: libc::c_int, chr: byte);
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
pub struct init_weps {
    pub iw_dam: *mut libc::c_char,
    pub iw_hrl: *mut libc::c_char,
    pub iw_launch: libc::c_char,
    pub iw_flags: libc::c_int,
}
static mut init_dam: [init_weps; 10] = [
    {
        let mut init = init_weps {
            iw_dam: b"2d4\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            iw_hrl: b"1d3\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            iw_launch: 100 as libc::c_int as libc::c_char,
            iw_flags: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = init_weps {
            iw_dam: b"3d4\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            iw_hrl: b"1d2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            iw_launch: 100 as libc::c_int as libc::c_char,
            iw_flags: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = init_weps {
            iw_dam: b"1d1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            iw_hrl: b"1d1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            iw_launch: 100 as libc::c_int as libc::c_char,
            iw_flags: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = init_weps {
            iw_dam: b"1d1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            iw_hrl: b"2d3\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            iw_launch: 2 as libc::c_int as libc::c_char,
            iw_flags: 0x20 as libc::c_int | 0x10 as libc::c_int,
        };
        init
    },
    {
        let mut init = init_weps {
            iw_dam: b"1d6\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            iw_hrl: b"1d4\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            iw_launch: 100 as libc::c_int as libc::c_char,
            iw_flags: 0x10 as libc::c_int,
        };
        init
    },
    {
        let mut init = init_weps {
            iw_dam: b"4d4\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            iw_hrl: b"1d2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            iw_launch: 100 as libc::c_int as libc::c_char,
            iw_flags: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = init_weps {
            iw_dam: b"1d1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            iw_hrl: b"1d3\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            iw_launch: 100 as libc::c_int as libc::c_char,
            iw_flags: 0x20 as libc::c_int | 0x10 as libc::c_int,
        };
        init
    },
    {
        let mut init = init_weps {
            iw_dam: b"1d1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            iw_hrl: b"1d1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            iw_launch: 100 as libc::c_int as libc::c_char,
            iw_flags: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = init_weps {
            iw_dam: b"1d2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            iw_hrl: b"2d5\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            iw_launch: 7 as libc::c_int as libc::c_char,
            iw_flags: 0x20 as libc::c_int | 0x10 as libc::c_int,
        };
        init
    },
    {
        let mut init = init_weps {
            iw_dam: b"2d3\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            iw_hrl: b"1d6\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            iw_launch: 100 as libc::c_int as libc::c_char,
            iw_flags: 0x10 as libc::c_int,
        };
        init
    },
];
#[no_mangle]
pub unsafe extern "C" fn missile(mut ydelta: libc::c_int, mut xdelta: libc::c_int) {
    let mut obj: *mut THING = 0 as *mut THING;
    let mut nitem: *mut THING = 0 as *mut THING;
    obj = get_item(
        b"throw\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0x18 as libc::c_int,
    );
    if obj.is_null() {
        return;
    }
    if !can_drop(obj) || is_current(obj) as libc::c_int != 0 {
        return;
    }
    loop {
        if (*obj)._o._o_count < 2 as libc::c_int {
            list_detach(&mut player._t._t_pack, obj);
            inpack -= 1;
            break;
        } else {
            nitem = new_item();
            if nitem.is_null() {
                (*obj)._o._o_count = 1 as libc::c_int;
                msg(b"something in your pack explodes!!!\0" as *const u8 as *const libc::c_char);
            } else {
                (*obj)._o._o_count -= 1;
                if (*obj)._o._o_group == 0 as libc::c_int {
                    inpack -= 1;
                }
                memmove(
                    nitem as *mut libc::c_void,
                    obj as *const libc::c_void,
                    ::core::mem::size_of::<THING>() as libc::c_ulong,
                );
                (*nitem)._o._o_count = 1 as libc::c_int;
                obj = nitem;
                break;
            }
        }
    }
    do_motion(obj, ydelta, xdelta);
    if (moat((*obj)._o._o_pos.y, (*obj)._o._o_pos.x)).is_null()
        || !hit_monster((*obj)._o._o_pos.y, (*obj)._o._o_pos.x, obj)
    {
        fall(obj, 1 as libc::c_int != 0);
    }
}
#[no_mangle]
pub unsafe extern "C" fn do_motion(
    mut obj: *mut THING,
    mut ydelta: libc::c_int,
    mut xdelta: libc::c_int,
) {
    let mut under: byte = '@' as i32 as byte;
    memmove(
        &mut (*obj)._o._o_pos as *mut coord as *mut libc::c_void,
        &mut player._t._t_pos as *mut coord as *const libc::c_void,
        ::core::mem::size_of::<coord>() as libc::c_ulong,
    );
    loop {
        let mut ch: libc::c_int = 0;
        if under as libc::c_int != '@' as i32
            && !_ce(&mut (*obj)._o._o_pos, &mut player._t._t_pos)
            && cansee((*obj)._o._o_pos.y, (*obj)._o._o_pos.x) as libc::c_int != 0
        {
            cur_mvaddch((*obj)._o._o_pos.y, (*obj)._o._o_pos.x, under);
        }
        (*obj)._o._o_pos.y += ydelta;
        (*obj)._o._o_pos.x += xdelta;
        ch = winat((*obj)._o._o_pos.y, (*obj)._o._o_pos.x) as libc::c_int;
        if !(step_ok(ch as byte) as libc::c_int != 0 && ch != 0xce as libc::c_int) {
            break;
        }
        if cansee((*obj)._o._o_pos.y, (*obj)._o._o_pos.x) {
            under = *_level.offset(INDEX((*obj)._o._o_pos.y, (*obj)._o._o_pos.x) as isize);
            cur_mvaddch(
                (*obj)._o._o_pos.y,
                (*obj)._o._o_pos.x,
                (*obj)._o._o_type as byte,
            );
            tick_pause();
        } else {
            under = '@' as i32 as byte;
        }
    }
}
unsafe extern "C" fn short_name(mut obj: *mut THING) -> *mut libc::c_char {
    match (*obj)._o._o_type {
        24 => return *w_names.as_mut_ptr().offset((*obj)._o._o_which as isize),
        8 => return *a_names.as_mut_ptr().offset((*obj)._o._o_which as isize),
        5 => return b"food\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        173 | 13 | 12 | 231 | 9 => {
            return (strchr(inv_name(obj, 1 as libc::c_int != 0), ' ' as i32))
                .offset(1 as libc::c_int as isize);
        }
        _ => {
            return b"bizzare thing\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn fall(mut obj: *mut THING, mut pr: bool) {
    static mut fpos: coord = coord { x: 0, y: 0 };
    let mut index: libc::c_int = 0;
    match fallpos(obj, &mut fpos) {
        1 => {
            index = INDEX(fpos.y, fpos.x);
            *_level.offset(index as isize) = (*obj)._o._o_type as byte;
            memmove(
                &mut (*obj)._o._o_pos as *mut coord as *mut libc::c_void,
                &mut fpos as *mut coord as *const libc::c_void,
                ::core::mem::size_of::<coord>() as libc::c_ulong,
            );
            if cansee(fpos.y, fpos.x) {
                if *_flags.offset(INDEX((*obj)._o._o_pos.y, (*obj)._o._o_pos.x) as isize)
                    as libc::c_int
                    & 0x40 as libc::c_int
                    != 0
                    || *_flags.offset(INDEX((*obj)._o._o_pos.y, (*obj)._o._o_pos.x) as isize)
                        as libc::c_int
                        & 0x20 as libc::c_int
                        != 0
                {
                    set_attr(14 as libc::c_int);
                }
                cur_mvaddch(fpos.y, fpos.x, (*obj)._o._o_type as byte);
                set_attr(0 as libc::c_int);
                if !(moat(fpos.y, fpos.x)).is_null() {
                    (*moat(fpos.y, fpos.x))._t._t_oldch = (*obj)._o._o_type as byte;
                }
            }
            list_attach(&mut lvl_obj, obj);
            return;
        }
        2 => {
            pr = 0 as libc::c_int != 0;
        }
        _ => {}
    }
    if pr {
        msg(
            b"the %s vanishes%s.\0" as *const u8 as *const libc::c_char,
            short_name(obj),
            noterse(
                b" as it hits the ground\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            ),
        );
    }
    discard(obj);
}
#[no_mangle]
pub unsafe extern "C" fn init_weapon(mut weap: *mut THING, mut type_0: byte) {
    let mut iwp: *mut init_weps = 0 as *mut init_weps;
    iwp = &mut *init_dam.as_mut_ptr().offset(type_0 as isize) as *mut init_weps;
    (*weap)._o._o_damage = (*iwp).iw_dam;
    (*weap)._o._o_hurldmg = (*iwp).iw_hrl;
    (*weap)._o._o_launch = (*iwp).iw_launch;
    (*weap)._o._o_flags = (*iwp).iw_flags as libc::c_short;
    if (*weap)._o._o_flags as libc::c_int & 0x20 as libc::c_int != 0 {
        (*weap)._o._o_count = rnd(8 as libc::c_int) + 8 as libc::c_int;
        let fresh0 = group;
        group = group + 1;
        (*weap)._o._o_group = fresh0;
    } else {
        (*weap)._o._o_count = 1 as libc::c_int;
    };
}
#[no_mangle]
pub unsafe extern "C" fn hit_monster(
    mut y: libc::c_int,
    mut x: libc::c_int,
    mut obj: *mut THING,
) -> bool {
    static mut mp: coord = coord { x: 0, y: 0 };
    let mut mo: *mut THING = moat(y, x);
    if !mo.is_null() {
        mp.y = y;
        mp.x = x;
        return fight(&mut mp, (*mo)._t._t_type, obj, 1 as libc::c_int != 0);
    }
    return 0 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn num(
    mut n1: libc::c_int,
    mut n2: libc::c_int,
    mut type_0: libc::c_char,
) -> *mut libc::c_char {
    static mut numbuf: [libc::c_char; 10] = [0; 10];
    sprintf(
        numbuf.as_mut_ptr(),
        b"%s%d\0" as *const u8 as *const libc::c_char,
        if n1 < 0 as libc::c_int {
            b"\0" as *const u8 as *const libc::c_char
        } else {
            b"+\0" as *const u8 as *const libc::c_char
        },
        n1,
    );
    if type_0 as libc::c_int == 0x18 as libc::c_int {
        sprintf(
            &mut *numbuf.as_mut_ptr().offset((strlen
                as unsafe extern "C" fn(*const libc::c_char) -> libc::c_ulong)(
                numbuf.as_mut_ptr()
            ) as isize) as *mut libc::c_char,
            b",%s%d\0" as *const u8 as *const libc::c_char,
            if n2 < 0 as libc::c_int {
                b"\0" as *const u8 as *const libc::c_char
            } else {
                b"+\0" as *const u8 as *const libc::c_char
            },
            n2,
        );
    }
    return numbuf.as_mut_ptr();
}
#[no_mangle]
pub unsafe extern "C" fn wield() {
    let mut obj: *mut THING = 0 as *mut THING;
    let mut oweapon: *mut THING = 0 as *mut THING;
    let mut sp: *mut libc::c_char = 0 as *mut libc::c_char;
    oweapon = cur_weapon;
    if !can_drop(cur_weapon) {
        cur_weapon = oweapon;
        return;
    }
    cur_weapon = oweapon;
    obj = get_item(
        b"wield\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0x18 as libc::c_int,
    );
    if !obj.is_null() {
        if (*obj)._o._o_type == 0x8 as libc::c_int {
            msg(b"you can't wield armor\0" as *const u8 as *const libc::c_char);
        } else if !is_current(obj) {
            sp = inv_name(obj, 1 as libc::c_int != 0);
            cur_weapon = obj;
            ifterse(
                b"now wielding %s (%c)\0" as *const u8 as *const libc::c_char,
                b"you are now wielding %s (%c)\0" as *const u8 as *const libc::c_char,
                sp,
                pack_char(obj) as libc::c_int,
            );
            return;
        }
    }
    after = 0 as libc::c_int != 0;
}
unsafe extern "C" fn fallpos(mut obj: *mut THING, mut newpos: *mut coord) -> libc::c_int {
    let mut y: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut cnt: libc::c_int = 0 as libc::c_int;
    let mut ch: libc::c_int = 0;
    let mut onfloor: *mut THING = 0 as *mut THING;
    y = (*obj)._o._o_pos.y - 1 as libc::c_int;
    while y <= (*obj)._o._o_pos.y + 1 as libc::c_int {
        x = (*obj)._o._o_pos.x - 1 as libc::c_int;
        while x <= (*obj)._o._o_pos.x + 1 as libc::c_int {
            if !(y == player._t._t_pos.y && x == player._t._t_pos.x
                || offmap(y, x) as libc::c_int != 0)
            {
                ch = *_level.offset(INDEX(y, x) as isize) as libc::c_int;
                if ch == 0xfa as libc::c_int || ch == 0xb1 as libc::c_int {
                    cnt += 1;
                    if rnd(cnt) == 0 as libc::c_int {
                        (*newpos).y = y;
                        (*newpos).x = x;
                    }
                } else if step_ok(ch as byte) as libc::c_int != 0
                    && {
                        onfloor = find_obj(y, x);
                        !onfloor.is_null()
                    }
                    && (*onfloor)._o._o_type == (*obj)._o._o_type
                    && (*onfloor)._o._o_group != 0
                    && (*onfloor)._o._o_group == (*obj)._o._o_group
                {
                    (*onfloor)._o._o_count += (*obj)._o._o_count;
                    return 2 as libc::c_int;
                }
            }
            x += 1;
        }
        y += 1;
    }
    return (cnt != 0 as libc::c_int) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn tick_pause() {
    cur_refresh();
    md_nanosleep(1000000 as libc::c_long * 55 as libc::c_int as libc::c_long);
}
