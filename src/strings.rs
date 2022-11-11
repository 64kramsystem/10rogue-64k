use ::libc;
extern "C" {
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn __ctype_tolower_loc() -> *mut *const __int32_t;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
}
pub type __int32_t = libc::c_int;
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
#[inline]
unsafe extern "C" fn tolower(mut __c: libc::c_int) -> libc::c_int {
    return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
        *(*__ctype_tolower_loc()).offset(__c as isize)
    } else {
        __c
    };
}
#[no_mangle]
pub unsafe extern "C" fn is_alpha(mut ch: libc::c_char) -> bool {
    return ch as libc::c_int & !(0x7f as libc::c_int) == 0 as libc::c_int
        && *(*__ctype_b_loc()).offset(ch as libc::c_int as isize) as libc::c_int
            & _ISalpha as libc::c_int as libc::c_ushort as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn is_upper(mut ch: libc::c_char) -> bool {
    return ch as libc::c_int & !(0x7f as libc::c_int) == 0 as libc::c_int
        && *(*__ctype_b_loc()).offset(ch as libc::c_int as isize) as libc::c_int
            & _ISupper as libc::c_int as libc::c_ushort as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn is_lower(mut ch: libc::c_char) -> bool {
    return ch as libc::c_int & !(0x7f as libc::c_int) == 0 as libc::c_int
        && *(*__ctype_b_loc()).offset(ch as libc::c_int as isize) as libc::c_int
            & _ISlower as libc::c_int as libc::c_ushort as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn is_digit(mut ch: libc::c_char) -> bool {
    return ch as libc::c_int & !(0x7f as libc::c_int) == 0 as libc::c_int
        && *(*__ctype_b_loc()).offset(ch as libc::c_int as isize) as libc::c_int
            & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn is_space(mut ch: libc::c_char) -> bool {
    return ch as libc::c_int & !(0x7f as libc::c_int) == 0 as libc::c_int
        && *(*__ctype_b_loc()).offset(ch as libc::c_int as isize) as libc::c_int
            & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn is_print(mut ch: libc::c_char) -> bool {
    return ch as libc::c_int & !(0x7f as libc::c_int) == 0 as libc::c_int
        && *(*__ctype_b_loc()).offset(ch as libc::c_int as isize) as libc::c_int
            & _ISprint as libc::c_int as libc::c_ushort as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn stccpy(
    mut s1: *mut libc::c_char,
    mut s2: *mut libc::c_char,
    mut count: libc::c_int,
) -> *mut libc::c_char {
    loop {
        let fresh0 = count;
        count = count - 1;
        if !(fresh0 > 0 as libc::c_int && *s2 as libc::c_int != 0) {
            break;
        }
        let fresh1 = s2;
        s2 = s2.offset(1);
        let fresh2 = s1;
        s1 = s1.offset(1);
        *fresh2 = *fresh1;
    }
    *s1 = 0 as libc::c_int as libc::c_char;
    return s1;
}
#[no_mangle]
pub unsafe extern "C" fn stpblk(mut str: *mut libc::c_char) -> *mut libc::c_char {
    while is_space(*str) {
        str = str.offset(1);
    }
    return str;
}
#[no_mangle]
pub unsafe extern "C" fn endblk(mut str: *mut libc::c_char) -> *mut libc::c_char {
    let mut backup: *mut libc::c_char = 0 as *mut libc::c_char;
    backup = str.offset(strlen(str) as isize);
    while backup != str
        && {
            backup = backup.offset(-1);
            is_space(*backup) as libc::c_int != 0
        }
    {
        *backup = 0 as libc::c_int as libc::c_char;
    }
    return str;
}
#[no_mangle]
pub unsafe extern "C" fn lcase(mut str: *mut libc::c_char) {
    loop {
        *str = ({
            let mut __res: libc::c_int = 0;
            if ::core::mem::size_of::<libc::c_char>() as libc::c_ulong
                > 1 as libc::c_int as libc::c_ulong
            {
                if 0 != 0 {
                    let mut __c: libc::c_int = *str as libc::c_int;
                    __res = if __c < -(128 as libc::c_int) || __c > 255 as libc::c_int {
                        __c
                    } else {
                        *(*__ctype_tolower_loc()).offset(__c as isize)
                    };
                } else {
                    __res = tolower(*str as libc::c_int);
                }
            } else {
                __res = *(*__ctype_tolower_loc()).offset(*str as libc::c_int as isize);
            }
            __res
        }) as libc::c_char;
        if !(*str != 0) {
            break;
        }
        str = str.offset(1);
    };
}
