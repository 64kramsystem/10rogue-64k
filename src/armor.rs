use ::libc;
extern "C" {
    static mut after: bool;
    static mut cur_armor: *mut THING;
    fn msg(fmt: *const libc::c_char, _: ...);
    fn inv_name(obj: *mut THING, drop_0: bool) -> *mut libc::c_char;
    fn do_fuses();
    fn do_daemons();
    fn get_item(purpose: *mut libc::c_char, type_0: libc::c_int) -> *mut THING;
    fn noterse(str: *mut libc::c_char) -> *mut libc::c_char;
    fn pack_char(obj: *mut THING) -> byte;
    fn can_drop(op: *mut THING) -> bool;
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
pub unsafe extern "C" fn wear() {
    let mut obj: *mut THING = 0 as *mut THING;
    let mut sp: *mut libc::c_char = 0 as *mut libc::c_char;
    if !cur_armor.is_null() {
        msg(
            b"you are already wearing some%s.\0" as *const u8 as *const libc::c_char,
            noterse(
                b".  You'll have to take it off first\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
            ),
        );
        after = 0 as libc::c_int != 0;
        return;
    }
    obj = get_item(
        b"wear\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0x8 as libc::c_int,
    );
    if obj.is_null() {
        return;
    }
    if (*obj)._o._o_type != 0x8 as libc::c_int {
        msg(b"you can't wear that\0" as *const u8 as *const libc::c_char);
        return;
    }
    waste_time();
    (*obj)
        ._o
        ._o_flags = ((*obj)._o._o_flags as libc::c_int | 0x2 as libc::c_int)
        as libc::c_short;
    sp = inv_name(obj, 1 as libc::c_int != 0);
    cur_armor = obj;
    msg(b"you are now wearing %s\0" as *const u8 as *const libc::c_char, sp);
}
#[no_mangle]
pub unsafe extern "C" fn take_off() {
    let mut obj: *mut THING = 0 as *mut THING;
    obj = cur_armor;
    if obj.is_null() {
        after = 0 as libc::c_int != 0;
        msg(b"you aren't wearing any armor\0" as *const u8 as *const libc::c_char);
        return;
    }
    if !can_drop(cur_armor) {
        return;
    }
    cur_armor = 0 as *mut THING;
    msg(
        b"you used to be wearing %c) %s\0" as *const u8 as *const libc::c_char,
        pack_char(obj) as libc::c_int,
        inv_name(obj, 1 as libc::c_int != 0),
    );
}
#[no_mangle]
pub unsafe extern "C" fn waste_time() {
    do_daemons();
    do_fuses();
}
