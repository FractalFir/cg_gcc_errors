#![recursion_limit = "1024"]
#![feature(custom_mir, core_intrinsics, lazy_get)]
#![allow(unused_parens, unused_assignments, overflowing_literals)]
extern crate core;
use core::intrinsics::mir::*;
use std::fmt::Debug;
#[inline(never)]
fn dump_var(
    f: usize,
    var0: usize,
    val0: impl Debug,
    var1: usize,
    val1: impl Debug,
    var2: usize,
    val2: impl Debug,
    var3: usize,
    val3: impl Debug,
) {
    println!(
        "fn{f}:_{var0} = {val0:?}\n_{var1} = {val1:?}\n_{var2} = {val2:?}\n_{var3} = {val3:?}"
    );
}

#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn2(
    mut _1: *const isize,
    mut _2: i16,
    mut _3: i32,
    mut _4: u32,
    mut _5: u32,
    mut _6: isize,
    mut _7: i8,
    mut _8: isize,
    mut _9: u32,
    mut _10: bool,
) -> isize {
    mir! {
    type RET = isize;
    let _11: u16;
    let _12: (f32, Adt19, u64);
    let _13: char;
    let _14: i128;
    let _15: u64;
    let _16: f64;
    let _17: (Adt38, *mut i8, f32, u16);
    let _18: &'static mut [u32; 7];
    let _19: usize;
    let _20: isize;
    let _21: &'static *mut &'static *mut f32;
    let _22: f32;
    let _23: isize;
    let _24: u32;
    let _25: bool;
    let _26: i16;
    let _27: isize;
    let _28: &'static &'static i32;
    let _29: isize;
    let _30: isize;
    let _31: (i16, &'static mut *const usize, *mut *mut u128);
    let _32: *const &'static *mut &'static *mut f32;
    let _33: ();
    let _34: ();
    {
    RET = _8 - _8;
    _4 = !_9;
    _2 = 27451_i16 ^ (-22932_i16);
    _1 = core::ptr::addr_of!(_8);
    Goto(bb1)
    }
    bb1 = {
    _9 = _4;
    (*_1) = _6;
    _11 = '\u{bc6e7}' as u16;
    RET = !(*_1);
    (*_1) = -RET;
    _10 = (*_1) > (*_1);
    _2 = (-32684_i16) << _8;
    (*_1) = _6 | RET;
    Call((*_1) = core::intrinsics::bswap(_6), ReturnTo(bb2), UnwindUnreachable())
    }
    bb2 = {
    (*_1) = !_6;
    (*_1) = !RET;
    (*_1) = _6;
    (*_1) = !RET;
    _4 = _5;
    (*_1) = _6;
    (*_1) = '\u{25263}' as isize;
    _13 = '\u{b0476}';
    (*_1) = RET | _6;
    (*_1) = RET;
    _12.0 = _11 as f32;
    match _5 {
    0 => bb3,
    1 => bb4,
    2 => bb5,
    3 => bb6,
    4 => bb7,
    5 => bb8,
    6 => bb9,
    1705677637 => bb11,
    _ => bb10
    }
    }
    bb3 = {
    _9 = _4;
    (*_1) = _6;
    _11 = '\u{bc6e7}' as u16;
    RET = !(*_1);
    (*_1) = -RET;
    _10 = (*_1) > (*_1);
    _2 = (-32684_i16) << _8;
    (*_1) = _6 | RET;
    Call((*_1) = core::intrinsics::bswap(_6), ReturnTo(bb2), UnwindUnreachable())
    }
    bb4 = {
    Return()
    }
    bb5 = {
    Return()
    }
    bb6 = {
    Return()
    }
    bb7 = {
    Return()
    }
    bb8 = {
    Return()
    }
    bb9 = {
    Return()
    }
    bb10 = {
    Return()
    }
    bb11 = {
    _7 = (-5965270870520108484_i64) as i8;
    _12.2 = (-164714684559875379919518575010379717139_i128) as u64;
    match _4 {
    0 => bb10,
    1 => bb2,
    2 => bb3,
    3 => bb4,
    4 => bb8,
    5 => bb6,
    1705677637 => bb12,
    _ => bb9
    }
    }
    bb12 = {
    (*_1) = RET & RET;
    _12.0 = 2259476867524834697_i64 as f32;
    (*_1) = _6 ^ _6;
    (*_1) = _6 + RET;
    _12.0 = _7 as f32;
    (*_1) = RET & RET;
    (*_1) = _6 - _6;
    (*_1) = _6 - RET;
    (*_1) = _6;
    (*_1) = RET + _6;
    _17.2 = _12.0;
    (*_1) = -RET;
    _17.0.fld0 = _10;
    _17.3 = !_11;
    (*_1) = RET;
    _12.1 = Adt19::Variant0 { fld0: _12.0 };
    (*_1) = -RET;
    _12.1 = Adt19::Variant1 { fld0: 5900416280192199214_i64,fld1: _17.3 };
    _12.0 = _17.2 - _17.2;
    _1 = core::ptr::addr_of!(_6);
    _1 = core::ptr::addr_of!((*_1));
    _4 = !_9;
    _8 = (*_1);
    _17.1 = core::ptr::addr_of_mut!(_7);
    _15 = _12.2 << (*_1);
    _16 = (-3256106608463376368_i64) as f64;
    (*_1) = RET ^ _8;
    (*_1) = RET >> _3;
    match _5 {
    0 => bb3,
    1705677637 => bb14,
    _ => bb13
    }
    }
    bb13 = {
    Return()
    }
    bb14 = {
    (*_1) = _8 * _8;
    (*_1) = !_8;
    _4 = _5;
    (*_1) = _7 as isize;
    _17.2 = _12.0 + _12.0;
    _12.0 = _17.2 * _17.2;
    _3 = (-2040533717_i32) | 1920924768_i32;
    _11 = _17.3;
    _1 = core::ptr::addr_of!(_6);
    _17.0.fld1 = _11 as u8;
    (*_1) = _8;
    (*_1) = 112574652318580291556652859739937967926_i128 as isize;
    Goto(bb15)
    }
    bb15 = {
    place!(Field::<i64>(Variant(_12.1, 1), 0)) = 5622074495717962247_i64 >> (*_1);
    (*_1) = !_8;
    (*_1) = _8;
    _12.2 = (*_1) as u64;
    (*_1) = _8 << _17.0.fld1;
    _12.0 = -_17.2;
    (*_1) = _8 + RET;
    _5 = _4 & _9;
    Call((*_1) = fn3(_17.0, Move(_17), Move(_1)), ReturnTo(bb16), UnwindUnreachable())
    }
    bb16 = {
    _9 = _5;
    _1 = core::ptr::addr_of!(_20);
    (*_1) = _6;
    _12.1 = Adt19::Variant0 { fld0: _12.0 };
    _17.0.fld0 = !_10;
    (*_1) = _13 as isize;
    _19 = !8709539860994046717_usize;
    (*_1) = _6;
    _12.1 = Adt19::Variant1 { fld0: 2102338319415284219_i64,fld1: _11 };
    _17.2 = _12.0 - _12.0;
    (*_1) = _6;
    (*_1) = _6;
    _14 = 158801104880259612327299785983295593779_i128 & 166022442718972847310586321733306049718_i128;
    _17.1 = core::ptr::addr_of_mut!(_7);
    _25 = _10;
    (*_1) = _6 & _6;
    _3 = (-1922871724_i32) * 995968542_i32;
    _17.2 = _11 as f32;
    (*_1) = _16 as isize;
    _27 = (*_1) << (*_1);
    match _4 {
    0 => bb13,
    1 => bb10,
    2 => bb3,
    3 => bb17,
    4 => bb18,
    5 => bb19,
    1705677637 => bb21,
    _ => bb20
    }
    }
    bb17 = {
    place!(Field::<i64>(Variant(_12.1, 1), 0)) = 5622074495717962247_i64 >> (*_1);
    (*_1) = !_8;
    (*_1) = _8;
    _12.2 = (*_1) as u64;
    (*_1) = _8 << _17.0.fld1;
    _12.0 = -_17.2;
    (*_1) = _8 + RET;
    _5 = _4 & _9;
    Call((*_1) = fn3(_17.0, Move(_17), Move(_1)), ReturnTo(bb16), UnwindUnreachable())
    }
    bb18 = {
    Return()
    }
    bb19 = {
    _9 = _4;
    (*_1) = _6;
    _11 = '\u{bc6e7}' as u16;
    RET = !(*_1);
    (*_1) = -RET;
    _10 = (*_1) > (*_1);
    _2 = (-32684_i16) << _8;
    (*_1) = _6 | RET;
    Call((*_1) = core::intrinsics::bswap(_6), ReturnTo(bb2), UnwindUnreachable())
    }
    bb20 = {
    _7 = (-5965270870520108484_i64) as i8;
    _12.2 = (-164714684559875379919518575010379717139_i128) as u64;
    match _4 {
    0 => bb10,
    1 => bb2,
    2 => bb3,
    3 => bb4,
    4 => bb8,
    5 => bb6,
    1705677637 => bb12,
    _ => bb9
    }
    }
    bb21 = {
    _17.1 = core::ptr::addr_of_mut!(_7);
    (*_1) = _19 as isize;
    _4 = !_5;
    (*_1) = _7 as isize;
    _11 = Field::<u16>(Variant(_12.1, 1), 1) | Field::<u16>(Variant(_12.1, 1), 1);
    (*_1) = -_6;
    (*_1) = _11 as isize;
    _5 = _9;
    (*_1) = _6 * _6;
    _17.2 = _12.0;
    _12.0 = _17.2 - _17.2;
    (*_1) = -_6;
    _5 = _9 | _4;
    _19 = !154324724564984337_usize;
    _24 = (-7387354944997064603_i64) as u32;
    (*_1) = _27 << _6;
    _24 = _9 + _9;
    _13 = '\u{80019}';
    _9 = _5;
    _8 = _19 as isize;
    (*_1) = !_6;
    _15 = _12.2 + _12.2;
    Goto(bb22)
    }
    bb22 = {
    _29 = (*_1) | (*_1);
    (*_1) = (-5700737117018847508_i64) as isize;
    _17.2 = -_12.0;
    (*_1) = _29;
    _30 = (*_1) - (*_1);
    _19 = 2_usize;
    _27 = (*_1) ^ (*_1);
    (*_1) = _29;
    _17.0.fld1 = !120_u8;
    _3 = 1974352442_i32 & 329090610_i32;
    (*_1) = !_30;
    _4 = !_9;
    _1 = core::ptr::addr_of!(RET);
    (*_1) = _30 + _20;
    (*_1) = 196282538932702649401602673873245405864_u128 as isize;
    _9 = !_5;
    (*_1) = _27 + _27;
    (*_1) = _30 * _29;
    (*_1) = _30;
    _12.1 = Adt19::Variant1 { fld0: 6895932766907231654_i64,fld1: _11 };
    _7 = 118_i8;
    place!(Field::<i64>(Variant(_12.1, 1), 0)) = _2 as i64;
    _25 = _10;
    _31.0 = _2;
    _20 = -(*_1);
    _2 = !_31.0;
    _4 = !_24;
    _13 = '\u{7a25f}';
    _4 = !_24;
    Return()
    }
    }
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn3(mut _1: Adt38, mut _2: (Adt38, *mut i8, f32, u16), mut _3: *const isize) -> isize {
    mir! {
    type RET = isize;
    let _4: &'static mut i128;
    let _5: i64;
    let _6: *const [u32; 7];
    let _7: *mut *mut u128;
    let _8: u8;
    let _9: char;
    let _10: f32;
    let _11: i64;
    let _12: (i16, &'static mut *const usize, *mut *mut u128);
    let _13: *const &'static *mut &'static *mut f32;
    let _14: *const (u16, char, u16, char);
    let _15: u32;
    let _16: (i64, i8, &'static mut u32);
    let _17: &'static mut *const usize;
    let _18: i8;
    let _19: &'static mut &'static mut &'static mut i128;
    let _20: *mut u128;
    let _21: &'static i32;
    let _22: (f32, Adt19, u64);
    let _23: &'static mut &'static mut i128;
    let _24: u8;
    let _25: f64;
    let _26: usize;
    let _27: i32;
    let _28: &'static mut u32;
    let _29: *const [u32; 7];
    let _30: *const (u16, char, u16, char);
    let _31: &'static u16;
    let _32: u64;
    let _33: isize;
    let _34: &'static *mut f32;
    let _35: &'static *mut &'static *mut f32;
    let _36: *const isize;
    let _37: Adt77;
    let _38: &'static mut *const usize;
    let _39: f32;
    let _40: &'static *const u32;
    let _41: i32;
    let _42: i8;
    let _43: *mut (*mut i8, f32);
    let _44: ();
    let _45: ();
    {
    _3 = core::ptr::addr_of!(RET);
    RET = 9223372036854775807_isize ^ 9223372036854775807_isize;
    _2.2 = 29_i8 as f32;
    _2.0 = _1;
    RET = 0_usize as isize;
    _1 = _2.0;
    _2.0.fld0 = _1.fld1 <= _1.fld1;
    _2.0.fld1 = _1.fld1 & _1.fld1;
    _1 = _2.0;
    _2.2 = 6_usize as f32;
    _5 = (-132614621780067676997298547168048385845_i128) as i64;
    _2.2 = 10788421843279431862_u64 as f32;
    _1.fld0 = _2.0.fld0 > _2.0.fld0;
    RET = (-9223372036854775808_isize) << _1.fld1;
    _2.0.fld0 = RET <= RET;
    _1.fld1 = (-1691482354_i32) as u8;
    _2.2 = _5 as f32;
    _10 = -_2.2;
    _2.3 = !58295_u16;
    _11 = _5;
    _2.2 = -_10;
    _10 = _2.2 + _2.2;
    _1.fld0 = !_2.0.fld0;
    _8 = _2.0.fld1 | _2.0.fld1;
    _15 = 110151075300492307991721281786217034188_i128 as u32;
    _10 = _2.2 * _2.2;
    _2.0.fld0 = !_1.fld0;
    _11 = !_5;
    _16.0 = _11 >> _1.fld1;
    _1.fld0 = true;
    _16.2 = &mut _15;
    _10 = (-13480_i16) as f32;
    RET = (-58_isize) - 9223372036854775807_isize;
    _12.0 = 81_i8 as i16;
    _2.3 = '\u{b28e7}' as u16;
    _1.fld1 = _8 | _2.0.fld1;
    _16.0 = -_11;
    _2.1 = core::ptr::addr_of_mut!(_16.1);
    _5 = _11 & _11;
    _7 = core::ptr::addr_of_mut!(_20);
    _12.2 = core::ptr::addr_of_mut!((*_7));
    _9 = '\u{100386}';
    RET = 9223372036854775807_isize >> _2.0.fld1;
    _18 = (-44_i8) | 60_i8;
    _22.1 = Adt19::Variant0 { fld0: _2.2 };
    _12.2 = core::ptr::addr_of_mut!((*_7));
    (*_3) = (-9223372036854775808_isize) & (-9223372036854775808_isize);
    _2.0.fld1 = _8 << (*_3);
    place!(Field::<f32>(Variant(_22.1, 0), 0)) = _2.2;
    _24 = !_8;
    _2.3 = !39834_u16;
    _22.2 = 14261340190625307726_u64;
    _1.fld1 = _8 & _8;
    _2.1 = core::ptr::addr_of_mut!(_18);
    _16.1 = 307949236_u32 as i8;
    _2.0.fld1 = _8;
    _16.0 = _5 & _5;
    _2.2 = -_10;
    (*_3) = -9223372036854775807_isize;
    _16.0 = _5;
    _22.0 = _10 - Field::<f32>(Variant(_22.1, 0), 0);
    (*_3) = 119_isize;
    (*_3) = (-9223372036854775808_isize);
    (*_3) = 9223372036854775807_isize ^ (-9223372036854775808_isize);
    _28 = Move(_16.2);
    (*_3) = (-9223372036854775808_isize) + 9223372036854775807_isize;
    (*_3) = (-9223372036854775808_isize);
    _2.0.fld0 = !_1.fld0;
    _12.2 = core::ptr::addr_of_mut!((*_7));
    (*_3) = _18 as isize;
    _27 = (-81860450_i32) ^ (-1802449442_i32);
    _12.2 = core::ptr::addr_of_mut!((*_7));
    _16.0 = -_5;
    (*_3) = 9223372036854775807_isize;
    _11 = _16.0 * _5;
    (*_3) = -(-9223372036854775808_isize);
    _9 = '\u{8d8f6}';
    Call(_10 = core::intrinsics::transmute(_27), ReturnTo(bb12), UnwindUnreachable())
    }
    bb12 = {
    _2.0 = Adt38 { fld0: _1.fld0,fld1: _8 };
    _16.0 = _11 | _11;
    _9 = '\u{7bc0}';
    _16.1 = _18 * _18;
    (*_3) = 9223372036854775807_isize & (-9223372036854775808_isize);
    _33 = (*_3) | (*_3);
    (*_3) = _33;
    RET = _33;
    (*_3) = _33 >> _16.1;
    _8 = _2.0.fld1;
    _2.0.fld1 = Field::<f32>(Variant(_22.1, 0), 0) as u8;
    (*_3) = _33;
    _27 = (-1348685535_i32);
    _1.fld1 = _8 + _24;
    (*_3) = _33 ^ _33;
    (*_3) = _33 ^ _33;
    _1.fld1 = _24 | _24;
    _16.1 = _18;
    _16.0 = !_5;
    (*_3) = -_33;
    _22.2 = 3438072089515700681_u64 - 6099435436600275609_u64;
    _2.0 = Adt38 { fld0: _1.fld0,fld1: _24 };
    place!(Field::<f32>(Variant(_22.1, 0), 0)) = _22.0 - _22.0;
    (*_3) = _33;
    RET = _33 - _33;
    _33 = (*_3);
    _31 = &_2.3;
    _32 = _22.2;
    (*_3) = _33;
    _3 = core::ptr::addr_of!((*_3));
    _2.0 = _1;
    _22.2 = _32 - _32;
    _10 = _22.2 as f32;
    (*_3) = -_33;
    _7 = core::ptr::addr_of_mut!((*_7));
    (*_3) = -_33;
    (*_3) = -_33;
    _33 = !(*_3);
    _11 = _1.fld1 as i64;
    (*_3) = !_33;
    _9 = '\u{b162c}';
    (*_3) = _33 | _33;
    _22.1 = Adt19::Variant0 { fld0: _10 };
    _22.1 = Adt19::Variant0 { fld0: _22.0 };
    (*_3) = _33 >> _11;
    _21 = &_27;
    _8 = _2.0.fld1 ^ _2.0.fld1;
    _9 = '\u{e0207}';
    (*_3) = _33 - _33;
    _2.1 = core::ptr::addr_of_mut!(_16.1);
    _27 = 768251332_i32;
    _16.1 = _18;
    _36 = core::ptr::addr_of!((*_3));
    (*_36) = !_33;
    _12.2 = core::ptr::addr_of_mut!((*_7));
    _33 = (*_3);
    (*_3) = _33 ^ _33;
    _3 = Move(_36);
    _22.0 = Field::<f32>(Variant(_22.1, 0), 0) + Field::<f32>(Variant(_22.1, 0), 0);
    _16.1 = 53278134074209558_usize as i8;
    _36 = core::ptr::addr_of!(_33);
    _22.1 = Adt19::Variant0 { fld0: _10 };
    _1 = _2.0;
    _26 = (*_31) as usize;
    _41 = (*_36) as i32;
    _2.2 = -_22.0;
    (*_36) = -RET;
    (*_36) = _2.2 as isize;
    _42 = -_18;
    (*_36) = _1.fld1 as isize;
    Call(_44 = dump_var(3_usize, 44_usize, Move(_44), 44_usize, Move(_44), 33_usize, Move(_33), 44_usize, Move(_44)), ReturnTo(bb22), UnwindUnreachable())
    }
    bb22 = {
    Return()
    }
    }
}
pub fn main() {
    let _1 = true;
    let _5 = '\u{f2e32}' as i16;
    let _3 = 1705677637_u32;
    let _11 = !(-69_isize);
    let _2 = '\u{9f647}';
    let _6 = (-579835010_i32) & (-913890743_i32);
    let _13 = _2 as isize;
    let RET = core::ptr::addr_of!(_13);
    let _4 = _5 as i8;
    fn2((RET), _5, _6, _3, _3, _11, _4, _11, _3, _1);
}
#[derive(Debug)]
pub enum Adt19 {
    Variant0 { fld0: f32 },
    Variant1 { fld0: i64, fld1: u16 },
}
#[derive(Debug)]
pub enum Adt27 {
    Variant0 {
        fld0: bool,
        fld1: *mut u128,
        fld2: i32,
        fld3: i128,
        fld4: *mut i8,
    },
    Variant1 {
        fld0: i128,
        fld1: *mut i8,
        fld2: i16,
        fld3: (u8,),
    },
    Variant2 {
        fld0: (u8,),
        fld1: [u64; 3],
        fld2: u16,
        fld3: [i8; 3],
        fld4: u64,
        fld5: i128,
        fld6: i64,
    },
    Variant3 {
        fld0: (*mut i8, f32),
        fld1: i32,
    },
}
#[derive(Debug)]
pub enum Adt31 {
    Variant0 {
        fld0: u32,
        fld1: u128,
        fld2: [char; 3],
        fld3: [i8; 3],
        fld4: *mut u128,
        fld5: [u64; 3],
    },
    Variant1 {
        fld0: bool,
        fld1: (u16, char, u16, char),
        fld2: isize,
        fld3: [u64; 3],
        fld4: u128,
        fld5: u32,
        fld6: *const usize,
        fld7: i128,
    },
    Variant2 {
        fld0: Adt19,
        fld1: f64,
        fld2: (u8,),
        fld3: [u64; 3],
    },
    Variant3 {
        fld0: (u8,),
        fld1: u8,
        fld2: isize,
    },
}
#[derive(Debug, Copy, Clone)]
pub struct Adt38 {
    fld0: bool,
    fld1: u8,
}
#[derive(Debug, Copy, Clone)]
pub struct Adt69 {}
#[derive(Debug)]
pub struct Adt73 {}
#[derive(Debug)]
pub enum Adt77 {
    Variant0 {
        fld0: bool,
        fld1: *mut (Adt27, [u64; 3]),
        fld2: (f32, Adt19, u64),
        fld3: Adt73,
        fld4: [u32; 7],
        fld5: i32,
    },
    Variant1 {
        fld0: [i16; 1],
        fld1: *mut i8,
        fld2: [u16; 2],
    },
    Variant2 {
        fld0: ((char, i64, u64),),
        fld1: Adt73,
        fld2: isize,
        fld3: *mut *const (u16, char, u16, char),
        fld4: (u8,),
        fld5: u16,
        fld6: [u64; 6],
    },
}
#[derive(Debug)]
pub enum Adt80 {
    Variant0 {
        fld0: Adt38,
        fld1: u16,
        fld2: *mut (*mut i8, f32),
    },
    Variant1 {
        fld0: *const Adt19,
        fld1: i16,
        fld2: ((char, i64, u64),),
        fld3: *const f64,
    },
    Variant2 {
        fld0: Adt77,
        fld1: *mut (Adt27, [u64; 3]),
        fld2: isize,
        fld3: (u16, char, u16, char),
    },
    Variant3 {
        fld0: (u16, char, u16, char),
        fld1: [i8; 3],
        fld2: isize,
        fld3: Adt77,
        fld4: usize,
        fld5: *mut f32,
        fld6: [u32; 3],
        fld7: i128,
    },
}
