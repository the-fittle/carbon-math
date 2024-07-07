use crate::*;
use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign};

macro_rules! impl_mat4 {
    ($t:ident) => {
        impl Mat4<$t> {
            pub fn new(m00: $t, m01: $t, m02: $t, m03: $t, m10: $t, m11: $t, m12: $t, m13: $t, m20: $t, m21: $t, m22: $t, m23: $t, m30: $t, m31: $t, m32: $t, m33: $t) -> Self {
                Self([
                    [m00, m01, m02, m03].into(),
                    [m10, m11, m12, m13].into(),
                    [m20, m21, m22, m23].into(),
                    [m30, m31, m32, m33].into(),
                ])
            }

            pub fn new_cols<C>(c0: C, c1: C, c2: C, c3: C) -> Self
            where
                C: Into<[$t; 4]>,
            {
                let [c00, c01, c02, c03] = c0.into();
                let [c10, c11, c12, c13] = c1.into();
                let [c20, c21, c22, c23] = c2.into();
                let [c30, c31, c32, c33] = c3.into();
                Self::new(c00, c10, c20, c30, c01, c11, c21, c31, c02, c12, c22, c32, c03, c13, c23, c33)
            }

            pub fn new_rows<R>(r0: R, r1: R, r2: R, r3: R) -> Self
            where
                R: Into<[$t; 4]>,
            {
                let [r00, r01, r02, r03] = r0.into();
                let [r10, r11, r12, r13] = r1.into();
                let [r20, r21, r22, r23] = r2.into();
                let [r30, r31, r32, r33] = r3.into();
                Self::new(r00, r01, r02, r03, r10, r11, r12, r13, r20, r21, r22, r23, r30, r31, r32, r33)
            }

            pub fn zero() -> Self {
                Self([Vec4::<$t>::zero(); 4])
            }

            pub fn one() -> Self {
                Self([Vec4::<$t>::one(); 4])
            }

            pub fn identity() -> Self {
                Self::new(
                    $t::one(),
                    $t::zero(),
                    $t::zero(),
                    $t::zero(),
                    $t::zero(),
                    $t::one(),
                    $t::zero(),
                    $t::zero(),
                    $t::zero(),
                    $t::zero(),
                    $t::one(),
                    $t::zero(),
                    $t::zero(),
                    $t::zero(),
                    $t::zero(),
                    $t::one(),
                )
            }

            pub fn transposed(&self) -> Self {
                let [m00, m01, m02, m03] = self[0].into();
                let [m10, m11, m12, m13] = self[1].into();
                let [m20, m21, m22, m23] = self[2].into();
                let [m30, m31, m32, m33] = self[3].into();
                Self::new(m00, m10, m20, m30, m01, m11, m21, m31, m02, m12, m22, m32, m03, m13, m23, m33)
            }

            pub fn inversed(&self) -> Option<Self> {
                let det = self.determinant();
                if det == $t::zero() {
                    None
                } else {
                    Some(self.adjugate() / det)
                }
            }

            pub fn determinant(&self) -> $t {
                let [m00, m01, m02, m03] = self[0].into();
                let [m10, m11, m12, m13] = self[1].into();
                let [m20, m21, m22, m23] = self[2].into();
                let [m30, m31, m32, m33] = self[3].into();

                m00 * (m11 * (m22 * m33 - m23 * m32) - m12 * (m21 * m33 - m23 * m31) + m13 * (m21 * m32 - m22 * m31))
                    - m01 * (m10 * (m22 * m33 - m23 * m32) - m12 * (m20 * m33 - m23 * m30) + m13 * (m20 * m32 - m22 * m30))
                    + m02 * (m10 * (m21 * m33 - m23 * m31) - m11 * (m20 * m33 - m23 * m30) + m13 * (m20 * m31 - m21 * m30))
                    - m03 * (m10 * (m21 * m32 - m22 * m31) - m11 * (m20 * m32 - m22 * m30) + m12 * (m20 * m31 - m21 * m30))
            }

            pub fn adjugate(&self) -> Self {
                let [m00, m01, m02, m03] = self[0].into();
                let [m10, m11, m12, m13] = self[1].into();
                let [m20, m21, m22, m23] = self[2].into();
                let [m30, m31, m32, m33] = self[3].into();

                Self::new(
                    m11 * (m22 * m33 - m23 * m32) - m12 * (m21 * m33 - m23 * m31) + m13 * (m21 * m32 - m22 * m31),
                    -(m10 * (m22 * m33 - m23 * m32) - m12 * (m20 * m33 - m23 * m30) + m13 * (m20 * m32 - m22 * m30)),
                    m10 * (m21 * m33 - m23 * m31) - m11 * (m20 * m33 - m23 * m30) + m13 * (m20 * m31 - m21 * m30),
                    -(m10 * (m21 * m32 - m22 * m31) - m11 * (m20 * m32 - m22 * m30) + m12 * (m20 * m31 - m21 * m30)),
                    -(m01 * (m22 * m33 - m23 * m32) - m02 * (m21 * m33 - m23 * m31) + m03 * (m21 * m32 - m22 * m31)),
                    m00 * (m22 * m33 - m23 * m32) - m02 * (m20 * m33 - m23 * m30) + m03 * (m20 * m32 - m22 * m30),
                    -(m00 * (m21 * m33 - m23 * m31) - m01 * (m20 * m33 - m23 * m30) + m03 * (m20 * m31 - m21 * m30)),
                    m00 * (m21 * m32 - m22 * m31) - m01 * (m20 * m32 - m22 * m30) + m02 * (m20 * m31 - m21 * m30),
                    m01 * (m12 * m33 - m13 * m32) - m02 * (m11 * m33 - m13 * m31) + m03 * (m11 * m32 - m12 * m31),
                    -(m00 * (m12 * m33 - m13 * m32) - m02 * (m10 * m33 - m13 * m30) + m03 * (m10 * m32 - m12 * m30)),
                    m00 * (m11 * m33 - m13 * m31) - m01 * (m10 * m33 - m13 * m30) + m03 * (m10 * m31 - m11 * m30),
                    -(m00 * (m11 * m32 - m12 * m31) - m01 * (m10 * m32 - m12 * m30) + m02 * (m10 * m31 - m11 * m30)),
                    -(m01 * (m12 * m23 - m13 * m22) - m02 * (m11 * m23 - m13 * m21) + m03 * (m11 * m22 - m12 * m21)),
                    m00 * (m12 * m23 - m13 * m22) - m02 * (m10 * m23 - m13 * m20) + m03 * (m10 * m22 - m12 * m20),
                    -(m00 * (m11 * m23 - m13 * m21) - m01 * (m10 * m23 - m13 * m20) + m03 * (m10 * m21 - m11 * m20)),
                    m00 * (m11 * m22 - m12 * m21) - m01 * (m10 * m22 - m12 * m20) + m02 * (m10 * m21 - m11 * m20),
                )
            }

            pub fn from_diagonal(diagonal: Vec4<$t>) -> Self {
                Self::new(
                    diagonal[0],
                    $t::zero(),
                    $t::zero(),
                    $t::zero(),
                    $t::zero(),
                    diagonal[1],
                    $t::zero(),
                    $t::zero(),
                    $t::zero(),
                    $t::zero(),
                    diagonal[2],
                    $t::zero(),
                    $t::zero(),
                    $t::zero(),
                    $t::zero(),
                    diagonal[3],
                )
            }

            pub fn get_diagonal(&self) -> Vec4<$t> {
                Vec4::<$t>::new(self[0][0], self[1][1], self[2][2], self[3][3])
            }

            pub fn from_translation(translation: Vec3<$t>) -> Self {
                Self::new(
                    $t::one(),
                    $t::zero(),
                    $t::zero(),
                    $t::zero(),
                    $t::zero(),
                    $t::one(),
                    $t::zero(),
                    $t::zero(),
                    $t::zero(),
                    $t::zero(),
                    $t::one(),
                    $t::zero(),
                    translation[0],
                    translation[1],
                    translation[2],
                    $t::one(),
                )
            }

            pub fn get_translation(&self) -> Vec3<$t> {
                Vec3::<$t>::new(self[3][0], self[3][1], self[3][2])
            }

            pub fn translate<T>(&mut self, translation: T)
			where
				T: Into<Vec3<$t>>,
			{
                *self *= Self::from_translation(translation.into());
            }

            pub fn from_rotation_x(rad: $t) -> Self {
                let (sin, cos) = rad.sin_cos();
                Self::new(
                    $t::one(),
                    $t::zero(),
                    $t::zero(),
                    $t::zero(),
                    $t::zero(),
                    cos,
                    sin,
                    $t::zero(),
                    $t::zero(),
                    -sin,
                    cos,
                    $t::zero(),
                    $t::zero(),
                    $t::zero(),
                    $t::zero(),
                    $t::one(),
                )
            }

            pub fn from_rotation_y(rad: $t) -> Self {
                let (sin, cos) = rad.sin_cos();
                Self::new(
                    cos,
                    $t::zero(),
                    -sin,
                    $t::zero(),
                    $t::zero(),
                    $t::one(),
                    $t::zero(),
                    $t::zero(),
                    sin,
                    $t::zero(),
                    cos,
                    $t::zero(),
                    $t::zero(),
                    $t::zero(),
                    $t::zero(),
                    $t::one(),
                )
            }

            pub fn from_rotation_z(rad: $t) -> Self {
                let (sin, cos) = rad.sin_cos();
                Self::new(
                    cos,
                    sin,
                    $t::zero(),
                    $t::zero(),
                    -sin,
                    cos,
                    $t::zero(),
                    $t::zero(),
                    $t::zero(),
                    $t::zero(),
                    $t::one(),
                    $t::zero(),
                    $t::zero(),
                    $t::zero(),
                    $t::zero(),
                    $t::one(),
                )
            }

            pub fn rotate_x(&mut self, rad: $t) {
                *self *= Self::from_rotation_x(rad);
            }

            pub fn rotate_y(&mut self, rad: $t) {
                *self *= Self::from_rotation_y(rad);
            }

            pub fn rotate_z(&mut self, rad: $t) {
                *self *= Self::from_rotation_z(rad);
            }

            pub fn from_rotor(rotor: Rot3<$t>) -> Self {
                rotor.to_mat4()
            }

            pub fn from_quat(quat: Quat<$t>) -> Self {
                quat.to_mat4()
            }

            pub fn from_euler(roll: $t, pitch: $t, yaw: $t) -> Self {
                let (sin_r, cos_r) = roll.sin_cos();
                let (sin_p, cos_p) = pitch.sin_cos();
                let (sin_y, cos_y) = yaw.sin_cos();

                let m00 = cos_y * cos_r + sin_p * sin_y * sin_r;
                let m01 = cos_r * sin_p * sin_y - cos_y * sin_r;
                let m02 = cos_p * sin_y;
                let m10 = cos_p * sin_r;
                let m11 = cos_p * cos_r;
                let m12 = -sin_p;
                let m20 = -cos_r * sin_y + cos_y * sin_p * sin_r;
                let m21 = cos_y * cos_r * sin_p + sin_y * sin_r;
                let m22 = cos_p * cos_y;

                Self::new(
                    m00,
                    m01,
                    m02,
                    $t::zero(),
                    m10,
                    m11,
                    m12,
                    $t::zero(),
                    m20,
                    m21,
                    m22,
                    $t::zero(),
                    $t::zero(),
                    $t::zero(),
                    $t::zero(),
                    $t::one(),
                )
            }

            pub fn get_euler(&self) -> Vec3<$t> {
                let m = self.transposed();
                let y = (-m[2][0]).asin();
                let x = m[2][1].atan2(m[2][2]);
                let z = m[1][0].atan2(m[0][0]);
                Vec3::<$t>::new(x, y, z)
            }

            pub fn from_rotation_axis(axis: Vec3<$t>, rad: $t) -> Self {
                let (sin, cos) = rad.sin_cos();
                let axis = axis.normalized();
                let one_minus_cos = $t::one() - cos;

                Self::new(
                    one_minus_cos * axis[0] * axis[0] + cos,
                    one_minus_cos * axis[0] * axis[1] + sin * axis[2],
                    one_minus_cos * axis[0] * axis[2] - sin * axis[1],
                    $t::zero(),
                    one_minus_cos * axis[0] * axis[1] - sin * axis[2],
                    one_minus_cos * axis[1] * axis[1] + cos,
                    one_minus_cos * axis[1] * axis[2] + sin * axis[0],
                    $t::zero(),
                    one_minus_cos * axis[0] * axis[2] + sin * axis[1],
                    one_minus_cos * axis[1] * axis[2] - sin * axis[0],
                    one_minus_cos * axis[2] * axis[2] + cos,
                    $t::zero(),
                    $t::zero(),
                    $t::zero(),
                    $t::zero(),
                    $t::one(),
                )
            }

            pub fn get_rotation_axis(&self) -> (Vec3<$t>, $t) {
                let x = self[0].truncated().normalized();
                let y = self[1].truncated().normalized();
                let z = self[2].truncated().normalized();
                let cos = (x[0] + y[1] + z[2] - $t::one()) * $t::splat(0.5);
                let sin = ($t::one() - cos * cos).sqrt();
                let rad = sin.acos();
                let axis = Vec3::<$t>::new(z[1] - y[2], x[2] - z[0], y[0] - x[1]);
                (axis.normalized(), rad)
            }

            pub fn rotate(&mut self, axis: Vec3<$t>, rad: $t) {
                *self *= Self::from_rotation_axis(axis, rad);
            }

            pub fn from_scale(scale: Vec3<$t>) -> Self {
                Self::new(
                    scale[0],
                    $t::zero(),
                    $t::zero(),
                    $t::zero(),
                    $t::zero(),
                    scale[1],
                    $t::zero(),
                    $t::zero(),
                    $t::zero(),
                    $t::zero(),
                    scale[2],
                    $t::zero(),
                    $t::zero(),
                    $t::zero(),
                    $t::zero(),
                    $t::one(),
                )
            }

            pub fn get_scale(&self) -> Vec3<$t> {
                Vec3::<$t>::new(self[0][0], self[1][1], self[2][2])
            }

            pub fn scale(&mut self, scale: Vec3<$t>) {
                *self *= Self::from_scale(scale);
            }

            pub fn look_at(eye: Vec3<$t>, target: Vec3<$t>, up: Vec3<$t>) -> Self {
                let f = (target - eye).normalized();
                let r = f.cross(up).normalized();
                let u = r.cross(f);

                Self::new(
                    r[0],
                    (u[0]),
                    f[0],
                    $t::zero(),

                    r[1],
                    (u[1]),
                    f[1],
                    $t::zero(),

                    r[2],
                    (u[2]),
                    f[2],
                    $t::zero(),

                    r.dot(eye),
                    (u.dot(eye)),
                    f.dot(eye),
                    $t::one(),
                )
            }

            // -1.0 - 1.0
            // pub fn perspective(fov: $t, aspect: $t, near: $t, far: $t) -> Self {
            //     let s = $t::one() / (fov * $t::splat(0.5)).tan();
            //     let n_diff_f = $t::one() / (near - far);

            //     Self::new(
            //         s / aspect,
            //         $t::zero(),
            //         $t::zero(),
            //         $t::zero(),
            //         $t::zero(),
            //         s,
            //         $t::zero(),
            //         $t::zero(),
            //         $t::zero(),
            //         $t::zero(),
            //         (far + near) * n_diff_f,
            //         -$t::one(),
            //         $t::zero(),
            //         $t::zero(),
            //         ($t::splat(2.0) * far * near) * n_diff_f,
            //         $t::zero(),
            //     )
            // }

            pub fn perspective(fov: $t, aspect: $t, near: $t, far: $t) -> Self {
                let s = $t::one() / (fov * $t::splat(0.5)).tan();
                let n_diff_f = $t::one() / (near - far);

                Self::new(
                    -(s / aspect),
                    $t::zero(),
                    $t::zero(),
                    $t::zero(),

                    $t::zero(),
                    -(s),
                    $t::zero(),
                    $t::zero(),

                    $t::zero(),
                    $t::zero(),
                    (far) * n_diff_f,
                    -$t::one(),

                    $t::zero(),
                    $t::zero(),
                    (far * near) * n_diff_f,
                    $t::zero(),
                )
            }

			// 0.0 - 1.0
            pub fn perspective_infinite(fov: $t, aspect: $t, near: $t, far: $t) -> Self {
                let s = $t::one() / (fov * $t::splat(0.5)).tan();
                let n_diff_f = $t::one() / (near - far);

                Self::new(
                    -(s / aspect),
                    $t::zero(),
                    $t::zero(),
                    $t::zero(),

                    $t::zero(),
                    -(s),
                    $t::zero(),
                    $t::zero(),

                    $t::zero(),
                    $t::zero(),
                    -$t::one(),
                    -$t::one(),

                    $t::zero(),
                    $t::zero(),
                    -(near),
                    $t::zero(),
                )
            }
        }


        // display
        // ---------------------------------------------------------------------------------------------------
        impl std::fmt::Display for Mat4<$t> {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                let [m00, m01, m02, m03] = self[0].into();
                let [m10, m11, m12, m13] = self[1].into();
                let [m20, m21, m22, m23] = self[2].into();
                let [m30, m31, m32, m33] = self[3].into();

                write!(
                    f,
                    "[{:>8.4} {:>8.4} {:>8.4} {:>8.4}]\n[{:>8.4} {:>8.4} {:>8.4} {:>8.4}]\n[{:>8.4} {:>8.4} {:>8.4} {:>8.4}]\n[{:>8.4} {:>8.4} {:>8.4} {:>8.4}]",
                    m00, m01, m02, m03, m10, m11, m12, m13, m20, m21, m22, m23, m30, m31, m32, m33,
                )
            }
        }

        // from
        // ---------------------------------------------------------------------------------------------------
        impl From<[Vec4<$t>; 4]> for Mat4<$t> {
            fn from(v: [Vec4<$t>; 4]) -> Self {
                Self(v)
            }
        }

        impl From<Mat4<$t>> for [Vec4<$t>; 4] {
            fn from(m: Mat4<$t>) -> Self {
                m.0
            }
        }

        impl From<[[$t; 4]; 4]> for Mat4<$t> {
            fn from(v: [[$t; 4]; 4]) -> Self {
                Self::new(
                    v[0][0], v[0][1], v[0][2], v[0][3], v[1][0], v[1][1], v[1][2], v[1][3], v[2][0], v[2][1], v[2][2], v[2][3], v[3][0], v[3][1], v[3][2], v[3][3],
                )
            }
        }

        impl From<Mat4<$t>> for [[$t; 4]; 4] {
            fn from(m: Mat4<$t>) -> Self {
                [
                    [m[0][0], m[0][1], m[0][2], m[0][3]],
                    [m[1][0], m[1][1], m[1][2], m[1][3]],
                    [m[2][0], m[2][1], m[2][2], m[2][3]],
                    [m[3][0], m[3][1], m[3][2], m[3][3]],
                ]
            }
        }

        impl From<Quat<$t>> for Mat4<$t> {
            fn from(q: Quat<$t>) -> Self {
                q.to_mat4()
            }
        }

        impl From<Rot3<$t>> for Mat4<$t> {
            fn from(r: Rot3<$t>) -> Self {
                r.to_mat4()
            }
        }

        // index
        // ---------------------------------------------------------------------------------------------------
        impl Index<usize> for Mat4<$t> {
            type Output = Vec4<$t>;

            fn index(&self, index: usize) -> &Self::Output {
                &self.0[index]
            }
        }

        impl IndexMut<usize> for Mat4<$t> {
            fn index_mut(&mut self, index: usize) -> &mut Vec4<$t> {
                &mut self.0[index]
            }
        }

        // addition
        // ---------------------------------------------------------------------------------------------------
        impl Add for Mat4<$t> {
            type Output = Self;
            fn add(self, rhs: Self) -> Self::Output {
                Self::new_cols(self[0] + rhs[0], self[1] + rhs[1], self[2] + rhs[2], self[3] + rhs[3])
            }
        }

        impl AddAssign for Mat4<$t> {
            fn add_assign(&mut self, rhs: Self) {
                *self = *self + rhs;
            }
        }

        // subtraction
        // ---------------------------------------------------------------------------------------------------
        impl Sub for Mat4<$t> {
            type Output = Self;
            fn sub(self, rhs: Self) -> Self::Output {
                Self::new_cols(self[0] - rhs[0], self[1] - rhs[1], self[2] - rhs[2], self[3] - rhs[3])
            }
        }

        impl SubAssign for Mat4<$t> {
            fn sub_assign(&mut self, rhs: Self) {
                *self = *self - rhs;
            }
        }

        // multiplication
        // ---------------------------------------------------------------------------------------------------
        impl Mul for Mat4<$t> {
            type Output = Self;
            fn mul(self, rhs: Self) -> Self::Output {
                let sa = self[0];
                let sb = self[1];
                let sc = self[2];
                let sd = self[3];
                let oa = rhs[0];
                let ob = rhs[1];
                let oc = rhs[2];
                let od = rhs[3];
                Self::new(
                    sa[0] * oa[0] + sb[0] * oa[1] + sc[0] * oa[2] + sd[0] * oa[3],
                    sa[1] * oa[0] + sb[1] * oa[1] + sc[1] * oa[2] + sd[1] * oa[3],
                    sa[2] * oa[0] + sb[2] * oa[1] + sc[2] * oa[2] + sd[2] * oa[3],
                    sa[3] * oa[0] + sb[3] * oa[1] + sc[3] * oa[2] + sd[3] * oa[3],
                    sa[0] * ob[0] + sb[0] * ob[1] + sc[0] * ob[2] + sd[0] * ob[3],
                    sa[1] * ob[0] + sb[1] * ob[1] + sc[1] * ob[2] + sd[1] * ob[3],
                    sa[2] * ob[0] + sb[2] * ob[1] + sc[2] * ob[2] + sd[2] * ob[3],
                    sa[3] * ob[0] + sb[3] * ob[1] + sc[3] * ob[2] + sd[3] * ob[3],
                    sa[0] * oc[0] + sb[0] * oc[1] + sc[0] * oc[2] + sd[0] * oc[3],
                    sa[1] * oc[0] + sb[1] * oc[1] + sc[1] * oc[2] + sd[1] * oc[3],
                    sa[2] * oc[0] + sb[2] * oc[1] + sc[2] * oc[2] + sd[2] * oc[3],
                    sa[3] * oc[0] + sb[3] * oc[1] + sc[3] * oc[2] + sd[3] * oc[3],
                    sa[0] * od[0] + sb[0] * od[1] + sc[0] * od[2] + sd[0] * od[3],
                    sa[1] * od[0] + sb[1] * od[1] + sc[1] * od[2] + sd[1] * od[3],
                    sa[2] * od[0] + sb[2] * od[1] + sc[2] * od[2] + sd[2] * od[3],
                    sa[3] * od[0] + sb[3] * od[1] + sc[3] * od[2] + sd[3] * od[3],
                )
            }
        }

        impl MulAssign for Mat4<$t> {
            fn mul_assign(&mut self, rhs: Self) {
                *self = *self * rhs;
            }
        }

        // vector multiplication
        // ---------------------------------------------------------------------------------------------------
        impl Mul<Vec4<$t>> for Mat4<$t> {
            type Output = Vec4<$t>;
            fn mul(self, rhs: Vec4<$t>) -> Self::Output {
                let sa = self[0];
                let sb = self[1];
                let sc = self[2];
                let sd = self[3];
                Vec4::<$t>::new(
                    sa[0] * rhs[0] + sb[0] * rhs[1] + sc[0] * rhs[2] + sd[0] * rhs[3],
                    sa[1] * rhs[0] + sb[1] * rhs[1] + sc[1] * rhs[2] + sd[1] * rhs[3],
                    sa[2] * rhs[0] + sb[2] * rhs[1] + sc[2] * rhs[2] + sd[2] * rhs[3],
                    sa[3] * rhs[0] + sb[3] * rhs[1] + sc[3] * rhs[2] + sd[3] * rhs[3],
                )
            }
        }

        // scalar multiplication
        // ---------------------------------------------------------------------------------------------------
        impl Mul<$t> for Mat4<$t> {
            type Output = Self;
            fn mul(self, rhs: $t) -> Self::Output {
                Self::new_cols(self[0] * rhs, self[1] * rhs, self[2] * rhs, self[3] * rhs)
            }
        }

        // scalar division
        // ---------------------------------------------------------------------------------------------------
        impl Div<$t> for Mat4<$t> {
            type Output = Self;
            fn div(self, rhs: $t) -> Self::Output {
                Self::new_cols(self[0] / rhs, self[1] / rhs, self[2] / rhs, self[3] / rhs)
            }
        }

        impl DivAssign<$t> for Mat4<$t> {
            fn div_assign(&mut self, rhs: $t) {
                *self = *self / rhs;
            }
        }

        // negation
        // ---------------------------------------------------------------------------------------------------
        impl Neg for Mat4<$t> {
            type Output = Self;
            fn neg(mut self) -> Self::Output {
                for i in 0..4 {
                    self[0][i] = -self[0][i];
                    self[1][i] = -self[1][i];
                    self[2][i] = -self[2][i];
                    self[3][i] = -self[3][i];
                }
                self
            }
        }
    };
}

impl_mat4!(f32);
impl_mat4!(f64);
