#![recursion_limit = "1024"]
    #![feature(custom_mir, core_intrinsics, lazy_get)]
    #![allow(unused_parens, unused_assignments, overflowing_literals)]
    extern crate core;
    use core::intrinsics::mir::*;

    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    use std::sync::LazyLock;

    static mut H: LazyLock<DefaultHasher> = LazyLock::new(|| DefaultHasher::new());

    #[inline(never)]
    fn dump_var(
        val0: impl Hash,
        val1: impl Hash,
        val2: impl Hash,
        val3: impl Hash,
    ) {
        unsafe {
            val0.hash(LazyLock::force_mut(&mut H));
            val1.hash(LazyLock::force_mut(&mut H));
            val2.hash(LazyLock::force_mut(&mut H));
            val3.hash(LazyLock::force_mut(&mut H));
        }
    }
    #[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn0(mut _1: u128,mut _2: char,mut _3: i16) -> i64 {
mir! {
type RET = i64;
let _4: u128;
let _5: Adt22;
let _6: f64;
let _7: u8;
let _8: *const (f32, i16, usize, u8);
let _9: (&'static mut char,);
let _10: u16;
let _11: u64;
let _12: &'static &'static char;
let _13: f64;
let _14: f64;
let _15: f32;
let _16: &'static i16;
let _17: &'static i64;
let _18: Adt52;
let _19: ((&'static mut *mut i128,), (f32, i16, usize, u8), (&'static &'static char, i64, i16, i64), &'static i64);
let _20: f64;
let _21: f32;
let _22: *mut Adt75;
let _23: *mut &'static *mut isize;
let _24: *mut Adt73;
let _25: *const [i32; 5];
let _26: bool;
let _27: i16;
let _28: i16;
let _29: bool;
let _30: ((i64,), &'static char);
let _31: *mut u16;
let _32: *const Adt22;
let _33: isize;
let _34: isize;
let _35: &'static u128;
let _36: u16;
let _37: *mut i128;
let _38: *mut Adt75;
let _39: Adt21;
let _40: u8;
let _41: bool;
let _42: f32;
let _43: &'static *mut isize;
let _44: i128;
let _45: Adt52;
let _46: &'static u128;
let _47: bool;
let _48: *mut [isize; 5];
let _49: &'static *mut isize;
let _50: *const f32;
let _51: &'static mut char;
let _52: f32;
let _53: &'static *mut isize;
let _54: i16;
let _55: *mut u16;
let _56: [bool; 2];
let _57: (Adt22, ((i64,), &'static char), Adt52, *mut [isize; 5]);
let _58: Adt21;
let _59: f32;
let _60: [char; 8];
let _61: Adt59;
let _62: &'static char;
let _63: &'static &'static mut (i64,);
let _64: u128;
let _65: usize;
let _66: isize;
let _67: u8;
let _68: *const i64;
let _69: *mut u16;
let _70: bool;
let _71: (i16, &'static i64, [u32; 8], i16);
let _72: *mut [u8; 4];
let _73: bool;
let _74: i128;
let _75: f32;
let _76: f32;
let _77: Adt59;
let _78: (u8, i64, &'static mut [bool; 2]);
let _79: char;
let _80: i8;
let _81: (Adt22, &'static u32, (&'static &'static char, i64, i16, i64));
let _82: u8;
let _83: f32;
let _84: isize;
let _85: bool;
let _86: f32;
let _87: f32;
let _88: f32;
let _89: (Adt22, &'static u32, (&'static &'static char, i64, i16, i64));
let _90: (&'static char, [u128; 7], *const f32, *const [i32; 5]);
let _91: bool;
let _92: usize;
let _93: *const i32;
let _94: &'static mut [bool; 2];
let _95: *const (f32, i16, usize, u8);
let _96: i64;
let _97: f64;
let _98: i64;
let _99: (&'static mut *mut i128,);
let _100: i16;
let _101: &'static mut char;
let _102: ();
let _103: ();
{
RET = !(-8579808514697280993_i64);
_2 = '\u{59f78}';
_2 = '\u{e3769}';
_4 = RET as u128;
RET = !(-1949758279140455417_i64);
_5.fld3 = 7_u8 as i8;
RET = 8178738028407298749_i64 >> _4;
RET = (-3292856128879328890_i64) + (-8005422363185047889_i64);
_6 = (-9223372036854775808_isize) as f64;
_7 = 36_u8 << _5.fld3;
Goto(bb1)
}
bb1 = {
_5.fld2 = _2 as u64;
_3 = (-30519_i16);
match _3 {
0 => bb2,
340282366920938463463374607431768180937 => bb4,
_ => bb3
}
}
bb2 = {
Return()
}
bb3 = {
Return()
}
bb4 = {
_5.fld2 = 3821280017126854791_u64 & 6897969943564176309_u64;
_6 = _7 as f64;
_5.fld3 = (-118_i8) ^ 23_i8;
_9.0 = &mut _2;
RET = (-5015449149091910229_i64) ^ 5518312032259800893_i64;
_5.fld0 = _6 > _6;
_7 = 1415997674_i32 as u8;
_5.fld1 = Adt21::Variant0 { fld0: _4,fld1: (-1866364508_i32),fld2: _5.fld3 };
_1 = !Field::<u128>(Variant(_5.fld1, 0), 0);
place!(Field::<u128>(Variant(_5.fld1, 0), 0)) = _4 >> _3;
place!(Field::<i32>(Variant(_5.fld1, 0), 1)) = _3 as i32;
match _3 {
340282366920938463463374607431768180937 => bb6,
_ => bb5
}
}
bb5 = {
_5.fld2 = _2 as u64;
_3 = (-30519_i16);
match _3 {
0 => bb2,
340282366920938463463374607431768180937 => bb4,
_ => bb3
}
}
bb6 = {
_5.fld1 = Adt21::Variant0 { fld0: _1,fld1: 1983437766_i32,fld2: _5.fld3 };
_5.fld2 = 14936610476514750160_u64;
_5.fld0 = true ^ true;
place!(Field::<i8>(Variant(_5.fld1, 0), 2)) = _5.fld3;
_5.fld2 = 16181301894261421562_u64 - 17277697806723189063_u64;
_4 = Field::<u128>(Variant(_5.fld1, 0), 0);
_6 = (-9223372036854775808_isize) as f64;
_1 = Field::<u128>(Variant(_5.fld1, 0), 0) * Field::<u128>(Variant(_5.fld1, 0), 0);
place!(Field::<i32>(Variant(_5.fld1, 0), 1)) = 10497635_i32;
_7 = !221_u8;
_3 = _5.fld3 as i16;
place!(Field::<i32>(Variant(_5.fld1, 0), 1)) = 1022901304_i32 | 999347513_i32;
RET = (-901351303130764211_i64) | 6989517193313286080_i64;
_5.fld2 = 14926674757242485575_u64;
_3 = !3269_i16;
_7 = !144_u8;
_11 = !_5.fld2;
_7 = 249_u8;
_3 = (-26904_i16);
_5.fld0 = true;
_5.fld0 = Field::<i8>(Variant(_5.fld1, 0), 2) <= Field::<i8>(Variant(_5.fld1, 0), 2);
_5.fld0 = false;
Call(_3 = core::intrinsics::bswap((-12004_i16)), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
place!(Field::<i8>(Variant(_5.fld1, 0), 2)) = _5.fld3 * _5.fld3;
_1 = !Field::<u128>(Variant(_5.fld1, 0), 0);
_5.fld2 = !_11;
_11 = _5.fld2 & _5.fld2;
_6 = Field::<i32>(Variant(_5.fld1, 0), 1) as f64;
RET = 696956091390715637_i64;
_14 = _6 + _6;
_14 = _6 - _6;
Call(place!(Field::<i32>(Variant(_5.fld1, 0), 1)) = fn1(Move(_9), _5.fld0, _14, _5.fld0, _5.fld0, _3), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_10 = 24624_u16 << Field::<i32>(Variant(_5.fld1, 0), 1);
_5.fld3 = Field::<i8>(Variant(_5.fld1, 0), 2);
_11 = _5.fld2 << Field::<i8>(Variant(_5.fld1, 0), 2);
_13 = _3 as f64;
_10 = !16864_u16;
place!(Field::<i32>(Variant(_5.fld1, 0), 1)) = -1724016202_i32;
place!(Field::<i32>(Variant(_5.fld1, 0), 1)) = _3 as i32;
_13 = _14 + _14;
_11 = _5.fld2 & _5.fld2;
_11 = !_5.fld2;
_7 = 104_u8;
_17 = &RET;
_19.2.2 = _3;
_19.1.2 = 7_usize;
_13 = _6 * _6;
_4 = !Field::<u128>(Variant(_5.fld1, 0), 0);
_3 = -_19.2.2;
_8 = core::ptr::addr_of!(_19.1);
Goto(bb9)
}
bb9 = {
(*_8).1 = !_19.2.2;
(*_8).2 = !7559778919863288043_usize;
(*_8).1 = _3 << (*_8).2;
(*_8).0 = _10 as f32;
(*_8).0 = Field::<u128>(Variant(_5.fld1, 0), 0) as f32;
(*_8).3 = _7 | _7;
_19.2.2 = (*_8).3 as i16;
place!(Field::<i32>(Variant(_5.fld1, 0), 1)) = 1814870527_i32;
(*_8).0 = _4 as f32;
_10 = !18192_u16;
(*_8).1 = _3 - _19.2.2;
(*_8).2 = 84098168957717598_usize;
(*_8).2 = 4318006644409268064_usize & 16318967849927192659_usize;
_10 = !47185_u16;
(*_8).3 = _7 + _7;
(*_8).2 = 12411712402385347547_usize;
_8 = core::ptr::addr_of!((*_8));
place!(Field::<u128>(Variant(_5.fld1, 0), 0)) = !_1;
(*_8).0 = (-9223372036854775808_isize) as f32;
(*_8).3 = _7 ^ _7;
(*_8).2 = 3691163166160538657_usize >> (*_8).1;
(*_8).0 = (-117416259187339395720005135981708554287_i128) as f32;
(*_8).1 = _5.fld2 as i16;
place!(Field::<u128>(Variant(_5.fld1, 0), 0)) = _11 as u128;
place!(Field::<i32>(Variant(_5.fld1, 0), 1)) = !(-2125902561_i32);
_5.fld2 = _11 - _11;
(*_8).3 = _10 as u8;
Goto(bb10)
}
bb10 = {
(*_8).1 = _10 as i16;
(*_8).0 = (*_8).1 as f32;
_19.2.2 = (*_8).1;
(*_8).2 = 0_usize;
(*_8).2 = _5.fld0 as usize;
(*_8).1 = _3 | _3;
_5.fld2 = _11 >> (*_8).2;
(*_8).3 = _7 % _7;
_19.3 = Move(_17);
(*_8).0 = (*_8).2 as f32;
(*_8).0 = Field::<i32>(Variant(_5.fld1, 0), 1) as f32;
(*_8).0 = 82463214882623863492532528882930374998_i128 as f32;
(*_8).2 = 10539122240032714843_usize;
(*_8).1 = _3 ^ _19.2.2;
(*_8).1 = _3;
(*_8).2 = 1649085053150035394_usize >> Field::<i8>(Variant(_5.fld1, 0), 2);
(*_8).2 = 2_usize * 4_usize;
(*_8).0 = (*_8).2 as f32;
(*_8).2 = 13515987966445337097_usize;
(*_8).1 = _19.2.2 << _10;
Goto(bb11)
}
bb11 = {
(*_8).1 = _19.2.2 ^ _19.2.2;
_16 = &(*_8).1;
(*_8).2 = 1_usize - 10090646830521256056_usize;
_19.2.3 = (*_8).0 as i64;
_13 = _14 + _6;
(*_8).2 = 634123779574797270_usize - 10239732678533057721_usize;
(*_8).0 = 103_isize as f32;
(*_8).1 = -_19.2.2;
(*_8).3 = _7;
(*_8).3 = !_7;
(*_8).1 = !_3;
(*_8).1 = _3 - _19.2.2;
_5.fld1 = Adt21::Variant1 { fld0: _5.fld0,fld1: _6,fld2: (-9223372036854775808_isize),fld3: _5.fld3,fld4: (*_8).1,fld5: 4233843021_u32,fld6: (*_8).0,fld7: (*_8).2 };
(*_8).2 = Field::<usize>(Variant(_5.fld1, 1), 7) | Field::<usize>(Variant(_5.fld1, 1), 7);
_19.2.1 = !_19.2.3;
(*_8) = (Field::<f32>(Variant(_5.fld1, 1), 6), Field::<i16>(Variant(_5.fld1, 1), 4), Field::<usize>(Variant(_5.fld1, 1), 7), _7);
Goto(bb12)
}
bb12 = {
(*_8).0 = -Field::<f32>(Variant(_5.fld1, 1), 6);
(*_8).3 = _7 + _7;
_19.1 = (Field::<f32>(Variant(_5.fld1, 1), 6), Field::<i16>(Variant(_5.fld1, 1), 4), Field::<usize>(Variant(_5.fld1, 1), 7), _7);
(*_8) = (Field::<f32>(Variant(_5.fld1, 1), 6), _3, Field::<usize>(Variant(_5.fld1, 1), 7), _7);
(*_8).0 = -Field::<f32>(Variant(_5.fld1, 1), 6);
(*_8).2 = (*_8).3 as usize;
_5.fld1 = Adt21::Variant0 { fld0: _4,fld1: 250551514_i32,fld2: _5.fld3 };
place!(Field::<i32>(Variant(_5.fld1, 0), 1)) = 513013440_i32 + (-492415746_i32);
(*_8).0 = (*_8).1 as f32;
place!(Field::<i32>(Variant(_5.fld1, 0), 1)) = 1391515561_i32 + 1417485434_i32;
(*_8).0 = Field::<i32>(Variant(_5.fld1, 0), 1) as f32;
(*_8).0 = (-9223372036854775808_isize) as f32;
(*_8).1 = _19.2.2 - _19.2.2;
(*_8).1 = _3 * _3;
(*_8).1 = _5.fld2 as i16;
place!(Field::<i8>(Variant(_5.fld1, 0), 2)) = _5.fld3 * _5.fld3;
(*_8).0 = (*_8).1 as f32;
_31 = core::ptr::addr_of_mut!(_10);
(*_8).2 = !6_usize;
(*_8).1 = _3 >> (*_31);
(*_31) = !50398_u16;
_1 = _4 + Field::<u128>(Variant(_5.fld1, 0), 0);
(*_8).1 = _3 ^ _3;
(*_8).3 = (*_8).1 as u8;
(*_31) = 33203_u16;
Goto(bb13)
}
bb13 = {
(*_31) = 35614_u16;
(*_8).0 = 95_isize as f32;
(*_8).2 = 6_usize - 3_usize;
(*_8).1 = _19.2.2;
(*_8).2 = 1_usize & 2_usize;
(*_8).3 = _7 % _7;
(*_8).2 = 3_usize;
(*_8).2 = 3_usize;
_19.1.3 = 2652218001_u32 as u8;
(*_31) = !40191_u16;
(*_8).2 = '\u{723bd}' as usize;
RET = _19.2.1 ^ _19.2.1;
(*_8).0 = Field::<i32>(Variant(_5.fld1, 0), 1) as f32;
(*_8).0 = 670468339_u32 as f32;
_31 = core::ptr::addr_of_mut!((*_31));
(*_8).1 = _19.2.2 >> (*_8).2;
(*_8).0 = (-118_isize) as f32;
_41 = _5.fld0;
_34 = 9223372036854775807_isize >> _5.fld3;
_21 = -(*_8).0;
(*_8).2 = (*_8).3 as usize;
(*_8).1 = 82556005667746185004524871980852456814_i128 as i16;
Goto(bb14)
}
bb14 = {
_32 = core::ptr::addr_of!(_5);
(*_32).fld2 = _11 & _11;
(*_32).fld0 = _41;
(*_32).fld2 = !_11;
_1 = RET as u128;
_35 = &place!(Field::<u128>(Variant((*_32).fld1, 0), 0));
(*_32).fld3 = Field::<i8>(Variant((*_32).fld1, 0), 2);
(*_32).fld1 = Adt21::Variant0 { fld0: _1,fld1: (-493007376_i32),fld2: (*_32).fld3 };
(*_32).fld2 = _19.2.3 as u64;
_14 = _13 * _13;
place!(Field::<i32>(Variant((*_32).fld1, 0), 1)) = (-104111261_i32);
(*_8).0 = -_21;
(*_31) = !64994_u16;
RET = !_19.2.1;
_14 = _6 - _13;
(*_32).fld2 = !_11;
(*_32).fld0 = _5.fld3 >= Field::<i8>(Variant((*_32).fld1, 0), 2);
(*_8).0 = _21;
(*_8).3 = !_7;
(*_8).3 = Field::<i32>(Variant((*_32).fld1, 0), 1) as u8;
(*_8).2 = (*_32).fld2 as usize;
(*_32).fld1 = Adt21::Variant0 { fld0: _1,fld1: 597214795_i32,fld2: (*_32).fld3 };
Goto(bb15)
}
bb15 = {
(*_8).3 = !_7;
(*_8).1 = _19.2.2 - _19.2.2;
place!(Field::<i8>(Variant((*_32).fld1, 0), 2)) = (*_32).fld3;
(*_8).3 = !_7;
(*_8).1 = !_19.2.2;
(*_32).fld1 = Adt21::Variant1 { fld0: (*_32).fld0,fld1: _14,fld2: _34,fld3: (*_32).fld3,fld4: (*_8).1,fld5: 3739800675_u32,fld6: (*_8).0,fld7: (*_8).2 };
(*_31) = !54011_u16;
place!(Field::<f64>(Variant((*_32).fld1, 1), 1)) = _13 - _14;
(*_32).fld3 = Field::<i8>(Variant((*_32).fld1, 1), 3) + Field::<i8>(Variant((*_32).fld1, 1), 3);
place!(Field::<bool>(Variant((*_32).fld1, 1), 0)) = (*_32).fld0 & _5.fld0;
(*_8) = (Field::<f32>(Variant((*_32).fld1, 1), 6), Field::<i16>(Variant((*_32).fld1, 1), 4), Field::<usize>(Variant((*_32).fld1, 1), 7), _7);
(*_8).0 = Field::<f32>(Variant((*_32).fld1, 1), 6) + Field::<f32>(Variant((*_32).fld1, 1), 6);
(*_32).fld0 = Field::<bool>(Variant((*_32).fld1, 1), 0) ^ Field::<bool>(Variant((*_32).fld1, 1), 0);
(*_32).fld1 = Adt21::Variant1 { fld0: (*_32).fld0,fld1: _13,fld2: _34,fld3: (*_32).fld3,fld4: (*_8).1,fld5: 109462157_u32,fld6: (*_8).0,fld7: (*_8).2 };
(*_32).fld0 = !Field::<bool>(Variant((*_32).fld1, 1), 0);
Call(place!(Field::<i16>(Variant((*_32).fld1, 1), 4)) = core::intrinsics::bswap((*_8).1), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
place!(Field::<u32>(Variant((*_32).fld1, 1), 5)) = (*_32).fld0 as u32;
RET = _19.2.3 + _19.2.3;
_28 = (*_8).1 >> (*_32).fld3;
(*_32).fld0 = Field::<i8>(Variant((*_32).fld1, 1), 3) > (*_32).fld3;
_19.1.0 = Field::<f32>(Variant((*_32).fld1, 1), 6) + Field::<f32>(Variant((*_32).fld1, 1), 6);
place!(Field::<u32>(Variant((*_32).fld1, 1), 5)) = !1689156719_u32;
place!(Field::<f32>(Variant((*_32).fld1, 1), 6)) = _21;
_50 = core::ptr::addr_of!(place!(Field::<f32>(Variant((*_32).fld1, 1), 6)));
place!(Field::<i8>(Variant((*_32).fld1, 1), 3)) = (*_32).fld0 as i8;
_54 = (*_8).1 ^ Field::<i16>(Variant((*_32).fld1, 1), 4);
place!(Field::<usize>(Variant((*_32).fld1, 1), 7)) = (*_8).2 | (*_8).2;
(*_32).fld2 = _11;
(*_31) = 20264_u16;
place!(Field::<i8>(Variant((*_32).fld1, 1), 3)) = -(*_32).fld3;
place!(Field::<usize>(Variant((*_32).fld1, 1), 7)) = _13 as usize;
place!(Field::<f64>(Variant((*_32).fld1, 1), 1)) = (*_8).1 as f64;
place!(Field::<i16>(Variant((*_32).fld1, 1), 4)) = (*_8).1 >> Field::<i8>(Variant((*_32).fld1, 1), 3);
_30.0 = (_19.2.3,);
place!(Field::<usize>(Variant((*_32).fld1, 1), 7)) = _34 as usize;
place!(Field::<f64>(Variant((*_32).fld1, 1), 1)) = Field::<i16>(Variant((*_32).fld1, 1), 4) as f64;
(*_50) = (*_8).0 + (*_8).0;
place!(Field::<bool>(Variant((*_32).fld1, 1), 0)) = !(*_32).fld0;
(*_8).0 = Field::<f32>(Variant((*_32).fld1, 1), 6) * (*_50);
(*_32).fld3 = Field::<i8>(Variant((*_32).fld1, 1), 3) ^ Field::<i8>(Variant((*_32).fld1, 1), 3);
(*_8).0 = Field::<f32>(Variant((*_32).fld1, 1), 6);
_15 = (*_32).fld3 as f32;
match (*_31) {
0 => bb17,
1 => bb18,
20264 => bb20,
_ => bb19
}
}
bb17 = {
place!(Field::<i8>(Variant(_5.fld1, 0), 2)) = _5.fld3 * _5.fld3;
_1 = !Field::<u128>(Variant(_5.fld1, 0), 0);
_5.fld2 = !_11;
_11 = _5.fld2 & _5.fld2;
_6 = Field::<i32>(Variant(_5.fld1, 0), 1) as f64;
RET = 696956091390715637_i64;
_14 = _6 + _6;
_14 = _6 - _6;
Call(place!(Field::<i32>(Variant(_5.fld1, 0), 1)) = fn1(Move(_9), _5.fld0, _14, _5.fld0, _5.fld0, _3), ReturnTo(bb8), UnwindUnreachable())
}
bb18 = {
_5.fld2 = _2 as u64;
_3 = (-30519_i16);
match _3 {
0 => bb2,
340282366920938463463374607431768180937 => bb4,
_ => bb3
}
}
bb19 = {
(*_31) = 35614_u16;
(*_8).0 = 95_isize as f32;
(*_8).2 = 6_usize - 3_usize;
(*_8).1 = _19.2.2;
(*_8).2 = 1_usize & 2_usize;
(*_8).3 = _7 % _7;
(*_8).2 = 3_usize;
(*_8).2 = 3_usize;
_19.1.3 = 2652218001_u32 as u8;
(*_31) = !40191_u16;
(*_8).2 = '\u{723bd}' as usize;
RET = _19.2.1 ^ _19.2.1;
(*_8).0 = Field::<i32>(Variant(_5.fld1, 0), 1) as f32;
(*_8).0 = 670468339_u32 as f32;
_31 = core::ptr::addr_of_mut!((*_31));
(*_8).1 = _19.2.2 >> (*_8).2;
(*_8).0 = (-118_isize) as f32;
_41 = _5.fld0;
_34 = 9223372036854775807_isize >> _5.fld3;
_21 = -(*_8).0;
(*_8).2 = (*_8).3 as usize;
(*_8).1 = 82556005667746185004524871980852456814_i128 as i16;
Goto(bb14)
}
bb20 = {
(*_8).2 = !Field::<usize>(Variant((*_32).fld1, 1), 7);
(*_32).fld1 = Adt21::Variant1 { fld0: (*_32).fld0,fld1: _13,fld2: _34,fld3: (*_32).fld3,fld4: _54,fld5: 2496134787_u32,fld6: (*_8).0,fld7: (*_8).2 };
place!(Field::<f32>(Variant((*_32).fld1, 1), 6)) = (*_8).0 - (*_8).0;
place!(Field::<i8>(Variant((*_32).fld1, 1), 3)) = (*_8).0 as i8;
place!(Field::<u32>(Variant((*_32).fld1, 1), 5)) = !725861980_u32;
(*_8).2 = Field::<usize>(Variant((*_32).fld1, 1), 7);
_7 = (*_8).3;
(*_32).fld1 = Adt21::Variant0 { fld0: _4,fld1: (-748948320_i32),fld2: (*_32).fld3 };
_36 = (*_8).2 as u16;
(*_32).fld2 = _11 >> Field::<i8>(Variant((*_32).fld1, 0), 2);
place!(Field::<i32>(Variant((*_32).fld1, 0), 1)) = (-1537676142_i32) - 130422537_i32;
(*_8) = (_15, _28, 10862467496856587271_usize, _7);
(*_8).3 = _7 + _7;
(*_8).3 = !_7;
(*_32).fld0 = (*_8).2 <= (*_8).2;
(*_32).fld0 = !_41;
(*_8).3 = Field::<i32>(Variant((*_32).fld1, 0), 1) as u8;
match (*_8).2 {
10862467496856587271 => bb22,
_ => bb21
}
}
bb21 = {
(*_8).3 = !_7;
(*_8).1 = _19.2.2 - _19.2.2;
place!(Field::<i8>(Variant((*_32).fld1, 0), 2)) = (*_32).fld3;
(*_8).3 = !_7;
(*_8).1 = !_19.2.2;
(*_32).fld1 = Adt21::Variant1 { fld0: (*_32).fld0,fld1: _14,fld2: _34,fld3: (*_32).fld3,fld4: (*_8).1,fld5: 3739800675_u32,fld6: (*_8).0,fld7: (*_8).2 };
(*_31) = !54011_u16;
place!(Field::<f64>(Variant((*_32).fld1, 1), 1)) = _13 - _14;
(*_32).fld3 = Field::<i8>(Variant((*_32).fld1, 1), 3) + Field::<i8>(Variant((*_32).fld1, 1), 3);
place!(Field::<bool>(Variant((*_32).fld1, 1), 0)) = (*_32).fld0 & _5.fld0;
(*_8) = (Field::<f32>(Variant((*_32).fld1, 1), 6), Field::<i16>(Variant((*_32).fld1, 1), 4), Field::<usize>(Variant((*_32).fld1, 1), 7), _7);
(*_8).0 = Field::<f32>(Variant((*_32).fld1, 1), 6) + Field::<f32>(Variant((*_32).fld1, 1), 6);
(*_32).fld0 = Field::<bool>(Variant((*_32).fld1, 1), 0) ^ Field::<bool>(Variant((*_32).fld1, 1), 0);
(*_32).fld1 = Adt21::Variant1 { fld0: (*_32).fld0,fld1: _13,fld2: _34,fld3: (*_32).fld3,fld4: (*_8).1,fld5: 109462157_u32,fld6: (*_8).0,fld7: (*_8).2 };
(*_32).fld0 = !Field::<bool>(Variant((*_32).fld1, 1), 0);
Call(place!(Field::<i16>(Variant((*_32).fld1, 1), 4)) = core::intrinsics::bswap((*_8).1), ReturnTo(bb16), UnwindUnreachable())
}
bb22 = {
_54 = (*_8).1 - (*_8).1;
_35 = &place!(Field::<u128>(Variant((*_32).fld1, 0), 0));
Goto(bb23)
}
bb23 = {
_6 = (*_31) as f64;
place!(Field::<i32>(Variant((*_32).fld1, 0), 1)) = 1361041659_i32;
(*_8).2 = 12954960913865323994_usize & 8688072049061560598_usize;
place!(Field::<u128>(Variant((*_32).fld1, 0), 0)) = _1;
(*_8).3 = _7 | _7;
(*_32).fld2 = _11 >> Field::<i8>(Variant((*_32).fld1, 0), 2);
_40 = (*_8).3 * (*_8).3;
(*_8).0 = -_15;
(*_32).fld1 = Adt21::Variant1 { fld0: (*_32).fld0,fld1: _14,fld2: _34,fld3: (*_32).fld3,fld4: (*_8).1,fld5: 982960414_u32,fld6: (*_8).0,fld7: (*_8).2 };
_26 = (*_32).fld0;
place!(Field::<usize>(Variant((*_32).fld1, 1), 7)) = !(*_8).2;
(*_32).fld2 = _11;
(*_8).1 = -Field::<i16>(Variant((*_32).fld1, 1), 4);
(*_8) = (Field::<f32>(Variant((*_32).fld1, 1), 6), Field::<i16>(Variant((*_32).fld1, 1), 4), Field::<usize>(Variant((*_32).fld1, 1), 7), _40);
place!(Field::<i16>(Variant((*_32).fld1, 1), 4)) = -_19.1.1;
place!(Field::<usize>(Variant((*_32).fld1, 1), 7)) = (*_8).2;
(*_32).fld0 = Field::<bool>(Variant((*_32).fld1, 1), 0);
(*_31) = _36 * _36;
_19.1.0 = Field::<f32>(Variant((*_32).fld1, 1), 6) + _15;
_60 = ['\u{42ec7}','\u{27aa}','\u{d7b1c}','\u{c144c}','\u{c8d8d}','\u{54c94}','\u{4552b}','\u{971e1}'];
(*_32).fld1 = Adt21::Variant0 { fld0: _1,fld1: 2015260367_i32,fld2: (*_32).fld3 };
_60 = ['\u{a1815}','\u{8802d}','\u{285b1}','\u{d8185}','\u{a7824}','\u{9b2da}','\u{cd647}','\u{1b48b}'];
place!(Field::<i8>(Variant((*_32).fld1, 0), 2)) = (*_32).fld3 ^ (*_32).fld3;
(*_32).fld1 = Adt21::Variant1 { fld0: (*_32).fld0,fld1: _13,fld2: _34,fld3: (*_32).fld3,fld4: (*_8).1,fld5: 611027022_u32,fld6: (*_8).0,fld7: (*_8).2 };
(*_32).fld0 = Field::<bool>(Variant((*_32).fld1, 1), 0) | Field::<bool>(Variant((*_32).fld1, 1), 0);
_69 = core::ptr::addr_of_mut!((*_31));
Goto(bb24)
}
bb24 = {
(*_32).fld2 = !_11;
_57.1.0 = (RET,);
Goto(bb25)
}
bb25 = {
(*_32).fld0 = Field::<i8>(Variant((*_32).fld1, 1), 3) > (*_32).fld3;
(*_8).2 = Field::<usize>(Variant((*_32).fld1, 1), 7) + Field::<usize>(Variant((*_32).fld1, 1), 7);
(*_69) = (*_32).fld0 as u16;
(*_8).2 = Field::<usize>(Variant((*_32).fld1, 1), 7);
(*_8).2 = !Field::<usize>(Variant((*_32).fld1, 1), 7);
_35 = &_1;
(*_32).fld1 = Adt21::Variant0 { fld0: (*_35),fld1: (-849780479_i32),fld2: (*_32).fld3 };
(*_8).0 = 1343430897_i32 as f32;
match _7 {
0 => bb18,
1 => bb8,
2 => bb24,
3 => bb4,
104 => bb26,
_ => bb7
}
}
bb26 = {
(*_31) = _36 << Field::<i8>(Variant((*_32).fld1, 0), 2);
(*_32).fld1 = Adt21::Variant1 { fld0: (*_32).fld0,fld1: _14,fld2: _34,fld3: (*_32).fld3,fld4: (*_8).1,fld5: 1928969223_u32,fld6: (*_8).0,fld7: (*_8).2 };
(*_32).fld1 = Adt21::Variant1 { fld0: (*_32).fld0,fld1: _14,fld2: _34,fld3: (*_32).fld3,fld4: (*_8).1,fld5: 2824370864_u32,fld6: _15,fld7: (*_8).2 };
(*_31) = _36 + _36;
(*_32).fld3 = Field::<i8>(Variant((*_32).fld1, 1), 3) >> Field::<i16>(Variant((*_32).fld1, 1), 4);
(*_8).2 = Field::<usize>(Variant((*_32).fld1, 1), 7) - Field::<usize>(Variant((*_32).fld1, 1), 7);
(*_31) = Field::<isize>(Variant((*_32).fld1, 1), 2) as u16;
_44 = (-96368458948106586660189364935826647126_i128);
(*_8).3 = _7;
place!(Field::<isize>(Variant((*_32).fld1, 1), 2)) = -_34;
(*_32).fld1 = Adt21::Variant0 { fld0: (*_35),fld1: (-1178775612_i32),fld2: (*_32).fld3 };
(*_32).fld1 = Adt21::Variant1 { fld0: (*_32).fld0,fld1: _13,fld2: _34,fld3: (*_32).fld3,fld4: (*_8).1,fld5: 461748241_u32,fld6: (*_8).0,fld7: (*_8).2 };
place!(Field::<bool>(Variant((*_32).fld1, 1), 0)) = (*_32).fld0 | (*_32).fld0;
place!(Field::<i16>(Variant((*_32).fld1, 1), 4)) = Field::<bool>(Variant((*_32).fld1, 1), 0) as i16;
place!(Field::<i8>(Variant((*_32).fld1, 1), 3)) = _40 as i8;
place!(Field::<u32>(Variant((*_32).fld1, 1), 5)) = 3370374660_u32 + 2121664684_u32;
place!(Field::<usize>(Variant((*_32).fld1, 1), 7)) = Field::<isize>(Variant((*_32).fld1, 1), 2) as usize;
place!(Field::<u32>(Variant((*_32).fld1, 1), 5)) = 715249846_u32 - 848135922_u32;
(*_8).2 = !Field::<usize>(Variant((*_32).fld1, 1), 7);
place!(Field::<usize>(Variant((*_32).fld1, 1), 7)) = (*_8).2;
place!(Field::<isize>(Variant((*_32).fld1, 1), 2)) = _34 >> Field::<i16>(Variant((*_32).fld1, 1), 4);
(*_8).2 = _15 as usize;
(*_8) = (_15, Field::<i16>(Variant((*_32).fld1, 1), 4), Field::<usize>(Variant((*_32).fld1, 1), 7), _40);
place!(Field::<i8>(Variant((*_32).fld1, 1), 3)) = -(*_32).fld3;
place!(Field::<bool>(Variant((*_32).fld1, 1), 0)) = !(*_32).fld0;
(*_32).fld2 = _11 + _11;
place!(Field::<i8>(Variant(_5.fld1, 1), 3)) = !(*_32).fld3;
Goto(bb27)
}
bb27 = {
(*_32).fld2 = !_11;
place!(Field::<usize>(Variant((*_32).fld1, 1), 7)) = !(*_8).2;
place!(Field::<u32>(Variant((*_32).fld1, 1), 5)) = 2771183187_u32 ^ 353123213_u32;
(*_32).fld1 = Adt21::Variant0 { fld0: (*_35),fld1: 784622293_i32,fld2: (*_32).fld3 };
(*_32).fld3 = Field::<i8>(Variant((*_32).fld1, 0), 2) - Field::<i8>(Variant((*_32).fld1, 0), 2);
(*_32).fld0 = _41 | _26;
(*_31) = !_36;
(*_8).3 = !_40;
(*_32).fld0 = _41;
_19.1.1 = !_54;
_47 = (*_8).1 < (*_8).1;
(*_8).2 = (*_8).1 as usize;
(*_32).fld1 = Adt21::Variant0 { fld0: (*_35),fld1: 162460293_i32,fld2: (*_32).fld3 };
_74 = _44 ^ _44;
(*_32).fld0 = (*_8).3 <= (*_8).3;
(*_32).fld0 = !_47;
_57.0.fld3 = _57.1.0.0 as i8;
(*_32).fld1 = Adt21::Variant1 { fld0: (*_32).fld0,fld1: _14,fld2: _34,fld3: (*_32).fld3,fld4: (*_8).1,fld5: 231489259_u32,fld6: (*_8).0,fld7: (*_8).2 };
(*_8).0 = Field::<f32>(Variant(_5.fld1, 1), 6);
_1 = Field::<isize>(Variant((*_32).fld1, 1), 2) as u128;
place!(Field::<isize>(Variant((*_32).fld1, 1), 2)) = !_34;
place!(Field::<bool>(Variant((*_32).fld1, 1), 0)) = _47 ^ (*_32).fld0;
place!(Field::<i16>(Variant((*_32).fld1, 1), 4)) = (*_8).1 | (*_8).1;
Goto(bb28)
}
bb28 = {
(*_32).fld1 = Adt21::Variant0 { fld0: _1,fld1: (-76093594_i32),fld2: (*_32).fld3 };
(*_8).2 = 2132025596_u32 as usize;
(*_32).fld0 = (*_32).fld3 == Field::<i8>(Variant(_5.fld1, 0), 2);
(*_31) = _13 as u16;
(*_8).2 = !8114229005697003874_usize;
_52 = (*_8).0 + (*_8).0;
(*_32).fld1 = Adt21::Variant0 { fld0: _1,fld1: 2133552373_i32,fld2: (*_32).fld3 };
_3 = (*_8).1 | (*_8).1;
(*_32).fld2 = _11 & _11;
(*_8) = (_52, _3, 2_usize, _40);
(*_8) = (_52, _3, 17326731092537618302_usize, _40);
match (*_8).2 {
0 => bb29,
1 => bb30,
2 => bb31,
3 => bb32,
4 => bb33,
5 => bb34,
6 => bb35,
17326731092537618302 => bb37,
_ => bb36
}
}
bb29 = {
(*_32).fld2 = !_11;
place!(Field::<usize>(Variant((*_32).fld1, 1), 7)) = !(*_8).2;
place!(Field::<u32>(Variant((*_32).fld1, 1), 5)) = 2771183187_u32 ^ 353123213_u32;
(*_32).fld1 = Adt21::Variant0 { fld0: (*_35),fld1: 784622293_i32,fld2: (*_32).fld3 };
(*_32).fld3 = Field::<i8>(Variant((*_32).fld1, 0), 2) - Field::<i8>(Variant((*_32).fld1, 0), 2);
(*_32).fld0 = _41 | _26;
(*_31) = !_36;
(*_8).3 = !_40;
(*_32).fld0 = _41;
_19.1.1 = !_54;
_47 = (*_8).1 < (*_8).1;
(*_8).2 = (*_8).1 as usize;
(*_32).fld1 = Adt21::Variant0 { fld0: (*_35),fld1: 162460293_i32,fld2: (*_32).fld3 };
_74 = _44 ^ _44;
(*_32).fld0 = (*_8).3 <= (*_8).3;
(*_32).fld0 = !_47;
_57.0.fld3 = _57.1.0.0 as i8;
(*_32).fld1 = Adt21::Variant1 { fld0: (*_32).fld0,fld1: _14,fld2: _34,fld3: (*_32).fld3,fld4: (*_8).1,fld5: 231489259_u32,fld6: (*_8).0,fld7: (*_8).2 };
(*_8).0 = Field::<f32>(Variant(_5.fld1, 1), 6);
_1 = Field::<isize>(Variant((*_32).fld1, 1), 2) as u128;
place!(Field::<isize>(Variant((*_32).fld1, 1), 2)) = !_34;
place!(Field::<bool>(Variant((*_32).fld1, 1), 0)) = _47 ^ (*_32).fld0;
place!(Field::<i16>(Variant((*_32).fld1, 1), 4)) = (*_8).1 | (*_8).1;
Goto(bb28)
}
bb30 = {
(*_31) = _36 << Field::<i8>(Variant((*_32).fld1, 0), 2);
(*_32).fld1 = Adt21::Variant1 { fld0: (*_32).fld0,fld1: _14,fld2: _34,fld3: (*_32).fld3,fld4: (*_8).1,fld5: 1928969223_u32,fld6: (*_8).0,fld7: (*_8).2 };
(*_32).fld1 = Adt21::Variant1 { fld0: (*_32).fld0,fld1: _14,fld2: _34,fld3: (*_32).fld3,fld4: (*_8).1,fld5: 2824370864_u32,fld6: _15,fld7: (*_8).2 };
(*_31) = _36 + _36;
(*_32).fld3 = Field::<i8>(Variant((*_32).fld1, 1), 3) >> Field::<i16>(Variant((*_32).fld1, 1), 4);
(*_8).2 = Field::<usize>(Variant((*_32).fld1, 1), 7) - Field::<usize>(Variant((*_32).fld1, 1), 7);
(*_31) = Field::<isize>(Variant((*_32).fld1, 1), 2) as u16;
_44 = (-96368458948106586660189364935826647126_i128);
(*_8).3 = _7;
place!(Field::<isize>(Variant((*_32).fld1, 1), 2)) = -_34;
(*_32).fld1 = Adt21::Variant0 { fld0: (*_35),fld1: (-1178775612_i32),fld2: (*_32).fld3 };
(*_32).fld1 = Adt21::Variant1 { fld0: (*_32).fld0,fld1: _13,fld2: _34,fld3: (*_32).fld3,fld4: (*_8).1,fld5: 461748241_u32,fld6: (*_8).0,fld7: (*_8).2 };
place!(Field::<bool>(Variant((*_32).fld1, 1), 0)) = (*_32).fld0 | (*_32).fld0;
place!(Field::<i16>(Variant((*_32).fld1, 1), 4)) = Field::<bool>(Variant((*_32).fld1, 1), 0) as i16;
place!(Field::<i8>(Variant((*_32).fld1, 1), 3)) = _40 as i8;
place!(Field::<u32>(Variant((*_32).fld1, 1), 5)) = 3370374660_u32 + 2121664684_u32;
place!(Field::<usize>(Variant((*_32).fld1, 1), 7)) = Field::<isize>(Variant((*_32).fld1, 1), 2) as usize;
place!(Field::<u32>(Variant((*_32).fld1, 1), 5)) = 715249846_u32 - 848135922_u32;
(*_8).2 = !Field::<usize>(Variant((*_32).fld1, 1), 7);
place!(Field::<usize>(Variant((*_32).fld1, 1), 7)) = (*_8).2;
place!(Field::<isize>(Variant((*_32).fld1, 1), 2)) = _34 >> Field::<i16>(Variant((*_32).fld1, 1), 4);
(*_8).2 = _15 as usize;
(*_8) = (_15, Field::<i16>(Variant((*_32).fld1, 1), 4), Field::<usize>(Variant((*_32).fld1, 1), 7), _40);
place!(Field::<i8>(Variant((*_32).fld1, 1), 3)) = -(*_32).fld3;
place!(Field::<bool>(Variant((*_32).fld1, 1), 0)) = !(*_32).fld0;
(*_32).fld2 = _11 + _11;
place!(Field::<i8>(Variant(_5.fld1, 1), 3)) = !(*_32).fld3;
Goto(bb27)
}
bb31 = {
(*_8).3 = !_7;
(*_8).1 = _19.2.2 - _19.2.2;
place!(Field::<i8>(Variant((*_32).fld1, 0), 2)) = (*_32).fld3;
(*_8).3 = !_7;
(*_8).1 = !_19.2.2;
(*_32).fld1 = Adt21::Variant1 { fld0: (*_32).fld0,fld1: _14,fld2: _34,fld3: (*_32).fld3,fld4: (*_8).1,fld5: 3739800675_u32,fld6: (*_8).0,fld7: (*_8).2 };
(*_31) = !54011_u16;
place!(Field::<f64>(Variant((*_32).fld1, 1), 1)) = _13 - _14;
(*_32).fld3 = Field::<i8>(Variant((*_32).fld1, 1), 3) + Field::<i8>(Variant((*_32).fld1, 1), 3);
place!(Field::<bool>(Variant((*_32).fld1, 1), 0)) = (*_32).fld0 & _5.fld0;
(*_8) = (Field::<f32>(Variant((*_32).fld1, 1), 6), Field::<i16>(Variant((*_32).fld1, 1), 4), Field::<usize>(Variant((*_32).fld1, 1), 7), _7);
(*_8).0 = Field::<f32>(Variant((*_32).fld1, 1), 6) + Field::<f32>(Variant((*_32).fld1, 1), 6);
(*_32).fld0 = Field::<bool>(Variant((*_32).fld1, 1), 0) ^ Field::<bool>(Variant((*_32).fld1, 1), 0);
(*_32).fld1 = Adt21::Variant1 { fld0: (*_32).fld0,fld1: _13,fld2: _34,fld3: (*_32).fld3,fld4: (*_8).1,fld5: 109462157_u32,fld6: (*_8).0,fld7: (*_8).2 };
(*_32).fld0 = !Field::<bool>(Variant((*_32).fld1, 1), 0);
Call(place!(Field::<i16>(Variant((*_32).fld1, 1), 4)) = core::intrinsics::bswap((*_8).1), ReturnTo(bb16), UnwindUnreachable())
}
bb32 = {
(*_8).0 = -Field::<f32>(Variant(_5.fld1, 1), 6);
(*_8).3 = _7 + _7;
_19.1 = (Field::<f32>(Variant(_5.fld1, 1), 6), Field::<i16>(Variant(_5.fld1, 1), 4), Field::<usize>(Variant(_5.fld1, 1), 7), _7);
(*_8) = (Field::<f32>(Variant(_5.fld1, 1), 6), _3, Field::<usize>(Variant(_5.fld1, 1), 7), _7);
(*_8).0 = -Field::<f32>(Variant(_5.fld1, 1), 6);
(*_8).2 = (*_8).3 as usize;
_5.fld1 = Adt21::Variant0 { fld0: _4,fld1: 250551514_i32,fld2: _5.fld3 };
place!(Field::<i32>(Variant(_5.fld1, 0), 1)) = 513013440_i32 + (-492415746_i32);
(*_8).0 = (*_8).1 as f32;
place!(Field::<i32>(Variant(_5.fld1, 0), 1)) = 1391515561_i32 + 1417485434_i32;
(*_8).0 = Field::<i32>(Variant(_5.fld1, 0), 1) as f32;
(*_8).0 = (-9223372036854775808_isize) as f32;
(*_8).1 = _19.2.2 - _19.2.2;
(*_8).1 = _3 * _3;
(*_8).1 = _5.fld2 as i16;
place!(Field::<i8>(Variant(_5.fld1, 0), 2)) = _5.fld3 * _5.fld3;
(*_8).0 = (*_8).1 as f32;
_31 = core::ptr::addr_of_mut!(_10);
(*_8).2 = !6_usize;
(*_8).1 = _3 >> (*_31);
(*_31) = !50398_u16;
_1 = _4 + Field::<u128>(Variant(_5.fld1, 0), 0);
(*_8).1 = _3 ^ _3;
(*_8).3 = (*_8).1 as u8;
(*_31) = 33203_u16;
Goto(bb13)
}
bb33 = {
_6 = (*_31) as f64;
place!(Field::<i32>(Variant((*_32).fld1, 0), 1)) = 1361041659_i32;
(*_8).2 = 12954960913865323994_usize & 8688072049061560598_usize;
place!(Field::<u128>(Variant((*_32).fld1, 0), 0)) = _1;
(*_8).3 = _7 | _7;
(*_32).fld2 = _11 >> Field::<i8>(Variant((*_32).fld1, 0), 2);
_40 = (*_8).3 * (*_8).3;
(*_8).0 = -_15;
(*_32).fld1 = Adt21::Variant1 { fld0: (*_32).fld0,fld1: _14,fld2: _34,fld3: (*_32).fld3,fld4: (*_8).1,fld5: 982960414_u32,fld6: (*_8).0,fld7: (*_8).2 };
_26 = (*_32).fld0;
place!(Field::<usize>(Variant((*_32).fld1, 1), 7)) = !(*_8).2;
(*_32).fld2 = _11;
(*_8).1 = -Field::<i16>(Variant((*_32).fld1, 1), 4);
(*_8) = (Field::<f32>(Variant((*_32).fld1, 1), 6), Field::<i16>(Variant((*_32).fld1, 1), 4), Field::<usize>(Variant((*_32).fld1, 1), 7), _40);
place!(Field::<i16>(Variant((*_32).fld1, 1), 4)) = -_19.1.1;
place!(Field::<usize>(Variant((*_32).fld1, 1), 7)) = (*_8).2;
(*_32).fld0 = Field::<bool>(Variant((*_32).fld1, 1), 0);
(*_31) = _36 * _36;
_19.1.0 = Field::<f32>(Variant((*_32).fld1, 1), 6) + _15;
_60 = ['\u{42ec7}','\u{27aa}','\u{d7b1c}','\u{c144c}','\u{c8d8d}','\u{54c94}','\u{4552b}','\u{971e1}'];
(*_32).fld1 = Adt21::Variant0 { fld0: _1,fld1: 2015260367_i32,fld2: (*_32).fld3 };
_60 = ['\u{a1815}','\u{8802d}','\u{285b1}','\u{d8185}','\u{a7824}','\u{9b2da}','\u{cd647}','\u{1b48b}'];
place!(Field::<i8>(Variant((*_32).fld1, 0), 2)) = (*_32).fld3 ^ (*_32).fld3;
(*_32).fld1 = Adt21::Variant1 { fld0: (*_32).fld0,fld1: _13,fld2: _34,fld3: (*_32).fld3,fld4: (*_8).1,fld5: 611027022_u32,fld6: (*_8).0,fld7: (*_8).2 };
(*_32).fld0 = Field::<bool>(Variant((*_32).fld1, 1), 0) | Field::<bool>(Variant((*_32).fld1, 1), 0);
_69 = core::ptr::addr_of_mut!((*_31));
Goto(bb24)
}
bb34 = {
_5.fld2 = _2 as u64;
_3 = (-30519_i16);
match _3 {
0 => bb2,
340282366920938463463374607431768180937 => bb4,
_ => bb3
}
}
bb35 = {
place!(Field::<i8>(Variant(_5.fld1, 0), 2)) = _5.fld3 * _5.fld3;
_1 = !Field::<u128>(Variant(_5.fld1, 0), 0);
_5.fld2 = !_11;
_11 = _5.fld2 & _5.fld2;
_6 = Field::<i32>(Variant(_5.fld1, 0), 1) as f64;
RET = 696956091390715637_i64;
_14 = _6 + _6;
_14 = _6 - _6;
Call(place!(Field::<i32>(Variant(_5.fld1, 0), 1)) = fn1(Move(_9), _5.fld0, _14, _5.fld0, _5.fld0, _3), ReturnTo(bb8), UnwindUnreachable())
}
bb36 = {
(*_8).1 = _19.2.2 ^ _19.2.2;
_16 = &(*_8).1;
(*_8).2 = 1_usize - 10090646830521256056_usize;
_19.2.3 = (*_8).0 as i64;
_13 = _14 + _6;
(*_8).2 = 634123779574797270_usize - 10239732678533057721_usize;
(*_8).0 = 103_isize as f32;
(*_8).1 = -_19.2.2;
(*_8).3 = _7;
(*_8).3 = !_7;
(*_8).1 = !_3;
(*_8).1 = _3 - _19.2.2;
_5.fld1 = Adt21::Variant1 { fld0: _5.fld0,fld1: _6,fld2: (-9223372036854775808_isize),fld3: _5.fld3,fld4: (*_8).1,fld5: 4233843021_u32,fld6: (*_8).0,fld7: (*_8).2 };
(*_8).2 = Field::<usize>(Variant(_5.fld1, 1), 7) | Field::<usize>(Variant(_5.fld1, 1), 7);
_19.2.1 = !_19.2.3;
(*_8) = (Field::<f32>(Variant(_5.fld1, 1), 6), Field::<i16>(Variant(_5.fld1, 1), 4), Field::<usize>(Variant(_5.fld1, 1), 7), _7);
Goto(bb12)
}
bb37 = {
(*_32).fld0 = Field::<i8>(Variant((*_32).fld1, 0), 2) == (*_32).fld3;
_40 = (*_8).3 ^ (*_8).3;
_81.2.2 = !_3;
(*_32).fld3 = Field::<i8>(Variant(_5.fld1, 0), 2) & Field::<i8>(Variant((*_32).fld1, 0), 2);
(*_31) = !_36;
(*_8).0 = _15 - _15;
(*_31) = _36;
_81.0.fld3 = !(*_32).fld3;
(*_32).fld2 = _11 & _11;
_16 = &_3;
_74 = _44 << _19.1.2;
(*_32).fld0 = !_47;
_17 = &_19.2.3;
_65 = (*_17) as usize;
place!(Field::<u128>(Variant((*_32).fld1, 0), 0)) = _1;
(*_32).fld3 = -_81.0.fld3;
(*_8).2 = _65 | _65;
_81.0.fld0 = (*_32).fld0 | (*_32).fld0;
(*_32).fld2 = !_11;
_57.1.0 = ((*_17),);
(*_32).fld1 = Adt21::Variant1 { fld0: (*_32).fld0,fld1: _14,fld2: _34,fld3: (*_32).fld3,fld4: (*_8).1,fld5: 3970330747_u32,fld6: (*_8).0,fld7: (*_8).2 };
place!(Field::<u32>(Variant((*_32).fld1, 1), 5)) = 2343772113_u32 | 2059024060_u32;
_8 = core::ptr::addr_of!((*_8));
(*_32).fld1 = Adt21::Variant1 { fld0: _47,fld1: _14,fld2: _34,fld3: (*_32).fld3,fld4: (*_16),fld5: 1427239725_u32,fld6: (*_8).0,fld7: (*_8).2 };
_57.0.fld0 = (*_32).fld0;
Call((*_8).2 = core::intrinsics::bswap(Field::<usize>(Variant((*_32).fld1, 1), 7)), ReturnTo(bb38), UnwindUnreachable())
}
bb38 = {
_88 = Field::<f64>(Variant((*_32).fld1, 1), 1) as f32;
_75 = -(*_8).0;
place!(Field::<u32>(Variant((*_32).fld1, 1), 5)) = _4 as u32;
(*_8).0 = Field::<f32>(Variant((*_32).fld1, 1), 6) + Field::<f32>(Variant(_5.fld1, 1), 6);
Goto(bb39)
}
bb39 = {
_55 = core::ptr::addr_of_mut!((*_31));
_84 = Field::<isize>(Variant((*_32).fld1, 1), 2) - Field::<isize>(Variant((*_32).fld1, 1), 2);
(*_55) = !_36;
_90.2 = Move(_50);
place!(Field::<i8>(Variant((*_32).fld1, 1), 3)) = Field::<isize>(Variant((*_32).fld1, 1), 2) as i8;
_70 = (*_32).fld0 | (*_32).fld0;
place!(Field::<u32>(Variant(_5.fld1, 1), 5)) = 2220848789_u32;
_90.2 = core::ptr::addr_of!(_21);
(*_8).3 = _40 >> (*_32).fld3;
place!(Field::<u32>(Variant((*_32).fld1, 1), 5)) = !599276646_u32;
place!(Field::<i16>(Variant((*_32).fld1, 1), 4)) = (*_8).1 ^ (*_8).1;
(*_32).fld3 = _81.0.fld3 >> _19.1.3;
(*_32).fld3 = (*_16) as i8;
place!(Field::<isize>(Variant((*_32).fld1, 1), 2)) = _74 as isize;
(*_8).3 = !_40;
place!(Field::<i8>(Variant((*_32).fld1, 1), 3)) = Field::<bool>(Variant((*_32).fld1, 1), 0) as i8;
_4 = _1 * _1;
_66 = -Field::<isize>(Variant((*_32).fld1, 1), 2);
(*_8).1 = Field::<i16>(Variant((*_32).fld1, 1), 4) - Field::<i16>(Variant((*_32).fld1, 1), 4);
match _7 {
0 => bb19,
1 => bb40,
104 => bb42,
_ => bb41
}
}
bb40 = {
_5.fld2 = _2 as u64;
_3 = (-30519_i16);
match _3 {
0 => bb2,
340282366920938463463374607431768180937 => bb4,
_ => bb3
}
}
bb41 = {
(*_8).3 = !_7;
(*_8).1 = _19.2.2 - _19.2.2;
place!(Field::<i8>(Variant((*_32).fld1, 0), 2)) = (*_32).fld3;
(*_8).3 = !_7;
(*_8).1 = !_19.2.2;
(*_32).fld1 = Adt21::Variant1 { fld0: (*_32).fld0,fld1: _14,fld2: _34,fld3: (*_32).fld3,fld4: (*_8).1,fld5: 3739800675_u32,fld6: (*_8).0,fld7: (*_8).2 };
(*_31) = !54011_u16;
place!(Field::<f64>(Variant((*_32).fld1, 1), 1)) = _13 - _14;
(*_32).fld3 = Field::<i8>(Variant((*_32).fld1, 1), 3) + Field::<i8>(Variant((*_32).fld1, 1), 3);
place!(Field::<bool>(Variant((*_32).fld1, 1), 0)) = (*_32).fld0 & _5.fld0;
(*_8) = (Field::<f32>(Variant((*_32).fld1, 1), 6), Field::<i16>(Variant((*_32).fld1, 1), 4), Field::<usize>(Variant((*_32).fld1, 1), 7), _7);
(*_8).0 = Field::<f32>(Variant((*_32).fld1, 1), 6) + Field::<f32>(Variant((*_32).fld1, 1), 6);
(*_32).fld0 = Field::<bool>(Variant((*_32).fld1, 1), 0) ^ Field::<bool>(Variant((*_32).fld1, 1), 0);
(*_32).fld1 = Adt21::Variant1 { fld0: (*_32).fld0,fld1: _13,fld2: _34,fld3: (*_32).fld3,fld4: (*_8).1,fld5: 109462157_u32,fld6: (*_8).0,fld7: (*_8).2 };
(*_32).fld0 = !Field::<bool>(Variant((*_32).fld1, 1), 0);
Call(place!(Field::<i16>(Variant((*_32).fld1, 1), 4)) = core::intrinsics::bswap((*_8).1), ReturnTo(bb16), UnwindUnreachable())
}
bb42 = {
place!(Field::<i16>(Variant((*_32).fld1, 1), 4)) = (*_16);
Goto(bb43)
}
bb43 = {
_41 = !Field::<bool>(Variant((*_32).fld1, 1), 0);
place!(Field::<i8>(Variant((*_32).fld1, 1), 3)) = _81.0.fld3;
_91 = _5.fld0;
place!(Field::<f32>(Variant((*_32).fld1, 1), 6)) = (*_8).0;
_17 = &_19.2.1;
match _7 {
0 => bb22,
1 => bb21,
2 => bb24,
3 => bb40,
4 => bb33,
5 => bb44,
6 => bb45,
104 => bb47,
_ => bb46
}
}
bb44 = {
(*_32).fld2 = !_11;
place!(Field::<usize>(Variant((*_32).fld1, 1), 7)) = !(*_8).2;
place!(Field::<u32>(Variant((*_32).fld1, 1), 5)) = 2771183187_u32 ^ 353123213_u32;
(*_32).fld1 = Adt21::Variant0 { fld0: (*_35),fld1: 784622293_i32,fld2: (*_32).fld3 };
(*_32).fld3 = Field::<i8>(Variant((*_32).fld1, 0), 2) - Field::<i8>(Variant((*_32).fld1, 0), 2);
(*_32).fld0 = _41 | _26;
(*_31) = !_36;
(*_8).3 = !_40;
(*_32).fld0 = _41;
_19.1.1 = !_54;
_47 = (*_8).1 < (*_8).1;
(*_8).2 = (*_8).1 as usize;
(*_32).fld1 = Adt21::Variant0 { fld0: (*_35),fld1: 162460293_i32,fld2: (*_32).fld3 };
_74 = _44 ^ _44;
(*_32).fld0 = (*_8).3 <= (*_8).3;
(*_32).fld0 = !_47;
_57.0.fld3 = _57.1.0.0 as i8;
(*_32).fld1 = Adt21::Variant1 { fld0: (*_32).fld0,fld1: _14,fld2: _34,fld3: (*_32).fld3,fld4: (*_8).1,fld5: 231489259_u32,fld6: (*_8).0,fld7: (*_8).2 };
(*_8).0 = Field::<f32>(Variant(_5.fld1, 1), 6);
_1 = Field::<isize>(Variant((*_32).fld1, 1), 2) as u128;
place!(Field::<isize>(Variant((*_32).fld1, 1), 2)) = !_34;
place!(Field::<bool>(Variant((*_32).fld1, 1), 0)) = _47 ^ (*_32).fld0;
place!(Field::<i16>(Variant((*_32).fld1, 1), 4)) = (*_8).1 | (*_8).1;
Goto(bb28)
}
bb45 = {
(*_8).3 = !_7;
(*_8).1 = _19.2.2 - _19.2.2;
place!(Field::<i8>(Variant((*_32).fld1, 0), 2)) = (*_32).fld3;
(*_8).3 = !_7;
(*_8).1 = !_19.2.2;
(*_32).fld1 = Adt21::Variant1 { fld0: (*_32).fld0,fld1: _14,fld2: _34,fld3: (*_32).fld3,fld4: (*_8).1,fld5: 3739800675_u32,fld6: (*_8).0,fld7: (*_8).2 };
(*_31) = !54011_u16;
place!(Field::<f64>(Variant((*_32).fld1, 1), 1)) = _13 - _14;
(*_32).fld3 = Field::<i8>(Variant((*_32).fld1, 1), 3) + Field::<i8>(Variant((*_32).fld1, 1), 3);
place!(Field::<bool>(Variant((*_32).fld1, 1), 0)) = (*_32).fld0 & _5.fld0;
(*_8) = (Field::<f32>(Variant((*_32).fld1, 1), 6), Field::<i16>(Variant((*_32).fld1, 1), 4), Field::<usize>(Variant((*_32).fld1, 1), 7), _7);
(*_8).0 = Field::<f32>(Variant((*_32).fld1, 1), 6) + Field::<f32>(Variant((*_32).fld1, 1), 6);
(*_32).fld0 = Field::<bool>(Variant((*_32).fld1, 1), 0) ^ Field::<bool>(Variant((*_32).fld1, 1), 0);
(*_32).fld1 = Adt21::Variant1 { fld0: (*_32).fld0,fld1: _13,fld2: _34,fld3: (*_32).fld3,fld4: (*_8).1,fld5: 109462157_u32,fld6: (*_8).0,fld7: (*_8).2 };
(*_32).fld0 = !Field::<bool>(Variant((*_32).fld1, 1), 0);
Call(place!(Field::<i16>(Variant((*_32).fld1, 1), 4)) = core::intrinsics::bswap((*_8).1), ReturnTo(bb16), UnwindUnreachable())
}
bb46 = {
_88 = Field::<f64>(Variant((*_32).fld1, 1), 1) as f32;
_75 = -(*_8).0;
place!(Field::<u32>(Variant((*_32).fld1, 1), 5)) = _4 as u32;
(*_8).0 = Field::<f32>(Variant((*_32).fld1, 1), 6) + Field::<f32>(Variant(_5.fld1, 1), 6);
Goto(bb39)
}
bb47 = {
place!(Field::<usize>(Variant((*_32).fld1, 1), 7)) = (*_16) as usize;
_31 = core::ptr::addr_of_mut!((*_55));
(*_8).1 = Field::<i16>(Variant((*_32).fld1, 1), 4);
(*_55) = _36 * _36;
place!(Field::<f64>(Variant((*_32).fld1, 1), 1)) = _13 - _13;
(*_32).fld1 = Adt21::Variant1 { fld0: (*_32).fld0,fld1: _14,fld2: _66,fld3: (*_32).fld3,fld4: (*_16),fld5: 386030716_u32,fld6: (*_8).0,fld7: (*_8).2 };
(*_8).3 = _40 & _40;
(*_32).fld1 = Adt21::Variant1 { fld0: (*_32).fld0,fld1: _13,fld2: _66,fld3: (*_32).fld3,fld4: (*_8).1,fld5: 2867874252_u32,fld6: (*_8).0,fld7: (*_8).2 };
(*_8).2 = _57.1.0.0 as usize;
_6 = -Field::<f64>(Variant((*_32).fld1, 1), 1);
_4 = _1;
place!(Field::<f64>(Variant(_5.fld1, 1), 1)) = _14 + _13;
(*_32).fld0 = Field::<bool>(Variant((*_32).fld1, 1), 0);
(*_32).fld1 = Adt21::Variant1 { fld0: (*_32).fld0,fld1: _14,fld2: _66,fld3: (*_32).fld3,fld4: (*_8).1,fld5: 1898273077_u32,fld6: (*_8).0,fld7: (*_8).2 };
place!(Field::<isize>(Variant((*_32).fld1, 1), 2)) = _66;
place!(Field::<u32>(Variant(_5.fld1, 1), 5)) = 2188688178_u32 * 3051850683_u32;
_89.0.fld0 = Field::<bool>(Variant((*_32).fld1, 1), 0);
(*_32).fld3 = Field::<i8>(Variant((*_32).fld1, 1), 3) - Field::<i8>(Variant((*_32).fld1, 1), 3);
match _7 {
104 => bb48,
_ => bb10
}
}
bb48 = {
place!(Field::<i8>(Variant((*_32).fld1, 1), 3)) = (*_32).fld3 - (*_32).fld3;
(*_8).0 = -Field::<f32>(Variant((*_32).fld1, 1), 6);
place!(Field::<i16>(Variant((*_32).fld1, 1), 4)) = _3 - (*_8).1;
(*_32).fld3 = Field::<u32>(Variant((*_32).fld1, 1), 5) as i8;
match _44 {
0 => bb36,
1 => bb49,
2 => bb50,
3 => bb51,
4 => bb52,
5 => bb53,
6 => bb54,
243913907972831876803185242495941564330 => bb56,
_ => bb55
}
}
bb49 = {
(*_8).2 = !Field::<usize>(Variant((*_32).fld1, 1), 7);
(*_32).fld1 = Adt21::Variant1 { fld0: (*_32).fld0,fld1: _13,fld2: _34,fld3: (*_32).fld3,fld4: _54,fld5: 2496134787_u32,fld6: (*_8).0,fld7: (*_8).2 };
place!(Field::<f32>(Variant((*_32).fld1, 1), 6)) = (*_8).0 - (*_8).0;
place!(Field::<i8>(Variant((*_32).fld1, 1), 3)) = (*_8).0 as i8;
place!(Field::<u32>(Variant((*_32).fld1, 1), 5)) = !725861980_u32;
(*_8).2 = Field::<usize>(Variant((*_32).fld1, 1), 7);
_7 = (*_8).3;
(*_32).fld1 = Adt21::Variant0 { fld0: _4,fld1: (-748948320_i32),fld2: (*_32).fld3 };
_36 = (*_8).2 as u16;
(*_32).fld2 = _11 >> Field::<i8>(Variant((*_32).fld1, 0), 2);
place!(Field::<i32>(Variant((*_32).fld1, 0), 1)) = (-1537676142_i32) - 130422537_i32;
(*_8) = (_15, _28, 10862467496856587271_usize, _7);
(*_8).3 = _7 + _7;
(*_8).3 = !_7;
(*_32).fld0 = (*_8).2 <= (*_8).2;
(*_32).fld0 = !_41;
(*_8).3 = Field::<i32>(Variant((*_32).fld1, 0), 1) as u8;
match (*_8).2 {
10862467496856587271 => bb22,
_ => bb21
}
}
bb50 = {
(*_32).fld2 = !_11;
place!(Field::<usize>(Variant((*_32).fld1, 1), 7)) = !(*_8).2;
place!(Field::<u32>(Variant((*_32).fld1, 1), 5)) = 2771183187_u32 ^ 353123213_u32;
(*_32).fld1 = Adt21::Variant0 { fld0: (*_35),fld1: 784622293_i32,fld2: (*_32).fld3 };
(*_32).fld3 = Field::<i8>(Variant((*_32).fld1, 0), 2) - Field::<i8>(Variant((*_32).fld1, 0), 2);
(*_32).fld0 = _41 | _26;
(*_31) = !_36;
(*_8).3 = !_40;
(*_32).fld0 = _41;
_19.1.1 = !_54;
_47 = (*_8).1 < (*_8).1;
(*_8).2 = (*_8).1 as usize;
(*_32).fld1 = Adt21::Variant0 { fld0: (*_35),fld1: 162460293_i32,fld2: (*_32).fld3 };
_74 = _44 ^ _44;
(*_32).fld0 = (*_8).3 <= (*_8).3;
(*_32).fld0 = !_47;
_57.0.fld3 = _57.1.0.0 as i8;
(*_32).fld1 = Adt21::Variant1 { fld0: (*_32).fld0,fld1: _14,fld2: _34,fld3: (*_32).fld3,fld4: (*_8).1,fld5: 231489259_u32,fld6: (*_8).0,fld7: (*_8).2 };
(*_8).0 = Field::<f32>(Variant(_5.fld1, 1), 6);
_1 = Field::<isize>(Variant((*_32).fld1, 1), 2) as u128;
place!(Field::<isize>(Variant((*_32).fld1, 1), 2)) = !_34;
place!(Field::<bool>(Variant((*_32).fld1, 1), 0)) = _47 ^ (*_32).fld0;
place!(Field::<i16>(Variant((*_32).fld1, 1), 4)) = (*_8).1 | (*_8).1;
Goto(bb28)
}
bb51 = {
(*_8).3 = !_7;
(*_8).1 = _19.2.2 - _19.2.2;
place!(Field::<i8>(Variant((*_32).fld1, 0), 2)) = (*_32).fld3;
(*_8).3 = !_7;
(*_8).1 = !_19.2.2;
(*_32).fld1 = Adt21::Variant1 { fld0: (*_32).fld0,fld1: _14,fld2: _34,fld3: (*_32).fld3,fld4: (*_8).1,fld5: 3739800675_u32,fld6: (*_8).0,fld7: (*_8).2 };
(*_31) = !54011_u16;
place!(Field::<f64>(Variant((*_32).fld1, 1), 1)) = _13 - _14;
(*_32).fld3 = Field::<i8>(Variant((*_32).fld1, 1), 3) + Field::<i8>(Variant((*_32).fld1, 1), 3);
place!(Field::<bool>(Variant((*_32).fld1, 1), 0)) = (*_32).fld0 & _5.fld0;
(*_8) = (Field::<f32>(Variant((*_32).fld1, 1), 6), Field::<i16>(Variant((*_32).fld1, 1), 4), Field::<usize>(Variant((*_32).fld1, 1), 7), _7);
(*_8).0 = Field::<f32>(Variant((*_32).fld1, 1), 6) + Field::<f32>(Variant((*_32).fld1, 1), 6);
(*_32).fld0 = Field::<bool>(Variant((*_32).fld1, 1), 0) ^ Field::<bool>(Variant((*_32).fld1, 1), 0);
(*_32).fld1 = Adt21::Variant1 { fld0: (*_32).fld0,fld1: _13,fld2: _34,fld3: (*_32).fld3,fld4: (*_8).1,fld5: 109462157_u32,fld6: (*_8).0,fld7: (*_8).2 };
(*_32).fld0 = !Field::<bool>(Variant((*_32).fld1, 1), 0);
Call(place!(Field::<i16>(Variant((*_32).fld1, 1), 4)) = core::intrinsics::bswap((*_8).1), ReturnTo(bb16), UnwindUnreachable())
}
bb52 = {
Return()
}
bb53 = {
(*_8).1 = _19.2.2 ^ _19.2.2;
_16 = &(*_8).1;
(*_8).2 = 1_usize - 10090646830521256056_usize;
_19.2.3 = (*_8).0 as i64;
_13 = _14 + _6;
(*_8).2 = 634123779574797270_usize - 10239732678533057721_usize;
(*_8).0 = 103_isize as f32;
(*_8).1 = -_19.2.2;
(*_8).3 = _7;
(*_8).3 = !_7;
(*_8).1 = !_3;
(*_8).1 = _3 - _19.2.2;
_5.fld1 = Adt21::Variant1 { fld0: _5.fld0,fld1: _6,fld2: (-9223372036854775808_isize),fld3: _5.fld3,fld4: (*_8).1,fld5: 4233843021_u32,fld6: (*_8).0,fld7: (*_8).2 };
(*_8).2 = Field::<usize>(Variant(_5.fld1, 1), 7) | Field::<usize>(Variant(_5.fld1, 1), 7);
_19.2.1 = !_19.2.3;
(*_8) = (Field::<f32>(Variant(_5.fld1, 1), 6), Field::<i16>(Variant(_5.fld1, 1), 4), Field::<usize>(Variant(_5.fld1, 1), 7), _7);
Goto(bb12)
}
bb54 = {
_55 = core::ptr::addr_of_mut!((*_31));
_84 = Field::<isize>(Variant((*_32).fld1, 1), 2) - Field::<isize>(Variant((*_32).fld1, 1), 2);
(*_55) = !_36;
_90.2 = Move(_50);
place!(Field::<i8>(Variant((*_32).fld1, 1), 3)) = Field::<isize>(Variant((*_32).fld1, 1), 2) as i8;
_70 = (*_32).fld0 | (*_32).fld0;
place!(Field::<u32>(Variant(_5.fld1, 1), 5)) = 2220848789_u32;
_90.2 = core::ptr::addr_of!(_21);
(*_8).3 = _40 >> (*_32).fld3;
place!(Field::<u32>(Variant((*_32).fld1, 1), 5)) = !599276646_u32;
place!(Field::<i16>(Variant((*_32).fld1, 1), 4)) = (*_8).1 ^ (*_8).1;
(*_32).fld3 = _81.0.fld3 >> _19.1.3;
(*_32).fld3 = (*_16) as i8;
place!(Field::<isize>(Variant((*_32).fld1, 1), 2)) = _74 as isize;
(*_8).3 = !_40;
place!(Field::<i8>(Variant((*_32).fld1, 1), 3)) = Field::<bool>(Variant((*_32).fld1, 1), 0) as i8;
_4 = _1 * _1;
_66 = -Field::<isize>(Variant((*_32).fld1, 1), 2);
(*_8).1 = Field::<i16>(Variant((*_32).fld1, 1), 4) - Field::<i16>(Variant((*_32).fld1, 1), 4);
match _7 {
0 => bb19,
1 => bb40,
104 => bb42,
_ => bb41
}
}
bb55 = {
(*_8).1 = _10 as i16;
(*_8).0 = (*_8).1 as f32;
_19.2.2 = (*_8).1;
(*_8).2 = 0_usize;
(*_8).2 = _5.fld0 as usize;
(*_8).1 = _3 | _3;
_5.fld2 = _11 >> (*_8).2;
(*_8).3 = _7 % _7;
_19.3 = Move(_17);
(*_8).0 = (*_8).2 as f32;
(*_8).0 = Field::<i32>(Variant(_5.fld1, 0), 1) as f32;
(*_8).0 = 82463214882623863492532528882930374998_i128 as f32;
(*_8).2 = 10539122240032714843_usize;
(*_8).1 = _3 ^ _19.2.2;
(*_8).1 = _3;
(*_8).2 = 1649085053150035394_usize >> Field::<i8>(Variant(_5.fld1, 0), 2);
(*_8).2 = 2_usize * 4_usize;
(*_8).0 = (*_8).2 as f32;
(*_8).2 = 13515987966445337097_usize;
(*_8).1 = _19.2.2 << _10;
Goto(bb11)
}
bb56 = {
place!(Field::<isize>(Variant((*_32).fld1, 1), 2)) = _66 & _66;
place!(Field::<i16>(Variant((*_32).fld1, 1), 4)) = (*_16);
place!(Field::<u32>(Variant((*_32).fld1, 1), 5)) = !155385346_u32;
_31 = Move(_69);
(*_55) = Field::<i8>(Variant((*_32).fld1, 1), 3) as u16;
_81.0 = Adt22 { fld0: _47,fld1: (*_32).fld1,fld2: _11,fld3: Field::<i8>(Variant((*_32).fld1, 1), 3) };
place!(Field::<i8>(Variant((*_32).fld1, 1), 3)) = (*_32).fld3 & Field::<i8>(Variant(_81.0.fld1, 1), 3);
(*_32).fld2 = _81.0.fld2 << Field::<i8>(Variant((*_32).fld1, 1), 3);
_57.0.fld2 = !(*_32).fld2;
Goto(bb57)
}
bb57 = {
Call(_102 = dump_var(Move(_40), Move(_7), Move(_84), Move(_11)), ReturnTo(bb58), UnwindUnreachable())
}
bb58 = {
Call(_102 = dump_var(Move(_28), Move(_60), Move(_1), Move(_65)), ReturnTo(bb59), UnwindUnreachable())
}
bb59 = {
Call(_102 = dump_var(Move(_41), Move(_74), Move(_47), _103), ReturnTo(bb60), UnwindUnreachable())
}
bb60 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn1(mut _1: (&'static mut char,),mut _2: bool,mut _3: f64,mut _4: bool,mut _5: bool,mut _6: i16) -> i32 {
mir! {
type RET = i32;
let _7: u16;
let _8: *mut Adt73;
let _9: &'static &'static mut (i64,);
let _10: u8;
let _11: f64;
let _12: Adt73;
let _13: *mut Adt73;
let _14: char;
let _15: bool;
let _16: u32;
let _17: f64;
let _18: i64;
let _19: bool;
let _20: bool;
let _21: &'static *mut isize;
let _22: i64;
let _23: *mut i128;
let _24: &'static char;
let _25: f64;
let _26: (&'static &'static char, i64, i16, i64);
let _27: &'static mut *mut i128;
let _28: [u128; 7];
let _29: Adt46;
let _30: [i32; 5];
let _31: &'static mut char;
let _32: [bool; 2];
let _33: ((i64,), &'static char);
let _34: *mut u16;
let _35: i16;
let _36: *mut &'static *mut isize;
let _37: *const i32;
let _38: *const Adt22;
let _39: u8;
let _40: &'static mut *mut i128;
let _41: &'static mut char;
let _42: &'static &'static mut (i64,);
let _43: &'static mut *mut i128;
let _44: isize;
let _45: u16;
let _46: isize;
let _47: u128;
let _48: [i32; 5];
let _49: f64;
let _50: [i8; 2];
let _51: (&'static mut *mut i128,);
let _52: f32;
let _53: f64;
let _54: f64;
let _55: &'static mut *mut i128;
let _56: &'static char;
let _57: *const f32;
let _58: u8;
let _59: usize;
let _60: f32;
let _61: (&'static &'static char, i64, i16, i64);
let _62: isize;
let _63: (Adt22, ((i64,), &'static char), Adt52, *mut [isize; 5]);
let _64: u16;
let _65: *const f32;
let _66: (i64,);
let _67: &'static i16;
let _68: u32;
let _69: &'static mut (i64,);
let _70: isize;
let _71: isize;
let _72: bool;
let _73: &'static u32;
let _74: ();
let _75: ();
{
_4 = _5;
Goto(bb1)
}
bb1 = {
RET = 2141769557_i32 - 991180120_i32;
_2 = _3 > _3;
_4 = !_2;
Goto(bb2)
}
bb2 = {
RET = !(-129893801_i32);
_2 = !_4;
_4 = _2;
_6 = -(-23105_i16);
_3 = 137_u8 as f64;
_4 = _2 >= _5;
_5 = _4;
_7 = 50110_u16;
_5 = _4;
_2 = !_4;
_7 = 36017_u16 >> RET;
_12.fld1 = [1653122711_u32,2638436903_u32,2662278559_u32,1798866065_u32,4039038469_u32,2576665296_u32,3056084367_u32,1011513901_u32];
_12.fld0.0 = 9101033005070790263_i64;
_11 = _3;
_8 = core::ptr::addr_of_mut!(_12);
(*_8).fld1 = [2451196578_u32,3665457101_u32,1600372494_u32,2385973098_u32,4194047652_u32,1679888767_u32,6929429_u32,995151465_u32];
(*_8).fld4 = Adt21::Variant0 { fld0: 11742851880240936961609446598017735454_u128,fld1: RET,fld2: (-87_i8) };
place!(Field::<u128>(Variant((*_8).fld4, 0), 0)) = 244798242330483238003683535804309231918_u128;
match (*_8).fld0.0 {
0 => bb3,
1 => bb4,
2 => bb5,
3 => bb6,
9101033005070790263 => bb8,
_ => bb7
}
}
bb3 = {
RET = 2141769557_i32 - 991180120_i32;
_2 = _3 > _3;
_4 = !_2;
Goto(bb2)
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
place!(Field::<i8>(Variant((*_8).fld4, 0), 2)) = 55_i8 << Field::<i32>(Variant((*_8).fld4, 0), 1);
(*_8).fld0 = ((-6645751833195667464_i64),);
place!(Field::<i8>(Variant((*_8).fld4, 0), 2)) = !(-121_i8);
place!(Field::<u128>(Variant((*_8).fld4, 0), 0)) = 265540727447796015076003223341111181380_u128 & 36050958349223601658216968109180168794_u128;
(*_8).fld0.0 = 6669274494389231575_i64 * 4634220250867003578_i64;
(*_8).fld0 = ((-8833732017430534543_i64),);
place!(Field::<i8>(Variant((*_8).fld4, 0), 2)) = (-123_i8);
place!(Field::<i8>(Variant((*_8).fld4, 0), 2)) = 77_i8;
_15 = _5;
(*_8).fld4 = Adt21::Variant0 { fld0: 144687046806197957530049807665729131781_u128,fld1: RET,fld2: (-117_i8) };
place!(Field::<i32>(Variant((*_8).fld4, 0), 1)) = RET << _12.fld0.0;
place!(Field::<u128>(Variant((*_8).fld4, 0), 0)) = !83276281097082049325355756232340041725_u128;
place!(Field::<i8>(Variant((*_8).fld4, 0), 2)) = (-29_i8) & (-77_i8);
(*_8).fld0 = ((-6929002615803241555_i64),);
(*_8).fld0 = ((-3608708911740981327_i64),);
(*_8).fld4 = Adt21::Variant0 { fld0: 173481522431704955493577158278045014039_u128,fld1: RET,fld2: (-46_i8) };
place!(Field::<i32>(Variant((*_8).fld4, 0), 1)) = RET ^ RET;
(*_8).fld4 = Adt21::Variant0 { fld0: 241411255796547968842642315423753024814_u128,fld1: RET,fld2: 1_i8 };
Call(_17 = fn2((*_8).fld0.0, Field::<i32>(Variant((*_8).fld4, 0), 1)), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
place!(Field::<u128>(Variant((*_8).fld4, 0), 0)) = Field::<i32>(Variant((*_8).fld4, 0), 1) as u128;
place!(Field::<i32>(Variant((*_8).fld4, 0), 1)) = RET - RET;
(*_8).fld0.0 = 125_i8 as i64;
place!(Field::<i8>(Variant((*_8).fld4, 0), 2)) = (-76_i8);
(*_8).fld0.0 = Field::<i32>(Variant((*_8).fld4, 0), 1) as i64;
(*_8).fld0 = (8164970368177625452_i64,);
_10 = 200_u8 >> _7;
_3 = _11 - _17;
_14 = '\u{1b9a2}';
(*_8).fld0 = ((-7038682894064217368_i64),);
place!(Field::<i32>(Variant((*_8).fld4, 0), 1)) = _7 as i32;
place!(Field::<u128>(Variant((*_8).fld4, 0), 0)) = 224356932501277709584029231608856763227_u128;
place!(Field::<i8>(Variant((*_8).fld4, 0), 2)) = (-6_i8);
match (*_8).fld0.0 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
6 => bb7,
340282366920938463456335924537703994088 => bb11,
_ => bb10
}
}
bb10 = {
RET = 2141769557_i32 - 991180120_i32;
_2 = _3 > _3;
_4 = !_2;
Goto(bb2)
}
bb11 = {
(*_8).fld1 = [1061060470_u32,2676810989_u32,2346018722_u32,2193341997_u32,2724533179_u32,256600131_u32,3387108445_u32,3576740244_u32];
place!(Field::<i8>(Variant((*_8).fld4, 0), 2)) = (-120_i8) * (-100_i8);
(*_8).fld0 = (3642429728685010836_i64,);
(*_8).fld0.0 = -(-6219261793521589477_i64);
(*_8).fld1 = [94616884_u32,1618271270_u32,1456808174_u32,3382377149_u32,1187682228_u32,748154778_u32,145568683_u32,3042828516_u32];
_18 = (*_8).fld0.0 - (*_8).fld0.0;
place!(Field::<i32>(Variant((*_8).fld4, 0), 1)) = RET;
(*_8).fld1 = [3929867654_u32,2580028676_u32,2836016984_u32,3507164960_u32,3779866957_u32,3943668858_u32,2393685687_u32,2883758082_u32];
(*_8).fld0.0 = _18;
(*_8).fld0 = (_18,);
(*_8).fld4 = Adt21::Variant0 { fld0: 326003404692942487869046793656081164365_u128,fld1: RET,fld2: 31_i8 };
place!(Field::<i32>(Variant((*_8).fld4, 0), 1)) = RET >> (*_8).fld0.0;
place!(Field::<i32>(Variant((*_8).fld4, 0), 1)) = RET ^ RET;
place!(Field::<u128>(Variant((*_8).fld4, 0), 0)) = 191717728402323986515719435623924166580_u128;
place!(Field::<i8>(Variant((*_8).fld4, 0), 2)) = 14334110037927938287_u64 as i8;
_17 = (*_8).fld0.0 as f64;
place!(Field::<i32>(Variant((*_8).fld4, 0), 1)) = RET;
(*_8).fld1 = [3865784651_u32,586882628_u32,3428553501_u32,3318980457_u32,3704224231_u32,3142011539_u32,1196205986_u32,1795960954_u32];
place!(Field::<u128>(Variant((*_8).fld4, 0), 0)) = 179447515565182073275729344988134056359_u128 * 294048055631119581177178438991848741103_u128;
Call((*_8).fld0.0 = core::intrinsics::bswap(_18), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
place!(Field::<u128>(Variant((*_8).fld4, 0), 0)) = !277857955161418801785286903947930111952_u128;
place!(Field::<u128>(Variant((*_8).fld4, 0), 0)) = 152152652887336933433673618191210694222_u128;
place!(Field::<i32>(Variant((*_8).fld4, 0), 1)) = !RET;
place!(Field::<i8>(Variant((*_8).fld4, 0), 2)) = Field::<u128>(Variant((*_8).fld4, 0), 0) as i8;
Goto(bb13)
}
bb13 = {
_4 = _5 | _5;
_7 = !24215_u16;
_16 = _6 as u32;
(*_8).fld0 = (_18,);
(*_8).fld0.0 = -_18;
Goto(bb14)
}
bb14 = {
_14 = '\u{d9b6d}';
place!(Field::<i32>(Variant((*_8).fld4, 0), 1)) = RET;
place!(Field::<i8>(Variant((*_8).fld4, 0), 2)) = -27_i8;
_20 = (*_8).fld0.0 > (*_8).fld0.0;
place!(Field::<i8>(Variant((*_8).fld4, 0), 2)) = (-21_i8) & (-1_i8);
place!(Field::<u128>(Variant((*_8).fld4, 0), 0)) = 130290701130428672986339715052468774757_u128 | 72614992726842437439354701687321844490_u128;
place!(Field::<i8>(Variant((*_8).fld4, 0), 2)) = 4_i8 - (-15_i8);
(*_8).fld0 = (_18,);
_18 = (-28511552435766689580436316578870012440_i128) as i64;
place!(Field::<u128>(Variant((*_8).fld4, 0), 0)) = 68843622007371145087468528894846175457_u128 & 133145862735607232246381309418394622690_u128;
_19 = _5 & _2;
(*_8).fld0 = (_18,);
(*_8).fld4 = Adt21::Variant0 { fld0: 295465955313801710104347883103829552120_u128,fld1: RET,fld2: (-23_i8) };
place!(Field::<i32>(Variant((*_8).fld4, 0), 1)) = RET;
_10 = 188_u8 & 143_u8;
_15 = _4;
place!(Field::<i8>(Variant((*_8).fld4, 0), 2)) = 122_i8;
place!(Field::<i32>(Variant((*_8).fld4, 0), 1)) = RET;
(*_8).fld0 = (_18,);
_8 = core::ptr::addr_of_mut!((*_8));
place!(Field::<i32>(Variant((*_8).fld4, 0), 1)) = RET - RET;
(*_8).fld1 = [_16,_16,_16,_16,_16,_16,_16,_16];
Goto(bb15)
}
bb15 = {
place!(Field::<u128>(Variant((*_8).fld4, 0), 0)) = !256185641275151956650211662242717879587_u128;
place!(Field::<u128>(Variant((*_8).fld4, 0), 0)) = 166580300006532588687496609481613853403_u128 + 85186188126488422666489998952945635281_u128;
_12.fld0.0 = _18 * _18;
place!(Field::<u128>(Variant((*_8).fld4, 0), 0)) = _14 as u128;
_14 = '\u{fdb56}';
place!(Field::<i8>(Variant(_12.fld4, 0), 2)) = 3_usize as i8;
(*_8).fld4 = Adt21::Variant0 { fld0: 150681643180176804457820697499621344753_u128,fld1: RET,fld2: 59_i8 };
place!(Field::<u128>(Variant((*_8).fld4, 0), 0)) = (-19906048869417438134338111130303025611_i128) as u128;
(*_8).fld4 = Adt21::Variant0 { fld0: 141625975868695232991751786047315592910_u128,fld1: RET,fld2: 71_i8 };
(*_8).fld0 = (_18,);
(*_8).fld0 = (_18,);
(*_8).fld1 = [_16,_16,_16,_16,_16,_16,_16,_16];
Goto(bb16)
}
bb16 = {
place!(Field::<i8>(Variant((*_8).fld4, 0), 2)) = (-108387868720627641255834456997395070475_i128) as i8;
_18 = 265003760040381366228469721798048677742_u128 as i64;
(*_8).fld0 = (_18,);
place!(Field::<u128>(Variant((*_8).fld4, 0), 0)) = _7 as u128;
_26.2 = !_6;
place!(Field::<i32>(Variant((*_8).fld4, 0), 1)) = RET << Field::<u128>(Variant((*_8).fld4, 0), 0);
place!(Field::<u128>(Variant((*_8).fld4, 0), 0)) = 67713776319936236673918083387964105268_u128;
_7 = !58979_u16;
_30 = [Field::<i32>(Variant((*_8).fld4, 0), 1),Field::<i32>(Variant((*_8).fld4, 0), 1),Field::<i32>(Variant((*_8).fld4, 0), 1),Field::<i32>(Variant((*_8).fld4, 0), 1),Field::<i32>(Variant((*_8).fld4, 0), 1)];
place!(Field::<u128>(Variant((*_8).fld4, 0), 0)) = _10 as u128;
(*_8).fld4 = Adt21::Variant0 { fld0: 312622473413660137830396728933202013814_u128,fld1: RET,fld2: 40_i8 };
Goto(bb17)
}
bb17 = {
place!(Field::<i32>(Variant((*_8).fld4, 0), 1)) = _10 as i32;
Goto(bb18)
}
bb18 = {
(*_8).fld0.0 = -_18;
(*_8).fld2 = core::ptr::addr_of!(_30);
(*_8).fld1 = [_16,_16,_16,_16,_16,_16,_16,_16];
place!(Field::<i8>(Variant((*_8).fld4, 0), 2)) = -4_i8;
(*_8).fld0 = (_18,);
place!(Field::<i32>(Variant((*_8).fld4, 0), 1)) = RET & RET;
(*_8).fld0 = (_18,);
(*_8).fld0.0 = !_18;
_7 = !39301_u16;
place!(Field::<u128>(Variant((*_8).fld4, 0), 0)) = _16 as u128;
_22 = (*_8).fld0.0 | _18;
(*_8).fld0.0 = _10 as i64;
_17 = -_3;
place!(Field::<i32>(Variant((*_8).fld4, 0), 1)) = RET;
_1.0 = &mut _14;
place!(Field::<i32>(Variant((*_8).fld4, 0), 1)) = RET >> (*_8).fld0.0;
place!(Field::<i32>(Variant((*_8).fld4, 0), 1)) = _6 as i32;
place!(Field::<i32>(Variant((*_8).fld4, 0), 1)) = RET;
(*_8).fld1 = [_16,_16,_16,_16,_16,_16,_16,_16];
place!(Field::<u128>(Variant((*_8).fld4, 0), 0)) = _15 as u128;
(*_8).fld0 = (_18,);
_28 = [Field::<u128>(Variant((*_8).fld4, 0), 0),Field::<u128>(Variant((*_8).fld4, 0), 0),Field::<u128>(Variant((*_8).fld4, 0), 0),Field::<u128>(Variant((*_8).fld4, 0), 0),Field::<u128>(Variant((*_8).fld4, 0), 0),Field::<u128>(Variant((*_8).fld4, 0), 0),Field::<u128>(Variant((*_8).fld4, 0), 0)];
(*_8).fld2 = core::ptr::addr_of!(_30);
(*_8).fld4 = Adt21::Variant0 { fld0: 145929149994670177009463253425711986285_u128,fld1: RET,fld2: (-8_i8) };
Goto(bb19)
}
bb19 = {
(*_8).fld0 = (_22,);
(*_8).fld4 = Adt21::Variant0 { fld0: 296150920951012675466015151748617117345_u128,fld1: RET,fld2: 21_i8 };
place!(Field::<i8>(Variant((*_8).fld4, 0), 2)) = 82_i8 >> _18;
(*_8).fld2 = core::ptr::addr_of!(_30);
(*_8).fld1 = [_16,_16,_16,_16,_16,_16,_16,_16];
(*_8).fld4 = Adt21::Variant0 { fld0: 148076460063161273606400528665920744300_u128,fld1: RET,fld2: (-73_i8) };
place!(Field::<i8>(Variant((*_8).fld4, 0), 2)) = _3 as i8;
(*_8).fld0.0 = _22 | _22;
(*_8).fld0 = (_18,);
(*_8).fld0 = (_22,);
(*_8).fld4 = Adt21::Variant0 { fld0: 169605273046897478713780319390569708132_u128,fld1: RET,fld2: 89_i8 };
(*_8).fld1 = [_16,_16,_16,_16,_16,_16,_16,_16];
(*_8).fld0 = (_18,);
place!(Field::<u128>(Variant((*_8).fld4, 0), 0)) = 56475203232359120978563535287697050619_u128 + 27377040103560485243244952672492376591_u128;
Goto(bb20)
}
bb20 = {
(*_8).fld0 = (_22,);
(*_8).fld4 = Adt21::Variant0 { fld0: 318344778564439930117851040406110292398_u128,fld1: RET,fld2: (-46_i8) };
Goto(bb21)
}
bb21 = {
_31 = Move(_1.0);
place!(Field::<u128>(Variant((*_8).fld4, 0), 0)) = 265316786038000718301066655110215738083_u128;
place!(Field::<i8>(Variant((*_8).fld4, 0), 2)) = -118_i8;
(*_8).fld0.0 = -_22;
(*_8).fld2 = core::ptr::addr_of!(_30);
place!(Field::<i32>(Variant((*_8).fld4, 0), 1)) = RET;
(*_8).fld0.0 = _18 & _18;
_33.0.0 = _22 + (*_8).fld0.0;
_12.fld2 = core::ptr::addr_of!(_30);
_13 = Move(_8);
_25 = _6 as f64;
_30 = [RET,Field::<i32>(Variant(_12.fld4, 0), 1),RET,RET,RET];
RET = Field::<i32>(Variant(_12.fld4, 0), 1) * Field::<i32>(Variant(_12.fld4, 0), 1);
_12.fld1 = [_16,_16,_16,_16,_16,_16,_16,_16];
place!(Field::<i32>(Variant(_12.fld4, 0), 1)) = RET >> _6;
place!(Field::<i8>(Variant(_12.fld4, 0), 2)) = 113_i8;
_35 = '\u{582ed}' as i16;
_28 = [Field::<u128>(Variant(_12.fld4, 0), 0),Field::<u128>(Variant(_12.fld4, 0), 0),Field::<u128>(Variant(_12.fld4, 0), 0),Field::<u128>(Variant(_12.fld4, 0), 0),Field::<u128>(Variant(_12.fld4, 0), 0),Field::<u128>(Variant(_12.fld4, 0), 0),Field::<u128>(Variant(_12.fld4, 0), 0)];
_3 = _25 + _17;
_8 = core::ptr::addr_of_mut!(_12);
place!(Field::<u128>(Variant((*_8).fld4, 0), 0)) = (-86298700202577887076571508167258661031_i128) as u128;
(*_8).fld2 = core::ptr::addr_of!(_30);
(*_8).fld0.0 = _18;
(*_8).fld0 = (_33.0.0,);
place!(Field::<u128>(Variant((*_8).fld4, 0), 0)) = 190743384319113197775996628777936900628_u128;
(*_8).fld0.0 = _33.0.0;
Goto(bb22)
}
bb22 = {
(*_8).fld0.0 = _22 - _22;
(*_8).fld1 = [_16,_16,_16,_16,_16,_16,_16,_16];
place!(Field::<i32>(Variant((*_8).fld4, 0), 1)) = RET - RET;
place!(Field::<u128>(Variant((*_8).fld4, 0), 0)) = !321232531356159087558586538581991220920_u128;
Call(_6 = core::intrinsics::bswap(_26.2), ReturnTo(bb23), UnwindUnreachable())
}
bb23 = {
place!(Field::<i32>(Variant((*_8).fld4, 0), 1)) = (-152799411973548067959282656460135268666_i128) as i32;
(*_8).fld0.0 = !_18;
(*_8).fld1 = [_16,_16,_16,_16,_16,_16,_16,_16];
(*_8).fld0 = _33.0;
place!(Field::<i32>(Variant((*_8).fld4, 0), 1)) = RET ^ RET;
_19 = _2;
_22 = Field::<u128>(Variant((*_8).fld4, 0), 0) as i64;
_35 = _6 << Field::<u128>(Variant((*_8).fld4, 0), 0);
place!(Field::<i32>(Variant((*_8).fld4, 0), 1)) = Field::<i8>(Variant((*_8).fld4, 0), 2) as i32;
(*_8).fld0.0 = _33.0.0;
(*_8).fld1 = [_16,_16,_16,_16,_16,_16,_16,_16];
place!(Field::<i32>(Variant((*_8).fld4, 0), 1)) = (-97_isize) as i32;
place!(Field::<u128>(Variant(_12.fld4, 0), 0)) = 301141928351833216777965056240139630580_u128 | 319374913149124167273079884095905279720_u128;
(*_8).fld2 = core::ptr::addr_of!(_30);
(*_8).fld1 = [_16,_16,_16,_16,_16,_16,_16,_16];
(*_8).fld4 = Adt21::Variant0 { fld0: 198611400177472490801354703596170764667_u128,fld1: RET,fld2: 98_i8 };
Goto(bb24)
}
bb24 = {
place!(Field::<i8>(Variant((*_8).fld4, 0), 2)) = 121_i8;
(*_8).fld1 = [_16,_16,_16,_16,_16,_16,_16,_16];
place!(Field::<u128>(Variant((*_8).fld4, 0), 0)) = 262037788722797903632809160363520657222_u128 & 107755688313460598863117718402642656096_u128;
place!(Field::<i8>(Variant((*_8).fld4, 0), 2)) = (*_8).fld0.0 as i8;
place!(Field::<u128>(Variant((*_8).fld4, 0), 0)) = 165929129086117558471419352480105209860_u128 + 334246317614400050644002814301491470626_u128;
_26.3 = _6 as i64;
(*_8).fld0.0 = _33.0.0 & _22;
place!(Field::<i32>(Variant(_12.fld4, 0), 1)) = RET * RET;
_10 = '\u{9bc81}' as u8;
place!(Field::<i32>(Variant((*_8).fld4, 0), 1)) = RET * RET;
_12.fld4 = Adt21::Variant0 { fld0: 298222510649306217018397370897617929362_u128,fld1: RET,fld2: (-49_i8) };
place!(Field::<i32>(Variant((*_8).fld4, 0), 1)) = RET ^ RET;
place!(Field::<i8>(Variant((*_8).fld4, 0), 2)) = -(-40_i8);
place!(Field::<i8>(Variant((*_8).fld4, 0), 2)) = (-16_i8) & (-94_i8);
(*_8).fld2 = core::ptr::addr_of!(_30);
_6 = _26.2 >> (*_8).fld0.0;
place!(Field::<u128>(Variant((*_8).fld4, 0), 0)) = 161239248615973581647061029511935672478_u128 | 53335440218712885909828659523555713140_u128;
place!(Field::<i8>(Variant((*_8).fld4, 0), 2)) = 25_i8 * 78_i8;
(*_8).fld2 = core::ptr::addr_of!(_30);
(*_8).fld4 = Adt21::Variant0 { fld0: 338474025020814167474898823443805354494_u128,fld1: RET,fld2: 75_i8 };
place!(Field::<u128>(Variant((*_8).fld4, 0), 0)) = 22732164551499554264458636781516446667_u128 >> (*_8).fld0.0;
place!(Field::<u128>(Variant((*_8).fld4, 0), 0)) = 126487107184386809739638749351065475827_u128 ^ 27048861174907549103663231469278451061_u128;
Goto(bb25)
}
bb25 = {
place!(Field::<u128>(Variant((*_8).fld4, 0), 0)) = 215187739049203321180423886608238400553_u128 >> Field::<i32>(Variant((*_8).fld4, 0), 1);
_45 = '\u{6dc72}' as u16;
(*_8).fld1 = [_16,_16,_16,_16,_16,_16,_16,_16];
(*_8).fld4 = Adt21::Variant0 { fld0: 147942583995370735235113431551542061472_u128,fld1: RET,fld2: 69_i8 };
place!(Field::<u128>(Variant((*_8).fld4, 0), 0)) = 191551384080396736383157265961045247713_u128 * 201380789157064930298742247741444286517_u128;
place!(Field::<u128>(Variant((*_8).fld4, 0), 0)) = 0_usize as u128;
(*_8).fld0 = _33.0;
_32 = [_5,_15];
place!(Field::<i8>(Variant((*_8).fld4, 0), 2)) = -18_i8;
_28 = [Field::<u128>(Variant((*_8).fld4, 0), 0),Field::<u128>(Variant((*_8).fld4, 0), 0),Field::<u128>(Variant((*_8).fld4, 0), 0),Field::<u128>(Variant((*_8).fld4, 0), 0),Field::<u128>(Variant((*_8).fld4, 0), 0),Field::<u128>(Variant((*_8).fld4, 0), 0),Field::<u128>(Variant((*_8).fld4, 0), 0)];
_44 = _4 as isize;
(*_8).fld0.0 = _10 as i64;
(*_8).fld1 = [_16,_16,_16,_16,_16,_16,_16,_16];
_10 = 70_u8;
Call(place!(Field::<i32>(Variant((*_8).fld4, 0), 1)) = core::intrinsics::bswap(RET), ReturnTo(bb26), UnwindUnreachable())
}
bb26 = {
place!(Field::<i32>(Variant((*_8).fld4, 0), 1)) = RET | RET;
(*_8).fld0 = _33.0;
place!(Field::<i32>(Variant((*_8).fld4, 0), 1)) = RET + RET;
_26.1 = _19 as i64;
place!(Field::<u128>(Variant((*_8).fld4, 0), 0)) = 147370335582802540234449858319467676365_u128 & 33321178437402720440443424945966277029_u128;
place!(Field::<i8>(Variant((*_8).fld4, 0), 2)) = 101_i8 >> Field::<i32>(Variant(_12.fld4, 0), 1);
_12.fld1 = [_16,_16,_16,_16,_16,_16,_16,_16];
(*_8).fld4 = Adt21::Variant0 { fld0: 12285129573543863886436116933362374093_u128,fld1: RET,fld2: (-98_i8) };
place!(Field::<i8>(Variant((*_8).fld4, 0), 2)) = 2644785896335852673_u64 as i8;
match _10 {
0 => bb27,
70 => bb29,
_ => bb28
}
}
bb27 = {
place!(Field::<i32>(Variant((*_8).fld4, 0), 1)) = _10 as i32;
Goto(bb18)
}
bb28 = {
(*_8).fld0.0 = _22 - _22;
(*_8).fld1 = [_16,_16,_16,_16,_16,_16,_16,_16];
place!(Field::<i32>(Variant((*_8).fld4, 0), 1)) = RET - RET;
place!(Field::<u128>(Variant((*_8).fld4, 0), 0)) = !321232531356159087558586538581991220920_u128;
Call(_6 = core::intrinsics::bswap(_26.2), ReturnTo(bb23), UnwindUnreachable())
}
bb29 = {
place!(Field::<i8>(Variant((*_8).fld4, 0), 2)) = -72_i8;
place!(Field::<u128>(Variant((*_8).fld4, 0), 0)) = !54044737234430938057451681665845822086_u128;
place!(Field::<i32>(Variant((*_8).fld4, 0), 1)) = _10 as i32;
place!(Field::<u128>(Variant((*_8).fld4, 0), 0)) = 150710952473569022411891762191346164585_u128 | 51564632765060764215060621369007245169_u128;
match _10 {
0 => bb2,
1 => bb30,
2 => bb31,
3 => bb32,
4 => bb33,
5 => bb34,
70 => bb36,
_ => bb35
}
}
bb30 = {
place!(Field::<i8>(Variant((*_8).fld4, 0), 2)) = 55_i8 << Field::<i32>(Variant((*_8).fld4, 0), 1);
(*_8).fld0 = ((-6645751833195667464_i64),);
place!(Field::<i8>(Variant((*_8).fld4, 0), 2)) = !(-121_i8);
place!(Field::<u128>(Variant((*_8).fld4, 0), 0)) = 265540727447796015076003223341111181380_u128 & 36050958349223601658216968109180168794_u128;
(*_8).fld0.0 = 6669274494389231575_i64 * 4634220250867003578_i64;
(*_8).fld0 = ((-8833732017430534543_i64),);
place!(Field::<i8>(Variant((*_8).fld4, 0), 2)) = (-123_i8);
place!(Field::<i8>(Variant((*_8).fld4, 0), 2)) = 77_i8;
_15 = _5;
(*_8).fld4 = Adt21::Variant0 { fld0: 144687046806197957530049807665729131781_u128,fld1: RET,fld2: (-117_i8) };
place!(Field::<i32>(Variant((*_8).fld4, 0), 1)) = RET << _12.fld0.0;
place!(Field::<u128>(Variant((*_8).fld4, 0), 0)) = !83276281097082049325355756232340041725_u128;
place!(Field::<i8>(Variant((*_8).fld4, 0), 2)) = (-29_i8) & (-77_i8);
(*_8).fld0 = ((-6929002615803241555_i64),);
(*_8).fld0 = ((-3608708911740981327_i64),);
(*_8).fld4 = Adt21::Variant0 { fld0: 173481522431704955493577158278045014039_u128,fld1: RET,fld2: (-46_i8) };
place!(Field::<i32>(Variant((*_8).fld4, 0), 1)) = RET ^ RET;
(*_8).fld4 = Adt21::Variant0 { fld0: 241411255796547968842642315423753024814_u128,fld1: RET,fld2: 1_i8 };
Call(_17 = fn2((*_8).fld0.0, Field::<i32>(Variant((*_8).fld4, 0), 1)), ReturnTo(bb9), UnwindUnreachable())
}
bb31 = {
RET = !(-129893801_i32);
_2 = !_4;
_4 = _2;
_6 = -(-23105_i16);
_3 = 137_u8 as f64;
_4 = _2 >= _5;
_5 = _4;
_7 = 50110_u16;
_5 = _4;
_2 = !_4;
_7 = 36017_u16 >> RET;
_12.fld1 = [1653122711_u32,2638436903_u32,2662278559_u32,1798866065_u32,4039038469_u32,2576665296_u32,3056084367_u32,1011513901_u32];
_12.fld0.0 = 9101033005070790263_i64;
_11 = _3;
_8 = core::ptr::addr_of_mut!(_12);
(*_8).fld1 = [2451196578_u32,3665457101_u32,1600372494_u32,2385973098_u32,4194047652_u32,1679888767_u32,6929429_u32,995151465_u32];
(*_8).fld4 = Adt21::Variant0 { fld0: 11742851880240936961609446598017735454_u128,fld1: RET,fld2: (-87_i8) };
place!(Field::<u128>(Variant((*_8).fld4, 0), 0)) = 244798242330483238003683535804309231918_u128;
match (*_8).fld0.0 {
0 => bb3,
1 => bb4,
2 => bb5,
3 => bb6,
9101033005070790263 => bb8,
_ => bb7
}
}
bb32 = {
place!(Field::<i32>(Variant((*_8).fld4, 0), 1)) = RET | RET;
(*_8).fld0 = _33.0;
place!(Field::<i32>(Variant((*_8).fld4, 0), 1)) = RET + RET;
_26.1 = _19 as i64;
place!(Field::<u128>(Variant((*_8).fld4, 0), 0)) = 147370335582802540234449858319467676365_u128 & 33321178437402720440443424945966277029_u128;
place!(Field::<i8>(Variant((*_8).fld4, 0), 2)) = 101_i8 >> Field::<i32>(Variant(_12.fld4, 0), 1);
_12.fld1 = [_16,_16,_16,_16,_16,_16,_16,_16];
(*_8).fld4 = Adt21::Variant0 { fld0: 12285129573543863886436116933362374093_u128,fld1: RET,fld2: (-98_i8) };
place!(Field::<i8>(Variant((*_8).fld4, 0), 2)) = 2644785896335852673_u64 as i8;
match _10 {
0 => bb27,
70 => bb29,
_ => bb28
}
}
bb33 = {
(*_8).fld1 = [1061060470_u32,2676810989_u32,2346018722_u32,2193341997_u32,2724533179_u32,256600131_u32,3387108445_u32,3576740244_u32];
place!(Field::<i8>(Variant((*_8).fld4, 0), 2)) = (-120_i8) * (-100_i8);
(*_8).fld0 = (3642429728685010836_i64,);
(*_8).fld0.0 = -(-6219261793521589477_i64);
(*_8).fld1 = [94616884_u32,1618271270_u32,1456808174_u32,3382377149_u32,1187682228_u32,748154778_u32,145568683_u32,3042828516_u32];
_18 = (*_8).fld0.0 - (*_8).fld0.0;
place!(Field::<i32>(Variant((*_8).fld4, 0), 1)) = RET;
(*_8).fld1 = [3929867654_u32,2580028676_u32,2836016984_u32,3507164960_u32,3779866957_u32,3943668858_u32,2393685687_u32,2883758082_u32];
(*_8).fld0.0 = _18;
(*_8).fld0 = (_18,);
(*_8).fld4 = Adt21::Variant0 { fld0: 326003404692942487869046793656081164365_u128,fld1: RET,fld2: 31_i8 };
place!(Field::<i32>(Variant((*_8).fld4, 0), 1)) = RET >> (*_8).fld0.0;
place!(Field::<i32>(Variant((*_8).fld4, 0), 1)) = RET ^ RET;
place!(Field::<u128>(Variant((*_8).fld4, 0), 0)) = 191717728402323986515719435623924166580_u128;
place!(Field::<i8>(Variant((*_8).fld4, 0), 2)) = 14334110037927938287_u64 as i8;
_17 = (*_8).fld0.0 as f64;
place!(Field::<i32>(Variant((*_8).fld4, 0), 1)) = RET;
(*_8).fld1 = [3865784651_u32,586882628_u32,3428553501_u32,3318980457_u32,3704224231_u32,3142011539_u32,1196205986_u32,1795960954_u32];
place!(Field::<u128>(Variant((*_8).fld4, 0), 0)) = 179447515565182073275729344988134056359_u128 * 294048055631119581177178438991848741103_u128;
Call((*_8).fld0.0 = core::intrinsics::bswap(_18), ReturnTo(bb12), UnwindUnreachable())
}
bb34 = {
_4 = _5 | _5;
_7 = !24215_u16;
_16 = _6 as u32;
(*_8).fld0 = (_18,);
(*_8).fld0.0 = -_18;
Goto(bb14)
}
bb35 = {
Return()
}
bb36 = {
(*_8).fld2 = core::ptr::addr_of!(_48);
(*_8).fld0.0 = _26.1 >> Field::<i8>(Variant((*_8).fld4, 0), 2);
(*_8).fld0 = (_26.1,);
_49 = _3 - _3;
_4 = _15 & _2;
place!(Field::<u128>(Variant((*_8).fld4, 0), 0)) = _33.0.0 as u128;
(*_8).fld1 = [_16,_16,_16,_16,_16,_16,_16,_16];
(*_8).fld1 = [_16,_16,_16,_16,_16,_16,_16,_16];
place!(Field::<i8>(Variant((*_8).fld4, 0), 2)) = 114_i8 ^ 121_i8;
(*_8).fld1 = [_16,_16,_16,_16,_16,_16,_16,_16];
place!(Field::<i8>(Variant((*_8).fld4, 0), 2)) = !(-119_i8);
(*_8).fld1 = [_16,_16,_16,_16,_16,_16,_16,_16];
(*_8).fld0 = (_26.1,);
_18 = -_12.fld0.0;
(*_8).fld2 = core::ptr::addr_of!(_30);
(*_8).fld2 = core::ptr::addr_of!(_48);
match _10 {
70 => bb38,
_ => bb37
}
}
bb37 = {
(*_8).fld0.0 = _22 - _22;
(*_8).fld1 = [_16,_16,_16,_16,_16,_16,_16,_16];
place!(Field::<i32>(Variant((*_8).fld4, 0), 1)) = RET - RET;
place!(Field::<u128>(Variant((*_8).fld4, 0), 0)) = !321232531356159087558586538581991220920_u128;
Call(_6 = core::intrinsics::bswap(_26.2), ReturnTo(bb23), UnwindUnreachable())
}
bb38 = {
(*_8).fld1 = [_16,_16,_16,_16,_16,_16,_16,_16];
place!(Field::<i8>(Variant((*_8).fld4, 0), 2)) = !14_i8;
match _10 {
0 => bb1,
1 => bb10,
2 => bb14,
3 => bb39,
4 => bb40,
5 => bb41,
70 => bb43,
_ => bb42
}
}
bb39 = {
(*_8).fld0.0 = _22 - _22;
(*_8).fld1 = [_16,_16,_16,_16,_16,_16,_16,_16];
place!(Field::<i32>(Variant((*_8).fld4, 0), 1)) = RET - RET;
place!(Field::<u128>(Variant((*_8).fld4, 0), 0)) = !321232531356159087558586538581991220920_u128;
Call(_6 = core::intrinsics::bswap(_26.2), ReturnTo(bb23), UnwindUnreachable())
}
bb40 = {
place!(Field::<i32>(Variant((*_8).fld4, 0), 1)) = _10 as i32;
Goto(bb18)
}
bb41 = {
place!(Field::<i32>(Variant((*_8).fld4, 0), 1)) = _10 as i32;
Goto(bb18)
}
bb42 = {
Return()
}
bb43 = {
(*_8).fld0 = (_26.1,);
Call((*_8).fld0.0 = core::intrinsics::bswap(_26.1), ReturnTo(bb44), UnwindUnreachable())
}
bb44 = {
_37 = core::ptr::addr_of!(place!(Field::<i32>(Variant((*_8).fld4, 0), 1)));
(*_8).fld1 = [_16,_16,_16,_16,_16,_16,_16,_16];
place!(Field::<i8>(Variant((*_8).fld4, 0), 2)) = (-12_i8);
place!(Field::<i32>(Variant((*_8).fld4, 0), 1)) = RET;
_45 = !_7;
_58 = !_10;
_47 = Field::<u128>(Variant((*_8).fld4, 0), 0) + Field::<u128>(Variant((*_8).fld4, 0), 0);
_45 = Field::<i8>(Variant((*_8).fld4, 0), 2) as u16;
place!(Field::<i8>(Variant((*_8).fld4, 0), 2)) = -68_i8;
_7 = _45 | _45;
(*_8).fld0.0 = _26.3 | _26.1;
(*_8).fld1 = [_16,_16,_16,_16,_16,_16,_16,_16];
_22 = !(*_8).fld0.0;
(*_8).fld2 = core::ptr::addr_of!(_30);
(*_8).fld1 = [_16,_16,_16,_16,_16,_16,_16,_16];
(*_8).fld0.0 = _58 as i64;
_7 = Field::<u128>(Variant((*_8).fld4, 0), 0) as u16;
_13 = core::ptr::addr_of_mut!((*_8));
(*_8).fld0 = (_33.0.0,);
place!(Field::<u128>(Variant((*_13).fld4, 0), 0)) = !_47;
(*_8).fld0 = _33.0;
place!(Field::<i8>(Variant((*_8).fld4, 0), 2)) = 17506031047358569026_usize as i8;
place!(Field::<i8>(Variant((*_8).fld4, 0), 2)) = -37_i8;
place!(Field::<u128>(Variant((*_13).fld4, 0), 0)) = (*_8).fld0.0 as u128;
(*_13).fld1 = [_16,_16,_16,_16,_16,_16,_16,_16];
Goto(bb45)
}
bb45 = {
place!(Field::<u128>(Variant((*_13).fld4, 0), 0)) = _47;
(*_8).fld4 = Adt21::Variant0 { fld0: _47,fld1: RET,fld2: 119_i8 };
place!(Field::<u128>(Variant((*_8).fld4, 0), 0)) = _10 as u128;
(*_13).fld1 = [_16,_16,_16,_16,_16,_16,_16,_16];
place!(Field::<i8>(Variant((*_8).fld4, 0), 2)) = 111_i8 & (-80_i8);
(*_8).fld2 = core::ptr::addr_of!(_48);
_63.0.fld1 = Adt21::Variant0 { fld0: Field::<u128>(Variant((*_8).fld4, 0), 0),fld1: Field::<i32>(Variant((*_8).fld4, 0), 1),fld2: Field::<i8>(Variant((*_8).fld4, 0), 2) };
_32 = [_4,_4];
_63.0.fld0 = _15;
_61.3 = _33.0.0 + _22;
place!(Field::<i8>(Variant((*_8).fld4, 0), 2)) = Field::<i8>(Variant(_63.0.fld1, 0), 2);
_63.0.fld2 = !8279115708629949879_u64;
(*_13).fld1 = [_16,_16,_16,_16,_16,_16,_16,_16];
place!(Field::<i32>(Variant((*_8).fld4, 0), 1)) = -RET;
(*_8).fld0.0 = _22 << _6;
RET = Field::<i32>(Variant((*_8).fld4, 0), 1) ^ Field::<i32>(Variant((*_8).fld4, 0), 1);
(*_8).fld1 = [_16,_16,_16,_16,_16,_16,_16,_16];
match _10 {
0 => bb3,
1 => bb46,
70 => bb48,
_ => bb47
}
}
bb46 = {
place!(Field::<i32>(Variant((*_8).fld4, 0), 1)) = _10 as i32;
Goto(bb18)
}
bb47 = {
place!(Field::<i8>(Variant((*_8).fld4, 0), 2)) = 55_i8 << Field::<i32>(Variant((*_8).fld4, 0), 1);
(*_8).fld0 = ((-6645751833195667464_i64),);
place!(Field::<i8>(Variant((*_8).fld4, 0), 2)) = !(-121_i8);
place!(Field::<u128>(Variant((*_8).fld4, 0), 0)) = 265540727447796015076003223341111181380_u128 & 36050958349223601658216968109180168794_u128;
(*_8).fld0.0 = 6669274494389231575_i64 * 4634220250867003578_i64;
(*_8).fld0 = ((-8833732017430534543_i64),);
place!(Field::<i8>(Variant((*_8).fld4, 0), 2)) = (-123_i8);
place!(Field::<i8>(Variant((*_8).fld4, 0), 2)) = 77_i8;
_15 = _5;
(*_8).fld4 = Adt21::Variant0 { fld0: 144687046806197957530049807665729131781_u128,fld1: RET,fld2: (-117_i8) };
place!(Field::<i32>(Variant((*_8).fld4, 0), 1)) = RET << _12.fld0.0;
place!(Field::<u128>(Variant((*_8).fld4, 0), 0)) = !83276281097082049325355756232340041725_u128;
place!(Field::<i8>(Variant((*_8).fld4, 0), 2)) = (-29_i8) & (-77_i8);
(*_8).fld0 = ((-6929002615803241555_i64),);
(*_8).fld0 = ((-3608708911740981327_i64),);
(*_8).fld4 = Adt21::Variant0 { fld0: 173481522431704955493577158278045014039_u128,fld1: RET,fld2: (-46_i8) };
place!(Field::<i32>(Variant((*_8).fld4, 0), 1)) = RET ^ RET;
(*_8).fld4 = Adt21::Variant0 { fld0: 241411255796547968842642315423753024814_u128,fld1: RET,fld2: 1_i8 };
Call(_17 = fn2((*_8).fld0.0, Field::<i32>(Variant((*_8).fld4, 0), 1)), ReturnTo(bb9), UnwindUnreachable())
}
bb48 = {
place!(Field::<i32>(Variant((*_8).fld4, 0), 1)) = RET;
(*_8).fld4 = _63.0.fld1;
(*_8).fld4 = _63.0.fld1;
(*_8).fld4 = _63.0.fld1;
(*_8).fld4 = _63.0.fld1;
(*_8).fld4 = _63.0.fld1;
Goto(bb49)
}
bb49 = {
place!(Field::<u128>(Variant(_63.0.fld1, 0), 0)) = Field::<i32>(Variant((*_8).fld4, 0), 1) as u128;
place!(Field::<i8>(Variant((*_8).fld4, 0), 2)) = Field::<i8>(Variant(_63.0.fld1, 0), 2);
(*_8).fld4 = _63.0.fld1;
place!(Field::<u128>(Variant((*_8).fld4, 0), 0)) = _47 + _47;
(*_8).fld1 = [_16,_16,_16,_16,_16,_16,_16,_16];
(*_8).fld0.0 = _18;
_60 = _10 as f32;
place!(Field::<u128>(Variant((*_8).fld4, 0), 0)) = _47;
(*_8).fld4 = Adt21::Variant0 { fld0: _47,fld1: Field::<i32>(Variant(_63.0.fld1, 0), 1),fld2: Field::<i8>(Variant(_63.0.fld1, 0), 2) };
_38 = core::ptr::addr_of!(_63.0);
(*_8).fld0 = (_18,);
(*_38).fld3 = Field::<i8>(Variant((*_38).fld1, 0), 2);
_62 = _44 & _44;
place!(Field::<u128>(Variant((*_38).fld1, 0), 0)) = Field::<u128>(Variant((*_8).fld4, 0), 0) * Field::<u128>(Variant((*_8).fld4, 0), 0);
_50 = [_63.0.fld3,Field::<i8>(Variant((*_38).fld1, 0), 2)];
place!(Field::<i32>(Variant((*_8).fld4, 0), 1)) = Field::<i32>(Variant((*_38).fld1, 0), 1);
_63.0.fld2 = 765329766458156218_u64 * 10240828541988482518_u64;
place!(Field::<i8>(Variant((*_38).fld1, 0), 2)) = Field::<i8>(Variant((*_8).fld4, 0), 2) | Field::<i8>(Variant((*_8).fld4, 0), 2);
(*_8).fld0 = _33.0;
(*_8).fld0 = (_22,);
place!(Field::<i8>(Variant((*_38).fld1, 0), 2)) = (*_38).fld3 ^ Field::<i8>(Variant((*_8).fld4, 0), 2);
(*_8).fld2 = core::ptr::addr_of!(_30);
Goto(bb50)
}
bb50 = {
Call(_74 = dump_var(Move(_35), Move(_62), Move(_10), Move(_50)), ReturnTo(bb51), UnwindUnreachable())
}
bb51 = {
Call(_74 = dump_var(Move(_4), Move(_5), Move(_19), Move(_14)), ReturnTo(bb52), UnwindUnreachable())
}
bb52 = {
Call(_74 = dump_var(Move(_2), Move(_45), Move(_16), _75), ReturnTo(bb53), UnwindUnreachable())
}
bb53 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn2(mut _1: i64,mut _2: i32) -> f64 {
mir! {
type RET = f64;
let _3: u8;
let _4: i128;
let _5: f64;
let _6: *const i128;
let _7: [i8; 2];
let _8: *mut u16;
let _9: isize;
let _10: u16;
let _11: f32;
let _12: ((i64,), &'static char);
let _13: &'static u128;
let _14: *mut isize;
let _15: &'static u32;
let _16: i16;
let _17: i16;
let _18: [char; 8];
let _19: i32;
let _20: u16;
let _21: isize;
let _22: &'static mut (i64,);
let _23: ((i64,), &'static char);
let _24: *mut Adt73;
let _25: &'static i16;
let _26: &'static char;
let _27: *const f32;
let _28: ([i128; 1], [i8; 2], *const Adt22);
let _29: bool;
let _30: isize;
let _31: *mut &'static *mut isize;
let _32: isize;
let _33: u128;
let _34: i16;
let _35: &'static mut [i8; 2];
let _36: [i32; 5];
let _37: f32;
let _38: [i128; 2];
let _39: *mut Adt75;
let _40: *mut &'static *mut isize;
let _41: *const Adt22;
let _42: i64;
let _43: bool;
let _44: isize;
let _45: isize;
let _46: u64;
let _47: isize;
let _48: char;
let _49: f64;
let _50: isize;
let _51: usize;
let _52: u64;
let _53: char;
let _54: (&'static mut char,);
let _55: ((u32,), &'static char, bool, bool);
let _56: isize;
let _57: bool;
let _58: &'static u128;
let _59: [u32; 8];
let _60: *const (f32, i16, usize, u8);
let _61: *mut &'static *mut isize;
let _62: *mut u16;
let _63: i64;
let _64: i128;
let _65: char;
let _66: [u32; 8];
let _67: *const i64;
let _68: f32;
let _69: f32;
let _70: (u8, i64, &'static mut [bool; 2]);
let _71: *mut i128;
let _72: i16;
let _73: char;
let _74: u32;
let _75: *mut &'static &'static mut (i64,);
let _76: *mut &'static &'static mut (i64,);
let _77: u64;
let _78: Adt73;
let _79: *const [i32; 5];
let _80: (f32, i16, usize, u8);
let _81: i8;
let _82: char;
let _83: ();
let _84: ();
{
_2 = (-1318672633_i32);
_3 = !204_u8;
RET = 19684020157172521671612514658412974019_i128 as f64;
_4 = 157441279278557248772025018296091053792_i128 * 115467022609034288265381916318417151234_i128;
RET = (-62_i8) as f64;
_3 = 1_u8;
RET = 9886706870086179632_usize as f64;
RET = 203089741772594004653559402256637269734_u128 as f64;
_1 = 7395860891196868409_i64 >> _2;
RET = _4 as f64;
_3 = 3_u8 & 61_u8;
RET = _3 as f64;
_1 = 222904058_u32 as i64;
_4 = (-116320084653183147952252351538156555529_i128) * (-42264725116856371202588433703508272757_i128);
RET = 12837423111094078538_u64 as f64;
_2 = (-516776698_i32);
_1 = 289068014053684727775622991962745768047_u128 as i64;
_1 = 1748275221621394153_i64;
_5 = _3 as f64;
_1 = (-8159703811617858547_i64) >> _3;
_6 = core::ptr::addr_of!(_4);
(*_6) = !167874110543036679739002229126602066974_i128;
(*_6) = 1777653473_u32 as i128;
RET = _5 - _5;
(*_6) = -(-153776946398744950629655204788857340106_i128);
(*_6) = (-158289870931562317643369330395985634930_i128) & (-57989520163135304888563770405370505265_i128);
(*_6) = -129544398297099553194843653274070940661_i128;
match _2 {
0 => bb1,
1 => bb2,
340282366920938463463374607431251434758 => bb4,
_ => bb3
}
}
bb1 = {
Return()
}
bb2 = {
Return()
}
bb3 = {
Return()
}
bb4 = {
(*_6) = !(-159050696320177080137953137949742191417_i128);
_7 = [114_i8,(-49_i8)];
(*_6) = _1 as i128;
_4 = 85063821584532369645422164890047672529_i128 ^ (-97212543884076314830870436739609248818_i128);
RET = (*_6) as f64;
Call((*_6) = fn3(Move(_6), _1, _2, _7), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_3 = 31675_u16 as u8;
_8 = core::ptr::addr_of_mut!(_10);
_9 = !(-9223372036854775808_isize);
(*_8) = 62910_u16 * 47439_u16;
(*_8) = 33883_u16 ^ 37792_u16;
(*_8) = 34539_u16 << _4;
(*_8) = 571900909_u32 as u16;
(*_8) = 64167_u16 >> _4;
Goto(bb6)
}
bb6 = {
_12.0 = (_1,);
_9 = (-87_isize) & (-9223372036854775808_isize);
(*_8) = !34372_u16;
(*_8) = 40276_u16 << _4;
(*_8) = 51757_u16 + 543_u16;
(*_8) = !28411_u16;
(*_8) = 17334_u16 + 40800_u16;
_2 = RET as i32;
Call((*_8) = fn5(Move(_8), _12.0.0, _9, _2), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_12.0 = (_1,);
_4 = 13659726469747743481_u64 as i128;
_11 = _2 as f32;
_14 = core::ptr::addr_of_mut!(_9);
Goto(bb8)
}
bb8 = {
_6 = core::ptr::addr_of!(_4);
_14 = core::ptr::addr_of_mut!((*_14));
(*_6) = -168205029478036262606788854860434038110_i128;
(*_6) = (-84281560651845735115967253868231602652_i128) & (-78258165957037402308513393094701742262_i128);
(*_14) = 111_isize & (-9223372036854775808_isize);
(*_14) = (-9223372036854775808_isize) & 9223372036854775807_isize;
(*_6) = 22425225614599250392956533437081994105_i128;
(*_14) = 9223372036854775807_isize + 47_isize;
(*_6) = !59200538613568646567686912379988404037_i128;
_11 = _12.0.0 as f32;
(*_14) = 9223372036854775807_isize ^ 9223372036854775807_isize;
(*_6) = (-57_i8) as i128;
(*_14) = 9223372036854775807_isize - 9223372036854775807_isize;
_5 = -RET;
Goto(bb9)
}
bb9 = {
_3 = 104_u8;
_5 = (*_14) as f64;
(*_14) = 9223372036854775807_isize ^ (-9223372036854775808_isize);
_8 = core::ptr::addr_of_mut!(_10);
(*_6) = true as i128;
(*_6) = -59087924297320135296597888507609289903_i128;
_14 = core::ptr::addr_of_mut!((*_14));
(*_8) = 9364_u16 & 53385_u16;
Goto(bb10)
}
bb10 = {
(*_6) = (-113738925151970585857940967288602853234_i128) + 988867973685641821342440474147884033_i128;
(*_8) = 27883_u16;
_9 = -(-58_isize);
(*_6) = (-40352716047621567400700825993230711218_i128) ^ 155162802551609341920672184721467505406_i128;
(*_8) = 19592_u16;
(*_14) = (-9223372036854775808_isize) & 117_isize;
_11 = 4288086114_u32 as f32;
(*_14) = 9223372036854775807_isize | 24_isize;
(*_14) = -(-9223372036854775808_isize);
(*_6) = !55159890528156386820036146179495352638_i128;
(*_14) = (-9223372036854775808_isize) + (-9223372036854775808_isize);
(*_8) = !43764_u16;
(*_8) = 3089_u16 + 10562_u16;
(*_14) = (-116_isize) & (-9223372036854775808_isize);
Goto(bb11)
}
bb11 = {
(*_6) = (-99584906030385621254214395364481446865_i128);
(*_14) = 43_isize | 106_isize;
match (*_6) {
0 => bb9,
1 => bb6,
2 => bb12,
3 => bb13,
4 => bb14,
5 => bb15,
240697460890552842209160212067286764591 => bb17,
_ => bb16
}
}
bb12 = {
(*_6) = (-113738925151970585857940967288602853234_i128) + 988867973685641821342440474147884033_i128;
(*_8) = 27883_u16;
_9 = -(-58_isize);
(*_6) = (-40352716047621567400700825993230711218_i128) ^ 155162802551609341920672184721467505406_i128;
(*_8) = 19592_u16;
(*_14) = (-9223372036854775808_isize) & 117_isize;
_11 = 4288086114_u32 as f32;
(*_14) = 9223372036854775807_isize | 24_isize;
(*_14) = -(-9223372036854775808_isize);
(*_6) = !55159890528156386820036146179495352638_i128;
(*_14) = (-9223372036854775808_isize) + (-9223372036854775808_isize);
(*_8) = !43764_u16;
(*_8) = 3089_u16 + 10562_u16;
(*_14) = (-116_isize) & (-9223372036854775808_isize);
Goto(bb11)
}
bb13 = {
Return()
}
bb14 = {
_6 = core::ptr::addr_of!(_4);
_14 = core::ptr::addr_of_mut!((*_14));
(*_6) = -168205029478036262606788854860434038110_i128;
(*_6) = (-84281560651845735115967253868231602652_i128) & (-78258165957037402308513393094701742262_i128);
(*_14) = 111_isize & (-9223372036854775808_isize);
(*_14) = (-9223372036854775808_isize) & 9223372036854775807_isize;
(*_6) = 22425225614599250392956533437081994105_i128;
(*_14) = 9223372036854775807_isize + 47_isize;
(*_6) = !59200538613568646567686912379988404037_i128;
_11 = _12.0.0 as f32;
(*_14) = 9223372036854775807_isize ^ 9223372036854775807_isize;
(*_6) = (-57_i8) as i128;
(*_14) = 9223372036854775807_isize - 9223372036854775807_isize;
_5 = -RET;
Goto(bb9)
}
bb15 = {
Return()
}
bb16 = {
_3 = 31675_u16 as u8;
_8 = core::ptr::addr_of_mut!(_10);
_9 = !(-9223372036854775808_isize);
(*_8) = 62910_u16 * 47439_u16;
(*_8) = 33883_u16 ^ 37792_u16;
(*_8) = 34539_u16 << _4;
(*_8) = 571900909_u32 as u16;
(*_8) = 64167_u16 >> _4;
Goto(bb6)
}
bb17 = {
(*_6) = !27827242660757161291332630041963316615_i128;
(*_6) = (-3295020074028054715686029931270437193_i128) >> (*_8);
(*_14) = 6630479363138193824_u64 as isize;
_12.0.0 = -_1;
_18 = ['\u{49a7d}','\u{d28c7}','\u{a2750}','\u{16e4e}','\u{a3009}','\u{eb62d}','\u{6bbd5}','\u{fa45b}'];
_3 = 128_u8 ^ 204_u8;
(*_6) = !68163750311847937598942613490661229076_i128;
(*_8) = 17905_u16 - 16477_u16;
(*_14) = (-9223372036854775808_isize);
(*_14) = 9223372036854775807_isize << _3;
(*_8) = 2538_u16;
(*_14) = 9223372036854775807_isize * 9223372036854775807_isize;
(*_14) = (-9223372036854775808_isize) - 9223372036854775807_isize;
(*_6) = 145072771796696912068311718011432511321_i128;
(*_8) = 6057297262382135213_u64 as u16;
_17 = _1 as i16;
_12.0.0 = _1 & _1;
(*_8) = !44418_u16;
(*_14) = (-9223372036854775808_isize) - (-66_isize);
(*_6) = (-109774539642632761270254751558869222591_i128);
(*_14) = !(-9223372036854775808_isize);
_5 = RET + RET;
(*_6) = 149107742429854577103595690407156500822_i128 << (*_8);
(*_14) = (-9223372036854775808_isize);
(*_8) = 45556_u16 * 52819_u16;
(*_6) = 65782270624569269119800852828033458794_i128 & 18436637500409906095499166552179921543_i128;
(*_14) = 3_isize << (*_6);
Call(_5 = core::intrinsics::fmaf64(RET, RET, RET), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
(*_6) = 75_i8 as i128;
RET = (*_8) as f64;
(*_6) = (-165027339750805248526194230224044526981_i128);
match (*_6) {
175255027170133214937180377207723684475 => bb20,
_ => bb19
}
}
bb19 = {
_12.0 = (_1,);
_4 = 13659726469747743481_u64 as i128;
_11 = _2 as f32;
_14 = core::ptr::addr_of_mut!(_9);
Goto(bb8)
}
bb20 = {
(*_14) = 10_isize * 9223372036854775807_isize;
_1 = _12.0.0 + _12.0.0;
(*_8) = 923_u16 >> _12.0.0;
_5 = RET + RET;
(*_6) = (-86782150970137967860291246621265680918_i128) ^ (-109131008502862694735880013107215704865_i128);
_2 = 1823833104_i32 >> (*_8);
(*_14) = 13872667942575758888_u64 as isize;
_23.0.0 = !_12.0.0;
_16 = _17;
Goto(bb21)
}
bb21 = {
_27 = core::ptr::addr_of!(_11);
_2 = -(-1892619586_i32);
(*_6) = 57515369398250738597285311211982308739_i128 | (-152201612475526379756034647461384124367_i128);
_5 = -RET;
(*_8) = 40094_u16 << _1;
(*_27) = 21974240524704710444062530243786004829_u128 as f32;
(*_8) = 49203_u16 >> _1;
(*_6) = 132444860854586285426262291330635582239_i128;
(*_14) = -9223372036854775807_isize;
(*_8) = 33572_u16;
(*_6) = 72773793733686141524935712281763127591_i128;
_17 = _16 * _16;
_7 = [(-25_i8),(-109_i8)];
(*_27) = (*_6) as f32;
(*_14) = _2 as isize;
(*_8) = !2675_u16;
(*_27) = (*_6) as f32;
(*_27) = 316407491740593291766179886644536274020_u128 as f32;
(*_8) = 28975_u16;
(*_6) = -(-86202600638783946740828666006762949816_i128);
(*_6) = 87325359104210524936285653110692950230_i128 * (-141130967481165013736517059070124958397_i128);
(*_27) = _17 as f32;
Goto(bb22)
}
bb22 = {
(*_6) = 2247683472506739035_u64 as i128;
_8 = core::ptr::addr_of_mut!((*_8));
(*_6) = _3 as i128;
Goto(bb23)
}
bb23 = {
(*_6) = -65231195863880459653829741210580781716_i128;
(*_6) = !163664951856491304496395791072094701720_i128;
(*_27) = 10412714392854960083_u64 as f32;
(*_8) = '\u{46dc7}' as u16;
(*_27) = 3443515647912899004_u64 as f32;
_8 = core::ptr::addr_of_mut!((*_8));
(*_6) = 151573399283171358864484341117542305889_i128 * (-15007554164241443221076985103352090713_i128);
_12.0 = (_23.0.0,);
(*_14) = (-120_i8) as isize;
(*_8) = !23457_u16;
(*_8) = !24674_u16;
_5 = 5_usize as f64;
(*_8) = _3 as u16;
(*_8) = 22650_u16;
match (*_8) {
0 => bb14,
22650 => bb25,
_ => bb24
}
}
bb24 = {
Return()
}
bb25 = {
(*_14) = 84_isize;
(*_14) = (-9223372036854775808_isize);
(*_14) = (-9223372036854775808_isize) ^ 9223372036854775807_isize;
(*_27) = _2 as f32;
(*_6) = 109549230835159465791273370564957268180_i128;
(*_27) = (*_6) as f32;
Goto(bb26)
}
bb26 = {
(*_14) = (-9223372036854775808_isize) & 92_isize;
_33 = 151841713823498332442073244020251827251_u128;
_16 = _17 - _17;
(*_14) = -9223372036854775807_isize;
(*_6) = _16 as i128;
(*_8) = 28043_u16 & 47419_u16;
_16 = _17 | _17;
(*_14) = 9223372036854775807_isize;
(*_14) = 9223372036854775807_isize & 19_isize;
(*_14) = !(-9223372036854775808_isize);
_14 = core::ptr::addr_of_mut!((*_14));
(*_27) = _3 as f32;
(*_14) = 9223372036854775807_isize >> (*_6);
_25 = &_16;
_43 = (*_8) != (*_8);
(*_27) = 11816197258224760315_usize as f32;
Goto(bb27)
}
bb27 = {
(*_6) = 129584258695339381449294080047036976211_i128 | 155933373912254429704985033203124202284_i128;
_22 = &mut _12.0;
(*_6) = (-92268327005375689815585190378927328625_i128) - (-26301279554594767930362343281245886366_i128);
(*_8) = 40594_u16 >> (*_14);
(*_22).0 = _1 >> _16;
(*_14) = (-9223372036854775808_isize) + (-9223372036854775808_isize);
_21 = (*_6) as isize;
(*_22) = (_1,);
match _33 {
151841713823498332442073244020251827251 => bb29,
_ => bb28
}
}
bb28 = {
_12.0 = (_1,);
_9 = (-87_isize) & (-9223372036854775808_isize);
(*_8) = !34372_u16;
(*_8) = 40276_u16 << _4;
(*_8) = 51757_u16 + 543_u16;
(*_8) = !28411_u16;
(*_8) = 17334_u16 + 40800_u16;
_2 = RET as i32;
Call((*_8) = fn5(Move(_8), _12.0.0, _9, _2), ReturnTo(bb7), UnwindUnreachable())
}
bb29 = {
(*_8) = 478_u16 * 40940_u16;
match _33 {
0 => bb2,
151841713823498332442073244020251827251 => bb31,
_ => bb30
}
}
bb30 = {
_3 = 31675_u16 as u8;
_8 = core::ptr::addr_of_mut!(_10);
_9 = !(-9223372036854775808_isize);
(*_8) = 62910_u16 * 47439_u16;
(*_8) = 33883_u16 ^ 37792_u16;
(*_8) = 34539_u16 << _4;
(*_8) = 571900909_u32 as u16;
(*_8) = 64167_u16 >> _4;
Goto(bb6)
}
bb31 = {
(*_22).0 = _23.0.0;
_19 = _2 | _2;
Goto(bb32)
}
bb32 = {
(*_22) = (_23.0.0,);
(*_8) = !59906_u16;
(*_27) = (*_14) as f32;
_16 = !_17;
(*_6) = (-138947066255598869137555542392158197513_i128);
_50 = (*_14) & (*_14);
_7 = [41_i8,66_i8];
(*_14) = _21;
_19 = _2 | _2;
(*_14) = _50;
_48 = '\u{ef7d1}';
_49 = _33 as f64;
_23.1 = &_48;
(*_6) = (-36986814848424649258083460490294759022_i128) << _19;
_35 = &mut _7;
(*_8) = _1 as u16;
(*_6) = (*_8) as i128;
(*_14) = _50 & _50;
(*_22) = _23.0;
_5 = (*_6) as f64;
_38 = [(*_6),(*_6)];
(*_35) = [72_i8,94_i8];
(*_22).0 = (*_14) as i64;
(*_14) = _50 - _21;
Call((*_6) = core::intrinsics::bswap((-103054635533656284319258955679983267418_i128)), ReturnTo(bb33), UnwindUnreachable())
}
bb33 = {
_33 = !119183769197632624403912748282430787749_u128;
(*_35) = [95_i8,89_i8];
(*_22) = (_23.0.0,);
_13 = &_33;
(*_35) = [45_i8,7_i8];
_16 = _17;
(*_27) = (*_13) as f32;
_55.0 = (2675399295_u32,);
(*_6) = (-63520366728291597865360642649438548933_i128) + (-86032098723609530097261970116729058681_i128);
_25 = &_17;
(*_6) = (-24_i8) as i128;
_5 = RET + _49;
_29 = (*_14) == (*_14);
(*_6) = (-127878534263774851401872186779650041586_i128) - (-87337983788008971690086819697158003578_i128);
(*_22).0 = _1 << (*_8);
(*_22).0 = _1 >> (*_8);
(*_35) = [86_i8,(-70_i8)];
Goto(bb34)
}
bb34 = {
(*_27) = (*_22).0 as f32;
_48 = '\u{bd665}';
(*_35) = [(-104_i8),(-79_i8)];
(*_14) = !_21;
(*_22) = _23.0;
(*_8) = 54246_u16 * 36332_u16;
_8 = core::ptr::addr_of_mut!((*_8));
_25 = &_16;
_15 = &_55.0.0;
(*_27) = 17006320936917574736_u64 as f32;
(*_22) = (_1,);
(*_22) = (_23.0.0,);
(*_22).0 = _1 & _1;
(*_35) = [(-9_i8),(-42_i8)];
_54.0 = &mut _48;
(*_8) = 26864_u16 | 32761_u16;
(*_27) = (-74_i8) as f32;
_33 = 198642665732206204149267806058946954682_u128;
(*_27) = (*_22).0 as f32;
(*_14) = _50 * _50;
(*_8) = !19487_u16;
(*_14) = _50 | _50;
_55.2 = !_29;
(*_35) = [(-62_i8),(-121_i8)];
_28.1 = [81_i8,47_i8];
match _33 {
0 => bb15,
1 => bb28,
2 => bb25,
3 => bb29,
198642665732206204149267806058946954682 => bb35,
_ => bb19
}
}
bb35 = {
_34 = _3 as i16;
(*_27) = (*_14) as f32;
(*_22) = (_1,);
(*_8) = _55.2 as u16;
(*_27) = (*_6) as f32;
(*_27) = 10903315246487814692_u64 as f32;
_42 = (*_22).0 << _1;
(*_22).0 = _23.0.0 * _42;
(*_22).0 = (*_15) as i64;
(*_6) = (-62490361791808168746482753767371386540_i128) - 64690765360484466242973612418307286338_i128;
(*_8) = !59864_u16;
_47 = !(*_14);
(*_8) = !17935_u16;
_63 = !_42;
(*_22) = _23.0;
_62 = core::ptr::addr_of_mut!((*_8));
_26 = Move(_23.1);
(*_35) = [(-35_i8),5_i8];
_28.0 = [(*_6)];
(*_27) = (*_62) as f32;
Goto(bb36)
}
bb36 = {
_44 = (*_15) as isize;
(*_22).0 = -_63;
_20 = (*_8);
(*_27) = 1470281093574671508_usize as f32;
(*_6) = 44853964421071903698604480404100337586_i128 + 166176302487447938815358194911008747144_i128;
(*_22).0 = _63 + _1;
(*_8) = _20 + _20;
_18 = ['\u{10c3ab}','\u{b6e75}','\u{d5305}','\u{6d5e0}','\u{87547}','\u{28e20}','\u{1532d}','\u{3eaa4}'];
(*_27) = (*_25) as f32;
_6 = core::ptr::addr_of!((*_6));
(*_6) = _2 as i128;
(*_6) = 55369735716983493038145992564285396243_i128;
_52 = 14447900205303213034_u64 ^ 1809307119758893863_u64;
(*_22).0 = (*_27) as i64;
(*_8) = _20;
_55.0.0 = !1774171755_u32;
(*_8) = _20 | _20;
_28.1 = [65_i8,7_i8];
(*_22) = (_42,);
(*_22) = _23.0;
(*_35) = [(-96_i8),(-89_i8)];
(*_6) = 77032791878056176363403597595159337705_i128 | (-56227486964117027902740033111496320925_i128);
(*_22).0 = _43 as i64;
_30 = (*_14) * (*_14);
(*_27) = (*_8) as f32;
(*_14) = _30;
_25 = &_34;
match _33 {
0 => bb13,
1 => bb28,
2 => bb5,
3 => bb37,
198642665732206204149267806058946954682 => bb39,
_ => bb38
}
}
bb37 = {
(*_6) = (-99584906030385621254214395364481446865_i128);
(*_14) = 43_isize | 106_isize;
match (*_6) {
0 => bb9,
1 => bb6,
2 => bb12,
3 => bb13,
4 => bb14,
5 => bb15,
240697460890552842209160212067286764591 => bb17,
_ => bb16
}
}
bb38 = {
_33 = !119183769197632624403912748282430787749_u128;
(*_35) = [95_i8,89_i8];
(*_22) = (_23.0.0,);
_13 = &_33;
(*_35) = [45_i8,7_i8];
_16 = _17;
(*_27) = (*_13) as f32;
_55.0 = (2675399295_u32,);
(*_6) = (-63520366728291597865360642649438548933_i128) + (-86032098723609530097261970116729058681_i128);
_25 = &_17;
(*_6) = (-24_i8) as i128;
_5 = RET + _49;
_29 = (*_14) == (*_14);
(*_6) = (-127878534263774851401872186779650041586_i128) - (-87337983788008971690086819697158003578_i128);
(*_22).0 = _1 << (*_8);
(*_22).0 = _1 >> (*_8);
(*_35) = [86_i8,(-70_i8)];
Goto(bb34)
}
bb39 = {
(*_22).0 = !_23.0.0;
(*_35) = [(-48_i8),30_i8];
(*_6) = 49417803248322767009727925946945218874_i128 * (-94607589127052983811125479497062572680_i128);
_55.3 = (*_22).0 <= _63;
_32 = -(*_14);
(*_8) = '\u{959a3}' as u16;
(*_14) = -_30;
(*_27) = (*_25) as f32;
_64 = (*_6) * (*_6);
(*_14) = !_50;
_59 = [_55.0.0,_55.0.0,_55.0.0,_55.0.0,_55.0.0,_55.0.0,_55.0.0,_55.0.0];
_43 = _29 == _55.3;
(*_14) = _30 | _32;
(*_14) = _30 | _32;
(*_6) = -_64;
(*_6) = (*_14) as i128;
(*_14) = _32 | _32;
(*_22) = (_42,);
_65 = '\u{78e58}';
(*_22).0 = _9 as i64;
_50 = (*_14) | (*_14);
(*_35) = [(-123_i8),55_i8];
_47 = (*_14) << (*_6);
_55.1 = &_65;
_59 = [_55.0.0,_55.0.0,_55.0.0,_55.0.0,_55.0.0,_55.0.0,_55.0.0,_55.0.0];
(*_6) = !_64;
(*_6) = _64 >> _9;
Goto(bb40)
}
bb40 = {
(*_35) = [34_i8,(-31_i8)];
match _33 {
0 => bb23,
1 => bb35,
2 => bb26,
198642665732206204149267806058946954682 => bb41,
_ => bb21
}
}
bb41 = {
_59 = [_55.0.0,_55.0.0,_55.0.0,_55.0.0,_55.0.0,_55.0.0,_55.0.0,_55.0.0];
(*_22) = (_1,);
_28.0 = [(*_6)];
_37 = -(*_27);
_67 = core::ptr::addr_of!(_42);
(*_27) = _37 + _37;
match _33 {
0 => bb18,
198642665732206204149267806058946954682 => bb43,
_ => bb42
}
}
bb42 = {
(*_22) = (_23.0.0,);
(*_8) = !59906_u16;
(*_27) = (*_14) as f32;
_16 = !_17;
(*_6) = (-138947066255598869137555542392158197513_i128);
_50 = (*_14) & (*_14);
_7 = [41_i8,66_i8];
(*_14) = _21;
_19 = _2 | _2;
(*_14) = _50;
_48 = '\u{ef7d1}';
_49 = _33 as f64;
_23.1 = &_48;
(*_6) = (-36986814848424649258083460490294759022_i128) << _19;
_35 = &mut _7;
(*_8) = _1 as u16;
(*_6) = (*_8) as i128;
(*_14) = _50 & _50;
(*_22) = _23.0;
_5 = (*_6) as f64;
_38 = [(*_6),(*_6)];
(*_35) = [72_i8,94_i8];
(*_22).0 = (*_14) as i64;
(*_14) = _50 - _21;
Call((*_6) = core::intrinsics::bswap((-103054635533656284319258955679983267418_i128)), ReturnTo(bb33), UnwindUnreachable())
}
bb43 = {
(*_67) = (*_22).0 + _1;
(*_8) = !_20;
match _33 {
0 => bb35,
198642665732206204149267806058946954682 => bb44,
_ => bb17
}
}
bb44 = {
_32 = (*_14);
Goto(bb45)
}
bb45 = {
(*_22).0 = (*_8) as i64;
(*_14) = _32 << (*_6);
_71 = core::ptr::addr_of_mut!((*_6));
(*_35) = [0_i8,23_i8];
(*_6) = !_64;
_57 = (*_67) < _42;
(*_22).0 = (*_67);
_21 = -_32;
_23 = ((*_22), Move(_55.1));
(*_8) = _20 + _20;
_4 = _64 * _64;
(*_22) = ((*_67),);
(*_67) = (*_22).0;
_33 = 45760053740326339682526548595108020385_u128;
(*_8) = !_20;
_78.fld0.0 = (*_6) as i64;
(*_27) = _37 + _37;
(*_22) = _23.0;
(*_14) = -_47;
(*_22).0 = -_42;
match _33 {
45760053740326339682526548595108020385 => bb47,
_ => bb46
}
}
bb46 = {
_44 = (*_15) as isize;
(*_22).0 = -_63;
_20 = (*_8);
(*_27) = 1470281093574671508_usize as f32;
(*_6) = 44853964421071903698604480404100337586_i128 + 166176302487447938815358194911008747144_i128;
(*_22).0 = _63 + _1;
(*_8) = _20 + _20;
_18 = ['\u{10c3ab}','\u{b6e75}','\u{d5305}','\u{6d5e0}','\u{87547}','\u{28e20}','\u{1532d}','\u{3eaa4}'];
(*_27) = (*_25) as f32;
_6 = core::ptr::addr_of!((*_6));
(*_6) = _2 as i128;
(*_6) = 55369735716983493038145992564285396243_i128;
_52 = 14447900205303213034_u64 ^ 1809307119758893863_u64;
(*_22).0 = (*_27) as i64;
(*_8) = _20;
_55.0.0 = !1774171755_u32;
(*_8) = _20 | _20;
_28.1 = [65_i8,7_i8];
(*_22) = (_42,);
(*_22) = _23.0;
(*_35) = [(-96_i8),(-89_i8)];
(*_6) = 77032791878056176363403597595159337705_i128 | (-56227486964117027902740033111496320925_i128);
(*_22).0 = _43 as i64;
_30 = (*_14) * (*_14);
(*_27) = (*_8) as f32;
(*_14) = _30;
_25 = &_34;
match _33 {
0 => bb13,
1 => bb28,
2 => bb5,
3 => bb37,
198642665732206204149267806058946954682 => bb39,
_ => bb38
}
}
bb47 = {
_58 = &_33;
_78.fld4 = Adt21::Variant0 { fld0: (*_58),fld1: _19,fld2: 35_i8 };
_25 = &_16;
(*_8) = _20 >> (*_22).0;
_73 = _65;
(*_14) = _65 as isize;
Goto(bb48)
}
bb48 = {
_38 = [(*_6),(*_6)];
_53 = _73;
_79 = core::ptr::addr_of!(_36);
(*_35) = [88_i8,(-27_i8)];
_80.1 = !(*_25);
_19 = -Field::<i32>(Variant(_78.fld4, 0), 1);
(*_22).0 = (*_67) | (*_67);
(*_35) = [(-81_i8),72_i8];
(*_35) = [(-22_i8),79_i8];
(*_6) = _64 >> (*_67);
_73 = _65;
(*_79) = [_19,Field::<i32>(Variant(_78.fld4, 0), 1),Field::<i32>(Variant(_78.fld4, 0), 1),Field::<i32>(Variant(_78.fld4, 0), 1),Field::<i32>(Variant(_78.fld4, 0), 1)];
_69 = -(*_27);
Goto(bb49)
}
bb49 = {
(*_22) = _23.0;
_81 = !53_i8;
_55.1 = &_65;
(*_22) = ((*_67),);
(*_22) = _23.0;
Goto(bb50)
}
bb50 = {
Call(_83 = dump_var(Move(_64), Move(_50), Move(_44), Move(_20)), ReturnTo(bb51), UnwindUnreachable())
}
bb51 = {
Call(_83 = dump_var(Move(_16), Move(_59), Move(_17), Move(_10)), ReturnTo(bb52), UnwindUnreachable())
}
bb52 = {
Call(_83 = dump_var(Move(_33), Move(_30), Move(_53), Move(_1)), ReturnTo(bb53), UnwindUnreachable())
}
bb53 = {
Call(_83 = dump_var(Move(_7), Move(_42), Move(_2), Move(_36)), ReturnTo(bb54), UnwindUnreachable())
}
bb54 = {
Call(_83 = dump_var(Move(_57), _84, _84, _84), ReturnTo(bb55), UnwindUnreachable())
}
bb55 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn3(mut _1: *const i128,mut _2: i64,mut _3: i32,mut _4: [i8; 2]) -> i128 {
mir! {
type RET = i128;
let _5: *const i64;
let _6: isize;
let _7: [u32; 8];
let _8: &'static u32;
let _9: isize;
let _10: ();
let _11: ();
{
RET = 157914916874671396295372288869916821310_i128;
_1 = core::ptr::addr_of!(RET);
(*_1) = !8815862172315937371871234841178655924_i128;
(*_1) = 50437509282228477874052355373763241403_i128;
(*_1) = 75472415005275348452052591399526729090_i128 + (-62471476567829356696100003382266427419_i128);
(*_1) = 30_u8 as i128;
(*_1) = 40948_u16 as i128;
(*_1) = 2_usize as i128;
(*_1) = !148700427603358682846035677049437425471_i128;
(*_1) = 157920746181939805344869076941080659545_i128;
(*_1) = 13286169365830149424414556149944263392_u128 as i128;
(*_1) = (-24450818364093602377303388001746633062_i128);
(*_1) = !147293046836392243595138090475748041615_i128;
(*_1) = -78493429344737592103449048294778980554_i128;
(*_1) = 164434158339230568658011892094775003922_i128 | (-12325360108990299211159306749777577804_i128);
_3 = (-1723072685_i32) & (-69276803_i32);
Call((*_1) = fn4(Move(_1), _2, _2, _4, _3, _4, _2, _2, _2, _3), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_5 = core::ptr::addr_of!(_2);
Goto(bb2)
}
bb2 = {
Call(_10 = dump_var(Move(_2), _11, _11, _11), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn4(mut _1: *const i128,mut _2: i64,mut _3: i64,mut _4: [i8; 2],mut _5: i32,mut _6: [i8; 2],mut _7: i64,mut _8: i64,mut _9: i64,mut _10: i32) -> i128 {
mir! {
type RET = i128;
let _11: char;
let _12: f32;
let _13: &'static i64;
let _14: (u32,);
let _15: i128;
let _16: f32;
let _17: f32;
let _18: Adt59;
let _19: (u8, i64, &'static mut [bool; 2]);
let _20: u16;
let _21: f32;
let _22: (u64, u128);
let _23: char;
let _24: *mut &'static &'static mut (i64,);
let _25: *mut [isize; 5];
let _26: [i16; 6];
let _27: &'static mut (i64,);
let _28: f32;
let _29: [i8; 3];
let _30: *const [i32; 5];
let _31: i128;
let _32: ();
let _33: ();
{
RET = (-94376163337055249997692122732080442144_i128) << _5;
_1 = core::ptr::addr_of!(RET);
(*_1) = (-165800082436603564588356220439737571884_i128);
_7 = -_3;
(*_1) = 112962065479809864861855037672579575021_i128 - (-126309490658289953561047766465114414494_i128);
_6 = [19_i8,84_i8];
(*_1) = (-9071641029123537645145739766486983418_i128) >> _2;
_11 = '\u{4e151}';
_3 = 29_u8 as i64;
_8 = !_2;
_9 = -_2;
_4 = _6;
_2 = _8 * _8;
(*_1) = (-34950632328580156401357942821365904258_i128) << _8;
(*_1) = 2701245439798429157439066344413689582_i128 >> _2;
_1 = core::ptr::addr_of!((*_1));
(*_1) = 94083702289703476656627201872235145997_i128 - 19271460245768125676158882223728487347_i128;
(*_1) = -(-73786636669416934754331680200523841404_i128);
_10 = _5;
Goto(bb1)
}
bb1 = {
(*_1) = 71937714157795082332811297971867983551_i128 + (-5550475850703782714561739128249811219_i128);
(*_1) = 137178229835090618058984110517304129274_i128 & (-111914421515035242543437253913178968290_i128);
(*_1) = (-65134284607295803082678677750944687887_i128) ^ (-47077126203420947123553176165897204340_i128);
_6 = _4;
(*_1) = 51445422423550130201558876507709335803_u128 as i128;
(*_1) = (-4223180821687349553898992044395534083_i128);
(*_1) = (-25935406702828806902971500560471861740_i128) * 86972786722928884372791892109893885323_i128;
_3 = _8 ^ _7;
(*_1) = (-65443936673620830306777789000014746320_i128) * (-67398629615748810880195732179185207795_i128);
_6 = [100_i8,106_i8];
(*_1) = 105090256939844516651824808272363929868_i128;
(*_1) = (-7155_i16) as i128;
_10 = _5 + _5;
(*_1) = 2_usize as i128;
(*_1) = !(-40713070347330948016272516554992897768_i128);
(*_1) = (-12627822784598469866344256207904161213_i128) | 118822727349560317853405996447606394736_i128;
_14.0 = 455744075_u32;
(*_1) = 64172632849060086133635981705478420516_i128 + 139055766381004348766694393364190136826_i128;
_4 = _6;
_2 = -_9;
_13 = &_7;
(*_1) = (-66414476962523872556407306548592493151_i128) >> (*_13);
(*_1) = 132606879422435263062761482987230459801_i128 + 134071619520893587374145801244200397388_i128;
_7 = _8 << _3;
match _14.0 {
0 => bb2,
1 => bb3,
455744075 => bb5,
_ => bb4
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
_3 = _2;
_11 = '\u{b8f7d}';
(*_1) = -28440883714868061583419189987944161871_i128;
Goto(bb6)
}
bb6 = {
(*_1) = !(-34889121859581937174863605453795748424_i128);
(*_1) = (-10023184229659130176174577686475559247_i128) * (-30790115273175130867536300454141409951_i128);
(*_1) = -(-13355665891520813591535988595825059777_i128);
_14 = (2487691926_u32,);
_9 = _2;
(*_1) = (-89771518395117783644778084250123366741_i128) - 107699565350196963345307335461154338079_i128;
(*_1) = (-45768636324860855944168001763744255304_i128) & (-79150346170533031703375556370639331157_i128);
match _14.0 {
0 => bb2,
1 => bb7,
2 => bb8,
3 => bb9,
4 => bb10,
2487691926 => bb12,
_ => bb11
}
}
bb7 = {
_3 = _2;
_11 = '\u{b8f7d}';
(*_1) = -28440883714868061583419189987944161871_i128;
Goto(bb6)
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
(*_1) = 71937714157795082332811297971867983551_i128 + (-5550475850703782714561739128249811219_i128);
(*_1) = 137178229835090618058984110517304129274_i128 & (-111914421515035242543437253913178968290_i128);
(*_1) = (-65134284607295803082678677750944687887_i128) ^ (-47077126203420947123553176165897204340_i128);
_6 = _4;
(*_1) = 51445422423550130201558876507709335803_u128 as i128;
(*_1) = (-4223180821687349553898992044395534083_i128);
(*_1) = (-25935406702828806902971500560471861740_i128) * 86972786722928884372791892109893885323_i128;
_3 = _8 ^ _7;
(*_1) = (-65443936673620830306777789000014746320_i128) * (-67398629615748810880195732179185207795_i128);
_6 = [100_i8,106_i8];
(*_1) = 105090256939844516651824808272363929868_i128;
(*_1) = (-7155_i16) as i128;
_10 = _5 + _5;
(*_1) = 2_usize as i128;
(*_1) = !(-40713070347330948016272516554992897768_i128);
(*_1) = (-12627822784598469866344256207904161213_i128) | 118822727349560317853405996447606394736_i128;
_14.0 = 455744075_u32;
(*_1) = 64172632849060086133635981705478420516_i128 + 139055766381004348766694393364190136826_i128;
_4 = _6;
_2 = -_9;
_13 = &_7;
(*_1) = (-66414476962523872556407306548592493151_i128) >> (*_13);
(*_1) = 132606879422435263062761482987230459801_i128 + 134071619520893587374145801244200397388_i128;
_7 = _8 << _3;
match _14.0 {
0 => bb2,
1 => bb3,
455744075 => bb5,
_ => bb4
}
}
bb12 = {
(*_1) = (-157213504088647482438572971351766062905_i128) - 162629941464718269476935679364961494729_i128;
(*_1) = 9223372036854775807_isize as i128;
_9 = (-65_i8) as i64;
(*_1) = 0_usize as i128;
(*_1) = 119347202810732290233418545727125319791_i128;
_3 = _7 ^ _7;
RET = (-119632012708642724048852650795587006833_i128) & (-28609057908218279914287403168280796080_i128);
_16 = 298585576278015115307211354557926363959_u128 as f32;
_17 = -_16;
(*_1) = (-159888784689825909901431920796638812682_i128);
(*_1) = 54785_u16 as i128;
_13 = &_8;
_14 = (872825443_u32,);
(*_1) = 144563177982123679022685892889977954002_i128 & (-136412154076218863036703682877412368711_i128);
(*_1) = _11 as i128;
_9 = !(*_13);
Goto(bb13)
}
bb13 = {
(*_1) = 144382124966126535298772701556858634919_i128 >> (*_13);
(*_1) = _17 as i128;
(*_1) = 348960928662444574420359742939310872_i128 - (-131905848102160000410853803113740531316_i128);
(*_1) = 219_u8 as i128;
_2 = (*_13) ^ (*_13);
(*_1) = 9875944108498114019_u64 as i128;
_8 = !_3;
(*_1) = 65769352539579177516805080452425870808_i128 + 137194894212890801162210266325315243633_i128;
_4 = [(-73_i8),(-60_i8)];
(*_1) = 69138727167539895326421653193897653942_i128 << _8;
(*_1) = 60_i8 as i128;
_11 = '\u{f6b1e}';
_11 = '\u{135a6}';
(*_1) = (-148065127180617371704877851528660109131_i128) << _3;
_14.0 = 1916729515_u32 ^ 3149898738_u32;
_16 = _17 + _17;
RET = (-44054034477589800881819950056907327676_i128) - (-57023711338738961047448188020507521489_i128);
(*_1) = -(-10890663040839366639239034507165428283_i128);
_19.1 = _8 - _3;
_20 = 55806_u16;
RET = 98173587718865703479502264310173987321_i128 + 12400554238587531729579194574341710235_i128;
_5 = _10;
(*_1) = 77438054360232087208259780512192562775_i128 << _7;
_15 = (*_1);
match _20 {
0 => bb9,
1 => bb5,
2 => bb3,
3 => bb14,
4 => bb15,
5 => bb16,
6 => bb17,
55806 => bb19,
_ => bb18
}
}
bb14 = {
(*_1) = (-157213504088647482438572971351766062905_i128) - 162629941464718269476935679364961494729_i128;
(*_1) = 9223372036854775807_isize as i128;
_9 = (-65_i8) as i64;
(*_1) = 0_usize as i128;
(*_1) = 119347202810732290233418545727125319791_i128;
_3 = _7 ^ _7;
RET = (-119632012708642724048852650795587006833_i128) & (-28609057908218279914287403168280796080_i128);
_16 = 298585576278015115307211354557926363959_u128 as f32;
_17 = -_16;
(*_1) = (-159888784689825909901431920796638812682_i128);
(*_1) = 54785_u16 as i128;
_13 = &_8;
_14 = (872825443_u32,);
(*_1) = 144563177982123679022685892889977954002_i128 & (-136412154076218863036703682877412368711_i128);
(*_1) = _11 as i128;
_9 = !(*_13);
Goto(bb13)
}
bb15 = {
_3 = _2;
_11 = '\u{b8f7d}';
(*_1) = -28440883714868061583419189987944161871_i128;
Goto(bb6)
}
bb16 = {
Return()
}
bb17 = {
Return()
}
bb18 = {
Return()
}
bb19 = {
_16 = -_17;
_14 = (615951363_u32,);
(*_1) = -_15;
(*_1) = _11 as i128;
(*_1) = _15 + _15;
(*_1) = false as i128;
_8 = _20 as i64;
(*_1) = _15;
_9 = !_2;
_22 = (13301914223319432652_u64, 76475065415684008436499359805776123101_u128);
(*_1) = -_15;
(*_1) = (-9223372036854775808_isize) as i128;
_21 = _17 - _17;
_26 = [(-15471_i16),(-18289_i16),3711_i16,25956_i16,(-7599_i16),4844_i16];
_7 = _19.1 | _19.1;
_21 = _16 + _17;
_4 = [43_i8,(-127_i8)];
(*_1) = 28102_i16 as i128;
_12 = -_21;
_22.1 = 223755755490209840377401064635368177026_u128 >> (*_1);
Call(_19.0 = core::intrinsics::bswap(141_u8), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
(*_1) = _15 - _15;
_22.1 = !143666226029050800444294717139557687482_u128;
(*_1) = _15 << _7;
(*_1) = _16 as i128;
_19.0 = !124_u8;
(*_1) = 56_i8 as i128;
(*_1) = _15 | _15;
_26 = [17932_i16,24880_i16,(-28595_i16),(-14314_i16),(-9579_i16),(-2941_i16)];
_28 = (-31648_i16) as f32;
(*_1) = _15;
(*_1) = -_15;
_23 = _11;
_11 = _23;
(*_1) = _15 + _15;
_4 = [(-88_i8),95_i8];
_21 = -_17;
Goto(bb21)
}
bb21 = {
Call(_32 = dump_var(Move(_11), Move(_5), Move(_14), Move(_10)), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Call(_32 = dump_var(Move(_7), Move(_3), Move(_23), Move(_2)), ReturnTo(bb23), UnwindUnreachable())
}
bb23 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn5(mut _1: *mut u16,mut _2: i64,mut _3: isize,mut _4: i32) -> u16 {
mir! {
type RET = u16;
let _5: [i128; 1];
let _6: &'static i16;
let _7: *const f32;
let _8: *mut i128;
let _9: u128;
let _10: *mut i128;
let _11: u128;
let _12: *mut &'static &'static mut (i64,);
let _13: *const [i32; 5];
let _14: u16;
let _15: i64;
let _16: (&'static u128, (i16, &'static i64, [u32; 8], i16), &'static u32);
let _17: [bool; 2];
let _18: isize;
let _19: f32;
let _20: i32;
let _21: *mut Adt73;
let _22: isize;
let _23: u16;
let _24: i128;
let _25: u32;
let _26: [i128; 2];
let _27: ([i128; 2], &'static mut *mut i128, i16);
let _28: *mut isize;
let _29: u8;
let _30: [u128; 7];
let _31: &'static mut u32;
let _32: [u8; 4];
let _33: f64;
let _34: isize;
let _35: u8;
let _36: (u32,);
let _37: (Adt22, &'static u32, (&'static &'static char, i64, i16, i64));
let _38: u64;
let _39: *mut &'static &'static mut (i64,);
let _40: &'static mut u32;
let _41: [i32; 5];
let _42: &'static &'static char;
let _43: isize;
let _44: *const i32;
let _45: *const i64;
let _46: *const i32;
let _47: Adt73;
let _48: f64;
let _49: i16;
let _50: [isize; 5];
let _51: isize;
let _52: f64;
let _53: f32;
let _54: usize;
let _55: char;
let _56: *mut [u8; 4];
let _57: isize;
let _58: i16;
let _59: u128;
let _60: *mut isize;
let _61: Adt22;
let _62: [i32; 5];
let _63: (&'static char, [u128; 7], *const f32, *const [i32; 5]);
let _64: (&'static char, [u128; 7], *const f32, *const [i32; 5]);
let _65: isize;
let _66: *mut Adt75;
let _67: bool;
let _68: i64;
let _69: &'static &'static mut (i64,);
let _70: *mut [isize; 5];
let _71: bool;
let _72: &'static &'static char;
let _73: f64;
let _74: char;
let _75: &'static mut [i8; 2];
let _76: (u8, i64, &'static mut [bool; 2]);
let _77: char;
let _78: [i8; 3];
let _79: &'static mut char;
let _80: (&'static mut char,);
let _81: isize;
let _82: i8;
let _83: i64;
let _84: u8;
let _85: isize;
let _86: f32;
let _87: (u64, u128);
let _88: &'static &'static char;
let _89: *const i128;
let _90: &'static u128;
let _91: ();
let _92: ();
{
RET = (-9066_i16) as u16;
_4 = (-2044976109_i32) << _3;
_4 = 2725430784_u32 as i32;
RET = 15906188569584886166_u64 as u16;
_1 = core::ptr::addr_of_mut!(RET);
_1 = core::ptr::addr_of_mut!((*_1));
(*_1) = 50626_u16 << _2;
(*_1) = !13456_u16;
(*_1) = 283453216369023855077659326444217145266_u128 as u16;
(*_1) = 41423_u16;
(*_1) = 38989_u16 >> _3;
(*_1) = 13907_u16;
match (*_1) {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
13907 => bb6,
_ => bb5
}
}
bb1 = {
Return()
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
(*_1) = !64448_u16;
Goto(bb7)
}
bb7 = {
(*_1) = 12420_u16;
_2 = -6760704161679723487_i64;
(*_1) = 103_u8 as u16;
_5 = [99637671320124621554516918217927606037_i128];
Call((*_1) = fn6(Move(_1), _3, _4, _2, _2, _3, _4, _5, _5, _3, _4, _2), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_1 = core::ptr::addr_of_mut!(RET);
(*_1) = 3059242258_u32 as u16;
Goto(bb9)
}
bb9 = {
_1 = core::ptr::addr_of_mut!((*_1));
(*_1) = 4006_u16 * 13154_u16;
(*_1) = 59850_u16;
(*_1) = 16039_u16;
(*_1) = 43595_u16 + 26178_u16;
(*_1) = 60627_u16 | 24241_u16;
(*_1) = 51_i8 as u16;
_2 = 5571369110059781294_i64 - (-6006728310299330130_i64);
(*_1) = 29721_u16 + 18187_u16;
(*_1) = !41431_u16;
_1 = core::ptr::addr_of_mut!((*_1));
_5 = [119454144005219995296163010240801382661_i128];
(*_1) = 45037_u16;
(*_1) = 38784_u16 - 16237_u16;
(*_1) = 54516_u16 << _2;
_4 = -(-227219692_i32);
(*_1) = 55802_u16 - 40624_u16;
(*_1) = 64374_u16 + 12779_u16;
(*_1) = !50122_u16;
(*_1) = 37577_u16;
(*_1) = 43867_u16;
(*_1) = 97855589151801949632100857113896038166_u128 as u16;
(*_1) = 40990_u16 | 65395_u16;
(*_1) = 56942_u16 * 11538_u16;
Goto(bb10)
}
bb10 = {
_3 = 9223372036854775807_isize >> (*_1);
(*_1) = !1602_u16;
(*_1) = _4 as u16;
_3 = -(-9223372036854775808_isize);
(*_1) = 54443_u16;
RET = 40716_u16 | 4688_u16;
Goto(bb11)
}
bb11 = {
(*_1) = _3 as u16;
(*_1) = 61753_u16 | 36031_u16;
_2 = !4177565161879635863_i64;
(*_1) = !41707_u16;
_2 = (-3058327667821719314_i64) & 499438391852729857_i64;
(*_1) = 10123_u16 + 18922_u16;
_2 = true as i64;
(*_1) = 8340_u16;
(*_1) = !31215_u16;
(*_1) = 2624297442_u32 as u16;
(*_1) = 54048_u16 << _3;
(*_1) = 58776_u16;
(*_1) = 28021_u16;
RET = !27290_u16;
(*_1) = !11085_u16;
(*_1) = !62868_u16;
Goto(bb12)
}
bb12 = {
(*_1) = 48039_u16 | 18058_u16;
_4 = (-1363563630_i32);
(*_1) = 55508_u16 + 55860_u16;
_3 = !102_isize;
(*_1) = (-15570_i16) as u16;
(*_1) = _3 as u16;
(*_1) = 16841_u16 - 51951_u16;
(*_1) = 16498_u16;
(*_1) = 28217_u16 >> _4;
(*_1) = 39456_u16 & 64455_u16;
_2 = 328386816551122025_usize as i64;
(*_1) = 49321_u16 * 40669_u16;
(*_1) = '\u{3d980}' as u16;
(*_1) = 48034_u16 | 1837_u16;
(*_1) = 24881_u16 ^ 27359_u16;
(*_1) = 49856_u16 >> _2;
(*_1) = !30392_u16;
RET = _2 as u16;
_9 = 228751281222737527234765933951978676372_u128 | 216972592389091356101114084350527309714_u128;
(*_1) = !3523_u16;
(*_1) = 13411_u16 - 18795_u16;
_5 = [(-123609588392813551881948770221331800280_i128)];
Goto(bb13)
}
bb13 = {
_5 = [38781620586917449776103784018160940307_i128];
(*_1) = 15437039881812540990_usize as u16;
RET = !51701_u16;
(*_1) = !14625_u16;
(*_1) = 2345_u16 >> _4;
_11 = _3 as u128;
_1 = core::ptr::addr_of_mut!((*_1));
(*_1) = 7447_u16 | 26536_u16;
_2 = (-6163515963924873096_i64) ^ 8701285670396608254_i64;
Goto(bb14)
}
bb14 = {
(*_1) = 35609_u16 >> _3;
RET = 64885_u16;
(*_1) = 35107_u16 - 2423_u16;
(*_1) = 184_u8 as u16;
_5 = [71781795327882574765690250365637651847_i128];
RET = 18927_u16 << _3;
_5 = [155257191541343683269732914011710137645_i128];
(*_1) = !16774_u16;
(*_1) = !1509_u16;
(*_1) = !11632_u16;
_3 = 6211986607526337408_u64 as isize;
(*_1) = !14984_u16;
(*_1) = !55112_u16;
(*_1) = 7_usize as u16;
(*_1) = !40571_u16;
(*_1) = !9145_u16;
(*_1) = 56077_u16 << _11;
_2 = 3026894584_u32 as i64;
_15 = !_2;
(*_1) = 49072_u16 & 38692_u16;
(*_1) = 41326_u16 & 31785_u16;
(*_1) = 24723_u16;
_18 = _3;
(*_1) = _2 as u16;
(*_1) = 24616_u16;
Goto(bb15)
}
bb15 = {
_2 = _15;
_16.1.2 = [3870697192_u32,126997758_u32,3412746949_u32,2557039250_u32,3202750137_u32,701530159_u32,4047099254_u32,3156650419_u32];
_16.1.0 = (-3315_i16) << (*_1);
(*_1) = _15 as u16;
(*_1) = !62720_u16;
_16.1.3 = _16.1.0 | _16.1.0;
_16.1.0 = _16.1.3 | _16.1.3;
_11 = !_9;
_6 = &_16.1.3;
(*_1) = 8602_u16 << (*_6);
_14 = _11 as u16;
(*_1) = _14 >> (*_6);
(*_1) = (*_6) as u16;
Goto(bb16)
}
bb16 = {
_5 = [120341834910658184624807022743963526866_i128];
_16.1.1 = &_2;
(*_1) = _14;
_6 = &_16.1.0;
_16.0 = &_9;
(*_1) = _14 | _14;
_2 = _3 as i64;
(*_1) = _14 + _14;
_15 = !_2;
_17 = [false,false];
(*_1) = _14 - _14;
_22 = _18 >> (*_6);
_20 = !_4;
match _4 {
0 => bb13,
340282366920938463463374607430404647826 => bb17,
_ => bb3
}
}
bb17 = {
(*_1) = 102559159218620899659487072210732057179_i128 as u16;
_10 = core::ptr::addr_of_mut!(_24);
match _4 {
0 => bb8,
1 => bb2,
2 => bb11,
340282366920938463463374607430404647826 => bb18,
_ => bb6
}
}
bb18 = {
(*_1) = _14 + _14;
(*_10) = (-145573296971380634377419922009387086562_i128);
Goto(bb19)
}
bb19 = {
(*_10) = 110047008887769960059474226870247330241_i128 | (-167259657790729411270897733810133320303_i128);
_16.1.0 = _16.1.3;
_19 = 4185401947_u32 as f32;
(*_1) = _14 ^ _14;
(*_10) = -86652863928929430169802312706709603360_i128;
_3 = _16.1.3 as isize;
_7 = core::ptr::addr_of!(_19);
(*_7) = 762991781_u32 as f32;
_15 = _2 << _16.1.3;
_2 = _15;
(*_1) = _14;
(*_7) = 4_usize as f32;
(*_1) = _14;
(*_7) = (*_1) as f32;
(*_1) = _14;
_8 = core::ptr::addr_of_mut!((*_10));
(*_7) = 11_u8 as f32;
(*_10) = _11 as i128;
(*_10) = (-19_i8) as i128;
(*_10) = 152827922579765983990227872561900301345_i128 + 63622146643061577635116414608273341528_i128;
(*_1) = _14 & _14;
_4 = _11 as i32;
(*_7) = _22 as f32;
(*_7) = _16.1.0 as f32;
(*_7) = 7678124534283056882_u64 as f32;
Goto(bb20)
}
bb20 = {
(*_7) = _11 as f32;
(*_1) = _14 & _14;
(*_10) = -44538977383107482273547004568964849405_i128;
_27.0 = [(*_10),(*_10)];
(*_1) = _14 & _14;
(*_10) = (*_1) as i128;
(*_7) = 1_usize as f32;
(*_7) = _9 as f32;
(*_1) = !_14;
_19 = 3691581242_u32 as f32;
(*_1) = !_14;
(*_7) = (*_1) as f32;
_17 = [false,true];
_26 = [_24,(*_10)];
(*_7) = (*_10) as f32;
(*_10) = 166636652141109981505167155275945961665_i128 & 65982321611925608114425707302375772523_i128;
(*_10) = 138405205668865345932043139967995280994_i128 * 43315929189387271613221169414828766731_i128;
_27.2 = _16.1.3;
_27.2 = _16.1.0 * _16.1.3;
_32 = [221_u8,232_u8,176_u8,231_u8];
(*_1) = _14 | _14;
(*_7) = _15 as f32;
_2 = _15 ^ _15;
(*_1) = _14 ^ _14;
Call((*_10) = core::intrinsics::transmute(_9), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
_6 = &_16.1.3;
Goto(bb22)
}
bb22 = {
_23 = 0_usize as u16;
(*_1) = !_14;
_2 = _15 & _15;
(*_10) = _9 as i128;
(*_1) = !_14;
(*_10) = 30494352155966091865122258856329163537_i128 ^ (-37157553467465895632219493124558556914_i128);
(*_7) = (*_1) as f32;
(*_7) = 205_u8 as f32;
(*_10) = (-15117701963598873892932388880213510511_i128);
_30 = [_11,_9,_9,_9,_11,_9,_11];
(*_1) = 96_i8 as u16;
(*_10) = (-136487412535609693338722082728180589834_i128) << (*_6);
(*_7) = _20 as f32;
_24 = !(-13821411969566994780040832905473793197_i128);
(*_7) = _4 as f32;
(*_7) = 10465566564357519780_usize as f32;
(*_7) = _4 as f32;
_27.1 = &mut _8;
(*_7) = RET as f32;
_24 = 122269616555615670938195318554852305245_i128;
(*_1) = _14 & _14;
(*_1) = 362584485_u32 as u16;
_34 = _22 - _18;
Goto(bb23)
}
bb23 = {
(*_10) = -(-15209977638990748695048687000545721968_i128);
(*_10) = 131161408080961329927960790074246693678_i128 | (-7612422807247260132809340561817416837_i128);
_28 = core::ptr::addr_of_mut!(_18);
(*_10) = (-23774189679806560539672356494128184334_i128) ^ 10506517568000886205785457418975195617_i128;
(*_7) = _34 as f32;
(*_1) = _14 + _23;
_5 = [(*_10)];
_15 = _2 ^ _2;
(*_1) = _14 ^ _23;
(*_10) = 28061931434027698297986697687510168825_i128 + 147563438038156471438266143181847219541_i128;
(*_28) = _34;
(*_7) = _9 as f32;
(*_1) = 2570805557_u32 as u16;
Goto(bb24)
}
bb24 = {
_30 = [_11,_9,_11,_11,_11,_11,_11];
_16.1.1 = &_15;
_28 = core::ptr::addr_of_mut!((*_28));
(*_7) = (*_1) as f32;
_37.2.3 = _15 ^ _15;
(*_7) = (*_28) as f32;
_16.1.3 = _16.1.0 >> _37.2.3;
(*_7) = 2984323416000873300_u64 as f32;
_16.1.0 = -_16.1.3;
(*_28) = (*_1) as isize;
_16.0 = &_11;
(*_28) = _22 >> _27.2;
_16.1.0 = _16.1.3 >> (*_28);
_36.0 = 1992287722_u32;
_7 = core::ptr::addr_of!((*_7));
_37.0.fld3 = !(-37_i8);
match _36.0 {
0 => bb25,
1 => bb26,
1992287722 => bb28,
_ => bb27
}
}
bb25 = {
_1 = core::ptr::addr_of_mut!((*_1));
(*_1) = 4006_u16 * 13154_u16;
(*_1) = 59850_u16;
(*_1) = 16039_u16;
(*_1) = 43595_u16 + 26178_u16;
(*_1) = 60627_u16 | 24241_u16;
(*_1) = 51_i8 as u16;
_2 = 5571369110059781294_i64 - (-6006728310299330130_i64);
(*_1) = 29721_u16 + 18187_u16;
(*_1) = !41431_u16;
_1 = core::ptr::addr_of_mut!((*_1));
_5 = [119454144005219995296163010240801382661_i128];
(*_1) = 45037_u16;
(*_1) = 38784_u16 - 16237_u16;
(*_1) = 54516_u16 << _2;
_4 = -(-227219692_i32);
(*_1) = 55802_u16 - 40624_u16;
(*_1) = 64374_u16 + 12779_u16;
(*_1) = !50122_u16;
(*_1) = 37577_u16;
(*_1) = 43867_u16;
(*_1) = 97855589151801949632100857113896038166_u128 as u16;
(*_1) = 40990_u16 | 65395_u16;
(*_1) = 56942_u16 * 11538_u16;
Goto(bb10)
}
bb26 = {
(*_1) = 102559159218620899659487072210732057179_i128 as u16;
_10 = core::ptr::addr_of_mut!(_24);
match _4 {
0 => bb8,
1 => bb2,
2 => bb11,
340282366920938463463374607430404647826 => bb18,
_ => bb6
}
}
bb27 = {
Return()
}
bb28 = {
(*_1) = _14;
(*_28) = _37.0.fld3 as isize;
match _36.0 {
0 => bb23,
1 => bb29,
1992287722 => bb31,
_ => bb30
}
}
bb29 = {
_5 = [38781620586917449776103784018160940307_i128];
(*_1) = 15437039881812540990_usize as u16;
RET = !51701_u16;
(*_1) = !14625_u16;
(*_1) = 2345_u16 >> _4;
_11 = _3 as u128;
_1 = core::ptr::addr_of_mut!((*_1));
(*_1) = 7447_u16 | 26536_u16;
_2 = (-6163515963924873096_i64) ^ 8701285670396608254_i64;
Goto(bb14)
}
bb30 = {
(*_1) = 102559159218620899659487072210732057179_i128 as u16;
_10 = core::ptr::addr_of_mut!(_24);
match _4 {
0 => bb8,
1 => bb2,
2 => bb11,
340282366920938463463374607430404647826 => bb18,
_ => bb6
}
}
bb31 = {
(*_28) = _16.1.0 as isize;
(*_28) = _34 - _34;
(*_7) = _20 as f32;
(*_7) = _4 as f32;
_27.2 = _16.1.3 << _16.1.0;
Goto(bb32)
}
bb32 = {
(*_28) = _22;
_30 = [_11,_11,_9,_9,_9,_11,_9];
_10 = core::ptr::addr_of_mut!((*_10));
(*_10) = !56579674160889048703482617952423288918_i128;
_3 = (*_28);
(*_7) = _37.2.3 as f32;
match _36.0 {
0 => bb17,
1 => bb2,
2 => bb16,
3 => bb4,
4 => bb23,
5 => bb12,
6 => bb31,
1992287722 => bb33,
_ => bb8
}
}
bb33 = {
(*_1) = !_14;
Goto(bb34)
}
bb34 = {
(*_7) = 6168964900295537033_u64 as f32;
_11 = _9;
(*_28) = _34 - _3;
_16.1.2 = [_36.0,_36.0,_36.0,_36.0,_36.0,_36.0,_36.0,_36.0];
(*_7) = 15179516198154958460_u64 as f32;
_2 = _37.2.3 | _37.2.3;
(*_28) = _4 as isize;
match _36.0 {
0 => bb8,
1 => bb10,
1992287722 => bb35,
_ => bb23
}
}
bb35 = {
(*_1) = _14 - _14;
(*_1) = 17331183141508998512_u64 as u16;
(*_1) = _14;
(*_1) = _23 ^ _14;
Goto(bb36)
}
bb36 = {
(*_10) = (-97332692143873945988500136229353359311_i128) + 632269327688491207155101213373812913_i128;
(*_1) = _14 & _14;
_46 = core::ptr::addr_of!(_20);
_34 = !(*_28);
_36.0 = 1513927517_u32 - 1249022200_u32;
(*_1) = _9 as u16;
(*_28) = -_22;
Goto(bb37)
}
bb37 = {
_37.2.2 = _16.1.3;
_16.0 = &_9;
_37.0.fld0 = false;
_48 = (*_10) as f64;
_46 = core::ptr::addr_of!((*_46));
_45 = core::ptr::addr_of!(_47.fld0.0);
(*_7) = _11 as f32;
_49 = (*_28) as i16;
(*_45) = (*_28) as i64;
(*_10) = (-102531093740343148570481697244653409114_i128) | 14215794039710110184557994129141982718_i128;
(*_28) = 11348803284404671408_u64 as isize;
(*_28) = _22 * _22;
_25 = !_36.0;
Goto(bb38)
}
bb38 = {
(*_46) = _4;
(*_28) = _22 ^ _22;
(*_7) = _16.1.3 as f32;
(*_28) = _22;
_35 = 211_u8 >> _2;
(*_10) = !59234854408627115310505261630010002526_i128;
(*_7) = _16.1.0 as f32;
(*_45) = (*_46) as i64;
_27.0 = [(*_10),(*_10)];
_6 = &_16.1.0;
_27.1 = &mut _10;
(*_1) = !_14;
(*_28) = _22 | _3;
Goto(bb39)
}
bb39 = {
_21 = core::ptr::addr_of_mut!(_47);
(*_21).fld1 = _16.1.2;
(*_45) = _2 - _37.2.3;
(*_21).fld2 = core::ptr::addr_of!(_41);
(*_21).fld4 = Adt21::Variant1 { fld0: _37.0.fld0,fld1: _48,fld2: (*_28),fld3: _37.0.fld3,fld4: (*_6),fld5: _25,fld6: (*_7),fld7: 12409913479999238271_usize };
place!(Field::<u32>(Variant((*_21).fld4, 1), 5)) = _25;
_47.fld4 = Adt21::Variant0 { fld0: _11,fld1: _4,fld2: _37.0.fld3 };
_6 = &_27.2;
place!(Field::<i8>(Variant((*_21).fld4, 0), 2)) = Field::<u128>(Variant((*_21).fld4, 0), 0) as i8;
(*_21).fld4 = Adt21::Variant0 { fld0: _11,fld1: (*_46),fld2: _37.0.fld3 };
_44 = core::ptr::addr_of!((*_46));
Goto(bb40)
}
bb40 = {
(*_28) = (*_45) as isize;
_44 = Move(_46);
(*_21).fld3 = core::ptr::addr_of_mut!(_50);
(*_21).fld3 = core::ptr::addr_of_mut!(_50);
(*_7) = (*_28) as f32;
(*_45) = _2 | _2;
(*_21).fld1 = [_25,_36.0,_36.0,_36.0,_36.0,_36.0,_25,_36.0];
(*_28) = Field::<i8>(Variant((*_21).fld4, 0), 2) as isize;
(*_7) = _36.0 as f32;
_21 = core::ptr::addr_of_mut!(_47);
place!(Field::<u128>(Variant(_47.fld4, 0), 0)) = _3 as u128;
_36 = (_25,);
_43 = -_3;
(*_45) = Field::<i32>(Variant((*_21).fld4, 0), 1) as i64;
(*_45) = _15 << (*_6);
(*_21).fld1 = [_36.0,_36.0,_25,_25,_25,_25,_36.0,_36.0];
_37.0 = Adt22 { fld0: true,fld1: (*_21).fld4,fld2: 6020313262877703624_u64,fld3: Field::<i8>(Variant((*_21).fld4, 0), 2) };
(*_21).fld2 = core::ptr::addr_of!(_41);
_19 = 1_usize as f32;
_47.fld0.0 = _2;
place!(Field::<i32>(Variant((*_21).fld4, 0), 1)) = Field::<i32>(Variant(_37.0.fld1, 0), 1);
_16.2 = &_25;
(*_21).fld4 = Adt21::Variant0 { fld0: Field::<u128>(Variant(_37.0.fld1, 0), 0),fld1: _4,fld2: _37.0.fld3 };
_48 = (*_7) as f64;
Goto(bb41)
}
bb41 = {
_61.fld1 = Adt21::Variant1 { fld0: _37.0.fld0,fld1: _48,fld2: (*_28),fld3: Field::<i8>(Variant((*_21).fld4, 0), 2),fld4: (*_6),fld5: _25,fld6: (*_7),fld7: 3_usize };
(*_1) = _23;
place!(Field::<i32>(Variant((*_21).fld4, 0), 1)) = _4;
(*_28) = _22;
(*_21).fld1 = _16.1.2;
(*_1) = 6_usize as u16;
_16.1.0 = (*_6);
_64.2 = core::ptr::addr_of!((*_7));
place!(Field::<usize>(Variant(_61.fld1, 1), 7)) = 4_usize - 6275421503396569656_usize;
(*_45) = Field::<usize>(Variant(_61.fld1, 1), 7) as i64;
place!(Field::<f64>(Variant(_61.fld1, 1), 1)) = _48;
(*_21).fld1 = [Field::<u32>(Variant(_61.fld1, 1), 5),Field::<u32>(Variant(_61.fld1, 1), 5),_36.0,_25,_36.0,Field::<u32>(Variant(_61.fld1, 1), 5),_36.0,_25];
_58 = _36.0 as i16;
_31 = &mut place!(Field::<u32>(Variant(_61.fld1, 1), 5));
_15 = (*_45) - (*_45);
_37.0.fld0 = _2 == _37.2.3;
(*_21).fld1 = _16.1.2;
(*_21).fld4 = Adt21::Variant1 { fld0: _37.0.fld0,fld1: _48,fld2: (*_28),fld3: Field::<i8>(Variant(_37.0.fld1, 0), 2),fld4: (*_6),fld5: (*_31),fld6: (*_7),fld7: 4_usize };
(*_21).fld1 = _16.1.2;
Goto(bb42)
}
bb42 = {
(*_31) = Field::<u32>(Variant((*_21).fld4, 1), 5);
(*_21).fld2 = core::ptr::addr_of!(_62);
(*_45) = _37.2.3 >> Field::<i16>(Variant((*_21).fld4, 1), 4);
place!(Field::<i8>(Variant((*_21).fld4, 1), 3)) = _37.0.fld3 >> (*_6);
(*_45) = _2 | _37.2.3;
match _37.0.fld2 {
0 => bb31,
1 => bb2,
2 => bb43,
3 => bb44,
6020313262877703624 => bb46,
_ => bb45
}
}
bb43 = {
_1 = core::ptr::addr_of_mut!(RET);
(*_1) = 3059242258_u32 as u16;
Goto(bb9)
}
bb44 = {
(*_10) = -(-15209977638990748695048687000545721968_i128);
(*_10) = 131161408080961329927960790074246693678_i128 | (-7612422807247260132809340561817416837_i128);
_28 = core::ptr::addr_of_mut!(_18);
(*_10) = (-23774189679806560539672356494128184334_i128) ^ 10506517568000886205785457418975195617_i128;
(*_7) = _34 as f32;
(*_1) = _14 + _23;
_5 = [(*_10)];
_15 = _2 ^ _2;
(*_1) = _14 ^ _23;
(*_10) = 28061931434027698297986697687510168825_i128 + 147563438038156471438266143181847219541_i128;
(*_28) = _34;
(*_7) = _9 as f32;
(*_1) = 2570805557_u32 as u16;
Goto(bb24)
}
bb45 = {
(*_28) = _22;
_30 = [_11,_11,_9,_9,_9,_11,_9];
_10 = core::ptr::addr_of_mut!((*_10));
(*_10) = !56579674160889048703482617952423288918_i128;
_3 = (*_28);
(*_7) = _37.2.3 as f32;
match _36.0 {
0 => bb17,
1 => bb2,
2 => bb16,
3 => bb4,
4 => bb23,
5 => bb12,
6 => bb31,
1992287722 => bb33,
_ => bb8
}
}
bb46 = {
_71 = Field::<bool>(Variant((*_21).fld4, 1), 0);
place!(Field::<usize>(Variant((*_21).fld4, 1), 7)) = !15329245323323202919_usize;
(*_21).fld2 = core::ptr::addr_of!(_62);
(*_28) = -Field::<isize>(Variant((*_21).fld4, 1), 2);
_46 = Move(_44);
(*_21).fld4 = Adt21::Variant1 { fld0: _71,fld1: _48,fld2: (*_28),fld3: Field::<i8>(Variant(_37.0.fld1, 0), 2),fld4: (*_6),fld5: (*_31),fld6: (*_7),fld7: 13206011315295469829_usize };
Goto(bb47)
}
bb47 = {
(*_7) = (*_45) as f32;
(*_31) = '\u{c582d}' as u32;
(*_31) = Field::<i8>(Variant((*_21).fld4, 1), 3) as u32;
place!(Field::<i8>(Variant(_47.fld4, 1), 3)) = -_37.0.fld3;
(*_45) = !_37.2.3;
(*_21).fld3 = core::ptr::addr_of_mut!(_50);
(*_31) = Field::<i32>(Variant(_37.0.fld1, 0), 1) as u32;
_16.0 = &place!(Field::<u128>(Variant(_37.0.fld1, 0), 0));
place!(Field::<i32>(Variant(_37.0.fld1, 0), 1)) = _4;
(*_7) = -Field::<f32>(Variant((*_21).fld4, 1), 6);
(*_7) = -Field::<f32>(Variant((*_21).fld4, 1), 6);
place!(Field::<usize>(Variant((*_21).fld4, 1), 7)) = 0_usize & 0_usize;
place!(Field::<bool>(Variant((*_21).fld4, 1), 0)) = !_71;
place!(Field::<usize>(Variant((*_21).fld4, 1), 7)) = (*_1) as usize;
place!(Field::<i16>(Variant((*_21).fld4, 1), 4)) = (*_6) << (*_6);
(*_45) = _2 | _2;
place!(Field::<isize>(Variant((*_21).fld4, 1), 2)) = (*_28);
_54 = Field::<usize>(Variant((*_21).fld4, 1), 7);
place!(Field::<f64>(Variant(_47.fld4, 1), 1)) = _48;
_76.1 = -(*_45);
_58 = (*_7) as i16;
Goto(bb48)
}
bb48 = {
place!(Field::<f64>(Variant((*_21).fld4, 1), 1)) = -_48;
place!(Field::<u32>(Variant((*_21).fld4, 1), 5)) = Field::<i32>(Variant(_37.0.fld1, 0), 1) as u32;
place!(Field::<isize>(Variant((*_21).fld4, 1), 2)) = Field::<f64>(Variant((*_21).fld4, 1), 1) as isize;
_37.0.fld1 = Adt21::Variant0 { fld0: _9,fld1: _4,fld2: Field::<i8>(Variant((*_21).fld4, 1), 3) };
(*_21).fld1 = [Field::<u32>(Variant((*_21).fld4, 1), 5),Field::<u32>(Variant((*_21).fld4, 1), 5),Field::<u32>(Variant((*_21).fld4, 1), 5),(*_31),_36.0,Field::<u32>(Variant((*_21).fld4, 1), 5),Field::<u32>(Variant(_47.fld4, 1), 5),Field::<u32>(Variant((*_21).fld4, 1), 5)];
(*_21).fld3 = core::ptr::addr_of_mut!(_50);
(*_45) = Field::<i16>(Variant((*_21).fld4, 1), 4) as i64;
place!(Field::<i16>(Variant((*_21).fld4, 1), 4)) = (*_6) - (*_6);
place!(Field::<i8>(Variant((*_21).fld4, 1), 3)) = -Field::<i8>(Variant(_37.0.fld1, 0), 2);
place!(Field::<usize>(Variant((*_21).fld4, 1), 7)) = _54 | _54;
_86 = -Field::<f32>(Variant((*_21).fld4, 1), 6);
(*_21).fld2 = core::ptr::addr_of!(_62);
(*_21).fld1 = [Field::<u32>(Variant((*_21).fld4, 1), 5),Field::<u32>(Variant(_47.fld4, 1), 5),Field::<u32>(Variant((*_21).fld4, 1), 5),Field::<u32>(Variant((*_21).fld4, 1), 5),(*_31),Field::<u32>(Variant((*_21).fld4, 1), 5),Field::<u32>(Variant((*_21).fld4, 1), 5),_36.0];
match _37.0.fld2 {
0 => bb49,
1 => bb50,
2 => bb51,
6020313262877703624 => bb53,
_ => bb52
}
}
bb49 = {
_5 = [38781620586917449776103784018160940307_i128];
(*_1) = 15437039881812540990_usize as u16;
RET = !51701_u16;
(*_1) = !14625_u16;
(*_1) = 2345_u16 >> _4;
_11 = _3 as u128;
_1 = core::ptr::addr_of_mut!((*_1));
(*_1) = 7447_u16 | 26536_u16;
_2 = (-6163515963924873096_i64) ^ 8701285670396608254_i64;
Goto(bb14)
}
bb50 = {
_71 = Field::<bool>(Variant((*_21).fld4, 1), 0);
place!(Field::<usize>(Variant((*_21).fld4, 1), 7)) = !15329245323323202919_usize;
(*_21).fld2 = core::ptr::addr_of!(_62);
(*_28) = -Field::<isize>(Variant((*_21).fld4, 1), 2);
_46 = Move(_44);
(*_21).fld4 = Adt21::Variant1 { fld0: _71,fld1: _48,fld2: (*_28),fld3: Field::<i8>(Variant(_37.0.fld1, 0), 2),fld4: (*_6),fld5: (*_31),fld6: (*_7),fld7: 13206011315295469829_usize };
Goto(bb47)
}
bb51 = {
_1 = core::ptr::addr_of_mut!(RET);
(*_1) = 3059242258_u32 as u16;
Goto(bb9)
}
bb52 = {
(*_10) = -(-15209977638990748695048687000545721968_i128);
(*_10) = 131161408080961329927960790074246693678_i128 | (-7612422807247260132809340561817416837_i128);
_28 = core::ptr::addr_of_mut!(_18);
(*_10) = (-23774189679806560539672356494128184334_i128) ^ 10506517568000886205785457418975195617_i128;
(*_7) = _34 as f32;
(*_1) = _14 + _23;
_5 = [(*_10)];
_15 = _2 ^ _2;
(*_1) = _14 ^ _23;
(*_10) = 28061931434027698297986697687510168825_i128 + 147563438038156471438266143181847219541_i128;
(*_28) = _34;
(*_7) = _9 as f32;
(*_1) = 2570805557_u32 as u16;
Goto(bb24)
}
bb53 = {
(*_7) = (*_45) as f32;
(*_21).fld2 = core::ptr::addr_of!(_41);
(*_21).fld3 = core::ptr::addr_of_mut!(_50);
_64.1 = _30;
(*_21).fld2 = core::ptr::addr_of!(_41);
(*_21).fld1 = _16.1.2;
_76.2 = &mut _17;
_50 = [(*_28),_43,(*_28),_43,Field::<isize>(Variant((*_21).fld4, 1), 2)];
place!(Field::<f64>(Variant((*_21).fld4, 1), 1)) = Field::<u32>(Variant((*_21).fld4, 1), 5) as f64;
(*_21).fld1 = [Field::<u32>(Variant((*_21).fld4, 1), 5),(*_31),Field::<u32>(Variant((*_21).fld4, 1), 5),_36.0,(*_31),(*_31),Field::<u32>(Variant(_47.fld4, 1), 5),Field::<u32>(Variant((*_21).fld4, 1), 5)];
(*_1) = _24 as u16;
(*_28) = -Field::<isize>(Variant((*_21).fld4, 1), 2);
_54 = Field::<usize>(Variant((*_21).fld4, 1), 7) + Field::<usize>(Variant(_47.fld4, 1), 7);
Goto(bb54)
}
bb54 = {
Call(_91 = dump_var(Move(_36), Move(_32), Move(_50), Move(_14)), ReturnTo(bb55), UnwindUnreachable())
}
bb55 = {
Call(_91 = dump_var(Move(_11), Move(_35), Move(_20), Move(_4)), ReturnTo(bb56), UnwindUnreachable())
}
bb56 = {
Call(_91 = dump_var(Move(_23), Move(_58), Move(_30), Move(_25)), ReturnTo(bb57), UnwindUnreachable())
}
bb57 = {
Call(_91 = dump_var(Move(_3), _92, _92, _92), ReturnTo(bb58), UnwindUnreachable())
}
bb58 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn6(mut _1: *mut u16,mut _2: isize,mut _3: i32,mut _4: i64,mut _5: i64,mut _6: isize,mut _7: i32,mut _8: [i128; 1],mut _9: [i128; 1],mut _10: isize,mut _11: i32,mut _12: i64) -> u16 {
mir! {
type RET = u16;
let _13: i32;
let _14: f32;
let _15: (&'static mut *mut i128,);
let _16: i64;
let _17: u128;
let _18: u32;
let _19: u64;
let _20: char;
let _21: i8;
let _22: char;
let _23: &'static &'static mut (i64,);
let _24: Adt46;
let _25: *const f32;
let _26: &'static mut [bool; 2];
let _27: f64;
let _28: [u8; 4];
let _29: ([i128; 1], [i8; 2], *const Adt22);
let _30: [i32; 5];
let _31: *mut isize;
let _32: &'static mut [i8; 2];
let _33: i16;
let _34: ();
let _35: ();
{
_7 = _11;
_3 = _11 ^ _7;
RET = 8193452479290275701_u64 as u16;
_8 = _9;
_7 = 273948566858150685345249746320442947593_u128 as i32;
_13 = _7 & _11;
_7 = _13 << _11;
_4 = 4189252370_u32 as i64;
_2 = _6 | _10;
_9 = [145537463753226803296146170750729858665_i128];
_9 = _8;
_6 = -_10;
_4 = _12 << _12;
_6 = _7 as isize;
_2 = !_6;
RET = 45077_u16;
_8 = [(-119297638660932993633467781816494705023_i128)];
_12 = _4 ^ _5;
_9 = _8;
_14 = 3225384952_u32 as f32;
match RET {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
45077 => bb6,
_ => bb5
}
}
bb1 = {
Return()
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
_18 = 1272826617_u32 - 1522495989_u32;
_10 = _6 + _6;
_3 = -_13;
_13 = _10 as i32;
_18 = 1597011013_u32 << _11;
RET = !49536_u16;
_17 = _12 as u128;
_2 = _18 as isize;
Goto(bb7)
}
bb7 = {
_19 = 12034294410377344336_u64 - 1222284653062601694_u64;
_4 = _12;
_14 = _17 as f32;
_8 = [(-117910997670773488377662612240805951844_i128)];
_1 = core::ptr::addr_of_mut!(RET);
_20 = '\u{ba41b}';
_4 = _12;
_12 = -_4;
(*_1) = _17 as u16;
(*_1) = 26074_u16 - 4594_u16;
_2 = !_10;
_21 = (-114_i8) >> _12;
(*_1) = !2980_u16;
(*_1) = 36702_u16 & 13628_u16;
Goto(bb8)
}
bb8 = {
(*_1) = 50984_u16 + 52156_u16;
_22 = _20;
(*_1) = 63999_u16 >> _7;
_2 = _6 & _10;
_21 = 49_i8 * (-56_i8);
_1 = core::ptr::addr_of_mut!((*_1));
_18 = !397037916_u32;
(*_1) = 64940_u16;
(*_1) = !58463_u16;
_16 = _4 ^ _4;
_17 = 144074629317543326695962000933425984350_u128;
_4 = _12;
(*_1) = 61962_u16;
_22 = _20;
_14 = _3 as f32;
_7 = _13 + _13;
(*_1) = !49679_u16;
_13 = 30565348115528486380975252686864193798_i128 as i32;
(*_1) = !39116_u16;
(*_1) = _16 as u16;
_12 = _16 - _4;
(*_1) = !3548_u16;
_27 = 10366764497766296169_usize as f64;
Goto(bb9)
}
bb9 = {
(*_1) = 17332_u16 + 50628_u16;
(*_1) = !33204_u16;
(*_1) = 56117_u16 * 48553_u16;
_25 = core::ptr::addr_of!(_14);
(*_25) = _13 as f32;
_10 = !_2;
(*_1) = 59144_u16 * 47911_u16;
_16 = -_12;
_25 = core::ptr::addr_of!((*_25));
Call((*_25) = fn7(Move(_1), (*_1), (*_1), Move(_25)), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_4 = _12 & _16;
_19 = _17 as u64;
_16 = 2_usize as i64;
_11 = !_3;
_28 = [183_u8,15_u8,23_u8,164_u8];
_1 = core::ptr::addr_of_mut!(RET);
_27 = _2 as f64;
(*_1) = 49891_u16 - 54313_u16;
(*_1) = 42197_u16 ^ 10976_u16;
_30 = [_3,_7,_7,_11,_7];
_1 = core::ptr::addr_of_mut!((*_1));
_27 = _4 as f64;
(*_1) = 20651_u16 * 36350_u16;
_9 = _8;
_3 = _7 | _7;
_21 = (-84_i8);
_25 = core::ptr::addr_of!(_14);
(*_1) = !55840_u16;
_13 = !_3;
_18 = 19962_i16 as u32;
(*_25) = _18 as f32;
_7 = -_13;
(*_25) = _2 as f32;
(*_1) = 12772_u16;
(*_1) = _17 as u16;
(*_1) = 41236_u16 >> _7;
Goto(bb11)
}
bb11 = {
Call(_34 = dump_var(Move(_22), Move(_10), Move(_4), Move(_5)), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
Call(_34 = dump_var(Move(_7), Move(_8), Move(_28), Move(_11)), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
Call(_34 = dump_var(Move(_3), Move(_18), _35, _35), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn7(mut _1: *mut u16,mut _2: u16,mut _3: u16,mut _4: *const f32) -> f32 {
mir! {
type RET = f32;
let _5: &'static mut [i8; 2];
let _6: i128;
let _7: *const i128;
let _8: u32;
let _9: f64;
let _10: i16;
let _11: &'static u128;
let _12: isize;
let _13: isize;
let _14: &'static u32;
let _15: &'static mut (i64,);
let _16: i8;
let _17: &'static mut u32;
let _18: bool;
let _19: &'static mut &'static mut (i64,);
let _20: *mut Adt73;
let _21: i64;
let _22: isize;
let _23: f32;
let _24: [i32; 5];
let _25: isize;
let _26: i32;
let _27: *mut i128;
let _28: Adt71;
let _29: char;
let _30: char;
let _31: *mut Adt73;
let _32: (&'static u128, (i16, &'static i64, [u32; 8], i16), &'static u32);
let _33: char;
let _34: (u64, u128);
let _35: &'static mut [i8; 2];
let _36: i16;
let _37: u32;
let _38: *mut isize;
let _39: (&'static mut char,);
let _40: &'static mut [bool; 2];
let _41: &'static &'static char;
let _42: &'static i16;
let _43: bool;
let _44: *const i128;
let _45: &'static char;
let _46: *mut isize;
let _47: Adt71;
let _48: isize;
let _49: ();
let _50: ();
{
_1 = core::ptr::addr_of_mut!(_2);
(*_1) = _3 - _3;
(*_1) = _3 * _3;
_1 = core::ptr::addr_of_mut!((*_1));
(*_1) = _3;
(*_1) = _3 | _3;
(*_1) = _3;
(*_1) = !_3;
(*_1) = _3 + _3;
(*_1) = !_3;
RET = 55_u8 as f32;
(*_1) = 18328138871498297337_u64 as u16;
(*_1) = _3;
(*_1) = _3;
_4 = core::ptr::addr_of!(RET);
(*_4) = 5655447117458785019_i64 as f32;
(*_1) = !_3;
(*_1) = _3 << _3;
(*_4) = (-2067352270002341524_i64) as f32;
(*_1) = !_3;
(*_1) = _3 >> _3;
Goto(bb1)
}
bb1 = {
(*_4) = 1663131486895752286_i64 as f32;
(*_1) = _3 & _3;
(*_4) = 75_i8 as f32;
(*_1) = _3;
Goto(bb2)
}
bb2 = {
(*_1) = !_3;
(*_1) = !_3;
(*_4) = 20535_i16 as f32;
(*_1) = _3 | _3;
_2 = !_3;
(*_1) = _3 & _3;
_4 = core::ptr::addr_of!((*_4));
Goto(bb3)
}
bb3 = {
(*_1) = _3 * _3;
(*_1) = (*_4) as u16;
(*_4) = 34_u8 as f32;
(*_1) = _3 + _3;
(*_1) = 279750693553341463732747230927348877241_u128 as u16;
(*_4) = (-165648232661037988222987322782726041385_i128) as f32;
Goto(bb4)
}
bb4 = {
(*_4) = (-3015090556902154553_i64) as f32;
(*_1) = _3 ^ _3;
(*_1) = _3 + _3;
Call((*_4) = fn8((*_1), Move(_4), Move(_1), (*_1), (*_1), (*_1), (*_1), (*_1), (*_1)), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_4 = core::ptr::addr_of!(RET);
_6 = 105176435673483549404645353486199325282_i128;
(*_4) = 7500633993498522530_i64 as f32;
(*_4) = 1266142725166125724_i64 as f32;
(*_4) = 183_u8 as f32;
_3 = _2 | _2;
(*_4) = 53_i8 as f32;
(*_4) = (-6588049588338088166_i64) as f32;
(*_4) = _3 as f32;
(*_4) = 1886860964_i32 as f32;
(*_4) = (-9223372036854775808_isize) as f32;
(*_4) = 1236659447_i32 as f32;
(*_4) = (-5280061654436317683_i64) as f32;
(*_4) = 167160306065107461164771183634015119956_u128 as f32;
(*_4) = (-41_isize) as f32;
RET = 7437535_u32 as f32;
(*_4) = 7_usize as f32;
_7 = core::ptr::addr_of!(_6);
match (*_7) {
0 => bb4,
1 => bb2,
2 => bb3,
3 => bb6,
4 => bb7,
5 => bb8,
105176435673483549404645353486199325282 => bb10,
_ => bb9
}
}
bb6 = {
(*_4) = (-3015090556902154553_i64) as f32;
(*_1) = _3 ^ _3;
(*_1) = _3 + _3;
Call((*_4) = fn8((*_1), Move(_4), Move(_1), (*_1), (*_1), (*_1), (*_1), (*_1), (*_1)), ReturnTo(bb5), UnwindUnreachable())
}
bb7 = {
(*_1) = _3 * _3;
(*_1) = (*_4) as u16;
(*_4) = 34_u8 as f32;
(*_1) = _3 + _3;
(*_1) = 279750693553341463732747230927348877241_u128 as u16;
(*_4) = (-165648232661037988222987322782726041385_i128) as f32;
Goto(bb4)
}
bb8 = {
(*_1) = !_3;
(*_1) = !_3;
(*_4) = 20535_i16 as f32;
(*_1) = _3 | _3;
_2 = !_3;
(*_1) = _3 & _3;
_4 = core::ptr::addr_of!((*_4));
Goto(bb3)
}
bb9 = {
(*_4) = 1663131486895752286_i64 as f32;
(*_1) = _3 & _3;
(*_4) = 75_i8 as f32;
(*_1) = _3;
Goto(bb2)
}
bb10 = {
(*_7) = -(-138093324647246110847663492209347648539_i128);
(*_7) = 2870657134226737314_u64 as i128;
_2 = 3892805021_u32 as u16;
(*_4) = 3022460324920744033_u64 as f32;
_1 = core::ptr::addr_of_mut!(_2);
(*_7) = 137789446542782370334517932731617932305_i128 ^ (-110887020415139251651942486183528039796_i128);
_1 = core::ptr::addr_of_mut!((*_1));
(*_4) = (*_1) as f32;
(*_4) = 11235504284726640787_u64 as f32;
(*_4) = 30_u8 as f32;
(*_7) = 169804624424459709238261474817910534844_i128 ^ (-119869282801393283687208533665060233504_i128);
(*_1) = _3;
_13 = 17239818148810486748_u64 as isize;
RET = (*_1) as f32;
Goto(bb11)
}
bb11 = {
(*_1) = _3;
(*_7) = -(-146164553316214527924849641765320873845_i128);
(*_7) = 72384425985161227965472731340520117086_i128 << (*_1);
(*_4) = 167260801_u32 as f32;
(*_1) = 17431601119494963569_u64 as u16;
(*_4) = (-638030112_i32) as f32;
(*_4) = 3798547143807883198_i64 as f32;
Goto(bb12)
}
bb12 = {
(*_1) = _3 - _3;
(*_1) = _3 << (*_7);
(*_4) = (-642341691_i32) as f32;
(*_1) = 2791350496887619797_u64 as u16;
(*_1) = _3 | _3;
(*_7) = -(-19915752373302077876550456007452572819_i128);
(*_1) = _3 ^ _3;
(*_4) = 21612_i16 as f32;
(*_4) = (-113_i8) as f32;
(*_1) = (*_4) as u16;
_6 = 1600476881880700336_u64 as i128;
(*_7) = (-89851456744516010823256252358825958909_i128);
(*_7) = '\u{195c5}' as i128;
(*_1) = '\u{3a350}' as u16;
_8 = 2072951948_u32 >> (*_7);
(*_1) = _3;
(*_7) = 72766200261151535648851576997694543596_i128 * (-65913329989282072468972960887945719110_i128);
_10 = 8701_i16 | (-22259_i16);
(*_7) = _13 as i128;
(*_4) = (*_1) as f32;
_16 = (-64_i8) + 73_i8;
(*_1) = _3 & _3;
_16 = 87_i8 + (-41_i8);
(*_7) = _8 as i128;
_3 = _2 << (*_1);
Goto(bb13)
}
bb13 = {
(*_7) = -(-62554880619886722740935590386155520121_i128);
(*_1) = _3 * _3;
(*_4) = _13 as f32;
_13 = (-9223372036854775808_isize) * (-9223372036854775808_isize);
(*_7) = 95820709330850080963720851699894106322_i128 | (-51232690890954901986753721710722252024_i128);
(*_7) = 2177875092861546991_u64 as i128;
_9 = 11678834973314599968_u64 as f64;
(*_4) = _8 as f32;
_10 = (-15643_i16);
(*_7) = 160142854808026018133958612544467524483_i128;
(*_7) = 8497768847484158538_i64 as i128;
_12 = -_13;
(*_7) = (-96844095251268724759402678995192607264_i128) & 91047988107880943288614784492064163695_i128;
(*_7) = (-34525142244797610540223304634916454009_i128) * 92902339368548127311435734468210775338_i128;
(*_1) = _3 >> _3;
_17 = &mut _8;
(*_7) = !13488020868379339280652305661941155194_i128;
_1 = core::ptr::addr_of_mut!((*_1));
(*_7) = -(-99938133110553492353111383081305051169_i128);
(*_4) = 149001640051828521589528327594561739324_u128 as f32;
match _10 {
0 => bb12,
1 => bb7,
2 => bb14,
3 => bb15,
340282366920938463463374607431768195813 => bb17,
_ => bb16
}
}
bb14 = {
(*_4) = (-3015090556902154553_i64) as f32;
(*_1) = _3 ^ _3;
(*_1) = _3 + _3;
Call((*_4) = fn8((*_1), Move(_4), Move(_1), (*_1), (*_1), (*_1), (*_1), (*_1), (*_1)), ReturnTo(bb5), UnwindUnreachable())
}
bb15 = {
(*_1) = _3;
(*_7) = -(-146164553316214527924849641765320873845_i128);
(*_7) = 72384425985161227965472731340520117086_i128 << (*_1);
(*_4) = 167260801_u32 as f32;
(*_1) = 17431601119494963569_u64 as u16;
(*_4) = (-638030112_i32) as f32;
(*_4) = 3798547143807883198_i64 as f32;
Goto(bb12)
}
bb16 = {
(*_7) = -(-138093324647246110847663492209347648539_i128);
(*_7) = 2870657134226737314_u64 as i128;
_2 = 3892805021_u32 as u16;
(*_4) = 3022460324920744033_u64 as f32;
_1 = core::ptr::addr_of_mut!(_2);
(*_7) = 137789446542782370334517932731617932305_i128 ^ (-110887020415139251651942486183528039796_i128);
_1 = core::ptr::addr_of_mut!((*_1));
(*_4) = (*_1) as f32;
(*_4) = 11235504284726640787_u64 as f32;
(*_4) = 30_u8 as f32;
(*_7) = 169804624424459709238261474817910534844_i128 ^ (-119869282801393283687208533665060233504_i128);
(*_1) = _3;
_13 = 17239818148810486748_u64 as isize;
RET = (*_1) as f32;
Goto(bb11)
}
bb17 = {
_18 = !false;
(*_17) = 1658333801_u32 + 1922114296_u32;
(*_7) = (-66291550826669541570003812771683112157_i128);
_4 = core::ptr::addr_of!((*_4));
(*_4) = (*_1) as f32;
RET = _10 as f32;
(*_7) = _9 as i128;
(*_7) = _10 as i128;
Goto(bb18)
}
bb18 = {
(*_7) = !103857651263402312785371316927508805606_i128;
(*_1) = _3 + _3;
(*_7) = (*_17) as i128;
(*_4) = 14277054913561021488_usize as f32;
(*_17) = (*_1) as u32;
(*_7) = 9616804507807690344662188917959693317_i128;
(*_4) = 1255129830599444244_usize as f32;
(*_7) = _12 as i128;
match _10 {
340282366920938463463374607431768195813 => bb20,
_ => bb19
}
}
bb19 = {
_18 = !false;
(*_17) = 1658333801_u32 + 1922114296_u32;
(*_7) = (-66291550826669541570003812771683112157_i128);
_4 = core::ptr::addr_of!((*_4));
(*_4) = (*_1) as f32;
RET = _10 as f32;
(*_7) = _9 as i128;
(*_7) = _10 as i128;
Goto(bb18)
}
bb20 = {
(*_7) = -(-32445707221703420344952868008673503501_i128);
(*_7) = (-84643668723720772016194911868453461873_i128);
(*_17) = 13640114202850944776_u64 as u32;
(*_7) = (-137581545486682565944037185938973124673_i128);
(*_4) = _13 as f32;
Goto(bb21)
}
bb21 = {
(*_7) = (-161384003638287845255975721846360614739_i128) >> (*_1);
(*_4) = 90_u8 as f32;
(*_17) = !4019558040_u32;
_21 = !6568380602463587617_i64;
_9 = _10 as f64;
(*_4) = 6568440559294611286_u64 as f32;
(*_1) = _3 >> (*_7);
(*_4) = (*_7) as f32;
_25 = -_13;
(*_17) = 191_u8 as u32;
(*_17) = 3315719616_u32 ^ 2441100841_u32;
(*_17) = 2939541484_u32 * 3122197685_u32;
RET = _21 as f32;
(*_7) = (-141449537503722788685112989578518246232_i128);
(*_1) = _3 >> _3;
(*_1) = _3 & _3;
(*_17) = 1341757615_u32 ^ 3555143034_u32;
(*_7) = _9 as i128;
match _10 {
0 => bb13,
1 => bb7,
2 => bb22,
3 => bb23,
4 => bb24,
340282366920938463463374607431768195813 => bb26,
_ => bb25
}
}
bb22 = {
(*_1) = _3 - _3;
(*_1) = _3 << (*_7);
(*_4) = (-642341691_i32) as f32;
(*_1) = 2791350496887619797_u64 as u16;
(*_1) = _3 | _3;
(*_7) = -(-19915752373302077876550456007452572819_i128);
(*_1) = _3 ^ _3;
(*_4) = 21612_i16 as f32;
(*_4) = (-113_i8) as f32;
(*_1) = (*_4) as u16;
_6 = 1600476881880700336_u64 as i128;
(*_7) = (-89851456744516010823256252358825958909_i128);
(*_7) = '\u{195c5}' as i128;
(*_1) = '\u{3a350}' as u16;
_8 = 2072951948_u32 >> (*_7);
(*_1) = _3;
(*_7) = 72766200261151535648851576997694543596_i128 * (-65913329989282072468972960887945719110_i128);
_10 = 8701_i16 | (-22259_i16);
(*_7) = _13 as i128;
(*_4) = (*_1) as f32;
_16 = (-64_i8) + 73_i8;
(*_1) = _3 & _3;
_16 = 87_i8 + (-41_i8);
(*_7) = _8 as i128;
_3 = _2 << (*_1);
Goto(bb13)
}
bb23 = {
_18 = !false;
(*_17) = 1658333801_u32 + 1922114296_u32;
(*_7) = (-66291550826669541570003812771683112157_i128);
_4 = core::ptr::addr_of!((*_4));
(*_4) = (*_1) as f32;
RET = _10 as f32;
(*_7) = _9 as i128;
(*_7) = _10 as i128;
Goto(bb18)
}
bb24 = {
(*_4) = (-3015090556902154553_i64) as f32;
(*_1) = _3 ^ _3;
(*_1) = _3 + _3;
Call((*_4) = fn8((*_1), Move(_4), Move(_1), (*_1), (*_1), (*_1), (*_1), (*_1), (*_1)), ReturnTo(bb5), UnwindUnreachable())
}
bb25 = {
_18 = !false;
(*_17) = 1658333801_u32 + 1922114296_u32;
(*_7) = (-66291550826669541570003812771683112157_i128);
_4 = core::ptr::addr_of!((*_4));
(*_4) = (*_1) as f32;
RET = _10 as f32;
(*_7) = _9 as i128;
(*_7) = _10 as i128;
Goto(bb18)
}
bb26 = {
(*_4) = _12 as f32;
(*_4) = (*_7) as f32;
(*_1) = _3 ^ _3;
(*_1) = _3 * _3;
(*_4) = 16451035959883587930_usize as f32;
(*_4) = _9 as f32;
(*_1) = !_3;
(*_17) = 1506563158_u32;
_23 = (*_4);
(*_17) = 937741700_u32 * 1723653699_u32;
(*_17) = 4087635742_u32;
_24 = [(-2144887490_i32),(-353712316_i32),(-1037492832_i32),1643377281_i32,2078647665_i32];
Goto(bb27)
}
bb27 = {
(*_4) = _23 - _23;
(*_7) = (-10903453974113885449539492672554666455_i128) + 93454206600114372349254609958914025298_i128;
(*_1) = !_3;
(*_1) = _3 + _3;
(*_1) = _3;
_13 = (*_4) as isize;
_32.1.3 = _10 >> (*_1);
(*_1) = !_3;
(*_4) = -_23;
_29 = '\u{525a4}';
(*_17) = _21 as u32;
(*_7) = 57602264555431726217149438092889248511_i128;
_32.1.2 = [(*_17),(*_17),(*_17),(*_17),(*_17),(*_17),(*_17),(*_17)];
(*_4) = _23 + _23;
(*_1) = (*_4) as u16;
_34.1 = 291768053845847624142963971864160336483_u128 & 235681329293520972917445946892488482512_u128;
(*_7) = (-13641180927262380963287111733129976148_i128) << (*_17);
(*_4) = _21 as f32;
match _10 {
0 => bb14,
1 => bb19,
2 => bb8,
3 => bb28,
4 => bb29,
5 => bb30,
340282366920938463463374607431768195813 => bb32,
_ => bb31
}
}
bb28 = {
(*_4) = _12 as f32;
(*_4) = (*_7) as f32;
(*_1) = _3 ^ _3;
(*_1) = _3 * _3;
(*_4) = 16451035959883587930_usize as f32;
(*_4) = _9 as f32;
(*_1) = !_3;
(*_17) = 1506563158_u32;
_23 = (*_4);
(*_17) = 937741700_u32 * 1723653699_u32;
(*_17) = 4087635742_u32;
_24 = [(-2144887490_i32),(-353712316_i32),(-1037492832_i32),1643377281_i32,2078647665_i32];
Goto(bb27)
}
bb29 = {
(*_7) = !103857651263402312785371316927508805606_i128;
(*_1) = _3 + _3;
(*_7) = (*_17) as i128;
(*_4) = 14277054913561021488_usize as f32;
(*_17) = (*_1) as u32;
(*_7) = 9616804507807690344662188917959693317_i128;
(*_4) = 1255129830599444244_usize as f32;
(*_7) = _12 as i128;
match _10 {
340282366920938463463374607431768195813 => bb20,
_ => bb19
}
}
bb30 = {
(*_1) = _3 * _3;
(*_1) = (*_4) as u16;
(*_4) = 34_u8 as f32;
(*_1) = _3 + _3;
(*_1) = 279750693553341463732747230927348877241_u128 as u16;
(*_4) = (-165648232661037988222987322782726041385_i128) as f32;
Goto(bb4)
}
bb31 = {
(*_1) = _3 * _3;
(*_1) = (*_4) as u16;
(*_4) = 34_u8 as f32;
(*_1) = _3 + _3;
(*_1) = 279750693553341463732747230927348877241_u128 as u16;
(*_4) = (-165648232661037988222987322782726041385_i128) as f32;
Goto(bb4)
}
bb32 = {
_25 = _13 & _12;
_32.1.1 = &_21;
_33 = _29;
(*_7) = 16738193338727090967770694268968594581_i128 ^ (-108056239354663933261004185238102546403_i128);
_21 = (-8353884169977120500_i64) * 2737951885653906178_i64;
(*_17) = 3734918705_u32 + 530167084_u32;
(*_1) = !_3;
(*_4) = _23 - _23;
_3 = (*_1);
_32.0 = &_34.1;
match _10 {
0 => bb12,
1 => bb30,
340282366920938463463374607431768195813 => bb34,
_ => bb33
}
}
bb33 = {
(*_7) = -(-138093324647246110847663492209347648539_i128);
(*_7) = 2870657134226737314_u64 as i128;
_2 = 3892805021_u32 as u16;
(*_4) = 3022460324920744033_u64 as f32;
_1 = core::ptr::addr_of_mut!(_2);
(*_7) = 137789446542782370334517932731617932305_i128 ^ (-110887020415139251651942486183528039796_i128);
_1 = core::ptr::addr_of_mut!((*_1));
(*_4) = (*_1) as f32;
(*_4) = 11235504284726640787_u64 as f32;
(*_4) = 30_u8 as f32;
(*_7) = 169804624424459709238261474817910534844_i128 ^ (-119869282801393283687208533665060233504_i128);
(*_1) = _3;
_13 = 17239818148810486748_u64 as isize;
RET = (*_1) as f32;
Goto(bb11)
}
bb34 = {
_36 = -_32.1.3;
_32.1.0 = _32.1.3;
(*_7) = (-49505926602051313673933334272789556985_i128) + (-79957797568322449962511851268242400383_i128);
_33 = _29;
_13 = _25 | _25;
(*_7) = (-166834931118324116679942349383629120742_i128) ^ (-159878918106869213893417688351719886182_i128);
match _10 {
0 => bb5,
340282366920938463463374607431768195813 => bb35,
_ => bb19
}
}
bb35 = {
_32.2 = &(*_17);
(*_17) = 3732637684_u32 - 27241843_u32;
_25 = 2891826539723829837_u64 as isize;
_22 = _25 | _25;
(*_17) = 3528669064_u32 ^ 3575337726_u32;
_4 = core::ptr::addr_of!((*_4));
(*_1) = _3;
_9 = 4_usize as f64;
(*_1) = _3;
match _10 {
340282366920938463463374607431768195813 => bb36,
_ => bb15
}
}
bb36 = {
(*_7) = -43085144735695728277623266684047846369_i128;
(*_7) = 58693752987832374889488369945776479425_i128 ^ 44922815816682845877536013789308658353_i128;
(*_4) = _23 - _23;
(*_1) = !_3;
_26 = (-326124032_i32) | (-614293689_i32);
_29 = _33;
(*_4) = _23 - _23;
(*_1) = _3 & _3;
(*_7) = !156290334628034631379802414224537344356_i128;
(*_1) = !_3;
(*_17) = 1342215895_u32 ^ 525072512_u32;
(*_17) = 2812483417_u32;
(*_17) = 1441368836_u32 * 1064857877_u32;
(*_1) = _3 ^ _3;
(*_1) = !_3;
(*_4) = -_23;
_36 = _32.1.3 + _32.1.3;
_33 = _29;
match _10 {
0 => bb11,
1 => bb15,
2 => bb37,
3 => bb38,
4 => bb39,
5 => bb40,
6 => bb41,
340282366920938463463374607431768195813 => bb43,
_ => bb42
}
}
bb37 = {
(*_4) = 1663131486895752286_i64 as f32;
(*_1) = _3 & _3;
(*_4) = 75_i8 as f32;
(*_1) = _3;
Goto(bb2)
}
bb38 = {
(*_4) = _23 - _23;
(*_7) = (-10903453974113885449539492672554666455_i128) + 93454206600114372349254609958914025298_i128;
(*_1) = !_3;
(*_1) = _3 + _3;
(*_1) = _3;
_13 = (*_4) as isize;
_32.1.3 = _10 >> (*_1);
(*_1) = !_3;
(*_4) = -_23;
_29 = '\u{525a4}';
(*_17) = _21 as u32;
(*_7) = 57602264555431726217149438092889248511_i128;
_32.1.2 = [(*_17),(*_17),(*_17),(*_17),(*_17),(*_17),(*_17),(*_17)];
(*_4) = _23 + _23;
(*_1) = (*_4) as u16;
_34.1 = 291768053845847624142963971864160336483_u128 & 235681329293520972917445946892488482512_u128;
(*_7) = (-13641180927262380963287111733129976148_i128) << (*_17);
(*_4) = _21 as f32;
match _10 {
0 => bb14,
1 => bb19,
2 => bb8,
3 => bb28,
4 => bb29,
5 => bb30,
340282366920938463463374607431768195813 => bb32,
_ => bb31
}
}
bb39 = {
(*_4) = (-3015090556902154553_i64) as f32;
(*_1) = _3 ^ _3;
(*_1) = _3 + _3;
Call((*_4) = fn8((*_1), Move(_4), Move(_1), (*_1), (*_1), (*_1), (*_1), (*_1), (*_1)), ReturnTo(bb5), UnwindUnreachable())
}
bb40 = {
_18 = !false;
(*_17) = 1658333801_u32 + 1922114296_u32;
(*_7) = (-66291550826669541570003812771683112157_i128);
_4 = core::ptr::addr_of!((*_4));
(*_4) = (*_1) as f32;
RET = _10 as f32;
(*_7) = _9 as i128;
(*_7) = _10 as i128;
Goto(bb18)
}
bb41 = {
(*_7) = -(-138093324647246110847663492209347648539_i128);
(*_7) = 2870657134226737314_u64 as i128;
_2 = 3892805021_u32 as u16;
(*_4) = 3022460324920744033_u64 as f32;
_1 = core::ptr::addr_of_mut!(_2);
(*_7) = 137789446542782370334517932731617932305_i128 ^ (-110887020415139251651942486183528039796_i128);
_1 = core::ptr::addr_of_mut!((*_1));
(*_4) = (*_1) as f32;
(*_4) = 11235504284726640787_u64 as f32;
(*_4) = 30_u8 as f32;
(*_7) = 169804624424459709238261474817910534844_i128 ^ (-119869282801393283687208533665060233504_i128);
(*_1) = _3;
_13 = 17239818148810486748_u64 as isize;
RET = (*_1) as f32;
Goto(bb11)
}
bb42 = {
(*_7) = !103857651263402312785371316927508805606_i128;
(*_1) = _3 + _3;
(*_7) = (*_17) as i128;
(*_4) = 14277054913561021488_usize as f32;
(*_17) = (*_1) as u32;
(*_7) = 9616804507807690344662188917959693317_i128;
(*_4) = 1255129830599444244_usize as f32;
(*_7) = _12 as i128;
match _10 {
340282366920938463463374607431768195813 => bb20,
_ => bb19
}
}
bb43 = {
(*_7) = 112896393388916439552241297187890320689_i128;
_9 = (*_1) as f64;
_42 = &_32.1.0;
_4 = core::ptr::addr_of!((*_4));
(*_1) = (*_4) as u16;
(*_17) = 289315221_u32 + 2172131145_u32;
(*_7) = -143974320181989881217559374456793929139_i128;
_38 = core::ptr::addr_of_mut!(_22);
(*_17) = 1196516097_u32;
(*_7) = 115493138797825130661279624829293674368_i128 >> (*_42);
(*_4) = _23;
(*_17) = 169781854_u32;
(*_38) = _13 & _13;
_18 = (*_42) == (*_42);
(*_7) = 137246668730700509042871568095759202957_i128 | (-24401300791321222008029310985818252609_i128);
(*_4) = _23 * _23;
(*_38) = _13 | _13;
_25 = (*_38) | (*_38);
(*_17) = (*_38) as u32;
(*_4) = -_23;
(*_17) = !265479649_u32;
(*_38) = !_25;
(*_1) = _3 + _3;
(*_1) = _3 + _3;
(*_17) = !943541117_u32;
(*_4) = _23;
(*_4) = -_23;
match _10 {
0 => bb16,
1 => bb44,
2 => bb45,
3 => bb46,
4 => bb47,
340282366920938463463374607431768195813 => bb49,
_ => bb48
}
}
bb44 = {
(*_4) = (-3015090556902154553_i64) as f32;
(*_1) = _3 ^ _3;
(*_1) = _3 + _3;
Call((*_4) = fn8((*_1), Move(_4), Move(_1), (*_1), (*_1), (*_1), (*_1), (*_1), (*_1)), ReturnTo(bb5), UnwindUnreachable())
}
bb45 = {
(*_4) = 1663131486895752286_i64 as f32;
(*_1) = _3 & _3;
(*_4) = 75_i8 as f32;
(*_1) = _3;
Goto(bb2)
}
bb46 = {
_18 = !false;
(*_17) = 1658333801_u32 + 1922114296_u32;
(*_7) = (-66291550826669541570003812771683112157_i128);
_4 = core::ptr::addr_of!((*_4));
(*_4) = (*_1) as f32;
RET = _10 as f32;
(*_7) = _9 as i128;
(*_7) = _10 as i128;
Goto(bb18)
}
bb47 = {
(*_7) = (-161384003638287845255975721846360614739_i128) >> (*_1);
(*_4) = 90_u8 as f32;
(*_17) = !4019558040_u32;
_21 = !6568380602463587617_i64;
_9 = _10 as f64;
(*_4) = 6568440559294611286_u64 as f32;
(*_1) = _3 >> (*_7);
(*_4) = (*_7) as f32;
_25 = -_13;
(*_17) = 191_u8 as u32;
(*_17) = 3315719616_u32 ^ 2441100841_u32;
(*_17) = 2939541484_u32 * 3122197685_u32;
RET = _21 as f32;
(*_7) = (-141449537503722788685112989578518246232_i128);
(*_1) = _3 >> _3;
(*_1) = _3 & _3;
(*_17) = 1341757615_u32 ^ 3555143034_u32;
(*_7) = _9 as i128;
match _10 {
0 => bb13,
1 => bb7,
2 => bb22,
3 => bb23,
4 => bb24,
340282366920938463463374607431768195813 => bb26,
_ => bb25
}
}
bb48 = {
_36 = -_32.1.3;
_32.1.0 = _32.1.3;
(*_7) = (-49505926602051313673933334272789556985_i128) + (-79957797568322449962511851268242400383_i128);
_33 = _29;
_13 = _25 | _25;
(*_7) = (-166834931118324116679942349383629120742_i128) ^ (-159878918106869213893417688351719886182_i128);
match _10 {
0 => bb5,
340282366920938463463374607431768195813 => bb35,
_ => bb19
}
}
bb49 = {
_10 = (*_42);
(*_1) = _9 as u16;
(*_17) = 88882980_u32;
(*_1) = !_3;
_11 = Move(_32.0);
RET = _23 * _23;
(*_4) = _23;
_1 = core::ptr::addr_of_mut!((*_1));
(*_4) = _23 * _23;
_34.1 = !256703891242752663481702493105784119506_u128;
(*_38) = _25 << (*_42);
_43 = (*_1) == (*_1);
(*_17) = 2788221204_u32 << (*_38);
(*_7) = 163203703635393082883759836648205512354_i128 ^ 79763221010369615687931251456897975031_i128;
_12 = (*_17) as isize;
(*_17) = 3696570837_u32;
(*_17) = 3683793507_u32 >> (*_38);
_32.1.0 = _36 | _36;
_38 = core::ptr::addr_of_mut!((*_38));
_32.1.3 = (*_4) as i16;
Goto(bb50)
}
bb50 = {
Call(_49 = dump_var(Move(_25), Move(_26), Move(_29), Move(_8)), ReturnTo(bb51), UnwindUnreachable())
}
bb51 = {
Call(_49 = dump_var(Move(_18), Move(_16), Move(_22), Move(_10)), ReturnTo(bb52), UnwindUnreachable())
}
bb52 = {
Call(_49 = dump_var(Move(_13), _50, _50, _50), ReturnTo(bb53), UnwindUnreachable())
}
bb53 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn8(mut _1: u16,mut _2: *const f32,mut _3: *mut u16,mut _4: u16,mut _5: u16,mut _6: u16,mut _7: u16,mut _8: u16,mut _9: u16) -> f32 {
mir! {
type RET = f32;
let _10: *mut &'static *mut isize;
let _11: i64;
let _12: &'static mut u32;
let _13: &'static *mut isize;
let _14: char;
let _15: isize;
let _16: u64;
let _17: bool;
let _18: isize;
let _19: i128;
let _20: *mut Adt73;
let _21: [u32; 8];
let _22: (Adt22, ((i64,), &'static char), Adt52, *mut [isize; 5]);
let _23: bool;
let _24: i64;
let _25: &'static mut [i8; 2];
let _26: (i16, &'static i64, [u32; 8], i16);
let _27: isize;
let _28: (&'static &'static char, i64, i16, i64);
let _29: *const (f32, i16, usize, u8);
let _30: char;
let _31: (&'static char, [u128; 7], *const f32, *const [i32; 5]);
let _32: isize;
let _33: u64;
let _34: f64;
let _35: ();
let _36: ();
{
_7 = !_1;
_1 = 139583514370632811324521105062458485988_u128 as u16;
_3 = core::ptr::addr_of_mut!(_1);
(*_3) = _9 ^ _8;
Goto(bb1)
}
bb1 = {
RET = 284772922857909186336184732426827562428_u128 as f32;
(*_3) = _6;
_6 = (*_3) * (*_3);
_9 = (*_3);
_4 = 163_u8 as u16;
(*_3) = _7 - _7;
(*_3) = _9 << _6;
_11 = (-2930109073808610647_i64);
(*_3) = _7 & _5;
(*_3) = _11 as u16;
(*_3) = 10065016976689838752_u64 as u16;
(*_3) = _7 ^ _7;
Call((*_3) = fn9(_8, Move(_3), _11), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_4 = !_7;
_7 = _1;
_2 = core::ptr::addr_of!(RET);
_3 = core::ptr::addr_of_mut!(_6);
_11 = 6874102856334034986_i64;
(*_3) = 9223372036854775807_isize as u16;
(*_3) = _1;
Goto(bb3)
}
bb3 = {
(*_3) = true as u16;
(*_3) = (-102441467206982789889761932449448490617_i128) as u16;
(*_3) = _1 >> _11;
(*_3) = !_7;
(*_2) = 217_u8 as f32;
(*_2) = 106474764132578537880185555455737348442_u128 as f32;
(*_3) = (-78650805715844791891688943742073034520_i128) as u16;
(*_2) = 274461033_u32 as f32;
Call((*_3) = core::intrinsics::bswap(_8), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
(*_3) = !_1;
_7 = (*_3) * (*_3);
(*_2) = 9223372036854775807_isize as f32;
(*_2) = _11 as f32;
(*_2) = (-1141388133_i32) as f32;
(*_3) = _7 >> _9;
_3 = core::ptr::addr_of_mut!((*_3));
_15 = (-9223372036854775808_isize) + 9223372036854775807_isize;
(*_3) = _7 - _7;
(*_3) = _5 << _7;
match _11 {
0 => bb5,
1 => bb6,
2 => bb7,
3 => bb8,
4 => bb9,
6874102856334034986 => bb11,
_ => bb10
}
}
bb5 = {
(*_3) = true as u16;
(*_3) = (-102441467206982789889761932449448490617_i128) as u16;
(*_3) = _1 >> _11;
(*_3) = !_7;
(*_2) = 217_u8 as f32;
(*_2) = 106474764132578537880185555455737348442_u128 as f32;
(*_3) = (-78650805715844791891688943742073034520_i128) as u16;
(*_2) = 274461033_u32 as f32;
Call((*_3) = core::intrinsics::bswap(_8), ReturnTo(bb4), UnwindUnreachable())
}
bb6 = {
_4 = !_7;
_7 = _1;
_2 = core::ptr::addr_of!(RET);
_3 = core::ptr::addr_of_mut!(_6);
_11 = 6874102856334034986_i64;
(*_3) = 9223372036854775807_isize as u16;
(*_3) = _1;
Goto(bb3)
}
bb7 = {
RET = 284772922857909186336184732426827562428_u128 as f32;
(*_3) = _6;
_6 = (*_3) * (*_3);
_9 = (*_3);
_4 = 163_u8 as u16;
(*_3) = _7 - _7;
(*_3) = _9 << _6;
_11 = (-2930109073808610647_i64);
(*_3) = _7 & _5;
(*_3) = _11 as u16;
(*_3) = 10065016976689838752_u64 as u16;
(*_3) = _7 ^ _7;
Call((*_3) = fn9(_8, Move(_3), _11), ReturnTo(bb2), UnwindUnreachable())
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
(*_2) = _15 as f32;
(*_3) = (-22618_i16) as u16;
(*_2) = 15636377706421166324828345825219452441_i128 as f32;
(*_3) = _1 >> _15;
(*_2) = 46183667890032560642032999766035837071_u128 as f32;
(*_3) = !_7;
_16 = 1630566152327079753_u64 ^ 1912894304181851288_u64;
(*_2) = 74_i8 as f32;
_11 = (-473162673861256501_i64);
(*_3) = _1 >> _1;
(*_2) = 82524861080909771396431118683766467810_i128 as f32;
(*_3) = 137776370064904586739261968830889772258_u128 as u16;
(*_3) = !_7;
match _11 {
0 => bb4,
1 => bb6,
2 => bb12,
340282366920938463462901444757906954955 => bb14,
_ => bb13
}
}
bb12 = {
Return()
}
bb13 = {
(*_3) = true as u16;
(*_3) = (-102441467206982789889761932449448490617_i128) as u16;
(*_3) = _1 >> _11;
(*_3) = !_7;
(*_2) = 217_u8 as f32;
(*_2) = 106474764132578537880185555455737348442_u128 as f32;
(*_3) = (-78650805715844791891688943742073034520_i128) as u16;
(*_2) = 274461033_u32 as f32;
Call((*_3) = core::intrinsics::bswap(_8), ReturnTo(bb4), UnwindUnreachable())
}
bb14 = {
RET = (-131153959507693763104314147821540655976_i128) as f32;
(*_2) = (-1983517807_i32) as f32;
(*_3) = _1;
_22.0.fld0 = false;
(*_2) = 1651207563_u32 as f32;
(*_2) = 6_usize as f32;
_19 = 166567123977436176366276006483772486319_i128;
(*_3) = _7 ^ _5;
(*_2) = (*_3) as f32;
_15 = (-9223372036854775808_isize) >> (*_3);
(*_3) = _7 + _1;
(*_2) = _15 as f32;
_26.3 = (-14685_i16) >> (*_3);
(*_2) = 100_u8 as f32;
_22.0.fld3 = 124_i8 * (-84_i8);
_28.2 = _26.3;
(*_3) = _7 * _1;
(*_2) = _19 as f32;
(*_3) = !_7;
(*_2) = _26.3 as f32;
(*_2) = 2_usize as f32;
match _11 {
0 => bb1,
1 => bb3,
340282366920938463462901444757906954955 => bb16,
_ => bb15
}
}
bb15 = {
RET = 284772922857909186336184732426827562428_u128 as f32;
(*_3) = _6;
_6 = (*_3) * (*_3);
_9 = (*_3);
_4 = 163_u8 as u16;
(*_3) = _7 - _7;
(*_3) = _9 << _6;
_11 = (-2930109073808610647_i64);
(*_3) = _7 & _5;
(*_3) = _11 as u16;
(*_3) = 10065016976689838752_u64 as u16;
(*_3) = _7 ^ _7;
Call((*_3) = fn9(_8, Move(_3), _11), ReturnTo(bb2), UnwindUnreachable())
}
bb16 = {
_26.0 = !_28.2;
_14 = '\u{cb99}';
(*_2) = _19 as f32;
(*_3) = _7 << _26.0;
_22.1.0.0 = !_11;
(*_3) = _1 & _9;
(*_2) = _22.0.fld3 as f32;
match _19 {
0 => bb2,
1 => bb17,
2 => bb18,
3 => bb19,
4 => bb20,
5 => bb21,
166567123977436176366276006483772486319 => bb23,
_ => bb22
}
}
bb17 = {
RET = 284772922857909186336184732426827562428_u128 as f32;
(*_3) = _6;
_6 = (*_3) * (*_3);
_9 = (*_3);
_4 = 163_u8 as u16;
(*_3) = _7 - _7;
(*_3) = _9 << _6;
_11 = (-2930109073808610647_i64);
(*_3) = _7 & _5;
(*_3) = _11 as u16;
(*_3) = 10065016976689838752_u64 as u16;
(*_3) = _7 ^ _7;
Call((*_3) = fn9(_8, Move(_3), _11), ReturnTo(bb2), UnwindUnreachable())
}
bb18 = {
Return()
}
bb19 = {
Return()
}
bb20 = {
_4 = !_7;
_7 = _1;
_2 = core::ptr::addr_of!(RET);
_3 = core::ptr::addr_of_mut!(_6);
_11 = 6874102856334034986_i64;
(*_3) = 9223372036854775807_isize as u16;
(*_3) = _1;
Goto(bb3)
}
bb21 = {
(*_3) = true as u16;
(*_3) = (-102441467206982789889761932449448490617_i128) as u16;
(*_3) = _1 >> _11;
(*_3) = !_7;
(*_2) = 217_u8 as f32;
(*_2) = 106474764132578537880185555455737348442_u128 as f32;
(*_3) = (-78650805715844791891688943742073034520_i128) as u16;
(*_2) = 274461033_u32 as f32;
Call((*_3) = core::intrinsics::bswap(_8), ReturnTo(bb4), UnwindUnreachable())
}
bb22 = {
(*_3) = true as u16;
(*_3) = (-102441467206982789889761932449448490617_i128) as u16;
(*_3) = _1 >> _11;
(*_3) = !_7;
(*_2) = 217_u8 as f32;
(*_2) = 106474764132578537880185555455737348442_u128 as f32;
(*_3) = (-78650805715844791891688943742073034520_i128) as u16;
(*_2) = 274461033_u32 as f32;
Call((*_3) = core::intrinsics::bswap(_8), ReturnTo(bb4), UnwindUnreachable())
}
bb23 = {
_17 = !_22.0.fld0;
(*_2) = (-446113880_i32) as f32;
(*_2) = _11 as f32;
(*_2) = _28.2 as f32;
RET = _15 as f32;
(*_2) = 1356186282_i32 as f32;
_22.1.0.0 = _11;
_24 = _22.1.0.0 << (*_3);
(*_3) = _1 - _9;
(*_2) = 3469205947949506530_usize as f32;
_26.3 = _26.0 >> (*_3);
_30 = _14;
_3 = core::ptr::addr_of_mut!((*_3));
(*_3) = _1;
(*_3) = _4 << _26.0;
(*_3) = _9 * _7;
(*_3) = _7 + _1;
(*_3) = _7;
_30 = _14;
_26.1 = &_22.1.0.0;
(*_2) = 60_u8 as f32;
_22.0.fld0 = _6 < (*_3);
(*_2) = _22.0.fld3 as f32;
(*_2) = 217561325064325701579588963321762255632_u128 as f32;
_17 = _22.0.fld0;
_22.0.fld1 = Adt21::Variant0 { fld0: 327667012163341732832969595279221807393_u128,fld1: (-1280849165_i32),fld2: _22.0.fld3 };
(*_2) = (-1471468960_i32) as f32;
_28.2 = _26.3;
Goto(bb24)
}
bb24 = {
_23 = _22.0.fld0 > _22.0.fld0;
_1 = _7 ^ (*_3);
(*_2) = (-585058890_i32) as f32;
_1 = (*_3);
(*_3) = _17 as u16;
(*_2) = _24 as f32;
(*_3) = _30 as u16;
_22.0.fld3 = Field::<i8>(Variant(_22.0.fld1, 0), 2) - Field::<i8>(Variant(_22.0.fld1, 0), 2);
_1 = !_7;
_15 = !(-9223372036854775808_isize);
_28.2 = _26.3 | _26.0;
(*_3) = _19 as u16;
(*_2) = 77657263932912532456463468076228097085_u128 as f32;
(*_2) = _22.0.fld3 as f32;
(*_2) = _19 as f32;
_22.1.0.0 = _24 << _26.0;
_31.2 = core::ptr::addr_of!((*_2));
(*_2) = 14220822282719486189_usize as f32;
(*_2) = _22.1.0.0 as f32;
_22.0.fld0 = !_17;
(*_3) = 1298061047_u32 as u16;
(*_3) = _9 * _5;
(*_3) = _1;
Goto(bb25)
}
bb25 = {
Call(_35 = dump_var(Move(_14), Move(_7), Move(_30), Move(_8)), ReturnTo(bb26), UnwindUnreachable())
}
bb26 = {
Call(_35 = dump_var(Move(_5), Move(_4), Move(_24), Move(_17)), ReturnTo(bb27), UnwindUnreachable())
}
bb27 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn9(mut _1: u16,mut _2: *mut u16,mut _3: i64) -> u16 {
mir! {
type RET = u16;
let _4: u128;
let _5: i128;
let _6: (u32,);
let _7: i16;
let _8: isize;
let _9: *mut &'static *mut isize;
let _10: (u64, u128);
let _11: i16;
let _12: f64;
let _13: i64;
let _14: isize;
let _15: i128;
let _16: bool;
let _17: i16;
let _18: ((&'static mut *mut i128,), i64, f32, &'static char);
let _19: bool;
let _20: *const i64;
let _21: f64;
let _22: &'static i64;
let _23: u128;
let _24: &'static mut [bool; 2];
let _25: *const Adt22;
let _26: (u32,);
let _27: f32;
let _28: isize;
let _29: *mut i128;
let _30: f32;
let _31: isize;
let _32: u128;
let _33: f32;
let _34: *mut Adt73;
let _35: isize;
let _36: i64;
let _37: &'static mut (i64,);
let _38: *mut i128;
let _39: Adt21;
let _40: &'static u128;
let _41: [i128; 1];
let _42: i64;
let _43: char;
let _44: bool;
let _45: i128;
let _46: bool;
let _47: u64;
let _48: *mut [u8; 4];
let _49: &'static mut (i64,);
let _50: ((i64,), &'static char);
let _51: i128;
let _52: *const i128;
let _53: [u32; 8];
let _54: f64;
let _55: Adt73;
let _56: u8;
let _57: &'static i16;
let _58: &'static mut (i64,);
let _59: i128;
let _60: bool;
let _61: isize;
let _62: [i128; 2];
let _63: char;
let _64: &'static mut (i64,);
let _65: *mut u16;
let _66: isize;
let _67: i64;
let _68: f64;
let _69: f64;
let _70: &'static i16;
let _71: *mut u16;
let _72: ();
let _73: ();
{
_2 = core::ptr::addr_of_mut!(_1);
(*_2) = 19593_u16 + 62674_u16;
(*_2) = 52455_u16 + 54952_u16;
(*_2) = 33629_u16;
(*_2) = false as u16;
_1 = false as u16;
_3 = !8792358054854172303_i64;
(*_2) = 27380_u16;
(*_2) = !50547_u16;
_4 = 219230672883021644801439010617692346181_u128 * 102074102259257300621537902840185153885_u128;
_3 = (-6359417820401378011_i64) | 6896402369047394576_i64;
(*_2) = (-9223372036854775808_isize) as u16;
(*_2) = 38933_u16 - 7083_u16;
_2 = core::ptr::addr_of_mut!((*_2));
_6 = (622119483_u32,);
_5 = 129685194025291794034742304879815626500_i128;
(*_2) = !24818_u16;
(*_2) = !61784_u16;
match _6.0 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
622119483 => bb8,
_ => bb7
}
}
bb1 = {
Return()
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
(*_2) = 51633_u16 >> _3;
RET = (*_2) + (*_2);
(*_2) = !RET;
_7 = (-3544_i16) - (-21536_i16);
_3 = 1095905846059820715_i64 + (-7768459529918151538_i64);
(*_2) = RET;
(*_2) = RET - RET;
_7 = (-9223372036854775808_isize) as i16;
(*_2) = RET;
(*_2) = RET * RET;
_8 = !9223372036854775807_isize;
(*_2) = RET << RET;
(*_2) = RET ^ RET;
match _6.0 {
0 => bb9,
1 => bb10,
622119483 => bb12,
_ => bb11
}
}
bb9 = {
Return()
}
bb10 = {
Return()
}
bb11 = {
Return()
}
bb12 = {
(*_2) = RET;
(*_2) = _8 as u16;
(*_2) = !RET;
(*_2) = RET >> _8;
(*_2) = RET * RET;
_6 = (3154969171_u32,);
_5 = 2857836268017766504519362809376009610_i128 * (-39029033601403084137546804581937609204_i128);
_3 = _7 as i64;
_10 = (7772564668240061903_u64, _4);
_2 = core::ptr::addr_of_mut!((*_2));
(*_2) = RET;
(*_2) = RET - RET;
_11 = _7 ^ _7;
(*_2) = !RET;
_8 = -(-104_isize);
_13 = _3;
match _6.0 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb10,
4 => bb5,
5 => bb11,
6 => bb13,
3154969171 => bb15,
_ => bb14
}
}
bb13 = {
Return()
}
bb14 = {
(*_2) = 51633_u16 >> _3;
RET = (*_2) + (*_2);
(*_2) = !RET;
_7 = (-3544_i16) - (-21536_i16);
_3 = 1095905846059820715_i64 + (-7768459529918151538_i64);
(*_2) = RET;
(*_2) = RET - RET;
_7 = (-9223372036854775808_isize) as i16;
(*_2) = RET;
(*_2) = RET * RET;
_8 = !9223372036854775807_isize;
(*_2) = RET << RET;
(*_2) = RET ^ RET;
match _6.0 {
0 => bb9,
1 => bb10,
622119483 => bb12,
_ => bb11
}
}
bb15 = {
(*_2) = RET & RET;
_12 = _8 as f64;
(*_2) = RET + RET;
_3 = !_13;
_2 = core::ptr::addr_of_mut!((*_2));
_14 = -_8;
_11 = !_7;
_7 = _11;
(*_2) = RET * RET;
_13 = _3 | _3;
_2 = core::ptr::addr_of_mut!((*_2));
_10.1 = _4 + _4;
_2 = core::ptr::addr_of_mut!((*_2));
_7 = _11;
_7 = _11 | _11;
_15 = '\u{66fe3}' as i128;
(*_2) = 6_usize as u16;
Goto(bb16)
}
bb16 = {
(*_2) = RET;
(*_2) = RET;
(*_2) = RET - RET;
(*_2) = RET;
_7 = _11 & _11;
(*_2) = RET - RET;
_2 = core::ptr::addr_of_mut!((*_2));
(*_2) = !RET;
_17 = _7 ^ _7;
_19 = true | false;
RET = _6.0 as u16;
_18.1 = _13;
_12 = _17 as f64;
_20 = core::ptr::addr_of!(_13);
(*_2) = RET + RET;
(*_2) = RET;
_20 = core::ptr::addr_of!((*_20));
_13 = _18.1 ^ _3;
(*_20) = _18.1 ^ _18.1;
(*_2) = !RET;
(*_20) = -_18.1;
match _6.0 {
3154969171 => bb18,
_ => bb17
}
}
bb17 = {
Return()
}
bb18 = {
(*_2) = _5 as u16;
_23 = _12 as u128;
Goto(bb19)
}
bb19 = {
_6 = (1285750445_u32,);
_3 = (*_2) as i64;
_26 = (_6.0,);
_10.0 = 3546750275223163544_u64 >> (*_2);
_19 = false;
(*_2) = RET + RET;
_20 = core::ptr::addr_of!((*_20));
_18.2 = _12 as f32;
(*_2) = RET;
_29 = core::ptr::addr_of_mut!(_5);
(*_2) = !RET;
_21 = _12 - _12;
(*_2) = !RET;
(*_20) = _18.1 * _18.1;
(*_2) = RET << (*_29);
(*_29) = _15 + _15;
Call((*_20) = fn10((*_2), (*_2), Move(_2), (*_2)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
_11 = !_17;
_28 = _14 ^ _14;
(*_20) = 182_u8 as i64;
(*_20) = _3 | _3;
(*_29) = _15;
_31 = _28 >> _3;
_11 = '\u{71f64}' as i16;
_18.0.0 = &mut _29;
_21 = -_12;
_2 = core::ptr::addr_of_mut!(_1);
Goto(bb21)
}
bb21 = {
_6.0 = _26.0;
(*_2) = RET - RET;
(*_2) = !RET;
_7 = _31 as i16;
(*_20) = _3 * _3;
(*_2) = RET - RET;
_17 = _7 ^ _11;
_30 = _18.2;
_6.0 = _26.0 << (*_20);
_4 = !_23;
match _26.0 {
0 => bb16,
1 => bb22,
1285750445 => bb24,
_ => bb23
}
}
bb22 = {
Return()
}
bb23 = {
(*_2) = _5 as u16;
_23 = _12 as u128;
Goto(bb19)
}
bb24 = {
_8 = _31 << (*_20);
_22 = &(*_20);
(*_20) = _18.1 + _18.1;
_23 = !_10.1;
(*_20) = -_18.1;
(*_2) = RET ^ RET;
_11 = _7 & _7;
(*_2) = RET >> _3;
(*_20) = 189_u8 as i64;
_1 = _19 as u16;
_18.1 = (*_20);
_10.0 = !9629234806919628742_u64;
_4 = !_23;
(*_2) = RET - RET;
_6 = _26;
(*_20) = _18.1 ^ _3;
(*_2) = !RET;
(*_20) = _18.1 * _18.1;
(*_20) = 65_i8 as i64;
_17 = _30 as i16;
Goto(bb25)
}
bb25 = {
_31 = _8 - _8;
_26.0 = !_6.0;
_4 = _23 | _10.1;
(*_20) = _3;
(*_20) = _18.1 | _3;
(*_20) = _3 >> (*_2);
(*_2) = RET * RET;
RET = (*_2) & (*_2);
_35 = !_28;
_27 = -_30;
_13 = _3 * _3;
_33 = _18.2 - _18.2;
_26 = (_6.0,);
_23 = (*_20) as u128;
(*_20) = _3 - _3;
_6 = _26;
_7 = _11 ^ _17;
(*_2) = _10.1 as u16;
(*_2) = RET;
_4 = _10.1 + _23;
_3 = _1 as i64;
match _6.0 {
0 => bb12,
1 => bb23,
2 => bb10,
1285750445 => bb26,
_ => bb7
}
}
bb26 = {
(*_20) = _3 & _3;
(*_20) = _3 & _18.1;
(*_20) = _3;
_10 = (15840263393129283811_u64, _23);
(*_20) = _3;
_10 = (527594829451306937_u64, _23);
_42 = 92_i8 as i64;
_40 = &_4;
_16 = _30 > _30;
(*_20) = _3;
RET = _1 ^ (*_2);
(*_20) = _19 as i64;
_27 = -_33;
_10.0 = _7 as u64;
_17 = _7 + _7;
_2 = core::ptr::addr_of_mut!((*_2));
_12 = _21 - _21;
_18.1 = (*_20) << (*_40);
Call(_4 = core::intrinsics::bswap(_10.1), ReturnTo(bb27), UnwindUnreachable())
}
bb27 = {
_12 = _21 - _21;
_42 = _18.1 & _3;
_38 = core::ptr::addr_of_mut!(_45);
(*_38) = !_15;
_10.1 = _4;
(*_20) = 221_u8 as i64;
(*_38) = _5 >> _42;
(*_20) = _42 - _3;
(*_2) = RET + RET;
(*_20) = _18.1 | _3;
_41 = [(*_38)];
(*_38) = _5 & _15;
_46 = !_16;
_39 = Adt21::Variant1 { fld0: _16,fld1: _12,fld2: _35,fld3: 22_i8,fld4: _17,fld5: _6.0,fld6: _33,fld7: 15324575690909414502_usize };
_4 = _23 | _10.1;
(*_20) = !_42;
(*_20) = _3;
(*_20) = _18.1 & _18.1;
(*_2) = !RET;
(*_20) = _10.1 as i64;
_31 = _8;
_28 = _11 as isize;
(*_2) = 19_i8 as u16;
_19 = Field::<bool>(Variant(_39, 1), 0) > _16;
(*_2) = (*_20) as u16;
_44 = Field::<bool>(Variant(_39, 1), 0);
(*_2) = RET << (*_20);
match _6.0 {
1285750445 => bb28,
_ => bb16
}
}
bb28 = {
_21 = _12 * Field::<f64>(Variant(_39, 1), 1);
(*_2) = RET - RET;
_53 = [_26.0,_6.0,_6.0,Field::<u32>(Variant(_39, 1), 5),Field::<u32>(Variant(_39, 1), 5),_26.0,Field::<u32>(Variant(_39, 1), 5),Field::<u32>(Variant(_39, 1), 5)];
_50.0 = ((*_20),);
_55.fld0.0 = (*_20);
_39 = Adt21::Variant1 { fld0: _46,fld1: _12,fld2: _8,fld3: 6_i8,fld4: _17,fld5: _6.0,fld6: _27,fld7: 15222343659352363436_usize };
(*_38) = _5 & _5;
(*_38) = -_5;
_18.2 = _10.0 as f32;
(*_2) = 10129014069717754218_usize as u16;
_40 = &_10.1;
_27 = -_18.2;
_55.fld4 = Adt21::Variant1 { fld0: _44,fld1: _21,fld2: _8,fld3: 105_i8,fld4: _11,fld5: _6.0,fld6: _33,fld7: 2_usize };
_28 = !Field::<isize>(Variant(_39, 1), 2);
place!(Field::<usize>(Variant(_55.fld4, 1), 7)) = 1_usize & 7_usize;
(*_38) = !_15;
_30 = _18.2;
Call((*_38) = core::intrinsics::transmute((*_40)), ReturnTo(bb29), UnwindUnreachable())
}
bb29 = {
(*_2) = RET | RET;
_10.1 = _4 << Field::<i16>(Variant(_39, 1), 4);
(*_20) = _18.1 >> (*_38);
_36 = (*_20) & _50.0.0;
_34 = core::ptr::addr_of_mut!(_55);
(*_34).fld4 = Adt21::Variant0 { fld0: _10.1,fld1: 2129279693_i32,fld2: 92_i8 };
place!(Field::<u128>(Variant(_55.fld4, 0), 0)) = _10.1 & _10.1;
(*_20) = _36;
_22 = &(*_34).fld0.0;
(*_34).fld1 = [_26.0,_6.0,Field::<u32>(Variant(_39, 1), 5),_6.0,_6.0,_26.0,Field::<u32>(Variant(_39, 1), 5),_26.0];
place!(Field::<u128>(Variant((*_34).fld4, 0), 0)) = !_4;
place!(Field::<i8>(Variant((*_34).fld4, 0), 2)) = -93_i8;
(*_34).fld4 = Adt21::Variant1 { fld0: _19,fld1: _12,fld2: _31,fld3: 56_i8,fld4: Field::<i16>(Variant(_39, 1), 4),fld5: _26.0,fld6: _27,fld7: 2164707333833299380_usize };
place!(Field::<bool>(Variant((*_34).fld4, 1), 0)) = !_19;
(*_34).fld4 = Adt21::Variant0 { fld0: _10.1,fld1: 83789253_i32,fld2: (-51_i8) };
_30 = _33;
(*_34).fld0.0 = -(*_20);
(*_34).fld4 = Adt21::Variant1 { fld0: Field::<bool>(Variant(_39, 1), 0),fld1: _21,fld2: _31,fld3: 0_i8,fld4: _11,fld5: _26.0,fld6: _30,fld7: 3_usize };
_55.fld1 = [Field::<u32>(Variant((*_34).fld4, 1), 5),Field::<u32>(Variant((*_34).fld4, 1), 5),_6.0,Field::<u32>(Variant((*_34).fld4, 1), 5),Field::<u32>(Variant(_55.fld4, 1), 5),Field::<u32>(Variant((*_34).fld4, 1), 5),Field::<u32>(Variant((*_34).fld4, 1), 5),Field::<u32>(Variant((*_34).fld4, 1), 5)];
place!(Field::<isize>(Variant((*_34).fld4, 1), 2)) = _31 | Field::<isize>(Variant(_39, 1), 2);
place!(Field::<usize>(Variant(_55.fld4, 1), 7)) = 2_usize | 5_usize;
(*_34).fld1 = [Field::<u32>(Variant((*_34).fld4, 1), 5),Field::<u32>(Variant((*_34).fld4, 1), 5),Field::<u32>(Variant((*_34).fld4, 1), 5),Field::<u32>(Variant((*_34).fld4, 1), 5),Field::<u32>(Variant((*_34).fld4, 1), 5),Field::<u32>(Variant((*_34).fld4, 1), 5),Field::<u32>(Variant(_39, 1), 5),Field::<u32>(Variant((*_34).fld4, 1), 5)];
place!(Field::<i8>(Variant((*_34).fld4, 1), 3)) = '\u{70b17}' as i8;
_26 = (Field::<u32>(Variant((*_34).fld4, 1), 5),);
Call(_11 = core::intrinsics::bswap(Field::<i16>(Variant((*_34).fld4, 1), 4)), ReturnTo(bb30), UnwindUnreachable())
}
bb30 = {
(*_2) = RET + RET;
place!(Field::<i16>(Variant(_39, 1), 4)) = Field::<i16>(Variant((*_34).fld4, 1), 4) ^ Field::<i16>(Variant((*_34).fld4, 1), 4);
(*_2) = RET << Field::<isize>(Variant((*_34).fld4, 1), 2);
_58 = &mut (*_34).fld0;
(*_2) = RET;
match Field::<u32>(Variant((*_34).fld4, 1), 5) {
0 => bb12,
1 => bb5,
2 => bb31,
3 => bb32,
1285750445 => bb34,
_ => bb33
}
}
bb31 = {
(*_2) = 51633_u16 >> _3;
RET = (*_2) + (*_2);
(*_2) = !RET;
_7 = (-3544_i16) - (-21536_i16);
_3 = 1095905846059820715_i64 + (-7768459529918151538_i64);
(*_2) = RET;
(*_2) = RET - RET;
_7 = (-9223372036854775808_isize) as i16;
(*_2) = RET;
(*_2) = RET * RET;
_8 = !9223372036854775807_isize;
(*_2) = RET << RET;
(*_2) = RET ^ RET;
match _6.0 {
0 => bb9,
1 => bb10,
622119483 => bb12,
_ => bb11
}
}
bb32 = {
Return()
}
bb33 = {
_12 = _21 - _21;
_42 = _18.1 & _3;
_38 = core::ptr::addr_of_mut!(_45);
(*_38) = !_15;
_10.1 = _4;
(*_20) = 221_u8 as i64;
(*_38) = _5 >> _42;
(*_20) = _42 - _3;
(*_2) = RET + RET;
(*_20) = _18.1 | _3;
_41 = [(*_38)];
(*_38) = _5 & _15;
_46 = !_16;
_39 = Adt21::Variant1 { fld0: _16,fld1: _12,fld2: _35,fld3: 22_i8,fld4: _17,fld5: _6.0,fld6: _33,fld7: 15324575690909414502_usize };
_4 = _23 | _10.1;
(*_20) = !_42;
(*_20) = _3;
(*_20) = _18.1 & _18.1;
(*_2) = !RET;
(*_20) = _10.1 as i64;
_31 = _8;
_28 = _11 as isize;
(*_2) = 19_i8 as u16;
_19 = Field::<bool>(Variant(_39, 1), 0) > _16;
(*_2) = (*_20) as u16;
_44 = Field::<bool>(Variant(_39, 1), 0);
(*_2) = RET << (*_20);
match _6.0 {
1285750445 => bb28,
_ => bb16
}
}
bb34 = {
(*_58).0 = (*_20) & _42;
place!(Field::<i8>(Variant(_39, 1), 3)) = Field::<i8>(Variant((*_34).fld4, 1), 3) & Field::<i8>(Variant((*_34).fld4, 1), 3);
_52 = core::ptr::addr_of!((*_38));
(*_52) = _5;
place!(Field::<isize>(Variant((*_34).fld4, 1), 2)) = '\u{ea181}' as isize;
(*_38) = Field::<u32>(Variant((*_34).fld4, 1), 5) as i128;
_14 = _31 * Field::<isize>(Variant((*_34).fld4, 1), 2);
_50.0.0 = -(*_20);
place!(Field::<u32>(Variant((*_34).fld4, 1), 5)) = Field::<u32>(Variant(_39, 1), 5) >> (*_58).0;
(*_34).fld4 = Adt21::Variant0 { fld0: _10.1,fld1: (-1669265355_i32),fld2: Field::<i8>(Variant(_39, 1), 3) };
(*_34).fld1 = [_26.0,_26.0,Field::<u32>(Variant(_39, 1), 5),Field::<u32>(Variant(_39, 1), 5),_26.0,_6.0,Field::<u32>(Variant(_39, 1), 5),_6.0];
(*_34).fld4 = Adt21::Variant1 { fld0: _46,fld1: _21,fld2: _28,fld3: Field::<i8>(Variant(_39, 1), 3),fld4: Field::<i16>(Variant(_39, 1), 4),fld5: _26.0,fld6: _27,fld7: 5_usize };
place!(Field::<f64>(Variant((*_34).fld4, 1), 1)) = _21 - Field::<f64>(Variant(_39, 1), 1);
(*_34).fld4 = Adt21::Variant0 { fld0: _10.1,fld1: 2136369290_i32,fld2: Field::<i8>(Variant(_39, 1), 3) };
(*_2) = !RET;
place!(Field::<i32>(Variant((*_34).fld4, 0), 1)) = !(-1911066975_i32);
place!(Field::<i8>(Variant((*_34).fld4, 0), 2)) = Field::<i16>(Variant(_39, 1), 4) as i8;
_5 = (*_38);
match _6.0 {
0 => bb6,
1 => bb21,
2 => bb10,
3 => bb32,
1285750445 => bb35,
_ => bb29
}
}
bb35 = {
(*_20) = _3;
_58 = &mut _50.0;
(*_58) = ((*_20),);
_18.1 = _36 >> _10.1;
(*_2) = RET & RET;
_33 = _27;
(*_58).0 = !_18.1;
place!(Field::<i32>(Variant((*_34).fld4, 0), 1)) = 1001807251_i32 | 545930709_i32;
(*_58) = ((*_20),);
place!(Field::<i8>(Variant(_39, 1), 3)) = Field::<i32>(Variant((*_34).fld4, 0), 1) as i8;
_63 = '\u{5a873}';
_10 = (2007856968840866856_u64, Field::<u128>(Variant((*_34).fld4, 0), 0));
(*_20) = _42 - _36;
(*_38) = -_5;
_37 = &mut (*_58);
match _10.0 {
0 => bb1,
1 => bb24,
2 => bb21,
3 => bb5,
4 => bb36,
5 => bb37,
2007856968840866856 => bb39,
_ => bb38
}
}
bb36 = {
(*_2) = RET & RET;
_12 = _8 as f64;
(*_2) = RET + RET;
_3 = !_13;
_2 = core::ptr::addr_of_mut!((*_2));
_14 = -_8;
_11 = !_7;
_7 = _11;
(*_2) = RET * RET;
_13 = _3 | _3;
_2 = core::ptr::addr_of_mut!((*_2));
_10.1 = _4 + _4;
_2 = core::ptr::addr_of_mut!((*_2));
_7 = _11;
_7 = _11 | _11;
_15 = '\u{66fe3}' as i128;
(*_2) = 6_usize as u16;
Goto(bb16)
}
bb37 = {
Return()
}
bb38 = {
(*_2) = 51633_u16 >> _3;
RET = (*_2) + (*_2);
(*_2) = !RET;
_7 = (-3544_i16) - (-21536_i16);
_3 = 1095905846059820715_i64 + (-7768459529918151538_i64);
(*_2) = RET;
(*_2) = RET - RET;
_7 = (-9223372036854775808_isize) as i16;
(*_2) = RET;
(*_2) = RET * RET;
_8 = !9223372036854775807_isize;
(*_2) = RET << RET;
(*_2) = RET ^ RET;
match _6.0 {
0 => bb9,
1 => bb10,
622119483 => bb12,
_ => bb11
}
}
bb39 = {
(*_38) = 8251594703158755651_usize as i128;
(*_34).fld1 = [_6.0,Field::<u32>(Variant(_39, 1), 5),Field::<u32>(Variant(_39, 1), 5),_26.0,_6.0,_6.0,Field::<u32>(Variant(_39, 1), 5),Field::<u32>(Variant(_39, 1), 5)];
(*_37).0 = (*_20) + (*_20);
place!(Field::<i32>(Variant((*_34).fld4, 0), 1)) = Field::<i8>(Variant((*_34).fld4, 0), 2) as i32;
(*_34).fld1 = [_6.0,_26.0,Field::<u32>(Variant(_39, 1), 5),Field::<u32>(Variant(_39, 1), 5),_26.0,_6.0,_26.0,_6.0];
(*_37).0 = (*_20) >> Field::<i8>(Variant((*_34).fld4, 0), 2);
place!(Field::<i32>(Variant((*_34).fld4, 0), 1)) = -(-894359996_i32);
place!(Field::<i32>(Variant((*_34).fld4, 0), 1)) = _11 as i32;
place!(Field::<u128>(Variant((*_34).fld4, 0), 0)) = _10.1 ^ _10.1;
_18.1 = Field::<i32>(Variant((*_34).fld4, 0), 1) as i64;
RET = (*_2);
(*_34).fld4 = Adt21::Variant1 { fld0: _44,fld1: Field::<f64>(Variant(_39, 1), 1),fld2: _31,fld3: Field::<i8>(Variant(_39, 1), 3),fld4: _7,fld5: Field::<u32>(Variant(_39, 1), 5),fld6: _33,fld7: 6_usize };
(*_34).fld4 = Adt21::Variant0 { fld0: _10.1,fld1: (-2106292943_i32),fld2: Field::<i8>(Variant(_39, 1), 3) };
(*_37).0 = 917558156_i32 as i64;
(*_34).fld4 = Adt21::Variant1 { fld0: _44,fld1: _12,fld2: _8,fld3: Field::<i8>(Variant(_39, 1), 3),fld4: _17,fld5: _6.0,fld6: _18.2,fld7: 5_usize };
_68 = (*_38) as f64;
_26.0 = !Field::<u32>(Variant(_39, 1), 5);
place!(Field::<i16>(Variant((*_34).fld4, 1), 4)) = !_17;
place!(Field::<f32>(Variant((*_34).fld4, 1), 6)) = Field::<f32>(Variant(_39, 1), 6) + _33;
(*_34).fld4 = Adt21::Variant0 { fld0: _4,fld1: (-1563762460_i32),fld2: Field::<i8>(Variant(_39, 1), 3) };
(*_37) = ((*_20),);
place!(Field::<u128>(Variant((*_34).fld4, 0), 0)) = Field::<f32>(Variant(_39, 1), 6) as u128;
(*_2) = 116_u8 as u16;
place!(Field::<u128>(Variant((*_34).fld4, 0), 0)) = _4;
Goto(bb40)
}
bb40 = {
Call(_72 = dump_var(Move(_10), Move(_63), Move(_5), Move(_36)), ReturnTo(bb41), UnwindUnreachable())
}
bb41 = {
Call(_72 = dump_var(Move(_46), Move(_26), Move(_17), Move(_35)), ReturnTo(bb42), UnwindUnreachable())
}
bb42 = {
Call(_72 = dump_var(Move(_7), Move(_8), Move(_6), Move(_15)), ReturnTo(bb43), UnwindUnreachable())
}
bb43 = {
Call(_72 = dump_var(Move(_19), Move(_44), _73, _73), ReturnTo(bb44), UnwindUnreachable())
}
bb44 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn10(mut _1: u16,mut _2: u16,mut _3: *mut u16,mut _4: u16) -> i64 {
mir! {
type RET = i64;
let _5: f64;
let _6: f32;
let _7: isize;
let _8: [i8; 3];
let _9: f32;
let _10: &'static i64;
let _11: *mut [u8; 4];
let _12: &'static &'static mut (i64,);
let _13: (&'static char, [u128; 7], *const f32, *const [i32; 5]);
let _14: &'static mut *mut i128;
let _15: i32;
let _16: *const i128;
let _17: *const [i32; 5];
let _18: (i64,);
let _19: *mut &'static *mut isize;
let _20: u64;
let _21: &'static mut char;
let _22: char;
let _23: u16;
let _24: *mut Adt75;
let _25: (Adt22, ((i64,), &'static char), Adt52, *mut [isize; 5]);
let _26: char;
let _27: (&'static mut char,);
let _28: ([i128; 1], [i8; 2], *const Adt22);
let _29: [i8; 2];
let _30: char;
let _31: [i8; 2];
let _32: u8;
let _33: usize;
let _34: ();
let _35: ();
{
RET = -5700667414821233185_i64;
RET = (-9197039210226173048_i64) >> _4;
_2 = _1 << _4;
_6 = 14257455270692423409_usize as f32;
_4 = _6 as u16;
RET = 3553778265240171598_i64 ^ (-2566596810810308968_i64);
_1 = _2 & _2;
_5 = (-124_i8) as f64;
_2 = _1 ^ _1;
_7 = 9223372036854775807_isize << _2;
_3 = core::ptr::addr_of_mut!(_4);
RET = !(-2716318782472160367_i64);
(*_3) = !_2;
_4 = 63417444_u32 as u16;
(*_3) = !_2;
_5 = 238_u8 as f64;
Goto(bb1)
}
bb1 = {
_7 = (-122_isize) * (-51_isize);
_5 = 6532188295316888840_u64 as f64;
_5 = 54801503663087540741094829470775859592_i128 as f64;
(*_3) = _2;
(*_3) = _2 ^ _2;
_1 = RET as u16;
(*_3) = _2;
(*_3) = _2 | _2;
_1 = (*_3) & (*_3);
_7 = 86668484519242299899851238366395635169_i128 as isize;
(*_3) = _1;
(*_3) = _1 - _2;
(*_3) = _1 - _2;
(*_3) = _1;
_3 = core::ptr::addr_of_mut!((*_3));
(*_3) = !_1;
(*_3) = _1 << _2;
(*_3) = _1 * _1;
_8 = [99_i8,(-94_i8),(-98_i8)];
(*_3) = 20406_i16 as u16;
_1 = (*_3) ^ _2;
(*_3) = 346439682_i32 as u16;
_9 = _6 + _6;
(*_3) = _1 ^ _2;
(*_3) = _1;
_1 = (*_3) * (*_3);
Goto(bb2)
}
bb2 = {
(*_3) = 10204_i16 as u16;
(*_3) = _2;
Goto(bb3)
}
bb3 = {
(*_3) = _2 << _1;
(*_3) = 2220317986239301540_usize as u16;
_5 = 2967633856809476738_u64 as f64;
_4 = _1 ^ _2;
(*_3) = _1 & _2;
(*_3) = _1;
(*_3) = _1 << _1;
_6 = -_9;
(*_3) = _1;
Call((*_3) = fn11(Move(_3), _8, _7, _1, _6), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_13.2 = core::ptr::addr_of!(_9);
_13.2 = core::ptr::addr_of!(_9);
_9 = _6;
Call(_1 = core::intrinsics::bswap(_2), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_3 = core::ptr::addr_of_mut!(_2);
(*_3) = _7 as u16;
(*_3) = _1;
_13.2 = core::ptr::addr_of!(_6);
_6 = (-128_i8) as f32;
(*_3) = _1 - _1;
_7 = (-9223372036854775808_isize);
(*_3) = _1;
(*_3) = !_1;
(*_3) = _1 * _1;
_13.2 = core::ptr::addr_of!(_9);
(*_3) = 5_usize as u16;
_9 = 7225261076749016833_u64 as f32;
_13.1 = [23081457558610548848329500017454384990_u128,312008852815943098092749958064417024933_u128,130790523932095778818633151680205574471_u128,191373382022892893134259418920513908456_u128,264907735907246559936428320916165889524_u128,330802181479751091410237121109226286510_u128,290061599677512608286425943277150054840_u128];
_13.2 = core::ptr::addr_of!(_9);
_1 = (*_3) & (*_3);
_7 = 9223372036854775807_isize << _4;
(*_3) = _4 ^ _4;
(*_3) = RET as u16;
_5 = 678301405_u32 as f64;
(*_3) = _4 * _4;
_18.0 = -RET;
Goto(bb6)
}
bb6 = {
(*_3) = _4 + _4;
(*_3) = !_4;
(*_3) = _4 >> _7;
_18 = (RET,);
(*_3) = _4 + _4;
(*_3) = _4 | _4;
(*_3) = _4 - _4;
(*_3) = 1550608968215201422_u64 as u16;
_10 = &RET;
(*_3) = _4 >> _7;
_18.0 = '\u{49276}' as i64;
(*_3) = _4 - _1;
_10 = &_18.0;
Goto(bb7)
}
bb7 = {
_9 = -_6;
(*_3) = _4 - _4;
(*_3) = _4;
_25.0.fld2 = (-47_i8) as u64;
(*_3) = 163_u8 as u16;
_23 = !_4;
_25.0.fld1 = Adt21::Variant1 { fld0: false,fld1: _5,fld2: _7,fld3: (-32_i8),fld4: 1847_i16,fld5: 3981830338_u32,fld6: _9,fld7: 0_usize };
_22 = '\u{91139}';
_1 = _23 ^ (*_3);
_25.1.0 = _18;
_9 = (-113_i8) as f32;
_18.0 = RET << Field::<isize>(Variant(_25.0.fld1, 1), 2);
_18.0 = _25.1.0.0 | RET;
_13.0 = &_22;
(*_3) = _1;
(*_3) = !_23;
_2 = !_1;
place!(Field::<bool>(Variant(_25.0.fld1, 1), 0)) = !false;
(*_3) = !_23;
(*_3) = _1 + _4;
place!(Field::<u32>(Variant(_25.0.fld1, 1), 5)) = 151_u8 as u32;
Goto(bb8)
}
bb8 = {
(*_3) = _1 << RET;
place!(Field::<usize>(Variant(_25.0.fld1, 1), 7)) = !7_usize;
(*_3) = _4 << _7;
place!(Field::<usize>(Variant(_25.0.fld1, 1), 7)) = 1563155536681300153_usize;
_22 = '\u{101829}';
(*_3) = _1 << _7;
(*_3) = !_23;
(*_3) = _23;
(*_3) = _4 & _1;
_22 = '\u{54ac3}';
_13.1 = [321841460555334524796760055861949292019_u128,86667143441208564227202898523471321982_u128,195115266560142494094054848622324643256_u128,319517763942845869290453680780544046097_u128,15596936809738567956288835378588110264_u128,328163982952994675804153530582603313420_u128,204350847912526850004877751102129451237_u128];
_7 = Field::<f32>(Variant(_25.0.fld1, 1), 6) as isize;
Goto(bb9)
}
bb9 = {
(*_3) = _23 - _1;
_25.0.fld1 = Adt21::Variant1 { fld0: false,fld1: _5,fld2: _7,fld3: (-117_i8),fld4: 17462_i16,fld5: 2252936183_u32,fld6: _9,fld7: 1_usize };
(*_3) = _1;
place!(Field::<bool>(Variant(_25.0.fld1, 1), 0)) = (*_3) == _2;
place!(Field::<isize>(Variant(_25.0.fld1, 1), 2)) = -_7;
_9 = Field::<f32>(Variant(_25.0.fld1, 1), 6) - _6;
_25.1 = (_18, Move(_13.0));
_28.2 = core::ptr::addr_of!(_25.0);
_8 = [(-21_i8),101_i8,49_i8];
(*_3) = _23 & _23;
_13.0 = &_22;
(*_3) = _23;
_32 = 147_u8;
RET = _25.1.0.0 >> (*_3);
_28.1 = [97_i8,38_i8];
place!(Field::<f64>(Variant(_25.0.fld1, 1), 1)) = _5 + _5;
_25.1 = (_18, Move(_13.0));
_29 = [(-42_i8),14_i8];
_33 = !3799870178122015841_usize;
_27.0 = &mut _22;
(*_3) = _23;
place!(Field::<usize>(Variant(_25.0.fld1, 1), 7)) = _33 & _33;
_10 = &RET;
_26 = '\u{e19f8}';
(*_3) = _1 ^ _23;
place!(Field::<isize>(Variant(_25.0.fld1, 1), 2)) = _7;
place!(Field::<i8>(Variant(_25.0.fld1, 1), 3)) = -(-31_i8);
Goto(bb10)
}
bb10 = {
Call(_34 = dump_var(Move(_29), Move(_8), Move(_23), Move(_22)), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
Call(_34 = dump_var(Move(_32), Move(_18), _35, _35), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn11(mut _1: *mut u16,mut _2: [i8; 3],mut _3: isize,mut _4: u16,mut _5: f32) -> u16 {
mir! {
type RET = u16;
let _6: *mut Adt75;
let _7: f32;
let _8: *mut u16;
let _9: Adt71;
let _10: *mut isize;
let _11: usize;
let _12: u32;
let _13: Adt52;
let _14: &'static mut &'static mut (i64,);
let _15: i32;
let _16: *mut &'static &'static mut (i64,);
let _17: Adt46;
let _18: *const i64;
let _19: bool;
let _20: &'static mut [i8; 2];
let _21: (u32,);
let _22: (Adt22, &'static u32, (&'static &'static char, i64, i16, i64));
let _23: [char; 8];
let _24: usize;
let _25: ((u32,), &'static char, bool, bool);
let _26: f64;
let _27: (u64, u128);
let _28: u16;
let _29: bool;
let _30: &'static mut [i8; 2];
let _31: *mut [u8; 4];
let _32: i16;
let _33: *const Adt22;
let _34: ([i128; 2], &'static mut *mut i128, i16);
let _35: [i8; 3];
let _36: &'static i64;
let _37: *const (f32, i16, usize, u8);
let _38: f64;
let _39: &'static mut u32;
let _40: isize;
let _41: (&'static &'static char, i64, i16, i64);
let _42: *mut isize;
let _43: i16;
let _44: &'static mut char;
let _45: f64;
let _46: [isize; 5];
let _47: *const i64;
let _48: [char; 8];
let _49: bool;
let _50: *mut Adt75;
let _51: u64;
let _52: (Adt22, ((i64,), &'static char), Adt52, *mut [isize; 5]);
let _53: &'static mut *mut i128;
let _54: *const i64;
let _55: &'static mut *mut i128;
let _56: ();
let _57: ();
{
RET = (-167976711_i32) as u16;
_4 = !RET;
RET = 62_i8 as u16;
_3 = -9223372036854775807_isize;
Goto(bb1)
}
bb1 = {
_7 = -_5;
_4 = 1216397715_u32 as u16;
_2 = [46_i8,88_i8,103_i8];
_8 = core::ptr::addr_of_mut!(_4);
_4 = (-6463186784106212225_i64) as u16;
(*_8) = RET + RET;
(*_8) = RET >> RET;
_4 = RET >> _3;
_5 = 1630324831_i32 as f32;
(*_8) = RET | RET;
(*_8) = RET;
(*_8) = RET;
(*_8) = RET ^ RET;
_3 = 9223372036854775807_isize;
(*_8) = !RET;
(*_8) = !RET;
_5 = _7 - _7;
_1 = Move(_8);
Goto(bb2)
}
bb2 = {
_10 = core::ptr::addr_of_mut!(_3);
(*_10) = (-61_isize) * 68_isize;
RET = _4;
_1 = core::ptr::addr_of_mut!(_4);
(*_10) = -9223372036854775807_isize;
(*_10) = 9223372036854775807_isize << (*_1);
(*_10) = (-9223372036854775808_isize) & 124_isize;
_8 = core::ptr::addr_of_mut!((*_1));
(*_1) = RET * RET;
(*_10) = 94361262248686622976703370693464820629_u128 as isize;
_3 = !127_isize;
(*_10) = 16381311045534071114_usize as isize;
_7 = -_5;
(*_10) = (-9223372036854775808_isize);
(*_1) = RET;
_4 = RET + RET;
(*_1) = false as u16;
(*_1) = RET;
(*_1) = RET;
(*_10) = 9223372036854775807_isize & 9223372036854775807_isize;
_4 = !RET;
Goto(bb3)
}
bb3 = {
(*_1) = RET | RET;
(*_1) = RET | RET;
_11 = 7_usize << (*_1);
(*_1) = RET >> (*_10);
(*_1) = 3052101470_u32 as u16;
_1 = core::ptr::addr_of_mut!((*_1));
_19 = false;
(*_1) = RET ^ RET;
(*_1) = RET << (*_10);
(*_1) = '\u{82290}' as u16;
(*_10) = 9223372036854775807_isize << (*_1);
Goto(bb4)
}
bb4 = {
(*_1) = RET << _11;
(*_1) = RET;
(*_10) = (-9223372036854775808_isize) >> _11;
_12 = '\u{a7ef2}' as u32;
_7 = _5 + _5;
_22.2.2 = (-19232_i16) * (-9554_i16);
(*_1) = !RET;
_5 = _7;
_22.1 = &_12;
(*_10) = !(-9223372036854775808_isize);
_15 = _11 as i32;
_19 = true;
(*_1) = RET << (*_10);
_21 = (_12,);
_22.2.2 = _21.0 as i16;
_22.0.fld1 = Adt21::Variant0 { fld0: 75726694235760799753868456980801672998_u128,fld1: _15,fld2: 107_i8 };
(*_1) = RET | RET;
_22.2.2 = (-21105_i16) - (-841_i16);
Goto(bb5)
}
bb5 = {
(*_1) = RET - RET;
(*_10) = -9223372036854775807_isize;
(*_10) = (-9223372036854775808_isize) * 9223372036854775807_isize;
_8 = core::ptr::addr_of_mut!(RET);
_10 = core::ptr::addr_of_mut!((*_10));
_22.2.1 = (-465554638035912188_i64) >> (*_1);
(*_10) = (*_1) as isize;
(*_8) = 13147625270596317565_u64 as u16;
place!(Field::<i8>(Variant(_22.0.fld1, 0), 2)) = 44_i8 * (-106_i8);
(*_1) = 164786183190320022205556860315116877796_i128 as u16;
_2 = [Field::<i8>(Variant(_22.0.fld1, 0), 2),Field::<i8>(Variant(_22.0.fld1, 0), 2),Field::<i8>(Variant(_22.0.fld1, 0), 2)];
_11 = !10919322807321308419_usize;
(*_10) = '\u{4ab2e}' as isize;
(*_10) = -(-9223372036854775808_isize);
(*_1) = _11 as u16;
(*_8) = (*_1);
_23 = ['\u{d3ca3}','\u{ca0e8}','\u{f06d}','\u{4dd70}','\u{9799b}','\u{eb572}','\u{3e1ab}','\u{5094d}'];
_22.1 = &_21.0;
(*_10) = Field::<i8>(Variant(_22.0.fld1, 0), 2) as isize;
(*_10) = 117_isize;
_27 = (17993388279433114002_u64, 262534864924838987847001559793684747071_u128);
_1 = Move(_8);
Goto(bb6)
}
bb6 = {
(*_10) = (-87_isize) >> Field::<i32>(Variant(_22.0.fld1, 0), 1);
_22.1 = &_12;
_25.2 = Field::<i8>(Variant(_22.0.fld1, 0), 2) != Field::<i8>(Variant(_22.0.fld1, 0), 2);
_7 = (*_10) as f32;
(*_10) = _7 as isize;
Goto(bb7)
}
bb7 = {
_21.0 = _12;
_11 = !2_usize;
(*_10) = 9223372036854775807_isize;
_4 = !RET;
_1 = core::ptr::addr_of_mut!(_4);
_24 = _11 >> (*_10);
Call((*_1) = fn12(Move(_10), (*_10), Move(_1), Move(_22.1), (*_10), _3, (*_10), _5, (*_10)), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_22.1 = &_12;
_24 = _22.2.1 as usize;
_25.3 = !_25.2;
_22.2.3 = 164028339449812143562884223741199285100_i128 as i64;
_27.0 = 15867191123159820292_u64;
_26 = _22.2.1 as f64;
_4 = RET - RET;
_34.2 = '\u{b8240}' as i16;
_27 = (15333214579840401276_u64, 199801392181332067684503784243205787824_u128);
_27.0 = 7556018024133213990_u64 + 11217164012518368328_u64;
_25.2 = Field::<i32>(Variant(_22.0.fld1, 0), 1) != _15;
Goto(bb9)
}
bb9 = {
_8 = core::ptr::addr_of_mut!(_4);
_11 = !_24;
(*_8) = !RET;
_22.2.2 = _34.2 + _34.2;
_22.0.fld1 = Adt21::Variant1 { fld0: _25.2,fld1: _26,fld2: _3,fld3: (-40_i8),fld4: _22.2.2,fld5: _12,fld6: _5,fld7: _11 };
(*_8) = RET << _22.2.1;
_22.0.fld2 = _27.0;
_33 = core::ptr::addr_of!(_22.0);
(*_33).fld3 = 251_u8 as i8;
place!(Field::<u32>(Variant((*_33).fld1, 1), 5)) = _21.0 - _21.0;
place!(Field::<u32>(Variant((*_33).fld1, 1), 5)) = _21.0;
_34.0 = [(-30417365035979122398318096727015686761_i128),(-88599398735950780805843357752404244893_i128)];
(*_33).fld2 = !_27.0;
Goto(bb10)
}
bb10 = {
place!(Field::<isize>(Variant((*_33).fld1, 1), 2)) = !_3;
_2 = [(*_33).fld3,(*_33).fld3,(*_33).fld3];
(*_33).fld0 = Field::<bool>(Variant((*_33).fld1, 1), 0) == Field::<bool>(Variant((*_33).fld1, 1), 0);
_22.0.fld0 = Field::<bool>(Variant((*_33).fld1, 1), 0);
place!(Field::<i16>(Variant((*_33).fld1, 1), 4)) = _34.2 & _22.2.2;
(*_33).fld1 = Adt21::Variant1 { fld0: (*_33).fld0,fld1: _26,fld2: _3,fld3: (*_33).fld3,fld4: _34.2,fld5: _12,fld6: _7,fld7: _11 };
place!(Field::<usize>(Variant((*_33).fld1, 1), 7)) = _11;
place!(Field::<f64>(Variant((*_33).fld1, 1), 1)) = _26 - _26;
place!(Field::<isize>(Variant((*_33).fld1, 1), 2)) = _3 & _3;
place!(Field::<i8>(Variant((*_33).fld1, 1), 3)) = (*_33).fld3;
place!(Field::<i16>(Variant((*_33).fld1, 1), 4)) = !_22.2.2;
place!(Field::<bool>(Variant((*_33).fld1, 1), 0)) = (*_33).fld0;
place!(Field::<isize>(Variant((*_33).fld1, 1), 2)) = !_3;
(*_33).fld0 = _26 < Field::<f64>(Variant((*_33).fld1, 1), 1);
match _3 {
0 => bb11,
1 => bb12,
9223372036854775807 => bb14,
_ => bb13
}
}
bb11 = {
(*_1) = RET | RET;
(*_1) = RET | RET;
_11 = 7_usize << (*_1);
(*_1) = RET >> (*_10);
(*_1) = 3052101470_u32 as u16;
_1 = core::ptr::addr_of_mut!((*_1));
_19 = false;
(*_1) = RET ^ RET;
(*_1) = RET << (*_10);
(*_1) = '\u{82290}' as u16;
(*_10) = 9223372036854775807_isize << (*_1);
Goto(bb4)
}
bb12 = {
(*_10) = (-87_isize) >> Field::<i32>(Variant(_22.0.fld1, 0), 1);
_22.1 = &_12;
_25.2 = Field::<i8>(Variant(_22.0.fld1, 0), 2) != Field::<i8>(Variant(_22.0.fld1, 0), 2);
_7 = (*_10) as f32;
(*_10) = _7 as isize;
Goto(bb7)
}
bb13 = {
_10 = core::ptr::addr_of_mut!(_3);
(*_10) = (-61_isize) * 68_isize;
RET = _4;
_1 = core::ptr::addr_of_mut!(_4);
(*_10) = -9223372036854775807_isize;
(*_10) = 9223372036854775807_isize << (*_1);
(*_10) = (-9223372036854775808_isize) & 124_isize;
_8 = core::ptr::addr_of_mut!((*_1));
(*_1) = RET * RET;
(*_10) = 94361262248686622976703370693464820629_u128 as isize;
_3 = !127_isize;
(*_10) = 16381311045534071114_usize as isize;
_7 = -_5;
(*_10) = (-9223372036854775808_isize);
(*_1) = RET;
_4 = RET + RET;
(*_1) = false as u16;
(*_1) = RET;
(*_1) = RET;
(*_10) = 9223372036854775807_isize & 9223372036854775807_isize;
_4 = !RET;
Goto(bb3)
}
bb14 = {
(*_33).fld3 = !Field::<i8>(Variant((*_33).fld1, 1), 3);
place!(Field::<i16>(Variant((*_33).fld1, 1), 4)) = _22.2.2 & _34.2;
_2 = [Field::<i8>(Variant((*_33).fld1, 1), 3),Field::<i8>(Variant((*_33).fld1, 1), 3),(*_33).fld3];
(*_8) = RET * RET;
(*_33).fld1 = Adt21::Variant0 { fld0: _27.1,fld1: _15,fld2: (*_33).fld3 };
(*_8) = !RET;
(*_33).fld3 = Field::<i8>(Variant((*_33).fld1, 0), 2) >> Field::<i32>(Variant((*_33).fld1, 0), 1);
(*_33).fld2 = _27.0;
(*_33).fld0 = _25.3 | _25.3;
Goto(bb15)
}
bb15 = {
place!(Field::<i32>(Variant((*_33).fld1, 0), 1)) = -_15;
place!(Field::<i8>(Variant((*_33).fld1, 0), 2)) = (*_33).fld3 + _22.0.fld3;
_24 = _11 ^ _11;
(*_33).fld0 = !_25.3;
(*_33).fld1 = Adt21::Variant1 { fld0: (*_33).fld0,fld1: _26,fld2: _3,fld3: (*_33).fld3,fld4: _34.2,fld5: _21.0,fld6: _5,fld7: _11 };
place!(Field::<u32>(Variant((*_33).fld1, 1), 5)) = _12 ^ _21.0;
place!(Field::<i16>(Variant((*_33).fld1, 1), 4)) = _22.2.2 << (*_8);
_25.2 = (*_33).fld0;
place!(Field::<usize>(Variant((*_33).fld1, 1), 7)) = _24 + _24;
place!(Field::<i8>(Variant(_22.0.fld1, 1), 3)) = -(*_33).fld3;
place!(Field::<isize>(Variant((*_33).fld1, 1), 2)) = _3 * _3;
place!(Field::<f32>(Variant((*_33).fld1, 1), 6)) = _7 - _5;
place!(Field::<u32>(Variant((*_33).fld1, 1), 5)) = (*_33).fld2 as u32;
(*_33).fld1 = Adt21::Variant1 { fld0: (*_33).fld0,fld1: _26,fld2: _3,fld3: (*_33).fld3,fld4: _34.2,fld5: _12,fld6: _5,fld7: _24 };
place!(Field::<i16>(Variant((*_33).fld1, 1), 4)) = _22.2.2;
place!(Field::<f64>(Variant((*_33).fld1, 1), 1)) = _26;
place!(Field::<i8>(Variant((*_33).fld1, 1), 3)) = Field::<isize>(Variant((*_33).fld1, 1), 2) as i8;
_36 = &_22.2.1;
_19 = (*_33).fld3 == (*_33).fld3;
place!(Field::<f64>(Variant(_22.0.fld1, 1), 1)) = _26 + _26;
place!(Field::<u32>(Variant((*_33).fld1, 1), 5)) = _21.0 << _24;
_39 = &mut place!(Field::<u32>(Variant((*_33).fld1, 1), 5));
(*_33).fld0 = (*_33).fld2 <= (*_33).fld2;
(*_8) = !RET;
Goto(bb16)
}
bb16 = {
place!(Field::<bool>(Variant((*_33).fld1, 1), 0)) = Field::<usize>(Variant((*_33).fld1, 1), 7) != Field::<usize>(Variant((*_33).fld1, 1), 7);
place!(Field::<i16>(Variant((*_33).fld1, 1), 4)) = _34.2;
(*_33).fld2 = _27.0 >> Field::<usize>(Variant((*_33).fld1, 1), 7);
place!(Field::<bool>(Variant((*_33).fld1, 1), 0)) = !(*_33).fld0;
_49 = (*_33).fld0 & (*_33).fld0;
(*_33).fld2 = !_27.0;
(*_8) = RET >> Field::<usize>(Variant((*_33).fld1, 1), 7);
place!(Field::<i16>(Variant((*_33).fld1, 1), 4)) = 133010999042352632502641748861101884225_i128 as i16;
(*_33).fld2 = _27.0;
place!(Field::<i8>(Variant((*_33).fld1, 1), 3)) = (*_33).fld3 ^ (*_33).fld3;
(*_33).fld0 = Field::<bool>(Variant((*_33).fld1, 1), 0) & Field::<bool>(Variant((*_33).fld1, 1), 0);
(*_33).fld3 = Field::<f64>(Variant((*_33).fld1, 1), 1) as i8;
Goto(bb17)
}
bb17 = {
RET = (*_8) << Field::<i8>(Variant((*_33).fld1, 1), 3);
place!(Field::<i16>(Variant((*_33).fld1, 1), 4)) = _34.2 & _34.2;
_41.3 = _15 as i64;
place!(Field::<i8>(Variant((*_33).fld1, 1), 3)) = -(*_33).fld3;
place!(Field::<f32>(Variant((*_33).fld1, 1), 6)) = _7 - _5;
(*_8) = !RET;
place!(Field::<i16>(Variant((*_33).fld1, 1), 4)) = _34.2 - _34.2;
place!(Field::<f64>(Variant((*_33).fld1, 1), 1)) = _26 + _26;
(*_33).fld3 = !Field::<i8>(Variant((*_33).fld1, 1), 3);
(*_33).fld2 = _27.0 ^ _27.0;
(*_33).fld0 = Field::<bool>(Variant((*_33).fld1, 1), 0) & Field::<bool>(Variant((*_33).fld1, 1), 0);
place!(Field::<bool>(Variant((*_33).fld1, 1), 0)) = !(*_33).fld0;
_52.3 = core::ptr::addr_of_mut!(_46);
_3 = Field::<isize>(Variant((*_33).fld1, 1), 2);
_8 = core::ptr::addr_of_mut!((*_8));
place!(Field::<f64>(Variant((*_33).fld1, 1), 1)) = _26;
_13 = Adt52::Variant0 { fld0: _27,fld1: _34.0,fld2: Field::<usize>(Variant((*_33).fld1, 1), 7),fld3: 6_u8 };
place!(Field::<f32>(Variant((*_33).fld1, 1), 6)) = _5;
(*_8) = RET;
Goto(bb18)
}
bb18 = {
Call(_56 = dump_var(Move(_4), Move(_2), Move(_49), Move(_27)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_56 = dump_var(Move(_24), Move(_15), _57, _57), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn12(mut _1: *mut isize,mut _2: isize,mut _3: *mut u16,mut _4: &'static u32,mut _5: isize,mut _6: isize,mut _7: isize,mut _8: f32,mut _9: isize) -> u16 {
mir! {
type RET = u16;
let _10: f32;
let _11: u64;
let _12: char;
let _13: f32;
let _14: char;
let _15: &'static mut [bool; 2];
let _16: i8;
let _17: char;
let _18: *mut u16;
let _19: [u8; 4];
let _20: (u64, u128);
let _21: (u32,);
let _22: f32;
let _23: ((&'static mut *mut i128,), i64, f32, &'static char);
let _24: ([i128; 2], &'static mut *mut i128, i16);
let _25: i32;
let _26: *mut &'static *mut isize;
let _27: i16;
let _28: (u8, i64, &'static mut [bool; 2]);
let _29: Adt22;
let _30: u32;
let _31: f32;
let _32: (u8, i64, &'static mut [bool; 2]);
let _33: &'static mut (i64,);
let _34: *mut Adt75;
let _35: (Adt22, ((i64,), &'static char), Adt52, *mut [isize; 5]);
let _36: &'static u32;
let _37: isize;
let _38: isize;
let _39: Adt21;
let _40: (f32, i16, usize, u8);
let _41: (&'static u128, (i16, &'static i64, [u32; 8], i16), &'static u32);
let _42: *mut i128;
let _43: (&'static u128, (i16, &'static i64, [u32; 8], i16), &'static u32);
let _44: i16;
let _45: ((&'static mut *mut i128,), i64, f32, &'static char);
let _46: u16;
let _47: &'static mut u32;
let _48: [i16; 6];
let _49: char;
let _50: char;
let _51: ([i128; 1], [i8; 2], *const Adt22);
let _52: &'static mut char;
let _53: &'static i64;
let _54: i32;
let _55: i32;
let _56: isize;
let _57: *mut [u8; 4];
let _58: char;
let _59: *const Adt22;
let _60: [i16; 6];
let _61: (i64,);
let _62: bool;
let _63: &'static &'static char;
let _64: u8;
let _65: f32;
let _66: *const i128;
let _67: Adt75;
let _68: isize;
let _69: ([i128; 2], &'static mut *mut i128, i16);
let _70: [i128; 2];
let _71: usize;
let _72: (u64, u128);
let _73: isize;
let _74: f64;
let _75: u32;
let _76: i64;
let _77: f64;
let _78: &'static mut *mut i128;
let _79: [isize; 5];
let _80: char;
let _81: &'static *mut isize;
let _82: (u8, i64, &'static mut [bool; 2]);
let _83: f64;
let _84: char;
let _85: &'static mut char;
let _86: isize;
let _87: *mut i128;
let _88: u128;
let _89: f32;
let _90: bool;
let _91: f32;
let _92: [i16; 6];
let _93: &'static i64;
let _94: ((&'static mut *mut i128,), (f32, i16, usize, u8), (&'static &'static char, i64, i16, i64), &'static i64);
let _95: Adt73;
let _96: i128;
let _97: bool;
let _98: f64;
let _99: &'static mut [i8; 2];
let _100: &'static mut char;
let _101: u64;
let _102: i8;
let _103: &'static char;
let _104: &'static &'static mut (i64,);
let _105: ();
let _106: ();
{
_3 = core::ptr::addr_of_mut!(RET);
(*_3) = 4344_u16;
(*_3) = !60578_u16;
(*_3) = 46436_u16 << _9;
match _9 {
9223372036854775807 => bb2,
_ => bb1
}
}
bb1 = {
Return()
}
bb2 = {
_8 = 7046583155880351367_u64 as f32;
_8 = (-1745900908_i32) as f32;
(*_3) = 51545_u16 * 4173_u16;
(*_3) = 23696_u16 - 6701_u16;
(*_3) = 28512_u16;
(*_3) = !12400_u16;
(*_3) = 30011637664518879733309371266493680969_i128 as u16;
_9 = -_6;
_2 = !_6;
(*_3) = 30309_i16 as u16;
(*_3) = 12846_u16 + 31067_u16;
(*_3) = 51668_u16;
_1 = core::ptr::addr_of_mut!(_5);
(*_3) = 52707_u16;
_9 = (*_1);
(*_1) = _2 & _6;
(*_1) = _2;
(*_1) = _7 + _9;
(*_3) = !34349_u16;
(*_3) = 57264_u16 - 63559_u16;
(*_1) = !_9;
(*_3) = !12810_u16;
_3 = core::ptr::addr_of_mut!((*_3));
_1 = core::ptr::addr_of_mut!((*_1));
Goto(bb3)
}
bb3 = {
_10 = -_8;
(*_3) = 566_u16 >> (*_1);
(*_1) = _9 >> (*_3);
(*_1) = _6;
(*_1) = _7 - _9;
(*_1) = -_2;
_1 = core::ptr::addr_of_mut!((*_1));
(*_3) = 20544_u16;
(*_1) = 20448_i16 as isize;
match (*_3) {
0 => bb2,
1 => bb4,
2 => bb5,
3 => bb6,
4 => bb7,
20544 => bb9,
_ => bb8
}
}
bb4 = {
_8 = 7046583155880351367_u64 as f32;
_8 = (-1745900908_i32) as f32;
(*_3) = 51545_u16 * 4173_u16;
(*_3) = 23696_u16 - 6701_u16;
(*_3) = 28512_u16;
(*_3) = !12400_u16;
(*_3) = 30011637664518879733309371266493680969_i128 as u16;
_9 = -_6;
_2 = !_6;
(*_3) = 30309_i16 as u16;
(*_3) = 12846_u16 + 31067_u16;
(*_3) = 51668_u16;
_1 = core::ptr::addr_of_mut!(_5);
(*_3) = 52707_u16;
_9 = (*_1);
(*_1) = _2 & _6;
(*_1) = _2;
(*_1) = _7 + _9;
(*_3) = !34349_u16;
(*_3) = 57264_u16 - 63559_u16;
(*_1) = !_9;
(*_3) = !12810_u16;
_3 = core::ptr::addr_of_mut!((*_3));
_1 = core::ptr::addr_of_mut!((*_1));
Goto(bb3)
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
(*_3) = !39971_u16;
(*_3) = 26550317_i32 as u16;
(*_3) = 211_u8 as u16;
(*_1) = !_9;
(*_3) = (-106_i8) as u16;
match _7 {
0 => bb10,
1 => bb11,
2 => bb12,
3 => bb13,
4 => bb14,
5 => bb15,
6 => bb16,
9223372036854775807 => bb18,
_ => bb17
}
}
bb10 = {
Return()
}
bb11 = {
Return()
}
bb12 = {
Return()
}
bb13 = {
Return()
}
bb14 = {
_8 = 7046583155880351367_u64 as f32;
_8 = (-1745900908_i32) as f32;
(*_3) = 51545_u16 * 4173_u16;
(*_3) = 23696_u16 - 6701_u16;
(*_3) = 28512_u16;
(*_3) = !12400_u16;
(*_3) = 30011637664518879733309371266493680969_i128 as u16;
_9 = -_6;
_2 = !_6;
(*_3) = 30309_i16 as u16;
(*_3) = 12846_u16 + 31067_u16;
(*_3) = 51668_u16;
_1 = core::ptr::addr_of_mut!(_5);
(*_3) = 52707_u16;
_9 = (*_1);
(*_1) = _2 & _6;
(*_1) = _2;
(*_1) = _7 + _9;
(*_3) = !34349_u16;
(*_3) = 57264_u16 - 63559_u16;
(*_1) = !_9;
(*_3) = !12810_u16;
_3 = core::ptr::addr_of_mut!((*_3));
_1 = core::ptr::addr_of_mut!((*_1));
Goto(bb3)
}
bb15 = {
_10 = -_8;
(*_3) = 566_u16 >> (*_1);
(*_1) = _9 >> (*_3);
(*_1) = _6;
(*_1) = _7 - _9;
(*_1) = -_2;
_1 = core::ptr::addr_of_mut!((*_1));
(*_3) = 20544_u16;
(*_1) = 20448_i16 as isize;
match (*_3) {
0 => bb2,
1 => bb4,
2 => bb5,
3 => bb6,
4 => bb7,
20544 => bb9,
_ => bb8
}
}
bb16 = {
_8 = 7046583155880351367_u64 as f32;
_8 = (-1745900908_i32) as f32;
(*_3) = 51545_u16 * 4173_u16;
(*_3) = 23696_u16 - 6701_u16;
(*_3) = 28512_u16;
(*_3) = !12400_u16;
(*_3) = 30011637664518879733309371266493680969_i128 as u16;
_9 = -_6;
_2 = !_6;
(*_3) = 30309_i16 as u16;
(*_3) = 12846_u16 + 31067_u16;
(*_3) = 51668_u16;
_1 = core::ptr::addr_of_mut!(_5);
(*_3) = 52707_u16;
_9 = (*_1);
(*_1) = _2 & _6;
(*_1) = _2;
(*_1) = _7 + _9;
(*_3) = !34349_u16;
(*_3) = 57264_u16 - 63559_u16;
(*_1) = !_9;
(*_3) = !12810_u16;
_3 = core::ptr::addr_of_mut!((*_3));
_1 = core::ptr::addr_of_mut!((*_1));
Goto(bb3)
}
bb17 = {
Return()
}
bb18 = {
(*_1) = _6 ^ _6;
(*_3) = !42284_u16;
Call((*_3) = fn13((*_1), (*_1), Move(_1), (*_1)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
_11 = 15261209785768992320_u64 | 1868146311421172813_u64;
(*_3) = 39556_u16;
(*_3) = 20499_u16 & 52853_u16;
_11 = !596972099632145011_u64;
_14 = '\u{803b9}';
_5 = -_6;
(*_3) = !51664_u16;
_8 = _10;
_5 = _7 & _9;
_13 = -_10;
_16 = (-51_i8) & 54_i8;
(*_3) = 47010_u16 | 21063_u16;
(*_3) = 56008_u16 * 34559_u16;
(*_3) = !52451_u16;
(*_3) = 18845_u16 ^ 8708_u16;
_9 = _5 | _2;
RET = 21057_u16;
(*_3) = !59396_u16;
(*_3) = 56803_u16 | 48500_u16;
(*_3) = 1216_u16 * 31523_u16;
(*_3) = 37996_u16;
(*_3) = 11105_u16 >> _16;
(*_3) = 12652_u16 * 238_u16;
(*_3) = 4388_u16 + 60246_u16;
(*_3) = 13852_u16 * 39337_u16;
_6 = _5;
_14 = '\u{ee5b}';
Goto(bb20)
}
bb20 = {
_8 = _13;
(*_3) = 13277_u16 + 38830_u16;
(*_3) = 11955_u16 | 5145_u16;
_1 = core::ptr::addr_of_mut!(_5);
_7 = _6 << (*_1);
_9 = -(*_1);
_11 = 13997426932763473612_u64;
_19 = [233_u8,166_u8,212_u8,57_u8];
_20.1 = 295051498937585257853987518455658272476_u128 * 120021100752353860594452352129236344537_u128;
(*_1) = _6 << (*_3);
(*_1) = 150_u8 as isize;
(*_3) = 1464_u16;
_6 = (*_1) >> (*_3);
_12 = _14;
_11 = 182709389145777301_u64;
(*_3) = 13875_u16;
(*_1) = !_7;
(*_1) = _7 << (*_3);
(*_3) = 4737_u16;
(*_3) = _8 as u16;
_14 = _12;
(*_3) = 23910_u16;
(*_1) = -_9;
_23.2 = _10 + _10;
Goto(bb21)
}
bb21 = {
(*_1) = _9;
(*_3) = 6307_u16;
_1 = core::ptr::addr_of_mut!((*_1));
_22 = _23.2 * _8;
_23.3 = &_12;
_3 = core::ptr::addr_of_mut!((*_3));
(*_3) = 15332_u16 - 31492_u16;
_2 = _5;
(*_1) = _9 - _7;
(*_1) = true as isize;
_18 = core::ptr::addr_of_mut!((*_3));
(*_1) = _2 + _2;
(*_3) = 14385_u16 | 58715_u16;
_29.fld0 = (*_3) == (*_3);
(*_1) = (*_3) as isize;
(*_1) = -_7;
match _11 {
182709389145777301 => bb23,
_ => bb22
}
}
bb22 = {
_11 = 15261209785768992320_u64 | 1868146311421172813_u64;
(*_3) = 39556_u16;
(*_3) = 20499_u16 & 52853_u16;
_11 = !596972099632145011_u64;
_14 = '\u{803b9}';
_5 = -_6;
(*_3) = !51664_u16;
_8 = _10;
_5 = _7 & _9;
_13 = -_10;
_16 = (-51_i8) & 54_i8;
(*_3) = 47010_u16 | 21063_u16;
(*_3) = 56008_u16 * 34559_u16;
(*_3) = !52451_u16;
(*_3) = 18845_u16 ^ 8708_u16;
_9 = _5 | _2;
RET = 21057_u16;
(*_3) = !59396_u16;
(*_3) = 56803_u16 | 48500_u16;
(*_3) = 1216_u16 * 31523_u16;
(*_3) = 37996_u16;
(*_3) = 11105_u16 >> _16;
(*_3) = 12652_u16 * 238_u16;
(*_3) = 4388_u16 + 60246_u16;
(*_3) = 13852_u16 * 39337_u16;
_6 = _5;
_14 = '\u{ee5b}';
Goto(bb20)
}
bb23 = {
(*_1) = _7 - _7;
(*_1) = _7 | _2;
_30 = !3076871435_u32;
_29.fld2 = (-5640141605583924341_i64) as u64;
(*_3) = 3492_u16 >> (*_1);
match _11 {
182709389145777301 => bb25,
_ => bb24
}
}
bb24 = {
_8 = 7046583155880351367_u64 as f32;
_8 = (-1745900908_i32) as f32;
(*_3) = 51545_u16 * 4173_u16;
(*_3) = 23696_u16 - 6701_u16;
(*_3) = 28512_u16;
(*_3) = !12400_u16;
(*_3) = 30011637664518879733309371266493680969_i128 as u16;
_9 = -_6;
_2 = !_6;
(*_3) = 30309_i16 as u16;
(*_3) = 12846_u16 + 31067_u16;
(*_3) = 51668_u16;
_1 = core::ptr::addr_of_mut!(_5);
(*_3) = 52707_u16;
_9 = (*_1);
(*_1) = _2 & _6;
(*_1) = _2;
(*_1) = _7 + _9;
(*_3) = !34349_u16;
(*_3) = 57264_u16 - 63559_u16;
(*_1) = !_9;
(*_3) = !12810_u16;
_3 = core::ptr::addr_of_mut!((*_3));
_1 = core::ptr::addr_of_mut!((*_1));
Goto(bb3)
}
bb25 = {
_35.0.fld2 = (*_1) as u64;
(*_3) = 20882_u16;
_17 = _12;
(*_3) = 24142_u16 + 11947_u16;
(*_3) = 47496_u16;
_29.fld2 = _20.1 as u64;
_21.0 = _16 as u32;
_31 = _23.2 + _22;
_23.2 = -_22;
(*_1) = _17 as isize;
_25 = 678543623_i32 << (*_1);
(*_1) = _7 & _2;
(*_1) = !_6;
_35.1.1 = &_14;
_35.0.fld3 = _16;
_21 = (_30,);
_21 = (_30,);
_32.1 = 3876372745647373124_i64 & (-6999450448304086205_i64);
Goto(bb26)
}
bb26 = {
_35.1.0 = (_32.1,);
Goto(bb27)
}
bb27 = {
_28.0 = 76_u8 ^ 54_u8;
_36 = &_21.0;
(*_1) = _35.0.fld2 as isize;
_29.fld2 = (*_1) as u64;
_12 = _17;
(*_1) = _6 << (*_36);
(*_1) = (*_36) as isize;
(*_1) = !_2;
_33 = &mut _35.1.0;
_20.0 = _29.fld2 >> (*_1);
_10 = _23.2;
_29.fld1 = Adt21::Variant0 { fld0: _20.1,fld1: _25,fld2: _16 };
(*_1) = _9;
(*_3) = (*_33).0 as u16;
_8 = (*_36) as f32;
(*_33).0 = !_32.1;
(*_3) = 18702_u16 * 46234_u16;
Goto(bb28)
}
bb28 = {
(*_33) = (_32.1,);
(*_33).0 = _32.1 - _32.1;
_22 = _31;
_23.1 = (*_33).0 & (*_33).0;
Goto(bb29)
}
bb29 = {
_41.1.2 = [(*_36),(*_36),(*_36),(*_36),_21.0,(*_36),(*_36),(*_36)];
(*_33).0 = _23.1 << (*_3);
place!(Field::<i8>(Variant(_29.fld1, 0), 2)) = -_16;
_29.fld3 = _6 as i8;
(*_33).0 = _23.1;
(*_1) = _9 >> (*_33).0;
_41.1.2 = [(*_36),(*_36),(*_36),(*_36),(*_36),(*_36),(*_36),_30];
_40.2 = (*_33).0 as usize;
_40 = (_22, 9477_i16, 5_usize, _28.0);
_7 = (*_1) * (*_1);
(*_33) = (_23.1,);
(*_1) = _7 >> (*_33).0;
_40.2 = Field::<i8>(Variant(_29.fld1, 0), 2) as usize;
_40 = (_23.2, (-8933_i16), 6_usize, _28.0);
_38 = (*_1) ^ (*_1);
_10 = (*_3) as f32;
_27 = _20.0 as i16;
_43.1.2 = [_21.0,(*_36),(*_36),(*_36),(*_36),(*_36),(*_36),(*_36)];
(*_33) = (_23.1,);
_24.0 = [(-115008551918631065413147714126309715677_i128),(-1554113517300966484562843715772176037_i128)];
(*_3) = (*_33).0 as u16;
(*_33).0 = _23.1;
_45.3 = &_12;
_45.2 = -_22;
_14 = _12;
_25 = Field::<i32>(Variant(_29.fld1, 0), 1) * Field::<i32>(Variant(_29.fld1, 0), 1);
Goto(bb30)
}
bb30 = {
(*_33).0 = !_23.1;
(*_3) = 34398_u16 << _7;
_20 = (_29.fld2, Field::<u128>(Variant(_29.fld1, 0), 0));
(*_3) = 47737_u16 + 21290_u16;
(*_33) = (_23.1,);
(*_3) = 27289_u16 * 15030_u16;
_22 = _31;
(*_33) = (_32.1,);
_24.2 = _27 >> (*_1);
_6 = (*_1);
Goto(bb31)
}
bb31 = {
_22 = -_45.2;
(*_33).0 = _23.1;
(*_33) = (_23.1,);
(*_33).0 = -_23.1;
_48 = [_24.2,_24.2,_24.2,_27,_24.2,_24.2];
_43.1.3 = _12 as i16;
(*_33).0 = _23.1 * _23.1;
_32.1 = (*_33).0 << (*_3);
_13 = _40.0;
_40.3 = !_28.0;
place!(Field::<u128>(Variant(_29.fld1, 0), 0)) = _20.1 & _20.1;
_20 = (_29.fld2, Field::<u128>(Variant(_29.fld1, 0), 0));
_44 = _24.2;
place!(Field::<u128>(Variant(_29.fld1, 0), 0)) = (*_36) as u128;
_32.1 = (*_33).0 << (*_1);
(*_33).0 = _23.1 << _6;
(*_3) = 58195_u16 * 26927_u16;
_12 = _17;
(*_33) = (_32.1,);
_24.0 = [165193433363545640887632367092014758436_i128,81429915548596592497999903601574109533_i128];
Call(_45.1 = core::intrinsics::transmute((*_1)), ReturnTo(bb32), UnwindUnreachable())
}
bb32 = {
_51.2 = core::ptr::addr_of!(_29);
_14 = _12;
_43.1.1 = &(*_33).0;
_40 = (_45.2, _44, 10490626232483885732_usize, _28.0);
(*_33).0 = _32.1;
(*_3) = 39606_u16 & 20187_u16;
(*_1) = _38;
_43.1.3 = _40.1 - _40.1;
_31 = _40.0 + _13;
(*_33) = (_32.1,);
(*_33) = (_32.1,);
(*_33).0 = _22 as i64;
_10 = _31 + _23.2;
(*_3) = 62555_u16 + 57937_u16;
_51.1 = [_16,_16];
(*_33).0 = -_32.1;
_29.fld2 = !_20.0;
(*_33).0 = _29.fld2 as i64;
_52 = &mut _12;
Goto(bb33)
}
bb33 = {
_45.3 = Move(_23.3);
(*_1) = !_6;
(*_52) = _14;
_30 = _28.0 as u32;
(*_52) = _14;
(*_33) = (_32.1,);
_43.1.0 = !_43.1.3;
_51.0 = [544246234137059632202135746171976526_i128];
_28.1 = _14 as i64;
(*_3) = !39266_u16;
(*_3) = 25055_u16;
_27 = !_43.1.0;
_2 = (*_1);
_23.1 = _24.2 as i64;
(*_33).0 = _23.1;
_51.1 = [_16,_16];
_29.fld3 = _16 - Field::<i8>(Variant(_29.fld1, 0), 2);
_29.fld0 = (*_1) == _38;
_21 = (_30,);
(*_52) = _14;
match _40.2 {
0 => bb19,
1 => bb8,
2 => bb12,
3 => bb25,
4 => bb9,
5 => bb34,
10490626232483885732 => bb36,
_ => bb35
}
}
bb34 = {
Return()
}
bb35 = {
Return()
}
bb36 = {
(*_3) = 44271_u16;
(*_1) = !_38;
_41.1.3 = _43.1.3 >> _45.1;
_45.1 = (*_33).0 << (*_1);
(*_52) = _17;
_29.fld3 = _16 - Field::<i8>(Variant(_29.fld1, 0), 2);
_40.0 = _22;
_55 = Field::<i32>(Variant(_29.fld1, 0), 1) ^ Field::<i32>(Variant(_29.fld1, 0), 1);
_43.2 = &_30;
(*_33).0 = _32.1;
(*_52) = _14;
Goto(bb37)
}
bb37 = {
_23.2 = _10 + _31;
(*_33).0 = _23.1 + _45.1;
_4 = Move(_43.2);
_41.1.2 = [_21.0,_21.0,_21.0,_21.0,_21.0,_21.0,_21.0,_30];
(*_33) = (_23.1,);
(*_1) = -_38;
_3 = core::ptr::addr_of_mut!(RET);
(*_33) = (_45.1,);
_40.2 = 6_usize << (*_33).0;
_50 = (*_52);
(*_1) = (*_3) as isize;
_46 = (*_3) * (*_3);
match (*_3) {
44271 => bb38,
_ => bb10
}
}
bb38 = {
(*_33) = (_23.1,);
_37 = _6 * _38;
_9 = _7 ^ _7;
(*_1) = _7 - _37;
(*_3) = (*_52) as u16;
(*_33).0 = _45.1 & _23.1;
_24.0 = [(-5753492464648999335686140843048774029_i128),106830535920906647477887416478608347173_i128];
(*_33).0 = !_32.1;
_61 = ((*_33).0,);
(*_1) = _37 + _38;
_51.2 = core::ptr::addr_of!(_29);
_55 = _25;
(*_33).0 = _45.1;
_41.1.3 = -_43.1.0;
_43.1.0 = !_27;
(*_52) = _17;
(*_3) = !_46;
_31 = _22;
(*_33).0 = _45.1 + _32.1;
(*_33).0 = _45.1;
_57 = core::ptr::addr_of_mut!(_19);
(*_33).0 = -_45.1;
_47 = &mut _21.0;
_62 = !_29.fld0;
_41.2 = &(*_47);
(*_1) = _37;
match _11 {
0 => bb21,
182709389145777301 => bb39,
_ => bb35
}
}
bb39 = {
_54 = _25 * Field::<i32>(Variant(_29.fld1, 0), 1);
_1 = core::ptr::addr_of_mut!((*_1));
_65 = -_10;
(*_1) = _9;
_56 = _9 ^ (*_1);
_41.1.1 = &(*_33).0;
_43.0 = &place!(Field::<u128>(Variant(_29.fld1, 0), 0));
(*_1) = _56 * _56;
(*_1) = _25 as isize;
_43.1.1 = Move(_41.1.1);
_40.1 = (*_33).0 as i16;
(*_33) = (_23.1,);
_69.2 = _40.1 >> _41.1.3;
(*_52) = _14;
(*_52) = _14;
_20 = (_29.fld2, Field::<u128>(Variant(_29.fld1, 0), 0));
(*_3) = !_46;
(*_1) = _9 * _56;
_11 = _20.0 + _29.fld2;
(*_52) = _50;
_2 = (*_1);
(*_52) = _17;
(*_52) = _17;
(*_33) = (_32.1,);
_1 = core::ptr::addr_of_mut!(_38);
(*_33) = (_61.0,);
_37 = -(*_1);
(*_57) = [_40.3,_28.0,_40.3,_28.0];
Goto(bb40)
}
bb40 = {
(*_57) = [_40.3,_28.0,_40.3,_40.3];
_59 = core::ptr::addr_of!(_29);
(*_59).fld0 = (*_33).0 >= (*_33).0;
(*_52) = _14;
_23.3 = &(*_52);
(*_1) = _9 ^ _5;
_69.2 = _44 * _40.1;
place!(Field::<i8>(Variant((*_59).fld1, 0), 2)) = (*_59).fld3 >> (*_1);
place!(Field::<i32>(Variant((*_59).fld1, 0), 1)) = _25;
(*_3) = _46 & _46;
(*_3) = !_46;
place!(Field::<i32>(Variant((*_59).fld1, 0), 1)) = _25;
_68 = -(*_1);
(*_59).fld3 = Field::<i8>(Variant((*_59).fld1, 0), 2) >> (*_1);
(*_33) = (_32.1,);
(*_3) = _46;
_23.3 = &_50;
Goto(bb41)
}
bb41 = {
_40.3 = (*_52) as u8;
(*_59).fld3 = Field::<i8>(Variant((*_59).fld1, 0), 2) | Field::<i8>(Variant((*_59).fld1, 0), 2);
place!(Field::<i32>(Variant((*_59).fld1, 0), 1)) = _54;
(*_52) = _50;
(*_33) = (_61.0,);
(*_1) = _45.1 as isize;
(*_33).0 = 53778107741444629714937244306997087058_i128 as i64;
(*_33).0 = _32.1 << Field::<i8>(Variant((*_59).fld1, 0), 2);
_32.0 = !_40.3;
_40.1 = _43.1.3 + _24.2;
(*_52) = _14;
(*_3) = _54 as u16;
(*_33) = (_45.1,);
(*_3) = _46 * _46;
(*_59).fld0 = _62 ^ _62;
(*_47) = _30 >> Field::<i8>(Variant((*_59).fld1, 0), 2);
(*_52) = _50;
(*_59).fld3 = !Field::<i8>(Variant((*_59).fld1, 0), 2);
(*_52) = _17;
_41.2 = &_30;
Goto(bb42)
}
bb42 = {
_47 = &mut _30;
_49 = (*_52);
_31 = _10 + _65;
(*_33).0 = _23.1 | _45.1;
(*_59).fld0 = _62;
(*_33) = _61;
(*_59).fld0 = (*_59).fld3 >= (*_59).fld3;
_53 = &_45.1;
_28.1 = (*_53);
(*_3) = Field::<i32>(Variant((*_59).fld1, 0), 1) as u16;
(*_1) = -_2;
_28.1 = (*_33).0 >> (*_1);
_74 = (*_3) as f64;
(*_59).fld3 = Field::<i8>(Variant((*_59).fld1, 0), 2);
(*_3) = _56 as u16;
(*_3) = _46 | _46;
(*_59).fld0 = _28.1 != (*_53);
place!(Field::<i32>(Variant((*_59).fld1, 0), 1)) = _55;
(*_59).fld3 = Field::<i8>(Variant((*_59).fld1, 0), 2);
_3 = core::ptr::addr_of_mut!((*_3));
(*_3) = !_46;
Goto(bb43)
}
bb43 = {
_29.fld3 = (*_52) as i8;
(*_59).fld3 = !Field::<i8>(Variant((*_59).fld1, 0), 2);
place!(Field::<i8>(Variant((*_59).fld1, 0), 2)) = (*_59).fld3 * (*_59).fld3;
_6 = _32.0 as isize;
_41.1.3 = Field::<i8>(Variant((*_59).fld1, 0), 2) as i16;
_29.fld3 = Field::<i8>(Variant((*_59).fld1, 0), 2) - Field::<i8>(Variant((*_59).fld1, 0), 2);
(*_52) = _14;
(*_59).fld2 = !_11;
_56 = (*_53) as isize;
_72.0 = (*_59).fld2 << (*_1);
_82.1 = (*_33).0;
_65 = _22 + _31;
(*_57) = [_28.0,_40.3,_28.0,_28.0];
(*_59).fld3 = Field::<i8>(Variant((*_59).fld1, 0), 2) & Field::<i8>(Variant((*_59).fld1, 0), 2);
(*_59).fld0 = _62 ^ _62;
_29.fld0 = _62;
(*_57) = [_28.0,_28.0,_32.0,_40.3];
_60 = _48;
(*_59).fld3 = Field::<i8>(Variant((*_59).fld1, 0), 2);
(*_59).fld0 = _62;
_29.fld2 = _72.0 * _72.0;
_70 = [75439555432322809657623462533815615022_i128,137666960702675080226641084076095885892_i128];
(*_3) = _46 | _46;
(*_59).fld0 = (*_59).fld2 <= (*_59).fld2;
place!(Field::<i8>(Variant((*_59).fld1, 0), 2)) = (*_52) as i8;
(*_57) = [_28.0,_28.0,_28.0,_28.0];
Goto(bb44)
}
bb44 = {
(*_1) = _56;
_48 = _60;
place!(Field::<i8>(Variant((*_59).fld1, 0), 2)) = (*_59).fld3;
_24.0 = [(-17219508931928426251303733340625612768_i128),72854809567485145472487688698591451107_i128];
_77 = -_74;
(*_57) = [_40.3,_32.0,_40.3,_40.3];
_81 = &_1;
(*_1) = _5 << (*_53);
_14 = (*_52);
Goto(bb45)
}
bb45 = {
place!(Field::<i8>(Variant((*_59).fld1, 0), 2)) = _40.2 as i8;
(*_33).0 = (*_53) | (*_53);
place!(Field::<i8>(Variant((*_59).fld1, 0), 2)) = (*_59).fld3 ^ (*_59).fld3;
_41.0 = &place!(Field::<u128>(Variant(_29.fld1, 0), 0));
_72 = ((*_59).fld2, Field::<u128>(Variant((*_59).fld1, 0), 0));
_28.0 = _32.0 * _40.3;
(*_52) = _14;
_45.2 = _40.0 + _31;
_58 = (*_52);
(*_59).fld3 = Field::<i8>(Variant(_29.fld1, 0), 2) * Field::<i8>(Variant((*_59).fld1, 0), 2);
place!(Field::<i8>(Variant((*_59).fld1, 0), 2)) = (*_59).fld3 - _29.fld3;
_84 = (*_52);
_43.1.0 = -_40.1;
_8 = _65 * _10;
_71 = _40.2;
(*_3) = _46 - _46;
(*_59).fld3 = Field::<i8>(Variant((*_59).fld1, 0), 2);
place!(Field::<i32>(Variant((*_59).fld1, 0), 1)) = _55 ^ _54;
(*_59).fld0 = !_62;
(*_57) = [_28.0,_28.0,_28.0,_28.0];
(*_57) = [_32.0,_28.0,_28.0,_40.3];
_26 = core::ptr::addr_of_mut!(_81);
(*_52) = _84;
(*_33) = ((*_53),);
_1 = core::ptr::addr_of_mut!(_38);
(*_33) = _61;
_95.fld4 = Adt21::Variant0 { fld0: Field::<u128>(Variant((*_59).fld1, 0), 0),fld1: Field::<i32>(Variant((*_59).fld1, 0), 1),fld2: Field::<i8>(Variant((*_59).fld1, 0), 2) };
Goto(bb46)
}
bb46 = {
(*_52) = _84;
_68 = (*_1);
place!(Field::<i8>(Variant((*_59).fld1, 0), 2)) = !(*_59).fld3;
_43.0 = &place!(Field::<u128>(Variant(_95.fld4, 0), 0));
(*_33) = _61;
_28.1 = (*_33).0;
(*_1) = _74 as isize;
_66 = core::ptr::addr_of!(_96);
place!(Field::<i32>(Variant((*_59).fld1, 0), 1)) = _40.1 as i32;
place!(Field::<i32>(Variant((*_59).fld1, 0), 1)) = (*_53) as i32;
_23.1 = (*_53);
(*_33) = ((*_53),);
(*_59).fld3 = Field::<u128>(Variant((*_59).fld1, 0), 0) as i8;
_43.1 = (_41.1.3, Move(_53), _41.1.2, _41.1.3);
(*_1) = _2;
(*_66) = -119497463931374651645032670171146913104_i128;
(*_1) = _56;
(*_59).fld2 = _72.0 | _72.0;
Goto(bb47)
}
bb47 = {
(*_59).fld2 = _72.0 ^ _72.0;
_97 = (*_59).fld0 ^ (*_59).fld0;
_94.1.3 = _28.0 ^ _28.0;
place!(Field::<i8>(Variant((*_59).fld1, 0), 2)) = Field::<i8>(Variant(_95.fld4, 0), 2) & Field::<i8>(Variant(_95.fld4, 0), 2);
(*_59).fld2 = !_72.0;
_50 = (*_52);
place!(Field::<i8>(Variant((*_59).fld1, 0), 2)) = (*_59).fld3 & Field::<i8>(Variant(_95.fld4, 0), 2);
_19 = [_94.1.3,_40.3,_94.1.3,_94.1.3];
_50 = _49;
Goto(bb48)
}
bb48 = {
(*_3) = _46 & _46;
_19 = [_28.0,_94.1.3,_28.0,_94.1.3];
(*_33).0 = Field::<i8>(Variant((*_59).fld1, 0), 2) as i64;
(*_33) = (_61.0,);
(*_33) = (_23.1,);
(*_3) = _46 >> (*_33).0;
(*_33) = (_45.1,);
(*_59).fld2 = _74 as u64;
(*_57) = [_94.1.3,_32.0,_28.0,_32.0];
_43.0 = &place!(Field::<u128>(Variant((*_59).fld1, 0), 0));
(*_33) = (_23.1,);
(*_33) = (_45.1,);
(*_1) = Field::<i8>(Variant((*_59).fld1, 0), 2) as isize;
_76 = (*_33).0;
(*_66) = (-142553715719213947894252384063546562833_i128) & (-108795421618521352879044034553239714810_i128);
place!(Field::<i8>(Variant((*_59).fld1, 0), 2)) = Field::<i8>(Variant(_95.fld4, 0), 2) - (*_59).fld3;
(*_33) = (_28.1,);
_18 = core::ptr::addr_of_mut!((*_3));
Goto(bb49)
}
bb49 = {
Call(_105 = dump_var(Move(_68), Move(_30), Move(_12), Move(_16)), ReturnTo(bb50), UnwindUnreachable())
}
bb50 = {
Call(_105 = dump_var(Move(_61), Move(_27), Move(_25), Move(_38)), ReturnTo(bb51), UnwindUnreachable())
}
bb51 = {
Call(_105 = dump_var(Move(_96), Move(_48), Move(_71), Move(_60)), ReturnTo(bb52), UnwindUnreachable())
}
bb52 = {
Call(_105 = dump_var(Move(_58), Move(_49), Move(_17), Move(_21)), ReturnTo(bb53), UnwindUnreachable())
}
bb53 = {
Call(_105 = dump_var(Move(_14), Move(_19), Move(_70), _106), ReturnTo(bb54), UnwindUnreachable())
}
bb54 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn13(mut _1: isize,mut _2: isize,mut _3: *mut isize,mut _4: isize) -> u16 {
mir! {
type RET = u16;
let _5: isize;
let _6: u64;
let _7: char;
let _8: isize;
let _9: [i8; 2];
let _10: [i8; 3];
let _11: [u8; 4];
let _12: *mut i128;
let _13: usize;
let _14: &'static mut char;
let _15: *const i64;
let _16: &'static mut char;
let _17: [i128; 2];
let _18: u8;
let _19: [u128; 7];
let _20: ([i128; 1], [i8; 2], *const Adt22);
let _21: *const f32;
let _22: (u32,);
let _23: u8;
let _24: f64;
let _25: i128;
let _26: &'static *mut isize;
let _27: [i128; 2];
let _28: *const [i32; 5];
let _29: *mut Adt73;
let _30: *mut &'static &'static mut (i64,);
let _31: char;
let _32: f64;
let _33: Adt22;
let _34: Adt59;
let _35: (&'static mut char,);
let _36: &'static &'static char;
let _37: *mut u16;
let _38: isize;
let _39: usize;
let _40: i128;
let _41: Adt71;
let _42: i16;
let _43: &'static mut [i8; 2];
let _44: &'static mut char;
let _45: i128;
let _46: i128;
let _47: i64;
let _48: Adt52;
let _49: f32;
let _50: char;
let _51: *const [i32; 5];
let _52: Adt59;
let _53: *mut u16;
let _54: u8;
let _55: u16;
let _56: i8;
let _57: f32;
let _58: i128;
let _59: &'static u128;
let _60: &'static *mut isize;
let _61: Adt75;
let _62: ([i128; 1], [i8; 2], *const Adt22);
let _63: usize;
let _64: Adt75;
let _65: i128;
let _66: isize;
let _67: u16;
let _68: [i8; 3];
let _69: &'static &'static char;
let _70: ();
let _71: ();
{
_4 = !_2;
RET = 31705_u16;
_3 = core::ptr::addr_of_mut!(_5);
match RET {
31705 => bb2,
_ => bb1
}
}
bb1 = {
Return()
}
bb2 = {
(*_3) = _1;
(*_3) = _2 | _2;
(*_3) = _1 & _1;
(*_3) = _1 >> _2;
(*_3) = -_1;
(*_3) = _1 << _1;
_4 = (*_3) - (*_3);
(*_3) = false as isize;
(*_3) = _4 + _4;
(*_3) = _4 - _2;
(*_3) = _4;
(*_3) = _4;
(*_3) = 3372073094_u32 as isize;
(*_3) = _4 << _4;
(*_3) = (-88_i8) as isize;
_3 = core::ptr::addr_of_mut!((*_3));
_1 = _5 & _4;
(*_3) = _4 + _2;
(*_3) = _1;
(*_3) = _1 * _4;
_6 = !13160132228019498831_u64;
(*_3) = 73222566284233521041028716855306799595_i128 as isize;
(*_3) = _6 as isize;
_2 = !_4;
(*_3) = _4 ^ _4;
(*_3) = -_1;
Call(_1 = core::intrinsics::bswap((*_3)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
(*_3) = -_2;
(*_3) = _1;
_10 = [(-69_i8),94_i8,14_i8];
(*_3) = _1 & _4;
(*_3) = _1 >> _2;
(*_3) = _4;
_8 = (*_3);
(*_3) = 6799080507687588217_i64 as isize;
(*_3) = !_2;
(*_3) = _2 - _1;
RET = 38398_u16 ^ 2103_u16;
(*_3) = _2;
(*_3) = _4;
_7 = '\u{f3126}';
(*_3) = _8 - _1;
_9 = [87_i8,96_i8];
_11 = [227_u8,56_u8,16_u8,76_u8];
(*_3) = _8 + _1;
(*_3) = _4;
(*_3) = 6_usize as isize;
(*_3) = _4 & _2;
(*_3) = _2 | _8;
_8 = (*_3) - (*_3);
(*_3) = _8 << _4;
Goto(bb4)
}
bb4 = {
_3 = core::ptr::addr_of_mut!((*_3));
(*_3) = _8 << _8;
(*_3) = _4 >> _8;
(*_3) = _4 | _8;
_9 = [(-83_i8),(-90_i8)];
_2 = 111_i8 as isize;
(*_3) = -_1;
(*_3) = 299923242962855035427038513919565966984_u128 as isize;
(*_3) = _1 + _1;
(*_3) = !_8;
_13 = 15763501494715851549_usize;
_9 = [(-48_i8),(-74_i8)];
Goto(bb5)
}
bb5 = {
(*_3) = _1;
(*_3) = _8 | _4;
RET = 9329_u16 * 55885_u16;
(*_3) = _8 + _8;
_8 = (*_3) + (*_3);
_16 = &mut _7;
Goto(bb6)
}
bb6 = {
(*_3) = _8 | _8;
_1 = (*_3);
_1 = (*_3) - (*_3);
(*_3) = _8;
_10 = [28_i8,(-90_i8),(-68_i8)];
(*_3) = _8;
(*_3) = _8 | _8;
_13 = 1349084868_i32 as usize;
_3 = core::ptr::addr_of_mut!((*_3));
_6 = 9702737435227966917_u64;
Goto(bb7)
}
bb7 = {
_5 = _1 * _1;
(*_16) = '\u{6ef24}';
(*_3) = _1;
(*_16) = '\u{e99bc}';
_19 = [312127790714559200315352004807021184067_u128,119912504771500531375515640773533879035_u128,131999922420550901136237313452879147010_u128,187616137845108713548695010042064515554_u128,100998397186698620136696862495907933575_u128,256061200267819424718184431140080894799_u128,63750044382655138298174158509242612213_u128];
(*_16) = '\u{1008d8}';
(*_16) = '\u{cbc21}';
(*_16) = '\u{a49e2}';
(*_3) = _8;
_10 = [122_i8,(-115_i8),(-54_i8)];
_14 = &mut (*_16);
(*_14) = '\u{af143}';
_24 = _13 as f64;
(*_14) = '\u{eb973}';
_18 = 132_u8 + 3_u8;
_12 = core::ptr::addr_of_mut!(_25);
_18 = !100_u8;
(*_14) = '\u{26967}';
(*_3) = -_1;
_16 = Move(_14);
(*_3) = -_1;
Goto(bb8)
}
bb8 = {
_17 = [72136372185844090996963558595103784690_i128,147579576222659357944790193520520657604_i128];
(*_12) = 81304321647498565112652736296233530916_i128;
(*_3) = _8 << _8;
_8 = (*_3) ^ (*_3);
(*_12) = (-102469144657219834765536935219749808875_i128) >> (*_3);
(*_3) = _1 << (*_12);
(*_12) = (*_3) as i128;
(*_12) = (-82148275014138187301500975506666849777_i128) >> (*_3);
(*_12) = (-342139272273703131753352705770587903_i128) & (-40226364389911384635404404163766324900_i128);
(*_12) = (-153207261529216777409891377565846967091_i128) & (-150789805581316998352759216570077513119_i128);
(*_12) = (-67231890362968921181668498844669709011_i128) | 63583707917489647135102433241039944180_i128;
(*_3) = _1;
(*_3) = _13 as isize;
(*_12) = (-11607849584066158752456936745701917738_i128);
_23 = !_18;
(*_12) = (-166909301604642259564155320234113099674_i128) ^ (-80719125769354290620102363743518433099_i128);
_2 = _1;
_10 = [70_i8,(-47_i8),6_i8];
match _6 {
0 => bb4,
1 => bb2,
2 => bb3,
3 => bb9,
4 => bb10,
9702737435227966917 => bb12,
_ => bb11
}
}
bb9 = {
Return()
}
bb10 = {
_3 = core::ptr::addr_of_mut!((*_3));
(*_3) = _8 << _8;
(*_3) = _4 >> _8;
(*_3) = _4 | _8;
_9 = [(-83_i8),(-90_i8)];
_2 = 111_i8 as isize;
(*_3) = -_1;
(*_3) = 299923242962855035427038513919565966984_u128 as isize;
(*_3) = _1 + _1;
(*_3) = !_8;
_13 = 15763501494715851549_usize;
_9 = [(-48_i8),(-74_i8)];
Goto(bb5)
}
bb11 = {
(*_3) = -_2;
(*_3) = _1;
_10 = [(-69_i8),94_i8,14_i8];
(*_3) = _1 & _4;
(*_3) = _1 >> _2;
(*_3) = _4;
_8 = (*_3);
(*_3) = 6799080507687588217_i64 as isize;
(*_3) = !_2;
(*_3) = _2 - _1;
RET = 38398_u16 ^ 2103_u16;
(*_3) = _2;
(*_3) = _4;
_7 = '\u{f3126}';
(*_3) = _8 - _1;
_9 = [87_i8,96_i8];
_11 = [227_u8,56_u8,16_u8,76_u8];
(*_3) = _8 + _1;
(*_3) = _4;
(*_3) = 6_usize as isize;
(*_3) = _4 & _2;
(*_3) = _2 | _8;
_8 = (*_3) - (*_3);
(*_3) = _8 << _4;
Goto(bb4)
}
bb12 = {
(*_12) = !(-75440029395989416886354565786842586069_i128);
_23 = _18 * _18;
(*_3) = _1;
_5 = _2 & _2;
_2 = 24358_i16 as isize;
(*_3) = (*_12) as isize;
(*_12) = 107666958723058557745005091016271196599_i128 * (-104102052316893470129199605235194009423_i128);
_22 = (175082776_u32,);
(*_3) = _8 << _8;
(*_12) = true as i128;
(*_3) = _8 - _1;
_20.1 = [13_i8,(-103_i8)];
_1 = _8 + (*_3);
_6 = 24_i8 as u64;
_13 = (-30355_i16) as usize;
_27 = [(*_12),(*_12)];
(*_12) = _23 as i128;
_26 = &_3;
(*_3) = _1;
Goto(bb13)
}
bb13 = {
(*_12) = (-69968518877879735481290561594121809302_i128) >> (*_3);
(*_3) = _22.0 as isize;
(*_12) = 64305001171204488029745288136287281305_i128;
(*_12) = !(-51873557611937071958418595114724105275_i128);
(*_12) = 62774307469998452725111575641183299820_i128 ^ (-53294279851577665261132058089755243076_i128);
(*_3) = _8 << _1;
(*_3) = _8;
_31 = '\u{3e1c8}';
(*_12) = (-51574655809522885756953496656969537946_i128) - (-1036639889723349120078278127773121288_i128);
(*_3) = _8 ^ _1;
(*_3) = _8 + _1;
_33.fld1 = Adt21::Variant0 { fld0: 119437941891258301957425914620384672163_u128,fld1: (-208925859_i32),fld2: (-56_i8) };
_33.fld2 = _6 ^ _6;
Goto(bb14)
}
bb14 = {
_17 = [(*_12),(*_12)];
place!(Field::<i8>(Variant(_33.fld1, 0), 2)) = (-97_i8);
_8 = (*_3) << (*_3);
(*_12) = (-30156757932462769075389354317761943477_i128);
match (*_12) {
0 => bb8,
1 => bb15,
310125608988475694387985253114006267979 => bb17,
_ => bb16
}
}
bb15 = {
(*_3) = _8 | _8;
_1 = (*_3);
_1 = (*_3) - (*_3);
(*_3) = _8;
_10 = [28_i8,(-90_i8),(-68_i8)];
(*_3) = _8;
(*_3) = _8 | _8;
_13 = 1349084868_i32 as usize;
_3 = core::ptr::addr_of_mut!((*_3));
_6 = 9702737435227966917_u64;
Goto(bb7)
}
bb16 = {
(*_3) = -_2;
(*_3) = _1;
_10 = [(-69_i8),94_i8,14_i8];
(*_3) = _1 & _4;
(*_3) = _1 >> _2;
(*_3) = _4;
_8 = (*_3);
(*_3) = 6799080507687588217_i64 as isize;
(*_3) = !_2;
(*_3) = _2 - _1;
RET = 38398_u16 ^ 2103_u16;
(*_3) = _2;
(*_3) = _4;
_7 = '\u{f3126}';
(*_3) = _8 - _1;
_9 = [87_i8,96_i8];
_11 = [227_u8,56_u8,16_u8,76_u8];
(*_3) = _8 + _1;
(*_3) = _4;
(*_3) = 6_usize as isize;
(*_3) = _4 & _2;
(*_3) = _2 | _8;
_8 = (*_3) - (*_3);
(*_3) = _8 << _4;
Goto(bb4)
}
bb17 = {
_14 = &mut _31;
(*_3) = _8;
(*_14) = '\u{df2d6}';
(*_3) = _8;
(*_14) = '\u{5b95c}';
(*_12) = -(-94060037741161208491435072109348656957_i128);
_27 = _17;
match Field::<i8>(Variant(_33.fld1, 0), 2) {
0 => bb10,
1 => bb2,
2 => bb9,
3 => bb4,
4 => bb15,
5 => bb6,
340282366920938463463374607431768211359 => bb18,
_ => bb7
}
}
bb18 = {
(*_12) = 42644402980954934289219417025163326323_i128 + (-115557743656428122765057753740522917882_i128);
(*_3) = (-1244597453_i32) as isize;
(*_3) = _1 ^ _8;
(*_14) = '\u{af968}';
(*_14) = '\u{926e5}';
(*_12) = 55762753936158222803407587551919700503_i128 & 146430937510032516110392918748939326733_i128;
(*_14) = '\u{b903e}';
_19 = [104661197517336613684171665624148909718_u128,227933447691945863802689006471747157261_u128,248745292607046150134518636953842364865_u128,197014968698198103432026430551709011368_u128,105063362081798500300257055968367132207_u128,306456609622774623034424614006748884986_u128,295090338386435117556867886045711733586_u128];
Call(_33.fld1 = fn14((*_12), (*_14), (*_14), (*_3), _19), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
(*_12) = (-144268085419786628838439462053919811056_i128) | (-46111787997965815980590420175360901242_i128);
(*_3) = _1 * _8;
_13 = Field::<usize>(Variant(_33.fld1, 1), 7) | Field::<usize>(Variant(_33.fld1, 1), 7);
_35.0 = &mut (*_14);
_18 = _23 | _23;
(*_3) = _1;
match _22.0 {
0 => bb5,
1 => bb20,
2 => bb21,
3 => bb22,
4 => bb23,
5 => bb24,
6 => bb25,
175082776 => bb27,
_ => bb26
}
}
bb20 = {
(*_3) = _1;
(*_3) = _2 | _2;
(*_3) = _1 & _1;
(*_3) = _1 >> _2;
(*_3) = -_1;
(*_3) = _1 << _1;
_4 = (*_3) - (*_3);
(*_3) = false as isize;
(*_3) = _4 + _4;
(*_3) = _4 - _2;
(*_3) = _4;
(*_3) = _4;
(*_3) = 3372073094_u32 as isize;
(*_3) = _4 << _4;
(*_3) = (-88_i8) as isize;
_3 = core::ptr::addr_of_mut!((*_3));
_1 = _5 & _4;
(*_3) = _4 + _2;
(*_3) = _1;
(*_3) = _1 * _4;
_6 = !13160132228019498831_u64;
(*_3) = 73222566284233521041028716855306799595_i128 as isize;
(*_3) = _6 as isize;
_2 = !_4;
(*_3) = _4 ^ _4;
(*_3) = -_1;
Call(_1 = core::intrinsics::bswap((*_3)), ReturnTo(bb3), UnwindUnreachable())
}
bb21 = {
_14 = &mut _31;
(*_3) = _8;
(*_14) = '\u{df2d6}';
(*_3) = _8;
(*_14) = '\u{5b95c}';
(*_12) = -(-94060037741161208491435072109348656957_i128);
_27 = _17;
match Field::<i8>(Variant(_33.fld1, 0), 2) {
0 => bb10,
1 => bb2,
2 => bb9,
3 => bb4,
4 => bb15,
5 => bb6,
340282366920938463463374607431768211359 => bb18,
_ => bb7
}
}
bb22 = {
(*_3) = _8 | _8;
_1 = (*_3);
_1 = (*_3) - (*_3);
(*_3) = _8;
_10 = [28_i8,(-90_i8),(-68_i8)];
(*_3) = _8;
(*_3) = _8 | _8;
_13 = 1349084868_i32 as usize;
_3 = core::ptr::addr_of_mut!((*_3));
_6 = 9702737435227966917_u64;
Goto(bb7)
}
bb23 = {
_3 = core::ptr::addr_of_mut!((*_3));
(*_3) = _8 << _8;
(*_3) = _4 >> _8;
(*_3) = _4 | _8;
_9 = [(-83_i8),(-90_i8)];
_2 = 111_i8 as isize;
(*_3) = -_1;
(*_3) = 299923242962855035427038513919565966984_u128 as isize;
(*_3) = _1 + _1;
(*_3) = !_8;
_13 = 15763501494715851549_usize;
_9 = [(-48_i8),(-74_i8)];
Goto(bb5)
}
bb24 = {
(*_3) = _1;
(*_3) = _8 | _4;
RET = 9329_u16 * 55885_u16;
(*_3) = _8 + _8;
_8 = (*_3) + (*_3);
_16 = &mut _7;
Goto(bb6)
}
bb25 = {
Return()
}
bb26 = {
(*_12) = !(-75440029395989416886354565786842586069_i128);
_23 = _18 * _18;
(*_3) = _1;
_5 = _2 & _2;
_2 = 24358_i16 as isize;
(*_3) = (*_12) as isize;
(*_12) = 107666958723058557745005091016271196599_i128 * (-104102052316893470129199605235194009423_i128);
_22 = (175082776_u32,);
(*_3) = _8 << _8;
(*_12) = true as i128;
(*_3) = _8 - _1;
_20.1 = [13_i8,(-103_i8)];
_1 = _8 + (*_3);
_6 = 24_i8 as u64;
_13 = (-30355_i16) as usize;
_27 = [(*_12),(*_12)];
(*_12) = _23 as i128;
_26 = &_3;
(*_3) = _1;
Goto(bb13)
}
bb27 = {
_37 = core::ptr::addr_of_mut!(RET);
(*_12) = _33.fld2 as i128;
_20.1 = [Field::<i8>(Variant(_33.fld1, 1), 3),Field::<i8>(Variant(_33.fld1, 1), 3)];
_35 = (Move(_14),);
(*_3) = _1 + _1;
_18 = _23;
(*_12) = !50743156094991052148468153595249482419_i128;
(*_37) = 171495156966931140712365666951942189117_u128 as u16;
(*_12) = (-95953158954835931006484028456307359767_i128);
_17 = [(*_12),(*_12)];
place!(Field::<usize>(Variant(_33.fld1, 1), 7)) = _13;
(*_37) = 60686_u16;
_38 = _8 + (*_3);
(*_3) = _8;
(*_3) = _6 as isize;
place!(Field::<i16>(Variant(_33.fld1, 1), 4)) = !13509_i16;
(*_3) = _38 >> _38;
(*_37) = !38939_u16;
(*_12) = 4391361663420185414_i64 as i128;
(*_37) = 57408_u16 * 3618_u16;
(*_3) = _38 & _38;
(*_12) = _6 as i128;
(*_12) = !(-98147899627459894230405439027575816466_i128);
(*_12) = (-95703046446923354834507340936634441334_i128) * 45522440175782863852172655260312711868_i128;
place!(Field::<usize>(Variant(_33.fld1, 1), 7)) = Field::<i16>(Variant(_33.fld1, 1), 4) as usize;
Goto(bb28)
}
bb28 = {
_20.2 = core::ptr::addr_of!(_33);
RET = !61431_u16;
(*_12) = -(-128448759433916052737794253922202542927_i128);
place!(Field::<u32>(Variant(_33.fld1, 1), 5)) = _22.0 & _22.0;
_39 = !_13;
(*_3) = Field::<i16>(Variant(_33.fld1, 1), 4) as isize;
_12 = core::ptr::addr_of_mut!((*_12));
_19 = [312259720029144545611523624388419318411_u128,175146713866141824175678187188130418371_u128,105442796352579121770021945297619564207_u128,38012623654654511540240833058058834645_u128,55470082042040026276321936595669827919_u128,162874521042070804226220842462246142299_u128,286506110888958399394785801173875029457_u128];
(*_37) = 8439_u16;
(*_12) = (-138315724578991585718303245686319554271_i128) << (*_3);
_22 = (Field::<u32>(Variant(_33.fld1, 1), 5),);
(*_3) = _1;
RET = Field::<i16>(Variant(_33.fld1, 1), 4) as u16;
_39 = !_13;
(*_37) = 24941_u16 & 25999_u16;
_17 = _27;
(*_37) = !9381_u16;
(*_12) = 58706473211469445053770909712705785071_i128;
(*_3) = _33.fld2 as isize;
Goto(bb29)
}
bb29 = {
_42 = Field::<i16>(Variant(_33.fld1, 1), 4) ^ Field::<i16>(Variant(_33.fld1, 1), 4);
(*_37) = 2668_u16;
(*_3) = _38 - _1;
_33.fld0 = Field::<bool>(Variant(_33.fld1, 1), 0);
RET = !41008_u16;
_21 = core::ptr::addr_of!(place!(Field::<f32>(Variant(_33.fld1, 1), 6)));
(*_3) = _1 * _1;
(*_21) = _18 as f32;
_5 = _1 + _8;
(*_21) = _33.fld2 as f32;
(*_21) = _23 as f32;
(*_3) = !_38;
(*_21) = (*_37) as f32;
_32 = Field::<f64>(Variant(_33.fld1, 1), 1);
(*_37) = 41675_u16;
(*_37) = !51448_u16;
(*_21) = 5968466988376442474_i64 as f32;
(*_21) = Field::<i16>(Variant(_33.fld1, 1), 4) as f32;
(*_12) = -(-11723087451474813940681728068195284839_i128);
(*_3) = !_1;
(*_12) = 137643192473483162191743231931214561441_i128;
(*_37) = !14071_u16;
_18 = (-2033854997362636525_i64) as u8;
(*_3) = !_8;
Goto(bb30)
}
bb30 = {
place!(Field::<usize>(Variant(_33.fld1, 1), 7)) = !_13;
_46 = '\u{72d29}' as i128;
(*_12) = _46 | _46;
(*_37) = 637492298_i32 as u16;
_49 = _22.0 as f32;
place!(Field::<usize>(Variant(_33.fld1, 1), 7)) = '\u{10bfc4}' as usize;
_13 = _39;
(*_12) = _46 + _46;
(*_21) = 4749049576844657742_i64 as f32;
(*_3) = _8;
(*_3) = _42 as isize;
_22 = (Field::<u32>(Variant(_33.fld1, 1), 5),);
(*_37) = !53154_u16;
(*_12) = -_46;
match Field::<i8>(Variant(_33.fld1, 1), 3) {
340282366920938463463374607431768211379 => bb31,
_ => bb17
}
}
bb31 = {
_20.0 = [(*_12)];
(*_3) = (-1287891958056547685_i64) as isize;
(*_37) = 31146_u16 * 60968_u16;
place!(Field::<i8>(Variant(_33.fld1, 1), 3)) = (-67_i8) * (-9_i8);
_20.1 = _9;
Goto(bb32)
}
bb32 = {
_46 = -(*_12);
(*_37) = 164876543378371892321981172529019777806_u128 as u16;
_45 = !(*_12);
_12 = core::ptr::addr_of_mut!((*_12));
(*_21) = _49 * _49;
_33.fld1 = Adt21::Variant0 { fld0: 108677381353057459771726476678744918909_u128,fld1: 337795048_i32,fld2: (-56_i8) };
_33.fld1 = Adt21::Variant0 { fld0: 196987956237439499500377557372707662806_u128,fld1: 847537473_i32,fld2: (-51_i8) };
_21 = core::ptr::addr_of!(_49);
(*_3) = _38 | _4;
_37 = core::ptr::addr_of_mut!((*_37));
(*_12) = _45 * _46;
_2 = (*_3) + (*_3);
_3 = core::ptr::addr_of_mut!(_2);
_38 = (*_3) - (*_3);
_43 = &mut _20.1;
_48 = Adt52::Variant2 { fld0: (-2052561051_i32),fld1: _24,fld2: _11 };
place!(Field::<i8>(Variant(_33.fld1, 0), 2)) = 15_i8 | 50_i8;
_53 = core::ptr::addr_of_mut!((*_37));
place!(Field::<f64>(Variant(_48, 2), 1)) = -_32;
(*_37) = 1663_u16 | 20825_u16;
place!(Field::<u128>(Variant(_33.fld1, 0), 0)) = 98429410217912494046057943902193636533_u128 & 90488029518053632603842992072395587611_u128;
(*_43) = _9;
(*_12) = _33.fld2 as i128;
_59 = &place!(Field::<u128>(Variant(_33.fld1, 0), 0));
_54 = 916752837045699380_i64 as u8;
Goto(bb33)
}
bb33 = {
_6 = _32 as u64;
(*_21) = _39 as f32;
(*_21) = (*_59) as f32;
(*_12) = _45 * _46;
_3 = core::ptr::addr_of_mut!(_1);
(*_43) = [Field::<i8>(Variant(_33.fld1, 0), 2),Field::<i8>(Variant(_33.fld1, 0), 2)];
(*_3) = -_5;
(*_21) = 8359094139059783945_i64 as f32;
(*_43) = _9;
_60 = &_3;
_4 = (*_3) + (*_3);
_13 = _39 << (*_3);
(*_3) = -_38;
(*_21) = _13 as f32;
(*_3) = _2;
place!(Field::<u128>(Variant(_33.fld1, 0), 0)) = 203325523260771204953171423446614541633_u128 - 156996930498238327708696151436774655622_u128;
(*_37) = 27067_u16 * 56845_u16;
Goto(bb34)
}
bb34 = {
_4 = !(*_3);
(*_3) = !_38;
(*_37) = 33637_u16 - 59105_u16;
_33.fld1 = Adt21::Variant1 { fld0: _33.fld0,fld1: Field::<f64>(Variant(_48, 2), 1),fld2: (*_3),fld3: (-49_i8),fld4: _42,fld5: _22.0,fld6: (*_21),fld7: _13 };
Goto(bb35)
}
bb35 = {
_22.0 = !Field::<u32>(Variant(_33.fld1, 1), 5);
(*_12) = _46 - _46;
_40 = (*_12) - _25;
_22 = (Field::<u32>(Variant(_33.fld1, 1), 5),);
place!(Field::<f32>(Variant(_33.fld1, 1), 6)) = (*_21);
_67 = RET >> (*_37);
_67 = !(*_37);
(*_12) = _40 + _40;
Goto(bb36)
}
bb36 = {
_55 = _13 as u16;
_66 = (*_3) ^ (*_3);
(*_21) = Field::<f32>(Variant(_33.fld1, 1), 6) - Field::<f32>(Variant(_33.fld1, 1), 6);
(*_43) = [87_i8,37_i8];
_23 = _54 >> (*_3);
(*_3) = 4366387119119637052_i64 as isize;
(*_21) = Field::<f32>(Variant(_33.fld1, 1), 6) + Field::<f32>(Variant(_33.fld1, 1), 6);
(*_12) = !_40;
(*_43) = [(-82_i8),88_i8];
_2 = !_8;
place!(Field::<i8>(Variant(_33.fld1, 1), 3)) = !(-69_i8);
(*_43) = _9;
(*_12) = _4 as i128;
Goto(bb37)
}
bb37 = {
(*_43) = [Field::<i8>(Variant(_33.fld1, 1), 3),Field::<i8>(Variant(_33.fld1, 1), 3)];
place!(Field::<[u8; 4]>(Variant(_48, 2), 2)) = [_23,_23,_23,_18];
_58 = -(*_12);
place!(Field::<i32>(Variant(_48, 2), 0)) = Field::<isize>(Variant(_33.fld1, 1), 2) as i32;
_2 = _66 ^ _38;
(*_37) = !_55;
_15 = core::ptr::addr_of!(_47);
_12 = core::ptr::addr_of_mut!((*_12));
(*_15) = !(-7598983487686768368_i64);
_3 = core::ptr::addr_of_mut!(_8);
(*_12) = _58 ^ _40;
_3 = core::ptr::addr_of_mut!((*_3));
(*_21) = Field::<f32>(Variant(_33.fld1, 1), 6) + Field::<f32>(Variant(_33.fld1, 1), 6);
_5 = _2;
(*_37) = _55 & _55;
_9 = (*_43);
_32 = -Field::<f64>(Variant(_33.fld1, 1), 1);
_62.2 = core::ptr::addr_of!(_33);
_21 = core::ptr::addr_of!((*_21));
Goto(bb38)
}
bb38 = {
Call(_70 = dump_var(Move(_38), Move(_19), Move(_47), Move(_10)), ReturnTo(bb39), UnwindUnreachable())
}
bb39 = {
Call(_70 = dump_var(Move(_13), Move(_18), Move(_46), Move(_55)), ReturnTo(bb40), UnwindUnreachable())
}
bb40 = {
Call(_70 = dump_var(Move(_67), Move(_9), Move(_17), Move(_58)), ReturnTo(bb41), UnwindUnreachable())
}
bb41 = {
Call(_70 = dump_var(Move(_4), Move(_31), Move(_23), _71), ReturnTo(bb42), UnwindUnreachable())
}
bb42 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn14(mut _1: i128,mut _2: char,mut _3: char,mut _4: isize,mut _5: [u128; 7]) -> Adt21 {
mir! {
type RET = Adt21;
let _6: u16;
let _7: (&'static &'static char, i64, i16, i64);
let _8: (i16, &'static i64, [u32; 8], i16);
let _9: ();
let _10: ();
{
_1 = 166681401455689423691367632622679882013_i128 + 93708208995393376495706332569039456727_i128;
_3 = _2;
Call(RET = fn15(_4, _2, _2, _5, _4, _1, _2), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_3 = _2;
place!(Field::<i8>(Variant(RET, 1), 3)) = (-77_i8);
_6 = 31562_u16 & 33048_u16;
_3 = _2;
place!(Field::<usize>(Variant(RET, 1), 7)) = 18298475602413673105_usize | 8769891993807340814_usize;
place!(Field::<f32>(Variant(RET, 1), 6)) = Field::<i8>(Variant(RET, 1), 3) as f32;
Goto(bb2)
}
bb2 = {
Call(_9 = dump_var(Move(_1), Move(_4), Move(_5), _10), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn15(mut _1: isize,mut _2: char,mut _3: char,mut _4: [u128; 7],mut _5: isize,mut _6: i128,mut _7: char) -> Adt21 {
mir! {
type RET = Adt21;
let _8: &'static u128;
let _9: u16;
let _10: u128;
let _11: &'static mut [i8; 2];
let _12: f32;
let _13: &'static mut (i64,);
let _14: &'static mut char;
let _15: [i16; 6];
let _16: &'static u128;
let _17: f32;
let _18: (&'static u128, (i16, &'static i64, [u32; 8], i16), &'static u32);
let _19: &'static *mut isize;
let _20: Adt22;
let _21: u64;
let _22: f64;
let _23: (u64, u128);
let _24: ();
let _25: ();
{
_1 = _5 ^ _5;
_6 = (-100638190937147023163259621367301151689_i128) >> _1;
_2 = _7;
Call(_4 = fn16(_6, _6, _3, _3, _3, _2, _6, _1), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_4 = [325898774954085120884226427414471451639_u128,171662175054526615852881259694200033260_u128,135145182432733018121673862303192120718_u128,153565305236107044889841354831563909363_u128,250996446859525218564317300052948114956_u128,313016073593342387304778391577115854898_u128,147480053071267120378605021549480924453_u128];
_5 = _1 * _1;
RET = Adt21::Variant0 { fld0: 264499276161675156068916676453385208955_u128,fld1: (-932114521_i32),fld2: (-27_i8) };
place!(Field::<u128>(Variant(RET, 0), 0)) = !57196588356462753878354400273474247409_u128;
_5 = -_1;
place!(Field::<i32>(Variant(RET, 0), 1)) = 276664112_i32;
_3 = _7;
_4 = [Field::<u128>(Variant(RET, 0), 0),Field::<u128>(Variant(RET, 0), 0),Field::<u128>(Variant(RET, 0), 0),Field::<u128>(Variant(RET, 0), 0),Field::<u128>(Variant(RET, 0), 0),Field::<u128>(Variant(RET, 0), 0),Field::<u128>(Variant(RET, 0), 0)];
_8 = &place!(Field::<u128>(Variant(RET, 0), 0));
Goto(bb2)
}
bb2 = {
_5 = _1;
RET = Adt21::Variant0 { fld0: 235902095014500509612722727895549644856_u128,fld1: (-817814508_i32),fld2: 69_i8 };
_9 = 54148_u16;
place!(Field::<i32>(Variant(RET, 0), 1)) = -(-1299597788_i32);
_5 = (-25814_i16) as isize;
_6 = Field::<i32>(Variant(RET, 0), 1) as i128;
place!(Field::<i32>(Variant(RET, 0), 1)) = 290068735815339146511468560213629490240_u128 as i32;
_1 = 1173661522_u32 as isize;
place!(Field::<u128>(Variant(RET, 0), 0)) = 22073893496975557536961067380214539630_u128 * 266774716827422131639647761220119967247_u128;
_4 = [Field::<u128>(Variant(RET, 0), 0),Field::<u128>(Variant(RET, 0), 0),Field::<u128>(Variant(RET, 0), 0),Field::<u128>(Variant(RET, 0), 0),Field::<u128>(Variant(RET, 0), 0),Field::<u128>(Variant(RET, 0), 0),Field::<u128>(Variant(RET, 0), 0)];
_8 = &place!(Field::<u128>(Variant(RET, 0), 0));
RET = Adt21::Variant0 { fld0: 68623238156364845600520885819853573051_u128,fld1: (-545986357_i32),fld2: 49_i8 };
place!(Field::<i32>(Variant(RET, 0), 1)) = !(-1653017942_i32);
_10 = 0_usize as u128;
_1 = _5 ^ _5;
_10 = 333078457714958266882435867293589431130_u128 ^ 280239690471289156246190995304650709454_u128;
place!(Field::<i8>(Variant(RET, 0), 2)) = _5 as i8;
_5 = (-6815_i16) as isize;
place!(Field::<i32>(Variant(RET, 0), 1)) = (-492688603_i32) ^ (-301379718_i32);
_1 = _5 & _5;
_1 = !_5;
RET = Adt21::Variant0 { fld0: _10,fld1: (-99873527_i32),fld2: (-91_i8) };
match _9 {
0 => bb1,
54148 => bb4,
_ => bb3
}
}
bb3 = {
_4 = [325898774954085120884226427414471451639_u128,171662175054526615852881259694200033260_u128,135145182432733018121673862303192120718_u128,153565305236107044889841354831563909363_u128,250996446859525218564317300052948114956_u128,313016073593342387304778391577115854898_u128,147480053071267120378605021549480924453_u128];
_5 = _1 * _1;
RET = Adt21::Variant0 { fld0: 264499276161675156068916676453385208955_u128,fld1: (-932114521_i32),fld2: (-27_i8) };
place!(Field::<u128>(Variant(RET, 0), 0)) = !57196588356462753878354400273474247409_u128;
_5 = -_1;
place!(Field::<i32>(Variant(RET, 0), 1)) = 276664112_i32;
_3 = _7;
_4 = [Field::<u128>(Variant(RET, 0), 0),Field::<u128>(Variant(RET, 0), 0),Field::<u128>(Variant(RET, 0), 0),Field::<u128>(Variant(RET, 0), 0),Field::<u128>(Variant(RET, 0), 0),Field::<u128>(Variant(RET, 0), 0),Field::<u128>(Variant(RET, 0), 0)];
_8 = &place!(Field::<u128>(Variant(RET, 0), 0));
Goto(bb2)
}
bb4 = {
_9 = 242_u8 as u16;
_5 = _1 * _1;
_14 = &mut _3;
(*_14) = _7;
(*_14) = _7;
(*_14) = _7;
place!(Field::<i8>(Variant(RET, 0), 2)) = 82_i8;
(*_14) = _7;
_12 = 38_u8 as f32;
(*_14) = _7;
_1 = _5;
Goto(bb5)
}
bb5 = {
(*_14) = _2;
(*_14) = _7;
_12 = 13502228218862155312_u64 as f32;
(*_14) = _2;
(*_14) = _7;
_6 = (-82437900864471214312776172184929547609_i128);
place!(Field::<i32>(Variant(RET, 0), 1)) = -2070824761_i32;
_1 = _5 | _5;
(*_14) = _7;
(*_14) = _7;
_10 = _5 as u128;
(*_14) = _7;
(*_14) = _2;
_8 = &_10;
(*_14) = _7;
_7 = (*_14);
Goto(bb6)
}
bb6 = {
_16 = &(*_8);
(*_14) = _7;
_14 = &mut _7;
(*_14) = _2;
(*_14) = _2;
(*_14) = _2;
_16 = &place!(Field::<u128>(Variant(RET, 0), 0));
place!(Field::<i32>(Variant(RET, 0), 1)) = -1477248088_i32;
_5 = _1;
_10 = (*_16) + (*_16);
(*_14) = _2;
_18.1.3 = 18588_i16;
(*_14) = _2;
_16 = &_10;
(*_14) = _2;
match Field::<i8>(Variant(RET, 0), 2) {
82 => bb7,
_ => bb2
}
}
bb7 = {
(*_14) = _2;
_10 = Field::<u128>(Variant(RET, 0), 0) - Field::<u128>(Variant(RET, 0), 0);
(*_14) = _2;
_2 = (*_14);
_15 = [_18.1.3,_18.1.3,_18.1.3,_18.1.3,_18.1.3,_18.1.3];
_18.1.0 = _18.1.3;
_18.1.3 = _18.1.0 + _18.1.0;
_18.1.0 = _18.1.3 & _18.1.3;
_14 = &mut _2;
(*_14) = '\u{38fd0}';
_20 = Adt22 { fld0: true,fld1: RET,fld2: 4676261462523866535_u64,fld3: Field::<i8>(Variant(RET, 0), 2) };
_18.0 = &_10;
_15 = [_18.1.0,_18.1.0,_18.1.0,_18.1.0,_18.1.0,_18.1.0];
_22 = _18.1.0 as f64;
(*_14) = '\u{8f491}';
RET = Adt21::Variant1 { fld0: _20.fld0,fld1: _22,fld2: _5,fld3: _20.fld3,fld4: _18.1.0,fld5: 1815718047_u32,fld6: _12,fld7: 2_usize };
match Field::<i8>(Variant(_20.fld1, 0), 2) {
0 => bb1,
1 => bb4,
82 => bb9,
_ => bb8
}
}
bb8 = {
_5 = _1;
RET = Adt21::Variant0 { fld0: 235902095014500509612722727895549644856_u128,fld1: (-817814508_i32),fld2: 69_i8 };
_9 = 54148_u16;
place!(Field::<i32>(Variant(RET, 0), 1)) = -(-1299597788_i32);
_5 = (-25814_i16) as isize;
_6 = Field::<i32>(Variant(RET, 0), 1) as i128;
place!(Field::<i32>(Variant(RET, 0), 1)) = 290068735815339146511468560213629490240_u128 as i32;
_1 = 1173661522_u32 as isize;
place!(Field::<u128>(Variant(RET, 0), 0)) = 22073893496975557536961067380214539630_u128 * 266774716827422131639647761220119967247_u128;
_4 = [Field::<u128>(Variant(RET, 0), 0),Field::<u128>(Variant(RET, 0), 0),Field::<u128>(Variant(RET, 0), 0),Field::<u128>(Variant(RET, 0), 0),Field::<u128>(Variant(RET, 0), 0),Field::<u128>(Variant(RET, 0), 0),Field::<u128>(Variant(RET, 0), 0)];
_8 = &place!(Field::<u128>(Variant(RET, 0), 0));
RET = Adt21::Variant0 { fld0: 68623238156364845600520885819853573051_u128,fld1: (-545986357_i32),fld2: 49_i8 };
place!(Field::<i32>(Variant(RET, 0), 1)) = !(-1653017942_i32);
_10 = 0_usize as u128;
_1 = _5 ^ _5;
_10 = 333078457714958266882435867293589431130_u128 ^ 280239690471289156246190995304650709454_u128;
place!(Field::<i8>(Variant(RET, 0), 2)) = _5 as i8;
_5 = (-6815_i16) as isize;
place!(Field::<i32>(Variant(RET, 0), 1)) = (-492688603_i32) ^ (-301379718_i32);
_1 = _5 & _5;
_1 = !_5;
RET = Adt21::Variant0 { fld0: _10,fld1: (-99873527_i32),fld2: (-91_i8) };
match _9 {
0 => bb1,
54148 => bb4,
_ => bb3
}
}
bb9 = {
place!(Field::<f64>(Variant(RET, 1), 1)) = _22;
_20.fld3 = Field::<i8>(Variant(RET, 1), 3) | Field::<i8>(Variant(_20.fld1, 0), 2);
_22 = Field::<f64>(Variant(RET, 1), 1);
(*_14) = '\u{60fd}';
_10 = Field::<u128>(Variant(_20.fld1, 0), 0) << _18.1.0;
place!(Field::<bool>(Variant(RET, 1), 0)) = _20.fld0 ^ _20.fld0;
_18.1.2 = [2510177397_u32,3409496161_u32,264881409_u32,114577651_u32,3198096615_u32,2041758927_u32,4160982035_u32,4176567152_u32];
place!(Field::<i16>(Variant(RET, 1), 4)) = _18.1.3;
_21 = 1304605584_u32 as u64;
_20.fld2 = _21 | _21;
_21 = Field::<bool>(Variant(RET, 1), 0) as u64;
place!(Field::<usize>(Variant(RET, 1), 7)) = _20.fld2 as usize;
place!(Field::<u32>(Variant(RET, 1), 5)) = !483209008_u32;
_18.2 = &place!(Field::<u32>(Variant(RET, 1), 5));
place!(Field::<i8>(Variant(RET, 1), 3)) = _20.fld3;
_12 = -Field::<f32>(Variant(RET, 1), 6);
place!(Field::<i16>(Variant(RET, 1), 4)) = !_18.1.0;
place!(Field::<i8>(Variant(_20.fld1, 0), 2)) = Field::<i8>(Variant(RET, 1), 3) ^ _20.fld3;
_4 = [_10,_10,_10,_10,Field::<u128>(Variant(_20.fld1, 0), 0),_10,_10];
(*_14) = '\u{32dd3}';
_23.0 = !_20.fld2;
(*_14) = '\u{f5299}';
place!(Field::<i8>(Variant(RET, 1), 3)) = -Field::<i8>(Variant(_20.fld1, 0), 2);
_18.0 = &place!(Field::<u128>(Variant(_20.fld1, 0), 0));
_10 = Field::<u128>(Variant(_20.fld1, 0), 0) + Field::<u128>(Variant(_20.fld1, 0), 0);
(*_14) = '\u{3272}';
_20.fld2 = _23.0 ^ _21;
Goto(bb10)
}
bb10 = {
Call(_24 = dump_var(Move(_4), Move(_2), Move(_6), Move(_3)), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
Call(_24 = dump_var(Move(_5), _25, _25, _25), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn16(mut _1: i128,mut _2: i128,mut _3: char,mut _4: char,mut _5: char,mut _6: char,mut _7: i128,mut _8: isize) -> [u128; 7] {
mir! {
type RET = [u128; 7];
let _9: isize;
let _10: &'static u128;
let _11: f64;
let _12: *const Adt22;
let _13: char;
let _14: &'static *mut isize;
let _15: &'static char;
let _16: f32;
let _17: f32;
let _18: isize;
let _19: *mut isize;
let _20: *const Adt22;
let _21: bool;
let _22: isize;
let _23: f32;
let _24: isize;
let _25: bool;
let _26: i16;
let _27: bool;
let _28: char;
let _29: Adt21;
let _30: [u32; 8];
let _31: Adt52;
let _32: i64;
let _33: *const f32;
let _34: f32;
let _35: *mut &'static *mut isize;
let _36: isize;
let _37: char;
let _38: *mut u16;
let _39: (i16, &'static i64, [u32; 8], i16);
let _40: isize;
let _41: *const Adt22;
let _42: [u128; 7];
let _43: ([i128; 2], &'static mut *mut i128, i16);
let _44: char;
let _45: isize;
let _46: &'static u128;
let _47: isize;
let _48: [char; 8];
let _49: [i8; 2];
let _50: f64;
let _51: i32;
let _52: (&'static char, [u128; 7], *const f32, *const [i32; 5]);
let _53: (i16, &'static i64, [u32; 8], i16);
let _54: f32;
let _55: *const (f32, i16, usize, u8);
let _56: *const f32;
let _57: isize;
let _58: char;
let _59: f32;
let _60: *mut &'static *mut isize;
let _61: &'static *mut isize;
let _62: i32;
let _63: (&'static mut char,);
let _64: (i64,);
let _65: *const i32;
let _66: [u32; 8];
let _67: f32;
let _68: bool;
let _69: ((u32,), &'static char, bool, bool);
let _70: isize;
let _71: char;
let _72: ([i128; 2], &'static mut *mut i128, i16);
let _73: bool;
let _74: isize;
let _75: char;
let _76: f32;
let _77: isize;
let _78: bool;
let _79: &'static u128;
let _80: [bool; 2];
let _81: f32;
let _82: ((&'static mut *mut i128,), (f32, i16, usize, u8), (&'static &'static char, i64, i16, i64), &'static i64);
let _83: i16;
let _84: u64;
let _85: *mut [u8; 4];
let _86: isize;
let _87: &'static mut *mut i128;
let _88: &'static u128;
let _89: isize;
let _90: isize;
let _91: &'static char;
let _92: i16;
let _93: usize;
let _94: (f32, i16, usize, u8);
let _95: f64;
let _96: isize;
let _97: isize;
let _98: &'static mut [i8; 2];
let _99: i64;
let _100: i64;
let _101: &'static mut char;
let _102: char;
let _103: &'static &'static char;
let _104: *const i64;
let _105: *mut [u8; 4];
let _106: &'static *mut isize;
let _107: &'static u32;
let _108: ();
let _109: ();
{
_8 = (-40_isize);
RET = [159614847177654469576806688732740564522_u128,290471604944969995179860958467000327671_u128,285905736801725917011314868587237361118_u128,169554248976485734121929513940137228911_u128,331034999676531299761530207796864352180_u128,143036195900426642581449492198771072928_u128,325578818476615144668797509508008575176_u128];
_6 = _4;
_1 = 993_u16 as i128;
_8 = !94_isize;
_6 = _5;
_3 = _4;
RET = [203586684584203871663283002321972802295_u128,137130975745908370635262615809754243109_u128,84549675477693190009016674944076324188_u128,302772272446917749424520502610254872924_u128,10543019611052558921266203252523904006_u128,91865700430192739449578069037530190002_u128,42895266996846738579946195012792608892_u128];
_3 = _4;
_7 = _2 | _2;
RET = [280806195255058623351793857575607818636_u128,10178747840483972154321846761196136531_u128,325696097923342267700621216316674348318_u128,256288336528683287313061920589199364988_u128,6029565423507122493318670886676369445_u128,318987857281443284963229753900165195970_u128,92996016351326459295490289770869513035_u128];
_4 = _5;
RET = [307543207138487235868382168218980206934_u128,197398808448959080921434965302774702036_u128,317532148502661607624045531353206124794_u128,300334546864259082000287299242535870727_u128,312690881007074359499018526974050582308_u128,132795408149340988121032496451540348388_u128,331342495984909187467523071079941039951_u128];
_1 = !_7;
_3 = _4;
_7 = _4 as i128;
_1 = _2;
_4 = _5;
_9 = 20_i8 as isize;
_7 = 531339886457873196_u64 as i128;
_3 = _5;
Goto(bb1)
}
bb1 = {
_7 = 13457703656818073981_usize as i128;
_6 = _5;
_1 = _2 ^ _2;
_4 = _5;
_9 = _8 << _2;
RET = [331077147784713877846082749713137949755_u128,259796619994597835583551859102925919962_u128,132179071729118013423336071745197050984_u128,109861805171838575309306313647548883143_u128,145480909639994171577792464226375953801_u128,73689074102917443452611869585350904429_u128,183578017340537109039488544560272819866_u128];
_11 = 209509172003190162637752855516918935261_u128 as f64;
_11 = _9 as f64;
_2 = 969659311_u32 as i128;
_7 = !_1;
_8 = 4006_i16 as isize;
_6 = _4;
_3 = _5;
_8 = 2_usize as isize;
_6 = _5;
_2 = _1 & _1;
_11 = _7 as f64;
_4 = _6;
_7 = _11 as i128;
Goto(bb2)
}
bb2 = {
_2 = _1 + _7;
_11 = 224_u8 as f64;
_1 = _2;
_2 = !_1;
_13 = _5;
_9 = _8;
_6 = _4;
_7 = -_1;
_6 = _13;
_5 = _3;
_7 = 40_i8 as i128;
_11 = 167632931225751647351087832157186147666_u128 as f64;
_7 = _1;
_13 = _3;
_9 = !_8;
_1 = -_7;
_8 = -_9;
_6 = _13;
_7 = -_1;
_5 = _6;
_3 = _5;
_8 = _9 | _9;
_9 = _8 & _8;
_7 = 4525_u16 as i128;
_5 = _4;
_1 = _11 as i128;
Goto(bb3)
}
bb3 = {
_11 = _9 as f64;
_1 = _2;
_7 = -_2;
_15 = &_3;
_8 = _9;
_2 = 1635_u16 as i128;
_1 = _7 - _7;
_11 = 18035837419729153765_usize as f64;
_3 = _13;
_11 = 14493458334519667569_u64 as f64;
_18 = _9 & _9;
_8 = _18 ^ _18;
_4 = _13;
_5 = _4;
_4 = _6;
_17 = 5007_u16 as f32;
_6 = _4;
_18 = !_9;
Goto(bb4)
}
bb4 = {
_5 = _6;
_5 = _13;
_3 = _4;
_16 = _11 as f32;
_8 = !_9;
_4 = _3;
_15 = &_4;
_9 = -_8;
_21 = true | false;
_16 = _17;
_6 = (*_15);
_5 = (*_15);
_11 = _17 as f64;
_8 = _9 - _18;
_11 = 128558883387734544814186302068795715307_u128 as f64;
_8 = !_18;
_15 = &_13;
_4 = (*_15);
_7 = -_1;
_15 = &_3;
_13 = (*_15);
_7 = 6_usize as i128;
_22 = 16930381632368161024_u64 as isize;
Call(_20 = fn17((*_15), Move(_15), (*_15), (*_15), (*_15), (*_15), _22, (*_15), (*_15), _3, (*_15), (*_15)), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_19 = core::ptr::addr_of_mut!(_8);
_8 = _9;
(*_19) = _13 as isize;
(*_19) = _18 | _18;
(*_19) = _18 | _9;
_15 = &_13;
(*_19) = _18 | _9;
(*_19) = _9;
_15 = &_3;
_13 = (*_15);
_1 = _2;
_2 = -_7;
(*_19) = _18 + _9;
_23 = (-53_i8) as f32;
(*_19) = (*_15) as isize;
(*_19) = _9 - _9;
_21 = false;
_17 = -_16;
(*_19) = _9 >> _22;
_3 = _4;
_26 = 29240_i16 | (-11431_i16);
_24 = _8 << (*_19);
_25 = _18 == _24;
Call((*_19) = core::intrinsics::transmute(_18), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
(*_19) = _22;
_6 = _3;
_3 = _4;
_16 = _2 as f32;
(*_19) = -_18;
_24 = !(*_19);
_17 = (-60_i8) as f32;
_2 = !_7;
(*_19) = !_9;
_11 = 6128_u16 as f64;
_14 = &_19;
(*_19) = !_9;
_2 = _16 as i128;
_11 = 4213751887_u32 as f64;
Goto(bb7)
}
bb7 = {
(*_19) = 4000426202_u32 as isize;
_12 = Move(_20);
(*_19) = _2 as isize;
_9 = _3 as isize;
(*_19) = _24 ^ _24;
_13 = _6;
(*_19) = _24;
_25 = _21 ^ _21;
_30 = [4081704382_u32,1866092474_u32,2531570176_u32,715294317_u32,3516286116_u32,3644725941_u32,4238371289_u32,3249457284_u32];
(*_19) = -_24;
(*_19) = 14715585692148657775_u64 as isize;
(*_19) = _24 * _18;
(*_19) = _1 as isize;
_7 = _11 as i128;
Goto(bb8)
}
bb8 = {
(*_19) = !_24;
_3 = _5;
RET = [326519667576607411831698293913394037076_u128,18631181469247441010265257104300130866_u128,114790350281731708335006700557876172123_u128,159307254000798425416557348723522454416_u128,136362691441617215575745130586022805482_u128,156701628126213286952829390089849349173_u128,309253432618272658880685667022527988231_u128];
(*_19) = _18;
(*_19) = !_24;
_27 = !_21;
_19 = core::ptr::addr_of_mut!(_36);
_25 = !_21;
(*_19) = _9 * _24;
_16 = -_17;
(*_19) = _22 << _8;
_9 = _7 as isize;
_28 = _5;
_13 = _28;
_28 = _3;
(*_19) = _9;
_32 = 5129790111948509636_usize as i64;
(*_19) = _8;
_19 = core::ptr::addr_of_mut!((*_19));
Call(_36 = core::intrinsics::bswap(_8), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
(*_19) = (-83_i8) as isize;
_16 = _17;
(*_19) = _24;
Goto(bb10)
}
bb10 = {
_34 = 16230673699079607090_u64 as f32;
(*_19) = _8 * _24;
(*_19) = _18 << _32;
(*_19) = _9 ^ _24;
(*_19) = _24 - _18;
_21 = (*_19) == (*_19);
_37 = _6;
_33 = core::ptr::addr_of!(_17);
_17 = _23 * _16;
(*_33) = _34 * _16;
(*_33) = _23 - _23;
_24 = (*_19) * (*_19);
_21 = (*_19) > (*_19);
Goto(bb11)
}
bb11 = {
(*_19) = -_24;
_6 = _3;
_7 = !_1;
(*_33) = -_34;
(*_33) = 91648236297086962987155801562009597247_u128 as f32;
_23 = (*_33);
(*_33) = _34 + _23;
(*_19) = _9 * _24;
(*_19) = (*_33) as isize;
(*_19) = !_24;
Goto(bb12)
}
bb12 = {
(*_19) = _9 & _9;
RET = [217000334776842713943587939388663027592_u128,319937319566285002606934018823729271561_u128,115467777143490527802590376868482269316_u128,298100573503365631164441905613722448802_u128,216649904981288502063307126848573927176_u128,250721837586239809658610699625072952344_u128,66568455374329649366476005483792289419_u128];
_33 = core::ptr::addr_of!((*_33));
(*_19) = _24 + _24;
(*_33) = _34 * _34;
(*_19) = _24 * _24;
_23 = _26 as f32;
_15 = &_5;
_4 = (*_15);
_3 = (*_15);
(*_33) = _23 + _16;
_13 = (*_15);
(*_19) = _8 + _24;
(*_19) = _18 >> _18;
(*_19) = _24 << _24;
_8 = _24;
Goto(bb13)
}
bb13 = {
_8 = (*_19) * (*_19);
(*_19) = -_8;
_21 = (*_19) > (*_19);
_5 = _6;
_21 = _25;
_33 = core::ptr::addr_of!((*_33));
(*_33) = _34 * _34;
(*_19) = _9 & _8;
(*_19) = _8;
_13 = _28;
RET = [234020897489436523367230971733247140363_u128,269553268952951119687345980443591763594_u128,243535614375210420120669491601414936794_u128,186953725355179189067579146964801882284_u128,132463821795241812566663136631421200938_u128,156958712751170530314455415386285079697_u128,133689982574687277702522338918171200532_u128];
_30 = [3898141891_u32,1629779478_u32,3327794820_u32,3527328013_u32,2757934742_u32,354104659_u32,4283144011_u32,4206754576_u32];
_15 = &_4;
_5 = _28;
(*_33) = _16 + _23;
(*_19) = 10466487377229645046_u64 as isize;
(*_33) = _16;
(*_19) = -_8;
_39.1 = &_32;
(*_33) = _16 * _34;
(*_33) = _23;
_39.3 = _26;
_42 = RET;
_21 = (*_19) < _8;
(*_33) = _23 - _16;
(*_19) = -_8;
Goto(bb14)
}
bb14 = {
(*_33) = _23;
(*_33) = _16 + _16;
_3 = (*_15);
_16 = (*_33) + (*_33);
(*_19) = (-19_i8) as isize;
(*_33) = _34 - _16;
(*_19) = _8;
(*_33) = _23 - _34;
_42 = [148929331337186684072021708021805404838_u128,52013761325943908032596167953637730520_u128,51902035717510182630725497886598623321_u128,75833881322197300813113815171563101233_u128,98921011370112943621142611101103790744_u128,78365395910354917945171514483950425588_u128,52578479596261694643893500995342183266_u128];
_43.2 = _26 | _26;
(*_19) = _8 + _8;
_26 = _43.2 >> (*_19);
_20 = Move(_12);
_42 = [155617452852977358614287323686580237801_u128,222038424988763972415346993240414637776_u128,24132059562577695270007436784741204220_u128,187823765075481988897572355707934475457_u128,21507815203356000599577103138291204164_u128,174154795046410140831038479253430087909_u128,226475335946838437763946424683067841922_u128];
_39.0 = !_26;
(*_33) = 184_u8 as f32;
_16 = -(*_33);
_39.0 = _26 ^ _26;
_40 = (*_19) ^ (*_19);
(*_19) = _40;
(*_33) = -_23;
(*_33) = _23;
_39.2 = _30;
(*_19) = _40 >> _39.0;
_32 = (*_19) as i64;
Goto(bb15)
}
bb15 = {
(*_19) = -_40;
(*_19) = _8 >> _26;
_33 = core::ptr::addr_of!((*_33));
(*_19) = _8 & _40;
_41 = Move(_20);
_20 = Move(_41);
_47 = (*_19) + (*_19);
(*_19) = _47 | _40;
(*_19) = _47 >> _40;
(*_33) = _23 - _16;
Goto(bb16)
}
bb16 = {
_35 = core::ptr::addr_of_mut!(_14);
(*_19) = _40;
_44 = _28;
_49 = [77_i8,67_i8];
_17 = _23 - _23;
_15 = &_37;
(*_33) = _1 as f32;
_40 = !(*_19);
(*_19) = _47 >> _47;
_24 = !(*_19);
_37 = _13;
(*_19) = _24 + _47;
(*_19) = _8;
(*_33) = _16 - _23;
Goto(bb17)
}
bb17 = {
_28 = _6;
_52.2 = core::ptr::addr_of!((*_33));
(*_33) = _34 * _23;
_51 = !1303075349_i32;
(*_19) = _40 * _40;
_43.0 = [_1,_7];
_18 = (*_19) - (*_19);
_1 = (*_19) as i128;
(*_33) = _34 - _16;
_5 = _3;
(*_19) = _24;
_26 = !_39.0;
_35 = core::ptr::addr_of_mut!((*_35));
_5 = _4;
_41 = Move(_20);
(*_19) = !_18;
Goto(bb18)
}
bb18 = {
(*_33) = _16;
_25 = (*_19) >= (*_19);
_39.2 = [3116972962_u32,3030460708_u32,2679610493_u32,3641074799_u32,3816282802_u32,4082006690_u32,4122159362_u32,3587384080_u32];
_1 = _7 ^ _2;
_33 = core::ptr::addr_of!((*_33));
(*_33) = _23;
_29 = Adt21::Variant0 { fld0: 214680201230760478767543069798550118620_u128,fld1: _51,fld2: 50_i8 };
_29 = Adt21::Variant1 { fld0: _21,fld1: _11,fld2: (*_19),fld3: 7_i8,fld4: _26,fld5: 1278005593_u32,fld6: (*_33),fld7: 4_usize };
Goto(bb19)
}
bb19 = {
_53.1 = &_32;
(*_33) = -_23;
(*_19) = _7 as isize;
_48 = [_44,_6,_37,_28,_13,_4,_6,_13];
(*_19) = _18 ^ _8;
_2 = _1;
place!(Field::<usize>(Variant(_29, 1), 7)) = 9122645344911315949_usize << (*_19);
(*_33) = Field::<f32>(Variant(_29, 1), 6);
(*_19) = Field::<bool>(Variant(_29, 1), 0) as isize;
(*_33) = -_34;
(*_33) = _23 + _23;
_23 = (*_33) - (*_33);
_3 = _4;
_58 = _44;
_39.3 = Field::<i16>(Variant(_29, 1), 4) & _39.0;
(*_33) = _23;
_54 = (*_33);
place!(Field::<f32>(Variant(_29, 1), 6)) = _51 as f32;
_33 = core::ptr::addr_of!((*_33));
(*_19) = _8 - _24;
(*_33) = _23 + _54;
place!(Field::<f64>(Variant(_29, 1), 1)) = _11 + _11;
(*_19) = _24;
_52.2 = core::ptr::addr_of!(_23);
_40 = (*_19) << _18;
place!(Field::<f32>(Variant(_29, 1), 6)) = _51 as f32;
Goto(bb20)
}
bb20 = {
_52.0 = &_6;
_25 = !_21;
_53.0 = _39.3 << _26;
Goto(bb21)
}
bb21 = {
_49 = [123_i8,109_i8];
(*_33) = 1841820218_u32 as f32;
(*_33) = _54 * _54;
_60 = core::ptr::addr_of_mut!((*_35));
_39.0 = (-109_i8) as i16;
_48 = [_37,_6,_4,_3,_3,_6,_28,_28];
_17 = _23 + _23;
_64.0 = _32 + _32;
_8 = (*_19);
(*_19) = (-87_i8) as isize;
Goto(bb22)
}
bb22 = {
(*_19) = (*_33) as isize;
_68 = !_21;
(*_19) = _18;
(*_19) = _8;
_68 = !_21;
_26 = _39.3 - _39.3;
_52.2 = core::ptr::addr_of!((*_33));
_61 = &_19;
place!(Field::<i8>(Variant(_29, 1), 3)) = (-63_i8);
(*_33) = 107_u8 as f32;
(*_35) = Move(_61);
(*_19) = -_47;
_60 = Move(_35);
_20 = Move(_41);
_39.3 = _53.0 + _53.0;
_29 = Adt21::Variant1 { fld0: _25,fld1: _11,fld2: (*_19),fld3: (-24_i8),fld4: _39.3,fld5: 2085596182_u32,fld6: (*_33),fld7: 8005324047524192903_usize };
_34 = _54 + (*_33);
_29 = Adt21::Variant1 { fld0: _68,fld1: _11,fld2: (*_19),fld3: 95_i8,fld4: _26,fld5: 3901092108_u32,fld6: _23,fld7: 5991203569580348723_usize };
_69.0.0 = !707436995_u32;
_53.1 = Move(_39.1);
_43.0 = [_2,_1];
_57 = (*_19) & (*_19);
_56 = core::ptr::addr_of!((*_33));
_62 = _51 << (*_19);
(*_19) = _8 & _47;
(*_33) = 15350_u16 as f32;
Goto(bb23)
}
bb23 = {
RET = [112864078486472006836041954046934154065_u128,302061379535453556624254153056569614029_u128,126068052722308021906491899992074639175_u128,288313787291561729852342416583322014771_u128,316585669272140740088690115618304903721_u128,310096660583382582908363264294467882976_u128,81120774835415097857666666491867876786_u128];
(*_33) = -_34;
Goto(bb24)
}
bb24 = {
(*_19) = _47;
_69.1 = Move(_15);
_16 = -(*_33);
_61 = &_19;
_73 = _53.0 >= _26;
RET = _42;
_69.2 = (*_19) != (*_19);
_9 = _64.0 as isize;
_54 = 10257510912470716543_usize as f32;
_21 = (*_19) > (*_19);
_39.1 = &_64.0;
(*_19) = _18;
_39.3 = !_53.0;
_72.0 = [_2,_7];
_39.2 = [_69.0.0,_69.0.0,_69.0.0,_69.0.0,_69.0.0,_69.0.0,_69.0.0,_69.0.0];
_66 = [_69.0.0,_69.0.0,_69.0.0,_69.0.0,_69.0.0,_69.0.0,_69.0.0,_69.0.0];
_70 = (*_19);
place!(Field::<u32>(Variant(_29, 1), 5)) = _69.0.0 >> (*_19);
Goto(bb25)
}
bb25 = {
_17 = -_16;
_72.0 = [_1,_1];
_77 = (*_19) | (*_19);
(*_19) = (-91_i8) as isize;
_26 = -_39.3;
(*_19) = _70 * _57;
(*_33) = _34 * _34;
Goto(bb26)
}
bb26 = {
_78 = (*_19) >= (*_19);
_52.1 = [233056578934867172133256749781121146939_u128,66446616470279200717410750996807323345_u128,8314813831134108012666786930732577654_u128,136006474815211943645508272203302694398_u128,255655138016248850775553799197519419198_u128,244729120606539423570650836676791335731_u128,297985513066365511896811123924488864005_u128];
_64 = (_32,);
place!(Field::<i8>(Variant(_29, 1), 3)) = -70_i8;
Goto(bb27)
}
bb27 = {
_71 = _28;
_52.0 = &_37;
_53.3 = !_39.3;
_72.2 = Field::<i16>(Variant(_29, 1), 4);
place!(Field::<f64>(Variant(_29, 1), 1)) = _11 - _11;
_45 = (*_19) & (*_19);
_69.3 = !_78;
_67 = (*_33) - (*_33);
_52.2 = core::ptr::addr_of!((*_33));
_13 = _4;
_69.2 = !_69.3;
_81 = 5_usize as f32;
_29 = Adt21::Variant1 { fld0: _69.2,fld1: _11,fld2: (*_19),fld3: (-91_i8),fld4: _53.0,fld5: _69.0.0,fld6: (*_33),fld7: 2_usize };
_59 = (*_33) - (*_33);
(*_19) = _18;
(*_33) = Field::<f32>(Variant(_29, 1), 6) - _67;
_75 = _28;
_24 = -(*_19);
place!(Field::<bool>(Variant(_29, 1), 0)) = !_21;
_25 = !_21;
_40 = (*_19) >> (*_19);
_5 = _44;
Goto(bb28)
}
bb28 = {
_82.1.0 = Field::<f64>(Variant(_29, 1), 1) as f32;
_29 = Adt21::Variant1 { fld0: _78,fld1: _11,fld2: (*_19),fld3: (-42_i8),fld4: _53.3,fld5: _69.0.0,fld6: _16,fld7: 7_usize };
_3 = _58;
_33 = core::ptr::addr_of!(_67);
(*_19) = Field::<isize>(Variant(_29, 1), 2) & _57;
Goto(bb29)
}
bb29 = {
(*_19) = !_24;
_22 = (*_19);
_12 = Move(_20);
_69.0 = (Field::<u32>(Variant(_29, 1), 5),);
(*_19) = 10672_u16 as isize;
(*_33) = _17;
(*_19) = !_18;
_76 = _67 * (*_33);
_50 = _11 + Field::<f64>(Variant(_29, 1), 1);
_82.2.2 = !Field::<i16>(Variant(_29, 1), 4);
place!(Field::<usize>(Variant(_29, 1), 7)) = !12616144206474716470_usize;
_3 = _58;
_39.0 = !_53.3;
place!(Field::<isize>(Variant(_29, 1), 2)) = -(*_19);
_66 = [Field::<u32>(Variant(_29, 1), 5),Field::<u32>(Variant(_29, 1), 5),Field::<u32>(Variant(_29, 1), 5),_69.0.0,_69.0.0,Field::<u32>(Variant(_29, 1), 5),_69.0.0,Field::<u32>(Variant(_29, 1), 5)];
_82.1.1 = !_39.3;
Goto(bb30)
}
bb30 = {
_44 = _37;
place!(Field::<i8>(Variant(_29, 1), 3)) = 69_i8 - (-97_i8);
_33 = core::ptr::addr_of!((*_33));
_53.2 = [_69.0.0,_69.0.0,Field::<u32>(Variant(_29, 1), 5),Field::<u32>(Variant(_29, 1), 5),Field::<u32>(Variant(_29, 1), 5),Field::<u32>(Variant(_29, 1), 5),_69.0.0,_69.0.0];
(*_33) = 326839020618291169881390072445839673349_u128 as f32;
_82.1.2 = _69.0.0 as usize;
Goto(bb31)
}
bb31 = {
_77 = _47 >> (*_19);
_66 = [_69.0.0,_69.0.0,Field::<u32>(Variant(_29, 1), 5),Field::<u32>(Variant(_29, 1), 5),_69.0.0,_69.0.0,_69.0.0,_69.0.0];
_57 = _22 ^ (*_19);
_29 = Adt21::Variant0 { fld0: 235712088407606096300041012264003684524_u128,fld1: _62,fld2: (-60_i8) };
_82.1 = (_17, _53.0, 2_usize, 78_u8);
_32 = -_64.0;
(*_33) = _17 - _76;
match _82.1.3 {
0 => bb32,
1 => bb33,
2 => bb34,
3 => bb35,
4 => bb36,
78 => bb38,
_ => bb37
}
}
bb32 = {
_11 = _9 as f64;
_1 = _2;
_7 = -_2;
_15 = &_3;
_8 = _9;
_2 = 1635_u16 as i128;
_1 = _7 - _7;
_11 = 18035837419729153765_usize as f64;
_3 = _13;
_11 = 14493458334519667569_u64 as f64;
_18 = _9 & _9;
_8 = _18 ^ _18;
_4 = _13;
_5 = _4;
_4 = _6;
_17 = 5007_u16 as f32;
_6 = _4;
_18 = !_9;
Goto(bb4)
}
bb33 = {
(*_19) = -_24;
_6 = _3;
_7 = !_1;
(*_33) = -_34;
(*_33) = 91648236297086962987155801562009597247_u128 as f32;
_23 = (*_33);
(*_33) = _34 + _23;
(*_19) = _9 * _24;
(*_19) = (*_33) as isize;
(*_19) = !_24;
Goto(bb12)
}
bb34 = {
_28 = _6;
_52.2 = core::ptr::addr_of!((*_33));
(*_33) = _34 * _23;
_51 = !1303075349_i32;
(*_19) = _40 * _40;
_43.0 = [_1,_7];
_18 = (*_19) - (*_19);
_1 = (*_19) as i128;
(*_33) = _34 - _16;
_5 = _3;
(*_19) = _24;
_26 = !_39.0;
_35 = core::ptr::addr_of_mut!((*_35));
_5 = _4;
_41 = Move(_20);
(*_19) = !_18;
Goto(bb18)
}
bb35 = {
(*_19) = (*_33) as isize;
_68 = !_21;
(*_19) = _18;
(*_19) = _8;
_68 = !_21;
_26 = _39.3 - _39.3;
_52.2 = core::ptr::addr_of!((*_33));
_61 = &_19;
place!(Field::<i8>(Variant(_29, 1), 3)) = (-63_i8);
(*_33) = 107_u8 as f32;
(*_35) = Move(_61);
(*_19) = -_47;
_60 = Move(_35);
_20 = Move(_41);
_39.3 = _53.0 + _53.0;
_29 = Adt21::Variant1 { fld0: _25,fld1: _11,fld2: (*_19),fld3: (-24_i8),fld4: _39.3,fld5: 2085596182_u32,fld6: (*_33),fld7: 8005324047524192903_usize };
_34 = _54 + (*_33);
_29 = Adt21::Variant1 { fld0: _68,fld1: _11,fld2: (*_19),fld3: 95_i8,fld4: _26,fld5: 3901092108_u32,fld6: _23,fld7: 5991203569580348723_usize };
_69.0.0 = !707436995_u32;
_53.1 = Move(_39.1);
_43.0 = [_2,_1];
_57 = (*_19) & (*_19);
_56 = core::ptr::addr_of!((*_33));
_62 = _51 << (*_19);
(*_19) = _8 & _47;
(*_33) = 15350_u16 as f32;
Goto(bb23)
}
bb36 = {
_8 = (*_19) * (*_19);
(*_19) = -_8;
_21 = (*_19) > (*_19);
_5 = _6;
_21 = _25;
_33 = core::ptr::addr_of!((*_33));
(*_33) = _34 * _34;
(*_19) = _9 & _8;
(*_19) = _8;
_13 = _28;
RET = [234020897489436523367230971733247140363_u128,269553268952951119687345980443591763594_u128,243535614375210420120669491601414936794_u128,186953725355179189067579146964801882284_u128,132463821795241812566663136631421200938_u128,156958712751170530314455415386285079697_u128,133689982574687277702522338918171200532_u128];
_30 = [3898141891_u32,1629779478_u32,3327794820_u32,3527328013_u32,2757934742_u32,354104659_u32,4283144011_u32,4206754576_u32];
_15 = &_4;
_5 = _28;
(*_33) = _16 + _23;
(*_19) = 10466487377229645046_u64 as isize;
(*_33) = _16;
(*_19) = -_8;
_39.1 = &_32;
(*_33) = _16 * _34;
(*_33) = _23;
_39.3 = _26;
_42 = RET;
_21 = (*_19) < _8;
(*_33) = _23 - _16;
(*_19) = -_8;
Goto(bb14)
}
bb37 = {
_19 = core::ptr::addr_of_mut!(_8);
_8 = _9;
(*_19) = _13 as isize;
(*_19) = _18 | _18;
(*_19) = _18 | _9;
_15 = &_13;
(*_19) = _18 | _9;
(*_19) = _9;
_15 = &_3;
_13 = (*_15);
_1 = _2;
_2 = -_7;
(*_19) = _18 + _9;
_23 = (-53_i8) as f32;
(*_19) = (*_15) as isize;
(*_19) = _9 - _9;
_21 = false;
_17 = -_16;
(*_19) = _9 >> _22;
_3 = _4;
_26 = 29240_i16 | (-11431_i16);
_24 = _8 << (*_19);
_25 = _18 == _24;
Call((*_19) = core::intrinsics::transmute(_18), ReturnTo(bb6), UnwindUnreachable())
}
bb38 = {
_89 = _36 | _70;
_44 = _37;
_65 = core::ptr::addr_of!(place!(Field::<i32>(Variant(_29, 0), 1)));
_69.0 = (3110362151_u32,);
match _82.1.2 {
0 => bb39,
1 => bb40,
2 => bb42,
_ => bb41
}
}
bb39 = {
_19 = core::ptr::addr_of_mut!(_8);
_8 = _9;
(*_19) = _13 as isize;
(*_19) = _18 | _18;
(*_19) = _18 | _9;
_15 = &_13;
(*_19) = _18 | _9;
(*_19) = _9;
_15 = &_3;
_13 = (*_15);
_1 = _2;
_2 = -_7;
(*_19) = _18 + _9;
_23 = (-53_i8) as f32;
(*_19) = (*_15) as isize;
(*_19) = _9 - _9;
_21 = false;
_17 = -_16;
(*_19) = _9 >> _22;
_3 = _4;
_26 = 29240_i16 | (-11431_i16);
_24 = _8 << (*_19);
_25 = _18 == _24;
Call((*_19) = core::intrinsics::transmute(_18), ReturnTo(bb6), UnwindUnreachable())
}
bb40 = {
_34 = 16230673699079607090_u64 as f32;
(*_19) = _8 * _24;
(*_19) = _18 << _32;
(*_19) = _9 ^ _24;
(*_19) = _24 - _18;
_21 = (*_19) == (*_19);
_37 = _6;
_33 = core::ptr::addr_of!(_17);
_17 = _23 * _16;
(*_33) = _34 * _16;
(*_33) = _23 - _23;
_24 = (*_19) * (*_19);
_21 = (*_19) > (*_19);
Goto(bb11)
}
bb41 = {
(*_19) = _9 & _9;
RET = [217000334776842713943587939388663027592_u128,319937319566285002606934018823729271561_u128,115467777143490527802590376868482269316_u128,298100573503365631164441905613722448802_u128,216649904981288502063307126848573927176_u128,250721837586239809658610699625072952344_u128,66568455374329649366476005483792289419_u128];
_33 = core::ptr::addr_of!((*_33));
(*_19) = _24 + _24;
(*_33) = _34 * _34;
(*_19) = _24 * _24;
_23 = _26 as f32;
_15 = &_5;
_4 = (*_15);
_3 = (*_15);
(*_33) = _23 + _16;
_13 = (*_15);
(*_19) = _8 + _24;
(*_19) = _18 >> _18;
(*_19) = _24 << _24;
_8 = _24;
Goto(bb13)
}
bb42 = {
_1 = _7;
(*_19) = _22 | _47;
_69.0.0 = 2871229038_u32 + 2702718426_u32;
_68 = _69.3;
_53.1 = &_64.0;
_6 = _13;
_63.0 = &mut _13;
_27 = _68 | _21;
_94 = _82.1;
match _82.1.3 {
0 => bb17,
1 => bb30,
2 => bb43,
78 => bb45,
_ => bb44
}
}
bb43 = {
(*_19) = _47;
_69.1 = Move(_15);
_16 = -(*_33);
_61 = &_19;
_73 = _53.0 >= _26;
RET = _42;
_69.2 = (*_19) != (*_19);
_9 = _64.0 as isize;
_54 = 10257510912470716543_usize as f32;
_21 = (*_19) > (*_19);
_39.1 = &_64.0;
(*_19) = _18;
_39.3 = !_53.0;
_72.0 = [_2,_7];
_39.2 = [_69.0.0,_69.0.0,_69.0.0,_69.0.0,_69.0.0,_69.0.0,_69.0.0,_69.0.0];
_66 = [_69.0.0,_69.0.0,_69.0.0,_69.0.0,_69.0.0,_69.0.0,_69.0.0,_69.0.0];
_70 = (*_19);
place!(Field::<u32>(Variant(_29, 1), 5)) = _69.0.0 >> (*_19);
Goto(bb25)
}
bb44 = {
_34 = 16230673699079607090_u64 as f32;
(*_19) = _8 * _24;
(*_19) = _18 << _32;
(*_19) = _9 ^ _24;
(*_19) = _24 - _18;
_21 = (*_19) == (*_19);
_37 = _6;
_33 = core::ptr::addr_of!(_17);
_17 = _23 * _16;
(*_33) = _34 * _16;
(*_33) = _23 - _23;
_24 = (*_19) * (*_19);
_21 = (*_19) > (*_19);
Goto(bb11)
}
bb45 = {
_15 = &_37;
(*_33) = _82.1.0 * _82.1.0;
place!(Field::<u128>(Variant(_29, 0), 0)) = 247298790578738903264439687192910443433_u128 << _18;
_52.2 = core::ptr::addr_of!((*_33));
_66 = _30;
_53.1 = &_32;
(*_65) = _62;
_10 = &place!(Field::<u128>(Variant(_29, 0), 0));
_82.2.1 = (*_19) as i64;
_81 = -(*_33);
_72.2 = !_94.1;
(*_33) = _59 - _94.0;
(*_19) = _22;
_93 = !_94.2;
Goto(bb46)
}
bb46 = {
(*_33) = _94.0 - _82.1.0;
_82.1.3 = !_94.3;
(*_33) = _82.1.3 as f32;
(*_65) = _62;
match _94.2 {
0 => bb6,
1 => bb2,
3 => bb48,
4 => bb49,
2 => bb51,
_ => bb50
}
}
bb47 = {
_15 = &_37;
(*_33) = _82.1.0 * _82.1.0;
place!(Field::<u128>(Variant(_29, 0), 0)) = 247298790578738903264439687192910443433_u128 << _18;
_52.2 = core::ptr::addr_of!((*_33));
_66 = _30;
_53.1 = &_32;
(*_65) = _62;
_10 = &place!(Field::<u128>(Variant(_29, 0), 0));
_82.2.1 = (*_19) as i64;
_81 = -(*_33);
_72.2 = !_94.1;
(*_33) = _59 - _94.0;
(*_19) = _22;
_93 = !_94.2;
Goto(bb46)
}
bb48 = {
(*_19) = !_24;
_22 = (*_19);
_12 = Move(_20);
_69.0 = (Field::<u32>(Variant(_29, 1), 5),);
(*_19) = 10672_u16 as isize;
(*_33) = _17;
(*_19) = !_18;
_76 = _67 * (*_33);
_50 = _11 + Field::<f64>(Variant(_29, 1), 1);
_82.2.2 = !Field::<i16>(Variant(_29, 1), 4);
place!(Field::<usize>(Variant(_29, 1), 7)) = !12616144206474716470_usize;
_3 = _58;
_39.0 = !_53.3;
place!(Field::<isize>(Variant(_29, 1), 2)) = -(*_19);
_66 = [Field::<u32>(Variant(_29, 1), 5),Field::<u32>(Variant(_29, 1), 5),Field::<u32>(Variant(_29, 1), 5),_69.0.0,_69.0.0,Field::<u32>(Variant(_29, 1), 5),_69.0.0,Field::<u32>(Variant(_29, 1), 5)];
_82.1.1 = !_39.3;
Goto(bb30)
}
bb49 = {
(*_19) = _47;
_69.1 = Move(_15);
_16 = -(*_33);
_61 = &_19;
_73 = _53.0 >= _26;
RET = _42;
_69.2 = (*_19) != (*_19);
_9 = _64.0 as isize;
_54 = 10257510912470716543_usize as f32;
_21 = (*_19) > (*_19);
_39.1 = &_64.0;
(*_19) = _18;
_39.3 = !_53.0;
_72.0 = [_2,_7];
_39.2 = [_69.0.0,_69.0.0,_69.0.0,_69.0.0,_69.0.0,_69.0.0,_69.0.0,_69.0.0];
_66 = [_69.0.0,_69.0.0,_69.0.0,_69.0.0,_69.0.0,_69.0.0,_69.0.0,_69.0.0];
_70 = (*_19);
place!(Field::<u32>(Variant(_29, 1), 5)) = _69.0.0 >> (*_19);
Goto(bb25)
}
bb50 = {
_34 = 16230673699079607090_u64 as f32;
(*_19) = _8 * _24;
(*_19) = _18 << _32;
(*_19) = _9 ^ _24;
(*_19) = _24 - _18;
_21 = (*_19) == (*_19);
_37 = _6;
_33 = core::ptr::addr_of!(_17);
_17 = _23 * _16;
(*_33) = _34 * _16;
(*_33) = _23 - _23;
_24 = (*_19) * (*_19);
_21 = (*_19) > (*_19);
Goto(bb11)
}
bb51 = {
(*_19) = _89;
_52.0 = &_6;
_98 = &mut _49;
_57 = (*_19) * _40;
(*_98) = [2_i8,(-9_i8)];
(*_19) = _24 << (*_10);
_39.2 = [_69.0.0,_69.0.0,_69.0.0,_69.0.0,_69.0.0,_69.0.0,_69.0.0,_69.0.0];
(*_98) = [69_i8,127_i8];
(*_33) = -_17;
_96 = -(*_19);
_101 = Move(_63.0);
_69.2 = _73;
(*_33) = _23 * _81;
_95 = 6860497909099894547_u64 as f64;
(*_98) = [58_i8,71_i8];
(*_65) = _62 | _62;
(*_33) = _82.1.0 - _17;
_80 = [_73,_69.3];
(*_65) = _62 * _62;
_22 = 51_i8 as isize;
_82.1 = ((*_33), _53.0, _94.2, _94.3);
Goto(bb52)
}
bb52 = {
Call(_108 = dump_var(Move(_45), Move(_62), Move(_44), Move(_37)), ReturnTo(bb53), UnwindUnreachable())
}
bb53 = {
Call(_108 = dump_var(Move(_73), Move(_18), Move(_51), Move(_1)), ReturnTo(bb54), UnwindUnreachable())
}
bb54 = {
Call(_108 = dump_var(Move(_57), Move(_48), Move(_25), Move(_89)), ReturnTo(bb55), UnwindUnreachable())
}
bb55 = {
Call(_108 = dump_var(Move(_96), Move(_26), Move(_64), Move(_8)), ReturnTo(bb56), UnwindUnreachable())
}
bb56 = {
Call(_108 = dump_var(Move(_75), Move(_42), Move(_5), Move(_13)), ReturnTo(bb57), UnwindUnreachable())
}
bb57 = {
Call(_108 = dump_var(Move(_47), Move(_30), Move(_49), _109), ReturnTo(bb58), UnwindUnreachable())
}
bb58 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn17(mut _1: char,mut _2: &'static char,mut _3: char,mut _4: char,mut _5: char,mut _6: char,mut _7: isize,mut _8: char,mut _9: char,mut _10: char,mut _11: char,mut _12: char) -> *const Adt22 {
mir! {
type RET = *const Adt22;
let _13: (Adt22, &'static u32, (&'static &'static char, i64, i16, i64));
let _14: (u8, i64, &'static mut [bool; 2]);
let _15: [i128; 1];
let _16: bool;
let _17: &'static mut [bool; 2];
let _18: (&'static &'static char, i64, i16, i64);
let _19: u16;
let _20: isize;
let _21: isize;
let _22: &'static i16;
let _23: i32;
let _24: u64;
let _25: i8;
let _26: i8;
let _27: bool;
let _28: bool;
let _29: (u64, u128);
let _30: &'static mut (i64,);
let _31: i32;
let _32: u64;
let _33: isize;
let _34: i128;
let _35: (u32,);
let _36: u16;
let _37: &'static mut [i8; 2];
let _38: char;
let _39: f64;
let _40: ((i64,), &'static char);
let _41: f32;
let _42: u64;
let _43: *const i64;
let _44: u16;
let _45: &'static mut [i8; 2];
let _46: *const Adt22;
let _47: u64;
let _48: i128;
let _49: &'static i64;
let _50: isize;
let _51: (u64, u128);
let _52: f64;
let _53: [isize; 5];
let _54: i8;
let _55: f32;
let _56: *const (f32, i16, usize, u8);
let _57: i16;
let _58: *mut [u8; 4];
let _59: (u64, u128);
let _60: char;
let _61: ([i128; 2], &'static mut *mut i128, i16);
let _62: *mut &'static *mut isize;
let _63: u64;
let _64: char;
let _65: [char; 8];
let _66: *mut [u8; 4];
let _67: *const i128;
let _68: i64;
let _69: f32;
let _70: u8;
let _71: [u32; 8];
let _72: Adt22;
let _73: char;
let _74: isize;
let _75: &'static i16;
let _76: *const i64;
let _77: isize;
let _78: isize;
let _79: [i8; 2];
let _80: (&'static char, [u128; 7], *const f32, *const [i32; 5]);
let _81: (u32,);
let _82: [i128; 2];
let _83: *mut u16;
let _84: i16;
let _85: &'static u128;
let _86: &'static mut (i64,);
let _87: char;
let _88: ();
let _89: ();
{
_13.2.1 = (-20453_i16) as i64;
RET = core::ptr::addr_of!(_13.0);
(*RET).fld3 = -(-112_i8);
(*RET).fld1 = Adt21::Variant0 { fld0: 134335044551386871685982191320032500332_u128,fld1: (-776676052_i32),fld2: (*RET).fld3 };
(*RET).fld1 = Adt21::Variant0 { fld0: 290902677373786222956784951944473141662_u128,fld1: (-1741849955_i32),fld2: (*RET).fld3 };
place!(Field::<i32>(Variant((*RET).fld1, 0), 1)) = -(-2096187430_i32);
(*RET).fld2 = 99543390211832421_u64;
(*RET).fld1 = Adt21::Variant0 { fld0: 399050281280952262873411417789458874_u128,fld1: (-1910270421_i32),fld2: (*RET).fld3 };
place!(Field::<i32>(Variant((*RET).fld1, 0), 1)) = (*RET).fld2 as i32;
_6 = _3;
place!(Field::<i32>(Variant((*RET).fld1, 0), 1)) = (-1669022822_i32);
(*RET).fld0 = false;
place!(Field::<u128>(Variant((*RET).fld1, 0), 0)) = 58470617684408208722658529176200413749_u128 << Field::<i8>(Variant((*RET).fld1, 0), 2);
(*RET).fld1 = Adt21::Variant0 { fld0: 32538098589278816962418288593318747693_u128,fld1: (-1963150573_i32),fld2: (*RET).fld3 };
(*RET).fld3 = !Field::<i8>(Variant((*RET).fld1, 0), 2);
place!(Field::<i32>(Variant((*RET).fld1, 0), 1)) = 1734390747_i32 - 1601142009_i32;
place!(Field::<i32>(Variant((*RET).fld1, 0), 1)) = 928113641_i32 | (-1833811146_i32);
match (*RET).fld2 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
99543390211832421 => bb7,
_ => bb6
}
}
bb1 = {
Return()
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
_13.2.3 = _13.2.1;
place!(Field::<u128>(Variant((*RET).fld1, 0), 0)) = 108270591958275496605654191170936726376_u128 | 116070340231268203090381616348086364526_u128;
(*RET).fld0 = false;
(*RET).fld3 = Field::<i8>(Variant((*RET).fld1, 0), 2) - Field::<i8>(Variant((*RET).fld1, 0), 2);
_1 = _8;
(*RET).fld3 = Field::<i8>(Variant((*RET).fld1, 0), 2) & Field::<i8>(Variant((*RET).fld1, 0), 2);
(*RET).fld0 = !false;
place!(Field::<i8>(Variant((*RET).fld1, 0), 2)) = Field::<u128>(Variant((*RET).fld1, 0), 0) as i8;
place!(Field::<i8>(Variant((*RET).fld1, 0), 2)) = (*RET).fld3 & (*RET).fld3;
(*RET).fld2 = (-20658_i16) as u64;
_18.3 = _13.2.3 << Field::<i8>(Variant((*RET).fld1, 0), 2);
place!(Field::<i32>(Variant((*RET).fld1, 0), 1)) = (-792165248_i32);
(*RET).fld3 = Field::<i8>(Variant((*RET).fld1, 0), 2);
(*RET).fld0 = !false;
(*RET).fld3 = Field::<i8>(Variant((*RET).fld1, 0), 2) & Field::<i8>(Variant((*RET).fld1, 0), 2);
(*RET).fld2 = 7053705115727876864_u64;
(*RET).fld2 = 385330299281341421_u64;
_13.2.2 = _18.3 as i16;
_18.1 = _18.3;
_13.0.fld0 = !true;
(*RET).fld2 = 10857551646251833795_u64;
Goto(bb8)
}
bb8 = {
(*RET).fld1 = Adt21::Variant0 { fld0: 258799839421470421220838852280313703558_u128,fld1: 1248568892_i32,fld2: (*RET).fld3 };
_13.0.fld0 = true ^ true;
place!(Field::<u128>(Variant((*RET).fld1, 0), 0)) = 44437305060303120578935944499273415483_u128 - 127811912491751112263642208986804936300_u128;
(*RET).fld2 = !11848056895722409612_u64;
_16 = !(*RET).fld0;
(*RET).fld1 = Adt21::Variant0 { fld0: 60478696490262276215829455195067226264_u128,fld1: (-1772063314_i32),fld2: (*RET).fld3 };
place!(Field::<i32>(Variant((*RET).fld1, 0), 1)) = 1760754296_i32;
place!(Field::<u128>(Variant(_13.0.fld1, 0), 0)) = 208859606292848246239464186621527893534_u128;
(*RET).fld3 = Field::<i8>(Variant((*RET).fld1, 0), 2);
place!(Field::<i8>(Variant((*RET).fld1, 0), 2)) = _13.0.fld3 << (*RET).fld3;
(*RET).fld0 = _16;
place!(Field::<u128>(Variant((*RET).fld1, 0), 0)) = 289111788037304967635705855407489859826_u128;
(*RET).fld1 = Adt21::Variant0 { fld0: 65951965391613625615359685553902904535_u128,fld1: 1095106488_i32,fld2: (*RET).fld3 };
place!(Field::<i32>(Variant((*RET).fld1, 0), 1)) = -(-286033530_i32);
_19 = !32486_u16;
place!(Field::<u128>(Variant((*RET).fld1, 0), 0)) = !216111820627490647126518311284705807306_u128;
(*RET).fld3 = !Field::<i8>(Variant((*RET).fld1, 0), 2);
_8 = _11;
place!(Field::<i8>(Variant((*RET).fld1, 0), 2)) = _13.0.fld3 - (*RET).fld3;
(*RET).fld0 = !_16;
(*RET).fld3 = -Field::<i8>(Variant((*RET).fld1, 0), 2);
place!(Field::<u128>(Variant((*RET).fld1, 0), 0)) = 301984280891116663610536651100772492755_u128 << Field::<i8>(Variant((*RET).fld1, 0), 2);
Goto(bb9)
}
bb9 = {
_14.0 = 195_u8;
match _14.0 {
0 => bb7,
1 => bb2,
2 => bb10,
3 => bb11,
4 => bb12,
5 => bb13,
6 => bb14,
195 => bb16,
_ => bb15
}
}
bb10 = {
(*RET).fld1 = Adt21::Variant0 { fld0: 258799839421470421220838852280313703558_u128,fld1: 1248568892_i32,fld2: (*RET).fld3 };
_13.0.fld0 = true ^ true;
place!(Field::<u128>(Variant((*RET).fld1, 0), 0)) = 44437305060303120578935944499273415483_u128 - 127811912491751112263642208986804936300_u128;
(*RET).fld2 = !11848056895722409612_u64;
_16 = !(*RET).fld0;
(*RET).fld1 = Adt21::Variant0 { fld0: 60478696490262276215829455195067226264_u128,fld1: (-1772063314_i32),fld2: (*RET).fld3 };
place!(Field::<i32>(Variant((*RET).fld1, 0), 1)) = 1760754296_i32;
place!(Field::<u128>(Variant(_13.0.fld1, 0), 0)) = 208859606292848246239464186621527893534_u128;
(*RET).fld3 = Field::<i8>(Variant((*RET).fld1, 0), 2);
place!(Field::<i8>(Variant((*RET).fld1, 0), 2)) = _13.0.fld3 << (*RET).fld3;
(*RET).fld0 = _16;
place!(Field::<u128>(Variant((*RET).fld1, 0), 0)) = 289111788037304967635705855407489859826_u128;
(*RET).fld1 = Adt21::Variant0 { fld0: 65951965391613625615359685553902904535_u128,fld1: 1095106488_i32,fld2: (*RET).fld3 };
place!(Field::<i32>(Variant((*RET).fld1, 0), 1)) = -(-286033530_i32);
_19 = !32486_u16;
place!(Field::<u128>(Variant((*RET).fld1, 0), 0)) = !216111820627490647126518311284705807306_u128;
(*RET).fld3 = !Field::<i8>(Variant((*RET).fld1, 0), 2);
_8 = _11;
place!(Field::<i8>(Variant((*RET).fld1, 0), 2)) = _13.0.fld3 - (*RET).fld3;
(*RET).fld0 = !_16;
(*RET).fld3 = -Field::<i8>(Variant((*RET).fld1, 0), 2);
place!(Field::<u128>(Variant((*RET).fld1, 0), 0)) = 301984280891116663610536651100772492755_u128 << Field::<i8>(Variant((*RET).fld1, 0), 2);
Goto(bb9)
}
bb11 = {
_13.2.3 = _13.2.1;
place!(Field::<u128>(Variant((*RET).fld1, 0), 0)) = 108270591958275496605654191170936726376_u128 | 116070340231268203090381616348086364526_u128;
(*RET).fld0 = false;
(*RET).fld3 = Field::<i8>(Variant((*RET).fld1, 0), 2) - Field::<i8>(Variant((*RET).fld1, 0), 2);
_1 = _8;
(*RET).fld3 = Field::<i8>(Variant((*RET).fld1, 0), 2) & Field::<i8>(Variant((*RET).fld1, 0), 2);
(*RET).fld0 = !false;
place!(Field::<i8>(Variant((*RET).fld1, 0), 2)) = Field::<u128>(Variant((*RET).fld1, 0), 0) as i8;
place!(Field::<i8>(Variant((*RET).fld1, 0), 2)) = (*RET).fld3 & (*RET).fld3;
(*RET).fld2 = (-20658_i16) as u64;
_18.3 = _13.2.3 << Field::<i8>(Variant((*RET).fld1, 0), 2);
place!(Field::<i32>(Variant((*RET).fld1, 0), 1)) = (-792165248_i32);
(*RET).fld3 = Field::<i8>(Variant((*RET).fld1, 0), 2);
(*RET).fld0 = !false;
(*RET).fld3 = Field::<i8>(Variant((*RET).fld1, 0), 2) & Field::<i8>(Variant((*RET).fld1, 0), 2);
(*RET).fld2 = 7053705115727876864_u64;
(*RET).fld2 = 385330299281341421_u64;
_13.2.2 = _18.3 as i16;
_18.1 = _18.3;
_13.0.fld0 = !true;
(*RET).fld2 = 10857551646251833795_u64;
Goto(bb8)
}
bb12 = {
Return()
}
bb13 = {
Return()
}
bb14 = {
Return()
}
bb15 = {
Return()
}
bb16 = {
_13.0.fld2 = 16406628404407003588_u64;
(*RET).fld1 = Adt21::Variant0 { fld0: 165356010266829951782627315769537680921_u128,fld1: (-390615217_i32),fld2: (*RET).fld3 };
(*RET).fld0 = _16 | _16;
place!(Field::<u128>(Variant((*RET).fld1, 0), 0)) = 224476307585052551059998990751411989844_u128 | 281181089542680420970381592859496252329_u128;
(*RET).fld0 = _16 | _16;
place!(Field::<i8>(Variant((*RET).fld1, 0), 2)) = (*RET).fld3 | (*RET).fld3;
place!(Field::<i32>(Variant((*RET).fld1, 0), 1)) = !(-276884285_i32);
place!(Field::<i32>(Variant((*RET).fld1, 0), 1)) = 1335522670_i32 - 2112143181_i32;
place!(Field::<i32>(Variant((*RET).fld1, 0), 1)) = !(-478985555_i32);
(*RET).fld2 = !17470022569989606789_u64;
(*RET).fld2 = 6166646519458135392_u64;
place!(Field::<i8>(Variant((*RET).fld1, 0), 2)) = (*RET).fld3 & (*RET).fld3;
(*RET).fld2 = !4502750156770998859_u64;
Goto(bb17)
}
bb17 = {
place!(Field::<i32>(Variant(_13.0.fld1, 0), 1)) = (-1792447608_i32);
(*RET).fld3 = (*RET).fld2 as i8;
(*RET).fld3 = -Field::<i8>(Variant((*RET).fld1, 0), 2);
(*RET).fld1 = Adt21::Variant0 { fld0: 94535286577081734824241570137803329024_u128,fld1: 1880065236_i32,fld2: (*RET).fld3 };
place!(Field::<i8>(Variant((*RET).fld1, 0), 2)) = (*RET).fld3 | (*RET).fld3;
_20 = _7 | _7;
(*RET).fld3 = Field::<i8>(Variant((*RET).fld1, 0), 2);
place!(Field::<i32>(Variant((*RET).fld1, 0), 1)) = 752502629_i32;
_21 = _20 << Field::<i8>(Variant((*RET).fld1, 0), 2);
_1 = _6;
place!(Field::<i32>(Variant((*RET).fld1, 0), 1)) = 187910916_i32 | (-189740058_i32);
_7 = _14.0 as isize;
place!(Field::<u128>(Variant((*RET).fld1, 0), 0)) = 105000791656817546387157293648709254562_u128 | 339410805688609812394309366782431552328_u128;
_3 = _1;
(*RET).fld2 = !4860519979336303305_u64;
place!(Field::<i8>(Variant((*RET).fld1, 0), 2)) = (*RET).fld3 + _13.0.fld3;
place!(Field::<i32>(Variant((*RET).fld1, 0), 1)) = Field::<i8>(Variant((*RET).fld1, 0), 2) as i32;
(*RET).fld3 = Field::<i8>(Variant((*RET).fld1, 0), 2) >> Field::<i32>(Variant((*RET).fld1, 0), 1);
(*RET).fld0 = _16 | _16;
place!(Field::<i32>(Variant((*RET).fld1, 0), 1)) = 455900062_i32;
(*RET).fld2 = 15540858416304688095_u64 << Field::<i8>(Variant(_13.0.fld1, 0), 2);
(*RET).fld3 = 60765321_u32 as i8;
_6 = _3;
(*RET).fld0 = !_16;
(*RET).fld0 = (*RET).fld2 >= (*RET).fld2;
Goto(bb18)
}
bb18 = {
place!(Field::<i32>(Variant((*RET).fld1, 0), 1)) = 1479205158_i32 & (-210344012_i32);
_18.2 = _13.2.2;
place!(Field::<i8>(Variant((*RET).fld1, 0), 2)) = (*RET).fld3 - (*RET).fld3;
place!(Field::<i32>(Variant((*RET).fld1, 0), 1)) = 1535233434_i32;
place!(Field::<u128>(Variant(_13.0.fld1, 0), 0)) = !279174521422603504851517315741933752755_u128;
place!(Field::<u128>(Variant((*RET).fld1, 0), 0)) = 313267841392737467279369924542839884219_u128;
place!(Field::<i8>(Variant((*RET).fld1, 0), 2)) = (-161276619404672494695587756945288327168_i128) as i8;
(*RET).fld2 = 2906393590566949466_u64 * 15392388053105213770_u64;
(*RET).fld2 = 5_usize as u64;
(*RET).fld0 = !_16;
place!(Field::<i32>(Variant((*RET).fld1, 0), 1)) = (-1444934657_i32) << _21;
place!(Field::<i32>(Variant((*RET).fld1, 0), 1)) = (-541444186_i32);
place!(Field::<i8>(Variant((*RET).fld1, 0), 2)) = _13.0.fld3;
_29 = ((*RET).fld2, Field::<u128>(Variant((*RET).fld1, 0), 0));
(*RET).fld0 = !_16;
place!(Field::<u128>(Variant((*RET).fld1, 0), 0)) = _29.1 / _29.1;
match Field::<i32>(Variant((*RET).fld1, 0), 1) {
0 => bb12,
1 => bb6,
2 => bb19,
3 => bb20,
4 => bb21,
340282366920938463463374607431226767270 => bb23,
_ => bb22
}
}
bb19 = {
place!(Field::<i32>(Variant(_13.0.fld1, 0), 1)) = (-1792447608_i32);
(*RET).fld3 = (*RET).fld2 as i8;
(*RET).fld3 = -Field::<i8>(Variant((*RET).fld1, 0), 2);
(*RET).fld1 = Adt21::Variant0 { fld0: 94535286577081734824241570137803329024_u128,fld1: 1880065236_i32,fld2: (*RET).fld3 };
place!(Field::<i8>(Variant((*RET).fld1, 0), 2)) = (*RET).fld3 | (*RET).fld3;
_20 = _7 | _7;
(*RET).fld3 = Field::<i8>(Variant((*RET).fld1, 0), 2);
place!(Field::<i32>(Variant((*RET).fld1, 0), 1)) = 752502629_i32;
_21 = _20 << Field::<i8>(Variant((*RET).fld1, 0), 2);
_1 = _6;
place!(Field::<i32>(Variant((*RET).fld1, 0), 1)) = 187910916_i32 | (-189740058_i32);
_7 = _14.0 as isize;
place!(Field::<u128>(Variant((*RET).fld1, 0), 0)) = 105000791656817546387157293648709254562_u128 | 339410805688609812394309366782431552328_u128;
_3 = _1;
(*RET).fld2 = !4860519979336303305_u64;
place!(Field::<i8>(Variant((*RET).fld1, 0), 2)) = (*RET).fld3 + _13.0.fld3;
place!(Field::<i32>(Variant((*RET).fld1, 0), 1)) = Field::<i8>(Variant((*RET).fld1, 0), 2) as i32;
(*RET).fld3 = Field::<i8>(Variant((*RET).fld1, 0), 2) >> Field::<i32>(Variant((*RET).fld1, 0), 1);
(*RET).fld0 = _16 | _16;
place!(Field::<i32>(Variant((*RET).fld1, 0), 1)) = 455900062_i32;
(*RET).fld2 = 15540858416304688095_u64 << Field::<i8>(Variant(_13.0.fld1, 0), 2);
(*RET).fld3 = 60765321_u32 as i8;
_6 = _3;
(*RET).fld0 = !_16;
(*RET).fld0 = (*RET).fld2 >= (*RET).fld2;
Goto(bb18)
}
bb20 = {
Return()
}
bb21 = {
Return()
}
bb22 = {
Return()
}
bb23 = {
place!(Field::<u128>(Variant((*RET).fld1, 0), 0)) = !_29.1;
(*RET).fld0 = _16;
place!(Field::<i8>(Variant((*RET).fld1, 0), 2)) = Field::<u128>(Variant((*RET).fld1, 0), 0) as i8;
(*RET).fld2 = _29.0 * _29.0;
(*RET).fld2 = !_29.0;
(*RET).fld1 = Adt21::Variant0 { fld0: _29.1,fld1: (-1830480336_i32),fld2: _13.0.fld3 };
_18.2 = _13.2.2 & _13.2.2;
place!(Field::<i8>(Variant((*RET).fld1, 0), 2)) = (*RET).fld3 - (*RET).fld3;
place!(Field::<i32>(Variant((*RET).fld1, 0), 1)) = (-620054293_i32);
place!(Field::<i8>(Variant((*RET).fld1, 0), 2)) = Field::<u128>(Variant(_13.0.fld1, 0), 0) as i8;
(*RET).fld3 = Field::<i8>(Variant((*RET).fld1, 0), 2) >> _19;
(*RET).fld0 = (*RET).fld3 >= Field::<i8>(Variant((*RET).fld1, 0), 2);
(*RET).fld0 = !_16;
Goto(bb24)
}
bb24 = {
_25 = _21 as i8;
_20 = _21 | _7;
_24 = (*RET).fld2 - (*RET).fld2;
place!(Field::<i32>(Variant((*RET).fld1, 0), 1)) = (-1041699428_i32) & 87199628_i32;
place!(Field::<i32>(Variant((*RET).fld1, 0), 1)) = (-1590656898_i32) - 1742901999_i32;
place!(Field::<i32>(Variant((*RET).fld1, 0), 1)) = (-423879071_i32) + (-225701554_i32);
place!(Field::<i8>(Variant((*RET).fld1, 0), 2)) = -_25;
place!(Field::<i8>(Variant((*RET).fld1, 0), 2)) = -(*RET).fld3;
place!(Field::<i32>(Variant((*RET).fld1, 0), 1)) = (-510937097_i32) ^ 528196125_i32;
(*RET).fld2 = 12764225223319618059_usize as u64;
place!(Field::<i32>(Variant((*RET).fld1, 0), 1)) = _21 as i32;
place!(Field::<i32>(Variant((*RET).fld1, 0), 1)) = 1820893147_i32 * (-588371878_i32);
place!(Field::<i32>(Variant((*RET).fld1, 0), 1)) = (-45887805285887964116061237649279599002_i128) as i32;
place!(Field::<i32>(Variant((*RET).fld1, 0), 1)) = !440098635_i32;
place!(Field::<u128>(Variant((*RET).fld1, 0), 0)) = !_29.1;
_3 = _9;
place!(Field::<i8>(Variant((*RET).fld1, 0), 2)) = (*RET).fld0 as i8;
(*RET).fld3 = !_25;
_35.0 = 1703369854_u32 >> (*RET).fld3;
place!(Field::<u128>(Variant((*RET).fld1, 0), 0)) = _29.1 % _29.1;
(*RET).fld0 = !_16;
place!(Field::<u128>(Variant((*RET).fld1, 0), 0)) = _29.1 & _29.1;
Goto(bb25)
}
bb25 = {
_18.3 = _18.1;
place!(Field::<i8>(Variant((*RET).fld1, 0), 2)) = (*RET).fld3 << (*RET).fld3;
(*RET).fld0 = _16 & _16;
(*RET).fld2 = !_24;
_40.1 = &_4;
(*RET).fld3 = Field::<i8>(Variant((*RET).fld1, 0), 2) << Field::<i8>(Variant((*RET).fld1, 0), 2);
place!(Field::<i32>(Variant((*RET).fld1, 0), 1)) = 88228530_i32;
_34 = (-18983391075538772674496276525494725636_i128) + (-6638513651617501363690527735517906703_i128);
_13.0.fld1 = Adt21::Variant0 { fld0: _29.1,fld1: (-1985986513_i32),fld2: (*RET).fld3 };
(*RET).fld3 = -Field::<i8>(Variant((*RET).fld1, 0), 2);
_18.3 = _13.2.1 & _18.1;
_13.2.0 = &_40.1;
_36 = !_19;
place!(Field::<i32>(Variant((*RET).fld1, 0), 1)) = _19 as i32;
place!(Field::<u128>(Variant((*RET).fld1, 0), 0)) = !_29.1;
(*RET).fld3 = _14.0 as i8;
place!(Field::<i32>(Variant((*RET).fld1, 0), 1)) = _34 as i32;
(*RET).fld3 = Field::<i8>(Variant((*RET).fld1, 0), 2);
place!(Field::<u128>(Variant((*RET).fld1, 0), 0)) = _34 as u128;
place!(Field::<i8>(Variant((*RET).fld1, 0), 2)) = !(*RET).fld3;
_22 = &_13.2.2;
place!(Field::<i32>(Variant((*RET).fld1, 0), 1)) = (*RET).fld2 as i32;
Call(_40.0.0 = fn18(Move(_22), Move((*RET)), Move(RET), Move(_13.2.0), _7), ReturnTo(bb26), UnwindUnreachable())
}
bb26 = {
_13.0.fld3 = _25 + _25;
_13.0.fld2 = _24;
_32 = _24 << _29.1;
_34 = 95218358622699170955526591255060690007_i128 | (-56053730790627837572367208943927204262_i128);
_13.0.fld0 = _13.0.fld3 <= _25;
_6 = _3;
_2 = &_12;
_46 = core::ptr::addr_of!(_13.0);
(*_46).fld3 = _25 + _25;
(*_46).fld3 = -_25;
(*_46).fld2 = !_24;
(*_46).fld3 = -_25;
_14.1 = _13.2.1 + _18.3;
(*_46).fld2 = _24;
(*_46).fld1 = Adt21::Variant0 { fld0: _29.1,fld1: (-967244089_i32),fld2: _25 };
_1 = (*_2);
Goto(bb27)
}
bb27 = {
_41 = _35.0 as f32;
place!(Field::<u128>(Variant((*_46).fld1, 0), 0)) = !_29.1;
place!(Field::<u128>(Variant((*_46).fld1, 0), 0)) = _29.1;
place!(Field::<u128>(Variant((*_46).fld1, 0), 0)) = _29.1;
place!(Field::<u128>(Variant((*_46).fld1, 0), 0)) = !_29.1;
_54 = (*_46).fld3;
(*_46).fld3 = Field::<u128>(Variant((*_46).fld1, 0), 0) as i8;
(*_46).fld0 = Field::<i8>(Variant(_13.0.fld1, 0), 2) <= Field::<i8>(Variant(_13.0.fld1, 0), 2);
(*_46).fld2 = _24 + _32;
place!(Field::<i8>(Variant((*_46).fld1, 0), 2)) = _54 * (*_46).fld3;
_32 = (*_46).fld2 >> Field::<i8>(Variant((*_46).fld1, 0), 2);
place!(Field::<i32>(Variant((*_46).fld1, 0), 1)) = (-957364022_i32);
_18.0 = &_40.1;
(*_46).fld3 = Field::<i8>(Variant((*_46).fld1, 0), 2) + Field::<i8>(Variant((*_46).fld1, 0), 2);
(*_46).fld3 = !Field::<i8>(Variant((*_46).fld1, 0), 2);
(*_46).fld2 = (*_46).fld0 as u64;
_38 = _3;
_51.0 = (*_46).fld2 << Field::<i8>(Variant((*_46).fld1, 0), 2);
(*_46).fld1 = Adt21::Variant0 { fld0: _29.1,fld1: (-1684315342_i32),fld2: (*_46).fld3 };
_40.0.0 = !_18.3;
_19 = _35.0 as u16;
place!(Field::<u128>(Variant((*_46).fld1, 0), 0)) = !_29.1;
_59 = ((*_46).fld2, Field::<u128>(Variant((*_46).fld1, 0), 0));
Goto(bb28)
}
bb28 = {
(*_46).fld2 = _51.0 + _59.0;
_57 = -_13.2.2;
place!(Field::<i32>(Variant((*_46).fld1, 0), 1)) = 1847504829_i32;
_13.0.fld0 = !_16;
_13.2.0 = &_2;
(*_46).fld1 = Adt21::Variant0 { fld0: _59.1,fld1: (-1904952185_i32),fld2: (*_46).fld3 };
place!(Field::<i32>(Variant((*_46).fld1, 0), 1)) = 610125648_i32;
(*_46).fld2 = _51.0 + _32;
_15 = [_34];
_22 = &_57;
_33 = _20 | _20;
_22 = &_18.2;
_68 = (*_22) as i64;
place!(Field::<i32>(Variant((*_46).fld1, 0), 1)) = (-1407071721_i32) << (*_46).fld2;
_51.0 = (*_46).fld2 - (*_46).fld2;
(*_46).fld3 = Field::<i8>(Variant(_13.0.fld1, 0), 2) * Field::<i8>(Variant((*_46).fld1, 0), 2);
_27 = (*_46).fld0 ^ (*_46).fld0;
(*_46).fld2 = _51.0 & _59.0;
place!(Field::<i8>(Variant((*_46).fld1, 0), 2)) = (*_46).fld3 - _13.0.fld3;
_26 = !Field::<i8>(Variant((*_46).fld1, 0), 2);
_13.0.fld0 = _16;
_50 = _33 & _33;
match _14.0 {
0 => bb21,
1 => bb4,
2 => bb27,
195 => bb30,
_ => bb29
}
}
bb29 = {
place!(Field::<i32>(Variant(_13.0.fld1, 0), 1)) = (-1792447608_i32);
(*RET).fld3 = (*RET).fld2 as i8;
(*RET).fld3 = -Field::<i8>(Variant((*RET).fld1, 0), 2);
(*RET).fld1 = Adt21::Variant0 { fld0: 94535286577081734824241570137803329024_u128,fld1: 1880065236_i32,fld2: (*RET).fld3 };
place!(Field::<i8>(Variant((*RET).fld1, 0), 2)) = (*RET).fld3 | (*RET).fld3;
_20 = _7 | _7;
(*RET).fld3 = Field::<i8>(Variant((*RET).fld1, 0), 2);
place!(Field::<i32>(Variant((*RET).fld1, 0), 1)) = 752502629_i32;
_21 = _20 << Field::<i8>(Variant((*RET).fld1, 0), 2);
_1 = _6;
place!(Field::<i32>(Variant((*RET).fld1, 0), 1)) = 187910916_i32 | (-189740058_i32);
_7 = _14.0 as isize;
place!(Field::<u128>(Variant((*RET).fld1, 0), 0)) = 105000791656817546387157293648709254562_u128 | 339410805688609812394309366782431552328_u128;
_3 = _1;
(*RET).fld2 = !4860519979336303305_u64;
place!(Field::<i8>(Variant((*RET).fld1, 0), 2)) = (*RET).fld3 + _13.0.fld3;
place!(Field::<i32>(Variant((*RET).fld1, 0), 1)) = Field::<i8>(Variant((*RET).fld1, 0), 2) as i32;
(*RET).fld3 = Field::<i8>(Variant((*RET).fld1, 0), 2) >> Field::<i32>(Variant((*RET).fld1, 0), 1);
(*RET).fld0 = _16 | _16;
place!(Field::<i32>(Variant((*RET).fld1, 0), 1)) = 455900062_i32;
(*RET).fld2 = 15540858416304688095_u64 << Field::<i8>(Variant(_13.0.fld1, 0), 2);
(*RET).fld3 = 60765321_u32 as i8;
_6 = _3;
(*RET).fld0 = !_16;
(*RET).fld0 = (*RET).fld2 >= (*RET).fld2;
Goto(bb18)
}
bb30 = {
(*_46).fld3 = Field::<i8>(Variant((*_46).fld1, 0), 2) + Field::<i8>(Variant((*_46).fld1, 0), 2);
place!(Field::<i8>(Variant((*_46).fld1, 0), 2)) = (*_46).fld3 | (*_46).fld3;
place!(Field::<i8>(Variant((*_46).fld1, 0), 2)) = (*_46).fld3 | _13.0.fld3;
(*_46).fld1 = Adt21::Variant0 { fld0: _59.1,fld1: 1771517925_i32,fld2: _26 };
_51.0 = _59.0 << Field::<i8>(Variant((*_46).fld1, 0), 2);
place!(Field::<i32>(Variant((*_46).fld1, 0), 1)) = _41 as i32;
place!(Field::<i8>(Variant((*_46).fld1, 0), 2)) = -(*_46).fld3;
place!(Field::<i32>(Variant((*_46).fld1, 0), 1)) = (-1007345368_i32) >> Field::<i8>(Variant((*_46).fld1, 0), 2);
(*_46).fld2 = _59.0 << _33;
_59 = ((*_46).fld2, Field::<u128>(Variant((*_46).fld1, 0), 0));
_12 = _6;
(*_46).fld0 = Field::<i8>(Variant((*_46).fld1, 0), 2) <= Field::<i8>(Variant((*_46).fld1, 0), 2);
_48 = _34 * _34;
place!(Field::<i32>(Variant((*_46).fld1, 0), 1)) = 265430482_i32 >> (*_46).fld3;
_49 = &_40.0.0;
(*_46).fld2 = !_51.0;
place!(Field::<i8>(Variant((*_46).fld1, 0), 2)) = -(*_46).fld3;
place!(Field::<u128>(Variant((*_46).fld1, 0), 0)) = _14.0 as u128;
_4 = _5;
(*_46).fld3 = _25 + Field::<i8>(Variant((*_46).fld1, 0), 2);
_55 = _41 * _41;
(*_46).fld2 = !_59.0;
match _29.1 {
0 => bb11,
1 => bb4,
2 => bb31,
3 => bb32,
4 => bb33,
5 => bb34,
313267841392737467279369924542839884219 => bb36,
_ => bb35
}
}
bb31 = {
(*RET).fld1 = Adt21::Variant0 { fld0: 258799839421470421220838852280313703558_u128,fld1: 1248568892_i32,fld2: (*RET).fld3 };
_13.0.fld0 = true ^ true;
place!(Field::<u128>(Variant((*RET).fld1, 0), 0)) = 44437305060303120578935944499273415483_u128 - 127811912491751112263642208986804936300_u128;
(*RET).fld2 = !11848056895722409612_u64;
_16 = !(*RET).fld0;
(*RET).fld1 = Adt21::Variant0 { fld0: 60478696490262276215829455195067226264_u128,fld1: (-1772063314_i32),fld2: (*RET).fld3 };
place!(Field::<i32>(Variant((*RET).fld1, 0), 1)) = 1760754296_i32;
place!(Field::<u128>(Variant(_13.0.fld1, 0), 0)) = 208859606292848246239464186621527893534_u128;
(*RET).fld3 = Field::<i8>(Variant((*RET).fld1, 0), 2);
place!(Field::<i8>(Variant((*RET).fld1, 0), 2)) = _13.0.fld3 << (*RET).fld3;
(*RET).fld0 = _16;
place!(Field::<u128>(Variant((*RET).fld1, 0), 0)) = 289111788037304967635705855407489859826_u128;
(*RET).fld1 = Adt21::Variant0 { fld0: 65951965391613625615359685553902904535_u128,fld1: 1095106488_i32,fld2: (*RET).fld3 };
place!(Field::<i32>(Variant((*RET).fld1, 0), 1)) = -(-286033530_i32);
_19 = !32486_u16;
place!(Field::<u128>(Variant((*RET).fld1, 0), 0)) = !216111820627490647126518311284705807306_u128;
(*RET).fld3 = !Field::<i8>(Variant((*RET).fld1, 0), 2);
_8 = _11;
place!(Field::<i8>(Variant((*RET).fld1, 0), 2)) = _13.0.fld3 - (*RET).fld3;
(*RET).fld0 = !_16;
(*RET).fld3 = -Field::<i8>(Variant((*RET).fld1, 0), 2);
place!(Field::<u128>(Variant((*RET).fld1, 0), 0)) = 301984280891116663610536651100772492755_u128 << Field::<i8>(Variant((*RET).fld1, 0), 2);
Goto(bb9)
}
bb32 = {
(*_46).fld2 = _51.0 + _59.0;
_57 = -_13.2.2;
place!(Field::<i32>(Variant((*_46).fld1, 0), 1)) = 1847504829_i32;
_13.0.fld0 = !_16;
_13.2.0 = &_2;
(*_46).fld1 = Adt21::Variant0 { fld0: _59.1,fld1: (-1904952185_i32),fld2: (*_46).fld3 };
place!(Field::<i32>(Variant((*_46).fld1, 0), 1)) = 610125648_i32;
(*_46).fld2 = _51.0 + _32;
_15 = [_34];
_22 = &_57;
_33 = _20 | _20;
_22 = &_18.2;
_68 = (*_22) as i64;
place!(Field::<i32>(Variant((*_46).fld1, 0), 1)) = (-1407071721_i32) << (*_46).fld2;
_51.0 = (*_46).fld2 - (*_46).fld2;
(*_46).fld3 = Field::<i8>(Variant(_13.0.fld1, 0), 2) * Field::<i8>(Variant((*_46).fld1, 0), 2);
_27 = (*_46).fld0 ^ (*_46).fld0;
(*_46).fld2 = _51.0 & _59.0;
place!(Field::<i8>(Variant((*_46).fld1, 0), 2)) = (*_46).fld3 - _13.0.fld3;
_26 = !Field::<i8>(Variant((*_46).fld1, 0), 2);
_13.0.fld0 = _16;
_50 = _33 & _33;
match _14.0 {
0 => bb21,
1 => bb4,
2 => bb27,
195 => bb30,
_ => bb29
}
}
bb33 = {
Return()
}
bb34 = {
place!(Field::<i32>(Variant((*RET).fld1, 0), 1)) = 1479205158_i32 & (-210344012_i32);
_18.2 = _13.2.2;
place!(Field::<i8>(Variant((*RET).fld1, 0), 2)) = (*RET).fld3 - (*RET).fld3;
place!(Field::<i32>(Variant((*RET).fld1, 0), 1)) = 1535233434_i32;
place!(Field::<u128>(Variant(_13.0.fld1, 0), 0)) = !279174521422603504851517315741933752755_u128;
place!(Field::<u128>(Variant((*RET).fld1, 0), 0)) = 313267841392737467279369924542839884219_u128;
place!(Field::<i8>(Variant((*RET).fld1, 0), 2)) = (-161276619404672494695587756945288327168_i128) as i8;
(*RET).fld2 = 2906393590566949466_u64 * 15392388053105213770_u64;
(*RET).fld2 = 5_usize as u64;
(*RET).fld0 = !_16;
place!(Field::<i32>(Variant((*RET).fld1, 0), 1)) = (-1444934657_i32) << _21;
place!(Field::<i32>(Variant((*RET).fld1, 0), 1)) = (-541444186_i32);
place!(Field::<i8>(Variant((*RET).fld1, 0), 2)) = _13.0.fld3;
_29 = ((*RET).fld2, Field::<u128>(Variant((*RET).fld1, 0), 0));
(*RET).fld0 = !_16;
place!(Field::<u128>(Variant((*RET).fld1, 0), 0)) = _29.1 / _29.1;
match Field::<i32>(Variant((*RET).fld1, 0), 1) {
0 => bb12,
1 => bb6,
2 => bb19,
3 => bb20,
4 => bb21,
340282366920938463463374607431226767270 => bb23,
_ => bb22
}
}
bb35 = {
Return()
}
bb36 = {
Call(place!(Field::<i32>(Variant((*_46).fld1, 0), 1)) = core::intrinsics::transmute(_10), ReturnTo(bb37), UnwindUnreachable())
}
bb37 = {
place!(Field::<i32>(Variant((*_46).fld1, 0), 1)) = 1347971976_i32 << _13.0.fld2;
(*_46).fld0 = !_27;
(*_46).fld1 = Adt21::Variant0 { fld0: _29.1,fld1: 421617301_i32,fld2: (*_46).fld3 };
_71 = [_35.0,_35.0,_35.0,_35.0,_35.0,_35.0,_35.0,_35.0];
Goto(bb38)
}
bb38 = {
_72.fld0 = (*_46).fld0;
_67 = core::ptr::addr_of!(_48);
_13.0.fld2 = _9 as u64;
(*_46).fld3 = Field::<i8>(Variant((*_46).fld1, 0), 2) >> (*_22);
place!(Field::<i32>(Variant((*_46).fld1, 0), 1)) = 840027824_i32 * 2095813430_i32;
(*_46).fld3 = !Field::<i8>(Variant((*_46).fld1, 0), 2);
_44 = !_19;
place!(Field::<u128>(Variant((*_46).fld1, 0), 0)) = !_59.1;
_15 = [(*_67)];
place!(Field::<i8>(Variant((*_46).fld1, 0), 2)) = (*_46).fld3;
(*_46).fld0 = (*_46).fld3 == (*_46).fld3;
(*_46).fld3 = Field::<i8>(Variant((*_46).fld1, 0), 2);
_10 = _5;
(*_46).fld0 = (*_49) >= _13.2.3;
place!(Field::<u128>(Variant((*_46).fld1, 0), 0)) = (*_46).fld0 as u128;
_14.1 = (*_49) | (*_49);
Goto(bb39)
}
bb39 = {
place!(Field::<i8>(Variant((*_46).fld1, 0), 2)) = !(*_46).fld3;
place!(Field::<i8>(Variant((*_46).fld1, 0), 2)) = (*_46).fld3 | (*_46).fld3;
(*_46).fld0 = (*_22) == (*_22);
(*_67) = !_34;
(*_46).fld0 = Field::<i8>(Variant((*_46).fld1, 0), 2) > Field::<i8>(Variant((*_46).fld1, 0), 2);
place!(Field::<u128>(Variant((*_46).fld1, 0), 0)) = _59.1 + _29.1;
place!(Field::<i32>(Variant((*_46).fld1, 0), 1)) = _12 as i32;
_72.fld1 = (*_46).fld1;
_18.0 = &_2;
_69 = -_55;
place!(Field::<i32>(Variant((*_46).fld1, 0), 1)) = Field::<i32>(Variant(_72.fld1, 0), 1);
(*_46).fld2 = _59.0 & _32;
(*_46) = Adt22 { fld0: _72.fld0,fld1: _72.fld1,fld2: _32,fld3: Field::<i8>(Variant(_72.fld1, 0), 2) };
_43 = core::ptr::addr_of!((*_49));
_72 = Adt22 { fld0: (*_46).fld0,fld1: (*_46).fld1,fld2: (*_46).fld2,fld3: (*_46).fld3 };
(*_46) = Adt22 { fld0: _27,fld1: _72.fld1,fld2: _51.0,fld3: _25 };
(*_46).fld1 = Adt21::Variant0 { fld0: Field::<u128>(Variant(_72.fld1, 0), 0),fld1: Field::<i32>(Variant(_72.fld1, 0), 1),fld2: (*_46).fld3 };
place!(Field::<i32>(Variant((*_46).fld1, 0), 1)) = Field::<i32>(Variant(_72.fld1, 0), 1) + Field::<i32>(Variant(_72.fld1, 0), 1);
Goto(bb40)
}
bb40 = {
_19 = _44 | _44;
place!(Field::<i32>(Variant((*_46).fld1, 0), 1)) = -Field::<i32>(Variant(_72.fld1, 0), 1);
(*_46).fld0 = _27 ^ _72.fld0;
place!(Field::<i8>(Variant((*_46).fld1, 0), 2)) = (*_46).fld3 ^ (*_46).fld3;
_49 = &_18.1;
(*_46) = Move(_72);
(*_46).fld0 = (*_46).fld2 > _32;
_72.fld0 = (*_46).fld0 == (*_46).fld0;
_18.2 = _57 - _13.2.2;
(*_46).fld0 = !_72.fld0;
_59.1 = Field::<u128>(Variant((*_46).fld1, 0), 0);
_69 = -_41;
(*_67) = -_34;
(*_46).fld2 = Field::<i8>(Variant((*_46).fld1, 0), 2) as u64;
(*_46).fld3 = Field::<i8>(Variant((*_46).fld1, 0), 2);
_79 = [Field::<i8>(Variant((*_46).fld1, 0), 2),(*_46).fld3];
match _14.0 {
0 => bb3,
1 => bb41,
2 => bb42,
3 => bb43,
4 => bb44,
5 => bb45,
195 => bb47,
_ => bb46
}
}
bb41 = {
Return()
}
bb42 = {
_72.fld0 = (*_46).fld0;
_67 = core::ptr::addr_of!(_48);
_13.0.fld2 = _9 as u64;
(*_46).fld3 = Field::<i8>(Variant((*_46).fld1, 0), 2) >> (*_22);
place!(Field::<i32>(Variant((*_46).fld1, 0), 1)) = 840027824_i32 * 2095813430_i32;
(*_46).fld3 = !Field::<i8>(Variant((*_46).fld1, 0), 2);
_44 = !_19;
place!(Field::<u128>(Variant((*_46).fld1, 0), 0)) = !_59.1;
_15 = [(*_67)];
place!(Field::<i8>(Variant((*_46).fld1, 0), 2)) = (*_46).fld3;
(*_46).fld0 = (*_46).fld3 == (*_46).fld3;
(*_46).fld3 = Field::<i8>(Variant((*_46).fld1, 0), 2);
_10 = _5;
(*_46).fld0 = (*_49) >= _13.2.3;
place!(Field::<u128>(Variant((*_46).fld1, 0), 0)) = (*_46).fld0 as u128;
_14.1 = (*_49) | (*_49);
Goto(bb39)
}
bb43 = {
Return()
}
bb44 = {
(*_46).fld3 = Field::<i8>(Variant((*_46).fld1, 0), 2) + Field::<i8>(Variant((*_46).fld1, 0), 2);
place!(Field::<i8>(Variant((*_46).fld1, 0), 2)) = (*_46).fld3 | (*_46).fld3;
place!(Field::<i8>(Variant((*_46).fld1, 0), 2)) = (*_46).fld3 | _13.0.fld3;
(*_46).fld1 = Adt21::Variant0 { fld0: _59.1,fld1: 1771517925_i32,fld2: _26 };
_51.0 = _59.0 << Field::<i8>(Variant((*_46).fld1, 0), 2);
place!(Field::<i32>(Variant((*_46).fld1, 0), 1)) = _41 as i32;
place!(Field::<i8>(Variant((*_46).fld1, 0), 2)) = -(*_46).fld3;
place!(Field::<i32>(Variant((*_46).fld1, 0), 1)) = (-1007345368_i32) >> Field::<i8>(Variant((*_46).fld1, 0), 2);
(*_46).fld2 = _59.0 << _33;
_59 = ((*_46).fld2, Field::<u128>(Variant((*_46).fld1, 0), 0));
_12 = _6;
(*_46).fld0 = Field::<i8>(Variant((*_46).fld1, 0), 2) <= Field::<i8>(Variant((*_46).fld1, 0), 2);
_48 = _34 * _34;
place!(Field::<i32>(Variant((*_46).fld1, 0), 1)) = 265430482_i32 >> (*_46).fld3;
_49 = &_40.0.0;
(*_46).fld2 = !_51.0;
place!(Field::<i8>(Variant((*_46).fld1, 0), 2)) = -(*_46).fld3;
place!(Field::<u128>(Variant((*_46).fld1, 0), 0)) = _14.0 as u128;
_4 = _5;
(*_46).fld3 = _25 + Field::<i8>(Variant((*_46).fld1, 0), 2);
_55 = _41 * _41;
(*_46).fld2 = !_59.0;
match _29.1 {
0 => bb11,
1 => bb4,
2 => bb31,
3 => bb32,
4 => bb33,
5 => bb34,
313267841392737467279369924542839884219 => bb36,
_ => bb35
}
}
bb45 = {
Return()
}
bb46 = {
place!(Field::<i32>(Variant(_13.0.fld1, 0), 1)) = (-1792447608_i32);
(*RET).fld3 = (*RET).fld2 as i8;
(*RET).fld3 = -Field::<i8>(Variant((*RET).fld1, 0), 2);
(*RET).fld1 = Adt21::Variant0 { fld0: 94535286577081734824241570137803329024_u128,fld1: 1880065236_i32,fld2: (*RET).fld3 };
place!(Field::<i8>(Variant((*RET).fld1, 0), 2)) = (*RET).fld3 | (*RET).fld3;
_20 = _7 | _7;
(*RET).fld3 = Field::<i8>(Variant((*RET).fld1, 0), 2);
place!(Field::<i32>(Variant((*RET).fld1, 0), 1)) = 752502629_i32;
_21 = _20 << Field::<i8>(Variant((*RET).fld1, 0), 2);
_1 = _6;
place!(Field::<i32>(Variant((*RET).fld1, 0), 1)) = 187910916_i32 | (-189740058_i32);
_7 = _14.0 as isize;
place!(Field::<u128>(Variant((*RET).fld1, 0), 0)) = 105000791656817546387157293648709254562_u128 | 339410805688609812394309366782431552328_u128;
_3 = _1;
(*RET).fld2 = !4860519979336303305_u64;
place!(Field::<i8>(Variant((*RET).fld1, 0), 2)) = (*RET).fld3 + _13.0.fld3;
place!(Field::<i32>(Variant((*RET).fld1, 0), 1)) = Field::<i8>(Variant((*RET).fld1, 0), 2) as i32;
(*RET).fld3 = Field::<i8>(Variant((*RET).fld1, 0), 2) >> Field::<i32>(Variant((*RET).fld1, 0), 1);
(*RET).fld0 = _16 | _16;
place!(Field::<i32>(Variant((*RET).fld1, 0), 1)) = 455900062_i32;
(*RET).fld2 = 15540858416304688095_u64 << Field::<i8>(Variant(_13.0.fld1, 0), 2);
(*RET).fld3 = 60765321_u32 as i8;
_6 = _3;
(*RET).fld0 = !_16;
(*RET).fld0 = (*RET).fld2 >= (*RET).fld2;
Goto(bb18)
}
bb47 = {
_52 = Field::<u128>(Variant((*_46).fld1, 0), 0) as f64;
(*_46).fld2 = _33 as u64;
(*_46).fld0 = (*_46).fld3 < Field::<i8>(Variant((*_46).fld1, 0), 2);
_65 = [_38,_11,_11,_9,_12,_3,_5,_3];
_80.0 = Move(_2);
_72.fld2 = (*_67) as u64;
_6 = _3;
(*_46).fld2 = !_59.0;
_13.1 = &_35.0;
(*_46).fld0 = _72.fld0 ^ _72.fld0;
Call(place!(Field::<u128>(Variant((*_46).fld1, 0), 0)) = core::intrinsics::bswap(_59.1), ReturnTo(bb48), UnwindUnreachable())
}
bb48 = {
(*_46).fld1 = Adt21::Variant0 { fld0: _59.1,fld1: (-762607622_i32),fld2: (*_46).fld3 };
_52 = (*_46).fld3 as f64;
_80.2 = core::ptr::addr_of!(_69);
_72.fld3 = (*_46).fld3 ^ (*_46).fld3;
_77 = _33;
(*_46).fld1 = Adt21::Variant1 { fld0: (*_46).fld0,fld1: _52,fld2: _77,fld3: (*_46).fld3,fld4: _18.2,fld5: _35.0,fld6: _41,fld7: 3_usize };
place!(Field::<bool>(Variant((*_46).fld1, 1), 0)) = Field::<f64>(Variant((*_46).fld1, 1), 1) < Field::<f64>(Variant((*_46).fld1, 1), 1);
place!(Field::<u32>(Variant((*_46).fld1, 1), 5)) = _35.0;
place!(Field::<u32>(Variant((*_46).fld1, 1), 5)) = _35.0 * _35.0;
Call(place!(Field::<i8>(Variant((*_46).fld1, 1), 3)) = fn19(Move(_46), _77, Field::<f32>(Variant((*_46).fld1, 1), 6), Field::<i16>(Variant((*_46).fld1, 1), 4), Field::<bool>(Variant((*_46).fld1, 1), 0), (*_46).fld2, Move(_67), (*_49), Move(_80.2), Move(_40.1), Field::<isize>(Variant((*_46).fld1, 1), 2)), ReturnTo(bb49), UnwindUnreachable())
}
bb49 = {
_79 = [_25,_26];
_82 = [_34,_48];
_14.0 = 13_u8 & 122_u8;
_36 = _19 & _19;
_18.3 = (*_49);
_5 = _11;
RET = core::ptr::addr_of!(_72);
_49 = &(*_43);
(*RET).fld2 = _13.0.fld2 >> (*RET).fld3;
(*RET).fld0 = !Field::<bool>(Variant(_13.0.fld1, 1), 0);
place!(Field::<i8>(Variant(_13.0.fld1, 1), 3)) = _13.0.fld3 & (*RET).fld3;
(*RET).fld3 = !_26;
_80.1 = [_59.1,_59.1,_59.1,_59.1,_59.1,_59.1,_29.1];
_80.0 = &_10;
_3 = _8;
_13.2.2 = !Field::<i16>(Variant(_13.0.fld1, 1), 4);
(*RET).fld3 = _25 * _13.0.fld3;
_67 = core::ptr::addr_of!(_48);
_13.2.0 = &_80.0;
Goto(bb50)
}
bb50 = {
Call(_88 = dump_var(Move(_82), Move(_57), Move(_68), Move(_33)), ReturnTo(bb51), UnwindUnreachable())
}
bb51 = {
Call(_88 = dump_var(Move(_65), Move(_35), Move(_44), Move(_1)), ReturnTo(bb52), UnwindUnreachable())
}
bb52 = {
Call(_88 = dump_var(Move(_15), Move(_20), Move(_4), Move(_10)), ReturnTo(bb53), UnwindUnreachable())
}
bb53 = {
Call(_88 = dump_var(Move(_11), Move(_25), Move(_79), Move(_16)), ReturnTo(bb54), UnwindUnreachable())
}
bb54 = {
Call(_88 = dump_var(Move(_77), Move(_26), Move(_29), _89), ReturnTo(bb55), UnwindUnreachable())
}
bb55 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn18(mut _1: &'static i16,mut _2: Adt22,mut _3: *const Adt22,mut _4: &'static &'static char,mut _5: isize) -> i64 {
mir! {
type RET = i64;
let _6: *mut [u8; 4];
let _7: u16;
let _8: char;
let _9: [u128; 7];
let _10: *mut [isize; 5];
let _11: i128;
let _12: char;
let _13: [u8; 4];
let _14: f64;
let _15: usize;
let _16: *const f32;
let _17: (&'static u128, (i16, &'static i64, [u32; 8], i16), &'static u32);
let _18: [char; 8];
let _19: bool;
let _20: char;
let _21: i16;
let _22: isize;
let _23: &'static mut (i64,);
let _24: &'static *mut isize;
let _25: f32;
let _26: [char; 8];
let _27: *mut u16;
let _28: *mut [u8; 4];
let _29: (u64, u128);
let _30: *mut isize;
let _31: [char; 8];
let _32: [i8; 2];
let _33: usize;
let _34: &'static mut [bool; 2];
let _35: u128;
let _36: *mut Adt75;
let _37: f32;
let _38: (&'static mut *mut i128,);
let _39: *mut i128;
let _40: u8;
let _41: [bool; 2];
let _42: (u64, u128);
let _43: *mut u16;
let _44: ([i128; 2], &'static mut *mut i128, i16);
let _45: f64;
let _46: isize;
let _47: Adt73;
let _48: i128;
let _49: [i128; 2];
let _50: ((&'static mut *mut i128,), (f32, i16, usize, u8), (&'static &'static char, i64, i16, i64), &'static i64);
let _51: *mut Adt73;
let _52: bool;
let _53: f64;
let _54: i64;
let _55: [u8; 4];
let _56: *const i128;
let _57: isize;
let _58: usize;
let _59: isize;
let _60: bool;
let _61: i8;
let _62: bool;
let _63: &'static u128;
let _64: [i128; 1];
let _65: *const Adt22;
let _66: (u32,);
let _67: *const i128;
let _68: *mut &'static *mut isize;
let _69: isize;
let _70: &'static u128;
let _71: (i64,);
let _72: (i16, &'static i64, [u32; 8], i16);
let _73: i8;
let _74: (f32, i16, usize, u8);
let _75: bool;
let _76: ();
let _77: ();
{
_2.fld1 = Adt21::Variant0 { fld0: 5610502514492860763776637284104107596_u128,fld1: (-1997242165_i32),fld2: _2.fld3 };
RET = 9_u8 as i64;
Goto(bb1)
}
bb1 = {
_2.fld3 = -Field::<i8>(Variant(_2.fld1, 0), 2);
_2.fld0 = false;
place!(Field::<u128>(Variant(_2.fld1, 0), 0)) = 166695586101368990003060982858003281706_u128;
place!(Field::<i8>(Variant(_2.fld1, 0), 2)) = _2.fld3;
_5 = 47_isize & (-9223372036854775808_isize);
RET = 6546563463845715655_i64;
_2.fld3 = Field::<i8>(Variant(_2.fld1, 0), 2) >> Field::<i8>(Variant(_2.fld1, 0), 2);
_2.fld1 = Adt21::Variant0 { fld0: 196718857085293825338195854263456265281_u128,fld1: 736737551_i32,fld2: _2.fld3 };
_3 = core::ptr::addr_of!(_2);
place!(Field::<i8>(Variant(_2.fld1, 0), 2)) = (*_3).fld3 << (*_3).fld3;
(*_3).fld0 = false;
place!(Field::<i8>(Variant((*_3).fld1, 0), 2)) = (*_3).fld0 as i8;
(*_3).fld2 = 11015704025903207688_u64;
(*_3).fld2 = 11112_i16 as u64;
place!(Field::<i32>(Variant((*_3).fld1, 0), 1)) = !(-385191832_i32);
place!(Field::<u128>(Variant((*_3).fld1, 0), 0)) = 174782804658046704017441464475867554021_u128;
Goto(bb2)
}
bb2 = {
_3 = core::ptr::addr_of!((*_3));
place!(Field::<i32>(Variant(_2.fld1, 0), 1)) = (-1934724721_i32) - (-1578598717_i32);
(*_3).fld3 = Field::<i8>(Variant((*_3).fld1, 0), 2) | Field::<i8>(Variant((*_3).fld1, 0), 2);
_5 = !(-37_isize);
place!(Field::<u128>(Variant((*_3).fld1, 0), 0)) = !143553356902212587491879399533793983534_u128;
place!(Field::<u128>(Variant((*_3).fld1, 0), 0)) = !255920104204445779620866013006549961402_u128;
place!(Field::<i32>(Variant(_2.fld1, 0), 1)) = (-1275018411_i32) | 43975879_i32;
place!(Field::<u128>(Variant((*_3).fld1, 0), 0)) = 109608621960070531286441282363562208511_u128 * 209616555391847672252459506939119223297_u128;
place!(Field::<u128>(Variant((*_3).fld1, 0), 0)) = _5 as u128;
(*_3).fld0 = !false;
place!(Field::<i32>(Variant((*_3).fld1, 0), 1)) = (-1871460978_i32);
place!(Field::<i32>(Variant((*_3).fld1, 0), 1)) = 1118016074_i32 ^ 1411735274_i32;
(*_3).fld0 = !true;
place!(Field::<u128>(Variant((*_3).fld1, 0), 0)) = Field::<i32>(Variant((*_3).fld1, 0), 1) as u128;
(*_3).fld2 = 7352643244875610582_u64 * 427134010308202520_u64;
_8 = '\u{b83fa}';
place!(Field::<i8>(Variant((*_3).fld1, 0), 2)) = (*_3).fld3 << Field::<i32>(Variant((*_3).fld1, 0), 1);
(*_3).fld0 = Field::<i8>(Variant((*_3).fld1, 0), 2) < Field::<i8>(Variant((*_3).fld1, 0), 2);
Call((*_3).fld3 = core::intrinsics::bswap(Field::<i8>(Variant((*_3).fld1, 0), 2)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
(*_3).fld2 = 1098824220906510750_u64;
(*_3).fld3 = Field::<i8>(Variant((*_3).fld1, 0), 2);
Goto(bb4)
}
bb4 = {
place!(Field::<i32>(Variant((*_3).fld1, 0), 1)) = (-1154785821_i32);
place!(Field::<i32>(Variant((*_3).fld1, 0), 1)) = 1459733431_i32;
place!(Field::<i8>(Variant((*_3).fld1, 0), 2)) = -_2.fld3;
place!(Field::<i8>(Variant((*_3).fld1, 0), 2)) = (*_3).fld3;
(*_3).fld3 = Field::<i8>(Variant((*_3).fld1, 0), 2) + Field::<i8>(Variant((*_3).fld1, 0), 2);
place!(Field::<i32>(Variant((*_3).fld1, 0), 1)) = (-357162107_i32) >> Field::<i8>(Variant((*_3).fld1, 0), 2);
(*_3).fld3 = !Field::<i8>(Variant((*_3).fld1, 0), 2);
place!(Field::<u128>(Variant((*_3).fld1, 0), 0)) = 207190339706932268412345202050457623193_u128 & 188662274962285834673031696432875122817_u128;
_7 = 42694_u16;
place!(Field::<i32>(Variant((*_3).fld1, 0), 1)) = 445433080_i32;
_2.fld2 = RET as u64;
place!(Field::<i8>(Variant((*_3).fld1, 0), 2)) = (*_3).fld3;
place!(Field::<i8>(Variant(_2.fld1, 0), 2)) = (*_3).fld3 ^ (*_3).fld3;
_17.1.2 = [2822630718_u32,598674690_u32,3854666655_u32,2928577171_u32,1242406887_u32,3933442440_u32,536891350_u32,1524273798_u32];
_17.1.3 = 679_i16 & (-26446_i16);
place!(Field::<i8>(Variant((*_3).fld1, 0), 2)) = (*_3).fld3 * (*_3).fld3;
place!(Field::<u128>(Variant((*_3).fld1, 0), 0)) = _8 as u128;
(*_3).fld0 = false | false;
place!(Field::<i8>(Variant((*_3).fld1, 0), 2)) = (*_3).fld3;
place!(Field::<i32>(Variant((*_3).fld1, 0), 1)) = 355076934_i32 - (-1879679531_i32);
match RET {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb5,
6546563463845715655 => bb7,
_ => bb6
}
}
bb5 = {
(*_3).fld2 = 1098824220906510750_u64;
(*_3).fld3 = Field::<i8>(Variant((*_3).fld1, 0), 2);
Goto(bb4)
}
bb6 = {
_3 = core::ptr::addr_of!((*_3));
place!(Field::<i32>(Variant(_2.fld1, 0), 1)) = (-1934724721_i32) - (-1578598717_i32);
(*_3).fld3 = Field::<i8>(Variant((*_3).fld1, 0), 2) | Field::<i8>(Variant((*_3).fld1, 0), 2);
_5 = !(-37_isize);
place!(Field::<u128>(Variant((*_3).fld1, 0), 0)) = !143553356902212587491879399533793983534_u128;
place!(Field::<u128>(Variant((*_3).fld1, 0), 0)) = !255920104204445779620866013006549961402_u128;
place!(Field::<i32>(Variant(_2.fld1, 0), 1)) = (-1275018411_i32) | 43975879_i32;
place!(Field::<u128>(Variant((*_3).fld1, 0), 0)) = 109608621960070531286441282363562208511_u128 * 209616555391847672252459506939119223297_u128;
place!(Field::<u128>(Variant((*_3).fld1, 0), 0)) = _5 as u128;
(*_3).fld0 = !false;
place!(Field::<i32>(Variant((*_3).fld1, 0), 1)) = (-1871460978_i32);
place!(Field::<i32>(Variant((*_3).fld1, 0), 1)) = 1118016074_i32 ^ 1411735274_i32;
(*_3).fld0 = !true;
place!(Field::<u128>(Variant((*_3).fld1, 0), 0)) = Field::<i32>(Variant((*_3).fld1, 0), 1) as u128;
(*_3).fld2 = 7352643244875610582_u64 * 427134010308202520_u64;
_8 = '\u{b83fa}';
place!(Field::<i8>(Variant((*_3).fld1, 0), 2)) = (*_3).fld3 << Field::<i32>(Variant((*_3).fld1, 0), 1);
(*_3).fld0 = Field::<i8>(Variant((*_3).fld1, 0), 2) < Field::<i8>(Variant((*_3).fld1, 0), 2);
Call((*_3).fld3 = core::intrinsics::bswap(Field::<i8>(Variant((*_3).fld1, 0), 2)), ReturnTo(bb3), UnwindUnreachable())
}
bb7 = {
_9 = [Field::<u128>(Variant((*_3).fld1, 0), 0),Field::<u128>(Variant((*_3).fld1, 0), 0),Field::<u128>(Variant((*_3).fld1, 0), 0),Field::<u128>(Variant((*_3).fld1, 0), 0),Field::<u128>(Variant((*_3).fld1, 0), 0),Field::<u128>(Variant((*_3).fld1, 0), 0),Field::<u128>(Variant((*_3).fld1, 0), 0)];
_14 = _5 as f64;
(*_3).fld0 = true;
place!(Field::<i8>(Variant((*_3).fld1, 0), 2)) = (*_3).fld3;
(*_3).fld2 = 2521865254332700539_u64 >> Field::<i32>(Variant((*_3).fld1, 0), 1);
(*_3).fld3 = Field::<i8>(Variant((*_3).fld1, 0), 2);
place!(Field::<u128>(Variant((*_3).fld1, 0), 0)) = 144925380157939860531701828547237283513_u128 | 273573995887735013793312738883779910854_u128;
(*_3).fld0 = false | true;
place!(Field::<i32>(Variant((*_3).fld1, 0), 1)) = 1690210703_i32;
(*_3).fld2 = 16567967350090701063_u64 * 17717485505171872416_u64;
(*_3).fld2 = Field::<i32>(Variant((*_3).fld1, 0), 1) as u64;
(*_3).fld2 = _14 as u64;
(*_3).fld2 = 16010825907605396816_u64 ^ 14551132514798622983_u64;
_7 = 46188_u16;
place!(Field::<u128>(Variant((*_3).fld1, 0), 0)) = 39658558175522165708221460554381081498_u128;
(*_3).fld0 = !false;
place!(Field::<i32>(Variant((*_3).fld1, 0), 1)) = -(-1961156759_i32);
place!(Field::<u128>(Variant((*_3).fld1, 0), 0)) = 264742836194948173707857622543849253597_u128 - 123472095532893502511650509672794914697_u128;
match RET {
0 => bb4,
1 => bb5,
2 => bb8,
6546563463845715655 => bb10,
_ => bb9
}
}
bb8 = {
_3 = core::ptr::addr_of!((*_3));
place!(Field::<i32>(Variant(_2.fld1, 0), 1)) = (-1934724721_i32) - (-1578598717_i32);
(*_3).fld3 = Field::<i8>(Variant((*_3).fld1, 0), 2) | Field::<i8>(Variant((*_3).fld1, 0), 2);
_5 = !(-37_isize);
place!(Field::<u128>(Variant((*_3).fld1, 0), 0)) = !143553356902212587491879399533793983534_u128;
place!(Field::<u128>(Variant((*_3).fld1, 0), 0)) = !255920104204445779620866013006549961402_u128;
place!(Field::<i32>(Variant(_2.fld1, 0), 1)) = (-1275018411_i32) | 43975879_i32;
place!(Field::<u128>(Variant((*_3).fld1, 0), 0)) = 109608621960070531286441282363562208511_u128 * 209616555391847672252459506939119223297_u128;
place!(Field::<u128>(Variant((*_3).fld1, 0), 0)) = _5 as u128;
(*_3).fld0 = !false;
place!(Field::<i32>(Variant((*_3).fld1, 0), 1)) = (-1871460978_i32);
place!(Field::<i32>(Variant((*_3).fld1, 0), 1)) = 1118016074_i32 ^ 1411735274_i32;
(*_3).fld0 = !true;
place!(Field::<u128>(Variant((*_3).fld1, 0), 0)) = Field::<i32>(Variant((*_3).fld1, 0), 1) as u128;
(*_3).fld2 = 7352643244875610582_u64 * 427134010308202520_u64;
_8 = '\u{b83fa}';
place!(Field::<i8>(Variant((*_3).fld1, 0), 2)) = (*_3).fld3 << Field::<i32>(Variant((*_3).fld1, 0), 1);
(*_3).fld0 = Field::<i8>(Variant((*_3).fld1, 0), 2) < Field::<i8>(Variant((*_3).fld1, 0), 2);
Call((*_3).fld3 = core::intrinsics::bswap(Field::<i8>(Variant((*_3).fld1, 0), 2)), ReturnTo(bb3), UnwindUnreachable())
}
bb9 = {
(*_3).fld2 = 1098824220906510750_u64;
(*_3).fld3 = Field::<i8>(Variant((*_3).fld1, 0), 2);
Goto(bb4)
}
bb10 = {
(*_3).fld2 = 247_u8 as u64;
_3 = core::ptr::addr_of!((*_3));
match RET {
0 => bb5,
1 => bb6,
2 => bb11,
3 => bb12,
6546563463845715655 => bb14,
_ => bb13
}
}
bb11 = {
_3 = core::ptr::addr_of!((*_3));
place!(Field::<i32>(Variant(_2.fld1, 0), 1)) = (-1934724721_i32) - (-1578598717_i32);
(*_3).fld3 = Field::<i8>(Variant((*_3).fld1, 0), 2) | Field::<i8>(Variant((*_3).fld1, 0), 2);
_5 = !(-37_isize);
place!(Field::<u128>(Variant((*_3).fld1, 0), 0)) = !143553356902212587491879399533793983534_u128;
place!(Field::<u128>(Variant((*_3).fld1, 0), 0)) = !255920104204445779620866013006549961402_u128;
place!(Field::<i32>(Variant(_2.fld1, 0), 1)) = (-1275018411_i32) | 43975879_i32;
place!(Field::<u128>(Variant((*_3).fld1, 0), 0)) = 109608621960070531286441282363562208511_u128 * 209616555391847672252459506939119223297_u128;
place!(Field::<u128>(Variant((*_3).fld1, 0), 0)) = _5 as u128;
(*_3).fld0 = !false;
place!(Field::<i32>(Variant((*_3).fld1, 0), 1)) = (-1871460978_i32);
place!(Field::<i32>(Variant((*_3).fld1, 0), 1)) = 1118016074_i32 ^ 1411735274_i32;
(*_3).fld0 = !true;
place!(Field::<u128>(Variant((*_3).fld1, 0), 0)) = Field::<i32>(Variant((*_3).fld1, 0), 1) as u128;
(*_3).fld2 = 7352643244875610582_u64 * 427134010308202520_u64;
_8 = '\u{b83fa}';
place!(Field::<i8>(Variant((*_3).fld1, 0), 2)) = (*_3).fld3 << Field::<i32>(Variant((*_3).fld1, 0), 1);
(*_3).fld0 = Field::<i8>(Variant((*_3).fld1, 0), 2) < Field::<i8>(Variant((*_3).fld1, 0), 2);
Call((*_3).fld3 = core::intrinsics::bswap(Field::<i8>(Variant((*_3).fld1, 0), 2)), ReturnTo(bb3), UnwindUnreachable())
}
bb12 = {
place!(Field::<i32>(Variant((*_3).fld1, 0), 1)) = (-1154785821_i32);
place!(Field::<i32>(Variant((*_3).fld1, 0), 1)) = 1459733431_i32;
place!(Field::<i8>(Variant((*_3).fld1, 0), 2)) = -_2.fld3;
place!(Field::<i8>(Variant((*_3).fld1, 0), 2)) = (*_3).fld3;
(*_3).fld3 = Field::<i8>(Variant((*_3).fld1, 0), 2) + Field::<i8>(Variant((*_3).fld1, 0), 2);
place!(Field::<i32>(Variant((*_3).fld1, 0), 1)) = (-357162107_i32) >> Field::<i8>(Variant((*_3).fld1, 0), 2);
(*_3).fld3 = !Field::<i8>(Variant((*_3).fld1, 0), 2);
place!(Field::<u128>(Variant((*_3).fld1, 0), 0)) = 207190339706932268412345202050457623193_u128 & 188662274962285834673031696432875122817_u128;
_7 = 42694_u16;
place!(Field::<i32>(Variant((*_3).fld1, 0), 1)) = 445433080_i32;
_2.fld2 = RET as u64;
place!(Field::<i8>(Variant((*_3).fld1, 0), 2)) = (*_3).fld3;
place!(Field::<i8>(Variant(_2.fld1, 0), 2)) = (*_3).fld3 ^ (*_3).fld3;
_17.1.2 = [2822630718_u32,598674690_u32,3854666655_u32,2928577171_u32,1242406887_u32,3933442440_u32,536891350_u32,1524273798_u32];
_17.1.3 = 679_i16 & (-26446_i16);
place!(Field::<i8>(Variant((*_3).fld1, 0), 2)) = (*_3).fld3 * (*_3).fld3;
place!(Field::<u128>(Variant((*_3).fld1, 0), 0)) = _8 as u128;
(*_3).fld0 = false | false;
place!(Field::<i8>(Variant((*_3).fld1, 0), 2)) = (*_3).fld3;
place!(Field::<i32>(Variant((*_3).fld1, 0), 1)) = 355076934_i32 - (-1879679531_i32);
match RET {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb5,
6546563463845715655 => bb7,
_ => bb6
}
}
bb13 = {
_9 = [Field::<u128>(Variant((*_3).fld1, 0), 0),Field::<u128>(Variant((*_3).fld1, 0), 0),Field::<u128>(Variant((*_3).fld1, 0), 0),Field::<u128>(Variant((*_3).fld1, 0), 0),Field::<u128>(Variant((*_3).fld1, 0), 0),Field::<u128>(Variant((*_3).fld1, 0), 0),Field::<u128>(Variant((*_3).fld1, 0), 0)];
_14 = _5 as f64;
(*_3).fld0 = true;
place!(Field::<i8>(Variant((*_3).fld1, 0), 2)) = (*_3).fld3;
(*_3).fld2 = 2521865254332700539_u64 >> Field::<i32>(Variant((*_3).fld1, 0), 1);
(*_3).fld3 = Field::<i8>(Variant((*_3).fld1, 0), 2);
place!(Field::<u128>(Variant((*_3).fld1, 0), 0)) = 144925380157939860531701828547237283513_u128 | 273573995887735013793312738883779910854_u128;
(*_3).fld0 = false | true;
place!(Field::<i32>(Variant((*_3).fld1, 0), 1)) = 1690210703_i32;
(*_3).fld2 = 16567967350090701063_u64 * 17717485505171872416_u64;
(*_3).fld2 = Field::<i32>(Variant((*_3).fld1, 0), 1) as u64;
(*_3).fld2 = _14 as u64;
(*_3).fld2 = 16010825907605396816_u64 ^ 14551132514798622983_u64;
_7 = 46188_u16;
place!(Field::<u128>(Variant((*_3).fld1, 0), 0)) = 39658558175522165708221460554381081498_u128;
(*_3).fld0 = !false;
place!(Field::<i32>(Variant((*_3).fld1, 0), 1)) = -(-1961156759_i32);
place!(Field::<u128>(Variant((*_3).fld1, 0), 0)) = 264742836194948173707857622543849253597_u128 - 123472095532893502511650509672794914697_u128;
match RET {
0 => bb4,
1 => bb5,
2 => bb8,
6546563463845715655 => bb10,
_ => bb9
}
}
bb14 = {
(*_3).fld1 = Adt21::Variant0 { fld0: 114403759757349058050303623338188135128_u128,fld1: 1582869391_i32,fld2: (*_3).fld3 };
(*_3).fld2 = 8479800190577908849_u64;
place!(Field::<i32>(Variant((*_3).fld1, 0), 1)) = 1060131598_i32;
(*_3).fld3 = Field::<i8>(Variant(_2.fld1, 0), 2) << Field::<i8>(Variant((*_3).fld1, 0), 2);
_19 = (*_3).fld3 > _2.fld3;
place!(Field::<i32>(Variant((*_3).fld1, 0), 1)) = (-401129325_i32);
place!(Field::<u128>(Variant((*_3).fld1, 0), 0)) = 299185273480134973220467824850798323_u128 * 78073845885660733058978951295196633556_u128;
place!(Field::<i32>(Variant((*_3).fld1, 0), 1)) = _17.1.3 as i32;
place!(Field::<i8>(Variant((*_3).fld1, 0), 2)) = _14 as i8;
place!(Field::<i8>(Variant((*_3).fld1, 0), 2)) = !(*_3).fld3;
_17.1.1 = &RET;
(*_3).fld3 = Field::<i8>(Variant((*_3).fld1, 0), 2) - Field::<i8>(Variant(_2.fld1, 0), 2);
(*_3).fld2 = !8717431624860109491_u64;
place!(Field::<i8>(Variant((*_3).fld1, 0), 2)) = 4102936183_u32 as i8;
(*_3).fld1 = Adt21::Variant0 { fld0: 227237329625267259378362671831511434742_u128,fld1: 413171404_i32,fld2: (*_3).fld3 };
_28 = core::ptr::addr_of_mut!(_13);
match _7 {
46188 => bb16,
_ => bb15
}
}
bb15 = {
(*_3).fld2 = 1098824220906510750_u64;
(*_3).fld3 = Field::<i8>(Variant((*_3).fld1, 0), 2);
Goto(bb4)
}
bb16 = {
place!(Field::<i32>(Variant((*_3).fld1, 0), 1)) = _14 as i32;
place!(Field::<i32>(Variant(_2.fld1, 0), 1)) = (-246100134_i32);
place!(Field::<u128>(Variant((*_3).fld1, 0), 0)) = 100257385841134743416539060254585121351_u128 | 280387884154970658856940543337503058391_u128;
_15 = 9783353744094665099_usize >> Field::<i8>(Variant((*_3).fld1, 0), 2);
(*_3).fld0 = _19;
_2.fld2 = (*_3).fld0 as u64;
(*_3).fld0 = _19 & _19;
_12 = _8;
(*_28) = [61_u8,85_u8,55_u8,39_u8];
(*_3).fld3 = Field::<i8>(Variant((*_3).fld1, 0), 2);
(*_3).fld2 = 43964227342606873803867213615396599936_i128 as u64;
(*_3).fld3 = Field::<i32>(Variant((*_3).fld1, 0), 1) as i8;
place!(Field::<u128>(Variant((*_3).fld1, 0), 0)) = !147619672499111956890915412395989739481_u128;
_22 = (*_3).fld0 as isize;
_16 = core::ptr::addr_of!(_25);
place!(Field::<i32>(Variant((*_3).fld1, 0), 1)) = 690876408_i32 ^ 362570471_i32;
_8 = _12;
_17.0 = &place!(Field::<u128>(Variant((*_3).fld1, 0), 0));
_31 = [_12,_8,_8,_8,_8,_8,_8,_12];
_17.1.2 = [3645362325_u32,1753124003_u32,962002770_u32,3356311537_u32,4289048693_u32,463085723_u32,826247670_u32,1409040782_u32];
_2.fld2 = Field::<u128>(Variant((*_3).fld1, 0), 0) as u64;
(*_3).fld3 = Field::<i8>(Variant((*_3).fld1, 0), 2);
(*_3).fld2 = _14 as u64;
match _7 {
0 => bb11,
1 => bb14,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
46188 => bb17,
_ => bb8
}
}
bb17 = {
match RET {
0 => bb18,
6546563463845715655 => bb20,
_ => bb19
}
}
bb18 = {
(*_3).fld2 = 1098824220906510750_u64;
(*_3).fld3 = Field::<i8>(Variant((*_3).fld1, 0), 2);
Goto(bb4)
}
bb19 = {
_9 = [Field::<u128>(Variant((*_3).fld1, 0), 0),Field::<u128>(Variant((*_3).fld1, 0), 0),Field::<u128>(Variant((*_3).fld1, 0), 0),Field::<u128>(Variant((*_3).fld1, 0), 0),Field::<u128>(Variant((*_3).fld1, 0), 0),Field::<u128>(Variant((*_3).fld1, 0), 0),Field::<u128>(Variant((*_3).fld1, 0), 0)];
_14 = _5 as f64;
(*_3).fld0 = true;
place!(Field::<i8>(Variant((*_3).fld1, 0), 2)) = (*_3).fld3;
(*_3).fld2 = 2521865254332700539_u64 >> Field::<i32>(Variant((*_3).fld1, 0), 1);
(*_3).fld3 = Field::<i8>(Variant((*_3).fld1, 0), 2);
place!(Field::<u128>(Variant((*_3).fld1, 0), 0)) = 144925380157939860531701828547237283513_u128 | 273573995887735013793312738883779910854_u128;
(*_3).fld0 = false | true;
place!(Field::<i32>(Variant((*_3).fld1, 0), 1)) = 1690210703_i32;
(*_3).fld2 = 16567967350090701063_u64 * 17717485505171872416_u64;
(*_3).fld2 = Field::<i32>(Variant((*_3).fld1, 0), 1) as u64;
(*_3).fld2 = _14 as u64;
(*_3).fld2 = 16010825907605396816_u64 ^ 14551132514798622983_u64;
_7 = 46188_u16;
place!(Field::<u128>(Variant((*_3).fld1, 0), 0)) = 39658558175522165708221460554381081498_u128;
(*_3).fld0 = !false;
place!(Field::<i32>(Variant((*_3).fld1, 0), 1)) = -(-1961156759_i32);
place!(Field::<u128>(Variant((*_3).fld1, 0), 0)) = 264742836194948173707857622543849253597_u128 - 123472095532893502511650509672794914697_u128;
match RET {
0 => bb4,
1 => bb5,
2 => bb8,
6546563463845715655 => bb10,
_ => bb9
}
}
bb20 = {
_27 = core::ptr::addr_of_mut!(_7);
(*_16) = (*_27) as f32;
_2.fld2 = 17191150948281838404_u64 ^ 13035384724131584898_u64;
_14 = (*_3).fld3 as f64;
(*_3).fld3 = Field::<i8>(Variant((*_3).fld1, 0), 2) << _22;
(*_28) = [30_u8,111_u8,112_u8,11_u8];
(*_16) = 158936758648882208333631924066622398214_i128 as f32;
(*_3).fld3 = (*_27) as i8;
(*_16) = _17.1.3 as f32;
place!(Field::<i32>(Variant((*_3).fld1, 0), 1)) = (-982999760_i32);
(*_16) = _15 as f32;
place!(Field::<i8>(Variant((*_3).fld1, 0), 2)) = (*_3).fld3 & _2.fld3;
match (*_27) {
0 => bb14,
1 => bb16,
46188 => bb21,
_ => bb19
}
}
bb21 = {
(*_28) = [60_u8,160_u8,117_u8,72_u8];
(*_3).fld0 = !_19;
(*_3).fld3 = !Field::<i8>(Variant((*_3).fld1, 0), 2);
(*_3).fld2 = 15738527896181135866_u64 << (*_27);
(*_3).fld3 = Field::<i8>(Variant((*_3).fld1, 0), 2);
place!(Field::<i8>(Variant((*_3).fld1, 0), 2)) = (*_3).fld3 >> Field::<i32>(Variant((*_3).fld1, 0), 1);
match Field::<i32>(Variant((*_3).fld1, 0), 1) {
340282366920938463463374607430785211696 => bb22,
_ => bb15
}
}
bb22 = {
(*_16) = Field::<i32>(Variant((*_3).fld1, 0), 1) as f32;
(*_27) = !575_u16;
(*_3).fld1 = Adt21::Variant0 { fld0: 94075424178276200800339599006624090000_u128,fld1: 313229527_i32,fld2: (*_3).fld3 };
place!(Field::<i8>(Variant((*_3).fld1, 0), 2)) = (*_3).fld3 - (*_3).fld3;
place!(Field::<u128>(Variant((*_3).fld1, 0), 0)) = 4500533633948047597525159256217155446_u128 - 152565989316415734658536205491967295972_u128;
(*_3).fld3 = (*_16) as i8;
place!(Field::<i32>(Variant((*_3).fld1, 0), 1)) = -(-330685443_i32);
_11 = 88448593118647664946199537289921782230_i128 << Field::<u128>(Variant((*_3).fld1, 0), 0);
(*_3).fld3 = Field::<i8>(Variant((*_3).fld1, 0), 2);
_17.1.2 = [2195518195_u32,4210582289_u32,3141183082_u32,3026571977_u32,1238435532_u32,1228239136_u32,2641444679_u32,2338720973_u32];
(*_16) = _15 as f32;
match RET {
0 => bb23,
1 => bb24,
2 => bb25,
3 => bb26,
4 => bb27,
5 => bb28,
6546563463845715655 => bb30,
_ => bb29
}
}
bb23 = {
(*_28) = [60_u8,160_u8,117_u8,72_u8];
(*_3).fld0 = !_19;
(*_3).fld3 = !Field::<i8>(Variant((*_3).fld1, 0), 2);
(*_3).fld2 = 15738527896181135866_u64 << (*_27);
(*_3).fld3 = Field::<i8>(Variant((*_3).fld1, 0), 2);
place!(Field::<i8>(Variant((*_3).fld1, 0), 2)) = (*_3).fld3 >> Field::<i32>(Variant((*_3).fld1, 0), 1);
match Field::<i32>(Variant((*_3).fld1, 0), 1) {
340282366920938463463374607430785211696 => bb22,
_ => bb15
}
}
bb24 = {
(*_3).fld2 = 1098824220906510750_u64;
(*_3).fld3 = Field::<i8>(Variant((*_3).fld1, 0), 2);
Goto(bb4)
}
bb25 = {
_3 = core::ptr::addr_of!((*_3));
place!(Field::<i32>(Variant(_2.fld1, 0), 1)) = (-1934724721_i32) - (-1578598717_i32);
(*_3).fld3 = Field::<i8>(Variant((*_3).fld1, 0), 2) | Field::<i8>(Variant((*_3).fld1, 0), 2);
_5 = !(-37_isize);
place!(Field::<u128>(Variant((*_3).fld1, 0), 0)) = !143553356902212587491879399533793983534_u128;
place!(Field::<u128>(Variant((*_3).fld1, 0), 0)) = !255920104204445779620866013006549961402_u128;
place!(Field::<i32>(Variant(_2.fld1, 0), 1)) = (-1275018411_i32) | 43975879_i32;
place!(Field::<u128>(Variant((*_3).fld1, 0), 0)) = 109608621960070531286441282363562208511_u128 * 209616555391847672252459506939119223297_u128;
place!(Field::<u128>(Variant((*_3).fld1, 0), 0)) = _5 as u128;
(*_3).fld0 = !false;
place!(Field::<i32>(Variant((*_3).fld1, 0), 1)) = (-1871460978_i32);
place!(Field::<i32>(Variant((*_3).fld1, 0), 1)) = 1118016074_i32 ^ 1411735274_i32;
(*_3).fld0 = !true;
place!(Field::<u128>(Variant((*_3).fld1, 0), 0)) = Field::<i32>(Variant((*_3).fld1, 0), 1) as u128;
(*_3).fld2 = 7352643244875610582_u64 * 427134010308202520_u64;
_8 = '\u{b83fa}';
place!(Field::<i8>(Variant((*_3).fld1, 0), 2)) = (*_3).fld3 << Field::<i32>(Variant((*_3).fld1, 0), 1);
(*_3).fld0 = Field::<i8>(Variant((*_3).fld1, 0), 2) < Field::<i8>(Variant((*_3).fld1, 0), 2);
Call((*_3).fld3 = core::intrinsics::bswap(Field::<i8>(Variant((*_3).fld1, 0), 2)), ReturnTo(bb3), UnwindUnreachable())
}
bb26 = {
(*_3).fld2 = 1098824220906510750_u64;
(*_3).fld3 = Field::<i8>(Variant((*_3).fld1, 0), 2);
Goto(bb4)
}
bb27 = {
(*_3).fld2 = 247_u8 as u64;
_3 = core::ptr::addr_of!((*_3));
match RET {
0 => bb5,
1 => bb6,
2 => bb11,
3 => bb12,
6546563463845715655 => bb14,
_ => bb13
}
}
bb28 = {
_3 = core::ptr::addr_of!((*_3));
place!(Field::<i32>(Variant(_2.fld1, 0), 1)) = (-1934724721_i32) - (-1578598717_i32);
(*_3).fld3 = Field::<i8>(Variant((*_3).fld1, 0), 2) | Field::<i8>(Variant((*_3).fld1, 0), 2);
_5 = !(-37_isize);
place!(Field::<u128>(Variant((*_3).fld1, 0), 0)) = !143553356902212587491879399533793983534_u128;
place!(Field::<u128>(Variant((*_3).fld1, 0), 0)) = !255920104204445779620866013006549961402_u128;
place!(Field::<i32>(Variant(_2.fld1, 0), 1)) = (-1275018411_i32) | 43975879_i32;
place!(Field::<u128>(Variant((*_3).fld1, 0), 0)) = 109608621960070531286441282363562208511_u128 * 209616555391847672252459506939119223297_u128;
place!(Field::<u128>(Variant((*_3).fld1, 0), 0)) = _5 as u128;
(*_3).fld0 = !false;
place!(Field::<i32>(Variant((*_3).fld1, 0), 1)) = (-1871460978_i32);
place!(Field::<i32>(Variant((*_3).fld1, 0), 1)) = 1118016074_i32 ^ 1411735274_i32;
(*_3).fld0 = !true;
place!(Field::<u128>(Variant((*_3).fld1, 0), 0)) = Field::<i32>(Variant((*_3).fld1, 0), 1) as u128;
(*_3).fld2 = 7352643244875610582_u64 * 427134010308202520_u64;
_8 = '\u{b83fa}';
place!(Field::<i8>(Variant((*_3).fld1, 0), 2)) = (*_3).fld3 << Field::<i32>(Variant((*_3).fld1, 0), 1);
(*_3).fld0 = Field::<i8>(Variant((*_3).fld1, 0), 2) < Field::<i8>(Variant((*_3).fld1, 0), 2);
Call((*_3).fld3 = core::intrinsics::bswap(Field::<i8>(Variant((*_3).fld1, 0), 2)), ReturnTo(bb3), UnwindUnreachable())
}
bb29 = {
_3 = core::ptr::addr_of!((*_3));
place!(Field::<i32>(Variant(_2.fld1, 0), 1)) = (-1934724721_i32) - (-1578598717_i32);
(*_3).fld3 = Field::<i8>(Variant((*_3).fld1, 0), 2) | Field::<i8>(Variant((*_3).fld1, 0), 2);
_5 = !(-37_isize);
place!(Field::<u128>(Variant((*_3).fld1, 0), 0)) = !143553356902212587491879399533793983534_u128;
place!(Field::<u128>(Variant((*_3).fld1, 0), 0)) = !255920104204445779620866013006549961402_u128;
place!(Field::<i32>(Variant(_2.fld1, 0), 1)) = (-1275018411_i32) | 43975879_i32;
place!(Field::<u128>(Variant((*_3).fld1, 0), 0)) = 109608621960070531286441282363562208511_u128 * 209616555391847672252459506939119223297_u128;
place!(Field::<u128>(Variant((*_3).fld1, 0), 0)) = _5 as u128;
(*_3).fld0 = !false;
place!(Field::<i32>(Variant((*_3).fld1, 0), 1)) = (-1871460978_i32);
place!(Field::<i32>(Variant((*_3).fld1, 0), 1)) = 1118016074_i32 ^ 1411735274_i32;
(*_3).fld0 = !true;
place!(Field::<u128>(Variant((*_3).fld1, 0), 0)) = Field::<i32>(Variant((*_3).fld1, 0), 1) as u128;
(*_3).fld2 = 7352643244875610582_u64 * 427134010308202520_u64;
_8 = '\u{b83fa}';
place!(Field::<i8>(Variant((*_3).fld1, 0), 2)) = (*_3).fld3 << Field::<i32>(Variant((*_3).fld1, 0), 1);
(*_3).fld0 = Field::<i8>(Variant((*_3).fld1, 0), 2) < Field::<i8>(Variant((*_3).fld1, 0), 2);
Call((*_3).fld3 = core::intrinsics::bswap(Field::<i8>(Variant((*_3).fld1, 0), 2)), ReturnTo(bb3), UnwindUnreachable())
}
bb30 = {
(*_3).fld2 = !11904285031903443030_u64;
(*_3).fld2 = !2110482335615838167_u64;
(*_3).fld3 = !Field::<i8>(Variant((*_3).fld1, 0), 2);
(*_3).fld3 = Field::<i8>(Variant((*_3).fld1, 0), 2) ^ Field::<i8>(Variant((*_3).fld1, 0), 2);
(*_3).fld2 = 11254759569736893616_u64;
(*_27) = !29135_u16;
(*_3).fld3 = !Field::<i8>(Variant((*_3).fld1, 0), 2);
Goto(bb31)
}
bb31 = {
(*_3).fld1 = Adt21::Variant0 { fld0: 46283744322621422665629410228621302388_u128,fld1: (-1913857192_i32),fld2: (*_3).fld3 };
(*_27) = 1019_u16 * 21694_u16;
_29 = ((*_3).fld2, 272542478671559765548397130923015071842_u128);
(*_3).fld2 = _29.0 % _29.0;
(*_3).fld0 = (*_16) > (*_16);
(*_28) = [91_u8,197_u8,169_u8,176_u8];
(*_3).fld1 = Adt21::Variant1 { fld0: (*_3).fld0,fld1: _14,fld2: _22,fld3: (*_3).fld3,fld4: _17.1.3,fld5: 3273039653_u32,fld6: (*_16),fld7: _15 };
(*_3).fld1 = Adt21::Variant0 { fld0: _29.1,fld1: 1942213361_i32,fld2: (*_3).fld3 };
_42.0 = !(*_3).fld2;
(*_3).fld2 = !_29.0;
_40 = !19_u8;
(*_3).fld1 = Adt21::Variant1 { fld0: (*_3).fld0,fld1: _14,fld2: _22,fld3: (*_3).fld3,fld4: _17.1.3,fld5: 1246440139_u32,fld6: (*_16),fld7: _15 };
Goto(bb32)
}
bb32 = {
(*_27) = 64212_u16 * 26847_u16;
(*_3).fld1 = Adt21::Variant0 { fld0: _29.1,fld1: (-1319397247_i32),fld2: (*_3).fld3 };
(*_3).fld1 = Adt21::Variant0 { fld0: _29.1,fld1: (-621624487_i32),fld2: (*_3).fld3 };
(*_3).fld0 = _19;
(*_27) = !61067_u16;
(*_28) = [_40,_40,_40,_40];
place!(Field::<i8>(Variant((*_3).fld1, 0), 2)) = (*_16) as i8;
place!(Field::<i8>(Variant((*_3).fld1, 0), 2)) = (*_3).fld3 << _22;
place!(Field::<i32>(Variant((*_3).fld1, 0), 1)) = _12 as i32;
_33 = _15 >> _22;
_42.1 = _11 as u128;
(*_3).fld1 = Adt21::Variant0 { fld0: _42.1,fld1: 402951937_i32,fld2: (*_3).fld3 };
_17.1.3 = 27785_i16 >> _42.1;
_5 = _22 ^ _22;
_16 = core::ptr::addr_of!((*_16));
_43 = core::ptr::addr_of_mut!(_7);
place!(Field::<i8>(Variant((*_3).fld1, 0), 2)) = !(*_3).fld3;
_17.0 = &place!(Field::<u128>(Variant((*_3).fld1, 0), 0));
_27 = core::ptr::addr_of_mut!((*_27));
place!(Field::<i32>(Variant((*_3).fld1, 0), 1)) = !(-838530241_i32);
_42.0 = (*_3).fld0 as u64;
place!(Field::<i32>(Variant((*_3).fld1, 0), 1)) = -(-1217856663_i32);
(*_3).fld2 = _42.0;
Call((*_43) = core::intrinsics::transmute(_17.1.3), ReturnTo(bb33), UnwindUnreachable())
}
bb33 = {
(*_16) = (*_3).fld2 as f32;
_28 = core::ptr::addr_of_mut!((*_28));
Goto(bb34)
}
bb34 = {
(*_28) = [_40,_40,_40,_40];
_12 = _8;
(*_3).fld3 = Field::<i8>(Variant((*_3).fld1, 0), 2) | Field::<i8>(Variant((*_3).fld1, 0), 2);
(*_16) = _2.fld2 as f32;
place!(Field::<i8>(Variant((*_3).fld1, 0), 2)) = (*_3).fld3 * (*_3).fld3;
_7 = 51067_u16 * 52766_u16;
_5 = (*_3).fld3 as isize;
_2.fld2 = _42.0;
(*_28) = [_40,_40,_40,_40];
(*_3).fld1 = Adt21::Variant0 { fld0: _29.1,fld1: (-1821382470_i32),fld2: (*_3).fld3 };
(*_3).fld3 = Field::<i8>(Variant((*_3).fld1, 0), 2) ^ Field::<i8>(Variant((*_3).fld1, 0), 2);
_46 = _22 & _22;
place!(Field::<i32>(Variant((*_3).fld1, 0), 1)) = 1495446015_i32;
(*_3).fld0 = _19;
place!(Field::<u128>(Variant(_2.fld1, 0), 0)) = _29.1 & _29.1;
_17.1.0 = _17.1.3 << (*_43);
_17.1.3 = _17.1.0;
(*_3).fld3 = Field::<i8>(Variant((*_3).fld1, 0), 2) >> (*_3).fld2;
_42.0 = (*_3).fld2;
_50.1.0 = _17.1.0 as f32;
(*_3).fld2 = _42.0 - _42.0;
(*_3).fld2 = Field::<i8>(Variant((*_3).fld1, 0), 2) as u64;
(*_43) = !18699_u16;
place!(Field::<u128>(Variant((*_3).fld1, 0), 0)) = _42.1;
Goto(bb35)
}
bb35 = {
_42.0 = (*_3).fld0 as u64;
_50.1.1 = _17.1.3 - _17.1.0;
_19 = _33 < _33;
_7 = 60951_u16;
(*_16) = _5 as f32;
Goto(bb36)
}
bb36 = {
_50.3 = &RET;
(*_43) = 23534_u16 << _33;
match Field::<i32>(Variant((*_3).fld1, 0), 1) {
0 => bb16,
1495446015 => bb38,
_ => bb37
}
}
bb37 = {
_3 = core::ptr::addr_of!((*_3));
place!(Field::<i32>(Variant(_2.fld1, 0), 1)) = (-1934724721_i32) - (-1578598717_i32);
(*_3).fld3 = Field::<i8>(Variant((*_3).fld1, 0), 2) | Field::<i8>(Variant((*_3).fld1, 0), 2);
_5 = !(-37_isize);
place!(Field::<u128>(Variant((*_3).fld1, 0), 0)) = !143553356902212587491879399533793983534_u128;
place!(Field::<u128>(Variant((*_3).fld1, 0), 0)) = !255920104204445779620866013006549961402_u128;
place!(Field::<i32>(Variant(_2.fld1, 0), 1)) = (-1275018411_i32) | 43975879_i32;
place!(Field::<u128>(Variant((*_3).fld1, 0), 0)) = 109608621960070531286441282363562208511_u128 * 209616555391847672252459506939119223297_u128;
place!(Field::<u128>(Variant((*_3).fld1, 0), 0)) = _5 as u128;
(*_3).fld0 = !false;
place!(Field::<i32>(Variant((*_3).fld1, 0), 1)) = (-1871460978_i32);
place!(Field::<i32>(Variant((*_3).fld1, 0), 1)) = 1118016074_i32 ^ 1411735274_i32;
(*_3).fld0 = !true;
place!(Field::<u128>(Variant((*_3).fld1, 0), 0)) = Field::<i32>(Variant((*_3).fld1, 0), 1) as u128;
(*_3).fld2 = 7352643244875610582_u64 * 427134010308202520_u64;
_8 = '\u{b83fa}';
place!(Field::<i8>(Variant((*_3).fld1, 0), 2)) = (*_3).fld3 << Field::<i32>(Variant((*_3).fld1, 0), 1);
(*_3).fld0 = Field::<i8>(Variant((*_3).fld1, 0), 2) < Field::<i8>(Variant((*_3).fld1, 0), 2);
Call((*_3).fld3 = core::intrinsics::bswap(Field::<i8>(Variant((*_3).fld1, 0), 2)), ReturnTo(bb3), UnwindUnreachable())
}
bb38 = {
(*_3).fld1 = Adt21::Variant0 { fld0: _42.1,fld1: (-704595875_i32),fld2: (*_3).fld3 };
place!(Field::<i32>(Variant((*_3).fld1, 0), 1)) = _8 as i32;
_17.1.2 = [1534609027_u32,2835456163_u32,195423791_u32,3902171232_u32,376230020_u32,3436637208_u32,3673456803_u32,3587118848_u32];
place!(Field::<u128>(Variant((*_3).fld1, 0), 0)) = !_29.1;
_35 = _14 as u128;
(*_3).fld2 = !_29.0;
_29 = ((*_3).fld2, Field::<u128>(Variant((*_3).fld1, 0), 0));
RET = 4835307410736301771_i64 - 3789232949671114766_i64;
(*_3).fld3 = Field::<i8>(Variant((*_3).fld1, 0), 2) - Field::<i8>(Variant((*_3).fld1, 0), 2);
Goto(bb39)
}
bb39 = {
place!(Field::<i8>(Variant((*_3).fld1, 0), 2)) = !(*_3).fld3;
(*_43) = _14 as u16;
(*_16) = _50.1.0 - _50.1.0;
_41 = [(*_3).fld0,(*_3).fld0];
Goto(bb40)
}
bb40 = {
_1 = &_50.1.1;
(*_3).fld0 = _19;
place!(Field::<i8>(Variant((*_3).fld1, 0), 2)) = (*_3).fld3;
_54 = -RET;
(*_43) = (*_16) as u16;
(*_28) = [_40,_40,_40,_40];
(*_3).fld0 = _19;
(*_16) = _50.1.0 - _50.1.0;
place!(Field::<i8>(Variant((*_3).fld1, 0), 2)) = (*_3).fld3 | (*_3).fld3;
Goto(bb41)
}
bb41 = {
(*_28) = [_40,_40,_40,_40];
_45 = -_14;
_39 = core::ptr::addr_of_mut!(_48);
_54 = RET << _17.1.3;
_54 = RET;
(*_3).fld0 = !_19;
(*_16) = _50.1.0 * _50.1.0;
(*_3).fld0 = _19;
(*_28) = [_40,_40,_40,_40];
_29.0 = !_42.0;
_8 = _12;
(*_3).fld1 = Adt21::Variant0 { fld0: _35,fld1: (-819312174_i32),fld2: (*_3).fld3 };
(*_3).fld1 = Adt21::Variant1 { fld0: (*_3).fld0,fld1: _45,fld2: _46,fld3: (*_3).fld3,fld4: (*_1),fld5: 3669704145_u32,fld6: (*_16),fld7: _15 };
place!(Field::<usize>(Variant((*_3).fld1, 1), 7)) = _33 - _33;
_34 = &mut _41;
_61 = -Field::<i8>(Variant((*_3).fld1, 1), 3);
(*_3).fld0 = (*_16) <= (*_16);
place!(Field::<usize>(Variant((*_3).fld1, 1), 7)) = _33 - _33;
place!(Field::<isize>(Variant((*_3).fld1, 1), 2)) = -_46;
(*_3).fld0 = Field::<bool>(Variant((*_3).fld1, 1), 0) & Field::<bool>(Variant((*_3).fld1, 1), 0);
place!(Field::<usize>(Variant((*_3).fld1, 1), 7)) = _33;
Goto(bb42)
}
bb42 = {
(*_3).fld1 = Adt21::Variant0 { fld0: _35,fld1: 603129535_i32,fld2: (*_3).fld3 };
_57 = _22;
_50.2.1 = RET + _54;
(*_3).fld0 = _19;
_50.3 = &_50.2.1;
(*_3).fld3 = Field::<i8>(Variant((*_3).fld1, 0), 2) ^ Field::<i8>(Variant((*_3).fld1, 0), 2);
_18 = [_8,_12,_8,_12,_8,_8,_8,_8];
place!(Field::<i32>(Variant((*_3).fld1, 0), 1)) = 927405604_i32;
Goto(bb43)
}
bb43 = {
_44.0 = [_11,_11];
(*_3).fld0 = (*_1) < (*_1);
(*_43) = 44104_u16 << _35;
place!(Field::<u128>(Variant((*_3).fld1, 0), 0)) = _45 as u128;
(*_39) = -_11;
(*_3).fld3 = Field::<i8>(Variant((*_3).fld1, 0), 2) ^ Field::<i8>(Variant((*_3).fld1, 0), 2);
_28 = core::ptr::addr_of_mut!((*_28));
_62 = _19;
(*_43) = !43971_u16;
_66 = (948166198_u32,);
(*_43) = _66.0 as u16;
place!(Field::<i8>(Variant((*_3).fld1, 0), 2)) = (*_3).fld3 >> _46;
(*_3).fld2 = !_29.0;
_2.fld3 = Field::<i8>(Variant((*_3).fld1, 0), 2) << (*_1);
(*_43) = 17839_u16 >> (*_3).fld3;
_55 = (*_28);
(*_39) = !_11;
_8 = _12;
_32 = [Field::<i8>(Variant((*_3).fld1, 0), 2),Field::<i8>(Variant((*_3).fld1, 0), 2)];
match Field::<i32>(Variant((*_3).fld1, 0), 1) {
0 => bb1,
1 => bb7,
2 => bb32,
3 => bb17,
4 => bb44,
5 => bb45,
6 => bb46,
927405604 => bb48,
_ => bb47
}
}
bb44 = {
_3 = core::ptr::addr_of!((*_3));
place!(Field::<i32>(Variant(_2.fld1, 0), 1)) = (-1934724721_i32) - (-1578598717_i32);
(*_3).fld3 = Field::<i8>(Variant((*_3).fld1, 0), 2) | Field::<i8>(Variant((*_3).fld1, 0), 2);
_5 = !(-37_isize);
place!(Field::<u128>(Variant((*_3).fld1, 0), 0)) = !143553356902212587491879399533793983534_u128;
place!(Field::<u128>(Variant((*_3).fld1, 0), 0)) = !255920104204445779620866013006549961402_u128;
place!(Field::<i32>(Variant(_2.fld1, 0), 1)) = (-1275018411_i32) | 43975879_i32;
place!(Field::<u128>(Variant((*_3).fld1, 0), 0)) = 109608621960070531286441282363562208511_u128 * 209616555391847672252459506939119223297_u128;
place!(Field::<u128>(Variant((*_3).fld1, 0), 0)) = _5 as u128;
(*_3).fld0 = !false;
place!(Field::<i32>(Variant((*_3).fld1, 0), 1)) = (-1871460978_i32);
place!(Field::<i32>(Variant((*_3).fld1, 0), 1)) = 1118016074_i32 ^ 1411735274_i32;
(*_3).fld0 = !true;
place!(Field::<u128>(Variant((*_3).fld1, 0), 0)) = Field::<i32>(Variant((*_3).fld1, 0), 1) as u128;
(*_3).fld2 = 7352643244875610582_u64 * 427134010308202520_u64;
_8 = '\u{b83fa}';
place!(Field::<i8>(Variant((*_3).fld1, 0), 2)) = (*_3).fld3 << Field::<i32>(Variant((*_3).fld1, 0), 1);
(*_3).fld0 = Field::<i8>(Variant((*_3).fld1, 0), 2) < Field::<i8>(Variant((*_3).fld1, 0), 2);
Call((*_3).fld3 = core::intrinsics::bswap(Field::<i8>(Variant((*_3).fld1, 0), 2)), ReturnTo(bb3), UnwindUnreachable())
}
bb45 = {
(*_28) = [_40,_40,_40,_40];
_45 = -_14;
_39 = core::ptr::addr_of_mut!(_48);
_54 = RET << _17.1.3;
_54 = RET;
(*_3).fld0 = !_19;
(*_16) = _50.1.0 * _50.1.0;
(*_3).fld0 = _19;
(*_28) = [_40,_40,_40,_40];
_29.0 = !_42.0;
_8 = _12;
(*_3).fld1 = Adt21::Variant0 { fld0: _35,fld1: (-819312174_i32),fld2: (*_3).fld3 };
(*_3).fld1 = Adt21::Variant1 { fld0: (*_3).fld0,fld1: _45,fld2: _46,fld3: (*_3).fld3,fld4: (*_1),fld5: 3669704145_u32,fld6: (*_16),fld7: _15 };
place!(Field::<usize>(Variant((*_3).fld1, 1), 7)) = _33 - _33;
_34 = &mut _41;
_61 = -Field::<i8>(Variant((*_3).fld1, 1), 3);
(*_3).fld0 = (*_16) <= (*_16);
place!(Field::<usize>(Variant((*_3).fld1, 1), 7)) = _33 - _33;
place!(Field::<isize>(Variant((*_3).fld1, 1), 2)) = -_46;
(*_3).fld0 = Field::<bool>(Variant((*_3).fld1, 1), 0) & Field::<bool>(Variant((*_3).fld1, 1), 0);
place!(Field::<usize>(Variant((*_3).fld1, 1), 7)) = _33;
Goto(bb42)
}
bb46 = {
_2.fld3 = -Field::<i8>(Variant(_2.fld1, 0), 2);
_2.fld0 = false;
place!(Field::<u128>(Variant(_2.fld1, 0), 0)) = 166695586101368990003060982858003281706_u128;
place!(Field::<i8>(Variant(_2.fld1, 0), 2)) = _2.fld3;
_5 = 47_isize & (-9223372036854775808_isize);
RET = 6546563463845715655_i64;
_2.fld3 = Field::<i8>(Variant(_2.fld1, 0), 2) >> Field::<i8>(Variant(_2.fld1, 0), 2);
_2.fld1 = Adt21::Variant0 { fld0: 196718857085293825338195854263456265281_u128,fld1: 736737551_i32,fld2: _2.fld3 };
_3 = core::ptr::addr_of!(_2);
place!(Field::<i8>(Variant(_2.fld1, 0), 2)) = (*_3).fld3 << (*_3).fld3;
(*_3).fld0 = false;
place!(Field::<i8>(Variant((*_3).fld1, 0), 2)) = (*_3).fld0 as i8;
(*_3).fld2 = 11015704025903207688_u64;
(*_3).fld2 = 11112_i16 as u64;
place!(Field::<i32>(Variant((*_3).fld1, 0), 1)) = !(-385191832_i32);
place!(Field::<u128>(Variant((*_3).fld1, 0), 0)) = 174782804658046704017441464475867554021_u128;
Goto(bb2)
}
bb47 = {
(*_3).fld2 = 247_u8 as u64;
_3 = core::ptr::addr_of!((*_3));
match RET {
0 => bb5,
1 => bb6,
2 => bb11,
3 => bb12,
6546563463845715655 => bb14,
_ => bb13
}
}
bb48 = {
_2.fld2 = Field::<i32>(Variant((*_3).fld1, 0), 1) as u64;
place!(Field::<i8>(Variant((*_3).fld1, 0), 2)) = RET as i8;
_39 = core::ptr::addr_of_mut!((*_39));
(*_16) = _50.1.0 - _50.1.0;
_17.1.0 = (*_1);
_49 = [(*_39),(*_39)];
place!(Field::<u128>(Variant((*_3).fld1, 0), 0)) = !_35;
(*_3).fld0 = _62 | _19;
_50.1 = ((*_16), _17.1.3, _15, _40);
_67 = core::ptr::addr_of!((*_39));
place!(Field::<i8>(Variant((*_3).fld1, 0), 2)) = (*_3).fld3 >> (*_3).fld3;
_72 = (_17.1.0, Move(_50.3), _17.1.2, _17.1.0);
_51 = core::ptr::addr_of_mut!(_47);
(*_51).fld0 = (_50.2.1,);
(*_51).fld1 = _17.1.2;
(*_51).fld1 = [_66.0,_66.0,_66.0,_66.0,_66.0,_66.0,_66.0,_66.0];
(*_67) = _11;
Goto(bb49)
}
bb49 = {
_2.fld2 = _42.0;
place!(Field::<u128>(Variant((*_3).fld1, 0), 0)) = !_35;
(*_28) = [_50.1.3,_50.1.3,_50.1.3,_40];
(*_3).fld2 = _29.0 | _42.0;
Goto(bb50)
}
bb50 = {
Call(_76 = dump_var(Move(_22), Move(_66), Move(_32), Move(_7)), ReturnTo(bb51), UnwindUnreachable())
}
bb51 = {
Call(_76 = dump_var(Move(_8), Move(_29), Move(_31), Move(_54)), ReturnTo(bb52), UnwindUnreachable())
}
bb52 = {
Call(_76 = dump_var(Move(_5), Move(_35), Move(_46), Move(_62)), ReturnTo(bb53), UnwindUnreachable())
}
bb53 = {
Call(_76 = dump_var(Move(_19), Move(_41), _77, _77), ReturnTo(bb54), UnwindUnreachable())
}
bb54 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn19(mut _1: *const Adt22,mut _2: isize,mut _3: f32,mut _4: i16,mut _5: bool,mut _6: u64,mut _7: *const i128,mut _8: i64,mut _9: *const f32,mut _10: &'static char,mut _11: isize) -> i8 {
mir! {
type RET = i8;
let _12: u128;
let _13: char;
let _14: i128;
let _15: (Adt22, ((i64,), &'static char), Adt52, *mut [isize; 5]);
let _16: isize;
let _17: bool;
let _18: ((u32,), &'static char, bool, bool);
let _19: isize;
let _20: isize;
let _21: f32;
let _22: &'static mut [bool; 2];
let _23: u128;
let _24: i128;
let _25: *mut Adt73;
let _26: *mut [u8; 4];
let _27: u64;
let _28: i32;
let _29: u128;
let _30: [char; 8];
let _31: isize;
let _32: Adt46;
let _33: bool;
let _34: &'static mut [bool; 2];
let _35: i64;
let _36: ((&'static mut *mut i128,), (f32, i16, usize, u8), (&'static &'static char, i64, i16, i64), &'static i64);
let _37: char;
let _38: [i32; 5];
let _39: (u32,);
let _40: isize;
let _41: *mut [u8; 4];
let _42: &'static mut &'static mut (i64,);
let _43: f32;
let _44: u64;
let _45: (i64,);
let _46: isize;
let _47: ([i128; 1], [i8; 2], *const Adt22);
let _48: usize;
let _49: (Adt22, ((i64,), &'static char), Adt52, *mut [isize; 5]);
let _50: f64;
let _51: &'static char;
let _52: &'static mut (i64,);
let _53: Adt73;
let _54: isize;
let _55: *const f32;
let _56: *const (f32, i16, usize, u8);
let _57: (i64,);
let _58: [i8; 3];
let _59: i16;
let _60: (u32,);
let _61: &'static u128;
let _62: (&'static u128, (i16, &'static i64, [u32; 8], i16), &'static u32);
let _63: f64;
let _64: *const i32;
let _65: isize;
let _66: [char; 8];
let _67: *mut i128;
let _68: i128;
let _69: &'static i64;
let _70: *const Adt22;
let _71: isize;
let _72: i128;
let _73: u64;
let _74: char;
let _75: &'static &'static char;
let _76: isize;
let _77: *const i128;
let _78: &'static mut u32;
let _79: isize;
let _80: &'static u32;
let _81: [isize; 5];
let _82: i128;
let _83: i64;
let _84: &'static mut char;
let _85: f32;
let _86: *mut isize;
let _87: isize;
let _88: isize;
let _89: (f32, i16, usize, u8);
let _90: i128;
let _91: Adt52;
let _92: i32;
let _93: [i8; 3];
let _94: [bool; 2];
let _95: *const i128;
let _96: &'static i64;
let _97: u8;
let _98: ();
let _99: ();
{
_3 = _2 as f32;
_2 = _11 | _11;
_2 = _11 * _11;
_4 = (-17060_i16) * (-8297_i16);
_2 = 185_u8 as isize;
_6 = 3197630957527865099_u64 ^ 10486029187982072709_u64;
_9 = core::ptr::addr_of!(_3);
(*_9) = 9675403199510407083_usize as f32;
(*_9) = (-1511764378_i32) as f32;
Goto(bb1)
}
bb1 = {
(*_9) = _6 as f32;
_14 = '\u{a210}' as i128;
_13 = '\u{58995}';
_4 = 314460498_i32 as i16;
_16 = _11 & _11;
_15.0.fld2 = 2902939418_u32 as u64;
(*_9) = _14 as f32;
(*_9) = _6 as f32;
_2 = _8 as isize;
_15.1.0.0 = _8 + _8;
_15.0.fld1 = Adt21::Variant0 { fld0: 120034910902584926625655474731689044186_u128,fld1: (-2066945086_i32),fld2: 29_i8 };
RET = -(-33_i8);
(*_9) = _11 as f32;
(*_9) = _14 as f32;
_17 = _5 | _5;
(*_9) = 905565841_u32 as f32;
_20 = _11 * _11;
_15.0.fld2 = _6 | _6;
_18.0 = (3325258367_u32,);
(*_9) = 6450524525995444338_usize as f32;
_15.1.0.0 = _8;
(*_9) = _4 as f32;
Call(_11 = core::intrinsics::bswap(_20), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
(*_9) = 9225484839420211826_usize as f32;
(*_9) = _14 as f32;
(*_9) = 18828_u16 as f32;
place!(Field::<i8>(Variant(_15.0.fld1, 0), 2)) = RET + RET;
_18.3 = _16 <= _20;
_18.1 = &_13;
_15.1.1 = Move(_18.1);
_6 = !_15.0.fld2;
_15.0.fld1 = Adt21::Variant0 { fld0: 84004418483253721330098157199954281113_u128,fld1: (-1489285688_i32),fld2: RET };
_10 = &_13;
_15.1.0.0 = 0_usize as i64;
_4 = (-23319_i16) - 6534_i16;
place!(Field::<i32>(Variant(_15.0.fld1, 0), 1)) = 924682692_i32 & 325529723_i32;
_18.1 = &(*_10);
(*_9) = _11 as f32;
_4 = (-27561_i16) ^ (-31349_i16);
(*_9) = 239423232091713230789654779063815245584_u128 as f32;
_10 = Move(_18.1);
_23 = 244985217358883522470943171854031430165_u128;
match _23 {
244985217358883522470943171854031430165 => bb4,
_ => bb3
}
}
bb3 = {
(*_9) = _6 as f32;
_14 = '\u{a210}' as i128;
_13 = '\u{58995}';
_4 = 314460498_i32 as i16;
_16 = _11 & _11;
_15.0.fld2 = 2902939418_u32 as u64;
(*_9) = _14 as f32;
(*_9) = _6 as f32;
_2 = _8 as isize;
_15.1.0.0 = _8 + _8;
_15.0.fld1 = Adt21::Variant0 { fld0: 120034910902584926625655474731689044186_u128,fld1: (-2066945086_i32),fld2: 29_i8 };
RET = -(-33_i8);
(*_9) = _11 as f32;
(*_9) = _14 as f32;
_17 = _5 | _5;
(*_9) = 905565841_u32 as f32;
_20 = _11 * _11;
_15.0.fld2 = _6 | _6;
_18.0 = (3325258367_u32,);
(*_9) = 6450524525995444338_usize as f32;
_15.1.0.0 = _8;
(*_9) = _4 as f32;
Call(_11 = core::intrinsics::bswap(_20), ReturnTo(bb2), UnwindUnreachable())
}
bb4 = {
(*_9) = _4 as f32;
_15.0.fld0 = !_17;
(*_9) = _8 as f32;
_21 = (*_9) - (*_9);
(*_9) = -_21;
_15.1.1 = &_13;
_17 = !_15.0.fld0;
_12 = _23 ^ _23;
place!(Field::<u128>(Variant(_15.0.fld1, 0), 0)) = _12 ^ _12;
match _18.0.0 {
0 => bb2,
1 => bb5,
2 => bb6,
3325258367 => bb8,
_ => bb7
}
}
bb5 = {
(*_9) = _6 as f32;
_14 = '\u{a210}' as i128;
_13 = '\u{58995}';
_4 = 314460498_i32 as i16;
_16 = _11 & _11;
_15.0.fld2 = 2902939418_u32 as u64;
(*_9) = _14 as f32;
(*_9) = _6 as f32;
_2 = _8 as isize;
_15.1.0.0 = _8 + _8;
_15.0.fld1 = Adt21::Variant0 { fld0: 120034910902584926625655474731689044186_u128,fld1: (-2066945086_i32),fld2: 29_i8 };
RET = -(-33_i8);
(*_9) = _11 as f32;
(*_9) = _14 as f32;
_17 = _5 | _5;
(*_9) = 905565841_u32 as f32;
_20 = _11 * _11;
_15.0.fld2 = _6 | _6;
_18.0 = (3325258367_u32,);
(*_9) = 6450524525995444338_usize as f32;
_15.1.0.0 = _8;
(*_9) = _4 as f32;
Call(_11 = core::intrinsics::bswap(_20), ReturnTo(bb2), UnwindUnreachable())
}
bb6 = {
(*_9) = 9225484839420211826_usize as f32;
(*_9) = _14 as f32;
(*_9) = 18828_u16 as f32;
place!(Field::<i8>(Variant(_15.0.fld1, 0), 2)) = RET + RET;
_18.3 = _16 <= _20;
_18.1 = &_13;
_15.1.1 = Move(_18.1);
_6 = !_15.0.fld2;
_15.0.fld1 = Adt21::Variant0 { fld0: 84004418483253721330098157199954281113_u128,fld1: (-1489285688_i32),fld2: RET };
_10 = &_13;
_15.1.0.0 = 0_usize as i64;
_4 = (-23319_i16) - 6534_i16;
place!(Field::<i32>(Variant(_15.0.fld1, 0), 1)) = 924682692_i32 & 325529723_i32;
_18.1 = &(*_10);
(*_9) = _11 as f32;
_4 = (-27561_i16) ^ (-31349_i16);
(*_9) = 239423232091713230789654779063815245584_u128 as f32;
_10 = Move(_18.1);
_23 = 244985217358883522470943171854031430165_u128;
match _23 {
244985217358883522470943171854031430165 => bb4,
_ => bb3
}
}
bb7 = {
(*_9) = _6 as f32;
_14 = '\u{a210}' as i128;
_13 = '\u{58995}';
_4 = 314460498_i32 as i16;
_16 = _11 & _11;
_15.0.fld2 = 2902939418_u32 as u64;
(*_9) = _14 as f32;
(*_9) = _6 as f32;
_2 = _8 as isize;
_15.1.0.0 = _8 + _8;
_15.0.fld1 = Adt21::Variant0 { fld0: 120034910902584926625655474731689044186_u128,fld1: (-2066945086_i32),fld2: 29_i8 };
RET = -(-33_i8);
(*_9) = _11 as f32;
(*_9) = _14 as f32;
_17 = _5 | _5;
(*_9) = 905565841_u32 as f32;
_20 = _11 * _11;
_15.0.fld2 = _6 | _6;
_18.0 = (3325258367_u32,);
(*_9) = 6450524525995444338_usize as f32;
_15.1.0.0 = _8;
(*_9) = _4 as f32;
Call(_11 = core::intrinsics::bswap(_20), ReturnTo(bb2), UnwindUnreachable())
}
bb8 = {
_3 = _15.0.fld2 as f32;
(*_9) = _21;
place!(Field::<i8>(Variant(_15.0.fld1, 0), 2)) = !RET;
_24 = _14;
_18.1 = &_13;
_17 = (*_9) < (*_9);
_15.1.0 = (_8,);
(*_9) = 2711659313235664463_usize as f32;
(*_9) = _21 + _21;
_15.0.fld3 = _14 as i8;
Goto(bb9)
}
bb9 = {
_15.1.1 = Move(_18.1);
_7 = core::ptr::addr_of!(_24);
(*_7) = _14;
(*_7) = !_14;
(*_7) = !_14;
_19 = 2_usize as isize;
(*_9) = _21;
_17 = (*_9) <= (*_9);
(*_7) = -_14;
_24 = !_14;
(*_7) = _14;
(*_9) = _21 + _21;
place!(Field::<u128>(Variant(_15.0.fld1, 0), 0)) = _23 * _23;
_11 = !_20;
_30 = [_13,_13,_13,_13,_13,_13,_13,_13];
(*_9) = _21 * _21;
(*_7) = _14 - _14;
_15.1.1 = &_13;
(*_9) = _21 * _21;
_18.2 = _15.0.fld0;
_18.0 = (1707639827_u32,);
_16 = _13 as isize;
(*_9) = _21 - _21;
_3 = _21;
(*_9) = _21 * _21;
_8 = _15.1.0.0;
_21 = (*_9) + (*_9);
match _23 {
0 => bb1,
1 => bb8,
2 => bb3,
3 => bb6,
4 => bb10,
5 => bb11,
6 => bb12,
244985217358883522470943171854031430165 => bb14,
_ => bb13
}
}
bb10 = {
_3 = _15.0.fld2 as f32;
(*_9) = _21;
place!(Field::<i8>(Variant(_15.0.fld1, 0), 2)) = !RET;
_24 = _14;
_18.1 = &_13;
_17 = (*_9) < (*_9);
_15.1.0 = (_8,);
(*_9) = 2711659313235664463_usize as f32;
(*_9) = _21 + _21;
_15.0.fld3 = _14 as i8;
Goto(bb9)
}
bb11 = {
(*_9) = _6 as f32;
_14 = '\u{a210}' as i128;
_13 = '\u{58995}';
_4 = 314460498_i32 as i16;
_16 = _11 & _11;
_15.0.fld2 = 2902939418_u32 as u64;
(*_9) = _14 as f32;
(*_9) = _6 as f32;
_2 = _8 as isize;
_15.1.0.0 = _8 + _8;
_15.0.fld1 = Adt21::Variant0 { fld0: 120034910902584926625655474731689044186_u128,fld1: (-2066945086_i32),fld2: 29_i8 };
RET = -(-33_i8);
(*_9) = _11 as f32;
(*_9) = _14 as f32;
_17 = _5 | _5;
(*_9) = 905565841_u32 as f32;
_20 = _11 * _11;
_15.0.fld2 = _6 | _6;
_18.0 = (3325258367_u32,);
(*_9) = 6450524525995444338_usize as f32;
_15.1.0.0 = _8;
(*_9) = _4 as f32;
Call(_11 = core::intrinsics::bswap(_20), ReturnTo(bb2), UnwindUnreachable())
}
bb12 = {
(*_9) = _6 as f32;
_14 = '\u{a210}' as i128;
_13 = '\u{58995}';
_4 = 314460498_i32 as i16;
_16 = _11 & _11;
_15.0.fld2 = 2902939418_u32 as u64;
(*_9) = _14 as f32;
(*_9) = _6 as f32;
_2 = _8 as isize;
_15.1.0.0 = _8 + _8;
_15.0.fld1 = Adt21::Variant0 { fld0: 120034910902584926625655474731689044186_u128,fld1: (-2066945086_i32),fld2: 29_i8 };
RET = -(-33_i8);
(*_9) = _11 as f32;
(*_9) = _14 as f32;
_17 = _5 | _5;
(*_9) = 905565841_u32 as f32;
_20 = _11 * _11;
_15.0.fld2 = _6 | _6;
_18.0 = (3325258367_u32,);
(*_9) = 6450524525995444338_usize as f32;
_15.1.0.0 = _8;
(*_9) = _4 as f32;
Call(_11 = core::intrinsics::bswap(_20), ReturnTo(bb2), UnwindUnreachable())
}
bb13 = {
(*_9) = _6 as f32;
_14 = '\u{a210}' as i128;
_13 = '\u{58995}';
_4 = 314460498_i32 as i16;
_16 = _11 & _11;
_15.0.fld2 = 2902939418_u32 as u64;
(*_9) = _14 as f32;
(*_9) = _6 as f32;
_2 = _8 as isize;
_15.1.0.0 = _8 + _8;
_15.0.fld1 = Adt21::Variant0 { fld0: 120034910902584926625655474731689044186_u128,fld1: (-2066945086_i32),fld2: 29_i8 };
RET = -(-33_i8);
(*_9) = _11 as f32;
(*_9) = _14 as f32;
_17 = _5 | _5;
(*_9) = 905565841_u32 as f32;
_20 = _11 * _11;
_15.0.fld2 = _6 | _6;
_18.0 = (3325258367_u32,);
(*_9) = 6450524525995444338_usize as f32;
_15.1.0.0 = _8;
(*_9) = _4 as f32;
Call(_11 = core::intrinsics::bswap(_20), ReturnTo(bb2), UnwindUnreachable())
}
bb14 = {
_19 = _20;
(*_7) = _14;
(*_7) = _14 + _14;
_31 = _19;
(*_7) = -_14;
(*_9) = -_21;
_12 = !Field::<u128>(Variant(_15.0.fld1, 0), 0);
_3 = _21 + _21;
_28 = -Field::<i32>(Variant(_15.0.fld1, 0), 1);
(*_9) = _21;
_29 = _13 as u128;
_10 = &_13;
_18.1 = Move(_15.1.1);
(*_9) = -_21;
_9 = core::ptr::addr_of!((*_9));
_9 = core::ptr::addr_of!(_21);
_36.2.0 = &_10;
match _18.0.0 {
0 => bb15,
1707639827 => bb17,
_ => bb16
}
}
bb15 = {
_3 = _15.0.fld2 as f32;
(*_9) = _21;
place!(Field::<i8>(Variant(_15.0.fld1, 0), 2)) = !RET;
_24 = _14;
_18.1 = &_13;
_17 = (*_9) < (*_9);
_15.1.0 = (_8,);
(*_9) = 2711659313235664463_usize as f32;
(*_9) = _21 + _21;
_15.0.fld3 = _14 as i8;
Goto(bb9)
}
bb16 = {
(*_9) = _6 as f32;
_14 = '\u{a210}' as i128;
_13 = '\u{58995}';
_4 = 314460498_i32 as i16;
_16 = _11 & _11;
_15.0.fld2 = 2902939418_u32 as u64;
(*_9) = _14 as f32;
(*_9) = _6 as f32;
_2 = _8 as isize;
_15.1.0.0 = _8 + _8;
_15.0.fld1 = Adt21::Variant0 { fld0: 120034910902584926625655474731689044186_u128,fld1: (-2066945086_i32),fld2: 29_i8 };
RET = -(-33_i8);
(*_9) = _11 as f32;
(*_9) = _14 as f32;
_17 = _5 | _5;
(*_9) = 905565841_u32 as f32;
_20 = _11 * _11;
_15.0.fld2 = _6 | _6;
_18.0 = (3325258367_u32,);
(*_9) = 6450524525995444338_usize as f32;
_15.1.0.0 = _8;
(*_9) = _4 as f32;
Call(_11 = core::intrinsics::bswap(_20), ReturnTo(bb2), UnwindUnreachable())
}
bb17 = {
_36.2.1 = !_15.1.0.0;
(*_9) = 0_usize as f32;
_36.1 = ((*_9), _4, 6248251708813138427_usize, 221_u8);
place!(Field::<i32>(Variant(_15.0.fld1, 0), 1)) = _28 * _28;
(*_9) = _3 * _3;
(*_9) = -_3;
Goto(bb18)
}
bb18 = {
_36.1.3 = RET as u8;
(*_7) = 22197_u16 as i128;
_16 = _11 ^ _19;
_24 = _6 as i128;
_40 = -_31;
_17 = (*_9) > (*_9);
(*_9) = -_3;
_4 = _36.1.1 << (*_7);
(*_7) = !_14;
_38 = [Field::<i32>(Variant(_15.0.fld1, 0), 1),_28,Field::<i32>(Variant(_15.0.fld1, 0), 1),Field::<i32>(Variant(_15.0.fld1, 0), 1),Field::<i32>(Variant(_15.0.fld1, 0), 1)];
_36.2.3 = _36.2.1;
_36.1.2 = 2_usize - 6_usize;
(*_9) = _3 - _3;
(*_7) = _6 as i128;
_43 = (*_9);
(*_9) = -_36.1.0;
(*_7) = _14 << _31;
(*_9) = -_43;
_27 = _15.0.fld2 >> (*_7);
(*_7) = _14;
(*_7) = _14 - _14;
(*_7) = _14;
_38 = [_28,Field::<i32>(Variant(_15.0.fld1, 0), 1),_28,_28,Field::<i32>(Variant(_15.0.fld1, 0), 1)];
(*_9) = _3 * _3;
_11 = -_19;
(*_7) = _14 | _14;
Goto(bb19)
}
bb19 = {
_27 = _36.1.3 as u64;
(*_9) = _43 - _43;
(*_7) = _14 - _14;
_39 = (_18.0.0,);
_36.2.2 = _4 << _16;
_7 = core::ptr::addr_of!((*_7));
_13 = '\u{78911}';
_23 = !_12;
(*_9) = _3 * _43;
_44 = !_27;
place!(Field::<i32>(Variant(_15.0.fld1, 0), 1)) = _28 | _28;
_6 = !_44;
(*_7) = _14 - _14;
_48 = _36.1.2;
_45 = _15.1.0;
_36.2.3 = _15.1.0.0 & _36.2.1;
(*_7) = _14 & _14;
_15.1.0 = (_36.2.3,);
(*_7) = Field::<i32>(Variant(_15.0.fld1, 0), 1) as i128;
(*_9) = _43;
(*_7) = _14 + _14;
(*_7) = _14 & _14;
(*_7) = -_14;
_51 = &_13;
_29 = _23 & _12;
Goto(bb20)
}
bb20 = {
_49.1.0.0 = _36.2.1 * _36.2.3;
_15.1.1 = &(*_51);
_15.0.fld1 = Adt21::Variant0 { fld0: _12,fld1: _28,fld2: RET };
_15.0.fld2 = _6 ^ _27;
(*_9) = _43 - _43;
_45 = (_49.1.0.0,);
_45 = (_36.2.3,);
(*_7) = _14;
_47.2 = core::ptr::addr_of!(_15.0);
_33 = _31 < _40;
_49.0.fld1 = _15.0.fld1;
(*_7) = _14;
_53.fld0.0 = _49.1.0.0 - _49.1.0.0;
_49.2 = Adt52::Variant1 { fld0: _4,fld1: _15.0.fld1,fld2: (*_7),fld3: _38 };
_49.1.0.0 = _53.fld0.0 + _36.2.3;
place!(Field::<u128>(Variant(_49.0.fld1, 0), 0)) = (*_9) as u128;
(*_7) = !_14;
_49.1 = (_53.fld0, Move(_51));
place!(Field::<i8>(Variant(_15.0.fld1, 0), 2)) = _5 as i8;
_49.1.0 = (_15.1.0.0,);
_36.3 = &_45.0;
(*_7) = -_14;
_53.fld0.0 = !_49.1.0.0;
_53.fld0.0 = _36.2.1;
_14 = (*_7);
(*_9) = _3 * _43;
(*_7) = _14 ^ Field::<i128>(Variant(_49.2, 1), 2);
match _18.0.0 {
0 => bb12,
1 => bb8,
2 => bb21,
3 => bb22,
1707639827 => bb24,
_ => bb23
}
}
bb21 = {
(*_9) = _4 as f32;
_15.0.fld0 = !_17;
(*_9) = _8 as f32;
_21 = (*_9) - (*_9);
(*_9) = -_21;
_15.1.1 = &_13;
_17 = !_15.0.fld0;
_12 = _23 ^ _23;
place!(Field::<u128>(Variant(_15.0.fld1, 0), 0)) = _12 ^ _12;
match _18.0.0 {
0 => bb2,
1 => bb5,
2 => bb6,
3325258367 => bb8,
_ => bb7
}
}
bb22 = {
_36.1.3 = RET as u8;
(*_7) = 22197_u16 as i128;
_16 = _11 ^ _19;
_24 = _6 as i128;
_40 = -_31;
_17 = (*_9) > (*_9);
(*_9) = -_3;
_4 = _36.1.1 << (*_7);
(*_7) = !_14;
_38 = [Field::<i32>(Variant(_15.0.fld1, 0), 1),_28,Field::<i32>(Variant(_15.0.fld1, 0), 1),Field::<i32>(Variant(_15.0.fld1, 0), 1),Field::<i32>(Variant(_15.0.fld1, 0), 1)];
_36.2.3 = _36.2.1;
_36.1.2 = 2_usize - 6_usize;
(*_9) = _3 - _3;
(*_7) = _6 as i128;
_43 = (*_9);
(*_9) = -_36.1.0;
(*_7) = _14 << _31;
(*_9) = -_43;
_27 = _15.0.fld2 >> (*_7);
(*_7) = _14;
(*_7) = _14 - _14;
(*_7) = _14;
_38 = [_28,Field::<i32>(Variant(_15.0.fld1, 0), 1),_28,_28,Field::<i32>(Variant(_15.0.fld1, 0), 1)];
(*_9) = _3 * _3;
_11 = -_19;
(*_7) = _14 | _14;
Goto(bb19)
}
bb23 = {
(*_9) = _6 as f32;
_14 = '\u{a210}' as i128;
_13 = '\u{58995}';
_4 = 314460498_i32 as i16;
_16 = _11 & _11;
_15.0.fld2 = 2902939418_u32 as u64;
(*_9) = _14 as f32;
(*_9) = _6 as f32;
_2 = _8 as isize;
_15.1.0.0 = _8 + _8;
_15.0.fld1 = Adt21::Variant0 { fld0: 120034910902584926625655474731689044186_u128,fld1: (-2066945086_i32),fld2: 29_i8 };
RET = -(-33_i8);
(*_9) = _11 as f32;
(*_9) = _14 as f32;
_17 = _5 | _5;
(*_9) = 905565841_u32 as f32;
_20 = _11 * _11;
_15.0.fld2 = _6 | _6;
_18.0 = (3325258367_u32,);
(*_9) = 6450524525995444338_usize as f32;
_15.1.0.0 = _8;
(*_9) = _4 as f32;
Call(_11 = core::intrinsics::bswap(_20), ReturnTo(bb2), UnwindUnreachable())
}
bb24 = {
_36.2.3 = (*_7) as i64;
_20 = _19 * _40;
place!(Field::<u128>(Variant(place!(Field::<Adt21>(Variant(_49.2, 1), 1)), 0), 0)) = _8 as u128;
_18.2 = _17 & _18.3;
_5 = !_33;
place!(Field::<[i32; 5]>(Variant(_49.2, 1), 3)) = [Field::<i32>(Variant(Field::<Adt21>(Variant(_49.2, 1), 1), 0), 1),Field::<i32>(Variant(Field::<Adt21>(Variant(_49.2, 1), 1), 0), 1),Field::<i32>(Variant(_49.0.fld1, 0), 1),Field::<i32>(Variant(_49.0.fld1, 0), 1),Field::<i32>(Variant(_49.0.fld1, 0), 1)];
_1 = Move(_47.2);
_52 = &mut _49.1.0;
_18.2 = _5;
(*_9) = _43 - _43;
_15.2 = Adt52::Variant1 { fld0: _36.2.2,fld1: _15.0.fld1,fld2: (*_7),fld3: _38 };
_14 = !(*_7);
_39.0 = _18.0.0 + _18.0.0;
(*_52) = (_15.1.0.0,);
(*_52) = _15.1.0;
(*_52) = _53.fld0;
_36.1.2 = !_48;
_50 = (*_52).0 as f64;
(*_52) = (_36.2.1,);
_19 = _16;
Goto(bb25)
}
bb25 = {
(*_9) = _43;
(*_7) = Field::<u128>(Variant(Field::<Adt21>(Variant(_15.2, 1), 1), 0), 0) as i128;
_48 = _36.1.2 & _36.1.2;
_20 = _40;
_35 = (*_52).0 | _8;
(*_52).0 = _48 as i64;
(*_52) = _45;
_29 = (*_9) as u128;
_15.0.fld3 = !Field::<i8>(Variant(_15.0.fld1, 0), 2);
place!(Field::<i8>(Variant(place!(Field::<Adt21>(Variant(_15.2, 1), 1)), 0), 2)) = Field::<i8>(Variant(_15.0.fld1, 0), 2);
(*_52).0 = !_45.0;
_54 = _11 >> _20;
place!(Field::<Adt21>(Variant(_15.2, 1), 1)) = Adt21::Variant0 { fld0: _29,fld1: _28,fld2: Field::<i8>(Variant(_15.0.fld1, 0), 2) };
_24 = (*_9) as i128;
(*_52).0 = (*_9) as i64;
_51 = &_13;
_15.0.fld0 = _18.2;
_50 = _39.0 as f64;
(*_52).0 = _8;
(*_9) = _3 + _3;
Goto(bb26)
}
bb26 = {
_32 = Adt46::Variant1 { fld0: _5,fld1: _50,fld2: Move(_15.0),fld3: _29,fld4: Move(_1) };
(*_52) = (_35,);
_59 = Field::<Adt22>(Variant(_32, 1), 2).fld3 as i16;
_3 = (*_9) * _43;
_42 = &mut _52;
_54 = _19 + _20;
_38 = [Field::<i32>(Variant(Field::<Adt21>(Variant(_15.2, 1), 1), 0), 1),Field::<i32>(Variant(Field::<Adt22>(Variant(_32, 1), 2).fld1, 0), 1),Field::<i32>(Variant(Field::<Adt22>(Variant(_32, 1), 2).fld1, 0), 1),Field::<i32>(Variant(Field::<Adt21>(Variant(_15.2, 1), 1), 0), 1),Field::<i32>(Variant(Field::<Adt21>(Variant(_15.2, 1), 1), 0), 1)];
(*_42) = &mut _53.fld0;
(*_9) = _43 * _3;
_57 = (_15.1.0.0,);
_14 = (*_7) * (*_7);
_61 = &place!(Field::<u128>(Variant(place!(Field::<Adt21>(Variant(_15.2, 1), 1)), 0), 0));
(*_9) = _3 + _43;
Goto(bb27)
}
bb27 = {
_61 = &_23;
(*_42) = &mut _57;
(*_7) = _14 >> _29;
_15.1.1 = Move(_10);
_28 = Field::<i32>(Variant(Field::<Adt21>(Variant(_15.2, 1), 1), 0), 1);
(*_9) = _43;
_55 = core::ptr::addr_of!((*_9));
_18.0.0 = (*_9) as u32;
_47.2 = core::ptr::addr_of!(_15.0);
(*_42) = &mut _45;
_16 = (*_55) as isize;
_33 = (*_7) != (*_7);
_15.1.0 = (_36.2.1,);
_29 = Field::<u128>(Variant(Field::<Adt21>(Variant(_15.2, 1), 1), 0), 0);
(*_55) = -_3;
_47.1 = [Field::<i8>(Variant(Field::<Adt21>(Variant(_15.2, 1), 1), 0), 2),Field::<i8>(Variant(Field::<Adt21>(Variant(_15.2, 1), 1), 0), 2)];
place!(Field::<Adt22>(Variant(_32, 1), 2)).fld0 = !Field::<bool>(Variant(_32, 1), 0);
_62.1.1 = &_15.1.0.0;
Goto(bb28)
}
bb28 = {
_28 = Field::<i32>(Variant(Field::<Adt22>(Variant(_32, 1), 2).fld1, 0), 1);
place!(Field::<Adt22>(Variant(_32, 1), 2)) = Adt22 { fld0: _33,fld1: Field::<Adt21>(Variant(_15.2, 1), 1),fld2: _44,fld3: Field::<i8>(Variant(Field::<Adt21>(Variant(_15.2, 1), 1), 0), 2) };
_62.1.3 = _4 | _59;
_18.3 = !Field::<Adt22>(Variant(_32, 1), 2).fld0;
(*_9) = -_43;
_15.0.fld3 = Field::<i8>(Variant(Field::<Adt21>(Variant(_15.2, 1), 1), 0), 2) << _59;
place!(Field::<Adt22>(Variant(_32, 1), 2)).fld0 = _18.2 ^ _33;
_6 = _27;
_63 = _50;
place!(Field::<u128>(Variant(_32, 1), 3)) = Field::<u128>(Variant(Field::<Adt22>(Variant(_32, 1), 2).fld1, 0), 0) + _29;
_59 = _62.1.3 ^ Field::<i16>(Variant(_15.2, 1), 0);
_64 = core::ptr::addr_of!(place!(Field::<i32>(Variant(place!(Field::<Adt21>(Variant(_15.2, 1), 1)), 0), 1)));
(*_7) = _48 as i128;
place!(Field::<i32>(Variant(place!(Field::<Adt22>(Variant(_32, 1), 2)).fld1, 0), 1)) = Field::<Adt22>(Variant(_32, 1), 2).fld3 as i32;
_62.1.1 = &_8;
_30 = [(*_51),(*_51),(*_51),(*_51),(*_51),(*_51),(*_51),(*_51)];
(*_9) = _39.0 as f32;
_62.0 = &_12;
place!(Field::<i32>(Variant(place!(Field::<Adt21>(Variant(_15.2, 1), 1)), 0), 1)) = Field::<i32>(Variant(Field::<Adt22>(Variant(_32, 1), 2).fld1, 0), 1) ^ Field::<i32>(Variant(Field::<Adt22>(Variant(_32, 1), 2).fld1, 0), 1);
_62.0 = Move(_61);
(*_7) = _14 & _14;
(*_42) = &mut _15.1.0;
(*_9) = _36.1.3 as f32;
Goto(bb29)
}
bb29 = {
(*_9) = _3;
(*_9) = _43 + _3;
_14 = (*_7) & (*_7);
_3 = (*_9) - (*_9);
(*_64) = Field::<i32>(Variant(Field::<Adt22>(Variant(_32, 1), 2).fld1, 0), 1) + Field::<i32>(Variant(Field::<Adt22>(Variant(_32, 1), 2).fld1, 0), 1);
(*_7) = _14;
_36.2.1 = _8 | _35;
_46 = -_20;
(*_9) = _43 * _43;
_55 = core::ptr::addr_of!((*_9));
_46 = !_54;
(*_7) = _14;
(*_7) = _14 ^ _14;
_65 = _19;
(*_9) = _36.2.1 as f32;
_61 = &place!(Field::<u128>(Variant(place!(Field::<Adt22>(Variant(_32, 1), 2)).fld1, 0), 0));
(*_64) = Field::<f64>(Variant(_32, 1), 1) as i32;
_30 = [(*_51),(*_51),(*_51),(*_51),(*_51),(*_51),(*_51),(*_51)];
(*_64) = Field::<i32>(Variant(Field::<Adt22>(Variant(_32, 1), 2).fld1, 0), 1) & Field::<i32>(Variant(Field::<Adt22>(Variant(_32, 1), 2).fld1, 0), 1);
_40 = _16 - _19;
(*_64) = !Field::<i32>(Variant(Field::<Adt22>(Variant(_32, 1), 2).fld1, 0), 1);
Goto(bb30)
}
bb30 = {
(*_7) = _36.2.1 as i128;
place!(Field::<f64>(Variant(_32, 1), 1)) = _50 * _50;
(*_7) = _36.1.3 as i128;
place!(Field::<i32>(Variant(place!(Field::<Adt22>(Variant(_32, 1), 2)).fld1, 0), 1)) = (*_64) ^ (*_64);
_56 = core::ptr::addr_of!(_36.1);
_10 = &(*_51);
(*_9) = (*_56).0 - (*_56).0;
(*_56).1 = _59 >> (*_64);
(*_56).2 = _48 & _48;
_66 = [(*_51),(*_51),(*_10),(*_51),(*_10),(*_51),(*_10),(*_51)];
(*_56).0 = -_43;
(*_7) = _14 ^ _14;
(*_56).1 = -_59;
_30 = [(*_51),_13,(*_10),(*_10),(*_51),(*_10),(*_10),(*_10)];
_35 = _36.2.1;
(*_7) = _14;
Goto(bb31)
}
bb31 = {
(*_56).0 = (*_9) * _3;
place!(Field::<*const Adt22>(Variant(_32, 1), 4)) = core::ptr::addr_of!(place!(Field::<Adt22>(Variant(_32, 1), 2)));
_43 = (*_56).0 + _3;
(*_56).3 = 23_u8 << (*_64);
(*_64) = Field::<i32>(Variant(Field::<Adt22>(Variant(_32, 1), 2).fld1, 0), 1) ^ Field::<i32>(Variant(Field::<Adt22>(Variant(_32, 1), 2).fld1, 0), 1);
_48 = (*_56).2;
_13 = '\u{54d6}';
_43 = (*_56).0 - (*_56).0;
(*_9) = -(*_56).0;
(*_56).0 = _43 + (*_9);
_10 = Move(_51);
(*_56).2 = (*_64) as usize;
(*_56).0 = 26714_u16 as f32;
(*_56).2 = !_48;
(*_9) = _43 - _3;
(*_64) = -Field::<i32>(Variant(Field::<Adt22>(Variant(_32, 1), 2).fld1, 0), 1);
Goto(bb32)
}
bb32 = {
place!(Field::<i8>(Variant(place!(Field::<Adt22>(Variant(_32, 1), 2)).fld1, 0), 2)) = Field::<Adt22>(Variant(_32, 1), 2).fld3 + Field::<Adt22>(Variant(_32, 1), 2).fld3;
(*_9) = _3 - _3;
(*_9) = -_43;
Goto(bb33)
}
bb33 = {
(*_64) = !Field::<i32>(Variant(Field::<Adt22>(Variant(_32, 1), 2).fld1, 0), 1);
_43 = (*_9) + (*_9);
place!(Field::<Adt22>(Variant(_32, 1), 2)).fld0 = (*_64) == (*_64);
(*_56).3 = Field::<Adt22>(Variant(_32, 1), 2).fld0 as u8;
(*_64) = Field::<i32>(Variant(Field::<Adt22>(Variant(_32, 1), 2).fld1, 0), 1) + Field::<i32>(Variant(Field::<Adt22>(Variant(_32, 1), 2).fld1, 0), 1);
(*_7) = _14 ^ _14;
_18.1 = &_13;
_62.1.1 = &_35;
(*_56).3 = (*_7) as u8;
place!(Field::<Adt22>(Variant(_32, 1), 2)).fld3 = (*_56).3 as i8;
(*_7) = _14 | _14;
place!(Field::<u128>(Variant(_32, 1), 3)) = _18.2 as u128;
_5 = _33;
(*_56).1 = _36.2.1 as i16;
(*_9) = _43 * _43;
(*_56).0 = (*_9) * (*_9);
(*_56).1 = _59;
_21 = (*_56).0 + _43;
_76 = -_65;
Goto(bb34)
}
bb34 = {
(*_56) = (_43, _62.1.3, _48, 227_u8);
(*_9) = (*_56).0 - (*_56).0;
_75 = &_18.1;
(*_56) = ((*_9), _59, _48, 124_u8);
Goto(bb35)
}
bb35 = {
_66 = [_13,_13,_13,_13,_13,_13,_13,_13];
_36.1 = ((*_9), _59, _48, 218_u8);
(*_64) = -Field::<i32>(Variant(Field::<Adt22>(Variant(_32, 1), 2).fld1, 0), 1);
(*_56).3 = _13 as u8;
_63 = _50 * _50;
_68 = _24 & (*_7);
(*_56).2 = !_48;
(*_56).2 = _48;
_36.3 = &_36.2.3;
_73 = _6;
(*_56).2 = !_48;
_73 = _44 | _44;
_47.1 = [Field::<Adt22>(Variant(_32, 1), 2).fld3,Field::<Adt22>(Variant(_32, 1), 2).fld3];
Goto(bb36)
}
bb36 = {
(*_56).1 = -_62.1.3;
_64 = core::ptr::addr_of!((*_64));
_77 = core::ptr::addr_of!((*_7));
(*_7) = _68 * _14;
_18.0 = (_39.0,);
(*_56).3 = 106_u8;
(*_56).3 = _36.2.1 as u8;
Goto(bb37)
}
bb37 = {
_27 = _44 ^ Field::<Adt22>(Variant(_32, 1), 2).fld2;
place!(Field::<i8>(Variant(place!(Field::<Adt22>(Variant(_32, 1), 2)).fld1, 0), 2)) = Field::<Adt22>(Variant(_32, 1), 2).fld3;
_36.1.3 = 249_u8;
place!(Field::<u128>(Variant(place!(Field::<Adt22>(Variant(_32, 1), 2)).fld1, 0), 0)) = !Field::<u128>(Variant(_32, 1), 3);
_16 = _65 - _46;
(*_7) = _39.0 as i128;
_76 = _20 >> Field::<i32>(Variant(Field::<Adt22>(Variant(_32, 1), 2).fld1, 0), 1);
(*_56).0 = (*_9) + (*_9);
(*_56).3 = 99_u8;
_62.1.2 = [_39.0,_18.0.0,_39.0,_18.0.0,_18.0.0,_39.0,_39.0,_39.0];
(*_56).1 = !_62.1.3;
(*_56).3 = 58_u8 ^ 42_u8;
place!(Field::<f64>(Variant(_32, 1), 1)) = _63 + _50;
_69 = &_8;
(*_56).2 = !_48;
_19 = _31 ^ _11;
place!(Field::<u128>(Variant(place!(Field::<Adt22>(Variant(_32, 1), 2)).fld1, 0), 0)) = !Field::<u128>(Variant(_32, 1), 3);
_36.1.0 = (*_9) + (*_9);
_18.3 = _18.2;
(*_56).3 = 26_u8 & 35_u8;
_51 = &_13;
(*_56).2 = _48 + _48;
_72 = _68 | _68;
_35 = (*_69) & (*_69);
_62.0 = Move(_61);
(*_9) = (*_56).0 + (*_56).0;
Goto(bb38)
}
bb38 = {
_37 = (*_51);
(*_64) = !Field::<i32>(Variant(Field::<Adt22>(Variant(_32, 1), 2).fld1, 0), 1);
(*_56).0 = -(*_9);
_36.1.1 = -_59;
(*_56) = ((*_9), _59, _48, 123_u8);
_18.1 = Move(_51);
(*_56).0 = (*_9) + (*_9);
_4 = (*_56).1 >> (*_64);
_74 = _13;
(*_7) = _68;
(*_56).3 = 181_u8;
place!(Field::<i8>(Variant(place!(Field::<Adt22>(Variant(_32, 1), 2)).fld1, 0), 2)) = _31 as i8;
(*_9) = (*_56).0 * (*_56).0;
(*_56).1 = _39.0 as i16;
_36.1 = ((*_9), _36.2.2, _48, 24_u8);
(*_56).1 = _59;
(*_56).0 = Field::<Adt22>(Variant(_32, 1), 2).fld2 as f32;
_18.2 = Field::<Adt22>(Variant(_32, 1), 2).fld0 & _17;
(*_56).0 = (*_9);
(*_9) = (*_56).0 * (*_56).0;
_3 = (*_9) * (*_56).0;
(*_64) = Field::<u128>(Variant(Field::<Adt22>(Variant(_32, 1), 2).fld1, 0), 0) as i32;
(*_9) = (*_56).0 - (*_56).0;
_47.0 = [_14];
_33 = _5;
(*_64) = Field::<i32>(Variant(Field::<Adt22>(Variant(_32, 1), 2).fld1, 0), 1) << (*_56).3;
match (*_56).3 {
0 => bb39,
1 => bb40,
2 => bb41,
3 => bb42,
4 => bb43,
24 => bb45,
_ => bb44
}
}
bb39 = {
(*_9) = _6 as f32;
_14 = '\u{a210}' as i128;
_13 = '\u{58995}';
_4 = 314460498_i32 as i16;
_16 = _11 & _11;
_15.0.fld2 = 2902939418_u32 as u64;
(*_9) = _14 as f32;
(*_9) = _6 as f32;
_2 = _8 as isize;
_15.1.0.0 = _8 + _8;
_15.0.fld1 = Adt21::Variant0 { fld0: 120034910902584926625655474731689044186_u128,fld1: (-2066945086_i32),fld2: 29_i8 };
RET = -(-33_i8);
(*_9) = _11 as f32;
(*_9) = _14 as f32;
_17 = _5 | _5;
(*_9) = 905565841_u32 as f32;
_20 = _11 * _11;
_15.0.fld2 = _6 | _6;
_18.0 = (3325258367_u32,);
(*_9) = 6450524525995444338_usize as f32;
_15.1.0.0 = _8;
(*_9) = _4 as f32;
Call(_11 = core::intrinsics::bswap(_20), ReturnTo(bb2), UnwindUnreachable())
}
bb40 = {
(*_56).1 = -_62.1.3;
_64 = core::ptr::addr_of!((*_64));
_77 = core::ptr::addr_of!((*_7));
(*_7) = _68 * _14;
_18.0 = (_39.0,);
(*_56).3 = 106_u8;
(*_56).3 = _36.2.1 as u8;
Goto(bb37)
}
bb41 = {
(*_9) = _43;
(*_7) = Field::<u128>(Variant(Field::<Adt21>(Variant(_15.2, 1), 1), 0), 0) as i128;
_48 = _36.1.2 & _36.1.2;
_20 = _40;
_35 = (*_52).0 | _8;
(*_52).0 = _48 as i64;
(*_52) = _45;
_29 = (*_9) as u128;
_15.0.fld3 = !Field::<i8>(Variant(_15.0.fld1, 0), 2);
place!(Field::<i8>(Variant(place!(Field::<Adt21>(Variant(_15.2, 1), 1)), 0), 2)) = Field::<i8>(Variant(_15.0.fld1, 0), 2);
(*_52).0 = !_45.0;
_54 = _11 >> _20;
place!(Field::<Adt21>(Variant(_15.2, 1), 1)) = Adt21::Variant0 { fld0: _29,fld1: _28,fld2: Field::<i8>(Variant(_15.0.fld1, 0), 2) };
_24 = (*_9) as i128;
(*_52).0 = (*_9) as i64;
_51 = &_13;
_15.0.fld0 = _18.2;
_50 = _39.0 as f64;
(*_52).0 = _8;
(*_9) = _3 + _3;
Goto(bb26)
}
bb42 = {
_3 = _15.0.fld2 as f32;
(*_9) = _21;
place!(Field::<i8>(Variant(_15.0.fld1, 0), 2)) = !RET;
_24 = _14;
_18.1 = &_13;
_17 = (*_9) < (*_9);
_15.1.0 = (_8,);
(*_9) = 2711659313235664463_usize as f32;
(*_9) = _21 + _21;
_15.0.fld3 = _14 as i8;
Goto(bb9)
}
bb43 = {
_36.1.3 = RET as u8;
(*_7) = 22197_u16 as i128;
_16 = _11 ^ _19;
_24 = _6 as i128;
_40 = -_31;
_17 = (*_9) > (*_9);
(*_9) = -_3;
_4 = _36.1.1 << (*_7);
(*_7) = !_14;
_38 = [Field::<i32>(Variant(_15.0.fld1, 0), 1),_28,Field::<i32>(Variant(_15.0.fld1, 0), 1),Field::<i32>(Variant(_15.0.fld1, 0), 1),Field::<i32>(Variant(_15.0.fld1, 0), 1)];
_36.2.3 = _36.2.1;
_36.1.2 = 2_usize - 6_usize;
(*_9) = _3 - _3;
(*_7) = _6 as i128;
_43 = (*_9);
(*_9) = -_36.1.0;
(*_7) = _14 << _31;
(*_9) = -_43;
_27 = _15.0.fld2 >> (*_7);
(*_7) = _14;
(*_7) = _14 - _14;
(*_7) = _14;
_38 = [_28,Field::<i32>(Variant(_15.0.fld1, 0), 1),_28,_28,Field::<i32>(Variant(_15.0.fld1, 0), 1)];
(*_9) = _3 * _3;
_11 = -_19;
(*_7) = _14 | _14;
Goto(bb19)
}
bb44 = {
_27 = _36.1.3 as u64;
(*_9) = _43 - _43;
(*_7) = _14 - _14;
_39 = (_18.0.0,);
_36.2.2 = _4 << _16;
_7 = core::ptr::addr_of!((*_7));
_13 = '\u{78911}';
_23 = !_12;
(*_9) = _3 * _43;
_44 = !_27;
place!(Field::<i32>(Variant(_15.0.fld1, 0), 1)) = _28 | _28;
_6 = !_44;
(*_7) = _14 - _14;
_48 = _36.1.2;
_45 = _15.1.0;
_36.2.3 = _15.1.0.0 & _36.2.1;
(*_7) = _14 & _14;
_15.1.0 = (_36.2.3,);
(*_7) = Field::<i32>(Variant(_15.0.fld1, 0), 1) as i128;
(*_9) = _43;
(*_7) = _14 + _14;
(*_7) = _14 & _14;
(*_7) = -_14;
_51 = &_13;
_29 = _23 & _12;
Goto(bb20)
}
bb45 = {
(*_56) = ((*_9), _59, _48, 84_u8);
_60 = (_39.0,);
(*_56).2 = 44928_u16 as usize;
place!(Field::<i32>(Variant(place!(Field::<Adt22>(Variant(_32, 1), 2)).fld1, 0), 1)) = (*_64) ^ (*_64);
_81 = [_54,_46,_16,_31,_54];
_9 = core::ptr::addr_of!((*_9));
(*_56).2 = Field::<bool>(Variant(_32, 1), 0) as usize;
(*_56).0 = (*_7) as f32;
(*_64) = (*_56).3 as i32;
(*_56).3 = 88_u8 * 120_u8;
(*_56).3 = !234_u8;
_20 = !_31;
(*_64) = !Field::<i32>(Variant(Field::<Adt22>(Variant(_32, 1), 2).fld1, 0), 1);
(*_56).0 = (*_64) as f32;
(*_56).3 = 156_u8 - 35_u8;
(*_56).2 = _48;
_80 = &_39.0;
(*_56).3 = !9_u8;
(*_56).0 = _3 - (*_9);
(*_64) = !Field::<i32>(Variant(Field::<Adt22>(Variant(_32, 1), 2).fld1, 0), 1);
(*_56).1 = !_59;
(*_56).2 = _48;
(*_56).1 = _4 - _59;
_83 = (*_69) - (*_69);
_86 = core::ptr::addr_of_mut!(_71);
Goto(bb46)
}
bb46 = {
_84 = &mut _74;
(*_64) = (*_9) as i32;
_48 = !(*_56).2;
(*_64) = Field::<i32>(Variant(Field::<Adt22>(Variant(_32, 1), 2).fld1, 0), 1);
(*_56).1 = !_59;
(*_56).1 = _59;
_75 = Move(_36.2.0);
(*_86) = (*_9) as isize;
Goto(bb47)
}
bb47 = {
(*_64) = Field::<i32>(Variant(Field::<Adt22>(Variant(_32, 1), 2).fld1, 0), 1);
(*_56) = ((*_9), _4, _48, 105_u8);
_19 = _20;
place!(Field::<Adt22>(Variant(_32, 1), 2)).fld0 = !_5;
_47.0 = [(*_7)];
_63 = Field::<f64>(Variant(_32, 1), 1);
(*_64) = Field::<i32>(Variant(Field::<Adt22>(Variant(_32, 1), 2).fld1, 0), 1);
_11 = -(*_86);
(*_56).1 = _4 + _36.2.2;
_90 = _14 << _46;
(*_56).3 = 134_u8;
(*_56).0 = -(*_9);
(*_56) = ((*_9), _4, _48, 81_u8);
(*_56).0 = -(*_9);
_63 = _50;
Call(_62.1.3 = core::intrinsics::transmute(_47.1), ReturnTo(bb48), UnwindUnreachable())
}
bb48 = {
(*_84) = _37;
(*_56).3 = (*_80) as u8;
(*_86) = _40;
(*_86) = _19 << (*_64);
_18.0.0 = (*_80);
_58 = [Field::<i8>(Variant(Field::<Adt22>(Variant(_32, 1), 2).fld1, 0), 2),Field::<Adt22>(Variant(_32, 1), 2).fld3,Field::<i8>(Variant(Field::<Adt22>(Variant(_32, 1), 2).fld1, 0), 2)];
(*_64) = Field::<f64>(Variant(_32, 1), 1) as i32;
(*_64) = -Field::<i32>(Variant(Field::<Adt22>(Variant(_32, 1), 2).fld1, 0), 1);
(*_64) = Field::<i32>(Variant(Field::<Adt22>(Variant(_32, 1), 2).fld1, 0), 1) >> _62.1.3;
_36.3 = &_8;
Goto(bb49)
}
bb49 = {
_89.1 = Field::<i8>(Variant(Field::<Adt22>(Variant(_32, 1), 2).fld1, 0), 2) as i16;
_40 = -(*_86);
(*_56).2 = (*_69) as usize;
(*_56).0 = (*_9) * _3;
(*_56).0 = (*_9) + (*_9);
place!(Field::<bool>(Variant(_32, 1), 0)) = (*_64) != (*_64);
_51 = &(*_84);
_7 = core::ptr::addr_of!((*_7));
place!(Field::<Adt22>(Variant(_32, 1), 2)).fld3 = Field::<i8>(Variant(Field::<Adt22>(Variant(_32, 1), 2).fld1, 0), 2) ^ Field::<i8>(Variant(Field::<Adt22>(Variant(_32, 1), 2).fld1, 0), 2);
(*_56).2 = (*_56).0 as usize;
_75 = &_51;
(*_64) = !Field::<i32>(Variant(Field::<Adt22>(Variant(_32, 1), 2).fld1, 0), 1);
(*_84) = _37;
(*_84) = _13;
(*_56).3 = _18.3 as u8;
_36.1.0 = (*_9) - (*_9);
_40 = _65 + (*_86);
_19 = (*_86);
(*_86) = _16;
Goto(bb50)
}
bb50 = {
Call(_98 = dump_var(Move(_12), Move(_8), Move(_4), Move(_58)), ReturnTo(bb51), UnwindUnreachable())
}
bb51 = {
Call(_98 = dump_var(Move(_33), Move(_59), Move(_65), Move(_37)), ReturnTo(bb52), UnwindUnreachable())
}
bb52 = {
Call(_98 = dump_var(Move(_14), Move(_31), Move(_11), Move(_73)), ReturnTo(bb53), UnwindUnreachable())
}
bb53 = {
Call(_98 = dump_var(Move(_28), Move(_13), Move(_19), Move(_45)), ReturnTo(bb54), UnwindUnreachable())
}
bb54 = {
Call(_98 = dump_var(Move(_6), Move(_46), Move(_68), Move(_40)), ReturnTo(bb55), UnwindUnreachable())
}
bb55 = {
Call(_98 = dump_var(Move(_29), Move(_35), Move(_76), _99), ReturnTo(bb56), UnwindUnreachable())
}
bb56 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(36411218073518437989376076820816932548_u128), std::hint::black_box('\u{2943}'), std::hint::black_box(1743_i16));
                
                unsafe {
                    println!("hash: {}", H.finish());
                }
            
            }
#[derive(Debug,Copy,Clone)]
pub enum Adt21 {
Variant0{
fld0: u128,
fld1: i32,
fld2: i8,

},
Variant1{
fld0: bool,
fld1: f64,
fld2: isize,
fld3: i8,
fld4: i16,
fld5: u32,
fld6: f32,
fld7: usize,

}}
#[derive(Debug)]
pub struct Adt22 {
fld0: bool,
fld1: Adt21,
fld2: u64,
fld3: i8,
}
#[derive(Debug)]
pub enum Adt46 {
Variant0{
fld0: i128,
fld1: Adt22,
fld2: i32,

},
Variant1{
fld0: bool,
fld1: f64,
fld2: Adt22,
fld3: u128,
fld4: *const Adt22,

}}
#[derive(Debug,Copy,Clone)]
pub enum Adt52 {
Variant0{
fld0: (u64, u128),
fld1: [i128; 2],
fld2: usize,
fld3: u8,

},
Variant1{
fld0: i16,
fld1: Adt21,
fld2: i128,
fld3: [i32; 5],

},
Variant2{
fld0: i32,
fld1: f64,
fld2: [u8; 4],

}}
#[derive(Debug)]
pub enum Adt59 {
Variant0{
fld0: *const f32,
fld1: [i16; 6],
fld2: isize,
fld3: *mut isize,
fld4: i16,
fld5: u128,
fld6: *const i128,
fld7: u8,

},
Variant1{
fld0: u8,
fld1: *mut isize,
fld2: *const Adt22,
fld3: u64,

},
Variant2{
fld0: [isize; 5],
fld1: [i128; 2],
fld2: i8,

},
Variant3{
fld0: bool,
fld1: [i16; 6],
fld2: Adt21,
fld3: (u64, u128),
fld4: i128,

}}
#[derive(Debug)]
pub enum Adt71 {
Variant0{
fld0: (u32,),
fld1: [i8; 3],
fld2: Adt59,
fld3: i8,
fld4: *mut i128,
fld5: i128,
fld6: f32,

},
Variant1{
fld0: [isize; 5],
fld1: char,

},
Variant2{
fld0: [i8; 3],
fld1: [i32; 5],
fld2: [i128; 2],

}}
#[derive(Debug)]
pub struct Adt73 {
fld0: (i64,),
fld1: [u32; 8],
fld2: *const [i32; 5],
fld3: *mut [isize; 5],
fld4: Adt21,
}
#[derive(Debug)]
pub enum Adt75 {
Variant0{
fld0: i128,
fld1: *const i32,
fld2: *mut i128,
fld3: Adt46,
fld4: [isize; 5],
fld5: [i16; 6],
fld6: (u64, u128),

},
Variant1{
fld0: Adt73,
fld1: *mut [isize; 5],
fld2: [char; 8],
fld3: i8,
fld4: i16,
fld5: usize,

},
Variant2{
fld0: bool,
fld1: [i128; 1],
fld2: Adt59,
fld3: Adt71,
fld4: (u64, u128),
fld5: u64,
fld6: [i128; 2],

}}

