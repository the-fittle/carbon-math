use crate::*;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

macro_rules! impl_rot2 {
    ($t:ident) => {
        impl Rot2<$t> {
            pub fn new(s: $t, bi: Bivec2<$t>) -> Self {
                Self(s, bi)
            }

            pub fn zero() -> Self {
                Self($t::zero(), Bivec2::<$t>::zero())
            }

            pub fn one() -> Self {
                Self($t::one(), Bivec2::<$t>::one())
            }

            pub fn identity() -> Self {
                Self($t::one(), Bivec2::<$t>::zero())
            }

            pub fn s(&self) -> $t {
                self.0
            }

            pub fn bi(&self) -> Bivec2<$t> {
                self.1
            }

            pub fn dot(&self, rhs: Self) -> $t {
                self.0 * rhs.0 + self.1.dot(rhs.1)
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

            pub fn reversed(&self) -> Self {
                Self(self.0, -self.1)
            }

            #[deprecated(note = "Use from_rotation instead")]
            pub fn from_rotation_xy(rad: $t) -> Self {
                Self::from_rotation_bi(rad, Bivec2::<$t>::unit_xy())
            }

            pub fn from_rotation(rad: $t) -> Self {
                Self::from_rotation_bi(rad, Bivec2::<$t>::unit_xy())
            }

            pub fn from_rotation_bi(rad: $t, bi: Bivec2<$t>) -> Self {
                let half_angle = rad * $t::splat(0.5);
                let (sin, cos) = half_angle.sin_cos();
                Self(cos, bi * -sin)
            }

            pub fn from_rotation_between(from: Vec2<$t>, to: Vec2<$t>) -> Self {
                Self::new($t::one() + to.dot(from), to.wedge(from)).normalized()
            }

            pub fn to_mat2(&self) -> Mat2<$t> {
                let s = self.0;
                let bi = self.1[0];
                let s2 = s * s;
                let bi2 = bi * bi;
                let s2_diff_bi2 = s2 - bi2;
                let two_s_bi = $t::splat(2.0) * s * bi;
                Mat2::<$t>::new(s2_diff_bi2, -two_s_bi, two_s_bi, s2_diff_bi2)
            }

            pub fn to_mat3(&self) -> Mat3<$t> {
                self.to_mat2()
                    .extended(Vec3::<$t>::new($t::zero(), $t::zero(), $t::one()))
            }

            pub fn to_quat(&self) -> Quat<$t> {
                let half_angle = self.0.acos();
                let sin = half_angle.sin();
                let axis = self.1.normalized();
                Quat::<$t>::new(axis[0] * sin, axis[1] * sin, $t::zero(), half_angle.cos())
            }
        }

        // display
        // ---------------------------------------------------------------------------------------------------
        impl std::fmt::Display for Rot2<$t> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let xy = self.1[0];
                write!(f, "(xy, s): [{:>8.4}, {:>8.4}]", xy, self.0)
            }
        }

        // addition
        // ---------------------------------------------------------------------------------------------------
        impl Add for Rot2<$t> {
            type Output = Self;
            fn add(self, rhs: Self) -> Self::Output {
                Self(self.0 + rhs.0, self.1 + rhs.1)
            }
        }

        impl AddAssign for Rot2<$t> {
            fn add_assign(&mut self, rhs: Self) {
                *self = *self + rhs;
            }
        }

        // subtraction
        // ---------------------------------------------------------------------------------------------------
        impl Sub for Rot2<$t> {
            type Output = Self;
            fn sub(self, rhs: Self) -> Self::Output {
                Self(self.0 - rhs.0, self.1 - rhs.1)
            }
        }

        impl SubAssign for Rot2<$t> {
            fn sub_assign(&mut self, rhs: Self) {
                *self = *self - rhs;
            }
        }

        // multiplication
        // ---------------------------------------------------------------------------------------------------
        impl Mul for Rot2<$t> {
            type Output = Self;
            fn mul(self, rhs: Self) -> Self::Output {
                let s_s = self.0;
                let s_xy = self.1[0];
                let r_s = rhs.0;
                let r_xy = rhs.1[0];
                let s = s_s * r_s - s_xy * r_xy;
                let xy = s_s * r_xy + s_xy * r_s;
                Self(s, Bivec2::<$t>::new(xy))
            }
        }

        impl MulAssign for Rot2<$t> {
            fn mul_assign(&mut self, rhs: Self) {
                *self = *self * rhs;
            }
        }

        // vector multiplication
        // ---------------------------------------------------------------------------------------------------
        impl Mul<Vec2<$t>> for Rot2<$t> {
            type Output = Vec2<$t>;
            fn mul(self, rhs: Vec2<$t>) -> Self::Output {
                let s = self.0;
                let bi = self.1[0];
                let [x, y] = rhs.into();
                let _x = s * x + bi * y; // x
                let _y = s * y - bi * x; // y

                let x = s * _x + bi * _y;
                let y = s * _y - bi * _x;
                Vec2::<$t>::new(x, y)
            }
        }

        // scalar multiplication
        // ---------------------------------------------------------------------------------------------------
        impl Mul<$t> for Rot2<$t> {
            type Output = Self;
            fn mul(self, rhs: $t) -> Self::Output {
                Self(self.0 * rhs, self.1 * rhs)
            }
        }

        impl Mul<Rot2<$t>> for $t {
            type Output = Rot2<$t>;
            fn mul(self, rhs: Rot2<$t>) -> Self::Output {
                rhs * self
            }
        }

        impl MulAssign<$t> for Rot2<$t> {
            fn mul_assign(&mut self, rhs: $t) {
                *self = *self * rhs;
            }
        }

        // scalar division
        // ---------------------------------------------------------------------------------------------------
        impl Div<$t> for Rot2<$t> {
            type Output = Self;
            fn div(self, rhs: $t) -> Self::Output {
                Self(self.0 / rhs, self.1 / rhs)
            }
        }

        impl DivAssign<$t> for Rot2<$t> {
            fn div_assign(&mut self, rhs: $t) {
                *self = *self / rhs;
            }
        }

        // negation
        // ---------------------------------------------------------------------------------------------------
        impl Neg for Rot2<$t> {
            type Output = Self;
            fn neg(self) -> Self::Output {
                Self(-self.0, -self.1)
            }
        }
    };
}

impl_rot2!(f32);
