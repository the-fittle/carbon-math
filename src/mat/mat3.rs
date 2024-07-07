use crate::*;
use std::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign,
};

macro_rules! impl_mat3 {
    ($t:ident) => {
        impl Mat3<$t> {
			pub fn new(m00: $t, m01: $t, m02: $t, m10: $t, m11: $t, m12: $t, m20: $t, m21: $t, m22: $t) -> Self
			{
				Self([
					[m00, m01, m02].into(),
					[m10, m11, m12].into(),
					[m20, m21, m22].into(),
				])
			}

			pub fn new_cols<C>(c0: C, c1: C, c2: C) -> Self
			where
				C: Into<[$t; 3]>,
			{
				let [c00, c01, c02] = c0.into();
				let [c10, c11, c12] = c1.into();
				let [c20, c21, c22] = c2.into();
				Self::new(c00, c10, c20, c01, c11, c21, c02, c12, c22)
			}

			pub fn new_rows<R>(r0: R, r1: R, r2: R) -> Self
			where
				R: Into<[$t; 3]>,
			{
				let [r00, r01, r02] = r0.into();
				let [r10, r11, r12] = r1.into();
				let [r20, r21, r22] = r2.into();
				Self::new(r00, r01, r02, r10, r11, r12, r20, r21, r22)
			}

            pub fn zero() -> Self {
                Self([Vec3::<$t>::zero(); 3])
            }

            pub fn one() -> Self {
                Self([Vec3::<$t>::one(); 3])
            }

            pub fn identity() -> Self {
                Self::new(
					$t::one(), $t::zero(), $t::zero(),
					$t::zero(), $t::one(), $t::zero(),
					$t::zero(), $t::zero(), $t::one(),
				)
            }

            pub fn transposed(&self) -> Self {
                let [m00, m01, m02] = self[0].into();
				let [m10, m11, m12] = self[1].into();
				let [m20, m21, m22] = self[2].into();
				Self::new(
					m00, m10, m20,
					m01, m11, m21,
					m02, m12, m22,
				)
            }

			pub fn inversed(&self) -> Option<Self> {
                let det = self.determinant();
				if det == $t::zero() {
					None
				} else {
					Some(self.adjugate() / det)
				}
            }

			pub fn extended(&self, vec: Vec4<$t>) -> Mat4<$t> {
                let [m00, m01, m02] = self[0].into();
				let [m10, m11, m12] = self[1].into();
				let [m20, m21, m22] = self[2].into();
				Mat4::<$t>::new_cols(
					[m00, m01, m02, $t::zero()],
					[m10, m11, m12, $t::zero()],
					[m20, m21, m22, $t::zero()],
					vec.into(),
				)
            }

            pub fn determinant(&self) -> $t {
				let [m00, m01, m02] = self[0].into();
				let [m10, m11, m12] = self[1].into();
				let [m20, m21, m22] = self[2].into();

				m00 * (m11 * m22 - m12 * m21) - m01 * (m10 * m22 - m12 * m20) + m02 * (m10 * m21 - m11 * m20)
            }

            pub fn adjugate(&self) -> Self {

				let [m00, m01, m02] = self[0].into();
				let [m10, m11, m12] = self[1].into();
				let [m20, m21, m22] = self[2].into();

				Self::new(
						m11 * m22 - m12 * m21,
						-(m10 * m22 - m12 * m20),
						m10 * m21 - m11 * m20,
						-(m01 * m22 - m02 * m21),
						m00 * m22 - m02 * m20,
						-(m00 * m21 - m01 * m20),
						m01 * m12 - m02 * m11,
						-(m00 * m12 - m02 * m10),
						m00 * m11 - m01 * m10,
				)
            }

			pub fn from_diagonal(diagonal: Vec3<$t>) -> Self {
				Self::new(
					diagonal[0], $t::zero(), $t::zero(),
					$t::zero(), diagonal[1], $t::zero(),
					$t::zero(), $t::zero(), diagonal[2],
				)
			}

			pub fn get_diagonal(&self) -> Vec3<$t> {
				Vec3::<$t>::new(self[0][0], self[1][1], self[2][2])
			}

            pub fn from_translation(translation: Vec2<$t>) -> Self {
                Self::new(
                    $t::one(), $t::zero(), $t::zero(),
                    $t::zero(), $t::one(), $t::zero(),
                    translation[0], translation[1], $t::one(),
                )
            }

            pub fn get_translation(&self) -> Vec2<$t> {
                Vec2::<$t>::new(self[2][0], self[2][1])
            }

            pub fn translate(&mut self, translation: Vec2<$t>) {
                *self *= Self::from_translation(translation);
            }

            pub fn from_rotation_x(theta: $t) -> Self {
                let (sin, cos) = theta.sin_cos();
                Self::new($t::one(), $t::zero(), $t::zero(), $t::zero(), cos, -sin, $t::zero(), sin, cos)
            }

            pub fn from_rotation_y(theta: $t) -> Self {
                let (sin, cos) = theta.sin_cos();
                Self::new(cos, $t::zero(), sin, $t::zero(), $t::one(), $t::zero(), -sin, $t::zero(), cos)
            }

            pub fn from_rotation_z(theta: $t) -> Self {
                let (sin, cos) = theta.sin_cos();
                Self::new(cos, -sin, $t::zero(), sin, cos, $t::zero(), $t::zero(), $t::zero(), $t::one())
            }

            pub fn get_rotation_x(&self) -> $t {
                self[1][1].atan2(self[2][1])
            }

            pub fn get_rotation_y(&self) -> $t {
                self[2][0].atan2(self[0][0])
            }

            pub fn get_rotation_z(&self) -> $t {
                self[0][1].atan2(self[1][0])
            }

            pub fn rotate_x(&mut self, theta: $t) {
                *self *= Self::from_rotation_x(theta);
            }

            pub fn rotate_y(&mut self, theta: $t) {
                *self *= Self::from_rotation_y(theta);
            }

            pub fn rotate_z(&mut self, theta: $t) {
                *self *= Self::from_rotation_z(theta);
            }

            pub fn from_scale(scale: Vec2<$t>) -> Self {
                Self::new(scale[0], $t::zero(), $t::zero(), $t::zero(), scale[1], $t::zero(), $t::zero(), $t::zero(), $t::one())
            }

            pub fn get_scale(&self) -> Vec2<$t> {
                Vec2::<$t>::new(self[0][0], self[1][1])
            }

            pub fn scale(&mut self, scale: Vec2<$t>) {
                *self *= Self::from_scale(scale);
            }
        }

		// display
		// ---------------------------------------------------------------------------------------------------
		impl std::fmt::Display for Mat3<$t> {
			fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
				let [m00, m01, m02] = self[0].into();
				let [m10, m11, m12] = self[1].into();
				let [m20, m21, m22] = self[2].into();

				write!(
					f,
					"[{:>8.4} {:>8.4} {:>8.4}]\n[{:>8.4} {:>8.4} {:>8.4}]\n[{:>8.4} {:>8.4} {:>8.4}]",
					m00, m01, m02, m10, m11, m12, m20, m21, m22
				)
			}
		}

		// from
		// ---------------------------------------------------------------------------------------------------
        impl From<[Vec3<$t>; 3]> for Mat3<$t> {
            fn from(v: [Vec3<$t>; 3]) -> Self {
                Self(v)
            }
        }

        impl From<Mat3<$t>> for [Vec3<$t>; 3] {
            fn from(m: Mat3<$t>) -> Self {
                m.0
            }
        }

		impl From<Quat<$t>> for Mat3<$t> {
			fn from(q: Quat<$t>) -> Self {
				let [x, y, z, w] = q.into();
				let x2 = x + x;
				let y2 = y + y;
				let z2 = z + z;
				let xx = x * x2;
				let xy = x * y2;
				let xz = x * z2;
				let yy = y * y2;
				let yz = y * z2;
				let zz = z * z2;
				let wx = w * x2;
				let wy = w * y2;
				let wz = w * z2;
				Self::new(
					$t::one() - (yy + zz), xy + wz, xz - wy,
					xy - wz, $t::one() - (xx + zz), yz + wx,
					xz + wy, yz - wx, $t::one() - (xx + yy),
				)
			}
		}

		// index
		// ---------------------------------------------------------------------------------------------------
        impl Index<usize> for Mat3<$t> {
            type Output = Vec3<$t>;

            fn index(&self, index: usize) -> &Self::Output {
                &self.0[index]
            }
        }

        impl IndexMut<usize> for Mat3<$t> {
            fn index_mut(&mut self, index: usize) -> &mut Vec3<$t> {
                &mut self.0[index]
            }
        }

        // addition
		// ---------------------------------------------------------------------------------------------------
        impl Add for Mat3<$t> {
            type Output = Self;
            fn add(self, rhs: Self) -> Self::Output {
                Self::new_cols(self[0] + rhs[0], self[1] + rhs[1], self[2] + rhs[2])
            }
        }

        impl AddAssign for Mat3<$t> {
            fn add_assign(&mut self, rhs: Self) {
                *self = *self + rhs;
            }
        }

        // subtraction
		// ---------------------------------------------------------------------------------------------------
        impl Sub for Mat3<$t> {
            type Output = Self;
            fn sub(self, rhs: Self) -> Self::Output {
                Self::new_cols(self[0] - rhs[0], self[1] - rhs[1], self[2] - rhs[2])
            }
        }

        impl SubAssign for Mat3<$t> {
            fn sub_assign(&mut self, rhs: Self) {
                *self = *self - rhs;
            }
        }

        // multiplication
		// ---------------------------------------------------------------------------------------------------
        impl Mul for Mat3<$t> {
            type Output = Self;
            fn mul(self, rhs: Self) -> Self::Output {
                let sa = self[0];
                let sb = self[1];
                let sc = self[2];
                let oa = rhs[0];
                let ob = rhs[1];
                let oc = rhs[2];
                Self::new(
                        sa[0] * oa[0] + sb[0] * oa[1] + sc[0] * oa[2],
                        sa[1] * oa[0] + sb[1] * oa[1] + sc[1] * oa[2],
                        sa[2] * oa[0] + sb[2] * oa[1] + sc[2] * oa[2],
                        sa[0] * ob[0] + sb[0] * ob[1] + sc[0] * ob[2],
                        sa[1] * ob[0] + sb[1] * ob[1] + sc[1] * ob[2],
                        sa[2] * ob[0] + sb[2] * ob[1] + sc[2] * ob[2],
                        sa[0] * oc[0] + sb[0] * oc[1] + sc[0] * oc[2],
                        sa[1] * oc[0] + sb[1] * oc[1] + sc[1] * oc[2],
                        sa[2] * oc[0] + sb[2] * oc[1] + sc[2] * oc[2],
                )
            }
        }

        impl MulAssign for Mat3<$t> {
            fn mul_assign(&mut self, rhs: Self) {
                *self = *self * rhs;
            }
        }

        // vector multiplication
		// ---------------------------------------------------------------------------------------------------
        impl Mul<Vec3<$t>> for Mat3<$t> {
            type Output = Vec3<$t>;
            fn mul(self, rhs: Vec3<$t>) -> Self::Output {
                let sa = self[0];
                let sb = self[1];
                let sc = self[2];
                Vec3::<$t>::new(
                    sa[0] * rhs[0] + sb[0] * rhs[1] + sc[0] * rhs[2],
                    sa[1] * rhs[0] + sb[1] * rhs[1] + sc[1] * rhs[2],
                    sa[2] * rhs[0] + sb[2] * rhs[1] + sc[2] * rhs[2],
                )
            }
        }

        // scalar multiplication
		// ---------------------------------------------------------------------------------------------------
        impl Mul<$t> for Mat3<$t> {
            type Output = Self;
            fn mul(self, rhs: $t) -> Self::Output {
                Self::new_cols(self[0] * rhs, self[1] * rhs, self[2] * rhs)
            }
        }

        impl Mul<Mat3<$t>> for $t {
            type Output = Mat3<$t>;
            fn mul(self, rhs: Mat3<$t>) -> Self::Output {
                Self::Output::new_cols(self * rhs[0], self * rhs[1], self * rhs[2])
            }
        }

        impl MulAssign<$t> for Mat3<$t> {
            fn mul_assign(&mut self, rhs: $t) {
                *self = *self * rhs;
            }
        }

        // scalar division
		// ---------------------------------------------------------------------------------------------------
        impl Div<$t> for Mat3<$t> {
            type Output = Self;
            fn div(self, rhs: $t) -> Self::Output {
                Self::new_cols(self[0] / rhs, self[1] / rhs, self[2] / rhs)
            }
        }

        impl DivAssign<$t> for Mat3<$t> {
            fn div_assign(&mut self, rhs: $t) {
                *self = *self / rhs;
            }
        }

        // negation
		// ---------------------------------------------------------------------------------------------------
        impl Neg for Mat3<$t> {
            type Output = Self;
            fn neg(self) -> Self::Output {
                Self::new_cols(-self[0], -self[1], -self[2])
            }
        }
    };
}

impl_mat3!(f32);
impl_mat3!(f64);
