mod quat;
pub use quat::*;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Quat<S>(pub [S; 4]);
unsafe impl<S> bytemuck::Zeroable for Quat<S> where S: bytemuck::Zeroable {}
unsafe impl<S> bytemuck::Pod for Quat<S> where S: bytemuck::Pod {}

// test
// -----------------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_quat_std_ops() {
        let q = Quat::<f32>::new(1.0, 2.0, 3.0, 4.0);

        assert_eq!(q + q, Quat::<f32>::new(2.0, 4.0, 6.0, 8.0));
        assert_eq!(q - q, Quat::<f32>::zero());
        assert_eq!(q * 2.0, Quat::<f32>::new(2.0, 4.0, 6.0, 8.0));
        assert_eq!(q / 2.0, Quat::<f32>::new(0.5, 1.0, 1.5, 2.0));
        assert_eq!(-q, Quat::<f32>::new(-1.0, -2.0, -3.0, -4.0));
    }
}
