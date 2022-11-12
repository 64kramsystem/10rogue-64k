use crate::rnd;
use ::libc;
extern "C" {
    static mut maxrow: libc::c_int;
    static mut _level: *mut byte;
    static mut _flags: *mut byte;
    fn INDEX(y: libc::c_int, x: libc::c_int) -> libc::c_int;
    fn offmap(y: libc::c_int, x: libc::c_int) -> bool;
    fn rnd_pos(rp: *mut room, cp: *mut coord);
    static mut COLS: libc::c_int;
}
pub type byte = libc::c_uchar;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct coord {
    pub x: libc::c_int,
    pub y: libc::c_int,
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
static mut frcnt: libc::c_int = 0;
static mut ny: libc::c_int = 0;
static mut nx: libc::c_int = 0;
static mut topy: libc::c_int = 0;
static mut topx: libc::c_int = 0;
static mut maxx: libc::c_int = 0;
static mut maxy: libc::c_int = 0;
static mut fr_y: *mut libc::c_int = 0 as *const libc::c_int as *mut libc::c_int;
static mut fr_x: *mut libc::c_int = 0 as *const libc::c_int as *mut libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn draw_maze(mut rp: *mut room) {
    let mut y: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut fy: [libc::c_int; 100] = [0; 100];
    let mut fx: [libc::c_int; 100] = [0; 100];
    let mut psgcnt: libc::c_int = 0;
    let mut spos: coord = coord { x: 0, y: 0 };
    fr_y = fy.as_mut_ptr();
    fr_x = fx.as_mut_ptr();
    maxy = 0 as libc::c_int;
    maxx = maxy;
    topy = (*rp).r_pos.y;
    if topy == 0 as libc::c_int {
        (*rp).r_pos.y += 1;
        topy = (*rp).r_pos.y;
    }
    topx = (*rp).r_pos.x;
    y = topy;
    x = topx;
    splat(y, x);
    new_frontier(y, x);
    while frcnt != 0 {
        con_frnt();
        new_frontier(ny, nx);
    }
    (*rp).r_max.x = maxx - (*rp).r_pos.x + 1 as libc::c_int;
    (*rp).r_max.y = maxy - (*rp).r_pos.y + 1 as libc::c_int;
    loop {
        static mut ld: [coord; 4] = [
            {
                let mut init = coord {
                    x: -(1 as libc::c_int),
                    y: 0 as libc::c_int,
                };
                init
            },
            {
                let mut init = coord {
                    x: 0 as libc::c_int,
                    y: 1 as libc::c_int,
                };
                init
            },
            {
                let mut init = coord {
                    x: 1 as libc::c_int,
                    y: 0 as libc::c_int,
                };
                init
            },
            {
                let mut init = coord {
                    x: 0 as libc::c_int,
                    y: -(1 as libc::c_int),
                };
                init
            },
        ];
        let mut cp: *mut coord = 0 as *mut coord;
        let mut sh: libc::c_int = 0;
        rnd_pos(rp, &mut spos);
        psgcnt = 0 as libc::c_int;
        cp = ld.as_mut_ptr();
        sh = 1 as libc::c_int;
        while cp < &mut *ld.as_mut_ptr().offset(4 as libc::c_int as isize) as *mut coord {
            y = (*cp).y + spos.y;
            x = (*cp).x + spos.x;
            if !offmap(y, x)
                && *_level.offset(INDEX(y, x) as isize) as libc::c_int == 0xb1 as libc::c_int
            {
                psgcnt += sh;
            }
            sh <<= 1 as libc::c_int;
            cp = cp.offset(1);
        }
        if !(*_level.offset(INDEX(spos.y, spos.x) as isize) as libc::c_int == 0xb1 as libc::c_int
            || psgcnt % 5 as libc::c_int != 0)
        {
            break;
        }
    }
    splat(spos.y, spos.x);
}
#[no_mangle]
pub unsafe extern "C" fn new_frontier(mut y: libc::c_int, mut x: libc::c_int) {
    add_frnt(y - 2 as libc::c_int, x);
    add_frnt(y + 2 as libc::c_int, x);
    add_frnt(y, x - 2 as libc::c_int);
    add_frnt(y, x + 2 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn add_frnt(mut y: libc::c_int, mut x: libc::c_int) {
    if inrange(y, x) as libc::c_int != 0
        && *_level.offset(INDEX(y, x) as isize) as libc::c_int == ' ' as i32
    {
        *_level.offset(INDEX(y, x) as isize) = 'F' as i32 as byte;
        *fr_y.offset(frcnt as isize) = y;
        let fresh0 = frcnt;
        frcnt = frcnt + 1;
        *fr_x.offset(fresh0 as isize) = x;
    }
}
#[no_mangle]
pub unsafe extern "C" fn con_frnt() {
    let mut n: libc::c_int = 0;
    let mut which: libc::c_int = 0;
    let mut ydelt: libc::c_int = 0 as libc::c_int;
    let mut xdelt: libc::c_int = 0 as libc::c_int;
    let mut choice: [libc::c_int; 4] = [0; 4];
    let mut cnt: libc::c_int = 0 as libc::c_int;
    let mut y: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    n = rnd(frcnt);
    ny = *fr_y.offset(n as isize);
    nx = *fr_x.offset(n as isize);
    *fr_y.offset(n as isize) = *fr_y.offset((frcnt - 1 as libc::c_int) as isize);
    frcnt -= 1;
    *fr_x.offset(n as isize) = *fr_x.offset(frcnt as isize);
    if maze_at(ny - 2 as libc::c_int, nx) {
        let fresh1 = cnt;
        cnt = cnt + 1;
        choice[fresh1 as usize] = 0 as libc::c_int;
    }
    if maze_at(ny + 2 as libc::c_int, nx) {
        let fresh2 = cnt;
        cnt = cnt + 1;
        choice[fresh2 as usize] = 1 as libc::c_int;
    }
    if maze_at(ny, nx - 2 as libc::c_int) {
        let fresh3 = cnt;
        cnt = cnt + 1;
        choice[fresh3 as usize] = 2 as libc::c_int;
    }
    if maze_at(ny, nx + 2 as libc::c_int) {
        let fresh4 = cnt;
        cnt = cnt + 1;
        choice[fresh4 as usize] = 3 as libc::c_int;
    }
    which = choice[rnd(cnt) as usize];
    splat(ny, nx);
    match which {
        0 => {
            which = 1 as libc::c_int;
            ydelt = -(1 as libc::c_int);
        }
        1 => {
            which = 0 as libc::c_int;
            ydelt = 1 as libc::c_int;
        }
        2 => {
            which = 3 as libc::c_int;
            xdelt = -(1 as libc::c_int);
        }
        3 => {
            which = 2 as libc::c_int;
            xdelt = 1 as libc::c_int;
        }
        _ => {}
    }
    y = ny + ydelt;
    x = nx + xdelt;
    if inrange(y, x) {
        splat(y, x);
    }
}
#[no_mangle]
pub unsafe extern "C" fn maze_at(mut y: libc::c_int, mut x: libc::c_int) -> bool {
    return inrange(y, x) as libc::c_int != 0
        && *_level.offset(INDEX(y, x) as isize) as libc::c_int == 0xb1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn splat(mut y: libc::c_int, mut x: libc::c_int) {
    *_level.offset(INDEX(y, x) as isize) = 0xb1 as libc::c_int as byte;
    *_flags.offset(INDEX(y, x) as isize) = (0x20 as libc::c_int | 0x10 as libc::c_int) as byte;
    if x > maxx {
        maxx = x;
    }
    if y > maxy {
        maxy = y;
    }
}
#[no_mangle]
pub unsafe extern "C" fn inrange(mut y: libc::c_int, mut x: libc::c_int) -> bool {
    return y >= topy
        && y < topy + (maxrow + 1 as libc::c_int) / 3 as libc::c_int
        && x >= topx
        && x < topx + COLS / 3 as libc::c_int;
}
