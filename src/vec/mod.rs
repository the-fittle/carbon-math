mod vec2;
mod vec3;
mod vec4;
pub use vec2::*;
pub use vec3::*;
pub use vec4::*;

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, PartialOrd)]
pub struct Vec2<S>(pub [S; 2]);
unsafe impl<S> bytemuck::Zeroable for Vec2<S> where S: bytemuck::Zeroable {}
unsafe impl<S> bytemuck::Pod for Vec2<S> where S: bytemuck::Pod {}
#[derive(Debug, Default, serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, PartialOrd)]
pub struct Vec3<S>(pub [S; 3]);
unsafe impl<S> bytemuck::Zeroable for Vec3<S> where S: bytemuck::Zeroable {}
unsafe impl<S> bytemuck::Pod for Vec3<S> where S: bytemuck::Pod {}
#[derive(Debug, Default, serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, PartialOrd)]
pub struct Vec4<S>(pub [S; 4]);
unsafe impl<S> bytemuck::Zeroable for Vec4<S> where S: bytemuck::Zeroable {}
unsafe impl<S> bytemuck::Pod for Vec4<S> where S: bytemuck::Pod {}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec2_std_ops() {
        let v = Vec2::<f32>::new(1.0, 2.0);

        assert_eq!(v + v, Vec2::<f32>::new(2.0, 4.0));
        assert_eq!(v - v, Vec2::<f32>::zero());
        assert_eq!(v * 2.0, Vec2::<f32>::new(2.0, 4.0));
        assert_eq!(v / 2.0, Vec2::<f32>::new(0.5, 1.0));
        assert_eq!(-v, Vec2::<f32>::new(-1.0, -2.0));
    }

    #[test]
    fn test_vec3_std_ops() {
        let v = Vec3::<f32>::new(1.0, 2.0, 3.0);

        assert_eq!(v + v, Vec3::<f32>::new(2.0, 4.0, 6.0));
        assert_eq!(v - v, Vec3::<f32>::zero());
        assert_eq!(v * 2.0, Vec3::<f32>::new(2.0, 4.0, 6.0));
        assert_eq!(v / 2.0, Vec3::<f32>::new(0.5, 1.0, 1.5));
        assert_eq!(-v, Vec3::<f32>::new(-1.0, -2.0, -3.0));
    }

    #[test]
    fn test_vec4_std_ops() {
        let v = Vec4::<f32>::new(1.0, 2.0, 3.0, 4.0);

        assert_eq!(v + v, Vec4::<f32>::new(2.0, 4.0, 6.0, 8.0));
        assert_eq!(v - v, Vec4::<f32>::zero());
        assert_eq!(v * 2.0, Vec4::<f32>::new(2.0, 4.0, 6.0, 8.0));
        assert_eq!(v / 2.0, Vec4::<f32>::new(0.5, 1.0, 1.5, 2.0));
        assert_eq!(-v, Vec4::<f32>::new(-1.0, -2.0, -3.0, -4.0));
    }
}
