use crate::*;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

macro_rules! impl_rot3 {
    ($t:ident) => {
        impl Rot3<$t> {
            pub fn new(s: $t, bi: Bivec3<$t>) -> Self {
                Self(s, bi)
            }

            pub fn zero() -> Self {
                Self($t::zero(), Bivec3::<$t>::zero())
            }

            pub fn one() -> Self {
                Self($t::one(), Bivec3::<$t>::one())
            }

            pub fn identity() -> Self {
                Self($t::one(), Bivec3::<$t>::zero())
            }

            pub fn s(&self) -> $t {
                self.0
            }

            pub fn bi(&self) -> Bivec3<$t> {
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

            #[deprecated(note = "Use from_rotation_z instead")]
            pub fn from_rotation_xy(rad: $t) -> Self {
                Self::from_rotation_bi(-rad, Bivec3::<$t>::unit_xy())
            }

            #[deprecated(note = "Use from_rotation_y instead")]
            pub fn from_rotation_xz(rad: $t) -> Self {
                Self::from_rotation_bi(rad, Bivec3::<$t>::unit_xz())
            }

            #[deprecated(note = "Use from_rotation_x instead")]
            pub fn from_rotation_yz(rad: $t) -> Self {
                Self::from_rotation_bi(-rad, Bivec3::<$t>::unit_yz())
            }

            pub fn from_rotation_x(rad: $t) -> Self {
                Self::from_rotation_bi(-rad, Bivec3::<$t>::unit_yz())
            }

            pub fn from_rotation_y(rad: $t) -> Self {
                Self::from_rotation_bi(rad, Bivec3::<$t>::unit_xz())
            }

            pub fn from_rotation_z(rad: $t) -> Self {
                Self::from_rotation_bi(-rad, Bivec3::<$t>::unit_xy())
            }

            pub fn from_rotation_bi(rad: $t, bi: Bivec3<$t>) -> Self {
                let half_angle = rad * $t::splat(0.5);
                let (sin, cos) = half_angle.sin_cos();
                Self(cos, bi * -sin).normalized()
            }

            pub fn from_rotation_between(from: Vec3<$t>, to: Vec3<$t>) -> Self {
                Self::new($t::one() + to.dot(from), to.wedge(from)).normalized()
            }

            pub fn from_euler(roll: $t, pitch: $t, yaw: $t) -> Self {
                Self::from_rotation_y(yaw) * Self::from_rotation_x(pitch) * Self::from_rotation_z(roll)
            }

            pub fn to_mat3(&self) -> Mat3<$t> {
                let s = self.0;
                let bi = self.1;
                let [xy, xz, yz] = bi.0;
                let s2 = s * s;
                let xy2 = xy * xy;
                let xz2 = xz * xz;
                let yz2 = yz * yz;
                let s_xy = s * xy;
                let s_xz = s * xz;
                let s_yz = s * yz;
                let xy_xz = xy * xz;
                let xy_yz = xy * yz;
                let xz_yz = xz * yz;
                let d = $t::splat(2.0);
                Mat3::<$t>::new(
                    s2 - xy2 - xz2 + yz2,
                    -d * (xz_yz + s_xy),
                    d * (xy_yz - s_xz),
                    d * (s_xy - xz_yz),
                    s2 - xy2 + xz2 - yz2,
                    -d * (s_yz + xy_xz),
                    d * (s_xz + xy_yz),
                    d * (s_yz - xy_xz),
                    s2 + xy2 - xz2 - yz2,
                )
            }

            pub fn to_mat4(&self) -> Mat4<$t> {
                self.to_mat3()
                    .extended(Vec4::<$t>::new($t::zero(), $t::zero(), $t::zero(), $t::one()))
            }

            pub fn to_quat(self) -> Quat<$t> {
                Quat::<$t>::new(-self.1[0], -self.1[1], -self.1[2], self.0)
            }
        }

        // display
        // ---------------------------------------------------------------------------------------------------
        impl std::fmt::Display for Rot3<$t> {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                let xy = self.1[0];
                let xz = self.1[1];
                let yz = self.1[2];
                write!(
                    f,
                    "(xy, xz, yz, s): [{:>8.4}, {:>8.4}, {:>8.4}, {:>8.4}]",
                    xy, xz, yz, self.0
                )
            }
        }

        // addition
        // ---------------------------------------------------------------------------------------------------
        impl Add for Rot3<$t> {
            type Output = Self;
            fn add(self, rhs: Self) -> Self {
                Self(self.0 + rhs.0, self.1 + rhs.1)
            }
        }

        impl AddAssign for Rot3<$t> {
            fn add_assign(&mut self, rhs: Self) {
                *self = *self + rhs;
            }
        }

        // subtraction
        // ---------------------------------------------------------------------------------------------------
        impl Sub for Rot3<$t> {
            type Output = Self;

            fn sub(self, rhs: Self) -> Self {
                Self(self.0 - rhs.0, self.1 - rhs.1)
            }
        }

        impl SubAssign for Rot3<$t> {
            fn sub_assign(&mut self, rhs: Self) {
                *self = *self - rhs;
            }
        }

        // multiplication
        // ---------------------------------------------------------------------------------------------------
        impl Mul for Rot3<$t> {
            type Output = Self;
            fn mul(self, rhs: Self) -> Self {
                let p3 = self.0;
                let pbi = self.1;
                let [p0, p1, p2] = pbi.0;
                let q3 = rhs.0;
                let qbi = rhs.1;
                let [q0, q1, q2] = qbi.0;
                let w = p3 * q3 - p0 * q0 - p1 * q1 - p2 * q2;
                let x = p0 * q3 + p3 * q0 + p2 * q1 - p1 * q2;
                let y = p1 * q3 + p3 * q1 - p2 * q0 + p0 * q2;
                let z = p2 * q3 + p3 * q2 + p1 * q0 - p0 * q1;
                Self(w, Bivec3::<$t>::new(x, y, z))
            }
        }

        impl MulAssign for Rot3<$t> {
            fn mul_assign(&mut self, rhs: Self) {
                *self = *self * rhs;
            }
        }

        // vector multiplication
        // ---------------------------------------------------------------------------------------------------
        impl Mul<Vec3<$t>> for Rot3<$t> {
            type Output = Vec3<$t>;
            fn mul(self, rhs: Vec3<$t>) -> Self::Output {
                let p3 = self.0;
                let bi = self.1;
                let [p0, p1, p2] = bi.0;
                let [x0, x1, x2] = rhs.into();
                let q0 = p3 * x0 + x1 * p0 + x2 * p1;
                let q1 = p3 * x1 - x0 * p0 + x2 * p2;
                let q2 = p3 * x2 - x0 * p1 - x1 * p2;
                let q3 = x0 * p2 - x1 * p1 + x2 * p0;
                let x = p3 * q0 + q1 * p0 + q2 * p1 + q3 * p2;
                let y = p3 * q1 - q0 * p0 - q3 * p1 + q2 * p2;
                let z = p3 * q2 + q3 * p0 - q0 * p1 - q1 * p2;
                Vec3::<$t>::new(x, y, z)
            }
        }

        // scalar multiplication
        // ---------------------------------------------------------------------------------------------------
        impl Mul<$t> for Rot3<$t> {
            type Output = Self;
            fn mul(self, rhs: $t) -> Self::Output {
                Self(self.0 * rhs, self.1 * rhs)
            }
        }

        impl Mul<Rot3<$t>> for $t {
            type Output = Rot3<$t>;
            fn mul(self, rhs: Rot3<$t>) -> Self::Output {
                rhs * self
            }
        }

        impl MulAssign<$t> for Rot3<$t> {
            fn mul_assign(&mut self, rhs: $t) {
                *self = *self * rhs;
            }
        }

        // scalar division
        // ---------------------------------------------------------------------------------------------------
        impl Div<$t> for Rot3<$t> {
            type Output = Self;
            fn div(self, rhs: $t) -> Self {
                Self(self.0 / rhs, self.1 / rhs)
            }
        }

        impl DivAssign<$t> for Rot3<$t> {
            fn div_assign(&mut self, rhs: $t) {
                *self = *self / rhs;
            }
        }

        // negation
        // ---------------------------------------------------------------------------------------------------
        impl Neg for Rot3<$t> {
            type Output = Self;
            fn neg(self) -> Self {
                Self(-self.0, -self.1)
            }
        }
    };
}

impl_rot3!(f32);
impl_rot3!(f64);
