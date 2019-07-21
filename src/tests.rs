macro_rules! gen_tests {
    ($mod_name:ident, $ty:ty, $val0:expr, $val1:expr, $with_logic:ident) => {
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
}

gen_tests!(_bool, bool, true, false, true);
gen_tests!(_u8, u8, 7u8, 33u8, true);
gen_tests!(_i8, i8, 7i8, 33i8, true);
gen_tests!(_u16, u16, 7u16, 33u16, true);
gen_tests!(_i16, i16, 7i16, 33i16, true);
gen_tests!(_u32, u32, 7u32, 33u32, true);
gen_tests!(_i32, i32, 7i32, 33i32, true);
gen_tests!(_u64, u64, 7u64, 33u64, true);
gen_tests!(_i64, i64, 7i64, 33i64, true);
gen_tests!(_usize, usize, 7usize, 33usize, true);
gen_tests!(_isize, isize, 7isize, 33isize, true);
gen_tests!(_ptr, *mut String, 0 as *mut String, 0xDEADBEEF as *mut String, false);