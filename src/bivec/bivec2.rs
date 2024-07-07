use crate::*;
use std::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign,
};

macro_rules! impl_bivec2 {
    ($t:ident) => {
        impl Bivec2<$t> {
            pub fn new(xy: $t) -> Self {
                Self(xy)
            }

            pub fn zero() -> Self {
                Self($t::zero())
            }

            pub fn one() -> Self {
                Self($t::one())
            }

            pub fn unit_xy() -> Self {
                Self($t::one())
            }

            pub fn xy(&self) -> $t {
                self[0]
            }

            pub fn dot(&self, rhs: Self) -> $t {
                self[0] * rhs[0]
            }

            pub fn length(&self) -> $t {
                self.dot(*self).sqrt()
            }

            pub fn normalized(&self) -> Self {
                if self.length() == 0.0 {
                    *self
                } else {
                    *self / self.length()
                }
            }
        }

        // display
        // ---------------------------------------------------------------------------------------------------
        impl std::fmt::Display for Bivec2<$t> {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "(xy): {:>8.4}", self[0])
            }
        }

        // index
        // ---------------------------------------------------------------------------------------------------
        impl Index<usize> for Bivec2<$t> {
            type Output = $t;
            fn index(&self, _index: usize) -> &Self::Output {
                &self.0
            }
        }

        impl IndexMut<usize> for Bivec2<$t> {
            fn index_mut(&mut self, _index: usize) -> &mut $t {
                &mut self.0
            }
        }

        // addition
        // ---------------------------------------------------------------------------------------------------
        impl Add for Bivec2<$t> {
            type Output = Self;
            fn add(self, rhs: Self) -> Self::Output {
                Self(self[0] + rhs[0])
            }
        }

        impl AddAssign for Bivec2<$t> {
            fn add_assign(&mut self, rhs: Self) {
                *self = *self + rhs;
            }
        }

        // subtraction
        // ---------------------------------------------------------------------------------------------------
        impl Sub for Bivec2<$t> {
            type Output = Self;
            fn sub(self, rhs: Self) -> Self::Output {
                Self(self[0] - rhs[0])
            }
        }

        impl SubAssign for Bivec2<$t> {
            fn sub_assign(&mut self, rhs: Self) {
                *self = *self - rhs;
            }
        }

        // multiplication
        // ---------------------------------------------------------------------------------------------------
        impl Mul for Bivec2<$t> {
            type Output = Self;
            fn mul(self, rhs: Self) -> Self::Output {
                Self(self[0] * rhs[0])
            }
        }

        impl MulAssign for Bivec2<$t> {
            fn mul_assign(&mut self, rhs: Self) {
                *self = *self * rhs;
            }
        }

        // scalar multiplication
        // ---------------------------------------------------------------------------------------------------
        impl Mul<$t> for Bivec2<$t> {
            type Output = Self;
            fn mul(self, rhs: $t) -> Self::Output {
                Self(self[0] * rhs)
            }
        }

        impl Mul<Bivec2<$t>> for $t {
            type Output = Bivec2<$t>;
            fn mul(self, rhs: Bivec2<$t>) -> Self::Output {
                rhs * self
            }
        }

        impl MulAssign<$t> for Bivec2<$t> {
            fn mul_assign(&mut self, rhs: $t) {
                *self = *self * rhs;
            }
        }

        // division
        // ---------------------------------------------------------------------------------------------------
        impl Div for Bivec2<$t> {
            type Output = Self;
            fn div(self, rhs: Self) -> Self::Output {
                Self(self[0] / rhs[0])
            }
        }

        impl DivAssign for Bivec2<$t> {
            fn div_assign(&mut self, rhs: Self) {
                *self = *self / rhs;
            }
        }

        // scalar division
        // ---------------------------------------------------------------------------------------------------
        impl Div<$t> for Bivec2<$t> {
            type Output = Self;
            fn div(self, rhs: $t) -> Self::Output {
                Self(self[0] / rhs)
            }
        }

        impl DivAssign<$t> for Bivec2<$t> {
            fn div_assign(&mut self, rhs: $t) {
                *self = *self / rhs;
            }
        }

        // negation
        // ---------------------------------------------------------------------------------------------------
        impl Neg for Bivec2<$t> {
            type Output = Self;
            fn neg(self) -> Self::Output {
                Self(-self[0])
            }
        }
    };
}

impl_bivec2!(f32);
impl_bivec2!(f64);
