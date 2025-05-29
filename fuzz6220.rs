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
pub fn fn0(
    mut _1: bool,
    mut _2: char,
    mut _3: u32,
    mut _4: i8,
    mut _5: i16,
    mut _6: i32,
    mut _7: u64,
    mut _8: i128,
    mut _9: usize,
    mut _10: u8,
) -> *const isize {
    mir! {
    type RET = *const isize;
    let _11: isize;
    let _12: *mut *const (u16, char, u16, char);
    let _13: isize;
    let _14: i128;
    let _15: *mut &'static mut i128;
    let _16: i128;
    let _17: isize;
    let _18: *mut *mut u128;
    let _19: i8;
    let _20: &'static u16;
    let _21: ();
    let _22: ();
    {
    _6 = (-490548648_i32) << 27_u8;
    _3 = 9223372036854775807_isize as u32;
    _9 = 4_usize | 0_usize;
    _3 = 276284447_u32;
    _5 = (-20316_i16) & 21671_i16;
    _9 = 3211410340839670799_usize;
    _10 = 117_u8 >> _3;
    _3 = 108473845_u32 * 3429943929_u32;
    _4 = 106_i8 << _10;
    Call(_3 = fn1(_9, _10, _6), ReturnTo(bb1), UnwindUnreachable())
    }
    bb1 = {
    _4 = 13_i8 | 69_i8;
    _11 = !9223372036854775807_isize;
    _4 = (-83_i8) * (-60_i8);
    _10 = 28_u8 ^ 126_u8;
    match _9 {
    0 => bb2,
    1 => bb3,
    2 => bb4,
    3 => bb5,
    4 => bb6,
    5 => bb7,
    6 => bb8,
    3211410340839670799 => bb10,
    _ => bb9
    }
    }
    bb2 = {
    Return()
    }
    bb3 = {
    Return()
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
    _8 = -(-74594111102561672337624229532925128096_i128);
    _6 = _10 as i32;
    _3 = 327404582_u32 - 3447443329_u32;
    _7 = 2566651110013045348_u64 + 18042342023407690942_u64;
    match _9 {
    0 => bb5,
    1 => bb2,
    3211410340839670799 => bb12,
    _ => bb11
    }
    }
    bb11 = {
    Return()
    }
    bb12 = {
    _9 = !1860958094811741148_usize;
    _1 = _3 == _3;
    _5 = '\u{f2e32}' as i16;
    _3 = 1705677637_u32;
    _11 = !(-69_isize);
    _2 = '\u{301b4}';
    _2 = '\u{44a45}';
    _6 = (-2083129733_i32) ^ (-35950126_i32);
    _2 = '\u{9f647}';
    _6 = (-579835010_i32) & (-913890743_i32);
    _10 = 247_u8 >> _7;
    RET = core::ptr::addr_of!(_13);
    RET = core::ptr::addr_of!((*RET));
    (*RET) = _11;
    (*RET) = _11 - _11;
    (*RET) = _11;
    _4 = _5 as i8;
    (*RET) = _2 as isize;
    Call((*RET) = fn2(Move(RET), _5, _6, _3, _3, _11, _4, _11, _3, _1), ReturnTo(bb13), UnwindUnreachable())
    }
    bb13 = {
    _9 = 7114802606059189462_usize;
    _5 = _8 as i16;
    _14 = _8 << _13;
    _2 = '\u{59ab6}';
    _10 = 226_u8 >> _9;
    _6 = 897749596_i32 << _14;
    Goto(bb14)
    }
    bb14 = {
    _13 = (-2536769339401828889_i64) as isize;
    _4 = 6410058577522609581_i64 as i8;
    _14 = _3 as i128;
    _9 = 2_usize - 4345415827160416690_usize;
    _2 = '\u{76aa2}';
    RET = core::ptr::addr_of!(_13);
    _13 = _11;
    _3 = 2820832109_u32 ^ 3855585435_u32;
    _4 = !(-26_i8);
    _3 = 148517108_u32 + 1896541022_u32;
    _9 = 6_usize;
    (*RET) = _11 * _11;
    _13 = _11 | _11;
    (*RET) = _11 | _11;
    _11 = (*RET);
    (*RET) = _1 as isize;
    _13 = _1 as isize;
    Call(_14 = core::intrinsics::transmute(_8), ReturnTo(bb15), UnwindUnreachable())
    }
    bb15 = {
    (*RET) = _11 >> _6;
    _11 = (*RET) | (*RET);
    (*RET) = !_11;
    (*RET) = _2 as isize;
    (*RET) = -_11;
    (*RET) = _11;
    (*RET) = !_11;
    (*RET) = !_11;
    _6 = _7 as i32;
    (*RET) = _11;
    _13 = _5 as isize;
    _16 = -_14;
    (*RET) = _11 + _11;
    RET = core::ptr::addr_of!(_11);
    (*RET) = _13;
    (*RET) = _13 ^ _13;
    (*RET) = _13 | _13;
    _6 = (*RET) as i32;
    _4 = _7 as i8;
    _9 = !2_usize;
    (*RET) = _1 as isize;
    (*RET) = _13 >> _13;
    _1 = !true;
    _3 = 1124440513_u32;
    _19 = !_4;
    (*RET) = -_13;
    _17 = 8896214980887425441_i64 as isize;
    Goto(bb16)
    }
    bb16 = {
    Call(_21 = dump_var(0_usize, 7_usize, Move(_7), 19_usize, Move(_19), 16_usize, Move(_16), 13_usize, Move(_13)), ReturnTo(bb17), UnwindUnreachable())
    }
    bb17 = {
    Call(_21 = dump_var(0_usize, 11_usize, Move(_11), 5_usize, Move(_5), 6_usize, Move(_6), 14_usize, Move(_14)), ReturnTo(bb18), UnwindUnreachable())
    }
    bb18 = {
    Return()
    }

    }
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn1(mut _1: usize, mut _2: u8, mut _3: i32) -> u32 {
    mir! {
    type RET = u32;
    let _4: &'static &'static i32;
    let _5: char;
    let _6: (char, i64, u64);
    let _7: isize;
    let _8: i64;
    let _9: f64;
    let _10: &'static *const u32;
    let _11: i128;
    let _12: [i128; 3];
    let _13: *const usize;
    let _14: *const &'static *mut &'static *mut f32;
    let _15: char;
    let _16: i8;
    let _17: isize;
    let _18: isize;
    let _19: isize;
    let _20: *const [u32; 7];
    let _21: *mut *const (u16, char, u16, char);
    let _22: (*mut &'static *mut f32, Adt19, usize);
    let _23: *mut f32;
    let _24: i32;
    let _25: ();
    let _26: ();
    {
    _1 = 10923621060836370633_usize - 7_usize;
    RET = 2470395427_u32;
    _1 = 2_usize;
    RET = 1061464682_u32;
    _1 = 10110641985695007050_usize & 12387858373422632693_usize;
    _3 = (-238975214_i32);
    _1 = !10704812173381955184_usize;
    _3 = _2 as i32;
    RET = !1688394608_u32;
    _2 = !12_u8;
    _1 = 17864847905402721782_usize;
    _5 = '\u{b372d}';
    RET = 59472_u16 as u32;
    RET = 2885046697_u32 & 361323776_u32;
    RET = 2599913190_u32 - 4102448881_u32;
    _5 = '\u{554fb}';
    _6 = (_5, (-7259225364892192805_i64), 16928225710780353299_u64);
    _5 = _6.0;
    _5 = _6.0;
    _7 = (-9223372036854775808_isize) | 9223372036854775807_isize;
    RET = 1436856598_u32;
    _8 = _6.2 as i64;
    _1 = 6_usize | 6_usize;
    _3 = !1217864800_i32;
    _6.2 = 4446556873352513170_u64 | 12196036179529192670_u64;
    match RET {
    1436856598 => bb2,
    _ => bb1
    }
    }
    bb1 = {
    Return()
    }
    bb2 = {
    _7 = _3 as isize;
    _3 = !(-743006746_i32);
    _9 = _7 as f64;
    _6 = (_5, _8, 3800893789261763850_u64);
    _5 = _6.0;
    _1 = !0_usize;
    _6.0 = _5;
    _7 = (-9223372036854775808_isize) - (-9223372036854775808_isize);
    _9 = (-101554931956271220413604886313671703357_i128) as f64;
    _3 = (-1096296857_i32);
    RET = 3121413472_u32 << _6.2;
    _2 = 40_u8 * 128_u8;
    _1 = 7997395823062230315_usize;
    _6 = (_5, _8, 2156106303908624764_u64);
    _8 = _6.1;
    _6 = (_5, _8, 11557505528953355191_u64);
    _3 = !(-1957322460_i32);
    _11 = 56420379835363344264942848716291695527_i128 >> _2;
    RET = 2418287626_u32 ^ 691610479_u32;
    _6.2 = 3350002059337292686_u64 - 18187277867447963995_u64;
    RET = 218931031874376858970617599593324445011_u128 as u32;
    _6 = (_5, _8, 16026867390980505515_u64);
    _6 = (_5, _8, 16482684623485857419_u64);
    _6 = (_5, _8, 7620587094831521921_u64);
    _12 = [_11,_11,_11];
    RET = 207119998_u32 - 1970465333_u32;
    _2 = !157_u8;
    Goto(bb3)
    }
    bb3 = {
    _6.1 = _8 << _7;
    _6.1 = _8 - _8;
    _6.2 = 4728293845938169984_u64;
    _6.1 = -_8;
    _3 = (-45064466_i32) << _11;
    _7 = (-9223372036854775808_isize) + (-94_isize);
    _13 = core::ptr::addr_of!(_1);
    (*_13) = (-8908_i16) as usize;
    (*_13) = 4_usize | 2_usize;
    _5 = _6.0;
    (*_13) = _6.2 as usize;
    (*_13) = 520818035361301192_usize;
    _8 = RET as i64;
    _9 = _11 as f64;
    _5 = _6.0;
    _11 = -111486309981891241854366817015718563730_i128;
    _6.2 = 18268522563136351047_u64;
    Call(_3 = core::intrinsics::transmute(_6.0), ReturnTo(bb4), UnwindUnreachable())
    }
    bb4 = {
    (*_13) = 0_usize;
    _11 = _12[_1] ^ _12[_1];
    (*_13) = !7_usize;
    (*_13) = 33843_u16 as usize;
    _6.0 = _5;
    (*_13) = 1_usize ^ 5_usize;
    (*_13) = 6_usize & 12064610476128499371_usize;
    (*_13) = 3612845010838103432_usize;
    (*_13) = 3_usize;
    _6.2 = 6015520656689617594_u64 ^ 9578640206225606554_u64;
    (*_13) = 4485177144260932209_usize + 15796985663520062514_usize;
    (*_13) = _9 as usize;
    (*_13) = 7_usize;
    (*_13) = !18174979463075466299_usize;
    _7 = (-46_isize) - (-9223372036854775808_isize);
    (*_13) = _5 as usize;
    (*_13) = 1949309668186196621_usize - 683507353911111252_usize;
    _7 = (*_13) as isize;
    (*_13) = 0_usize - 6_usize;
    _15 = _5;
    (*_13) = 7_usize ^ 4_usize;
    _8 = _6.1;
    Goto(bb5)
    }
    bb5 = {
    (*_13) = _3 as usize;
    (*_13) = (-21177_i16) as usize;
    _16 = 66_i8 | (-3_i8);
    (*_13) = 2_usize;
    (*_13) = _6.0 as usize;
    (*_13) = RET as usize;
    _6.2 = !7840761727199031439_u64;
    Goto(bb6)
    }
    bb6 = {
    _6.2 = 5850583067398516892_u64 + 147201484560160354_u64;
    (*_13) = 6_usize * 9004595466435624565_usize;
    _11 = -158630285373352166778915638158916988224_i128;
    (*_13) = !14004190613148182900_usize;
    (*_13) = 18237734597913013231_usize - 4299765422103124995_usize;
    (*_13) = _11 as usize;
    (*_13) = !4_usize;
    (*_13) = _15 as usize;
    (*_13) = 0_usize ^ 0_usize;
    _6 = (_15, _8, 15327673155462763199_u64);
    _7 = -9223372036854775807_isize;
    (*_13) = !4_usize;
    (*_13) = 3_usize;
    _6.2 = 48655_u16 as u64;
    (*_13) = 45786533605681192_usize;
    _19 = _7 + _7;
    _18 = _19 * _19;
    (*_13) = 10895206503955656036_usize;
    _6.2 = 2863664591329399741_u64 | 18089281652400719164_u64;
    (*_13) = _18 as usize;
    _22.2 = (*_13) - (*_13);
    _7 = -_18;
    Goto(bb7)
    }
    bb7 = {
    (*_13) = _22.2 ^ _22.2;
    _12 = [_11,_11,_11];
    (*_13) = _22.2 + _22.2;
    (*_13) = !_22.2;
    _13 = core::ptr::addr_of!((*_13));
    (*_13) = _22.2 & _22.2;
    _6.2 = 17442543694354436944_u64 - 827520451716642830_u64;
    (*_13) = _6.1 as usize;
    RET = 3889540976_u32 << (*_13);
    (*_13) = _22.2 - _22.2;
    _8 = _6.1 << (*_13);
    (*_13) = _22.2;
    (*_13) = _22.2 - _22.2;
    _12 = [_11,_11,_11];
    _6 = (_5, _8, 1255929956430962267_u64);
    _24 = false as i32;
    _6.0 = _5;
    RET = 1019379600_u32 >> (*_13);
    _17 = true as isize;
    (*_13) = _22.2 | _22.2;
    (*_13) = _22.2;
    _8 = _6.1;
    _17 = _7 >> (*_13);
    (*_13) = _22.2 | _22.2;
    _22.2 = _1;
    _22.2 = (*_13) * (*_13);
    (*_13) = _22.2 * _22.2;
    Goto(bb8)
    }
    bb8 = {
    Call(_25 = dump_var(1_usize, 8_usize, Move(_8), 2_usize, Move(_2), 17_usize, Move(_17), 1_usize, Move(_1)), ReturnTo(bb9), UnwindUnreachable())
    }
    bb9 = {
    Call(_25 = dump_var(1_usize, 18_usize, Move(_18), 12_usize, Move(_12), 5_usize, Move(_5), 26_usize, _26), ReturnTo(bb10), UnwindUnreachable())
    }
    bb10 = {
    Return()
    }

    }
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
    Goto(bb23)
    }
    bb23 = {
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
    Goto(bb24)
    }
    bb24 = {
    Call(_33 = dump_var(2_usize, 4_usize, Move(_4), 5_usize, Move(_5), 25_usize, Move(_25), 2_usize, Move(_2)), ReturnTo(bb25), UnwindUnreachable())
    }
    bb25 = {
    Call(_33 = dump_var(2_usize, 10_usize, Move(_10), 11_usize, Move(_11), 7_usize, Move(_7), 30_usize, Move(_30)), ReturnTo(bb26), UnwindUnreachable())
    }
    bb26 = {
    Call(_33 = dump_var(2_usize, 13_usize, Move(_13), 14_usize, Move(_14), 34_usize, _34, 34_usize, _34), ReturnTo(bb27), UnwindUnreachable())
    }
    bb27 = {
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
    (*_3) = -9223372036854775807_isize;
    (*_3) = (-9223372036854775808_isize) + 111_isize;
    (*_3) = (-9223372036854775808_isize) - (-9223372036854775808_isize);
    RET = 9223372036854775807_isize ^ 9223372036854775807_isize;
    _2.2 = 29_i8 as f32;
    (*_3) = 26_isize * (-9223372036854775808_isize);
    _2.0 = _1;
    _3 = core::ptr::addr_of!((*_3));
    (*_3) = (-9223372036854775808_isize);
    (*_3) = (-9223372036854775808_isize) + 9223372036854775807_isize;
    RET = 0_usize as isize;
    (*_3) = !19_isize;
    _1 = _2.0;

    _2.0.fld0 = _1.fld1 <= _1.fld1;
    _2.0.fld1 = _1.fld1 & _1.fld1;
    _3 = core::ptr::addr_of!(RET);
    (*_3) = 4144192226440297703_usize as isize;
    (*_3) = !9223372036854775807_isize;
    (*_3) = (-9223372036854775808_isize) - 9223372036854775807_isize;
    (*_3) = (-90_isize) << _2.0.fld1;
    _1 = _2.0;
    _2.2 = 6_usize as f32;
    (*_3) = !9223372036854775807_isize;
    (*_3) = 3295993644_u32 as isize;
    (*_3) = 39_isize & 102_isize;

    _5 = (-132614621780067676997298547168048385845_i128) as i64;
    _2.2 = 10788421843279431862_u64 as f32;
    (*_3) = (-9223372036854775808_isize);
    (*_3) = (-63_isize) << _2.0.fld1;
    _1.fld0 = _2.0.fld0 > _2.0.fld0;
    (*_3) = (-9223372036854775808_isize) >> _5;
    RET = (-9223372036854775808_isize) << _1.fld1;
    (*_3) = (-9223372036854775808_isize) + (-9223372036854775808_isize);
    (*_3) = (-74_isize);
    (*_3) = 96_isize & 9223372036854775807_isize;
    (*_3) = (-122_isize) * 9223372036854775807_isize;
    (*_3) = (-13_isize);

    (*_3) = 9223372036854775807_isize;
    (*_3) = -(-9223372036854775808_isize);
    _2.0.fld0 = (*_3) <= RET;
    (*_3) = 9223372036854775807_isize - 9223372036854775807_isize;
    _1.fld1 = (-1691482354_i32) as u8;
    _2.2 = _5 as f32;
    (*_3) = !9223372036854775807_isize;
    _3 = core::ptr::addr_of!((*_3));
    _10 = -_2.2;
    (*_3) = (-81_isize);
    (*_3) = '\u{964bf}' as isize;
    _2.3 = !58295_u16;
    _11 = _5;
    (*_3) = -9223372036854775807_isize;
    (*_3) = (-9223372036854775808_isize) - (-9223372036854775808_isize);
    (*_3) = !9223372036854775807_isize;

    _2.2 = -_10;
    (*_3) = 1_usize as isize;
    _10 = _2.2 + _2.2;
    (*_3) = -33_isize;
    _1.fld0 = !_2.0.fld0;
    _8 = _2.0.fld1 | _2.0.fld1;
    (*_3) = -9223372036854775807_isize;
    (*_3) = 9223372036854775807_isize & (-9223372036854775808_isize);
    (*_3) = !(-7_isize);
    _15 = 110151075300492307991721281786217034188_i128 as u32;
    (*_3) = 9223372036854775807_isize * (-9223372036854775808_isize);
    _10 = _2.2 * _2.2;
    _2.0.fld0 = !_1.fld0;
    _11 = !_5;
    (*_3) = (-27_isize);
    _16.0 = _11 >> _1.fld1;
    _1.fld0 = (*_3) >= (*_3);
    (*_3) = 9223372036854775807_isize | 9223372036854775807_isize;

    (*_3) = _2.3 as isize;
    _16.2 = &mut _15;
    (*_3) = -(-111_isize);
    _10 = (-13480_i16) as f32;
    (*_3) = 18099697809476749944_u64 as isize;
    RET = (-58_isize) - 9223372036854775807_isize;
    _12.0 = 81_i8 as i16;
    _2.3 = '\u{b28e7}' as u16;
    _1.fld1 = _8 | _2.0.fld1;
    (*_3) = -9223372036854775807_isize;
    (*_3) = _2.3 as isize;
    _16.0 = -_11;
    _2.1 = core::ptr::addr_of_mut!(_16.1);
    (*_3) = (-9223372036854775808_isize) ^ (-9223372036854775808_isize);
    _5 = _11 & _11;
    _7 = core::ptr::addr_of_mut!(_20);
    _12.2 = core::ptr::addr_of_mut!((*_7));
    (*_3) = (-77_isize) << _8;
    (*_3) = 63_isize;
    (*_3) = (-9223372036854775808_isize);
    _9 = '\u{100386}';
    (*_3) = 4193060160_u32 as isize;
    RET = 9223372036854775807_isize >> _2.0.fld1;

    (*_3) = 9223372036854775807_isize - 112_isize;
    _18 = (-44_i8) | 60_i8;
    _22.1 = Adt19::Variant0 { fld0: _2.2 };
    (*_3) = !9223372036854775807_isize;
    (*_3) = (-122_isize) << _2.0.fld1;
    (*_3) = 9223372036854775807_isize ^ 9223372036854775807_isize;
    (*_3) = _9 as isize;
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

    Call(_44 = dump_var(3_usize, 32_usize, Move(_32), 41_usize, Move(_41), 33_usize, Move(_33), 26_usize, Move(_26)), ReturnTo(bb22), UnwindUnreachable())
    }
    bb22 = {
    Call(_44 = dump_var(3_usize, 18_usize, Move(_18), 5_usize, Move(_5), 45_usize, _45, 45_usize, _45), ReturnTo(bb23), UnwindUnreachable())
    }
    bb23 = {
    Return()
    }

    }
}
pub fn main() {
    fn0(
        std::hint::black_box(true),
        std::hint::black_box('\u{10ebef}'),
        std::hint::black_box(3125710407_u32),
        std::hint::black_box((-31_i8)),
        std::hint::black_box((-20643_i16)),
        std::hint::black_box(597372240_i32),
        std::hint::black_box(6630875623603933953_u64),
        std::hint::black_box(88385379200907167168816002794425816667_i128),
        std::hint::black_box(6077590283005072736_usize),
        std::hint::black_box(77_u8),
    );
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
pub struct Adt69 {
    fld0: [char; 3],
}
#[derive(Debug)]
pub struct Adt73 {
    fld0: i64,
    fld1: usize,
    fld2: [u64; 6],
    fld3: *const u32,
}
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
