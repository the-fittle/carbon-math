mod mat2;
mod mat3;
mod mat4;
pub use mat2::*;
pub use mat3::*;
pub use mat4::*;

use crate::*;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Mat2<S>(pub [Vec2<S>; 2]);
unsafe impl<S> bytemuck::Zeroable for Mat2<S> where S: bytemuck::Zeroable {}
unsafe impl<S> bytemuck::Pod for Mat2<S> where S: bytemuck::Pod {}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Mat3<S>(pub [Vec3<S>; 3]);
unsafe impl<S> bytemuck::Zeroable for Mat3<S> where S: bytemuck::Zeroable {}
unsafe impl<S> bytemuck::Pod for Mat3<S> where S: bytemuck::Pod {}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Mat4<S>(pub [Vec4<S>; 4]);
unsafe impl<S> bytemuck::Zeroable for Mat4<S> where S: bytemuck::Zeroable {}
unsafe impl<S> bytemuck::Pod for Mat4<S> where S: bytemuck::Pod {}

// test
// -----------------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mat2_std_ops() {
        let m = Mat2::<f32>::new_cols(Vec2::<f32>::new(1.0, 2.0), Vec2::<f32>::new(3.0, 4.0));

        assert_eq!(
            m + m,
            Mat2::<f32>::new_cols(Vec2::<f32>::new(2.0, 4.0), Vec2::<f32>::new(6.0, 8.0))
        );

        assert_eq!(m - m, Mat2::<f32>::zero());

        assert_eq!(
            m * m,
            Mat2::<f32>::new_cols(Vec2::<f32>::new(7.0, 10.0), Vec2::<f32>::new(15.0, 22.0))
        );

        assert_eq!(
            m * 2.0,
            Mat2::<f32>::new_cols(Vec2::<f32>::new(2.0, 4.0), Vec2::<f32>::new(6.0, 8.0))
        );

        assert_eq!(
            m / 2.0,
            Mat2::<f32>::new_cols(Vec2::<f32>::new(0.5, 1.0), Vec2::<f32>::new(1.5, 2.0))
        );

        assert_eq!(
            -m,
            Mat2::<f32>::new_cols(Vec2::<f32>::new(-1.0, -2.0), Vec2::<f32>::new(-3.0, -4.0))
        );
    }

    #[test]
    fn test_mat3_std_ops() {
        let m = Mat3::<f32>::new_cols(
            Vec3::<f32>::new(1.0, 2.0, 3.0),
            Vec3::<f32>::new(4.0, 5.0, 6.0),
            Vec3::<f32>::new(7.0, 8.0, 9.0),
        );

        assert_eq!(
            m + m,
            Mat3::<f32>::new_cols(
                Vec3::<f32>::new(2.0, 4.0, 6.0),
                Vec3::<f32>::new(8.0, 10.0, 12.0),
                Vec3::<f32>::new(14.0, 16.0, 18.0),
            )
        );

        assert_eq!(m - m, Mat3::<f32>::zero());

        assert_eq!(
            m * m,
            Mat3::<f32>::new_cols(
                Vec3::<f32>::new(30.0, 36.0, 42.0),
                Vec3::<f32>::new(66.0, 81.0, 96.0),
                Vec3::<f32>::new(102.0, 126.0, 150.0),
            )
        );

        assert_eq!(
            m * 2.0,
            Mat3::<f32>::new_cols(
                Vec3::<f32>::new(2.0, 4.0, 6.0),
                Vec3::<f32>::new(8.0, 10.0, 12.0),
                Vec3::<f32>::new(14.0, 16.0, 18.0),
            )
        );

        assert_eq!(
            m / 2.0,
            Mat3::<f32>::new_cols(
                Vec3::<f32>::new(0.5, 1.0, 1.5),
                Vec3::<f32>::new(2.0, 2.5, 3.0),
                Vec3::<f32>::new(3.5, 4.0, 4.5),
            )
        );

        assert_eq!(
            -m,
            Mat3::<f32>::new_cols(
                Vec3::<f32>::new(-1.0, -2.0, -3.0),
                Vec3::<f32>::new(-4.0, -5.0, -6.0),
                Vec3::<f32>::new(-7.0, -8.0, -9.0),
            )
        );
    }

    #[test]
    fn test_mat4_std_ops() {
        let m = Mat4::<f32>::new_cols(
            Vec4::<f32>::new(1.0, 2.0, 3.0, 4.0),
            Vec4::<f32>::new(5.0, 6.0, 7.0, 8.0),
            Vec4::<f32>::new(9.0, 10.0, 11.0, 12.0),
            Vec4::<f32>::new(13.0, 14.0, 15.0, 16.0),
        );

        assert_eq!(
            m + m,
            Mat4::<f32>::new_cols(
                Vec4::<f32>::new(2.0, 4.0, 6.0, 8.0),
                Vec4::<f32>::new(10.0, 12.0, 14.0, 16.0),
                Vec4::<f32>::new(18.0, 20.0, 22.0, 24.0),
                Vec4::<f32>::new(26.0, 28.0, 30.0, 32.0),
            )
        );

        assert_eq!(m - m, Mat4::<f32>::zero());

        assert_eq!(
            m * m,
            Mat4::<f32>::new_cols(
                Vec4::<f32>::new(90.0, 100.0, 110.0, 120.0),
                Vec4::<f32>::new(202.0, 228.0, 254.0, 280.0),
                Vec4::<f32>::new(314.0, 356.0, 398.0, 440.0),
                Vec4::<f32>::new(426.0, 484.0, 542.0, 600.0),
            )
        );

        assert_eq!(
            m * 2.0,
            Mat4::<f32>::new_cols(
                Vec4::<f32>::new(2.0, 4.0, 6.0, 8.0),
                Vec4::<f32>::new(10.0, 12.0, 14.0, 16.0),
                Vec4::<f32>::new(18.0, 20.0, 22.0, 24.0),
                Vec4::<f32>::new(26.0, 28.0, 30.0, 32.0),
            )
        );

        assert_eq!(
            m / 2.0,
            Mat4::<f32>::new_cols(
                Vec4::<f32>::new(0.5, 1.0, 1.5, 2.0),
                Vec4::<f32>::new(2.5, 3.0, 3.5, 4.0),
                Vec4::<f32>::new(4.5, 5.0, 5.5, 6.0),
                Vec4::<f32>::new(6.5, 7.0, 7.5, 8.0),
            )
        );

        assert_eq!(
            -m,
            Mat4::<f32>::new_cols(
                Vec4::<f32>::new(-1.0, -2.0, -3.0, -4.0),
                Vec4::<f32>::new(-5.0, -6.0, -7.0, -8.0),
                Vec4::<f32>::new(-9.0, -10.0, -11.0, -12.0),
                Vec4::<f32>::new(-13.0, -14.0, -15.0, -16.0),
            )
        );
    }
}
