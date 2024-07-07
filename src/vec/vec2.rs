use crate::*;
use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign};

macro_rules! impl_vec2 {
    ($t:ident) => {
        impl Vec2<$t> {
            pub fn zero() -> Self {
                Self([$t::zero(); 2])
            }

            #[inline]
            pub fn one() -> Self {
                Self([$t::one(); 2])
            }

            #[inline]
            pub fn new(x: $t, y: $t) -> Self {
                Self([x, y])
            }

            #[inline]
            pub fn unit_x() -> Self {
                Self([$t::one(), $t::zero()])
            }

            #[inline]
            pub fn unit_y() -> Self {
                Self([$t::zero(), $t::one()])
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
            pub fn dot(&self, rhs: Self) -> $t {
                (self[0] * rhs[0]) + (self[1] * rhs[1])
            }

            #[inline]
            pub fn wedge(&self, rhs: Self) -> Bivec2<$t> {
                Bivec2((self[0] * rhs[1]) - (self[1] * rhs[0]))
            }

            #[inline]
            pub fn geometric(&self, rhs: Self) -> Rot2<$t> {
                Rot2(self.dot(rhs), self.wedge(rhs))
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
                *self - normal * ($t::splat(2.0) * self.dot(normal))
            }

            #[inline]
            pub fn extended(&self, z: $t) -> Vec3<$t> {
                Vec3([self[0], self[1], z])
            }
        }

        // display
        // ---------------------------------------------------------------------------------------------------
        impl std::fmt::Display for Vec2<$t> {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "(x, y): [{:>8.4}, {:>8.4}]", self[0], self[1])
            }
        }

        // from
        // ---------------------------------------------------------------------------------------------------
        impl From<[$t; 2]> for Vec2<$t> {
            fn from(v: [$t; 2]) -> Self {
                Self(v)
            }
        }

        impl From<Vec2<$t>> for [$t; 2] {
            fn from(v: Vec2<$t>) -> Self {
                v.0
            }
        }

        // index
        // ---------------------------------------------------------------------------------------------------
        impl Index<usize> for Vec2<$t> {
            type Output = $t;
            fn index(&self, index: usize) -> &Self::Output {
                &self.0[index]
            }
        }

        impl IndexMut<usize> for Vec2<$t> {
            fn index_mut(&mut self, index: usize) -> &mut $t {
                &mut self.0[index]
            }
        }

        // addition
        // ---------------------------------------------------------------------------------------------------
        impl Add for Vec2<$t> {
            type Output = Self;
            fn add(self, rhs: Self) -> Self::Output {
                Self([self[0] + rhs[0], self[1] + rhs[1]])
            }
        }

        impl AddAssign for Vec2<$t> {
            fn add_assign(&mut self, rhs: Self) {
                *self = *self + rhs;
            }
        }

        // subtraction
        // ---------------------------------------------------------------------------------------------------
        impl Sub for Vec2<$t> {
            type Output = Self;
            fn sub(self, rhs: Self) -> Self::Output {
                Self([self[0] - rhs[0], self[1] - rhs[1]])
            }
        }

        impl SubAssign for Vec2<$t> {
            fn sub_assign(&mut self, rhs: Self) {
                *self = *self - rhs;
            }
        }

        // multiplication
        // ---------------------------------------------------------------------------------------------------
        impl Mul for Vec2<$t> {
            type Output = Self;
            fn mul(self, rhs: Vec2<$t>) -> Self::Output {
                Self([self[0] * rhs[0], self[1] * rhs[1]])
            }
        }

        impl MulAssign for Vec2<$t> {
            fn mul_assign(&mut self, rhs: Vec2<$t>) {
                *self = *self * rhs;
            }
        }

        // vector multiplication
        // ---------------------------------------------------------------------------------------------------
        impl Mul<Vec2<$t>> for $t {
            type Output = Vec2<$t>;
            fn mul(self, rhs: Vec2<$t>) -> Self::Output {
                rhs * self
            }
        }

        // scalar multiplication
        // ---------------------------------------------------------------------------------------------------
        impl Mul<$t> for Vec2<$t> {
            type Output = Self;
            fn mul(self, rhs: $t) -> Self::Output {
                Self([self[0] * rhs, self[1] * rhs])
            }
        }

        impl MulAssign<$t> for Vec2<$t> {
            fn mul_assign(&mut self, rhs: $t) {
                *self = *self * rhs;
            }
        }

        // scalar division
        // ---------------------------------------------------------------------------------------------------
        impl Div<$t> for Vec2<$t> {
            type Output = Self;
            fn div(self, rhs: $t) -> Self::Output {
                Self([self[0] / rhs, self[1] / rhs])
            }
        }

        impl DivAssign<$t> for Vec2<$t> {
            fn div_assign(&mut self, rhs: $t) {
                *self = *self / rhs;
            }
        }

        // negation
        // ---------------------------------------------------------------------------------------------------
        impl Neg for Vec2<$t> {
            type Output = Self;
            fn neg(self) -> Self::Output {
                Self::zero() - self
            }
        }
    };
}

impl_vec2!(f32);
impl_vec2!(f64);
