use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type SDL_Window;
    pub type SDL_SysWMmsg;
    pub type SDL_Renderer;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn __errno_location() -> *mut libc::c_int;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn vsnprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ::core::ffi::VaList,
    ) -> libc::c_int;
    fn fgetc(__stream: *mut FILE) -> libc::c_int;
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn feof(__stream: *mut FILE) -> libc::c_int;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn SDL_GetError() -> *const libc::c_char;
    fn SDL_Quit();
    fn SDL_Init(flags: Uint32) -> libc::c_int;
    fn SDL_Delay(ms: Uint32);
    fn SDL_GetTicks() -> Uint32;
    fn SDL_DestroyRenderer(renderer: *mut SDL_Renderer);
    fn SDL_RenderPresent(renderer: *mut SDL_Renderer);
    fn SDL_RenderDrawPoint(
        renderer: *mut SDL_Renderer,
        x: libc::c_int,
        y: libc::c_int,
    ) -> libc::c_int;
    fn SDL_RenderClear(renderer: *mut SDL_Renderer) -> libc::c_int;
    fn SDL_SetRenderDrawColor(
        renderer: *mut SDL_Renderer,
        r: Uint8,
        g: Uint8,
        b: Uint8,
        a: Uint8,
    ) -> libc::c_int;
    fn SDL_RenderSetLogicalSize(
        renderer: *mut SDL_Renderer,
        w: libc::c_int,
        h: libc::c_int,
    ) -> libc::c_int;
    fn SDL_CreateWindowAndRenderer(
        width: libc::c_int,
        height: libc::c_int,
        window_flags: Uint32,
        window: *mut *mut SDL_Window,
        renderer: *mut *mut SDL_Renderer,
    ) -> libc::c_int;
    fn SDL_SetHint(name: *const libc::c_char, value: *const libc::c_char) -> SDL_bool;
    fn SDL_PollEvent(event: *mut SDL_Event) -> libc::c_int;
    fn SDL_DestroyWindow(window: *mut SDL_Window);
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type va_list = __builtin_va_list;
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
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
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type SDL_bool = libc::c_uint;
pub const SDL_TRUE: SDL_bool = 1;
pub const SDL_FALSE: SDL_bool = 0;
pub type Uint8 = uint8_t;
pub type Sint16 = int16_t;
pub type Uint16 = uint16_t;
pub type Sint32 = int32_t;
pub type Uint32 = uint32_t;
pub type Sint64 = int64_t;
pub type C2RustUnnamed = libc::c_uint;
pub const SDL_WINDOW_INPUT_GRABBED: C2RustUnnamed = 256;
pub const SDL_WINDOW_METAL: C2RustUnnamed = 536870912;
pub const SDL_WINDOW_VULKAN: C2RustUnnamed = 268435456;
pub const SDL_WINDOW_KEYBOARD_GRABBED: C2RustUnnamed = 1048576;
pub const SDL_WINDOW_POPUP_MENU: C2RustUnnamed = 524288;
pub const SDL_WINDOW_TOOLTIP: C2RustUnnamed = 262144;
pub const SDL_WINDOW_UTILITY: C2RustUnnamed = 131072;
pub const SDL_WINDOW_SKIP_TASKBAR: C2RustUnnamed = 65536;
pub const SDL_WINDOW_ALWAYS_ON_TOP: C2RustUnnamed = 32768;
pub const SDL_WINDOW_MOUSE_CAPTURE: C2RustUnnamed = 16384;
pub const SDL_WINDOW_ALLOW_HIGHDPI: C2RustUnnamed = 8192;
pub const SDL_WINDOW_FOREIGN: C2RustUnnamed = 2048;
pub const SDL_WINDOW_FULLSCREEN_DESKTOP: C2RustUnnamed = 4097;
pub const SDL_WINDOW_MOUSE_FOCUS: C2RustUnnamed = 1024;
pub const SDL_WINDOW_INPUT_FOCUS: C2RustUnnamed = 512;
pub const SDL_WINDOW_MOUSE_GRABBED: C2RustUnnamed = 256;
pub const SDL_WINDOW_MAXIMIZED: C2RustUnnamed = 128;
pub const SDL_WINDOW_MINIMIZED: C2RustUnnamed = 64;
pub const SDL_WINDOW_RESIZABLE: C2RustUnnamed = 32;
pub const SDL_WINDOW_BORDERLESS: C2RustUnnamed = 16;
pub const SDL_WINDOW_HIDDEN: C2RustUnnamed = 8;
pub const SDL_WINDOW_SHOWN: C2RustUnnamed = 4;
pub const SDL_WINDOW_OPENGL: C2RustUnnamed = 2;
pub const SDL_WINDOW_FULLSCREEN: C2RustUnnamed = 1;
pub type SDL_Scancode = libc::c_uint;
pub const SDL_NUM_SCANCODES: SDL_Scancode = 512;
pub const SDL_SCANCODE_AUDIOFASTFORWARD: SDL_Scancode = 286;
pub const SDL_SCANCODE_AUDIOREWIND: SDL_Scancode = 285;
pub const SDL_SCANCODE_APP2: SDL_Scancode = 284;
pub const SDL_SCANCODE_APP1: SDL_Scancode = 283;
pub const SDL_SCANCODE_SLEEP: SDL_Scancode = 282;
pub const SDL_SCANCODE_EJECT: SDL_Scancode = 281;
pub const SDL_SCANCODE_KBDILLUMUP: SDL_Scancode = 280;
pub const SDL_SCANCODE_KBDILLUMDOWN: SDL_Scancode = 279;
pub const SDL_SCANCODE_KBDILLUMTOGGLE: SDL_Scancode = 278;
pub const SDL_SCANCODE_DISPLAYSWITCH: SDL_Scancode = 277;
pub const SDL_SCANCODE_BRIGHTNESSUP: SDL_Scancode = 276;
pub const SDL_SCANCODE_BRIGHTNESSDOWN: SDL_Scancode = 275;
pub const SDL_SCANCODE_AC_BOOKMARKS: SDL_Scancode = 274;
pub const SDL_SCANCODE_AC_REFRESH: SDL_Scancode = 273;
pub const SDL_SCANCODE_AC_STOP: SDL_Scancode = 272;
pub const SDL_SCANCODE_AC_FORWARD: SDL_Scancode = 271;
pub const SDL_SCANCODE_AC_BACK: SDL_Scancode = 270;
pub const SDL_SCANCODE_AC_HOME: SDL_Scancode = 269;
pub const SDL_SCANCODE_AC_SEARCH: SDL_Scancode = 268;
pub const SDL_SCANCODE_COMPUTER: SDL_Scancode = 267;
pub const SDL_SCANCODE_CALCULATOR: SDL_Scancode = 266;
pub const SDL_SCANCODE_MAIL: SDL_Scancode = 265;
pub const SDL_SCANCODE_WWW: SDL_Scancode = 264;
pub const SDL_SCANCODE_MEDIASELECT: SDL_Scancode = 263;
pub const SDL_SCANCODE_AUDIOMUTE: SDL_Scancode = 262;
pub const SDL_SCANCODE_AUDIOPLAY: SDL_Scancode = 261;
pub const SDL_SCANCODE_AUDIOSTOP: SDL_Scancode = 260;
pub const SDL_SCANCODE_AUDIOPREV: SDL_Scancode = 259;
pub const SDL_SCANCODE_AUDIONEXT: SDL_Scancode = 258;
pub const SDL_SCANCODE_MODE: SDL_Scancode = 257;
pub const SDL_SCANCODE_RGUI: SDL_Scancode = 231;
pub const SDL_SCANCODE_RALT: SDL_Scancode = 230;
pub const SDL_SCANCODE_RSHIFT: SDL_Scancode = 229;
pub const SDL_SCANCODE_RCTRL: SDL_Scancode = 228;
pub const SDL_SCANCODE_LGUI: SDL_Scancode = 227;
pub const SDL_SCANCODE_LALT: SDL_Scancode = 226;
pub const SDL_SCANCODE_LSHIFT: SDL_Scancode = 225;
pub const SDL_SCANCODE_LCTRL: SDL_Scancode = 224;
pub const SDL_SCANCODE_KP_HEXADECIMAL: SDL_Scancode = 221;
pub const SDL_SCANCODE_KP_DECIMAL: SDL_Scancode = 220;
pub const SDL_SCANCODE_KP_OCTAL: SDL_Scancode = 219;
pub const SDL_SCANCODE_KP_BINARY: SDL_Scancode = 218;
pub const SDL_SCANCODE_KP_CLEARENTRY: SDL_Scancode = 217;
pub const SDL_SCANCODE_KP_CLEAR: SDL_Scancode = 216;
pub const SDL_SCANCODE_KP_PLUSMINUS: SDL_Scancode = 215;
pub const SDL_SCANCODE_KP_MEMDIVIDE: SDL_Scancode = 214;
pub const SDL_SCANCODE_KP_MEMMULTIPLY: SDL_Scancode = 213;
pub const SDL_SCANCODE_KP_MEMSUBTRACT: SDL_Scancode = 212;
pub const SDL_SCANCODE_KP_MEMADD: SDL_Scancode = 211;
pub const SDL_SCANCODE_KP_MEMCLEAR: SDL_Scancode = 210;
pub const SDL_SCANCODE_KP_MEMRECALL: SDL_Scancode = 209;
pub const SDL_SCANCODE_KP_MEMSTORE: SDL_Scancode = 208;
pub const SDL_SCANCODE_KP_EXCLAM: SDL_Scancode = 207;
pub const SDL_SCANCODE_KP_AT: SDL_Scancode = 206;
pub const SDL_SCANCODE_KP_SPACE: SDL_Scancode = 205;
pub const SDL_SCANCODE_KP_HASH: SDL_Scancode = 204;
pub const SDL_SCANCODE_KP_COLON: SDL_Scancode = 203;
pub const SDL_SCANCODE_KP_DBLVERTICALBAR: SDL_Scancode = 202;
pub const SDL_SCANCODE_KP_VERTICALBAR: SDL_Scancode = 201;
pub const SDL_SCANCODE_KP_DBLAMPERSAND: SDL_Scancode = 200;
pub const SDL_SCANCODE_KP_AMPERSAND: SDL_Scancode = 199;
pub const SDL_SCANCODE_KP_GREATER: SDL_Scancode = 198;
pub const SDL_SCANCODE_KP_LESS: SDL_Scancode = 197;
pub const SDL_SCANCODE_KP_PERCENT: SDL_Scancode = 196;
pub const SDL_SCANCODE_KP_POWER: SDL_Scancode = 195;
pub const SDL_SCANCODE_KP_XOR: SDL_Scancode = 194;
pub const SDL_SCANCODE_KP_F: SDL_Scancode = 193;
pub const SDL_SCANCODE_KP_E: SDL_Scancode = 192;
pub const SDL_SCANCODE_KP_D: SDL_Scancode = 191;
pub const SDL_SCANCODE_KP_C: SDL_Scancode = 190;
pub const SDL_SCANCODE_KP_B: SDL_Scancode = 189;
pub const SDL_SCANCODE_KP_A: SDL_Scancode = 188;
pub const SDL_SCANCODE_KP_BACKSPACE: SDL_Scancode = 187;
pub const SDL_SCANCODE_KP_TAB: SDL_Scancode = 186;
pub const SDL_SCANCODE_KP_RIGHTBRACE: SDL_Scancode = 185;
pub const SDL_SCANCODE_KP_LEFTBRACE: SDL_Scancode = 184;
pub const SDL_SCANCODE_KP_RIGHTPAREN: SDL_Scancode = 183;
pub const SDL_SCANCODE_KP_LEFTPAREN: SDL_Scancode = 182;
pub const SDL_SCANCODE_CURRENCYSUBUNIT: SDL_Scancode = 181;
pub const SDL_SCANCODE_CURRENCYUNIT: SDL_Scancode = 180;
pub const SDL_SCANCODE_DECIMALSEPARATOR: SDL_Scancode = 179;
pub const SDL_SCANCODE_THOUSANDSSEPARATOR: SDL_Scancode = 178;
pub const SDL_SCANCODE_KP_000: SDL_Scancode = 177;
pub const SDL_SCANCODE_KP_00: SDL_Scancode = 176;
pub const SDL_SCANCODE_EXSEL: SDL_Scancode = 164;
pub const SDL_SCANCODE_CRSEL: SDL_Scancode = 163;
pub const SDL_SCANCODE_CLEARAGAIN: SDL_Scancode = 162;
pub const SDL_SCANCODE_OPER: SDL_Scancode = 161;
pub const SDL_SCANCODE_OUT: SDL_Scancode = 160;
pub const SDL_SCANCODE_SEPARATOR: SDL_Scancode = 159;
pub const SDL_SCANCODE_RETURN2: SDL_Scancode = 158;
pub const SDL_SCANCODE_PRIOR: SDL_Scancode = 157;
pub const SDL_SCANCODE_CLEAR: SDL_Scancode = 156;
pub const SDL_SCANCODE_CANCEL: SDL_Scancode = 155;
pub const SDL_SCANCODE_SYSREQ: SDL_Scancode = 154;
pub const SDL_SCANCODE_ALTERASE: SDL_Scancode = 153;
pub const SDL_SCANCODE_LANG9: SDL_Scancode = 152;
pub const SDL_SCANCODE_LANG8: SDL_Scancode = 151;
pub const SDL_SCANCODE_LANG7: SDL_Scancode = 150;
pub const SDL_SCANCODE_LANG6: SDL_Scancode = 149;
pub const SDL_SCANCODE_LANG5: SDL_Scancode = 148;
pub const SDL_SCANCODE_LANG4: SDL_Scancode = 147;
pub const SDL_SCANCODE_LANG3: SDL_Scancode = 146;
pub const SDL_SCANCODE_LANG2: SDL_Scancode = 145;
pub const SDL_SCANCODE_LANG1: SDL_Scancode = 144;
pub const SDL_SCANCODE_INTERNATIONAL9: SDL_Scancode = 143;
pub const SDL_SCANCODE_INTERNATIONAL8: SDL_Scancode = 142;
pub const SDL_SCANCODE_INTERNATIONAL7: SDL_Scancode = 141;
pub const SDL_SCANCODE_INTERNATIONAL6: SDL_Scancode = 140;
pub const SDL_SCANCODE_INTERNATIONAL5: SDL_Scancode = 139;
pub const SDL_SCANCODE_INTERNATIONAL4: SDL_Scancode = 138;
pub const SDL_SCANCODE_INTERNATIONAL3: SDL_Scancode = 137;
pub const SDL_SCANCODE_INTERNATIONAL2: SDL_Scancode = 136;
pub const SDL_SCANCODE_INTERNATIONAL1: SDL_Scancode = 135;
pub const SDL_SCANCODE_KP_EQUALSAS400: SDL_Scancode = 134;
pub const SDL_SCANCODE_KP_COMMA: SDL_Scancode = 133;
pub const SDL_SCANCODE_VOLUMEDOWN: SDL_Scancode = 129;
pub const SDL_SCANCODE_VOLUMEUP: SDL_Scancode = 128;
pub const SDL_SCANCODE_MUTE: SDL_Scancode = 127;
pub const SDL_SCANCODE_FIND: SDL_Scancode = 126;
pub const SDL_SCANCODE_PASTE: SDL_Scancode = 125;
pub const SDL_SCANCODE_COPY: SDL_Scancode = 124;
pub const SDL_SCANCODE_CUT: SDL_Scancode = 123;
pub const SDL_SCANCODE_UNDO: SDL_Scancode = 122;
pub const SDL_SCANCODE_AGAIN: SDL_Scancode = 121;
pub const SDL_SCANCODE_STOP: SDL_Scancode = 120;
pub const SDL_SCANCODE_SELECT: SDL_Scancode = 119;
pub const SDL_SCANCODE_MENU: SDL_Scancode = 118;
pub const SDL_SCANCODE_HELP: SDL_Scancode = 117;
pub const SDL_SCANCODE_EXECUTE: SDL_Scancode = 116;
pub const SDL_SCANCODE_F24: SDL_Scancode = 115;
pub const SDL_SCANCODE_F23: SDL_Scancode = 114;
pub const SDL_SCANCODE_F22: SDL_Scancode = 113;
pub const SDL_SCANCODE_F21: SDL_Scancode = 112;
pub const SDL_SCANCODE_F20: SDL_Scancode = 111;
pub const SDL_SCANCODE_F19: SDL_Scancode = 110;
pub const SDL_SCANCODE_F18: SDL_Scancode = 109;
pub const SDL_SCANCODE_F17: SDL_Scancode = 108;
pub const SDL_SCANCODE_F16: SDL_Scancode = 107;
pub const SDL_SCANCODE_F15: SDL_Scancode = 106;
pub const SDL_SCANCODE_F14: SDL_Scancode = 105;
pub const SDL_SCANCODE_F13: SDL_Scancode = 104;
pub const SDL_SCANCODE_KP_EQUALS: SDL_Scancode = 103;
pub const SDL_SCANCODE_POWER: SDL_Scancode = 102;
pub const SDL_SCANCODE_APPLICATION: SDL_Scancode = 101;
pub const SDL_SCANCODE_NONUSBACKSLASH: SDL_Scancode = 100;
pub const SDL_SCANCODE_KP_PERIOD: SDL_Scancode = 99;
pub const SDL_SCANCODE_KP_0: SDL_Scancode = 98;
pub const SDL_SCANCODE_KP_9: SDL_Scancode = 97;
pub const SDL_SCANCODE_KP_8: SDL_Scancode = 96;
pub const SDL_SCANCODE_KP_7: SDL_Scancode = 95;
pub const SDL_SCANCODE_KP_6: SDL_Scancode = 94;
pub const SDL_SCANCODE_KP_5: SDL_Scancode = 93;
pub const SDL_SCANCODE_KP_4: SDL_Scancode = 92;
pub const SDL_SCANCODE_KP_3: SDL_Scancode = 91;
pub const SDL_SCANCODE_KP_2: SDL_Scancode = 90;
pub const SDL_SCANCODE_KP_1: SDL_Scancode = 89;
pub const SDL_SCANCODE_KP_ENTER: SDL_Scancode = 88;
pub const SDL_SCANCODE_KP_PLUS: SDL_Scancode = 87;
pub const SDL_SCANCODE_KP_MINUS: SDL_Scancode = 86;
pub const SDL_SCANCODE_KP_MULTIPLY: SDL_Scancode = 85;
pub const SDL_SCANCODE_KP_DIVIDE: SDL_Scancode = 84;
pub const SDL_SCANCODE_NUMLOCKCLEAR: SDL_Scancode = 83;
pub const SDL_SCANCODE_UP: SDL_Scancode = 82;
pub const SDL_SCANCODE_DOWN: SDL_Scancode = 81;
pub const SDL_SCANCODE_LEFT: SDL_Scancode = 80;
pub const SDL_SCANCODE_RIGHT: SDL_Scancode = 79;
pub const SDL_SCANCODE_PAGEDOWN: SDL_Scancode = 78;
pub const SDL_SCANCODE_END: SDL_Scancode = 77;
pub const SDL_SCANCODE_DELETE: SDL_Scancode = 76;
pub const SDL_SCANCODE_PAGEUP: SDL_Scancode = 75;
pub const SDL_SCANCODE_HOME: SDL_Scancode = 74;
pub const SDL_SCANCODE_INSERT: SDL_Scancode = 73;
pub const SDL_SCANCODE_PAUSE: SDL_Scancode = 72;
pub const SDL_SCANCODE_SCROLLLOCK: SDL_Scancode = 71;
pub const SDL_SCANCODE_PRINTSCREEN: SDL_Scancode = 70;
pub const SDL_SCANCODE_F12: SDL_Scancode = 69;
pub const SDL_SCANCODE_F11: SDL_Scancode = 68;
pub const SDL_SCANCODE_F10: SDL_Scancode = 67;
pub const SDL_SCANCODE_F9: SDL_Scancode = 66;
pub const SDL_SCANCODE_F8: SDL_Scancode = 65;
pub const SDL_SCANCODE_F7: SDL_Scancode = 64;
pub const SDL_SCANCODE_F6: SDL_Scancode = 63;
pub const SDL_SCANCODE_F5: SDL_Scancode = 62;
pub const SDL_SCANCODE_F4: SDL_Scancode = 61;
pub const SDL_SCANCODE_F3: SDL_Scancode = 60;
pub const SDL_SCANCODE_F2: SDL_Scancode = 59;
pub const SDL_SCANCODE_F1: SDL_Scancode = 58;
pub const SDL_SCANCODE_CAPSLOCK: SDL_Scancode = 57;
pub const SDL_SCANCODE_SLASH: SDL_Scancode = 56;
pub const SDL_SCANCODE_PERIOD: SDL_Scancode = 55;
pub const SDL_SCANCODE_COMMA: SDL_Scancode = 54;
pub const SDL_SCANCODE_GRAVE: SDL_Scancode = 53;
pub const SDL_SCANCODE_APOSTROPHE: SDL_Scancode = 52;
pub const SDL_SCANCODE_SEMICOLON: SDL_Scancode = 51;
pub const SDL_SCANCODE_NONUSHASH: SDL_Scancode = 50;
pub const SDL_SCANCODE_BACKSLASH: SDL_Scancode = 49;
pub const SDL_SCANCODE_RIGHTBRACKET: SDL_Scancode = 48;
pub const SDL_SCANCODE_LEFTBRACKET: SDL_Scancode = 47;
pub const SDL_SCANCODE_EQUALS: SDL_Scancode = 46;
pub const SDL_SCANCODE_MINUS: SDL_Scancode = 45;
pub const SDL_SCANCODE_SPACE: SDL_Scancode = 44;
pub const SDL_SCANCODE_TAB: SDL_Scancode = 43;
pub const SDL_SCANCODE_BACKSPACE: SDL_Scancode = 42;
pub const SDL_SCANCODE_ESCAPE: SDL_Scancode = 41;
pub const SDL_SCANCODE_RETURN: SDL_Scancode = 40;
pub const SDL_SCANCODE_0: SDL_Scancode = 39;
pub const SDL_SCANCODE_9: SDL_Scancode = 38;
pub const SDL_SCANCODE_8: SDL_Scancode = 37;
pub const SDL_SCANCODE_7: SDL_Scancode = 36;
pub const SDL_SCANCODE_6: SDL_Scancode = 35;
pub const SDL_SCANCODE_5: SDL_Scancode = 34;
pub const SDL_SCANCODE_4: SDL_Scancode = 33;
pub const SDL_SCANCODE_3: SDL_Scancode = 32;
pub const SDL_SCANCODE_2: SDL_Scancode = 31;
pub const SDL_SCANCODE_1: SDL_Scancode = 30;
pub const SDL_SCANCODE_Z: SDL_Scancode = 29;
pub const SDL_SCANCODE_Y: SDL_Scancode = 28;
pub const SDL_SCANCODE_X: SDL_Scancode = 27;
pub const SDL_SCANCODE_W: SDL_Scancode = 26;
pub const SDL_SCANCODE_V: SDL_Scancode = 25;
pub const SDL_SCANCODE_U: SDL_Scancode = 24;
pub const SDL_SCANCODE_T: SDL_Scancode = 23;
pub const SDL_SCANCODE_S: SDL_Scancode = 22;
pub const SDL_SCANCODE_R: SDL_Scancode = 21;
pub const SDL_SCANCODE_Q: SDL_Scancode = 20;
pub const SDL_SCANCODE_P: SDL_Scancode = 19;
pub const SDL_SCANCODE_O: SDL_Scancode = 18;
pub const SDL_SCANCODE_N: SDL_Scancode = 17;
pub const SDL_SCANCODE_M: SDL_Scancode = 16;
pub const SDL_SCANCODE_L: SDL_Scancode = 15;
pub const SDL_SCANCODE_K: SDL_Scancode = 14;
pub const SDL_SCANCODE_J: SDL_Scancode = 13;
pub const SDL_SCANCODE_I: SDL_Scancode = 12;
pub const SDL_SCANCODE_H: SDL_Scancode = 11;
pub const SDL_SCANCODE_G: SDL_Scancode = 10;
pub const SDL_SCANCODE_F: SDL_Scancode = 9;
pub const SDL_SCANCODE_E: SDL_Scancode = 8;
pub const SDL_SCANCODE_D: SDL_Scancode = 7;
pub const SDL_SCANCODE_C: SDL_Scancode = 6;
pub const SDL_SCANCODE_B: SDL_Scancode = 5;
pub const SDL_SCANCODE_A: SDL_Scancode = 4;
pub const SDL_SCANCODE_UNKNOWN: SDL_Scancode = 0;
pub type SDL_Keycode = Sint32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_Keysym {
    pub scancode: SDL_Scancode,
    pub sym: SDL_Keycode,
    pub mod_0: Uint16,
    pub unused: Uint32,
}
pub type SDL_JoystickID = Sint32;
pub type SDL_TouchID = Sint64;
pub type SDL_FingerID = Sint64;
pub type SDL_GestureID = Sint64;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const SDL_LASTEVENT: C2RustUnnamed_0 = 65535;
pub const SDL_USEREVENT: C2RustUnnamed_0 = 32768;
pub const SDL_POLLSENTINEL: C2RustUnnamed_0 = 32512;
pub const SDL_RENDER_DEVICE_RESET: C2RustUnnamed_0 = 8193;
pub const SDL_RENDER_TARGETS_RESET: C2RustUnnamed_0 = 8192;
pub const SDL_SENSORUPDATE: C2RustUnnamed_0 = 4608;
pub const SDL_AUDIODEVICEREMOVED: C2RustUnnamed_0 = 4353;
pub const SDL_AUDIODEVICEADDED: C2RustUnnamed_0 = 4352;
pub const SDL_DROPCOMPLETE: C2RustUnnamed_0 = 4099;
pub const SDL_DROPBEGIN: C2RustUnnamed_0 = 4098;
pub const SDL_DROPTEXT: C2RustUnnamed_0 = 4097;
pub const SDL_DROPFILE: C2RustUnnamed_0 = 4096;
pub const SDL_CLIPBOARDUPDATE: C2RustUnnamed_0 = 2304;
pub const SDL_MULTIGESTURE: C2RustUnnamed_0 = 2050;
pub const SDL_DOLLARRECORD: C2RustUnnamed_0 = 2049;
pub const SDL_DOLLARGESTURE: C2RustUnnamed_0 = 2048;
pub const SDL_FINGERMOTION: C2RustUnnamed_0 = 1794;
pub const SDL_FINGERUP: C2RustUnnamed_0 = 1793;
pub const SDL_FINGERDOWN: C2RustUnnamed_0 = 1792;
pub const SDL_CONTROLLERSENSORUPDATE: C2RustUnnamed_0 = 1625;
pub const SDL_CONTROLLERTOUCHPADUP: C2RustUnnamed_0 = 1624;
pub const SDL_CONTROLLERTOUCHPADMOTION: C2RustUnnamed_0 = 1623;
pub const SDL_CONTROLLERTOUCHPADDOWN: C2RustUnnamed_0 = 1622;
pub const SDL_CONTROLLERDEVICEREMAPPED: C2RustUnnamed_0 = 1621;
pub const SDL_CONTROLLERDEVICEREMOVED: C2RustUnnamed_0 = 1620;
pub const SDL_CONTROLLERDEVICEADDED: C2RustUnnamed_0 = 1619;
pub const SDL_CONTROLLERBUTTONUP: C2RustUnnamed_0 = 1618;
pub const SDL_CONTROLLERBUTTONDOWN: C2RustUnnamed_0 = 1617;
pub const SDL_CONTROLLERAXISMOTION: C2RustUnnamed_0 = 1616;
pub const SDL_JOYDEVICEREMOVED: C2RustUnnamed_0 = 1542;
pub const SDL_JOYDEVICEADDED: C2RustUnnamed_0 = 1541;
pub const SDL_JOYBUTTONUP: C2RustUnnamed_0 = 1540;
pub const SDL_JOYBUTTONDOWN: C2RustUnnamed_0 = 1539;
pub const SDL_JOYHATMOTION: C2RustUnnamed_0 = 1538;
pub const SDL_JOYBALLMOTION: C2RustUnnamed_0 = 1537;
pub const SDL_JOYAXISMOTION: C2RustUnnamed_0 = 1536;
pub const SDL_MOUSEWHEEL: C2RustUnnamed_0 = 1027;
pub const SDL_MOUSEBUTTONUP: C2RustUnnamed_0 = 1026;
pub const SDL_MOUSEBUTTONDOWN: C2RustUnnamed_0 = 1025;
pub const SDL_MOUSEMOTION: C2RustUnnamed_0 = 1024;
pub const SDL_KEYMAPCHANGED: C2RustUnnamed_0 = 772;
pub const SDL_TEXTINPUT: C2RustUnnamed_0 = 771;
pub const SDL_TEXTEDITING: C2RustUnnamed_0 = 770;
pub const SDL_KEYUP: C2RustUnnamed_0 = 769;
pub const SDL_KEYDOWN: C2RustUnnamed_0 = 768;
pub const SDL_SYSWMEVENT: C2RustUnnamed_0 = 513;
pub const SDL_WINDOWEVENT: C2RustUnnamed_0 = 512;
pub const SDL_DISPLAYEVENT: C2RustUnnamed_0 = 336;
pub const SDL_LOCALECHANGED: C2RustUnnamed_0 = 263;
pub const SDL_APP_DIDENTERFOREGROUND: C2RustUnnamed_0 = 262;
pub const SDL_APP_WILLENTERFOREGROUND: C2RustUnnamed_0 = 261;
pub const SDL_APP_DIDENTERBACKGROUND: C2RustUnnamed_0 = 260;
pub const SDL_APP_WILLENTERBACKGROUND: C2RustUnnamed_0 = 259;
pub const SDL_APP_LOWMEMORY: C2RustUnnamed_0 = 258;
pub const SDL_APP_TERMINATING: C2RustUnnamed_0 = 257;
pub const SDL_QUIT: C2RustUnnamed_0 = 256;
pub const SDL_FIRSTEVENT: C2RustUnnamed_0 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_CommonEvent {
    pub type_0: Uint32,
    pub timestamp: Uint32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_DisplayEvent {
    pub type_0: Uint32,
    pub timestamp: Uint32,
    pub display: Uint32,
    pub event: Uint8,
    pub padding1: Uint8,
    pub padding2: Uint8,
    pub padding3: Uint8,
    pub data1: Sint32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_WindowEvent {
    pub type_0: Uint32,
    pub timestamp: Uint32,
    pub windowID: Uint32,
    pub event: Uint8,
    pub padding1: Uint8,
    pub padding2: Uint8,
    pub padding3: Uint8,
    pub data1: Sint32,
    pub data2: Sint32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_KeyboardEvent {
    pub type_0: Uint32,
    pub timestamp: Uint32,
    pub windowID: Uint32,
    pub state: Uint8,
    pub repeat: Uint8,
    pub padding2: Uint8,
    pub padding3: Uint8,
    pub keysym: SDL_Keysym,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_TextEditingEvent {
    pub type_0: Uint32,
    pub timestamp: Uint32,
    pub windowID: Uint32,
    pub text: [libc::c_char; 32],
    pub start: Sint32,
    pub length: Sint32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_TextInputEvent {
    pub type_0: Uint32,
    pub timestamp: Uint32,
    pub windowID: Uint32,
    pub text: [libc::c_char; 32],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_MouseMotionEvent {
    pub type_0: Uint32,
    pub timestamp: Uint32,
    pub windowID: Uint32,
    pub which: Uint32,
    pub state: Uint32,
    pub x: Sint32,
    pub y: Sint32,
    pub xrel: Sint32,
    pub yrel: Sint32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_MouseButtonEvent {
    pub type_0: Uint32,
    pub timestamp: Uint32,
    pub windowID: Uint32,
    pub which: Uint32,
    pub button: Uint8,
    pub state: Uint8,
    pub clicks: Uint8,
    pub padding1: Uint8,
    pub x: Sint32,
    pub y: Sint32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_MouseWheelEvent {
    pub type_0: Uint32,
    pub timestamp: Uint32,
    pub windowID: Uint32,
    pub which: Uint32,
    pub x: Sint32,
    pub y: Sint32,
    pub direction: Uint32,
    pub preciseX: libc::c_float,
    pub preciseY: libc::c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_JoyAxisEvent {
    pub type_0: Uint32,
    pub timestamp: Uint32,
    pub which: SDL_JoystickID,
    pub axis: Uint8,
    pub padding1: Uint8,
    pub padding2: Uint8,
    pub padding3: Uint8,
    pub value: Sint16,
    pub padding4: Uint16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_JoyBallEvent {
    pub type_0: Uint32,
    pub timestamp: Uint32,
    pub which: SDL_JoystickID,
    pub ball: Uint8,
    pub padding1: Uint8,
    pub padding2: Uint8,
    pub padding3: Uint8,
    pub xrel: Sint16,
    pub yrel: Sint16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_JoyHatEvent {
    pub type_0: Uint32,
    pub timestamp: Uint32,
    pub which: SDL_JoystickID,
    pub hat: Uint8,
    pub value: Uint8,
    pub padding1: Uint8,
    pub padding2: Uint8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_JoyButtonEvent {
    pub type_0: Uint32,
    pub timestamp: Uint32,
    pub which: SDL_JoystickID,
    pub button: Uint8,
    pub state: Uint8,
    pub padding1: Uint8,
    pub padding2: Uint8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_JoyDeviceEvent {
    pub type_0: Uint32,
    pub timestamp: Uint32,
    pub which: Sint32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_ControllerAxisEvent {
    pub type_0: Uint32,
    pub timestamp: Uint32,
    pub which: SDL_JoystickID,
    pub axis: Uint8,
    pub padding1: Uint8,
    pub padding2: Uint8,
    pub padding3: Uint8,
    pub value: Sint16,
    pub padding4: Uint16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_ControllerButtonEvent {
    pub type_0: Uint32,
    pub timestamp: Uint32,
    pub which: SDL_JoystickID,
    pub button: Uint8,
    pub state: Uint8,
    pub padding1: Uint8,
    pub padding2: Uint8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_ControllerDeviceEvent {
    pub type_0: Uint32,
    pub timestamp: Uint32,
    pub which: Sint32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_ControllerTouchpadEvent {
    pub type_0: Uint32,
    pub timestamp: Uint32,
    pub which: SDL_JoystickID,
    pub touchpad: Sint32,
    pub finger: Sint32,
    pub x: libc::c_float,
    pub y: libc::c_float,
    pub pressure: libc::c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_ControllerSensorEvent {
    pub type_0: Uint32,
    pub timestamp: Uint32,
    pub which: SDL_JoystickID,
    pub sensor: Sint32,
    pub data: [libc::c_float; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_AudioDeviceEvent {
    pub type_0: Uint32,
    pub timestamp: Uint32,
    pub which: Uint32,
    pub iscapture: Uint8,
    pub padding1: Uint8,
    pub padding2: Uint8,
    pub padding3: Uint8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_TouchFingerEvent {
    pub type_0: Uint32,
    pub timestamp: Uint32,
    pub touchId: SDL_TouchID,
    pub fingerId: SDL_FingerID,
    pub x: libc::c_float,
    pub y: libc::c_float,
    pub dx: libc::c_float,
    pub dy: libc::c_float,
    pub pressure: libc::c_float,
    pub windowID: Uint32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_MultiGestureEvent {
    pub type_0: Uint32,
    pub timestamp: Uint32,
    pub touchId: SDL_TouchID,
    pub dTheta: libc::c_float,
    pub dDist: libc::c_float,
    pub x: libc::c_float,
    pub y: libc::c_float,
    pub numFingers: Uint16,
    pub padding: Uint16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_DollarGestureEvent {
    pub type_0: Uint32,
    pub timestamp: Uint32,
    pub touchId: SDL_TouchID,
    pub gestureId: SDL_GestureID,
    pub numFingers: Uint32,
    pub error: libc::c_float,
    pub x: libc::c_float,
    pub y: libc::c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_DropEvent {
    pub type_0: Uint32,
    pub timestamp: Uint32,
    pub file: *mut libc::c_char,
    pub windowID: Uint32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_SensorEvent {
    pub type_0: Uint32,
    pub timestamp: Uint32,
    pub which: Sint32,
    pub data: [libc::c_float; 6],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_QuitEvent {
    pub type_0: Uint32,
    pub timestamp: Uint32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_UserEvent {
    pub type_0: Uint32,
    pub timestamp: Uint32,
    pub windowID: Uint32,
    pub code: Sint32,
    pub data1: *mut libc::c_void,
    pub data2: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_SysWMEvent {
    pub type_0: Uint32,
    pub timestamp: Uint32,
    pub msg: *mut SDL_SysWMmsg,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union SDL_Event {
    pub type_0: Uint32,
    pub common: SDL_CommonEvent,
    pub display: SDL_DisplayEvent,
    pub window: SDL_WindowEvent,
    pub key: SDL_KeyboardEvent,
    pub edit: SDL_TextEditingEvent,
    pub text: SDL_TextInputEvent,
    pub motion: SDL_MouseMotionEvent,
    pub button: SDL_MouseButtonEvent,
    pub wheel: SDL_MouseWheelEvent,
    pub jaxis: SDL_JoyAxisEvent,
    pub jball: SDL_JoyBallEvent,
    pub jhat: SDL_JoyHatEvent,
    pub jbutton: SDL_JoyButtonEvent,
    pub jdevice: SDL_JoyDeviceEvent,
    pub caxis: SDL_ControllerAxisEvent,
    pub cbutton: SDL_ControllerButtonEvent,
    pub cdevice: SDL_ControllerDeviceEvent,
    pub ctouchpad: SDL_ControllerTouchpadEvent,
    pub csensor: SDL_ControllerSensorEvent,
    pub adevice: SDL_AudioDeviceEvent,
    pub sensor: SDL_SensorEvent,
    pub quit: SDL_QuitEvent,
    pub user: SDL_UserEvent,
    pub syswm: SDL_SysWMEvent,
    pub tfinger: SDL_TouchFingerEvent,
    pub mgesture: SDL_MultiGestureEvent,
    pub dgesture: SDL_DollarGestureEvent,
    pub drop: SDL_DropEvent,
    pub padding: [Uint8; 56],
}
#[no_mangle]
pub unsafe extern "C" fn log2i(mut x: libc::c_int) -> libc::c_int {
    let mut r: libc::c_int = 0 as libc::c_int;
    loop {
        x >>= 1 as libc::c_int;
        if !(x != 0) {
            break;
        }
        r += 1;
    }
    return r;
}
#[no_mangle]
pub unsafe extern "C" fn printerr(mut fmt: *const libc::c_char, mut args: ...) {
    let mut msg: [libc::c_char; 1000] = [0; 1000];
    let mut argp: ::core::ffi::VaListImpl;
    argp = args.clone();
    vsnprintf(
        msg.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 1000]>() as libc::c_ulong,
        fmt,
        argp.as_va_list(),
    );
    fprintf(
        stderr,
        b"error: %s\n\0" as *const u8 as *const libc::c_char,
        msg.as_mut_ptr(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn printwarn(mut fmt: *const libc::c_char, mut args: ...) {
    let mut msg: [libc::c_char; 1000] = [0; 1000];
    let mut argp: ::core::ffi::VaListImpl;
    argp = args.clone();
    vsnprintf(
        msg.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 1000]>() as libc::c_ulong,
        fmt,
        argp.as_va_list(),
    );
    fprintf(
        stderr,
        b"warning: %s\n\0" as *const u8 as *const libc::c_char,
        msg.as_mut_ptr(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn freaderror(
    mut file: *mut FILE,
    mut path: *const libc::c_char,
    mut size: libc::c_int,
    mut type_0: *const libc::c_char,
) {
    if feof(file) != 0 {
        printerr(
            b"invalid BSAVE %s, PIC must have %d bytes: %s\0" as *const u8
                as *const libc::c_char,
            type_0,
            size,
            path,
        );
    } else {
        printerr(
            b"could not read file: %s\n\0" as *const u8 as *const libc::c_char,
            path,
        );
    }
    fclose(file);
}
#[no_mangle]
pub unsafe extern "C" fn epyx_yeah(mut path: *const libc::c_char) -> libc::c_int {
    let CGA_WIDTH: libc::c_int = 320 as libc::c_int;
    let CGA_HEIGHT: libc::c_int = 200 as libc::c_int;
    let CGA_BG_COLOR: libc::c_int = 0 as libc::c_int;
    let CGA_FIELDS: libc::c_int = 2 as libc::c_int;
    let CGA_PADDING: libc::c_int = 192 as libc::c_int;
    let CGA_COLORS: [[libc::c_int; 3]; 4] = [
        [0 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int],
        [85 as libc::c_int, 255 as libc::c_int, 255 as libc::c_int],
        [255 as libc::c_int, 85 as libc::c_int, 255 as libc::c_int],
        [255 as libc::c_int, 255 as libc::c_int, 255 as libc::c_int],
    ];
    let CGA_NUM_COLORS: libc::c_int = (::core::mem::size_of::<[[libc::c_int; 3]; 4]>()
        as libc::c_ulong)
        .wrapping_div(::core::mem::size_of::<[libc::c_int; 3]>() as libc::c_ulong)
        as libc::c_int;
    let CGA_BIT_DEPTH: libc::c_int = log2i(CGA_NUM_COLORS);
    let CGA_PPB: libc::c_int = 8 as libc::c_int / CGA_BIT_DEPTH;
    let CGA_DATA_SIZE: libc::c_int = CGA_WIDTH * CGA_HEIGHT / CGA_PPB;
    let CGA_SIZE: libc::c_int = CGA_DATA_SIZE + CGA_FIELDS * CGA_PADDING;
    let BSAVE_HEADER: [libc::c_uchar; 7] = [
        -3i32 as libc::c_uchar,
        '\0' as i32 as libc::c_uchar,
        -72i32 as libc::c_uchar,
        '\0' as i32 as libc::c_uchar,
        '\0' as i32 as libc::c_uchar,
        '\0' as i32 as libc::c_uchar,
        '@' as i32 as libc::c_uchar,
    ];
    let PIC_SIG: *const libc::c_char = b"PCPaint V1.0\0" as *const u8
        as *const libc::c_char;
    let SIG_OFFSET: libc::c_int = CGA_DATA_SIZE / CGA_FIELDS;
    let BSAVE_SIZE: libc::c_int = (::core::mem::size_of::<[libc::c_uchar; 7]>()
        as libc::c_ulong)
        .wrapping_add(CGA_SIZE as libc::c_ulong) as libc::c_int;
    let vla = CGA_SIZE as usize;
    let mut data: Vec::<libc::c_uchar> = ::std::vec::from_elem(0, vla);
    let mut file: *mut FILE = 0 as *mut FILE;
    let mut window: *mut SDL_Window = 0 as *mut SDL_Window;
    let mut renderer: *mut SDL_Renderer = 0 as *mut SDL_Renderer;
    let mut event: SDL_Event = SDL_Event { type_0: 0 };
    file = fopen(path, b"rb\0" as *const u8 as *const libc::c_char);
    if file.is_null() {
        printerr(
            b"%s: %s\0" as *const u8 as *const libc::c_char,
            strerror(*__errno_location()),
            path,
        );
        return 0 as libc::c_int;
    }
    if fread(
        data.as_mut_ptr() as *mut libc::c_void,
        ::core::mem::size_of::<[libc::c_uchar; 7]>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        file,
    ) == 0
    {
        freaderror(
            file,
            path,
            BSAVE_SIZE,
            b"header\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if memcmp(
        data.as_mut_ptr() as *const libc::c_void,
        BSAVE_HEADER.as_ptr() as *const libc::c_void,
        ::core::mem::size_of::<[libc::c_uchar; 7]>() as libc::c_ulong,
    ) != 0
    {
        printwarn(
            b"invalid header, possibly not a valid PIC image in BSAVE format: %s\0"
                as *const u8 as *const libc::c_char,
            path,
        );
    }
    if fread(
        data.as_mut_ptr() as *mut libc::c_void,
        (vla * ::core::mem::size_of::<libc::c_uchar>()) as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        file,
    ) == 0
    {
        freaderror(
            file,
            path,
            BSAVE_SIZE,
            b"data\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if fgetc(file) != -(1 as libc::c_int) {
        printwarn(
            b"file is larger than %d bytes, possibly not a valid PIC image in BSAVE format: %s\0"
                as *const u8 as *const libc::c_char,
            BSAVE_SIZE,
            path,
        );
    }
    fclose(file);
    if strncmp(
        PIC_SIG,
        &mut *data.as_mut_ptr().offset(SIG_OFFSET as isize) as *mut libc::c_uchar
            as *mut libc::c_char,
        strlen(PIC_SIG) as libc::c_int as libc::c_ulong,
    ) != 0 as libc::c_int
    {
        printwarn(
            b"invalid PIC signature at offset 0x%X, expected '%s' in: %s\0" as *const u8
                as *const libc::c_char,
            (::core::mem::size_of::<[libc::c_uchar; 7]>() as libc::c_ulong)
                .wrapping_add(SIG_OFFSET as libc::c_ulong),
            PIC_SIG,
            path,
        );
    }
    if SDL_Init(0x20 as libc::c_uint) != 0 as libc::c_int
        || SDL_SetHint(
            b"SDL_RENDER_SCALE_QUALITY\0" as *const u8 as *const libc::c_char,
            b"linear\0" as *const u8 as *const libc::c_char,
        ) as libc::c_uint != SDL_TRUE as libc::c_int as libc::c_uint
        || SDL_CreateWindowAndRenderer(
            0 as libc::c_int,
            0 as libc::c_int,
            SDL_WINDOW_FULLSCREEN_DESKTOP as libc::c_int as Uint32,
            &mut window,
            &mut renderer,
        ) != 0 as libc::c_int
    {
        SDL_Quit();
        printerr(
            b"could not initialize SDL: %s\0" as *const u8 as *const libc::c_char,
            SDL_GetError(),
        );
        return 0 as libc::c_int;
    }
    SDL_RenderSetLogicalSize(renderer, CGA_WIDTH, CGA_HEIGHT);
    if CGA_BG_COLOR >= 0 as libc::c_int && CGA_BG_COLOR < CGA_NUM_COLORS {} else {
        __assert_fail(
            b"CGA_BG_COLOR >= 0 && CGA_BG_COLOR < CGA_NUM_COLORS\0" as *const u8
                as *const libc::c_char,
            b"load_sdl.c\0" as *const u8 as *const libc::c_char,
            141 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 28],
                &[libc::c_char; 28],
            >(b"int epyx_yeah(const char *)\0"))
                .as_ptr(),
        );
    };
    SDL_SetRenderDrawColor(
        renderer,
        CGA_COLORS[CGA_BG_COLOR as usize][0 as libc::c_int as usize] as Uint8,
        CGA_COLORS[CGA_BG_COLOR as usize][1 as libc::c_int as usize] as Uint8,
        CGA_COLORS[CGA_BG_COLOR as usize][2 as libc::c_int as usize] as Uint8,
        255 as libc::c_int as Uint8,
    );
    SDL_RenderClear(renderer);
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut field: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut p: libc::c_int = 0;
    let mut c: libc::c_uchar = 0;
    let mut prevc: libc::c_uchar = CGA_BG_COLOR as libc::c_uchar;
    field = 0 as libc::c_int;
    while field < CGA_FIELDS {
        y = 0 as libc::c_int;
        while y < CGA_HEIGHT {
            x = 0 as libc::c_int;
            while x < CGA_WIDTH {
                if i < CGA_SIZE {} else {
                    __assert_fail(
                        b"i < CGA_SIZE\0" as *const u8 as *const libc::c_char,
                        b"load_sdl.c\0" as *const u8 as *const libc::c_char,
                        161 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 28],
                            &[libc::c_char; 28],
                        >(b"int epyx_yeah(const char *)\0"))
                            .as_ptr(),
                    );
                };
                p = 0 as libc::c_int;
                while p < CGA_PPB {
                    c = (*data.as_mut_ptr().offset(i as isize) as libc::c_int
                        >> (CGA_PPB - p - 1 as libc::c_int) * CGA_BIT_DEPTH
                        & CGA_NUM_COLORS - 1 as libc::c_int) as libc::c_uchar;
                    if (c as libc::c_int) < CGA_NUM_COLORS {} else {
                        __assert_fail(
                            b"c < CGA_NUM_COLORS\0" as *const u8 as *const libc::c_char,
                            b"load_sdl.c\0" as *const u8 as *const libc::c_char,
                            167 as libc::c_int as libc::c_uint,
                            (*::core::mem::transmute::<
                                &[u8; 28],
                                &[libc::c_char; 28],
                            >(b"int epyx_yeah(const char *)\0"))
                                .as_ptr(),
                        );
                    };
                    if c as libc::c_int != prevc as libc::c_int {
                        SDL_SetRenderDrawColor(
                            renderer,
                            CGA_COLORS[c as usize][0 as libc::c_int as usize] as Uint8,
                            CGA_COLORS[c as usize][1 as libc::c_int as usize] as Uint8,
                            CGA_COLORS[c as usize][2 as libc::c_int as usize] as Uint8,
                            255 as libc::c_int as Uint8,
                        );
                        prevc = c;
                    }
                    SDL_RenderDrawPoint(renderer, x + p, y + field);
                    p += 1;
                }
                x += CGA_PPB;
                i += 1;
            }
            y += CGA_FIELDS;
        }
        field += 1;
        i += CGA_PADDING;
    }
    if i == CGA_SIZE {} else {
        __assert_fail(
            b"i == CGA_SIZE\0" as *const u8 as *const libc::c_char,
            b"load_sdl.c\0" as *const u8 as *const libc::c_char,
            185 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 28],
                &[libc::c_char; 28],
            >(b"int epyx_yeah(const char *)\0"))
                .as_ptr(),
        );
    };
    SDL_RenderPresent(renderer);
    while SDL_GetTicks()
        < (1000 as libc::c_int * 60 as libc::c_int * 5 as libc::c_int) as libc::c_uint
    {
        if SDL_PollEvent(&mut event) != 0
            && (event.type_0 == SDL_QUIT as libc::c_int as libc::c_uint
                || event.type_0 == SDL_KEYUP as libc::c_int as libc::c_uint
                || event.type_0 == SDL_MOUSEBUTTONUP as libc::c_int as libc::c_uint)
        {
            break;
        }
        SDL_Delay(16 as libc::c_int as Uint32);
    }
    SDL_DestroyRenderer(renderer);
    SDL_DestroyWindow(window);
    SDL_Quit();
    return 1 as libc::c_int;
}
