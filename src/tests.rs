use super::Atom;

macro_rules! gen_tests {
    (
        $mod_name:ident, $ty:ty, $val0:expr, $val1:expr,
        $with_logic:ident, $with_int:ident, $with_default:ident
    ) => {
        mod $mod_name {
            use crate::{Atomic, Ordering};

            #[test]
            fn new() {
                let _: Atomic<$ty> = Atomic::new($val0);
            }

            #[test]
            fn load_store() {
                let a = Atomic::new($val0);
                assert_eq!(a.load(Ordering::SeqCst), $val0);

                a.store($val1, Ordering::SeqCst);
                assert_eq!(a.load(Ordering::SeqCst), $val1);
            }

            #[test]
            fn into_inner() {
                let a = Atomic::new($val0);
                assert_eq!(a.into_inner(), $val0);
            }

            #[test]
            fn swap() {
                let a = Atomic::new($val0);
                assert_eq!(a.swap($val1, Ordering::SeqCst), $val0);
                assert_eq!(a.load(Ordering::SeqCst), $val1);
            }

            // TODO: compare_and_* methods

            gen_tests!(@logic $val0, $val1, $with_logic);

            gen_tests!(@integer $val0, $val1, $with_int);

            gen_tests!(@default $ty, $with_default);


            #[test]
            fn fmt_debug() {
                let a = Atomic::new($val0);
                assert_eq!(format!("{:?}", a), format!("{:?}", $val0));
            }

            #[test]
            fn from() {
                let a = Atomic::new($val0);
                let b: Atomic<$ty> = $val0.into();
                assert_eq!(a.load(Ordering::SeqCst), b.load(Ordering::SeqCst));
            }
        }
    };
    (@logic $val0:expr, $val1:expr, true) => {
        #[test]
        fn logic() {
            let a = Atomic::new($val0);
            assert_eq!(a.fetch_and($val1, Ordering::SeqCst), $val0);
            assert_eq!(a.load(Ordering::SeqCst), $val0 & $val1);

            let a = Atomic::new($val0);
            assert_eq!(a.fetch_nand($val1, Ordering::SeqCst), $val0);
            assert_eq!(a.load(Ordering::SeqCst), !($val0 & $val1));

            let a = Atomic::new($val0);
            assert_eq!(a.fetch_or($val1, Ordering::SeqCst), $val0);
            assert_eq!(a.load(Ordering::SeqCst), $val0 | $val1);

            let a = Atomic::new($val0);
            assert_eq!(a.fetch_xor($val1, Ordering::SeqCst), $val0);
            assert_eq!(a.load(Ordering::SeqCst), $val0 ^ $val1);
        }
    };
    (@logic $val0:expr, $val1:expr, false) => {};

    (@integer $val0:expr, $val1:expr, true) => {
        #[test]
        fn integer() {
            let a = Atomic::new($val0);
            assert_eq!(a.fetch_add($val1, Ordering::SeqCst), $val0);
            assert_eq!(a.load(Ordering::SeqCst), $val0.wrapping_add($val1));

            let a = Atomic::new($val0);
            assert_eq!(a.fetch_sub($val1, Ordering::SeqCst), $val0);
            assert_eq!(a.load(Ordering::SeqCst), $val0.wrapping_sub($val1));
        }
    };
    (@integer $val0:expr, $val1:expr, false) => {};

    (@default $ty:ty, true) => {
        #[test]
        fn default() {
            let a: Atomic<$ty> = Default::default();
            assert_eq!(a.load(Ordering::SeqCst), <$ty>::default());
        }
    };
    (@default $ty:ty, false) => {};
}

//         mod     ty     val0    val1     logic  int    default
gen_tests!(_bool,  bool,  true,   false,   true,  false, true);
gen_tests!(_u8,    u8,    7u8,    33u8,    true,  true,  true);
gen_tests!(_i8,    i8,    7i8,    33i8,    true,  true,  true);
gen_tests!(_u16,   u16,   7u16,   33u16,   true,  true,  true);
gen_tests!(_i16,   i16,   7i16,   33i16,   true,  true,  true);
gen_tests!(_u32,   u32,   7u32,   33u32,   true,  true,  true);
gen_tests!(_i32,   i32,   7i32,   33i32,   true,  true,  true);
gen_tests!(_u64,   u64,   7u64,   33u64,   true,  true,  true);
gen_tests!(_i64,   i64,   7i64,   33i64,   true,  true,  true);
gen_tests!(_usize, usize, 7usize, 33usize, true,  true,  true);
gen_tests!(_isize, isize, 7isize, 33isize, true,  true,  true);
gen_tests!(_f32,   f32,   7.0f32, 33.0f32, false, false, true);
gen_tests!(_f64,   f64,   7.0f64, 33.0f64, false, false, true);
gen_tests!(_char,  char,  'x',    '♥',     false, false, true);
gen_tests!(_ptr, *mut String, 0 as *mut String, 0xBADC0DE as *mut String, false, false, false);
gen_tests!(custom, super::Foo, super::Foo::Nothing, super::Foo::Set(0b101), false, false, true);


#[derive(Debug, PartialEq, Eq)]
enum Foo {
    Nothing,
    Set(u8),
}

impl Default for Foo {
    fn default() -> Self {
        Foo::Set(0b10101010)
    }
}

impl Atom for Foo {
    type Repr = u16;
    fn pack(self) -> Self::Repr {
        match self {
            Foo::Nothing => 0x01FF,
            Foo::Set(s) => s as u16,
        }
    }
    fn unpack(src: Self::Repr) -> Self {
        if src & 0x0100 != 0 {
            Foo::Nothing
        } else {
            Foo::Set((src & 0xFF) as u8)
        }
    }
}
