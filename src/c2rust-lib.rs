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

extern crate libc;
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
