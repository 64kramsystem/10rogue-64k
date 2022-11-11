use ::libc;
extern "C" {
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    static mut maxitems: libc::c_int;
    static mut total: libc::c_int;
    static mut _things: *mut THING;
    static mut _t_alloc: *mut libc::c_int;
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
pub unsafe extern "C" fn list_detach(mut list: *mut *mut THING, mut item: *mut THING) {
    if *list == item {
        *list = (*item)._t._l_next;
    }
    if !((*item)._t._l_prev).is_null() {
        (*(*item)._t._l_prev)._t._l_next = (*item)._t._l_next;
    }
    if !((*item)._t._l_next).is_null() {
        (*(*item)._t._l_next)._t._l_prev = (*item)._t._l_prev;
    }
    (*item)._t._l_next = 0 as *mut thing;
    (*item)._t._l_prev = 0 as *mut thing;
}
#[no_mangle]
pub unsafe extern "C" fn list_attach(mut list: *mut *mut THING, mut item: *mut THING) {
    if !(*list).is_null() {
        (*item)._t._l_next = *list;
        (**list)._t._l_prev = item;
        (*item)._t._l_prev = 0 as *mut thing;
    } else {
        (*item)._t._l_next = 0 as *mut thing;
        (*item)._t._l_prev = 0 as *mut thing;
    }
    *list = item;
}
#[no_mangle]
pub unsafe extern "C" fn list_free(mut ptr: *mut *mut THING) {
    let mut item: *mut THING = 0 as *mut THING;
    while !(*ptr).is_null() {
        item = *ptr;
        *ptr = (*item)._t._l_next;
        discard(item);
    }
}
#[no_mangle]
pub unsafe extern "C" fn new_item() -> *mut THING {
    let mut item: *mut THING = 0 as *mut THING;
    item = talloc() as *mut THING;
    if !item.is_null() {
        (*item)._t._l_prev = 0 as *mut thing;
        (*item)._t._l_next = (*item)._t._l_prev;
    }
    return item;
}
unsafe extern "C" fn talloc() -> *mut libc::c_void {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 83 as libc::c_int {
        if *_t_alloc.offset(i as isize) == 0 as libc::c_int {
            total += 1;
            if total > maxitems {
                maxitems = total;
            }
            let ref mut fresh0 = *_t_alloc.offset(i as isize);
            *fresh0 += 1;
            memset(
                &mut *_things.offset(i as isize) as *mut THING as *mut libc::c_void,
                0 as libc::c_int,
                ::core::mem::size_of::<THING>() as libc::c_ulong,
            );
            return &mut *_things.offset(i as isize) as *mut THING as *mut libc::c_void;
        }
        i += 1;
    }
    return 0 as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn discard(mut item: *mut THING) -> libc::c_int {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 83 as libc::c_int {
        if item == &mut *_things.offset(i as isize) as *mut THING {
            total -= 1;
            *_t_alloc.offset(i as isize) = 0 as libc::c_int;
            return 1 as libc::c_int;
        }
        i += 1;
    }
    return 0 as libc::c_int;
}
