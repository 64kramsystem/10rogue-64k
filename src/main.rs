#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]

// Rust port: Watch out! Clippy removes this, which causes the build to fail.
#[allow(unused_imports)]
use ::c2rust_out::*;

extern "C" {
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char, _: libc::c_ulong) -> libc::c_int;
    fn setlocale(__category: libc::c_int, __locale: *const libc::c_char) -> *mut libc::c_char;
    fn md_srand() -> libc::c_int;
    fn setup();
    fn credits();
    fn readchar() -> byte;
    fn fatal(msg_0: *const libc::c_char, _: ...);
    static mut noscore: bool;
    static mut playing: bool;
    static mut terse: bool;
    static mut count: libc::c_int;
    static mut dnum: libc::c_int;
    static mut mpos: libc::c_int;
    static mut purse: libc::c_int;
    static mut seed: libc::c_long;
    static mut goodchk: libc::c_int;
    static mut player: THING;
    static mut oldpos: coord;
    static mut oldrp: *mut room;
    static mut s_save: [libc::c_char; 0];
    static mut s_screen: [libc::c_char; 0];
    static mut whoami: [libc::c_char; 0];
    fn runners();
    fn roomin(cp: *mut coord) -> *mut room;
    fn command();
    fn start_daemon(func: Option<unsafe extern "C" fn() -> ()>);
    fn fuse(func: Option<unsafe extern "C" fn() -> ()>, time: libc::c_int);
    fn doctor();
    fn swander();
    fn stomach();
    fn setenv_from_file(envfile: *mut libc::c_char) -> bool;
    fn init_player();
    fn init_things();
    fn init_colors();
    fn init_names();
    fn init_stones();
    fn init_materials();
    fn init_ds();
    fn msg(fmt: *const libc::c_char, _: ...);
    fn status();
    fn str_attr(str: *mut libc::c_char);
    fn noterse(str: *mut libc::c_char) -> *mut libc::c_char;
    fn find_drive() -> libc::c_int;
    fn epyx_yeah(path: *const libc::c_char) -> libc::c_int;
    fn restore(savefile: *mut libc::c_char);
    fn score(amount: libc::c_int, flags: libc::c_int, monst: libc::c_char);
    fn look(wakeup: bool);
    fn spread(nm: libc::c_int) -> libc::c_int;
    fn new_level();
    fn protect(drive: libc::c_int);
    fn cur_clear();
    fn cursor(ison: bool) -> bool;
    fn getrc(rp: *mut libc::c_int, cp: *mut libc::c_int);
    fn cur_clrtoeol();
    fn cur_addstr(s: *mut libc::c_char);
    fn winit();
    fn cur_printw(msg_0: *const libc::c_char, _: ...);
    fn drop_curtain();
    fn raise_curtain();
    fn cur_move(row: libc::c_int, col: libc::c_int) -> libc::c_int;
    static mut LINES: libc::c_int;
    static mut is_saved: libc::c_int;
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
pub static mut bwflag: libc::c_int = 0 as libc::c_int;
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char) -> libc::c_int {
    let mut curarg: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
    let mut savfile: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
    setlocale(6 as libc::c_int, b"\0" as *const u8 as *const libc::c_char);
    epyx_yeah(b"rogue.pic\0" as *const u8 as *const libc::c_char);
    init_ds();
    setenv_from_file(b"rogue.opt\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    protect(find_drive());
    if strncmp(
        s_screen.as_mut_ptr(),
        b"bw\0" as *const u8 as *const libc::c_char,
        2 as libc::c_int as libc::c_ulong,
    ) == 0 as libc::c_int
    {
        bwflag = 1 as libc::c_int;
    }
    dnum = 0 as libc::c_int;
    loop {
        argc -= 1;
        if !(argc != 0 && goodchk == 0xd0d as libc::c_int) {
            break;
        }
        argv = argv.offset(1);
        curarg = *argv;
        if *curarg as libc::c_int == '-' as i32 || *curarg as libc::c_int == '/' as i32 {
            match *curarg.offset(1 as libc::c_int as isize) as libc::c_int {
                82 | 114 => {
                    savfile = s_save.as_mut_ptr();
                }
                115 | 83 => {
                    winit();
                    noscore = 1 as libc::c_int != 0;
                    is_saved = 1 as libc::c_int;
                    score(
                        0 as libc::c_int,
                        0 as libc::c_int,
                        0 as libc::c_int as libc::c_char,
                    );
                    fatal(b"\0" as *const u8 as *const libc::c_char);
                }
                _ => {}
            }
        } else if savfile.is_null() {
            savfile = curarg;
        }
    }
    if savfile.is_null() {
        savfile = std::ptr::null_mut::<libc::c_char>();
        winit();
        credits();
        if dnum == 0 as libc::c_int {
            dnum = md_srand();
        }
        seed = dnum as libc::c_long;
        init_player();
        init_things();
        init_names();
        init_colors();
        init_stones();
        init_materials();
        setup();
        drop_curtain();
        new_level();
        start_daemon(::core::mem::transmute::<
            Option<unsafe extern "C" fn() -> ()>,
            Option<unsafe extern "C" fn() -> ()>,
        >(Some(doctor as unsafe extern "C" fn() -> ())));
        fuse(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn() -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(swander as unsafe extern "C" fn() -> ())),
            spread(70 as libc::c_int),
        );
        start_daemon(::core::mem::transmute::<
            Option<unsafe extern "C" fn() -> ()>,
            Option<unsafe extern "C" fn() -> ()>,
        >(Some(stomach as unsafe extern "C" fn() -> ())));
        start_daemon(::core::mem::transmute::<
            Option<unsafe extern "C" fn() -> ()>,
            Option<unsafe extern "C" fn() -> ()>,
        >(Some(runners as unsafe extern "C" fn() -> ())));
        msg(
            b"Hello %s%s.\0" as *const u8 as *const libc::c_char,
            whoami.as_mut_ptr(),
            noterse(
                b".  Welcome to the Dungeons of Doom\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            ),
        );
        raise_curtain();
    }
    playit(savfile);
    0 as libc::c_int
}
#[no_mangle]
pub unsafe extern "C" fn endit() {
    fatal(
        b"Ok, if you want to exit that badly, I'll have to allow it\n\0" as *const u8
            as *const libc::c_char,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ran() -> libc::c_long {
    seed *= 125 as libc::c_int as libc::c_long;
    seed -= seed / 2796203 as libc::c_int as libc::c_long * 2796203 as libc::c_int as libc::c_long;
    seed
}
#[no_mangle]
pub unsafe extern "C" fn rnd(mut range: libc::c_int) -> libc::c_int {
    (if range < 1 as libc::c_int {
        0 as libc::c_int as libc::c_long
    } else {
        ((ran() + ran()) & 0x7fffffff as libc::c_long) % range as libc::c_long
    }) as libc::c_int
}
#[no_mangle]
pub unsafe extern "C" fn roll(mut number: libc::c_int, mut sides: libc::c_int) -> libc::c_int {
    let mut dtotal: libc::c_int = 0 as libc::c_int;
    loop {
        let fresh0 = number;
        number -= 1;
        if fresh0 == 0 {
            break;
        }
        dtotal += rnd(sides) + 1 as libc::c_int;
    }
    dtotal
}
#[no_mangle]
pub unsafe extern "C" fn playit(mut sname: *mut libc::c_char) {
    if !sname.is_null() {
        restore(sname);
        setup();
        cursor(0 as libc::c_int != 0);
    } else {
        oldpos.x = player._t._t_pos.x;
        oldpos.y = player._t._t_pos.y;
        oldrp = roomin(&mut player._t._t_pos);
    }
    while playing {
        command();
    }
    endit();
}
#[no_mangle]
pub unsafe extern "C" fn quit() {
    let mut oy: libc::c_int = 0;
    let mut ox: libc::c_int = 0;
    let mut answer: byte = 0;
    static mut qstate: bool = 0 as libc::c_int != 0;
    if qstate as libc::c_int == 1 as libc::c_int {
        leave();
    }
    qstate = 1 as libc::c_int != 0;
    mpos = 0 as libc::c_int;
    getrc(&mut oy, &mut ox);
    cur_move(0 as libc::c_int, 0 as libc::c_int);
    cur_clrtoeol();
    cur_move(0 as libc::c_int, 0 as libc::c_int);
    if !terse {
        cur_addstr(b"Do you wish to \0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    }
    str_attr(
        b"end your quest now (%Yes/%No) ?\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    look(0 as libc::c_int != 0);
    answer = readchar();
    if answer as libc::c_int == 'y' as i32 || answer as libc::c_int == 'Y' as i32 {
        cur_clear();
        cur_move(0 as libc::c_int, 0 as libc::c_int);
        cur_printw(
            b"You quit with %u gold pieces\n\0" as *const u8 as *const libc::c_char,
            purse,
        );
        score(purse, 1 as libc::c_int, 0 as libc::c_int as libc::c_char);
        fatal(b"\0" as *const u8 as *const libc::c_char);
    } else {
        cur_move(0 as libc::c_int, 0 as libc::c_int);
        cur_clrtoeol();
        status();
        cur_move(oy, ox);
        mpos = 0 as libc::c_int;
        count = 0 as libc::c_int;
    }
    qstate = 0 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn leave() {
    look(0 as libc::c_int != 0);
    cur_move(LINES - 1 as libc::c_int, 0 as libc::c_int);
    cur_clrtoeol();
    cur_move(LINES - 2 as libc::c_int, 0 as libc::c_int);
    cur_clrtoeol();
    cur_move(LINES - 2 as libc::c_int, 0 as libc::c_int);
    fatal(b"Ok, if you want to leave that badly\n\0" as *const u8 as *const libc::c_char);
}
pub fn main() {
    let mut args: Vec<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::core::ptr::null_mut());
    unsafe {
        ::std::process::exit(main_0(
            (args.len() - 1) as libc::c_int,
            args.as_mut_ptr() as *mut *mut libc::c_char,
        ) as i32)
    }
}
