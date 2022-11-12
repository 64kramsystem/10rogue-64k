use crate::rnd;
use ::libc;
extern "C" {
    fn abs(_: libc::c_int) -> libc::c_int;
    static mut level: libc::c_int;
    static mut passages: [room; 0];
    static mut rooms: [room; 0];
    static mut _level: *mut byte;
    static mut _flags: *mut byte;
    fn _ce(a: *mut coord, b: *mut coord) -> bool;
    fn offmap(y: libc::c_int, x: libc::c_int) -> bool;
    fn INDEX(y: libc::c_int, x: libc::c_int) -> libc::c_int;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rdes {
    pub conn: [libc::c_char; 9],
    pub isconn: [libc::c_char; 9],
    pub ingraph: libc::c_char,
}
#[no_mangle]
pub unsafe extern "C" fn conn(mut r1: libc::c_int, mut r2: libc::c_int) {
    let mut rpf: *mut room = 0 as *mut room;
    let mut rpt: *mut room = 0 as *mut room;
    let mut rmt: libc::c_int = 0;
    let mut rm: libc::c_int = 0;
    let mut distance: libc::c_int = 0 as libc::c_int;
    let mut turn_spot: libc::c_int = 0;
    let mut turn_distance: libc::c_int = 0;
    let mut direc: libc::c_int = 0;
    let mut del: coord = coord { x: 0, y: 0 };
    let mut curr: coord = coord { x: 0, y: 0 };
    let mut turn_delta: coord = coord { x: 0, y: 0 };
    let mut spos: coord = coord { x: 0, y: 0 };
    let mut epos: coord = coord { x: 0, y: 0 };
    if r1 < r2 {
        rm = r1;
        if r1 + 1 as libc::c_int == r2 {
            direc = 'r' as i32;
        } else {
            direc = 'd' as i32;
        }
    } else {
        rm = r2;
        if r2 + 1 as libc::c_int == r1 {
            direc = 'r' as i32;
        } else {
            direc = 'd' as i32;
        }
    }
    rpf = &mut *rooms.as_mut_ptr().offset(rm as isize) as *mut room;
    if direc == 'd' as i32 {
        rmt = rm + 3 as libc::c_int;
        rpt = &mut *rooms.as_mut_ptr().offset(rmt as isize) as *mut room;
        del.x = 0 as libc::c_int;
        del.y = 1 as libc::c_int;
        if (*rpf).r_flags as libc::c_int & 0x2 as libc::c_int == 0 as libc::c_int
            || (*rpf).r_flags as libc::c_int & 0x4 as libc::c_int != 0
        {
            spos.y = (*rpf).r_pos.y + (*rpf).r_max.y - 1 as libc::c_int;
            loop {
                spos.x = (*rpf).r_pos.x + rnd((*rpf).r_max.x - 2 as libc::c_int) + 1 as libc::c_int;
                if !(*_level.offset(INDEX(spos.y, spos.x) as isize) as libc::c_int == ' ' as i32) {
                    break;
                }
            }
        } else {
            spos.x = (*rpf).r_pos.x;
            spos.y = (*rpf).r_pos.y;
        }
        epos.y = (*rpt).r_pos.y;
        if (*rpt).r_flags as libc::c_int & 0x2 as libc::c_int == 0 as libc::c_int
            || (*rpt).r_flags as libc::c_int & 0x4 as libc::c_int != 0
        {
            loop {
                epos.x = (*rpt).r_pos.x + rnd((*rpt).r_max.x - 2 as libc::c_int) + 1 as libc::c_int;
                if !(*_level.offset(INDEX(epos.y, epos.x) as isize) as libc::c_int == ' ' as i32) {
                    break;
                }
            }
        } else {
            epos.x = (*rpt).r_pos.x;
        }
        distance = abs(spos.y - epos.y) - 1 as libc::c_int;
        turn_delta.y = 0 as libc::c_int;
        turn_delta.x = if spos.x < epos.x {
            1 as libc::c_int
        } else {
            -(1 as libc::c_int)
        };
        turn_distance = abs(spos.x - epos.x);
    } else if direc == 'r' as i32 {
        rmt = rm + 1 as libc::c_int;
        rpt = &mut *rooms.as_mut_ptr().offset(rmt as isize) as *mut room;
        del.x = 1 as libc::c_int;
        del.y = 0 as libc::c_int;
        if (*rpf).r_flags as libc::c_int & 0x2 as libc::c_int == 0 as libc::c_int
            || (*rpf).r_flags as libc::c_int & 0x4 as libc::c_int != 0
        {
            spos.x = (*rpf).r_pos.x + (*rpf).r_max.x - 1 as libc::c_int;
            loop {
                spos.y = (*rpf).r_pos.y + rnd((*rpf).r_max.y - 2 as libc::c_int) + 1 as libc::c_int;
                if !(*_level.offset(INDEX(spos.y, spos.x) as isize) as libc::c_int == ' ' as i32) {
                    break;
                }
            }
        } else {
            spos.x = (*rpf).r_pos.x;
            spos.y = (*rpf).r_pos.y;
        }
        epos.x = (*rpt).r_pos.x;
        if (*rpt).r_flags as libc::c_int & 0x2 as libc::c_int == 0 as libc::c_int
            || (*rpt).r_flags as libc::c_int & 0x4 as libc::c_int != 0
        {
            loop {
                epos.y = (*rpt).r_pos.y + rnd((*rpt).r_max.y - 2 as libc::c_int) + 1 as libc::c_int;
                if !(*_level.offset(INDEX(epos.y, epos.x) as isize) as libc::c_int == ' ' as i32) {
                    break;
                }
            }
        } else {
            epos.y = (*rpt).r_pos.y;
        }
        distance = abs(spos.x - epos.x) - 1 as libc::c_int;
        turn_delta.y = if spos.y < epos.y {
            1 as libc::c_int
        } else {
            -(1 as libc::c_int)
        };
        turn_delta.x = 0 as libc::c_int;
        turn_distance = abs(spos.y - epos.y);
    }
    turn_spot = rnd(distance - 1 as libc::c_int) + 1 as libc::c_int;
    if (*rpf).r_flags as libc::c_int & 0x2 as libc::c_int == 0 {
        door(rpf, &mut spos);
    } else {
        psplat(spos.y, spos.x);
    }
    if !rpt.is_null() && (*rpt).r_flags as libc::c_int & 0x2 as libc::c_int == 0 {
        door(rpt, &mut epos);
    } else {
        psplat(epos.y, epos.x);
    }
    curr.x = spos.x;
    curr.y = spos.y;
    while distance != 0 {
        curr.x += del.x;
        curr.y += del.y;
        if distance == turn_spot {
            loop {
                let fresh0 = turn_distance;
                turn_distance = turn_distance - 1;
                if !(fresh0 != 0) {
                    break;
                }
                psplat(curr.y, curr.x);
                curr.x += turn_delta.x;
                curr.y += turn_delta.y;
            }
        }
        psplat(curr.y, curr.x);
        distance -= 1;
    }
    curr.x += del.x;
    curr.y += del.y;
    if !_ce(&mut curr, &mut epos) {
        epos.x -= del.x;
        epos.y -= del.y;
        psplat(epos.y, epos.x);
    }
}
#[no_mangle]
pub unsafe extern "C" fn do_passages() {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut roomcount: libc::c_int = 0;
    static mut rdes_0: [rdes; 9] = [
        {
            let mut init = rdes {
                conn: [
                    0 as libc::c_int as libc::c_char,
                    1 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    1 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                ],
                isconn: [
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                ],
                ingraph: 0 as libc::c_int as libc::c_char,
            };
            init
        },
        {
            let mut init = rdes {
                conn: [
                    1 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    1 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    1 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                ],
                isconn: [
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                ],
                ingraph: 0 as libc::c_int as libc::c_char,
            };
            init
        },
        {
            let mut init = rdes {
                conn: [
                    0 as libc::c_int as libc::c_char,
                    1 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    1 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                ],
                isconn: [
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                ],
                ingraph: 0 as libc::c_int as libc::c_char,
            };
            init
        },
        {
            let mut init = rdes {
                conn: [
                    1 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    1 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    1 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                ],
                isconn: [
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                ],
                ingraph: 0 as libc::c_int as libc::c_char,
            };
            init
        },
        {
            let mut init = rdes {
                conn: [
                    0 as libc::c_int as libc::c_char,
                    1 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    1 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    1 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    1 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                ],
                isconn: [
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                ],
                ingraph: 0 as libc::c_int as libc::c_char,
            };
            init
        },
        {
            let mut init = rdes {
                conn: [
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    1 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    1 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    1 as libc::c_int as libc::c_char,
                ],
                isconn: [
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                ],
                ingraph: 0 as libc::c_int as libc::c_char,
            };
            init
        },
        {
            let mut init = rdes {
                conn: [
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    1 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    1 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                ],
                isconn: [
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                ],
                ingraph: 0 as libc::c_int as libc::c_char,
            };
            init
        },
        {
            let mut init = rdes {
                conn: [
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    1 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    1 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    1 as libc::c_int as libc::c_char,
                ],
                isconn: [
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                ],
                ingraph: 0 as libc::c_int as libc::c_char,
            };
            init
        },
        {
            let mut init = rdes {
                conn: [
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    1 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    1 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                ],
                isconn: [
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                ],
                ingraph: 0 as libc::c_int as libc::c_char,
            };
            init
        },
    ];
    let mut r1: *mut rdes = 0 as *mut rdes;
    let mut r2: *mut rdes = 0 as *mut rdes;
    r1 = rdes_0.as_mut_ptr();
    while r1 < &mut *rdes_0.as_mut_ptr().offset(9 as libc::c_int as isize) as *mut rdes {
        j = 0 as libc::c_int;
        while j < 9 as libc::c_int {
            (*r1).isconn[j as usize] = 0 as libc::c_int as libc::c_char;
            j += 1;
        }
        (*r1).ingraph = 0 as libc::c_int as libc::c_char;
        r1 = r1.offset(1);
    }
    roomcount = 1 as libc::c_int;
    r1 = &mut *rdes_0.as_mut_ptr().offset(rnd(9) as isize) as *mut rdes;
    (*r1).ingraph = 1 as libc::c_int as libc::c_char;
    loop {
        j = 0 as libc::c_int;
        i = 0 as libc::c_int;
        while i < 9 as libc::c_int {
            if (*r1).conn[i as usize] as libc::c_int != 0 && rdes_0[i as usize].ingraph == 0 && {
                j += 1;
                rnd(j) == 0 as libc::c_int
            } {
                r2 = &mut *rdes_0.as_mut_ptr().offset(i as isize) as *mut rdes;
            }
            i += 1;
        }
        if j == 0 as libc::c_int {
            loop {
                r1 = &mut *rdes_0.as_mut_ptr().offset(rnd(9) as isize) as *mut rdes;
                if !((*r1).ingraph == 0) {
                    break;
                }
            }
        } else {
            (*r2).ingraph = 1 as libc::c_int as libc::c_char;
            i = r1.offset_from(rdes_0.as_mut_ptr()) as libc::c_long as libc::c_int;
            j = r2.offset_from(rdes_0.as_mut_ptr()) as libc::c_long as libc::c_int;
            conn(i, j);
            (*r1).isconn[j as usize] = 1 as libc::c_int as libc::c_char;
            (*r2).isconn[i as usize] = 1 as libc::c_int as libc::c_char;
            roomcount += 1;
        }
        if !(roomcount < 9 as libc::c_int) {
            break;
        }
    }
    roomcount = rnd(5 as libc::c_int);
    while roomcount > 0 as libc::c_int {
        r1 = &mut *rdes_0.as_mut_ptr().offset(rnd(9) as isize) as *mut rdes;
        j = 0 as libc::c_int;
        i = 0 as libc::c_int;
        while i < 9 as libc::c_int {
            if (*r1).conn[i as usize] as libc::c_int != 0 && (*r1).isconn[i as usize] == 0 && {
                j += 1;
                rnd(j) == 0 as libc::c_int
            } {
                r2 = &mut *rdes_0.as_mut_ptr().offset(i as isize) as *mut rdes;
            }
            i += 1;
        }
        if j != 0 as libc::c_int {
            i = r1.offset_from(rdes_0.as_mut_ptr()) as libc::c_long as libc::c_int;
            j = r2.offset_from(rdes_0.as_mut_ptr()) as libc::c_long as libc::c_int;
            conn(i, j);
            (*r1).isconn[j as usize] = 1 as libc::c_int as libc::c_char;
            (*r2).isconn[i as usize] = 1 as libc::c_int as libc::c_char;
        }
        roomcount -= 1;
    }
    passnum();
}
#[no_mangle]
pub unsafe extern "C" fn door(mut rm: *mut room, mut cp: *mut coord) {
    let mut index: libc::c_int = 0;
    let mut xit: libc::c_int = 0;
    index = INDEX((*cp).y, (*cp).x);
    if (rnd(10 as libc::c_int) + 1 as libc::c_int) < level
        && rnd(5 as libc::c_int) == 0 as libc::c_int
    {
        *_level.offset(index as isize) = (if (*cp).y == (*rm).r_pos.y
            || (*cp).y == (*rm).r_pos.y + (*rm).r_max.y - 1 as libc::c_int
        {
            0xcd as libc::c_int
        } else {
            0xba as libc::c_int
        }) as byte;
        let ref mut fresh1 = *_flags.offset(index as isize);
        *fresh1 = (*fresh1 as libc::c_int & !(0x10 as libc::c_int)) as byte;
    } else {
        *_level.offset(index as isize) = 0xce as libc::c_int as byte;
    }
    let fresh2 = (*rm).r_nexits;
    (*rm).r_nexits = (*rm).r_nexits + 1;
    xit = fresh2;
    (*rm).r_exit[xit as usize].y = (*cp).y;
    (*rm).r_exit[xit as usize].x = (*cp).x;
}
static mut pnum: libc::c_int = 0;
static mut newpnum: byte = 0;
#[no_mangle]
pub unsafe extern "C" fn passnum() {
    let mut rp: *mut room = 0 as *mut room;
    let mut i: libc::c_int = 0;
    pnum = 0 as libc::c_int;
    newpnum = 0 as libc::c_int as byte;
    rp = passages.as_mut_ptr();
    while rp < &mut *passages.as_mut_ptr().offset(13 as libc::c_int as isize) as *mut room {
        (*rp).r_nexits = 0 as libc::c_int;
        rp = rp.offset(1);
    }
    rp = rooms.as_mut_ptr();
    while rp < &mut *rooms.as_mut_ptr().offset(9 as libc::c_int as isize) as *mut room {
        i = 0 as libc::c_int;
        while i < (*rp).r_nexits {
            newpnum = newpnum.wrapping_add(1);
            numpass((*rp).r_exit[i as usize].y, (*rp).r_exit[i as usize].x);
            i += 1;
        }
        rp = rp.offset(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn numpass(mut y: libc::c_int, mut x: libc::c_int) {
    let mut fp: *mut byte = 0 as *mut byte;
    let mut rp: *mut room = 0 as *mut room;
    let mut ch: byte = 0;
    if offmap(y, x) {
        return;
    }
    fp = &mut *_flags
        .offset(
            (INDEX as unsafe extern "C" fn(libc::c_int, libc::c_int) -> libc::c_int)(y, x) as isize,
        ) as *mut byte;
    if *fp as libc::c_int & 0xf as libc::c_int != 0 {
        return;
    }
    if newpnum != 0 {
        pnum += 1;
        newpnum = 0 as libc::c_int as byte;
    }
    ch = *_level.offset(INDEX(y, x) as isize);
    if ch as libc::c_int == 0xce as libc::c_int
        || *fp as libc::c_int & 0x10 as libc::c_int == 0 && ch as libc::c_int != 0xfa as libc::c_int
    {
        rp = &mut *passages.as_mut_ptr().offset(pnum as isize) as *mut room;
        (*rp).r_exit[(*rp).r_nexits as usize].y = y;
        let fresh3 = (*rp).r_nexits;
        (*rp).r_nexits = (*rp).r_nexits + 1;
        (*rp).r_exit[fresh3 as usize].x = x;
    } else if *fp as libc::c_int & 0x40 as libc::c_int == 0 {
        return;
    }
    *fp = (*fp as libc::c_int | pnum) as byte;
    numpass(y + 1 as libc::c_int, x);
    numpass(y - 1 as libc::c_int, x);
    numpass(y, x + 1 as libc::c_int);
    numpass(y, x - 1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn psplat(mut y: libc::c_int, mut x: libc::c_int) {
    let mut idx: libc::c_int = 0;
    idx = INDEX(y, x);
    *_level.offset(idx as isize) = 0xb1 as libc::c_int as byte;
    let ref mut fresh4 = *_flags.offset(idx as isize);
    *fresh4 = (*fresh4 as libc::c_int | 0x40 as libc::c_int) as byte;
}
