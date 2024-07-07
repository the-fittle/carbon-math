use crate::*;
use std::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign,
};

macro_rules! impl_bivec3 {
    ($t:ident) => {
        impl Bivec3<$t> {
            pub fn new(xy: $t, xz: $t, yz: $t) -> Self {
                Self([xy, xz, yz])
            }

            pub fn zero() -> Self {
                Self([$t::zero(); 3])
            }

            pub fn one() -> Self {
                Self([$t::one(); 3])
            }

            pub fn unit_xy() -> Self {
                Self([$t::one(), $t::zero(), $t::zero()])
            }

            pub fn unit_xz() -> Self {
                Self([$t::zero(), $t::one(), $t::zero()])
            }

            pub fn unit_yz() -> Self {
                Self([$t::zero(), $t::zero(), $t::one()])
            }

            pub fn xy(&self) -> $t {
                self[0]
            }

            pub fn xz(&self) -> $t {
                self[1]
            }

            pub fn yz(&self) -> $t {
                self[2]
            }

            pub fn dot(&self, rhs: Self) -> $t {
                (self[0] * rhs[0]) + (self[1] * rhs[1]) + (self[2] * rhs[2])
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
        impl std::fmt::Display for Bivec3<$t> {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(
                    f,
                    "(xy, xz, yz): {:>8.4} {:>8.4} {:>8.4}",
                    self[0], self[1], self[2]
                )
            }
        }

        // index
        // ---------------------------------------------------------------------------------------------------
        impl Index<usize> for Bivec3<$t> {
            type Output = $t;
            fn index(&self, index: usize) -> &Self::Output {
                &self.0[index]
            }
        }

        impl IndexMut<usize> for Bivec3<$t> {
            fn index_mut(&mut self, index: usize) -> &mut $t {
                &mut self.0[index]
            }
        }

        // addition
        // ---------------------------------------------------------------------------------------------------
        impl Add for Bivec3<$t> {
            type Output = Self;
            fn add(self, rhs: Self) -> Self::Output {
                Self([self[0] + rhs[0], self[1] + rhs[1], self[2] + rhs[2]])
            }
        }

        impl AddAssign for Bivec3<$t> {
            fn add_assign(&mut self, rhs: Self) {
                *self = *self + rhs;
            }
        }

        // subtraction
        // ---------------------------------------------------------------------------------------------------
        impl Sub for Bivec3<$t> {
            type Output = Self;
            fn sub(self, rhs: Self) -> Self::Output {
                Self([self[0] - rhs[0], self[1] - rhs[1], self[2] - rhs[2]])
            }
        }

        impl SubAssign for Bivec3<$t> {
            fn sub_assign(&mut self, rhs: Self) {
                *self = *self - rhs;
            }
        }

        // multiplication
        // ---------------------------------------------------------------------------------------------------
        impl Mul for Bivec3<$t> {
            type Output = Self;
            fn mul(self, rhs: Self) -> Self::Output {
                Self([self[0] * rhs[0], self[1] * rhs[1], self[2] * rhs[2]])
            }
        }

        impl MulAssign for Bivec3<$t> {
            fn mul_assign(&mut self, rhs: Self) {
                *self = *self * rhs;
            }
        }

        // scalar multiplication
        // ---------------------------------------------------------------------------------------------------
        impl Mul<$t> for Bivec3<$t> {
            type Output = Self;
            fn mul(self, rhs: $t) -> Self::Output {
                Self([self[0] * rhs, self[1] * rhs, self[2] * rhs])
            }
        }

        impl Mul<Bivec3<$t>> for $t {
            type Output = Bivec3<$t>;
            fn mul(self, rhs: Bivec3<$t>) -> Self::Output {
                rhs * self
            }
        }

        impl MulAssign<$t> for Bivec3<$t> {
            fn mul_assign(&mut self, rhs: $t) {
                *self = *self * rhs;
            }
        }

        // division
        // ---------------------------------------------------------------------------------------------------
        impl Div for Bivec3<$t> {
            type Output = Self;
            fn div(self, rhs: Self) -> Self::Output {
                Self([self[0] / rhs[0], self[1] / rhs[1], self[2] / rhs[2]])
            }
        }

        impl DivAssign for Bivec3<$t> {
            fn div_assign(&mut self, rhs: Self) {
                *self = *self / rhs;
            }
        }

        // scalar division
        // ---------------------------------------------------------------------------------------------------
        impl Div<$t> for Bivec3<$t> {
            type Output = Self;
            fn div(self, rhs: $t) -> Self::Output {
                Self([self[0] / rhs, self[1] / rhs, self[2] / rhs])
            }
        }

        impl DivAssign<$t> for Bivec3<$t> {
            fn div_assign(&mut self, rhs: $t) {
                *self = *self / rhs;
            }
        }

        // negation
        // ---------------------------------------------------------------------------------------------------
        impl Neg for Bivec3<$t> {
            type Output = Self;
            fn neg(self) -> Self::Output {
                Self([-self[0], -self[1], -self[2]])
            }
        }
    };
}

impl_bivec3!(f32);
impl_bivec3!(f64);
