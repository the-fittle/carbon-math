use crate::*;
use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign};

macro_rules! impl_vec3 {
    ($t:ident) => {
        impl Vec3<$t> {
            #[inline]
            pub fn zero() -> Self {
                Self([$t::zero(); 3])
            }

            #[inline]
            pub fn one() -> Self {
                Self([$t::one(); 3])
            }

            #[inline]
            pub fn new(x: $t, y: $t, z: $t) -> Self {
                Self([x, y, z])
            }

            #[inline]
            pub fn unit_x() -> Self {
                Self([$t::one(), $t::zero(), $t::zero()])
            }

            #[inline]
            pub fn unit_y() -> Self {
                Self([$t::zero(), $t::one(), $t::zero()])
            }

            #[inline]
            pub fn unit_z() -> Self {
                Self([$t::zero(), $t::zero(), $t::one()])
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
            pub fn dot(&self, rhs: Self) -> $t {
                (self[0] * rhs[0]) + (self[1] * rhs[1]) + (self[2] * rhs[2])
            }

            #[inline]
            pub fn cross(&self, rhs: Self) -> Self {
                Self([
                    (self[1] * rhs[2]) - (self[2] * rhs[1]),
                    (self[2] * rhs[0]) - (self[0] * rhs[2]),
                    (self[0] * rhs[1]) - (self[1] * rhs[0]),
                ])
            }

            #[inline]
            pub fn wedge(&self, rhs: Self) -> Bivec3<$t> {
                Bivec3([
                    (self[0] * rhs[1]) - (self[1] * rhs[0]), // xy
                    (self[0] * rhs[2]) - (self[2] * rhs[0]), // xz
                    (self[1] * rhs[2]) - (self[2] * rhs[1]), // yz
                ])
            }

            #[inline]
            pub fn geometric(&self, rhs: Self) -> Rot3<$t> {
                Rot3(self.dot(rhs), self.wedge(rhs))
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
            pub fn truncated(self) -> Vec2<$t> {
                Vec2([self[0], self[1]])
            }

            #[inline]
            pub fn extended(self, w: $t) -> Vec4<$t> {
                Vec4([self[0], self[1], self[2], w])
            }
        }

        // display
        // ---------------------------------------------------------------------------------------------------
        impl std::fmt::Display for Vec3<$t> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(
                    f,
                    "(x, y, z): [{:>8.4} {:>8.4} {:>8.4}]",
                    self[0], self[1], self[2]
                )
            }
        }

        // from
        // ---------------------------------------------------------------------------------------------------
        impl From<[$t; 3]> for Vec3<$t> {
            fn from(v: [$t; 3]) -> Self {
                Self(v)
            }
        }

        impl From<Vec3<$t>> for [$t; 3] {
            fn from(v: Vec3<$t>) -> Self {
                v.0
            }
        }

        // index
        // ---------------------------------------------------------------------------------------------------
        impl Index<usize> for Vec3<$t> {
            type Output = $t;
            fn index(&self, index: usize) -> &Self::Output {
                &self.0[index]
            }
        }

        impl IndexMut<usize> for Vec3<$t> {
            fn index_mut(&mut self, index: usize) -> &mut $t {
                &mut self.0[index]
            }
        }

        // addition
        // ---------------------------------------------------------------------------------------------------
        impl Add for Vec3<$t> {
            type Output = Self;
            fn add(self, rhs: Self) -> Self::Output {
                Self([self[0] + rhs[0], self[1] + rhs[1], self[2] + rhs[2]])
            }
        }

        impl AddAssign for Vec3<$t> {
            fn add_assign(&mut self, rhs: Self) {
                *self = *self + rhs;
            }
        }

        // subtraction
        // ---------------------------------------------------------------------------------------------------
        impl Sub for Vec3<$t> {
            type Output = Self;
            fn sub(self, rhs: Self) -> Self::Output {
                Self([self[0] - rhs[0], self[1] - rhs[1], self[2] - rhs[2]])
            }
        }

        impl SubAssign for Vec3<$t> {
            fn sub_assign(&mut self, rhs: Self) {
                *self = *self - rhs;
            }
        }

        // multiplication
        // ---------------------------------------------------------------------------------------------------
        impl Mul for Vec3<$t> {
            type Output = Self;
            fn mul(self, rhs: Self) -> Self::Output {
                Self([self[0] * rhs[0], self[1] * rhs[1], self[2] * rhs[2]])
            }
        }

        impl MulAssign for Vec3<$t> {
            fn mul_assign(&mut self, rhs: Self) {
                *self = *self * rhs;
            }
        }

        // scalar multiplication
        // ---------------------------------------------------------------------------------------------------
        impl Mul<$t> for Vec3<$t> {
            type Output = Self;
            fn mul(self, rhs: $t) -> Self::Output {
                Self([self[0] * rhs, self[1] * rhs, self[2] * rhs])
            }
        }

        impl MulAssign<$t> for Vec3<$t> {
            fn mul_assign(&mut self, rhs: $t) {
                *self = *self * rhs;
            }
        }

        impl Mul<Vec3<$t>> for $t {
            type Output = Vec3<$t>;
            fn mul(self, rhs: Vec3<$t>) -> Self::Output {
                rhs * self
            }
        }

        // scalar division
        // ---------------------------------------------------------------------------------------------------
        impl Div<$t> for Vec3<$t> {
            type Output = Self;
            fn div(self, rhs: $t) -> Self::Output {
                Self([self[0] / rhs, self[1] / rhs, self[2] / rhs])
            }
        }

        impl DivAssign<$t> for Vec3<$t> {
            fn div_assign(&mut self, rhs: $t) {
                *self = *self / rhs;
            }
        }

        // negation
        // ---------------------------------------------------------------------------------------------------
        impl Neg for Vec3<$t> {
            type Output = Self;
            fn neg(self) -> Self::Output {
                Self::zero() - self
            }
        }
    };
}

impl_vec3!(f32);
impl_vec3!(f64);
