use crate::*;
use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign};

macro_rules! impl_quat {
    ($t:ident) => {
        impl Quat<$t> {
            pub fn zero() -> Self {
                Self([$t::zero(), $t::zero(), $t::zero(), $t::zero()])
            }

            pub fn one() -> Self {
                Self([$t::one(), $t::one(), $t::one(), $t::one()])
            }

            pub fn new(x: $t, y: $t, z: $t, w: $t) -> Self {
                Self([x, y, z, w])
            }

            pub fn identity() -> Self {
                Self([$t::zero(), $t::zero(), $t::zero(), $t::one()])
            }

            pub fn x(&self) -> $t {
                self[0]
            }

            pub fn y(&self) -> $t {
                self[1]
            }

            pub fn z(&self) -> $t {
                self[2]
            }

            pub fn w(&self) -> $t {
                self[3]
            }

            pub fn dot(&self, rhs: Self) -> $t {
                (self[0] * rhs[0]) + (self[1] * rhs[1]) + (self[2] * rhs[2]) + (self[3] * rhs[3])
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

            pub fn conjugate(&self) -> Self {
                Self([-self[0], -self[1], -self[2], self[3]])
            }

            pub fn inverse(&self) -> Self {
                self.conjugate() / self.dot(*self)
            }

            pub fn from_rotation_axis(rad: $t, axis: Vec3<$t>) -> Self {
                let half_angle = rad * $t::splat(0.5);
                let (sin, cos) = half_angle.sin_cos();
                Self([axis[0] * sin, axis[1] * sin, axis[2] * sin, cos])
            }

            pub fn get_rotation_axis(&self) -> ($t, Vec3<$t>) {
                let sin_sq = $t::one() - self[3] * self[3];
                let angle = sin_sq.sqrt().atan2(self[3]) * 2.0;
                let axis = Vec3::<$t>::new(self[0], self[1], self[2]) / sin_sq.sqrt();
                (angle, axis)
            }

            pub fn from_rotation_between(from: Vec3<$t>, to: Vec3<$t>) -> Self {
                let w = $t::one() + to.dot(from);
                let [x, y, z] = from.cross(to).into();
                Self([x, y, z, w]).normalized()
            }

            pub fn from_euler(roll: $t, pitch: $t, yaw: $t) -> Self {
                let half_roll = roll * $t::splat(0.5);
                let half_pitch = pitch * $t::splat(0.5);
                let half_yaw = yaw * $t::splat(0.5);
                let (sin_roll, cos_roll) = half_roll.sin_cos();
                let (sin_pitch, cos_pitch) = half_pitch.sin_cos();
                let (sin_yaw, cos_yaw) = half_yaw.sin_cos();
                Self([
                    cos_roll * cos_pitch * cos_yaw + sin_roll * sin_pitch * sin_yaw,
                    sin_roll * cos_pitch * cos_yaw - cos_roll * sin_pitch * sin_yaw,
                    cos_roll * sin_pitch * cos_yaw + sin_roll * cos_pitch * sin_yaw,
                    cos_roll * cos_pitch * sin_yaw - sin_roll * sin_pitch * cos_yaw,
                ])
            }

            pub fn to_mat3(&self) -> Mat3<$t> {
                let [x, y, z, w] = self.0;
                let xx = x * x;
                let yy = y * y;
                let zz = z * z;
                let xy = x * y;
                let xz = x * z;
                let yz = y * z;
                let wx = w * x;
                let wy = w * y;
                let wz = w * z;
                Mat3::<$t>::new(
                    $t::one() - (yy + zz),
                    xy + wz,
                    xz - wy,
                    xy - wz,
                    $t::one() - (xx + zz),
                    yz + wx,
                    xz + wy,
                    yz - wx,
                    $t::one() - (xx + yy),
                )
            }

            pub fn to_mat4(&self) -> Mat4<$t> {
                self.to_mat3().extended(Vec4::<$t>::new($t::zero(), $t::zero(), $t::zero(), $t::one()))
            }
        }

        // display
        // ---------------------------------------------------------------------------------------------------
        impl std::fmt::Display for Quat<$t> {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "(x, y, z, w): [{:>8.4}, {:>8.4}, {:>8.4}, {:>8.4}]", self[0], self[1], self[2], self[3])
            }
        }

        // from
        // ---------------------------------------------------------------------------------------------------
        impl From<[$t; 4]> for Quat<$t> {
            fn from(v: [$t; 4]) -> Self {
                Self(v)
            }
        }

        impl From<Quat<$t>> for [$t; 4] {
            fn from(q: Quat<$t>) -> Self {
                q.0
            }
        }

        // index
        // ---------------------------------------------------------------------------------------------------
        impl Index<usize> for Quat<$t> {
            type Output = $t;
            fn index(&self, index: usize) -> &Self::Output {
                &self.0[index]
            }
        }

        impl IndexMut<usize> for Quat<$t> {
            fn index_mut(&mut self, index: usize) -> &mut $t {
                &mut self.0[index]
            }
        }

        // addition
        // ---------------------------------------------------------------------------------------------------
        impl Add for Quat<$t> {
            type Output = Self;
            fn add(self, rhs: Self) -> Self {
                Self([self[0] + rhs[0], self[1] + rhs[1], self[2] + rhs[2], self[3] + rhs[3]])
            }
        }

        impl AddAssign for Quat<$t> {
            fn add_assign(&mut self, rhs: Self) {
                *self = *self + rhs;
            }
        }

        // subtraction
        // ---------------------------------------------------------------------------------------------------
        impl Sub for Quat<$t> {
            type Output = Self;
            fn sub(self, rhs: Self) -> Self {
                Self([self[0] - rhs[0], self[1] - rhs[1], self[2] - rhs[2], self[3] - rhs[3]])
            }
        }

        impl SubAssign for Quat<$t> {
            fn sub_assign(&mut self, rhs: Self) {
                *self = *self - rhs;
            }
        }

        // multiplication
        // ---------------------------------------------------------------------------------------------------
        impl Mul for Quat<$t> {
            type Output = Self;
            fn mul(self, rhs: Self) -> Self {
                let [p0, p1, p2, pa] = self.0;
                let [q0, q1, q2, qa] = rhs.0;
                Self([
                    pa * q0 + qa * p0 + p1 * q2 - p2 * q1, // x
                    pa * q1 + qa * p1 + p2 * q0 - p0 * q2, // y
                    pa * q2 + qa * p2 + p0 * q1 - p1 * q0, // z
                    pa * qa - p0 * q0 - p1 * q1 - p2 * q2, // w
                ])
            }
        }

        impl MulAssign for Quat<$t> {
            fn mul_assign(&mut self, rhs: Self) {
                *self = *self * rhs;
            }
        }

        // vector multiplication
        // ---------------------------------------------------------------------------------------------------
        impl Mul<Vec3<$t>> for Quat<$t> {
            type Output = Vec3<$t>;
            fn mul(self, rhs: Vec3<$t>) -> Self::Output {
                let [p0, p1, p2, pa] = self.0;
                let [x0, x1, x2] = rhs.into();
                let q0 = pa * x0 + p1 * x2 - p2 * x1;
                let q1 = pa * x1 + p2 * x0 - p0 * x2;
                let q2 = pa * x2 + p0 * x1 - p1 * x0;
                let q3 = -p0 * x0 - p1 * x1 - p2 * x2;
                let x = q3 * -p0 + pa * q0 - q1 * p2 + q2 * p1;
                let y = q3 * -p1 + pa * q1 - q2 * p0 + q0 * p2;
                let z = q3 * -p2 + pa * q2 - q0 * p1 + q1 * p0;
                Vec3::<$t>::new(x, y, z)
            }
        }

        // scalar multiplication
        // ---------------------------------------------------------------------------------------------------
        impl Mul<$t> for Quat<$t> {
            type Output = Self;
            fn mul(self, rhs: $t) -> Self {
                Self([self[0] * rhs, self[1] * rhs, self[2] * rhs, self[3] * rhs])
            }
        }

        impl MulAssign<$t> for Quat<$t> {
            fn mul_assign(&mut self, rhs: $t) {
                *self = *self * rhs;
            }
        }

        // scalar division
        // ---------------------------------------------------------------------------------------------------
        impl Div<$t> for Quat<$t> {
            type Output = Self;
            fn div(self, rhs: $t) -> Self {
                Self([self[0] / rhs, self[1] / rhs, self[2] / rhs, self[3] / rhs])
            }
        }

        impl DivAssign<$t> for Quat<$t> {
            fn div_assign(&mut self, rhs: $t) {
                *self = *self / rhs;
            }
        }

        // negation
        // ---------------------------------------------------------------------------------------------------
        impl Neg for Quat<$t> {
            type Output = Self;
            fn neg(self) -> Self {
                Self([-self[0], -self[1], -self[2], -self[3]])
            }
        }
    };
}

impl_quat!(f32);
impl_quat!(f64);
