use crate::*;
use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign};

macro_rules! impl_vec3 {
    ($t:ident) => {
        impl Vec4<$t> {
            #[inline]
            pub fn zero() -> Self {
                Self([$t::zero(); 4])
            }

            #[inline]
            pub fn one() -> Self {
                Self([$t::one(); 4])
            }

            #[inline]
            pub fn new(x: $t, y: $t, z: $t, w: $t) -> Self {
                Self([x, y, z, w])
            }

            #[inline]
            pub fn unit_x() -> Self {
                Self([$t::one(), $t::zero(), $t::zero(), $t::zero()])
            }

            #[inline]
            pub fn unit_y() -> Self {
                Self([$t::zero(), $t::one(), $t::zero(), $t::zero()])
            }

            #[inline]
            pub fn unit_z() -> Self {
                Self([$t::zero(), $t::zero(), $t::one(), $t::zero()])
            }

            #[inline]
            pub fn unit_w() -> Self {
                Self([$t::zero(), $t::zero(), $t::zero(), $t::one()])
            }

            #[inline]
            pub fn identity() -> Self {
                Self([$t::zero(), $t::zero(), $t::zero(), $t::one()])
            }

            #[inline]
            pub fn x(&self) -> $t {
                self[0]
            }

            #[inline]
            pub fn y(&self) -> $t {
                self[1]
            }

            #[inline]
            pub fn z(&self) -> $t {
                self[2]
            }

            #[inline]
            pub fn w(&self) -> $t {
                self[3]
            }

            #[inline]
            pub fn dot(&self, rhs: Self) -> $t {
                (self[0] * rhs[0]) + (self[1] * rhs[1]) + (self[2] * rhs[2]) + (self[3] * rhs[3])
            }

            #[inline]
            pub fn length(&self) -> $t {
                self.dot(*self).sqrt()
            }

            #[inline]
            pub fn normalized(&self) -> Self {
                if self.length() == 0.0 {
                    *self
                } else {
                    *self / self.length()
                }
            }

            #[inline]
            pub fn reflected(&self, normal: Self) -> Self {
                *self - normal * (2.0 * self.dot(normal))
            }

            #[inline]
            pub fn truncated(self) -> Vec3<$t> {
                Vec3([self[0], self[1], self[2]])
            }
        }

        // display
        // ---------------------------------------------------------------------------------------------------
        impl std::fmt::Display for Vec4<$t> {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(
                    f,
                    "(x, y, z, w): [{:>8.4} {:>8.4} {:>8.4} {:>8.4}]",
                    self[0], self[1], self[2], self[3]
                )
            }
        }

        // from
        // ---------------------------------------------------------------------------------------------------
        impl From<[$t; 4]> for Vec4<$t> {
            fn from(v: [$t; 4]) -> Self {
                Self(v)
            }
        }

        impl From<Vec4<$t>> for [$t; 4] {
            fn from(v: Vec4<$t>) -> Self {
                v.0
            }
        }

        // index
        // ---------------------------------------------------------------------------------------------------
        impl Index<usize> for Vec4<$t> {
            type Output = $t;
            fn index(&self, index: usize) -> &Self::Output {
                &self.0[index]
            }
        }

        impl IndexMut<usize> for Vec4<$t> {
            fn index_mut(&mut self, index: usize) -> &mut $t {
                &mut self.0[index]
            }
        }

        // addition
        // ---------------------------------------------------------------------------------------------------
        impl Add for Vec4<$t> {
            type Output = Self;
            fn add(self, rhs: Self) -> Self::Output {
                Self([
                    self[0] + rhs[0],
                    self[1] + rhs[1],
                    self[2] + rhs[2],
                    self[3] + rhs[3],
                ])
            }
        }

        impl AddAssign for Vec4<$t> {
            fn add_assign(&mut self, rhs: Self) {
                *self = *self + rhs;
            }
        }

        // subtraction
        // ---------------------------------------------------------------------------------------------------
        impl Sub for Vec4<$t> {
            type Output = Self;
            fn sub(self, rhs: Self) -> Self::Output {
                Self([
                    self[0] - rhs[0],
                    self[1] - rhs[1],
                    self[2] - rhs[2],
                    self[3] - rhs[3],
                ])
            }
        }

        impl SubAssign for Vec4<$t> {
            fn sub_assign(&mut self, rhs: Self) {
                *self = *self - rhs;
            }
        }

        // multiplication
        // ---------------------------------------------------------------------------------------------------
        impl Mul for Vec4<$t> {
            type Output = Self;
            fn mul(self, rhs: Self) -> Self::Output {
                Self([
                    self[0] * rhs[0],
                    self[1] * rhs[1],
                    self[2] * rhs[2],
                    self[3] * rhs[3],
                ])
            }
        }

        impl MulAssign for Vec4<$t> {
            fn mul_assign(&mut self, rhs: Self) {
                *self = *self * rhs;
            }
        }

        // scalar multiplication
        // ---------------------------------------------------------------------------------------------------
        impl Mul<$t> for Vec4<$t> {
            type Output = Self;
            fn mul(self, rhs: $t) -> Self::Output {
                Self([self[0] * rhs, self[1] * rhs, self[2] * rhs, self[3] * rhs])
            }
        }

        impl MulAssign<$t> for Vec4<$t> {
            fn mul_assign(&mut self, rhs: $t) {
                *self = *self * rhs;
            }
        }

        impl Mul<Vec4<$t>> for $t {
            type Output = Vec4<$t>;
            fn mul(self, rhs: Vec4<$t>) -> Self::Output {
                rhs * self
            }
        }

        // scalar division
        // ---------------------------------------------------------------------------------------------------
        impl Div<$t> for Vec4<$t> {
            type Output = Self;
            fn div(self, rhs: $t) -> Self::Output {
                Self([self[0] / rhs, self[1] / rhs, self[2] / rhs, self[3] / rhs])
            }
        }

        impl DivAssign<$t> for Vec4<$t> {
            fn div_assign(&mut self, rhs: $t) {
                *self = *self / rhs;
            }
        }

        // negation
        // ---------------------------------------------------------------------------------------------------
        impl Neg for Vec4<$t> {
            type Output = Self;
            fn neg(self) -> Self::Output {
                Self::zero() - self
            }
        }
    };
}

impl_vec3!(f32);
impl_vec3!(f64);
