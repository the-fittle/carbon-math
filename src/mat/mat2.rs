use crate::*;
use std::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign,
};

macro_rules! impl_mat2 {
    ($t:ident) => {
        impl Mat2<$t> {
            // pub fn new<C>(c0: C, c1: C) -> Self
            // where
            //     C: Into<Vec2<$t>>,
            // {
            //     Self([c0.into(), c1.into()])
            // }

            pub fn new(m00: $t, m01: $t, m10: $t, m11: $t) -> Self {
                Self([[m00, m01].into(), [m10, m11].into()])
            }

            pub fn new_cols<C>(c0: C, c1: C) -> Self
            where
                C: Into<[$t; 2]>,
            {
                let [c00, c01] = c0.into();
                let [c10, c11] = c1.into();
                Self::new(c00, c01, c10, c11)
            }

            pub fn new_rows<R>(r0: R, r1: R) -> Self
            where
                R: Into<[$t; 2]>,
            {
                let [r00, r10] = r0.into();
                let [r01, r11] = r1.into();
                Self::new(r00, r01, r10, r11)
            }

            pub fn zero() -> Self {
                Self([Vec2::<$t>::zero(); 2])
            }

            pub fn one() -> Self {
                Self([Vec2::<$t>::one(); 2])
            }

            pub fn identity() -> Self {
                Self::new($t::one(), $t::zero(), $t::zero(), $t::one())
            }

            pub fn transposed(&self) -> Self {
                let [m00, m01] = self[0].into();
                let [m10, m11] = self[1].into();
                Self::new(m00, m10, m01, m11)
            }

            pub fn inversed(&self) -> Option<Self> {
                let det = self.determinant();
                if det == $t::zero() {
                    None
                } else {
                    Some(self.adjugate() / det)
                }
            }

            pub fn extended(&self, vec: Vec3<$t>) -> Mat3<$t> {
                let [m00, m01] = self[0].into();
                let [m10, m11] = self[1].into();
                let [m20, m21, m22] = vec.into();
                Mat3::<$t>::new(m00, m01, $t::zero(), m10, m11, $t::zero(), m20, m21, m22)
            }

            pub fn determinant(&self) -> $t {
                let [m00, m01] = self[0].into();
                let [m10, m11] = self[1].into();

                m00 * m11 - m01 * m10
            }

            pub fn adjugate(&self) -> Self {
                let [m00, m01] = self[0].into();
                let [m10, m11] = self[1].into();

                Self::new(m11, -m01, -m10, m00)
            }

            pub fn from_translation(translation: $t) -> Self {
                Self::new($t::one(), $t::zero(), translation, $t::one())
            }

            pub fn get_translation(&self) -> $t {
                self[1][0]
            }

            pub fn translate(&mut self, translation: $t) {
                *self *= Self::from_translation(translation);
            }

            pub fn from_rotation(rad: $t) -> Self {
                let (s, c) = (rad.sin(), rad.cos());
                Self::new(c, -s, s, c)
            }

            pub fn get_rotation(&self) -> $t {
                self[0][0].atan2(self[1][0])
            }

            pub fn rotate(&mut self, rad: $t) {
                *self *= Self::from_rotation(rad);
            }

            pub fn from_scale(scale: $t) -> Self {
                Self::new(scale, $t::zero(), $t::zero(), $t::one())
            }

            pub fn get_scale(&self) -> $t {
                self[0][0]
            }

            pub fn scale(&mut self, scale: $t) {
                *self *= Self::from_scale(scale);
            }
        }

        // display
        // ---------------------------------------------------------------------------------------------------
        impl std::fmt::Display for Mat2<$t> {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                let [m00, m01] = self[0].into();
                let [m10, m11] = self[1].into();

                write!(
                    f,
                    "[{:>8.4} {:>8.4}]\n[{:>8.4} {:>8.4}]",
                    m00, m01, m10, m11
                )
            }
        }

        // from
        // ---------------------------------------------------------------------------------------------------
        impl From<[Vec2<$t>; 2]> for Mat2<$t> {
            fn from(m: [Vec2<$t>; 2]) -> Self {
                Self(m)
            }
        }

        impl From<Mat2<$t>> for [Vec2<$t>; 2] {
            fn from(m: Mat2<$t>) -> Self {
                m.0
            }
        }

        impl Index<usize> for Mat2<$t> {
            type Output = Vec2<$t>;
            fn index(&self, index: usize) -> &Self::Output {
                &self.0[index]
            }
        }

        impl IndexMut<usize> for Mat2<$t> {
            fn index_mut(&mut self, index: usize) -> &mut Vec2<$t> {
                &mut self.0[index]
            }
        }

        // addition
        // ---------------------------------------------------------------------------------------------------
        impl Add for Mat2<$t> {
            type Output = Self;
            fn add(self, rhs: Self) -> Self::Output {
                Self::new_cols(self[0] + rhs[0], self[1] + rhs[1])
            }
        }

        impl AddAssign for Mat2<$t> {
            fn add_assign(&mut self, rhs: Self) {
                *self = *self + rhs;
            }
        }

        // subtraction
        // ---------------------------------------------------------------------------------------------------
        impl Sub for Mat2<$t> {
            type Output = Self;
            fn sub(self, rhs: Self) -> Self::Output {
                Self::new_cols(self[0] - rhs[0], self[1] - rhs[1])
            }
        }

        impl SubAssign for Mat2<$t> {
            fn sub_assign(&mut self, rhs: Self) {
                *self = *self - rhs;
            }
        }

        // multiplication
        // ---------------------------------------------------------------------------------------------------
        impl Mul for Mat2<$t> {
            type Output = Self;
            fn mul(self, rhs: Self) -> Self::Output {
                let sa = self[0];
                let sb = self[1];
                let oa = rhs[0];
                let ob = rhs[1];
                Self::new(
                    sa[0] * oa[0] + sb[0] * oa[1],
                    sa[1] * oa[0] + sb[1] * oa[1],
                    sa[0] * ob[0] + sb[0] * ob[1],
                    sa[1] * ob[0] + sb[1] * ob[1],
                )
            }
        }

        impl MulAssign for Mat2<$t> {
            fn mul_assign(&mut self, rhs: Self) {
                *self = *self * rhs;
            }
        }

        // vector multiplication
        // ---------------------------------------------------------------------------------------------------
        impl Mul<Vec2<$t>> for Mat2<$t> {
            type Output = Vec2<$t>;
            fn mul(self, rhs: Vec2<$t>) -> Self::Output {
                let sa = self[0];
                let sb = self[1];
                Vec2::<$t>::new(
                    sa[0] * rhs[0] + sb[0] * rhs[1],
                    sa[1] * rhs[0] + sb[1] * rhs[1],
                )
            }
        }

        // scalar multiplication
        // ---------------------------------------------------------------------------------------------------
        impl Mul<$t> for Mat2<$t> {
            type Output = Self;
            fn mul(self, rhs: $t) -> Self::Output {
                Self::new_cols(self[0] * rhs, self[1] * rhs)
            }
        }

        impl Mul<Mat2<$t>> for $t {
            type Output = Mat2<$t>;
            fn mul(self, rhs: Mat2<$t>) -> Self::Output {
                Self::Output::new_cols(self * rhs[0], self * rhs[1])
            }
        }

        impl MulAssign<$t> for Mat2<$t> {
            fn mul_assign(&mut self, rhs: $t) {
                *self = *self * rhs;
            }
        }

        // scalar division
        // ---------------------------------------------------------------------------------------------------
        impl Div<$t> for Mat2<$t> {
            type Output = Self;
            fn div(self, rhs: $t) -> Self::Output {
                Self::new_cols(self[0] / rhs, self[1] / rhs)
            }
        }

        impl DivAssign<$t> for Mat2<$t> {
            fn div_assign(&mut self, rhs: $t) {
                *self = *self / rhs;
            }
        }

        // negation
        // ---------------------------------------------------------------------------------------------------
        impl Neg for Mat2<$t> {
            type Output = Self;
            fn neg(self) -> Self::Output {
                Self::new_cols(-self[0], -self[1])
            }
        }
    };
}

impl_mat2!(f32);
impl_mat2!(f64);
