#[allow(dead_code)]
pub mod math {
    use std::ops::{Add, Mul, Sub};

    macro_rules! saturate {
        ($t:ident, $f:ident, $out:ty, $unit:ident) => {
            impl $t for $unit {
                type Output = Self;
                fn $f(self, rhs: Self) -> $out {
                    self.$f(rhs)
                }
            }
        };
    }

    macro_rules! make_saturate_types {
        ($t:ident, $f:ident, $out:ty) => {
            saturate!($t, $f, $out, u8);
            saturate!($t, $f, $out, u16);
            saturate!($t, $f, $out, u32);
            saturate!($t, $f, $out, u64);
            saturate!($t, $f, $out, u128);
            saturate!($t, $f, $out, usize);

            saturate!($t, $f, $out, i8);
            saturate!($t, $f, $out, i16);
            saturate!($t, $f, $out, i32);
            saturate!($t, $f, $out, i64);
            saturate!($t, $f, $out, i128);
            saturate!($t, $f, $out, isize);
        };
    }

    macro_rules! make_saturating {
        ($t:ident, $f:ident, $out:ty) => {
            pub trait $t {
                type Output;
                fn $f(self, rhs: Self) -> $out;
            }

            make_saturate_types!($t, $f, $out);
        };
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Vector2<Unit>(pub Unit, pub Unit);

    impl<U> Vector2<U> {
        pub fn map<T, F: Fn(&U) -> T>(&self, map: F) -> Vector2<T> {
            Vector2(map(&self.0), map(&self.1))
        }
    }

    impl<U: Clone> From<U> for Vector2<U> {
        fn from(value: U) -> Self {
            Self(value.clone(), value)
        }
    }

    impl<U> From<(U, U)> for Vector2<U> {
        fn from(value: (U, U)) -> Self {
            Self(value.0, value.1)
        }
    }

    impl<U> From<Vector2<U>> for (U, U) {
        fn from(value: Vector2<U>) -> Self {
            (value.0, value.1)
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Zone<Unit> {
        pub position: Vector2<Unit>, // x, y
        pub size: Vector2<Unit>,     // width, height
    }

    macro_rules! make_basic {
        ($t:ident, $f:ident) => {
            impl<U: $t> $t for Vector2<U> {
                type Output = Vector2<U::Output>;
                fn $f(self, rhs: Self) -> Self::Output {
                    Vector2(self.0.$f(rhs.0), self.1.$f(rhs.1))
                }
            }

            impl<U: $t> $t for Zone<U> {
                type Output = Zone<U::Output>;
                fn $f(self, rhs: Self) -> Self::Output {
                    Zone {
                        position: self.position.$f(rhs.position),
                        size: self.size.$f(rhs.size),
                    }
                }
            }
        };
    }

    macro_rules! make_basic_option {
        ($t:ident, $f:ident, $out:ty) => {
            impl<U: $t> $t for Vector2<U> {
                type Output = Vector2<U::Output>;
                fn $f(self, rhs: Self) -> $out {
                    Some(Vector2(self.0.$f(rhs.0)?, self.1.$f(rhs.1)?))
                }
            }

            impl<U: $t> $t for Zone<U> {
                type Output = Zone<U::Output>;
                fn $f(self, rhs: Self) -> $out {
                    Some(Zone {
                        position: self.position.$f(rhs.position)?,
                        size: self.size.$f(rhs.size)?,
                    })
                }
            }
        };
    }

    macro_rules! make_basic_saturating {
        ($t:ident, $f:ident) => {
            make_saturating!($t, $f, Self::Output);
            make_basic!($t, $f);
        };
        ($t:ident, $f:ident, $out:ty) => {
            make_saturating!($t, $f, $out);
            make_basic_option!($t, $f, $out);
        };
    }

    make_basic!(Add, add);
    make_basic!(Sub, sub);
    make_basic!(Mul, mul);
    make_basic_saturating!(SaturatingAdd, saturating_add);
    make_basic_saturating!(SaturatingSub, saturating_sub);
    make_basic_saturating!(SaturatingMul, saturating_mul);
    make_basic_saturating!(CheckedAdd, checked_add, Option<Self::Output>);
    make_basic_saturating!(CheckedSub, checked_sub, Option<Self::Output>);
    make_basic_saturating!(CheckedMul, checked_mul, Option<Self::Output>);
}
