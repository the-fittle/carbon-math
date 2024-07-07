mod bivec2;
mod bivec3;
pub use bivec2::*;
pub use bivec3::*;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Bivec2<S>(pub S);
unsafe impl<S> bytemuck::Zeroable for Bivec2<S> where S: bytemuck::Zeroable {}
unsafe impl<S> bytemuck::Pod for Bivec2<S> where S: bytemuck::Pod {}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Bivec3<S>(pub [S; 3]);
unsafe impl<S> bytemuck::Zeroable for Bivec3<S> where S: bytemuck::Zeroable {}
unsafe impl<S> bytemuck::Pod for Bivec3<S> where S: bytemuck::Pod {}

// test
// -----------------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bivec2_std_ops() {
        let bi = Bivec2::<f32>::new(3.0);

        assert_eq!(bi + bi, Bivec2::<f32>::new(6.0));
        assert_eq!(bi - bi, Bivec2::<f32>::zero());
        assert_eq!(bi * 2.0, Bivec2::<f32>::new(6.0));
        assert_eq!(bi / 2.0, Bivec2::<f32>::new(1.5));
        assert_eq!(-bi, Bivec2::<f32>::new(-3.0));
    }

    #[test]
    fn test_bivec3_std_ops() {
        let bi = Bivec3::<f32>::new(1.0, 2.0, 3.0);

        assert_eq!(bi + bi, Bivec3::<f32>::new(2.0, 4.0, 6.0));
        assert_eq!(bi - bi, Bivec3::<f32>::zero());
        assert_eq!(bi * 2.0, Bivec3::<f32>::new(2.0, 4.0, 6.0));
        assert_eq!(bi / 2.0, Bivec3::<f32>::new(0.5, 1.0, 1.5));
        assert_eq!(-bi, Bivec3::<f32>::new(-1.0, -2.0, -3.0));
    }
}
