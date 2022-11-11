use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn readchar() -> byte;
    fn md_localtime() -> *mut TM;
    fn md_exit(status: libc::c_int);
    static mut noscore: bool;
    static mut terse: bool;
    static mut p_know: [bool; 0];
    static mut r_know: [bool; 0];
    static mut s_know: [bool; 0];
    static mut ws_know: [bool; 0];
    static mut he_man: [*mut libc::c_char; 0];
    static mut a_class: [libc::c_int; 0];
    static mut max_level: libc::c_int;
    static mut purse: libc::c_int;
    static mut your_na: *mut libc::c_char;
    static mut kild_by: *mut libc::c_char;
    static mut player: THING;
    static mut monsters: [monster; 0];
    static mut p_magic: [magic_item; 0];
    static mut r_magic: [magic_item; 0];
    static mut s_magic: [magic_item; 0];
    static mut ws_magic: [magic_item; 0];
    static mut s_score: [libc::c_char; 0];
    static mut whoami: [libc::c_char; 0];
    static mut prbuf: *mut libc::c_char;
    fn wait_msg(msg: *const libc::c_char);
    fn wait_for(ch: byte);
    fn str_attr(str: *mut libc::c_char);
    fn vowelstr(str: *mut libc::c_char) -> *mut libc::c_char;
    fn is_alpha(ch: libc::c_char) -> bool;
    fn inv_name(obj: *mut THING, drop_0: bool) -> *mut libc::c_char;
    fn cur_clear();
    fn cur_mvaddstr(r: libc::c_int, c: libc::c_int, s: *mut libc::c_char);
    fn cur_addstr(s: *mut libc::c_char);
    fn set_attr(bute: libc::c_int);
    fn cur_box(ul_r: libc::c_int, ul_c: libc::c_int, lr_r: libc::c_int, lr_c: libc::c_int);
    fn center(row: libc::c_int, string: *mut libc::c_char);
    fn cur_printw(msg: *const libc::c_char, _: ...);
    fn drop_curtain();
    fn raise_curtain();
    static mut LINES: libc::c_int;
    static mut is_saved: libc::c_int;
    static mut scr_type: libc::c_int;
    fn cur_move(row: libc::c_int, col: libc::c_int) -> libc::c_int;
    static mut COLS: libc::c_int;
}
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct md_tm {
    pub second: libc::c_int,
    pub minute: libc::c_int,
    pub hour: libc::c_int,
    pub day: libc::c_int,
    pub month: libc::c_int,
    pub year: libc::c_int,
}
pub type TM = md_tm;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sc_ent {
    pub sc_name: [libc::c_char; 38],
    pub sc_rank: libc::c_int,
    pub sc_gold: libc::c_int,
    pub sc_fate: libc::c_int,
    pub sc_level: libc::c_int,
}
static mut file: *mut FILE = 0 as *const FILE as *mut FILE;
#[no_mangle]
pub unsafe extern "C" fn score(
    mut amount: libc::c_int,
    mut flags: libc::c_int,
    mut monst: libc::c_char,
) {
    let mut his_score: sc_ent = sc_ent {
        sc_name: [0; 38],
        sc_rank: 0,
        sc_gold: 0,
        sc_fate: 0,
        sc_level: 0,
    };
    let mut top_ten: [sc_ent; 10] = [sc_ent {
        sc_name: [0; 38],
        sc_rank: 0,
        sc_gold: 0,
        sc_fate: 0,
        sc_level: 0,
    }; 10];
    let mut rank: libc::c_int = 0 as libc::c_int;
    let mut response: libc::c_char = ' ' as i32 as libc::c_char;
    is_saved = 1 as libc::c_int;
    if amount != 0 || flags != 0 || monst as libc::c_int != 0 {
        wait_msg(b"see rankings\0" as *const u8 as *const libc::c_char);
    }
    loop {
        file = fopen(
            s_score.as_mut_ptr(),
            b"r\0" as *const u8 as *const libc::c_char,
        );
        if !file.is_null() {
            break;
        }
        cur_printw(b"\n\0" as *const u8 as *const libc::c_char);
        if noscore as libc::c_int != 0 || amount == 0 as libc::c_int {
            return;
        }
        str_attr(
            b"No scorefile: %Create %Retry %Abort\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        loop {
            response = readchar() as libc::c_char;
            match response as libc::c_int {
                99 | 67 => {
                    fclose(fopen(
                        s_score.as_mut_ptr(),
                        b"w\0" as *const u8 as *const libc::c_char,
                    ));
                    break;
                }
                114 | 82 => {
                    break;
                }
                97 | 65 => return,
                _ => {}
            }
        }
    }
    cur_printw(b"\n\0" as *const u8 as *const libc::c_char);
    get_scores(top_ten.as_mut_ptr());
    if noscore as libc::c_int != 1 as libc::c_int {
        strcpy((his_score.sc_name).as_mut_ptr(), whoami.as_mut_ptr());
        his_score.sc_gold = amount;
        his_score.sc_fate = if flags != 0 {
            flags
        } else {
            monst as libc::c_int
        };
        his_score.sc_level = max_level;
        his_score.sc_rank = player._t._t_stats.s_lvl;
        rank = add_scores(&mut his_score, top_ten.as_mut_ptr());
    }
    fclose(file);
    if rank > 0 as libc::c_int {
        file = fopen(
            s_score.as_mut_ptr(),
            b"w\0" as *const u8 as *const libc::c_char,
        );
        if !file.is_null() {
            put_scores(top_ten.as_mut_ptr());
            fclose(file);
        }
    }
    pr_scores(rank, top_ten.as_mut_ptr());
    wait_msg(b"exit\0" as *const u8 as *const libc::c_char);
    cur_printw(b"\n\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn get_scores(mut top10: *mut sc_ent) {
    let mut i: libc::c_int = 0;
    let mut retcode: libc::c_int = 1 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 10 as libc::c_int {
        if retcode > 0 as libc::c_int {
            retcode = fread(
                top10 as *mut libc::c_void,
                ::core::mem::size_of::<sc_ent>() as libc::c_ulong,
                1 as libc::c_int as libc::c_ulong,
                file,
            ) as libc::c_int;
        }
        if retcode <= 0 as libc::c_int {
            (*top10).sc_gold = 0 as libc::c_int;
        }
        i += 1;
        top10 = top10.offset(1);
    }
}
unsafe extern "C" fn put_scores(mut top10: *mut sc_ent) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 10 as libc::c_int && (*top10).sc_gold != 0 {
        if fwrite(
            top10 as *const libc::c_void,
            ::core::mem::size_of::<sc_ent>() as libc::c_ulong,
            1 as libc::c_int as libc::c_ulong,
            file,
        ) <= 0 as libc::c_int as libc::c_ulong
        {
            return;
        }
        i += 1;
        top10 = top10.offset(1);
    }
}
unsafe extern "C" fn pr_scores(mut newrank: libc::c_int, mut top10: *mut sc_ent) {
    let mut i: libc::c_int = 0;
    let mut curl: libc::c_int = 0;
    let mut dthstr: [libc::c_char; 30] = [0; 30];
    let mut altmsg: *mut libc::c_char = 0 as *mut libc::c_char;
    cur_clear();
    set_attr(15 as libc::c_int);
    if scr_type == 7 as libc::c_int {
        set_attr(14 as libc::c_int);
    }
    cur_mvaddstr(
        0 as libc::c_int,
        0 as libc::c_int,
        b"Guildmaster's Hall Of Fame:\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    set_attr(0 as libc::c_int);
    set_attr(11 as libc::c_int);
    cur_mvaddstr(
        2 as libc::c_int,
        0 as libc::c_int,
        b"Gold\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    i = 0 as libc::c_int;
    while i < 10 as libc::c_int {
        altmsg = 0 as *mut libc::c_char;
        set_attr(5 as libc::c_int);
        if newrank - 1 as libc::c_int == i {
            if scr_type == 7 as libc::c_int {
                set_attr(14 as libc::c_int);
            } else {
                set_attr(11 as libc::c_int);
            }
        }
        if (*top10).sc_gold <= 0 as libc::c_int {
            break;
        }
        curl = 4 as libc::c_int
            + (if COLS == 40 as libc::c_int {
                i * 2 as libc::c_int
            } else {
                i
            });
        cur_move(curl, 0 as libc::c_int);
        cur_printw(
            b"%d \0" as *const u8 as *const libc::c_char,
            (*top10).sc_gold,
        );
        cur_move(curl, 6 as libc::c_int);
        if newrank - 1 as libc::c_int != i {
            set_attr(3 as libc::c_int);
        }
        cur_printw(
            b"%s\0" as *const u8 as *const libc::c_char,
            ((*top10).sc_name).as_mut_ptr(),
        );
        if newrank - 1 as libc::c_int != i {
            set_attr(5 as libc::c_int);
        }
        if (*top10).sc_level >= 26 as libc::c_int {
            altmsg =
                b" Honored by the Guild\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
        if is_alpha((*top10).sc_fate as libc::c_char) {
            sprintf(
                dthstr.as_mut_ptr(),
                b" killed by %s\0" as *const u8 as *const libc::c_char,
                killname(
                    (0xff as libc::c_int & (*top10).sc_fate) as byte,
                    1 as libc::c_int != 0,
                ),
            );
            if COLS == 40 as libc::c_int
                && strlen(dthstr.as_mut_ptr()) > 23 as libc::c_int as libc::c_ulong
            {
                strcpy(
                    dthstr.as_mut_ptr(),
                    b" killed\0" as *const u8 as *const libc::c_char,
                );
            }
        } else {
            match (*top10).sc_fate {
                2 => {
                    altmsg = b" A total winner!\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char;
                }
                1 => {
                    strcpy(
                        dthstr.as_mut_ptr(),
                        b" quit\0" as *const u8 as *const libc::c_char,
                    );
                }
                _ => {
                    strcpy(
                        dthstr.as_mut_ptr(),
                        b" wierded out\0" as *const u8 as *const libc::c_char,
                    );
                }
            }
        }
        if ((strlen(((*top10).sc_name).as_mut_ptr()))
            .wrapping_add(10 as libc::c_int as libc::c_ulong)
            .wrapping_add(strlen(
                *he_man
                    .as_mut_ptr()
                    .offset(((*top10).sc_rank - 1 as libc::c_int) as isize),
            )) as libc::c_int)
            < COLS
        {
            if (*top10).sc_rank > 1 as libc::c_int && strlen(((*top10).sc_name).as_mut_ptr()) != 0 {
                cur_printw(
                    b" \"%s\"\0" as *const u8 as *const libc::c_char,
                    *he_man
                        .as_mut_ptr()
                        .offset(((*top10).sc_rank - 1 as libc::c_int) as isize),
                );
            }
        }
        if COLS == 40 as libc::c_int {
            cur_move(curl + 1 as libc::c_int, 6 as libc::c_int);
        }
        if altmsg.is_null() {
            cur_printw(
                b"%s on level %d\0" as *const u8 as *const libc::c_char,
                dthstr.as_mut_ptr(),
                (*top10).sc_level,
            );
        } else {
            cur_addstr(altmsg);
        }
        i += 1;
        top10 = top10.offset(1);
    }
    set_attr(0 as libc::c_int);
    if COLS == 80 as libc::c_int {
        cur_addstr(b"\n\n\n\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    }
}
unsafe extern "C" fn add_scores(
    mut newscore: *mut sc_ent,
    mut oldlist: *mut sc_ent,
) -> libc::c_int {
    let mut sentry: *mut sc_ent = 0 as *mut sc_ent;
    let mut insert: *mut sc_ent = 0 as *mut sc_ent;
    let mut retcode: libc::c_int = 10 as libc::c_int + 1 as libc::c_int;
    sentry = &mut *oldlist.offset((10 as libc::c_int - 1 as libc::c_int) as isize) as *mut sc_ent;
    while sentry >= oldlist {
        if !((*newscore).sc_gold as libc::c_uint > (*sentry).sc_gold as libc::c_uint) {
            break;
        }
        insert = sentry;
        retcode -= 1;
        if insert
            < &mut *oldlist.offset((10 as libc::c_int - 1 as libc::c_int) as isize) as *mut sc_ent
            && (*sentry).sc_gold != 0
        {
            *sentry.offset(1 as libc::c_int as isize) = *sentry;
        }
        sentry = sentry.offset(-1);
    }
    if retcode == 11 as libc::c_int {
        return 0 as libc::c_int;
    }
    *insert = *newscore;
    return retcode;
}
#[no_mangle]
pub unsafe extern "C" fn death(mut monst: libc::c_char) {
    let mut buf: [libc::c_char; 80] = [0; 80];
    let mut year: libc::c_int = 0;
    purse -= purse / 10 as libc::c_int;
    drop_curtain();
    if scr_type != 7 as libc::c_int {
        set_attr(5 as libc::c_int);
    }
    cur_box(
        if COLS == 40 as libc::c_int {
            1 as libc::c_int
        } else {
            7 as libc::c_int
        },
        (COLS - 28 as libc::c_int) / 2 as libc::c_int,
        22 as libc::c_int,
        (COLS + 28 as libc::c_int) / 2 as libc::c_int,
    );
    set_attr(0 as libc::c_int);
    center(
        10 as libc::c_int,
        b"REST\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    center(
        11 as libc::c_int,
        b"IN\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    center(
        12 as libc::c_int,
        b"PEACE\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    set_attr(3 as libc::c_int);
    center(
        21 as libc::c_int,
        b"  *    *      * \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    set_attr(1 as libc::c_int);
    center(
        22 as libc::c_int,
        b"___\\/(\\/)/(\\/ \\\\(//)\\)\\/(//)\\\\)//(\\__\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    set_attr(0 as libc::c_int);
    if scr_type == 7 as libc::c_int {
        set_attr(12 as libc::c_int);
    }
    center(14 as libc::c_int, your_na);
    set_attr(0 as libc::c_int);
    killname(monst as byte, 1 as libc::c_int != 0);
    strcpy(
        buf.as_mut_ptr(),
        b"killed by\0" as *const u8 as *const libc::c_char,
    );
    center(15 as libc::c_int, buf.as_mut_ptr());
    center(16 as libc::c_int, kild_by);
    sprintf(
        buf.as_mut_ptr(),
        b"%u Au\0" as *const u8 as *const libc::c_char,
        purse,
    );
    center(18 as libc::c_int, buf.as_mut_ptr());
    year = (*md_localtime()).year;
    sprintf(
        buf.as_mut_ptr(),
        b"%u\0" as *const u8 as *const libc::c_char,
        year,
    );
    center(19 as libc::c_int, buf.as_mut_ptr());
    raise_curtain();
    cur_move(LINES - 1 as libc::c_int, 0 as libc::c_int);
    score(purse, 0 as libc::c_int, monst);
    md_exit(0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn total_winner() {
    let mut obj: *mut THING = 0 as *mut THING;
    let mut worth: libc::c_int = 0 as libc::c_int;
    let mut c: byte = 0;
    let mut oldpurse: libc::c_int = 0;
    cur_clear();
    if !terse {
        set_attr(14 as libc::c_int);
        cur_printw(
            b"                                                               \n\0" as *const u8
                as *const libc::c_char,
        );
        cur_printw(
            b"  @   @               @   @           @          @@@  @     @  \n\0" as *const u8
                as *const libc::c_char,
        );
        cur_printw(
            b"  @   @               @@ @@           @           @   @     @  \n\0" as *const u8
                as *const libc::c_char,
        );
        cur_printw(
            b"  @   @  @@@  @   @   @ @ @  @@@   @@@@  @@@      @  @@@    @  \n\0" as *const u8
                as *const libc::c_char,
        );
        cur_printw(
            b"   @@@@ @   @ @   @   @   @     @ @   @ @   @     @   @     @  \n\0" as *const u8
                as *const libc::c_char,
        );
        cur_printw(
            b"      @ @   @ @   @   @   @  @@@@ @   @ @@@@@     @   @     @  \n\0" as *const u8
                as *const libc::c_char,
        );
        cur_printw(
            b"  @   @ @   @ @  @@   @   @ @   @ @   @ @         @   @  @     \n\0" as *const u8
                as *const libc::c_char,
        );
        cur_printw(
            b"   @@@   @@@   @@ @   @   @  @@@@  @@@@  @@@     @@@   @@   @  \n\0" as *const u8
                as *const libc::c_char,
        );
    }
    cur_printw(
        b"                                                               \n\0" as *const u8
            as *const libc::c_char,
    );
    cur_printw(
        b"     Congratulations, you have made it to the light of day!    \n\0" as *const u8
            as *const libc::c_char,
    );
    set_attr(0 as libc::c_int);
    cur_printw(
        b"\nYou have joined the elite ranks of those who have escaped the\n\0" as *const u8
            as *const libc::c_char,
    );
    cur_printw(
        b"Dungeons of Doom alive.  You journey home and sell all your loot at\n\0" as *const u8
            as *const libc::c_char,
    );
    cur_printw(
        b"a great profit and are admitted to the fighters guild.\n\0" as *const u8
            as *const libc::c_char,
    );
    cur_mvaddstr(
        LINES - 1 as libc::c_int,
        0 as libc::c_int,
        b"--Press space to continue--\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    wait_for(' ' as i32 as byte);
    cur_clear();
    cur_mvaddstr(
        0 as libc::c_int,
        0 as libc::c_int,
        b"   Worth  Item\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    oldpurse = purse;
    c = 'a' as i32 as byte;
    obj = player._t._t_pack;
    while !obj.is_null() {
        match (*obj)._o._o_type {
            5 => {
                worth = 2 as libc::c_int * (*obj)._o._o_count;
            }
            24 => {
                match (*obj)._o._o_which {
                    0 => {
                        worth = 8 as libc::c_int;
                    }
                    1 => {
                        worth = 15 as libc::c_int;
                    }
                    7 => {
                        worth = 30 as libc::c_int;
                    }
                    3 => {
                        worth = 1 as libc::c_int;
                    }
                    4 => {
                        worth = 2 as libc::c_int;
                    }
                    5 => {
                        worth = 75 as libc::c_int;
                    }
                    6 => {
                        worth = 1 as libc::c_int;
                    }
                    2 => {
                        worth = 15 as libc::c_int;
                    }
                    8 => {
                        worth = 1 as libc::c_int;
                    }
                    9 => {
                        worth = 5 as libc::c_int;
                    }
                    _ => {}
                }
                worth *= 3 as libc::c_int * ((*obj)._o._o_hplus + (*obj)._o._o_dplus)
                    + (*obj)._o._o_count;
                (*obj)._o._o_flags =
                    ((*obj)._o._o_flags as libc::c_int | 0x2 as libc::c_int) as libc::c_short;
            }
            8 => {
                match (*obj)._o._o_which {
                    0 => {
                        worth = 20 as libc::c_int;
                    }
                    1 => {
                        worth = 25 as libc::c_int;
                    }
                    2 => {
                        worth = 20 as libc::c_int;
                    }
                    3 => {
                        worth = 30 as libc::c_int;
                    }
                    4 => {
                        worth = 75 as libc::c_int;
                    }
                    5 => {
                        worth = 80 as libc::c_int;
                    }
                    6 => {
                        worth = 90 as libc::c_int;
                    }
                    7 => {
                        worth = 150 as libc::c_int;
                    }
                    _ => {}
                }
                worth += (9 as libc::c_int - (*obj)._o._o_ac as libc::c_int) * 100 as libc::c_int;
                worth += 10 as libc::c_int
                    * (*a_class.as_mut_ptr().offset((*obj)._o._o_which as isize)
                        - (*obj)._o._o_ac as libc::c_int);
                (*obj)._o._o_flags =
                    ((*obj)._o._o_flags as libc::c_int | 0x2 as libc::c_int) as libc::c_short;
            }
            13 => {
                worth = (*s_magic.as_mut_ptr().offset((*obj)._o._o_which as isize)).mi_worth
                    as libc::c_int;
                worth *= (*obj)._o._o_count;
                if !*s_know.as_mut_ptr().offset((*obj)._o._o_which as isize) {
                    worth /= 2 as libc::c_int;
                }
                *s_know.as_mut_ptr().offset((*obj)._o._o_which as isize) = 1 as libc::c_int != 0;
            }
            173 => {
                worth = (*p_magic.as_mut_ptr().offset((*obj)._o._o_which as isize)).mi_worth
                    as libc::c_int;
                worth *= (*obj)._o._o_count;
                if !*p_know.as_mut_ptr().offset((*obj)._o._o_which as isize) {
                    worth /= 2 as libc::c_int;
                }
                *p_know.as_mut_ptr().offset((*obj)._o._o_which as isize) = 1 as libc::c_int != 0;
            }
            9 => {
                worth = (*r_magic.as_mut_ptr().offset((*obj)._o._o_which as isize)).mi_worth
                    as libc::c_int;
                if (*obj)._o._o_which == 1 as libc::c_int
                    || (*obj)._o._o_which == 8 as libc::c_int
                    || (*obj)._o._o_which == 0 as libc::c_int
                    || (*obj)._o._o_which == 7 as libc::c_int
                {
                    if (*obj)._o._o_ac as libc::c_int > 0 as libc::c_int {
                        worth += (*obj)._o._o_ac as libc::c_int * 100 as libc::c_int;
                    } else {
                        worth = 10 as libc::c_int;
                    }
                }
                if (*obj)._o._o_flags as libc::c_int & 0x2 as libc::c_int == 0 {
                    worth /= 2 as libc::c_int;
                }
                (*obj)._o._o_flags =
                    ((*obj)._o._o_flags as libc::c_int | 0x2 as libc::c_int) as libc::c_short;
                *r_know.as_mut_ptr().offset((*obj)._o._o_which as isize) = 1 as libc::c_int != 0;
            }
            231 => {
                worth = (*ws_magic.as_mut_ptr().offset((*obj)._o._o_which as isize)).mi_worth
                    as libc::c_int;
                worth += 20 as libc::c_int * (*obj)._o._o_ac as libc::c_int;
                if (*obj)._o._o_flags as libc::c_int & 0x2 as libc::c_int == 0 {
                    worth /= 2 as libc::c_int;
                }
                (*obj)._o._o_flags =
                    ((*obj)._o._o_flags as libc::c_int | 0x2 as libc::c_int) as libc::c_short;
                *ws_know.as_mut_ptr().offset((*obj)._o._o_which as isize) = 1 as libc::c_int != 0;
            }
            12 => {
                worth = 1000 as libc::c_int;
            }
            _ => {}
        }
        if worth < 0 as libc::c_int {
            worth = 0 as libc::c_int;
        }
        cur_move(
            c as libc::c_int - 'a' as i32 + 1 as libc::c_int,
            0 as libc::c_int,
        );
        cur_printw(
            b"%c) %5d  %s\0" as *const u8 as *const libc::c_char,
            c as libc::c_int,
            worth,
            inv_name(obj, 0 as libc::c_int != 0),
        );
        purse += worth;
        c = c.wrapping_add(1);
        obj = (*obj)._t._l_next;
    }
    cur_move(
        c as libc::c_int - 'a' as i32 + 1 as libc::c_int,
        0 as libc::c_int,
    );
    cur_printw(
        b"   %5u  Gold Pieces          \0" as *const u8 as *const libc::c_char,
        oldpurse,
    );
    score(purse, 2 as libc::c_int, 0 as libc::c_int as libc::c_char);
    md_exit(0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn killname(mut monst: byte, mut doart: bool) -> *mut libc::c_char {
    let mut sp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut article: bool = false;
    sp = prbuf;
    article = 1 as libc::c_int != 0;
    match monst as libc::c_int {
        97 => {
            sp = b"arrow\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
        98 => {
            sp = b"bolt\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
        100 => {
            sp = b"dart\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
        115 => {
            sp = b"starvation\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
            article = 0 as libc::c_int != 0;
        }
        102 => {
            sp = b"fall\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
        _ => {
            if monst as libc::c_int >= 'A' as i32 && monst as libc::c_int <= 'Z' as i32 {
                sp = (*monsters
                    .as_mut_ptr()
                    .offset((monst as libc::c_int - 'A' as i32) as isize))
                .m_name;
            } else {
                sp = b"God\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
                article = 0 as libc::c_int != 0;
            }
        }
    }
    if doart as libc::c_int != 0 && article as libc::c_int != 0 {
        sprintf(
            prbuf,
            b"a%s \0" as *const u8 as *const libc::c_char,
            vowelstr(sp),
        );
    } else {
        *prbuf.offset(0 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
    }
    strcat(prbuf, sp);
    return prbuf;
}
