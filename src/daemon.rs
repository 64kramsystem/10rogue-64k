use ::libc;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct delayed_action {
    pub d_func: Option<unsafe extern "C" fn() -> ()>,
    pub d_time: libc::c_int,
}
static mut d_list: [delayed_action; 20] = [delayed_action {
    d_func: None,
    d_time: 0,
}; 20];
unsafe extern "C" fn d_slot() -> *mut delayed_action {
    let mut dev: *mut delayed_action = 0 as *mut delayed_action;
    dev = d_list.as_mut_ptr();
    while dev < &mut *d_list.as_mut_ptr().offset(20 as libc::c_int as isize) as *mut delayed_action
    {
        if ((*dev).d_func).is_none() {
            return dev;
        }
        dev = dev.offset(1);
    }
    return 0 as *mut delayed_action;
}
unsafe extern "C" fn find_slot(
    mut func: Option<unsafe extern "C" fn() -> ()>,
) -> *mut delayed_action {
    let mut dev: *mut delayed_action = 0 as *mut delayed_action;
    dev = d_list.as_mut_ptr();
    while dev < &mut *d_list.as_mut_ptr().offset(20 as libc::c_int as isize) as *mut delayed_action
    {
        if func == (*dev).d_func {
            return dev;
        }
        dev = dev.offset(1);
    }
    return 0 as *mut delayed_action;
}
#[no_mangle]
pub unsafe extern "C" fn start_daemon(mut func: Option<unsafe extern "C" fn() -> ()>) {
    let mut dev: *mut delayed_action = 0 as *mut delayed_action;
    dev = d_slot();
    (*dev).d_func = func;
    (*dev).d_time = -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn do_daemons() {
    let mut dev: *mut delayed_action = 0 as *mut delayed_action;
    dev = d_list.as_mut_ptr();
    while dev < &mut *d_list.as_mut_ptr().offset(20 as libc::c_int as isize) as *mut delayed_action
    {
        if (*dev).d_time == -(1 as libc::c_int) && ((*dev).d_func).is_some() {
            ::core::mem::transmute::<_, fn()>(
                (Some(((*dev).d_func).expect("non-null function pointer")))
                    .expect("non-null function pointer"),
            )();
        }
        dev = dev.offset(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn fuse(
    mut func: Option<unsafe extern "C" fn() -> ()>,
    mut time: libc::c_int,
) {
    let mut wire: *mut delayed_action = 0 as *mut delayed_action;
    wire = d_slot();
    (*wire).d_func = func;
    (*wire).d_time = time;
}
#[no_mangle]
pub unsafe extern "C" fn lengthen(
    mut func: Option<unsafe extern "C" fn() -> ()>,
    mut xtime: libc::c_int,
) {
    let mut wire: *mut delayed_action = 0 as *mut delayed_action;
    wire = find_slot(func);
    if wire.is_null() {
        return;
    }
    (*wire).d_time += xtime;
}
#[no_mangle]
pub unsafe extern "C" fn extinguish(mut func: Option<unsafe extern "C" fn() -> ()>) {
    let mut wire: *mut delayed_action = 0 as *mut delayed_action;
    wire = find_slot(func);
    if wire.is_null() {
        return;
    }
    (*wire).d_func = None;
}
#[no_mangle]
pub unsafe extern "C" fn do_fuses() {
    let mut wire: *mut delayed_action = 0 as *mut delayed_action;
    wire = d_list.as_mut_ptr();
    while wire < &mut *d_list.as_mut_ptr().offset(20 as libc::c_int as isize) as *mut delayed_action
    {
        if ((*wire).d_func).is_some() && (*wire).d_time > 0 as libc::c_int && {
            (*wire).d_time -= 1;
            (*wire).d_time == 0 as libc::c_int
        } {
            ::core::mem::transmute::<_, fn()>(
                (Some(((*wire).d_func).expect("non-null function pointer")))
                    .expect("non-null function pointer"),
            )();
            (*wire).d_func = None;
        }
        wire = wire.offset(1);
    }
}
