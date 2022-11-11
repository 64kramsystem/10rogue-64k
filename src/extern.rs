use ::libc;
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
pub struct magic_item {
    pub mi_name: *mut libc::c_char,
    pub mi_prob: libc::c_int,
    pub mi_worth: libc::c_short,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct monster {
    pub m_name: *mut libc::c_char,
    pub m_carry: libc::c_int,
    pub m_flags: libc::c_ushort,
    pub m_stats: stats,
}
#[no_mangle]
pub static mut revno: libc::c_int = 1 as libc::c_int;
#[no_mangle]
pub static mut verno: libc::c_int = 48 as libc::c_int;
#[no_mangle]
pub static mut w_names: [*mut libc::c_char; 11] = [
    b"mace\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"long sword\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"short bow\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"arrow\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"dagger\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"two handed sword\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"dart\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"crossbow\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"crossbow bolt\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"spear\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
];
#[no_mangle]
pub static mut a_names: [*mut libc::c_char; 8] = [
    b"leather armor\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"ring mail\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"studded leather armor\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"scale mail\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"chain mail\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"splint mail\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"banded mail\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"plate mail\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
];
#[no_mangle]
pub static mut a_chances: [libc::c_int; 8] = [
    20 as libc::c_int,
    35 as libc::c_int,
    50 as libc::c_int,
    63 as libc::c_int,
    75 as libc::c_int,
    85 as libc::c_int,
    95 as libc::c_int,
    100 as libc::c_int,
];
#[no_mangle]
pub static mut a_class: [libc::c_int; 8] = [
    8 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    6 as libc::c_int,
    5 as libc::c_int,
    4 as libc::c_int,
    4 as libc::c_int,
    3 as libc::c_int,
];
#[no_mangle]
pub static mut s_magic: [magic_item; 15] = [
    {
        let mut init = magic_item {
            mi_name: b"monster confusion\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            mi_prob: 8 as libc::c_int,
            mi_worth: 140 as libc::c_int as libc::c_short,
        };
        init
    },
    {
        let mut init = magic_item {
            mi_name: b"magic mapping\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            mi_prob: 5 as libc::c_int,
            mi_worth: 150 as libc::c_int as libc::c_short,
        };
        init
    },
    {
        let mut init = magic_item {
            mi_name: b"hold monster\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            mi_prob: 3 as libc::c_int,
            mi_worth: 180 as libc::c_int as libc::c_short,
        };
        init
    },
    {
        let mut init = magic_item {
            mi_name: b"sleep\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            mi_prob: 5 as libc::c_int,
            mi_worth: 5 as libc::c_int as libc::c_short,
        };
        init
    },
    {
        let mut init = magic_item {
            mi_name: b"enchant armor\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            mi_prob: 8 as libc::c_int,
            mi_worth: 160 as libc::c_int as libc::c_short,
        };
        init
    },
    {
        let mut init = magic_item {
            mi_name: b"identify\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            mi_prob: 27 as libc::c_int,
            mi_worth: 100 as libc::c_int as libc::c_short,
        };
        init
    },
    {
        let mut init = magic_item {
            mi_name: b"scare monster\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            mi_prob: 4 as libc::c_int,
            mi_worth: 200 as libc::c_int as libc::c_short,
        };
        init
    },
    {
        let mut init = magic_item {
            mi_name: b"food detection\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            mi_prob: 4 as libc::c_int,
            mi_worth: 50 as libc::c_int as libc::c_short,
        };
        init
    },
    {
        let mut init = magic_item {
            mi_name: b"teleportation\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            mi_prob: 7 as libc::c_int,
            mi_worth: 165 as libc::c_int as libc::c_short,
        };
        init
    },
    {
        let mut init = magic_item {
            mi_name: b"enchant weapon\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            mi_prob: 10 as libc::c_int,
            mi_worth: 150 as libc::c_int as libc::c_short,
        };
        init
    },
    {
        let mut init = magic_item {
            mi_name: b"create monster\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            mi_prob: 5 as libc::c_int,
            mi_worth: 75 as libc::c_int as libc::c_short,
        };
        init
    },
    {
        let mut init = magic_item {
            mi_name: b"remove curse\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            mi_prob: 8 as libc::c_int,
            mi_worth: 105 as libc::c_int as libc::c_short,
        };
        init
    },
    {
        let mut init = magic_item {
            mi_name: b"aggravate monsters\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            mi_prob: 4 as libc::c_int,
            mi_worth: 20 as libc::c_int as libc::c_short,
        };
        init
    },
    {
        let mut init = magic_item {
            mi_name: b"blank paper\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            mi_prob: 1 as libc::c_int,
            mi_worth: 5 as libc::c_int as libc::c_short,
        };
        init
    },
    {
        let mut init = magic_item {
            mi_name: b"vorpalize weapon\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            mi_prob: 1 as libc::c_int,
            mi_worth: 300 as libc::c_int as libc::c_short,
        };
        init
    },
];
#[no_mangle]
pub static mut p_magic: [magic_item; 14] = [
    {
        let mut init = magic_item {
            mi_name: b"confusion\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            mi_prob: 8 as libc::c_int,
            mi_worth: 5 as libc::c_int as libc::c_short,
        };
        init
    },
    {
        let mut init = magic_item {
            mi_name: b"paralysis\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            mi_prob: 10 as libc::c_int,
            mi_worth: 5 as libc::c_int as libc::c_short,
        };
        init
    },
    {
        let mut init = magic_item {
            mi_name: b"poison\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            mi_prob: 8 as libc::c_int,
            mi_worth: 5 as libc::c_int as libc::c_short,
        };
        init
    },
    {
        let mut init = magic_item {
            mi_name: b"gain strength\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            mi_prob: 15 as libc::c_int,
            mi_worth: 150 as libc::c_int as libc::c_short,
        };
        init
    },
    {
        let mut init = magic_item {
            mi_name: b"see invisible\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            mi_prob: 2 as libc::c_int,
            mi_worth: 100 as libc::c_int as libc::c_short,
        };
        init
    },
    {
        let mut init = magic_item {
            mi_name: b"healing\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            mi_prob: 15 as libc::c_int,
            mi_worth: 130 as libc::c_int as libc::c_short,
        };
        init
    },
    {
        let mut init = magic_item {
            mi_name: b"monster detection\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            mi_prob: 6 as libc::c_int,
            mi_worth: 130 as libc::c_int as libc::c_short,
        };
        init
    },
    {
        let mut init = magic_item {
            mi_name: b"magic detection\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            mi_prob: 6 as libc::c_int,
            mi_worth: 105 as libc::c_int as libc::c_short,
        };
        init
    },
    {
        let mut init = magic_item {
            mi_name: b"raise level\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            mi_prob: 2 as libc::c_int,
            mi_worth: 250 as libc::c_int as libc::c_short,
        };
        init
    },
    {
        let mut init = magic_item {
            mi_name: b"extra healing\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            mi_prob: 5 as libc::c_int,
            mi_worth: 200 as libc::c_int as libc::c_short,
        };
        init
    },
    {
        let mut init = magic_item {
            mi_name: b"haste self\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            mi_prob: 4 as libc::c_int,
            mi_worth: 190 as libc::c_int as libc::c_short,
        };
        init
    },
    {
        let mut init = magic_item {
            mi_name: b"restore strength\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            mi_prob: 14 as libc::c_int,
            mi_worth: 130 as libc::c_int as libc::c_short,
        };
        init
    },
    {
        let mut init = magic_item {
            mi_name: b"blindness\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            mi_prob: 4 as libc::c_int,
            mi_worth: 5 as libc::c_int as libc::c_short,
        };
        init
    },
    {
        let mut init = magic_item {
            mi_name: b"thirst quenching\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            mi_prob: 1 as libc::c_int,
            mi_worth: 5 as libc::c_int as libc::c_short,
        };
        init
    },
];
#[no_mangle]
pub static mut r_magic: [magic_item; 14] = [
    {
        let mut init = magic_item {
            mi_name: b"protection\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            mi_prob: 9 as libc::c_int,
            mi_worth: 400 as libc::c_int as libc::c_short,
        };
        init
    },
    {
        let mut init = magic_item {
            mi_name: b"add strength\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            mi_prob: 9 as libc::c_int,
            mi_worth: 400 as libc::c_int as libc::c_short,
        };
        init
    },
    {
        let mut init = magic_item {
            mi_name: b"sustain strength\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            mi_prob: 5 as libc::c_int,
            mi_worth: 280 as libc::c_int as libc::c_short,
        };
        init
    },
    {
        let mut init = magic_item {
            mi_name: b"searching\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            mi_prob: 10 as libc::c_int,
            mi_worth: 420 as libc::c_int as libc::c_short,
        };
        init
    },
    {
        let mut init = magic_item {
            mi_name: b"see invisible\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            mi_prob: 10 as libc::c_int,
            mi_worth: 310 as libc::c_int as libc::c_short,
        };
        init
    },
    {
        let mut init = magic_item {
            mi_name: b"adornment\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            mi_prob: 1 as libc::c_int,
            mi_worth: 10 as libc::c_int as libc::c_short,
        };
        init
    },
    {
        let mut init = magic_item {
            mi_name: b"aggravate monster\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            mi_prob: 10 as libc::c_int,
            mi_worth: 10 as libc::c_int as libc::c_short,
        };
        init
    },
    {
        let mut init = magic_item {
            mi_name: b"dexterity\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            mi_prob: 8 as libc::c_int,
            mi_worth: 440 as libc::c_int as libc::c_short,
        };
        init
    },
    {
        let mut init = magic_item {
            mi_name: b"increase damage\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            mi_prob: 8 as libc::c_int,
            mi_worth: 400 as libc::c_int as libc::c_short,
        };
        init
    },
    {
        let mut init = magic_item {
            mi_name: b"regeneration\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            mi_prob: 4 as libc::c_int,
            mi_worth: 460 as libc::c_int as libc::c_short,
        };
        init
    },
    {
        let mut init = magic_item {
            mi_name: b"slow digestion\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            mi_prob: 9 as libc::c_int,
            mi_worth: 240 as libc::c_int as libc::c_short,
        };
        init
    },
    {
        let mut init = magic_item {
            mi_name: b"teleportation\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            mi_prob: 5 as libc::c_int,
            mi_worth: 30 as libc::c_int as libc::c_short,
        };
        init
    },
    {
        let mut init = magic_item {
            mi_name: b"stealth\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            mi_prob: 7 as libc::c_int,
            mi_worth: 470 as libc::c_int as libc::c_short,
        };
        init
    },
    {
        let mut init = magic_item {
            mi_name: b"maintain armor\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            mi_prob: 5 as libc::c_int,
            mi_worth: 380 as libc::c_int as libc::c_short,
        };
        init
    },
];
#[no_mangle]
pub static mut ws_magic: [magic_item; 14] = [
    {
        let mut init = magic_item {
            mi_name: b"light\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            mi_prob: 12 as libc::c_int,
            mi_worth: 250 as libc::c_int as libc::c_short,
        };
        init
    },
    {
        let mut init = magic_item {
            mi_name: b"striking\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            mi_prob: 9 as libc::c_int,
            mi_worth: 75 as libc::c_int as libc::c_short,
        };
        init
    },
    {
        let mut init = magic_item {
            mi_name: b"lightning\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            mi_prob: 3 as libc::c_int,
            mi_worth: 330 as libc::c_int as libc::c_short,
        };
        init
    },
    {
        let mut init = magic_item {
            mi_name: b"fire\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            mi_prob: 3 as libc::c_int,
            mi_worth: 330 as libc::c_int as libc::c_short,
        };
        init
    },
    {
        let mut init = magic_item {
            mi_name: b"cold\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            mi_prob: 3 as libc::c_int,
            mi_worth: 330 as libc::c_int as libc::c_short,
        };
        init
    },
    {
        let mut init = magic_item {
            mi_name: b"polymorph\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            mi_prob: 15 as libc::c_int,
            mi_worth: 310 as libc::c_int as libc::c_short,
        };
        init
    },
    {
        let mut init = magic_item {
            mi_name: b"magic missile\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            mi_prob: 10 as libc::c_int,
            mi_worth: 170 as libc::c_int as libc::c_short,
        };
        init
    },
    {
        let mut init = magic_item {
            mi_name: b"haste monster\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            mi_prob: 9 as libc::c_int,
            mi_worth: 5 as libc::c_int as libc::c_short,
        };
        init
    },
    {
        let mut init = magic_item {
            mi_name: b"slow monster\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            mi_prob: 11 as libc::c_int,
            mi_worth: 350 as libc::c_int as libc::c_short,
        };
        init
    },
    {
        let mut init = magic_item {
            mi_name: b"drain life\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            mi_prob: 9 as libc::c_int,
            mi_worth: 300 as libc::c_int as libc::c_short,
        };
        init
    },
    {
        let mut init = magic_item {
            mi_name: b"nothing\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            mi_prob: 1 as libc::c_int,
            mi_worth: 5 as libc::c_int as libc::c_short,
        };
        init
    },
    {
        let mut init = magic_item {
            mi_name: b"teleport away\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            mi_prob: 5 as libc::c_int,
            mi_worth: 340 as libc::c_int as libc::c_short,
        };
        init
    },
    {
        let mut init = magic_item {
            mi_name: b"teleport to\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            mi_prob: 5 as libc::c_int,
            mi_worth: 50 as libc::c_int as libc::c_short,
        };
        init
    },
    {
        let mut init = magic_item {
            mi_name: b"cancellation\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            mi_prob: 5 as libc::c_int,
            mi_worth: 280 as libc::c_int as libc::c_short,
        };
        init
    },
];
#[no_mangle]
pub static mut helpcoms: [h_list; 65] = unsafe {
    [
        {
            let mut init = h_list {
                h_chstr: *::core::mem::transmute::<&[u8; 6], &mut [byte; 6]>(b"\0\0\0\0\0\0"),
                h_desc: b"F1     list of commands\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = h_list {
                h_chstr: *::core::mem::transmute::<&[u8; 6], &mut [byte; 6]>(b"\0\0\0\0\0\0"),
                h_desc: b"F2     list of symbols\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = h_list {
                h_chstr: *::core::mem::transmute::<&[u8; 6], &mut [byte; 6]>(b"\0\0\0\0\0\0"),
                h_desc: b"F3     repeat command\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = h_list {
                h_chstr: *::core::mem::transmute::<&[u8; 6], &mut [byte; 6]>(b"\0\0\0\0\0\0"),
                h_desc: b"F4     repeat message\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = h_list {
                h_chstr: *::core::mem::transmute::<&[u8; 6], &mut [byte; 6]>(b"\0\0\0\0\0\0"),
                h_desc: b"F5     rename something\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = h_list {
                h_chstr: *::core::mem::transmute::<&[u8; 6], &mut [byte; 6]>(b"\0\0\0\0\0\0"),
                h_desc: b"F6     recall what's been discovered\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = h_list {
                h_chstr: *::core::mem::transmute::<&[u8; 6], &mut [byte; 6]>(b"\0\0\0\0\0\0"),
                h_desc: b"F7     inventory of your possessions\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = h_list {
                h_chstr: *::core::mem::transmute::<&[u8; 6], &mut [byte; 6]>(b"\0\0\0\0\0\0"),
                h_desc: b"F8     <dir> identify trap type\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = h_list {
                h_chstr: *::core::mem::transmute::<&[u8; 6], &mut [byte; 6]>(b"\0\0\0\0\0\0"),
                h_desc: b"F9     The Any Key (definable)\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = h_list {
                h_chstr: *::core::mem::transmute::<&[u8; 6], &mut [byte; 6]>(b"\0\0\0\0\0\0"),
                h_desc: b"Alt F9 defines the Any Key\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = h_list {
                h_chstr: *::core::mem::transmute::<&[u8; 6], &mut [byte; 6]>(b"\0\0\0\0\0\0"),
                h_desc: b"F10    Supervisor Key (fake dos)\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = h_list {
                h_chstr: *::core::mem::transmute::<&[u8; 6], &mut [byte; 6]>(b"\0\0\0\0\0\0"),
                h_desc: b"Space  Clear -More- message\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = h_list {
                h_chstr: *::core::mem::transmute::<&[u8; 6], &mut [byte; 6]>(b"\0\0\0\0\0\0"),
                h_desc: b"\x11\xD9     the Enter Key\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = h_list {
                h_chstr: *::core::mem::transmute::<&[u8; 6], &mut [byte; 6]>(b"\0\0\0\0\0\0"),
                h_desc: b"\x1B      left\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = h_list {
                h_chstr: *::core::mem::transmute::<&[u8; 6], &mut [byte; 6]>(b"\0\0\0\0\0\0"),
                h_desc: b"\x19      down\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = h_list {
                h_chstr: *::core::mem::transmute::<&[u8; 6], &mut [byte; 6]>(b"\0\0\0\0\0\0"),
                h_desc: b"\x18      up\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = h_list {
                h_chstr: *::core::mem::transmute::<&[u8; 6], &mut [byte; 6]>(b"\0\0\0\0\0\0"),
                h_desc: b"\x1A      right\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = h_list {
                h_chstr: *::core::mem::transmute::<&[u8; 6], &mut [byte; 6]>(b"\0\0\0\0\0\0"),
                h_desc: b"Home   up & left\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = h_list {
                h_chstr: *::core::mem::transmute::<&[u8; 6], &mut [byte; 6]>(b"\0\0\0\0\0\0"),
                h_desc: b"PgUp   up & right\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = h_list {
                h_chstr: *::core::mem::transmute::<&[u8; 6], &mut [byte; 6]>(b"\0\0\0\0\0\0"),
                h_desc: b"End    down & left\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = h_list {
                h_chstr: *::core::mem::transmute::<&[u8; 6], &mut [byte; 6]>(b"\0\0\0\0\0\0"),
                h_desc: b"PgDn   down & right\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = h_list {
                h_chstr: *::core::mem::transmute::<&[u8; 6], &mut [byte; 6]>(b"\0\0\0\0\0\0"),
                h_desc: b"Scroll Fast Play mode\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = h_list {
                h_chstr: *::core::mem::transmute::<&[u8; 6], &mut [byte; 6]>(b"\0\0\0\0\0\0"),
                h_desc: b".      rest\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = h_list {
                h_chstr: *::core::mem::transmute::<&[u8; 6], &mut [byte; 6]>(b"\0\0\0\0\0\0"),
                h_desc: b">      go down a staircase\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = h_list {
                h_chstr: *::core::mem::transmute::<&[u8; 6], &mut [byte; 6]>(b"\0\0\0\0\0\0"),
                h_desc: b"<      go up a staircase\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = h_list {
                h_chstr: *::core::mem::transmute::<&[u8; 6], &mut [byte; 6]>(b"\0\0\0\0\0\0"),
                h_desc: b"Esc    cancel command\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = h_list {
                h_chstr: *::core::mem::transmute::<&[u8; 6], &mut [byte; 6]>(b"\0\0\0\0\0\0"),
                h_desc: b"d      drop object\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = h_list {
                h_chstr: *::core::mem::transmute::<&[u8; 6], &mut [byte; 6]>(b"\0\0\0\0\0\0"),
                h_desc: b"e      eat food\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = h_list {
                h_chstr: *::core::mem::transmute::<&[u8; 6], &mut [byte; 6]>(b"\0\0\0\0\0\0"),
                h_desc: b"f      <dir> find something\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = h_list {
                h_chstr: *::core::mem::transmute::<&[u8; 6], &mut [byte; 6]>(b"\0\0\0\0\0\0"),
                h_desc: b"q      quaff potion\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = h_list {
                h_chstr: *::core::mem::transmute::<&[u8; 6], &mut [byte; 6]>(b"\0\0\0\0\0\0"),
                h_desc: b"r      read paper\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = h_list {
                h_chstr: *::core::mem::transmute::<&[u8; 6], &mut [byte; 6]>(b"\0\0\0\0\0\0"),
                h_desc: b"s      search for trap/secret door\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = h_list {
                h_chstr: *::core::mem::transmute::<&[u8; 6], &mut [byte; 6]>(b"\0\0\0\0\0\0"),
                h_desc: b"t      <dir> throw something\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = h_list {
                h_chstr: *::core::mem::transmute::<&[u8; 6], &mut [byte; 6]>(b"\0\0\0\0\0\0"),
                h_desc: b"w      wield a weapon\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = h_list {
                h_chstr: *::core::mem::transmute::<&[u8; 6], &mut [byte; 6]>(b"\0\0\0\0\0\0"),
                h_desc: b"z      <dir> zap with a wand\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = h_list {
                h_chstr: *::core::mem::transmute::<&[u8; 6], &mut [byte; 6]>(b"\0\0\0\0\0\0"),
                h_desc: b"B      run down & left\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = h_list {
                h_chstr: *::core::mem::transmute::<&[u8; 6], &mut [byte; 6]>(b"\0\0\0\0\0\0"),
                h_desc: b"H      run left\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = h_list {
                h_chstr: *::core::mem::transmute::<&[u8; 6], &mut [byte; 6]>(b"\0\0\0\0\0\0"),
                h_desc: b"J      run down\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = h_list {
                h_chstr: *::core::mem::transmute::<&[u8; 6], &mut [byte; 6]>(b"\0\0\0\0\0\0"),
                h_desc: b"K      run up\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = h_list {
                h_chstr: *::core::mem::transmute::<&[u8; 6], &mut [byte; 6]>(b"\0\0\0\0\0\0"),
                h_desc: b"L      run right\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = h_list {
                h_chstr: *::core::mem::transmute::<&[u8; 6], &mut [byte; 6]>(b"\0\0\0\0\0\0"),
                h_desc: b"N      run down & right\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = h_list {
                h_chstr: *::core::mem::transmute::<&[u8; 6], &mut [byte; 6]>(b"\0\0\0\0\0\0"),
                h_desc: b"U      run up & right\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = h_list {
                h_chstr: *::core::mem::transmute::<&[u8; 6], &mut [byte; 6]>(b"\0\0\0\0\0\0"),
                h_desc: b"Y      run up & left\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = h_list {
                h_chstr: *::core::mem::transmute::<&[u8; 6], &mut [byte; 6]>(b"\0\0\0\0\0\0"),
                h_desc: b"W      wear armor\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = h_list {
                h_chstr: *::core::mem::transmute::<&[u8; 6], &mut [byte; 6]>(b"\0\0\0\0\0\0"),
                h_desc: b"T      take armor off\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = h_list {
                h_chstr: *::core::mem::transmute::<&[u8; 6], &mut [byte; 6]>(b"\0\0\0\0\0\0"),
                h_desc: b"P      put on ring\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = h_list {
                h_chstr: *::core::mem::transmute::<&[u8; 6], &mut [byte; 6]>(b"\0\0\0\0\0\0"),
                h_desc: b"Q      quit\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = h_list {
                h_chstr: *::core::mem::transmute::<&[u8; 6], &mut [byte; 6]>(b"\0\0\0\0\0\0"),
                h_desc: b"R      remove ring\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = h_list {
                h_chstr: *::core::mem::transmute::<&[u8; 6], &mut [byte; 6]>(b"\0\0\0\0\0\0"),
                h_desc: b"S      save game\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = h_list {
                h_chstr: *::core::mem::transmute::<&[u8; 6], &mut [byte; 6]>(b"\0\0\0\0\0\0"),
                h_desc: b"^      identify trap\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = h_list {
                h_chstr: *::core::mem::transmute::<&[u8; 6], &mut [byte; 6]>(b"\0\0\0\0\0\0"),
                h_desc: b"?      help\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = h_list {
                h_chstr: *::core::mem::transmute::<&[u8; 6], &mut [byte; 6]>(b"\0\0\0\0\0\0"),
                h_desc: b"/      key\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = h_list {
                h_chstr: *::core::mem::transmute::<&[u8; 6], &mut [byte; 6]>(b"\0\0\0\0\0\0"),
                h_desc: b"+      throw\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = h_list {
                h_chstr: *::core::mem::transmute::<&[u8; 6], &mut [byte; 6]>(b"\0\0\0\0\0\0"),
                h_desc: b"-      zap\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = h_list {
                h_chstr: *::core::mem::transmute::<&[u8; 6], &mut [byte; 6]>(b"\0\0\0\0\0\0"),
                h_desc: b"Ctrl t terse message format\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = h_list {
                h_chstr: *::core::mem::transmute::<&[u8; 6], &mut [byte; 6]>(b"\0\0\0\0\0\0"),
                h_desc: b"Ctrl r repeat message\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = h_list {
                h_chstr: *::core::mem::transmute::<&[u8; 6], &mut [byte; 6]>(b"\0\0\0\0\0\0"),
                h_desc: b"Del    search for something hidden\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = h_list {
                h_chstr: *::core::mem::transmute::<&[u8; 6], &mut [byte; 6]>(b"\0\0\0\0\0\0"),
                h_desc: b"Ins    <dir> find something\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = h_list {
                h_chstr: *::core::mem::transmute::<&[u8; 6], &mut [byte; 6]>(b"\0\0\0\0\0\0"),
                h_desc: b"a      repeat command\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = h_list {
                h_chstr: *::core::mem::transmute::<&[u8; 6], &mut [byte; 6]>(b"\0\0\0\0\0\0"),
                h_desc: b"c      rename something\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = h_list {
                h_chstr: *::core::mem::transmute::<&[u8; 6], &mut [byte; 6]>(b"\0\0\0\0\0\0"),
                h_desc: b"i      inventory\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = h_list {
                h_chstr: *::core::mem::transmute::<&[u8; 6], &mut [byte; 6]>(b"\0\0\0\0\0\0"),
                h_desc: b"v      version number\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = h_list {
                h_chstr: *::core::mem::transmute::<&[u8; 6], &mut [byte; 6]>(b"\0\0\0\0\0\0"),
                h_desc: b"!      Supervisor Key (fake DOS)\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = h_list {
                h_chstr: *::core::mem::transmute::<&[u8; 6], &mut [byte; 6]>(b"\0\0\0\0\0\0"),
                h_desc: b"D      list what has been discovered\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = h_list {
                h_chstr: *::core::mem::transmute::<&[u8; 6], &mut [byte; 6]>(b"\0\0\0\0\0\0"),
                h_desc: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
    ]
};
#[no_mangle]
pub static mut helpobjs: [h_list; 24] = unsafe {
    [
        {
            let mut init = h_list {
                h_chstr: [
                    0xfa as libc::c_int as byte,
                    ':' as i32 as byte,
                    ' ' as i32 as byte,
                    '\0' as i32 as byte,
                    0,
                    0,
                ],
                h_desc: b"the floor\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = h_list {
                h_chstr: [
                    0x1 as libc::c_int as byte,
                    ':' as i32 as byte,
                    ' ' as i32 as byte,
                    '\0' as i32 as byte,
                    0,
                    0,
                ],
                h_desc: b"the hero\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = h_list {
                h_chstr: [
                    0x5 as libc::c_int as byte,
                    ':' as i32 as byte,
                    ' ' as i32 as byte,
                    '\0' as i32 as byte,
                    0,
                    0,
                ],
                h_desc: b"some food\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = h_list {
                h_chstr: [
                    0xc as libc::c_int as byte,
                    ':' as i32 as byte,
                    ' ' as i32 as byte,
                    '\0' as i32 as byte,
                    0,
                    0,
                ],
                h_desc: b"the amulet of yendor\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = h_list {
                h_chstr: [
                    0xd as libc::c_int as byte,
                    ':' as i32 as byte,
                    ' ' as i32 as byte,
                    '\0' as i32 as byte,
                    0,
                    0,
                ],
                h_desc: b"a scroll\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = h_list {
                h_chstr: [
                    0x18 as libc::c_int as byte,
                    ':' as i32 as byte,
                    ' ' as i32 as byte,
                    '\0' as i32 as byte,
                    0,
                    0,
                ],
                h_desc: b"a weapon\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = h_list {
                h_chstr: [
                    0x8 as libc::c_int as byte,
                    ':' as i32 as byte,
                    ' ' as i32 as byte,
                    '\0' as i32 as byte,
                    0,
                    0,
                ],
                h_desc: b"a piece of armor\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = h_list {
                h_chstr: [
                    0xf as libc::c_int as byte,
                    ':' as i32 as byte,
                    ' ' as i32 as byte,
                    '\0' as i32 as byte,
                    0,
                    0,
                ],
                h_desc: b"some gold\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = h_list {
                h_chstr: [
                    0xe7 as libc::c_int as byte,
                    ':' as i32 as byte,
                    ' ' as i32 as byte,
                    '\0' as i32 as byte,
                    0,
                    0,
                ],
                h_desc: b"a magic staff\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = h_list {
                h_chstr: [
                    0xad as libc::c_int as byte,
                    ':' as i32 as byte,
                    ' ' as i32 as byte,
                    '\0' as i32 as byte,
                    0,
                    0,
                ],
                h_desc: b"a potion\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = h_list {
                h_chstr: [
                    0x9 as libc::c_int as byte,
                    ':' as i32 as byte,
                    ' ' as i32 as byte,
                    '\0' as i32 as byte,
                    0,
                    0,
                ],
                h_desc: b"a magic ring\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = h_list {
                h_chstr: [
                    0xb2 as libc::c_int as byte,
                    ':' as i32 as byte,
                    ' ' as i32 as byte,
                    '\0' as i32 as byte,
                    0,
                    0,
                ],
                h_desc: b"a passage\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = h_list {
                h_chstr: [
                    0xce as libc::c_int as byte,
                    ':' as i32 as byte,
                    ' ' as i32 as byte,
                    '\0' as i32 as byte,
                    0,
                    0,
                ],
                h_desc: b"a door\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = h_list {
                h_chstr: [
                    0xc9 as libc::c_int as byte,
                    ':' as i32 as byte,
                    ' ' as i32 as byte,
                    '\0' as i32 as byte,
                    0,
                    0,
                ],
                h_desc: b"an upper left corner\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = h_list {
                h_chstr: [
                    0x4 as libc::c_int as byte,
                    ':' as i32 as byte,
                    ' ' as i32 as byte,
                    '\0' as i32 as byte,
                    0,
                    0,
                ],
                h_desc: b"a trap\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = h_list {
                h_chstr: [
                    0xcd as libc::c_int as byte,
                    ':' as i32 as byte,
                    ' ' as i32 as byte,
                    '\0' as i32 as byte,
                    0,
                    0,
                ],
                h_desc: b"a horizontal wall\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = h_list {
                h_chstr: [
                    0xbc as libc::c_int as byte,
                    ':' as i32 as byte,
                    ' ' as i32 as byte,
                    '\0' as i32 as byte,
                    0,
                    0,
                ],
                h_desc: b"a lower right corner\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = h_list {
                h_chstr: [
                    0xc8 as libc::c_int as byte,
                    ':' as i32 as byte,
                    ' ' as i32 as byte,
                    '\0' as i32 as byte,
                    0,
                    0,
                ],
                h_desc: b"a lower left corner\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = h_list {
                h_chstr: [
                    0xba as libc::c_int as byte,
                    ':' as i32 as byte,
                    ' ' as i32 as byte,
                    '\0' as i32 as byte,
                    0,
                    0,
                ],
                h_desc: b"a vertical wall\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = h_list {
                h_chstr: [
                    0xbb as libc::c_int as byte,
                    ':' as i32 as byte,
                    ' ' as i32 as byte,
                    '\0' as i32 as byte,
                    0,
                    0,
                ],
                h_desc: b"an upper right corner\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = h_list {
                h_chstr: [
                    0xf0 as libc::c_int as byte,
                    ':' as i32 as byte,
                    ' ' as i32 as byte,
                    '\0' as i32 as byte,
                    0,
                    0,
                ],
                h_desc: b"a stair case\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = h_list {
                h_chstr: [
                    '$' as i32 as byte,
                    ',' as i32 as byte,
                    '~' as i32 as byte,
                    ':' as i32 as byte,
                    ' ' as i32 as byte,
                    '\0' as i32 as byte,
                ],
                h_desc: b"safe and perilous magic\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = h_list {
                h_chstr: [
                    'A' as i32 as byte,
                    '-' as i32 as byte,
                    'Z' as i32 as byte,
                    ':' as i32 as byte,
                    ' ' as i32 as byte,
                    '\0' as i32 as byte,
                ],
                h_desc: b"26 different monsters\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = h_list {
                h_chstr: *::core::mem::transmute::<&[u8; 6], &mut [byte; 6]>(b"\0\0\0\0\0\0"),
                h_desc: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
    ]
};
#[no_mangle]
pub static mut he_man: [*mut libc::c_char; 21] = [
    b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Guild Novice\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Apprentice\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Journeyman\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Adventurer\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Fighter\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Warrior\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Rogue\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Champion\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Master Rogue\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Warlord\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Hero\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Guild Master\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Dragonlord\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Wizard\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Rogue Geek\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Rogue Addict\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Schmendrick\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Gunfighter\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Time Waster\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Bug Chaser\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
];
#[no_mangle]
pub static mut maxitems: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut reinit: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut after: bool = false;
#[no_mangle]
pub static mut noscore: bool = false;
#[no_mangle]
pub static mut again: bool = false;
#[no_mangle]
pub static mut s_know: [bool; 15] = [false; 15];
#[no_mangle]
pub static mut p_know: [bool; 14] = [false; 14];
#[no_mangle]
pub static mut r_know: [bool; 14] = [false; 14];
#[no_mangle]
pub static mut ws_know: [bool; 14] = [false; 14];
#[no_mangle]
pub static mut amulet: bool = 0 as libc::c_int != 0;
#[no_mangle]
pub static mut saw_amulet: bool = 0 as libc::c_int != 0;
#[no_mangle]
pub static mut door_stop: bool = 0 as libc::c_int != 0;
#[no_mangle]
pub static mut fastmode: bool = 0 as libc::c_int != 0;
#[no_mangle]
pub static mut faststate: bool = 0 as libc::c_int != 0;
#[no_mangle]
pub static mut firstmove: bool = 0 as libc::c_int != 0;
#[no_mangle]
pub static mut playing: bool = 1 as libc::c_int != 0;
#[no_mangle]
pub static mut running: bool = 0 as libc::c_int != 0;
#[no_mangle]
pub static mut save_msg: bool = 1 as libc::c_int != 0;
#[no_mangle]
pub static mut terse: bool = 0 as libc::c_int != 0;
#[no_mangle]
pub static mut expert: bool = 0 as libc::c_int != 0;
#[no_mangle]
pub static mut was_trapped: libc::c_uchar = 0 as libc::c_int as libc::c_uchar;
#[no_mangle]
pub static mut bailout: bool = 0 as libc::c_int != 0;
#[no_mangle]
pub static mut take: libc::c_char = 0;
#[no_mangle]
pub static mut runch: libc::c_char = 0;
#[no_mangle]
pub static mut s_names: [array; 15] = [array { storage: [0; 21] }; 15];
#[no_mangle]
pub static mut p_colors: [*mut libc::c_char; 14] =
    [0 as *const libc::c_char as *mut libc::c_char; 14];
#[no_mangle]
pub static mut r_stones: [*mut libc::c_char; 14] =
    [0 as *const libc::c_char as *mut libc::c_char; 14];
#[no_mangle]
pub static mut ws_made: [*mut libc::c_char; 14] =
    [0 as *const libc::c_char as *mut libc::c_char; 14];
#[no_mangle]
pub static mut huh: [libc::c_char; 128] = [0; 128];
#[no_mangle]
pub static mut s_guess: [*mut libc::c_char; 15] =
    [0 as *const libc::c_char as *mut libc::c_char; 15];
#[no_mangle]
pub static mut p_guess: [*mut libc::c_char; 14] =
    [0 as *const libc::c_char as *mut libc::c_char; 14];
#[no_mangle]
pub static mut r_guess: [*mut libc::c_char; 14] =
    [0 as *const libc::c_char as *mut libc::c_char; 14];
#[no_mangle]
pub static mut ws_guess: [*mut libc::c_char; 14] =
    [0 as *const libc::c_char as *mut libc::c_char; 14];
#[no_mangle]
pub static mut _guesses: [array; 57] = [array { storage: [0; 21] }; 57];
#[no_mangle]
pub static mut iguess: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut ws_type: [*mut libc::c_char; 14] =
    [0 as *const libc::c_char as *mut libc::c_char; 14];
#[no_mangle]
pub static mut maxrow: libc::c_int = 0;
#[no_mangle]
pub static mut max_level: libc::c_int = 0;
#[no_mangle]
pub static mut ntraps: libc::c_int = 0;
#[no_mangle]
pub static mut dnum: libc::c_int = 0;
#[no_mangle]
pub static mut level: libc::c_int = 1 as libc::c_int;
#[no_mangle]
pub static mut purse: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut mpos: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut no_move: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut no_command: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut inpack: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut total: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut no_food: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut count: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut fung_hit: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut quiet: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut food_left: libc::c_int = 0;
#[no_mangle]
pub static mut group: libc::c_int = 2 as libc::c_int;
#[no_mangle]
pub static mut hungry_state: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut cksum: libc::c_int = -(1632 as libc::c_int);
#[no_mangle]
pub static mut seed: libc::c_long = 0;
#[no_mangle]
pub static mut hit_mul: libc::c_int = 6 as libc::c_int;
#[no_mangle]
pub static mut goodchk: libc::c_int = 1 as libc::c_int;
#[no_mangle]
pub static mut your_na: *mut libc::c_char =
    b"Software Pirate\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut kild_by: *mut libc::c_char =
    b"Copy Protection Mafia\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut _whoami: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut oldpos: coord = coord { x: 0, y: 0 };
#[no_mangle]
pub static mut delta: coord = coord { x: 0, y: 0 };
#[no_mangle]
pub static mut cur_armor: *mut THING = 0 as *const THING as *mut THING;
#[no_mangle]
pub static mut cur_ring: [*mut THING; 2] = [0 as *const THING as *mut THING; 2];
#[no_mangle]
pub static mut cur_weapon: *mut THING = 0 as *const THING as *mut THING;
#[no_mangle]
pub static mut oldrp: *mut room = 0 as *const room as *mut room;
#[no_mangle]
pub static mut rooms: [room; 9] = [room {
    r_pos: coord { x: 0, y: 0 },
    r_max: coord { x: 0, y: 0 },
    r_gold: coord { x: 0, y: 0 },
    r_goldval: 0,
    r_flags: 0,
    r_nexits: 0,
    r_exit: [coord { x: 0, y: 0 }; 12],
}; 9];
#[no_mangle]
pub static mut passages: [room; 13] = [
    {
        let mut init = room {
            r_pos: {
                let mut init = coord {
                    x: 0 as libc::c_int,
                    y: 0 as libc::c_int,
                };
                init
            },
            r_max: {
                let mut init = coord {
                    x: 0 as libc::c_int,
                    y: 0 as libc::c_int,
                };
                init
            },
            r_gold: {
                let mut init = coord {
                    x: 0 as libc::c_int,
                    y: 0 as libc::c_int,
                };
                init
            },
            r_goldval: 0 as libc::c_int,
            r_flags: (0x2 as libc::c_int | 0x1 as libc::c_int) as libc::c_short,
            r_nexits: 0 as libc::c_int,
            r_exit: [
                {
                    let mut init = coord {
                        x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = coord {
                        x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = coord {
                        x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = coord {
                        x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = coord {
                        x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = coord {
                        x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = coord {
                        x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = coord {
                        x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = coord {
                        x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = coord {
                        x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = coord {
                        x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = coord {
                        x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                    };
                    init
                },
            ],
        };
        init
    },
    {
        let mut init = room {
            r_pos: {
                let mut init = coord {
                    x: 0 as libc::c_int,
                    y: 0 as libc::c_int,
                };
                init
            },
            r_max: {
                let mut init = coord {
                    x: 0 as libc::c_int,
                    y: 0 as libc::c_int,
                };
                init
            },
            r_gold: {
                let mut init = coord {
                    x: 0 as libc::c_int,
                    y: 0 as libc::c_int,
                };
                init
            },
            r_goldval: 0 as libc::c_int,
            r_flags: (0x2 as libc::c_int | 0x1 as libc::c_int) as libc::c_short,
            r_nexits: 0 as libc::c_int,
            r_exit: [
                {
                    let mut init = coord {
                        x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = coord {
                        x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = coord {
                        x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = coord {
                        x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = coord {
                        x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = coord {
                        x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = coord {
                        x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = coord {
                        x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = coord {
                        x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = coord {
                        x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = coord {
                        x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = coord {
                        x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                    };
                    init
                },
            ],
        };
        init
    },
    {
        let mut init = room {
            r_pos: {
                let mut init = coord {
                    x: 0 as libc::c_int,
                    y: 0 as libc::c_int,
                };
                init
            },
            r_max: {
                let mut init = coord {
                    x: 0 as libc::c_int,
                    y: 0 as libc::c_int,
                };
                init
            },
            r_gold: {
                let mut init = coord {
                    x: 0 as libc::c_int,
                    y: 0 as libc::c_int,
                };
                init
            },
            r_goldval: 0 as libc::c_int,
            r_flags: (0x2 as libc::c_int | 0x1 as libc::c_int) as libc::c_short,
            r_nexits: 0 as libc::c_int,
            r_exit: [
                {
                    let mut init = coord {
                        x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = coord {
                        x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = coord {
                        x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = coord {
                        x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = coord {
                        x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = coord {
                        x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = coord {
                        x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = coord {
                        x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = coord {
                        x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = coord {
                        x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = coord {
                        x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = coord {
                        x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                    };
                    init
                },
            ],
        };
        init
    },
    {
        let mut init = room {
            r_pos: {
                let mut init = coord {
                    x: 0 as libc::c_int,
                    y: 0 as libc::c_int,
                };
                init
            },
            r_max: {
                let mut init = coord {
                    x: 0 as libc::c_int,
                    y: 0 as libc::c_int,
                };
                init
            },
            r_gold: {
                let mut init = coord {
                    x: 0 as libc::c_int,
                    y: 0 as libc::c_int,
                };
                init
            },
            r_goldval: 0 as libc::c_int,
            r_flags: (0x2 as libc::c_int | 0x1 as libc::c_int) as libc::c_short,
            r_nexits: 0 as libc::c_int,
            r_exit: [
                {
                    let mut init = coord {
                        x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = coord {
                        x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = coord {
                        x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = coord {
                        x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = coord {
                        x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = coord {
                        x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = coord {
                        x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = coord {
                        x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = coord {
                        x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = coord {
                        x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = coord {
                        x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = coord {
                        x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                    };
                    init
                },
            ],
        };
        init
    },
    {
        let mut init = room {
            r_pos: {
                let mut init = coord {
                    x: 0 as libc::c_int,
                    y: 0 as libc::c_int,
                };
                init
            },
            r_max: {
                let mut init = coord {
                    x: 0 as libc::c_int,
                    y: 0 as libc::c_int,
                };
                init
            },
            r_gold: {
                let mut init = coord {
                    x: 0 as libc::c_int,
                    y: 0 as libc::c_int,
                };
                init
            },
            r_goldval: 0 as libc::c_int,
            r_flags: (0x2 as libc::c_int | 0x1 as libc::c_int) as libc::c_short,
            r_nexits: 0 as libc::c_int,
            r_exit: [
                {
                    let mut init = coord {
                        x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = coord {
                        x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = coord {
                        x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = coord {
                        x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = coord {
                        x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = coord {
                        x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = coord {
                        x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = coord {
                        x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = coord {
                        x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = coord {
                        x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = coord {
                        x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = coord {
                        x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                    };
                    init
                },
            ],
        };
        init
    },
    {
        let mut init = room {
            r_pos: {
                let mut init = coord {
                    x: 0 as libc::c_int,
                    y: 0 as libc::c_int,
                };
                init
            },
            r_max: {
                let mut init = coord {
                    x: 0 as libc::c_int,
                    y: 0 as libc::c_int,
                };
                init
            },
            r_gold: {
                let mut init = coord {
                    x: 0 as libc::c_int,
                    y: 0 as libc::c_int,
                };
                init
            },
            r_goldval: 0 as libc::c_int,
            r_flags: (0x2 as libc::c_int | 0x1 as libc::c_int) as libc::c_short,
            r_nexits: 0 as libc::c_int,
            r_exit: [
                {
                    let mut init = coord {
                        x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = coord {
                        x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = coord {
                        x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = coord {
                        x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = coord {
                        x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = coord {
                        x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = coord {
                        x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = coord {
                        x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = coord {
                        x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = coord {
                        x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = coord {
                        x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = coord {
                        x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                    };
                    init
                },
            ],
        };
        init
    },
    {
        let mut init = room {
            r_pos: {
                let mut init = coord {
                    x: 0 as libc::c_int,
                    y: 0 as libc::c_int,
                };
                init
            },
            r_max: {
                let mut init = coord {
                    x: 0 as libc::c_int,
                    y: 0 as libc::c_int,
                };
                init
            },
            r_gold: {
                let mut init = coord {
                    x: 0 as libc::c_int,
                    y: 0 as libc::c_int,
                };
                init
            },
            r_goldval: 0 as libc::c_int,
            r_flags: (0x2 as libc::c_int | 0x1 as libc::c_int) as libc::c_short,
            r_nexits: 0 as libc::c_int,
            r_exit: [
                {
                    let mut init = coord {
                        x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = coord {
                        x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = coord {
                        x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = coord {
                        x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = coord {
                        x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = coord {
                        x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = coord {
                        x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = coord {
                        x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = coord {
                        x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = coord {
                        x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = coord {
                        x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = coord {
                        x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                    };
                    init
                },
            ],
        };
        init
    },
    {
        let mut init = room {
            r_pos: {
                let mut init = coord {
                    x: 0 as libc::c_int,
                    y: 0 as libc::c_int,
                };
                init
            },
            r_max: {
                let mut init = coord {
                    x: 0 as libc::c_int,
                    y: 0 as libc::c_int,
                };
                init
            },
            r_gold: {
                let mut init = coord {
                    x: 0 as libc::c_int,
                    y: 0 as libc::c_int,
                };
                init
            },
            r_goldval: 0 as libc::c_int,
            r_flags: (0x2 as libc::c_int | 0x1 as libc::c_int) as libc::c_short,
            r_nexits: 0 as libc::c_int,
            r_exit: [
                {
                    let mut init = coord {
                        x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = coord {
                        x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = coord {
                        x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = coord {
                        x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = coord {
                        x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = coord {
                        x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = coord {
                        x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = coord {
                        x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = coord {
                        x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = coord {
                        x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = coord {
                        x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = coord {
                        x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                    };
                    init
                },
            ],
        };
        init
    },
    {
        let mut init = room {
            r_pos: {
                let mut init = coord {
                    x: 0 as libc::c_int,
                    y: 0 as libc::c_int,
                };
                init
            },
            r_max: {
                let mut init = coord {
                    x: 0 as libc::c_int,
                    y: 0 as libc::c_int,
                };
                init
            },
            r_gold: {
                let mut init = coord {
                    x: 0 as libc::c_int,
                    y: 0 as libc::c_int,
                };
                init
            },
            r_goldval: 0 as libc::c_int,
            r_flags: (0x2 as libc::c_int | 0x1 as libc::c_int) as libc::c_short,
            r_nexits: 0 as libc::c_int,
            r_exit: [
                {
                    let mut init = coord {
                        x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = coord {
                        x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = coord {
                        x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = coord {
                        x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = coord {
                        x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = coord {
                        x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = coord {
                        x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = coord {
                        x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = coord {
                        x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = coord {
                        x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = coord {
                        x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = coord {
                        x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                    };
                    init
                },
            ],
        };
        init
    },
    {
        let mut init = room {
            r_pos: {
                let mut init = coord {
                    x: 0 as libc::c_int,
                    y: 0 as libc::c_int,
                };
                init
            },
            r_max: {
                let mut init = coord {
                    x: 0 as libc::c_int,
                    y: 0 as libc::c_int,
                };
                init
            },
            r_gold: {
                let mut init = coord {
                    x: 0 as libc::c_int,
                    y: 0 as libc::c_int,
                };
                init
            },
            r_goldval: 0 as libc::c_int,
            r_flags: (0x2 as libc::c_int | 0x1 as libc::c_int) as libc::c_short,
            r_nexits: 0 as libc::c_int,
            r_exit: [
                {
                    let mut init = coord {
                        x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = coord {
                        x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = coord {
                        x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = coord {
                        x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = coord {
                        x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = coord {
                        x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = coord {
                        x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = coord {
                        x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = coord {
                        x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = coord {
                        x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = coord {
                        x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = coord {
                        x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                    };
                    init
                },
            ],
        };
        init
    },
    {
        let mut init = room {
            r_pos: {
                let mut init = coord {
                    x: 0 as libc::c_int,
                    y: 0 as libc::c_int,
                };
                init
            },
            r_max: {
                let mut init = coord {
                    x: 0 as libc::c_int,
                    y: 0 as libc::c_int,
                };
                init
            },
            r_gold: {
                let mut init = coord {
                    x: 0 as libc::c_int,
                    y: 0 as libc::c_int,
                };
                init
            },
            r_goldval: 0 as libc::c_int,
            r_flags: (0x2 as libc::c_int | 0x1 as libc::c_int) as libc::c_short,
            r_nexits: 0 as libc::c_int,
            r_exit: [
                {
                    let mut init = coord {
                        x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = coord {
                        x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = coord {
                        x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = coord {
                        x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = coord {
                        x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = coord {
                        x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = coord {
                        x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = coord {
                        x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = coord {
                        x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = coord {
                        x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = coord {
                        x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = coord {
                        x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                    };
                    init
                },
            ],
        };
        init
    },
    {
        let mut init = room {
            r_pos: {
                let mut init = coord {
                    x: 0 as libc::c_int,
                    y: 0 as libc::c_int,
                };
                init
            },
            r_max: {
                let mut init = coord {
                    x: 0 as libc::c_int,
                    y: 0 as libc::c_int,
                };
                init
            },
            r_gold: {
                let mut init = coord {
                    x: 0 as libc::c_int,
                    y: 0 as libc::c_int,
                };
                init
            },
            r_goldval: 0 as libc::c_int,
            r_flags: (0x2 as libc::c_int | 0x1 as libc::c_int) as libc::c_short,
            r_nexits: 0 as libc::c_int,
            r_exit: [
                {
                    let mut init = coord {
                        x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = coord {
                        x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = coord {
                        x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = coord {
                        x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = coord {
                        x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = coord {
                        x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = coord {
                        x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = coord {
                        x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = coord {
                        x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = coord {
                        x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = coord {
                        x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = coord {
                        x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                    };
                    init
                },
            ],
        };
        init
    },
    room {
        r_pos: coord { x: 0, y: 0 },
        r_max: coord { x: 0, y: 0 },
        r_gold: coord { x: 0, y: 0 },
        r_goldval: 0,
        r_flags: 0,
        r_nexits: 0,
        r_exit: [coord { x: 0, y: 0 }; 12],
    },
];
#[no_mangle]
pub static mut max_stats: stats = {
    let mut init = stats {
        s_str: 16 as libc::c_int as str_t,
        s_exp: 0 as libc::c_int as libc::c_long,
        s_lvl: 1 as libc::c_int,
        s_arm: 10 as libc::c_int,
        s_hpt: 12 as libc::c_int,
        s_dmg: b"1d4\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        s_maxhp: 12 as libc::c_int,
    };
    init
};
#[no_mangle]
pub static mut player: THING = thing {
    _t: C2RustUnnamed_0 {
        _l_next: 0 as *const thing as *mut thing,
        _l_prev: 0 as *const thing as *mut thing,
        _t_pos: coord { x: 0, y: 0 },
        _t_turn: 0,
        _t_type: 0,
        _t_disguise: 0,
        _t_oldch: 0,
        _t_dest: 0 as *const coord as *mut coord,
        _t_flags: 0,
        _t_stats: stats {
            s_str: 0,
            s_exp: 0,
            s_lvl: 0,
            s_arm: 0,
            s_hpt: 0,
            s_dmg: 0 as *const libc::c_char as *mut libc::c_char,
            s_maxhp: 0,
        },
        _t_room: 0 as *const room as *mut room,
        _t_pack: 0 as *const thing as *mut thing,
    },
};
#[no_mangle]
pub static mut lvl_obj: *mut THING = 0 as *const THING as *mut THING;
#[no_mangle]
pub static mut mlist: *mut THING = 0 as *const THING as *mut THING;
#[no_mangle]
pub static mut monsters: [monster; 26] = [
    {
        let mut init = monster {
            m_name: b"aquator\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            m_carry: 0 as libc::c_int,
            m_flags: 0x20 as libc::c_int as libc::c_ushort,
            m_stats: {
                let mut init = stats {
                    s_str: 10 as libc::c_int as str_t,
                    s_exp: 20 as libc::c_int as libc::c_long,
                    s_lvl: 5 as libc::c_int,
                    s_arm: 2 as libc::c_int,
                    s_hpt: 1 as libc::c_int,
                    s_dmg: b"0d0/0d0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    s_maxhp: 1 as libc::c_int,
                };
                init
            },
        };
        init
    },
    {
        let mut init = monster {
            m_name: b"bat\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            m_carry: 0 as libc::c_int,
            m_flags: 0x8000 as libc::c_int as libc::c_ushort,
            m_stats: {
                let mut init = stats {
                    s_str: 10 as libc::c_int as str_t,
                    s_exp: 1 as libc::c_int as libc::c_long,
                    s_lvl: 1 as libc::c_int,
                    s_arm: 3 as libc::c_int,
                    s_hpt: 1 as libc::c_int,
                    s_dmg: b"1d2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    s_maxhp: 1 as libc::c_int,
                };
                init
            },
        };
        init
    },
    {
        let mut init = monster {
            m_name: b"centaur\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            m_carry: 15 as libc::c_int,
            m_flags: 0 as libc::c_int as libc::c_ushort,
            m_stats: {
                let mut init = stats {
                    s_str: 10 as libc::c_int as str_t,
                    s_exp: 25 as libc::c_int as libc::c_long,
                    s_lvl: 4 as libc::c_int,
                    s_arm: 4 as libc::c_int,
                    s_hpt: 1 as libc::c_int,
                    s_dmg: b"1d6/1d6\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    s_maxhp: 1 as libc::c_int,
                };
                init
            },
        };
        init
    },
    {
        let mut init = monster {
            m_name: b"dragon\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            m_carry: 100 as libc::c_int,
            m_flags: 0x20 as libc::c_int as libc::c_ushort,
            m_stats: {
                let mut init = stats {
                    s_str: 10 as libc::c_int as str_t,
                    s_exp: 6800 as libc::c_int as libc::c_long,
                    s_lvl: 10 as libc::c_int,
                    s_arm: -(1 as libc::c_int),
                    s_hpt: 1 as libc::c_int,
                    s_dmg: b"1d8/1d8/3d10\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    s_maxhp: 1 as libc::c_int,
                };
                init
            },
        };
        init
    },
    {
        let mut init = monster {
            m_name: b"emu\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            m_carry: 0 as libc::c_int,
            m_flags: 0x20 as libc::c_int as libc::c_ushort,
            m_stats: {
                let mut init = stats {
                    s_str: 10 as libc::c_int as str_t,
                    s_exp: 2 as libc::c_int as libc::c_long,
                    s_lvl: 1 as libc::c_int,
                    s_arm: 7 as libc::c_int,
                    s_hpt: 1 as libc::c_int,
                    s_dmg: b"1d2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    s_maxhp: 1 as libc::c_int,
                };
                init
            },
        };
        init
    },
    {
        let mut init = monster {
            m_name: b"venus flytrap\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            m_carry: 0 as libc::c_int,
            m_flags: 0x20 as libc::c_int as libc::c_ushort,
            m_stats: {
                let mut init = stats {
                    s_str: 10 as libc::c_int as str_t,
                    s_exp: 80 as libc::c_int as libc::c_long,
                    s_lvl: 8 as libc::c_int,
                    s_arm: 3 as libc::c_int,
                    s_hpt: 1 as libc::c_int,
                    s_dmg: b"%%%d0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    s_maxhp: 1 as libc::c_int,
                };
                init
            },
        };
        init
    },
    {
        let mut init = monster {
            m_name: b"griffin\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            m_carry: 20 as libc::c_int,
            m_flags: (0x20 as libc::c_int | 0x8000 as libc::c_int | 0x200 as libc::c_int)
                as libc::c_ushort,
            m_stats: {
                let mut init = stats {
                    s_str: 10 as libc::c_int as str_t,
                    s_exp: 2000 as libc::c_int as libc::c_long,
                    s_lvl: 13 as libc::c_int,
                    s_arm: 2 as libc::c_int,
                    s_hpt: 1 as libc::c_int,
                    s_dmg: b"4d3/3d5/4d3\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    s_maxhp: 1 as libc::c_int,
                };
                init
            },
        };
        init
    },
    {
        let mut init = monster {
            m_name: b"hobgoblin\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            m_carry: 0 as libc::c_int,
            m_flags: 0x20 as libc::c_int as libc::c_ushort,
            m_stats: {
                let mut init = stats {
                    s_str: 10 as libc::c_int as str_t,
                    s_exp: 3 as libc::c_int as libc::c_long,
                    s_lvl: 1 as libc::c_int,
                    s_arm: 5 as libc::c_int,
                    s_hpt: 1 as libc::c_int,
                    s_dmg: b"1d8\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    s_maxhp: 1 as libc::c_int,
                };
                init
            },
        };
        init
    },
    {
        let mut init = monster {
            m_name: b"ice monster\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            m_carry: 0 as libc::c_int,
            m_flags: 0x20 as libc::c_int as libc::c_ushort,
            m_stats: {
                let mut init = stats {
                    s_str: 10 as libc::c_int as str_t,
                    s_exp: 15 as libc::c_int as libc::c_long,
                    s_lvl: 1 as libc::c_int,
                    s_arm: 9 as libc::c_int,
                    s_hpt: 1 as libc::c_int,
                    s_dmg: b"1d2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    s_maxhp: 1 as libc::c_int,
                };
                init
            },
        };
        init
    },
    {
        let mut init = monster {
            m_name: b"jabberwock\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            m_carry: 70 as libc::c_int,
            m_flags: 0 as libc::c_int as libc::c_ushort,
            m_stats: {
                let mut init = stats {
                    s_str: 10 as libc::c_int as str_t,
                    s_exp: 4000 as libc::c_int as libc::c_long,
                    s_lvl: 15 as libc::c_int,
                    s_arm: 6 as libc::c_int,
                    s_hpt: 1 as libc::c_int,
                    s_dmg: b"2d12/2d4\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    s_maxhp: 1 as libc::c_int,
                };
                init
            },
        };
        init
    },
    {
        let mut init = monster {
            m_name: b"kestral\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            m_carry: 0 as libc::c_int,
            m_flags: (0x20 as libc::c_int | 0x8000 as libc::c_int) as libc::c_ushort,
            m_stats: {
                let mut init = stats {
                    s_str: 10 as libc::c_int as str_t,
                    s_exp: 1 as libc::c_int as libc::c_long,
                    s_lvl: 1 as libc::c_int,
                    s_arm: 7 as libc::c_int,
                    s_hpt: 1 as libc::c_int,
                    s_dmg: b"1d4\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    s_maxhp: 1 as libc::c_int,
                };
                init
            },
        };
        init
    },
    {
        let mut init = monster {
            m_name: b"leprechaun\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            m_carry: 0x40 as libc::c_int,
            m_flags: 0 as libc::c_int as libc::c_ushort,
            m_stats: {
                let mut init = stats {
                    s_str: 10 as libc::c_int as str_t,
                    s_exp: 10 as libc::c_int as libc::c_long,
                    s_lvl: 3 as libc::c_int,
                    s_arm: 8 as libc::c_int,
                    s_hpt: 1 as libc::c_int,
                    s_dmg: b"1d2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    s_maxhp: 1 as libc::c_int,
                };
                init
            },
        };
        init
    },
    {
        let mut init = monster {
            m_name: b"medusa\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            m_carry: 40 as libc::c_int,
            m_flags: 0x20 as libc::c_int as libc::c_ushort,
            m_stats: {
                let mut init = stats {
                    s_str: 10 as libc::c_int as str_t,
                    s_exp: 200 as libc::c_int as libc::c_long,
                    s_lvl: 8 as libc::c_int,
                    s_arm: 2 as libc::c_int,
                    s_hpt: 1 as libc::c_int,
                    s_dmg: b"3d4/3d4/2d5\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    s_maxhp: 1 as libc::c_int,
                };
                init
            },
        };
        init
    },
    {
        let mut init = monster {
            m_name: b"nymph\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            m_carry: 100 as libc::c_int,
            m_flags: 0 as libc::c_int as libc::c_ushort,
            m_stats: {
                let mut init = stats {
                    s_str: 10 as libc::c_int as str_t,
                    s_exp: 37 as libc::c_int as libc::c_long,
                    s_lvl: 3 as libc::c_int,
                    s_arm: 9 as libc::c_int,
                    s_hpt: 1 as libc::c_int,
                    s_dmg: b"0d0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    s_maxhp: 1 as libc::c_int,
                };
                init
            },
        };
        init
    },
    {
        let mut init = monster {
            m_name: b"orc\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            m_carry: 15 as libc::c_int,
            m_flags: 0x40 as libc::c_int as libc::c_ushort,
            m_stats: {
                let mut init = stats {
                    s_str: 10 as libc::c_int as str_t,
                    s_exp: 5 as libc::c_int as libc::c_long,
                    s_lvl: 1 as libc::c_int,
                    s_arm: 6 as libc::c_int,
                    s_hpt: 1 as libc::c_int,
                    s_dmg: b"1d8\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    s_maxhp: 1 as libc::c_int,
                };
                init
            },
        };
        init
    },
    {
        let mut init = monster {
            m_name: b"phantom\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            m_carry: 0 as libc::c_int,
            m_flags: 0x10 as libc::c_int as libc::c_ushort,
            m_stats: {
                let mut init = stats {
                    s_str: 10 as libc::c_int as str_t,
                    s_exp: 120 as libc::c_int as libc::c_long,
                    s_lvl: 8 as libc::c_int,
                    s_arm: 3 as libc::c_int,
                    s_hpt: 1 as libc::c_int,
                    s_dmg: b"4d4\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    s_maxhp: 1 as libc::c_int,
                };
                init
            },
        };
        init
    },
    {
        let mut init = monster {
            m_name: b"quagga\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            m_carry: 30 as libc::c_int,
            m_flags: 0x20 as libc::c_int as libc::c_ushort,
            m_stats: {
                let mut init = stats {
                    s_str: 10 as libc::c_int as str_t,
                    s_exp: 32 as libc::c_int as libc::c_long,
                    s_lvl: 3 as libc::c_int,
                    s_arm: 2 as libc::c_int,
                    s_hpt: 1 as libc::c_int,
                    s_dmg: b"1d2/1d2/1d4\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    s_maxhp: 1 as libc::c_int,
                };
                init
            },
        };
        init
    },
    {
        let mut init = monster {
            m_name: b"rattlesnake\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            m_carry: 0 as libc::c_int,
            m_flags: 0x20 as libc::c_int as libc::c_ushort,
            m_stats: {
                let mut init = stats {
                    s_str: 10 as libc::c_int as str_t,
                    s_exp: 9 as libc::c_int as libc::c_long,
                    s_lvl: 2 as libc::c_int,
                    s_arm: 3 as libc::c_int,
                    s_hpt: 1 as libc::c_int,
                    s_dmg: b"1d6\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    s_maxhp: 1 as libc::c_int,
                };
                init
            },
        };
        init
    },
    {
        let mut init = monster {
            m_name: b"slime\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            m_carry: 0 as libc::c_int,
            m_flags: 0x20 as libc::c_int as libc::c_ushort,
            m_stats: {
                let mut init = stats {
                    s_str: 10 as libc::c_int as str_t,
                    s_exp: 1 as libc::c_int as libc::c_long,
                    s_lvl: 2 as libc::c_int,
                    s_arm: 8 as libc::c_int,
                    s_hpt: 1 as libc::c_int,
                    s_dmg: b"1d3\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    s_maxhp: 1 as libc::c_int,
                };
                init
            },
        };
        init
    },
    {
        let mut init = monster {
            m_name: b"troll\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            m_carry: 50 as libc::c_int,
            m_flags: (0x200 as libc::c_int | 0x20 as libc::c_int) as libc::c_ushort,
            m_stats: {
                let mut init = stats {
                    s_str: 10 as libc::c_int as str_t,
                    s_exp: 120 as libc::c_int as libc::c_long,
                    s_lvl: 6 as libc::c_int,
                    s_arm: 4 as libc::c_int,
                    s_hpt: 1 as libc::c_int,
                    s_dmg: b"1d8/1d8/2d6\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    s_maxhp: 1 as libc::c_int,
                };
                init
            },
        };
        init
    },
    {
        let mut init = monster {
            m_name: b"ur-vile\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            m_carry: 0 as libc::c_int,
            m_flags: 0x20 as libc::c_int as libc::c_ushort,
            m_stats: {
                let mut init = stats {
                    s_str: 10 as libc::c_int as str_t,
                    s_exp: 190 as libc::c_int as libc::c_long,
                    s_lvl: 7 as libc::c_int,
                    s_arm: -(2 as libc::c_int),
                    s_hpt: 1 as libc::c_int,
                    s_dmg: b"1d3/1d3/1d3/4d6\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    s_maxhp: 1 as libc::c_int,
                };
                init
            },
        };
        init
    },
    {
        let mut init = monster {
            m_name: b"vampire\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            m_carry: 20 as libc::c_int,
            m_flags: (0x200 as libc::c_int | 0x20 as libc::c_int) as libc::c_ushort,
            m_stats: {
                let mut init = stats {
                    s_str: 10 as libc::c_int as str_t,
                    s_exp: 350 as libc::c_int as libc::c_long,
                    s_lvl: 8 as libc::c_int,
                    s_arm: 1 as libc::c_int,
                    s_hpt: 1 as libc::c_int,
                    s_dmg: b"1d10\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    s_maxhp: 1 as libc::c_int,
                };
                init
            },
        };
        init
    },
    {
        let mut init = monster {
            m_name: b"wraith\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            m_carry: 0 as libc::c_int,
            m_flags: 0 as libc::c_int as libc::c_ushort,
            m_stats: {
                let mut init = stats {
                    s_str: 10 as libc::c_int as str_t,
                    s_exp: 55 as libc::c_int as libc::c_long,
                    s_lvl: 5 as libc::c_int,
                    s_arm: 4 as libc::c_int,
                    s_hpt: 1 as libc::c_int,
                    s_dmg: b"1d6\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    s_maxhp: 1 as libc::c_int,
                };
                init
            },
        };
        init
    },
    {
        let mut init = monster {
            m_name: b"xeroc\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            m_carry: 30 as libc::c_int,
            m_flags: 0 as libc::c_int as libc::c_ushort,
            m_stats: {
                let mut init = stats {
                    s_str: 10 as libc::c_int as str_t,
                    s_exp: 100 as libc::c_int as libc::c_long,
                    s_lvl: 7 as libc::c_int,
                    s_arm: 7 as libc::c_int,
                    s_hpt: 1 as libc::c_int,
                    s_dmg: b"3d4\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    s_maxhp: 1 as libc::c_int,
                };
                init
            },
        };
        init
    },
    {
        let mut init = monster {
            m_name: b"yeti\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            m_carry: 30 as libc::c_int,
            m_flags: 0 as libc::c_int as libc::c_ushort,
            m_stats: {
                let mut init = stats {
                    s_str: 10 as libc::c_int as str_t,
                    s_exp: 50 as libc::c_int as libc::c_long,
                    s_lvl: 4 as libc::c_int,
                    s_arm: 6 as libc::c_int,
                    s_hpt: 1 as libc::c_int,
                    s_dmg: b"1d6/1d6\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    s_maxhp: 1 as libc::c_int,
                };
                init
            },
        };
        init
    },
    {
        let mut init = monster {
            m_name: b"zombie\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            m_carry: 0 as libc::c_int,
            m_flags: 0x20 as libc::c_int as libc::c_ushort,
            m_stats: {
                let mut init = stats {
                    s_str: 10 as libc::c_int as str_t,
                    s_exp: 6 as libc::c_int as libc::c_long,
                    s_lvl: 2 as libc::c_int,
                    s_arm: 8 as libc::c_int,
                    s_hpt: 1 as libc::c_int,
                    s_dmg: b"1d8\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    s_maxhp: 1 as libc::c_int,
                };
                init
            },
        };
        init
    },
];
#[no_mangle]
pub static mut f_damage: [libc::c_char; 10] = [0; 10];
#[no_mangle]
pub static mut things: [magic_item; 7] = [
    {
        let mut init = magic_item {
            mi_name: 0 as *const libc::c_char as *mut libc::c_char,
            mi_prob: 27 as libc::c_int,
            mi_worth: 1 as libc::c_int as libc::c_short,
        };
        init
    },
    {
        let mut init = magic_item {
            mi_name: 0 as *const libc::c_char as *mut libc::c_char,
            mi_prob: 30 as libc::c_int,
            mi_worth: 1 as libc::c_int as libc::c_short,
        };
        init
    },
    {
        let mut init = magic_item {
            mi_name: 0 as *const libc::c_char as *mut libc::c_char,
            mi_prob: 17 as libc::c_int,
            mi_worth: 1 as libc::c_int as libc::c_short,
        };
        init
    },
    {
        let mut init = magic_item {
            mi_name: 0 as *const libc::c_char as *mut libc::c_char,
            mi_prob: 8 as libc::c_int,
            mi_worth: 1 as libc::c_int as libc::c_short,
        };
        init
    },
    {
        let mut init = magic_item {
            mi_name: 0 as *const libc::c_char as *mut libc::c_char,
            mi_prob: 8 as libc::c_int,
            mi_worth: 1 as libc::c_int as libc::c_short,
        };
        init
    },
    {
        let mut init = magic_item {
            mi_name: 0 as *const libc::c_char as *mut libc::c_char,
            mi_prob: 5 as libc::c_int,
            mi_worth: 1 as libc::c_int as libc::c_short,
        };
        init
    },
    {
        let mut init = magic_item {
            mi_name: 0 as *const libc::c_char as *mut libc::c_char,
            mi_prob: 5 as libc::c_int,
            mi_worth: 1 as libc::c_int as libc::c_short,
        };
        init
    },
];
#[no_mangle]
pub static mut nullstr: [libc::c_char; 1] =
    unsafe { *::core::mem::transmute::<&[u8; 1], &mut [libc::c_char; 1]>(b"\0") };
#[no_mangle]
pub static mut typebuf: *mut libc::c_char = unsafe { nullstr.as_ptr() as *mut _ };
#[no_mangle]
pub static mut intense: *mut libc::c_char =
    b" of intense white light\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut flashmsg: *mut libc::c_char =
    b"your %s gives off a flash%s\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut it: *mut libc::c_char =
    b"it\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut you: *mut libc::c_char =
    b"you\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut no_mem: *mut libc::c_char =
    b"Not enough Memory\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
