use ::libc;
extern "C" {
    fn memmove(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
        -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn free(_: *mut libc::c_void);
    static mut s_names: [array; 0];
    // Rust port: Fixed missing parameter
    fn newmem(p0: u64) -> *mut libc::c_char;
    static mut iguess: libc::c_int;
    static mut s_guess: [*mut libc::c_char; 0];
    static mut p_guess: [*mut libc::c_char; 0];
    static mut r_guess: [*mut libc::c_char; 0];
    static mut ws_guess: [*mut libc::c_char; 0];
    static mut terse: bool;
    static mut p_know: [bool; 0];
    static mut r_know: [bool; 0];
    static mut s_know: [bool; 0];
    static mut ws_know: [bool; 0];
    static mut p_colors: [*mut libc::c_char; 0];
    static mut r_stones: [*mut libc::c_char; 0];
    static mut ws_made: [*mut libc::c_char; 0];
    static mut ws_type: [*mut libc::c_char; 0];
    static mut a_class: [libc::c_int; 0];
    static mut food_left: libc::c_int;
    static mut cur_armor: *mut THING;
    static mut cur_weapon: *mut THING;
    static mut player: THING;
    static mut max_stats: stats;
    static mut p_magic: [magic_item; 0];
    static mut r_magic: [magic_item; 0];
    static mut s_magic: [magic_item; 0];
    static mut things: [magic_item; 0];
    static mut ws_magic: [magic_item; 0];
    static mut _guesses: [array; 0];
    fn add_pack(obj: *mut THING, silent: bool);
    fn new_item() -> *mut THING;
    fn rnd(range: libc::c_int) -> libc::c_int;
    fn init_weapon(weap: *mut THING, type_0: byte);
    fn spread(nm: libc::c_int) -> libc::c_int;
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
pub struct STONE {
    pub st_name: *mut libc::c_char,
    pub st_value: libc::c_int,
}
#[no_mangle]
pub static mut _things: *mut THING = 0 as *const THING as *mut THING;
#[no_mangle]
pub static mut _t_alloc: *mut libc::c_int = 0 as *const libc::c_int as *mut libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn init_player() {
    let mut obj: *mut THING = 0 as *mut THING;
    memmove(
        &mut player._t._t_stats as *mut stats as *mut libc::c_void,
        &mut max_stats as *mut stats as *const libc::c_void,
        ::core::mem::size_of::<stats>() as libc::c_ulong,
    );
    food_left = spread(1300 as libc::c_int);
    memset(
        _things as *mut libc::c_void,
        0 as libc::c_int,
        (83 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<THING>() as libc::c_ulong),
    );
    memset(
        _t_alloc as *mut libc::c_void,
        0 as libc::c_int,
        (83 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    );
    obj = new_item();
    (*obj)._o._o_type = 0x18 as libc::c_int;
    (*obj)._o._o_which = 0 as libc::c_int;
    init_weapon(obj, 0 as libc::c_int as byte);
    (*obj)._o._o_hplus = 1 as libc::c_int;
    (*obj)._o._o_dplus = 1 as libc::c_int;
    (*obj)._o._o_flags = ((*obj)._o._o_flags as libc::c_int | 0x2 as libc::c_int) as libc::c_short;
    (*obj)._o._o_count = 1 as libc::c_int;
    (*obj)._o._o_group = 0 as libc::c_int;
    add_pack(obj, 1 as libc::c_int != 0);
    cur_weapon = obj;
    obj = new_item();
    (*obj)._o._o_type = 0x18 as libc::c_int;
    (*obj)._o._o_which = 2 as libc::c_int;
    init_weapon(obj, 2 as libc::c_int as byte);
    (*obj)._o._o_hplus = 1 as libc::c_int;
    (*obj)._o._o_dplus = 0 as libc::c_int;
    (*obj)._o._o_count = 1 as libc::c_int;
    (*obj)._o._o_group = 0 as libc::c_int;
    (*obj)._o._o_flags = ((*obj)._o._o_flags as libc::c_int | 0x2 as libc::c_int) as libc::c_short;
    add_pack(obj, 1 as libc::c_int != 0);
    obj = new_item();
    (*obj)._o._o_type = 0x18 as libc::c_int;
    (*obj)._o._o_which = 3 as libc::c_int;
    init_weapon(obj, 3 as libc::c_int as byte);
    (*obj)._o._o_count = rnd(15 as libc::c_int) + 25 as libc::c_int;
    (*obj)._o._o_dplus = 0 as libc::c_int;
    (*obj)._o._o_hplus = (*obj)._o._o_dplus;
    (*obj)._o._o_flags = ((*obj)._o._o_flags as libc::c_int | 0x2 as libc::c_int) as libc::c_short;
    add_pack(obj, 1 as libc::c_int != 0);
    obj = new_item();
    (*obj)._o._o_type = 0x8 as libc::c_int;
    (*obj)._o._o_which = 1 as libc::c_int;
    (*obj)._o._o_ac = (*a_class.as_mut_ptr().offset(1 as libc::c_int as isize) - 1 as libc::c_int)
        as libc::c_short;
    (*obj)._o._o_flags = ((*obj)._o._o_flags as libc::c_int | 0x2 as libc::c_int) as libc::c_short;
    (*obj)._o._o_count = 1 as libc::c_int;
    (*obj)._o._o_group = 0 as libc::c_int;
    cur_armor = obj;
    add_pack(obj, 1 as libc::c_int != 0);
    obj = new_item();
    (*obj)._o._o_type = 0x5 as libc::c_int;
    (*obj)._o._o_count = 1 as libc::c_int;
    (*obj)._o._o_which = 0 as libc::c_int;
    (*obj)._o._o_group = 0 as libc::c_int;
    add_pack(obj, 1 as libc::c_int != 0);
}
static mut rainbow: [*mut libc::c_char; 27] = [
    b"amber\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"aquamarine\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"black\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"blue\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"brown\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"clear\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"crimson\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"cyan\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"ecru\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"gold\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"green\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"grey\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"magenta\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"orange\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"pink\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"plaid\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"purple\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"red\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"silver\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"tan\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"tangerine\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"topaz\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"turquoise\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"vermilion\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"violet\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"white\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"yellow\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
];
static mut c_set: *mut libc::c_char =
    b"bcdfghjklmnpqrstvwxyz\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
static mut v_set: *mut libc::c_char =
    b"aeiou\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
static mut stones: [STONE; 26] = [
    {
        let mut init = STONE {
            st_name: b"agate\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            st_value: 25 as libc::c_int,
        };
        init
    },
    {
        let mut init = STONE {
            st_name: b"alexandrite\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            st_value: 40 as libc::c_int,
        };
        init
    },
    {
        let mut init = STONE {
            st_name: b"amethyst\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            st_value: 50 as libc::c_int,
        };
        init
    },
    {
        let mut init = STONE {
            st_name: b"carnelian\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            st_value: 40 as libc::c_int,
        };
        init
    },
    {
        let mut init = STONE {
            st_name: b"diamond\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            st_value: 300 as libc::c_int,
        };
        init
    },
    {
        let mut init = STONE {
            st_name: b"emerald\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            st_value: 300 as libc::c_int,
        };
        init
    },
    {
        let mut init = STONE {
            st_name: b"germanium\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            st_value: 225 as libc::c_int,
        };
        init
    },
    {
        let mut init = STONE {
            st_name: b"granite\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            st_value: 5 as libc::c_int,
        };
        init
    },
    {
        let mut init = STONE {
            st_name: b"garnet\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            st_value: 50 as libc::c_int,
        };
        init
    },
    {
        let mut init = STONE {
            st_name: b"jade\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            st_value: 150 as libc::c_int,
        };
        init
    },
    {
        let mut init = STONE {
            st_name: b"kryptonite\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            st_value: 300 as libc::c_int,
        };
        init
    },
    {
        let mut init = STONE {
            st_name: b"lapis lazuli\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            st_value: 50 as libc::c_int,
        };
        init
    },
    {
        let mut init = STONE {
            st_name: b"moonstone\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            st_value: 50 as libc::c_int,
        };
        init
    },
    {
        let mut init = STONE {
            st_name: b"obsidian\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            st_value: 15 as libc::c_int,
        };
        init
    },
    {
        let mut init = STONE {
            st_name: b"onyx\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            st_value: 60 as libc::c_int,
        };
        init
    },
    {
        let mut init = STONE {
            st_name: b"opal\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            st_value: 200 as libc::c_int,
        };
        init
    },
    {
        let mut init = STONE {
            st_name: b"pearl\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            st_value: 220 as libc::c_int,
        };
        init
    },
    {
        let mut init = STONE {
            st_name: b"peridot\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            st_value: 63 as libc::c_int,
        };
        init
    },
    {
        let mut init = STONE {
            st_name: b"ruby\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            st_value: 350 as libc::c_int,
        };
        init
    },
    {
        let mut init = STONE {
            st_name: b"sapphire\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            st_value: 285 as libc::c_int,
        };
        init
    },
    {
        let mut init = STONE {
            st_name: b"stibotantalite\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            st_value: 200 as libc::c_int,
        };
        init
    },
    {
        let mut init = STONE {
            st_name: b"tiger eye\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            st_value: 50 as libc::c_int,
        };
        init
    },
    {
        let mut init = STONE {
            st_name: b"topaz\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            st_value: 60 as libc::c_int,
        };
        init
    },
    {
        let mut init = STONE {
            st_name: b"turquoise\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            st_value: 70 as libc::c_int,
        };
        init
    },
    {
        let mut init = STONE {
            st_name: b"taaffeite\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            st_value: 300 as libc::c_int,
        };
        init
    },
    {
        let mut init = STONE {
            st_name: b"zircon\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            st_value: 80 as libc::c_int,
        };
        init
    },
];
static mut wood: [*mut libc::c_char; 33] = [
    b"avocado wood\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"balsa\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"bamboo\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"banyan\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"birch\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"cedar\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"cherry\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"cinnibar\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"cypress\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"dogwood\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"driftwood\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"ebony\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"elm\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"eucalyptus\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"fall\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"hemlock\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"holly\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"ironwood\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"kukui wood\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"mahogany\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"manzanita\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"maple\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"oaken\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"persimmon wood\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"pecan\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"pine\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"poplar\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"redwood\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"rosewood\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"spruce\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"teak\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"walnut\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"zebrawood\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
];
static mut metal: [*mut libc::c_char; 22] = [
    b"aluminum\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"beryllium\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"bone\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"brass\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"bronze\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"copper\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"electrum\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"gold\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"iron\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"lead\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"magnesium\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"mercury\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"nickel\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"pewter\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"platinum\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"steel\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"silver\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"silicon\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"tin\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"titanium\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"tungsten\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"zinc\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
];
#[no_mangle]
pub unsafe extern "C" fn init_things() {
    let mut mp: *mut magic_item = 0 as *mut magic_item;
    mp = &mut *things.as_mut_ptr().offset(1 as libc::c_int as isize) as *mut magic_item;
    while mp
        <= &mut *things
            .as_mut_ptr()
            .offset((7 as libc::c_int - 1 as libc::c_int) as isize) as *mut magic_item
    {
        (*mp).mi_prob += (*mp.offset(-(1 as libc::c_int as isize))).mi_prob;
        mp = mp.offset(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn init_colors() {
    let mut i: libc::c_uint = 0;
    let mut j: libc::c_uint = 0;
    let mut used: [bool; 27] = [false; 27];
    i = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong)
        < (::core::mem::size_of::<[*mut libc::c_char; 27]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
    {
        used[i as usize] = 0 as libc::c_int != 0;
        i = i.wrapping_add(1);
    }
    i = 0 as libc::c_int as libc::c_uint;
    while i < 14 as libc::c_int as libc::c_uint {
        loop {
            j = rnd(
                (::core::mem::size_of::<[*mut libc::c_char; 27]>() as libc::c_ulong)
                    .wrapping_div(::core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
                    as libc::c_int,
            ) as libc::c_uint;
            if !used[j as usize] {
                break;
            }
        }
        used[j as usize] = 1 as libc::c_int != 0;
        let ref mut fresh0 = *p_colors.as_mut_ptr().offset(i as isize);
        *fresh0 = rainbow[j as usize];
        *p_know.as_mut_ptr().offset(i as isize) = 0 as libc::c_int != 0;
        let fresh1 = iguess;
        iguess = iguess + 1;
        let ref mut fresh2 = *p_guess.as_mut_ptr().offset(i as isize);
        *fresh2 =
            &mut *_guesses.as_mut_ptr().offset(fresh1 as isize) as *mut array as *mut libc::c_char;
        if i > 0 as libc::c_int as libc::c_uint {
            (*p_magic.as_mut_ptr().offset(i as isize)).mi_prob += (*p_magic
                .as_mut_ptr()
                .offset(i.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize))
            .mi_prob;
        }
        i = i.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn init_names() {
    let mut nsyl: libc::c_int = 0;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut sp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut nwords: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 15 as libc::c_int {
        cp = prbuf;
        nwords = rnd(if terse as libc::c_int != 0 {
            3 as libc::c_int
        } else {
            4 as libc::c_int
        }) + 2 as libc::c_int;
        loop {
            let fresh3 = nwords;
            nwords = nwords - 1;
            if !(fresh3 != 0) {
                break;
            }
            nsyl = rnd(2 as libc::c_int) + 1 as libc::c_int;
            loop {
                let fresh4 = nsyl;
                nsyl = nsyl - 1;
                if !(fresh4 != 0) {
                    break;
                }
                sp = getsyl();
                if &mut *cp.offset((strlen
                    as unsafe extern "C" fn(*const libc::c_char) -> libc::c_ulong)(
                    sp
                ) as isize) as *mut libc::c_char
                    > &mut *prbuf.offset((20 as libc::c_int - 1 as libc::c_int) as isize)
                        as *mut libc::c_char
                {
                    nwords = 0 as libc::c_int;
                    break;
                } else {
                    while *sp != 0 {
                        let fresh5 = sp;
                        sp = sp.offset(1);
                        let fresh6 = cp;
                        cp = cp.offset(1);
                        *fresh6 = *fresh5;
                    }
                }
            }
            let fresh7 = cp;
            cp = cp.offset(1);
            *fresh7 = ' ' as i32 as libc::c_char;
        }
        cp = cp.offset(-1);
        *cp = '\0' as i32 as libc::c_char;
        *prbuf.offset(20 as libc::c_int as isize) = 0 as libc::c_int as libc::c_char;
        *s_know.as_mut_ptr().offset(i as isize) = 0 as libc::c_int != 0;
        let fresh8 = iguess;
        iguess = iguess + 1;
        let ref mut fresh9 = *s_guess.as_mut_ptr().offset(i as isize);
        *fresh9 =
            &mut *_guesses.as_mut_ptr().offset(fresh8 as isize) as *mut array as *mut libc::c_char;
        strcpy(
            &mut *s_names.as_mut_ptr().offset(i as isize) as *mut array as *mut libc::c_char,
            prbuf,
        );
        if i > 0 as libc::c_int {
            (*s_magic.as_mut_ptr().offset(i as isize)).mi_prob +=
                (*s_magic.as_mut_ptr().offset((i - 1 as libc::c_int) as isize)).mi_prob;
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn getsyl() -> *mut libc::c_char {
    static mut _tsyl: [libc::c_char; 4] = [0; 4];
    _tsyl[3 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    _tsyl[2 as libc::c_int as usize] = rchr(c_set);
    _tsyl[1 as libc::c_int as usize] = rchr(v_set);
    _tsyl[0 as libc::c_int as usize] = rchr(c_set);
    return _tsyl.as_mut_ptr();
}
#[no_mangle]
pub unsafe extern "C" fn rchr(mut string: *mut libc::c_char) -> libc::c_char {
    return *string.offset(rnd(strlen(string) as libc::c_int) as isize);
}
#[no_mangle]
pub unsafe extern "C" fn init_stones() {
    let mut i: libc::c_uint = 0;
    let mut j: libc::c_uint = 0;
    let mut used: [bool; 26] = [false; 26];
    i = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong)
        < (::core::mem::size_of::<[STONE; 26]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<STONE>() as libc::c_ulong)
    {
        used[i as usize] = 0 as libc::c_int != 0;
        i = i.wrapping_add(1);
    }
    i = 0 as libc::c_int as libc::c_uint;
    while i < 14 as libc::c_int as libc::c_uint {
        loop {
            j = rnd((::core::mem::size_of::<[STONE; 26]>() as libc::c_ulong)
                .wrapping_div(::core::mem::size_of::<STONE>() as libc::c_ulong)
                as libc::c_int) as libc::c_uint;
            if !used[j as usize] {
                break;
            }
        }
        used[j as usize] = 1 as libc::c_int != 0;
        let ref mut fresh10 = *r_stones.as_mut_ptr().offset(i as isize);
        *fresh10 = stones[j as usize].st_name;
        *r_know.as_mut_ptr().offset(i as isize) = 0 as libc::c_int != 0;
        let fresh11 = iguess;
        iguess = iguess + 1;
        let ref mut fresh12 = *r_guess.as_mut_ptr().offset(i as isize);
        *fresh12 =
            &mut *_guesses.as_mut_ptr().offset(fresh11 as isize) as *mut array as *mut libc::c_char;
        if i > 0 as libc::c_int as libc::c_uint {
            (*r_magic.as_mut_ptr().offset(i as isize)).mi_prob += (*r_magic
                .as_mut_ptr()
                .offset(i.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize))
            .mi_prob;
        }
        let ref mut fresh13 = (*r_magic.as_mut_ptr().offset(i as isize)).mi_worth;
        *fresh13 = (*fresh13 as libc::c_int + stones[j as usize].st_value) as libc::c_short;
        i = i.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn init_materials() {
    let mut i: libc::c_uint = 0;
    let mut j: libc::c_uint = 0;
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut metused: [bool; 22] = [false; 22];
    let mut woodused: [bool; 33] = [false; 33];
    i = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong)
        < (::core::mem::size_of::<[*mut libc::c_char; 33]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
    {
        woodused[i as usize] = 0 as libc::c_int != 0;
        i = i.wrapping_add(1);
    }
    i = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong)
        < (::core::mem::size_of::<[*mut libc::c_char; 22]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
    {
        metused[i as usize] = 0 as libc::c_int != 0;
        i = i.wrapping_add(1);
    }
    i = 0 as libc::c_int as libc::c_uint;
    while i < 14 as libc::c_int as libc::c_uint {
        loop {
            if rnd(2 as libc::c_int) == 0 as libc::c_int {
                j = rnd(
                    (::core::mem::size_of::<[*mut libc::c_char; 22]>() as libc::c_ulong)
                        .wrapping_div(::core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
                        as libc::c_int,
                ) as libc::c_uint;
                if metused[j as usize] {
                    continue;
                }
                let ref mut fresh14 = *ws_type.as_mut_ptr().offset(i as isize);
                *fresh14 = b"wand\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
                str = metal[j as usize];
                metused[j as usize] = 1 as libc::c_int != 0;
                break;
            } else {
                j = rnd(
                    (::core::mem::size_of::<[*mut libc::c_char; 33]>() as libc::c_ulong)
                        .wrapping_div(::core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
                        as libc::c_int,
                ) as libc::c_uint;
                if woodused[j as usize] {
                    continue;
                }
                let ref mut fresh15 = *ws_type.as_mut_ptr().offset(i as isize);
                *fresh15 = b"staff\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
                str = wood[j as usize];
                woodused[j as usize] = 1 as libc::c_int != 0;
                break;
            }
        }
        let ref mut fresh16 = *ws_made.as_mut_ptr().offset(i as isize);
        *fresh16 = str;
        *ws_know.as_mut_ptr().offset(i as isize) = 0 as libc::c_int != 0;
        let fresh17 = iguess;
        iguess = iguess + 1;
        let ref mut fresh18 = *ws_guess.as_mut_ptr().offset(i as isize);
        *fresh18 =
            &mut *_guesses.as_mut_ptr().offset(fresh17 as isize) as *mut array as *mut libc::c_char;
        if i > 0 as libc::c_int as libc::c_uint {
            (*ws_magic.as_mut_ptr().offset(i as isize)).mi_prob += (*ws_magic
                .as_mut_ptr()
                .offset(i.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize))
            .mi_prob;
        }
        i = i.wrapping_add(1);
    }
}
#[no_mangle]
pub static mut e_levels: *mut libc::c_long = 0 as *const libc::c_long as *mut libc::c_long;
#[no_mangle]
pub static mut tbuf: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut msgbuf: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut prbuf: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut ring_buf: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut _level: *mut byte = 0 as *const byte as *mut byte;
#[no_mangle]
pub static mut _flags: *mut byte = 0 as *const byte as *mut byte;
#[no_mangle]
pub unsafe extern "C" fn init_ds() {
    let mut ep: *mut libc::c_long = 0 as *mut libc::c_long;
    _flags = newmem((25 - 3) * 80) as *mut byte;
    _level = newmem((25 - 3) * 80) as *mut byte;
    _things = newmem(
        (::core::mem::size_of::<THING>() as libc::c_ulong)
            .wrapping_mul(83 as libc::c_int as libc::c_ulong),
    ) as *mut THING;
    _t_alloc = newmem(
        (83 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    tbuf = newmem(80);
    msgbuf = newmem(128);
    prbuf = newmem(80);
    ring_buf = newmem(6);
    e_levels = newmem(
        (20 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_long>() as libc::c_ulong),
    ) as *mut libc::c_long;
    ep = e_levels.offset(1 as libc::c_int as isize);
    *e_levels = 10 as libc::c_long;
    while ep < e_levels.offset(19 as libc::c_int as isize) {
        *ep = *ep.offset(-(1 as libc::c_int as isize)) << 1 as libc::c_int;
        ep = ep.offset(1);
    }
    *ep = 0 as libc::c_long;
}
#[no_mangle]
pub unsafe extern "C" fn free_ds() {
    free(_flags as *mut libc::c_void);
    free(_level as *mut libc::c_void);
    free(_things as *mut libc::c_void);
    free(_t_alloc as *mut libc::c_void);
    free(tbuf as *mut libc::c_void);
    free(msgbuf as *mut libc::c_void);
    free(prbuf as *mut libc::c_void);
    free(ring_buf as *mut libc::c_void);
    free(e_levels as *mut libc::c_void);
}
