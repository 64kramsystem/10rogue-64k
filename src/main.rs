/*
 * ###   ###   ###  #   # #####
 * #  # #   # #   # #   # #
 * #  # #   # #   # #   # #
 * ###  #   # #     #   # ###
 * #  # #   # #  ## #   # #
 * #  # #   # #   # #   # #
 * #  #  ###   ###   ###  #####
 *
 * Exploring the Dungeons of Doom
 * Copyright (C) 1981 by Michael Toy, Ken Arnold, and Glenn Wichman
 * main.c	1.4 (A.I. Design) 11/28/84
 * All rights reserved
 * Copyright (C) 1983 by Mel Sibony, Jon Lane (AI Design update for the IBMPC)
 */

#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![allow(clippy::all)]
#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(c_variadic)]
#![feature(extern_types)]

pub mod armor;
pub mod chase;
pub mod command;
pub mod curses;
pub mod daemon;
pub mod daemons;
pub mod env;
pub mod r#extern;
pub mod fakedos;
pub mod fight;
pub mod init;
pub mod io;
pub mod list;
pub mod load;
pub mod mach_dep;
pub mod maze;
pub mod misc;
pub mod monsters;
pub mod r#move;
pub mod new_leve;
pub mod pack;
pub mod passages;
pub mod potions;
pub mod protect;
pub mod rings;
pub mod rip;
pub mod rooms;
pub mod save;
pub mod scrolls;
pub mod slime;
pub mod splash {
    pub mod load_sdl;
    pub mod splash;
} // mod splash
pub mod sticks;
pub mod strings;
pub mod things;
pub mod weapons;
pub mod wizard;

use std::process;

use r#extern::seed;

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
fn main() {
    let mut args = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::core::ptr::null_mut());

    let mut argc = args.len() - 1;
    let mut argv = args.as_mut_ptr() as *mut *mut libc::c_char;

    unsafe {
        let mut curarg = std::ptr::null_mut::<libc::c_char>();
        let mut savfile = std::ptr::null_mut::<libc::c_char>();
        setlocale(6, b"\0" as *const u8 as *const libc::c_char);
        epyx_yeah(b"rogue.pic\0" as *const u8 as *const libc::c_char);
        init_ds();
        setenv_from_file(b"rogue.opt\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
        protect(find_drive());
        if strncmp(
            s_screen.as_mut_ptr(),
            b"bw\0" as *const u8 as *const libc::c_char,
            2,
        ) == 0
        {
            bwflag = 1;
        }
        dnum = 0;
        loop {
            argc -= 1;
            if !(argc != 0 && goodchk == 0xd0d as libc::c_int) {
                break;
            }
            argv = argv.offset(1);
            curarg = *argv;
            if *curarg as libc::c_int == '-' as i32 || *curarg as libc::c_int == '/' as i32 {
                match *curarg.offset(1) as libc::c_int {
                    82 | 114 => {
                        savfile = s_save.as_mut_ptr();
                    }
                    115 | 83 => {
                        winit();
                        noscore = 1 != 0;
                        is_saved = 1;
                        score(0, 0, 0);
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
            if dnum == 0 {
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
            start_daemon(Some(doctor as unsafe extern "C" fn() -> ()));
            fuse(Some(swander as unsafe extern "C" fn() -> ()), spread(70));
            start_daemon(Some(stomach as unsafe extern "C" fn() -> ()));
            start_daemon(Some(runners as unsafe extern "C" fn() -> ()));
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
    }
    process::exit(0);
}
#[no_mangle]
pub unsafe extern "C" fn endit() {
    fatal(
        b"Ok, if you want to exit that badly, I'll have to allow it\n\0" as *const u8
            as *const libc::c_char,
    );
}

//@ no need to declare in rogue.h
/*
 * Random number generator -
 * adapted from the FORTRAN version
 * in "Software Manual for the Elementary Functions"
 * by W.J. Cody, Jr and William Waite.
 */
// Rust port: the original port changes the semantics, since it retains the long data type (64 bits),
// which was 32 bits in the original (16-bit DOS) program.
fn ran() -> i64 {
    unsafe {
        seed *= 125;
        seed -= seed / 2796203 * 2796203;
        seed
    }
}

/*
 * rnd:
 *	Pick a very random number.
 */
pub fn rnd(mut range: i32) -> i32 {
    /*@
     * range size was expected to be 16 bit
     * function will return the seed itself if range value is >= 2^31 - 1
     */
    if range < 1 {
        0
    } else {
        ((ran() + ran()) & 0x7fffffff) as i32 % range
    }
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
