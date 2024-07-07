mod rot2;
mod rot3;
pub use rot2::*;
pub use rot3::*;

use crate::{Bivec2, Bivec3};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Rot2<S>(pub S, pub Bivec2<S>);
unsafe impl<S> bytemuck::Zeroable for Rot2<S> where S: bytemuck::Zeroable {}
unsafe impl<S> bytemuck::Pod for Rot2<S> where S: bytemuck::Pod {}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Rot3<S>(pub S, pub Bivec3<S>);
unsafe impl<S> bytemuck::Zeroable for Rot3<S> where S: bytemuck::Zeroable {}
unsafe impl<S> bytemuck::Pod for Rot3<S> where S: bytemuck::Pod {}

// test
// -----------------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_rot2_std_ops() {
        let r = Rot2::<f32>::new(1.0, Bivec2::<f32>::new(3.0));

        assert_eq!(r + r, Rot2::<f32>::new(2.0, Bivec2::<f32>::new(6.0)));
        assert_eq!(r - r, Rot2::<f32>::zero());
        assert_eq!(r * 2.0, Rot2::<f32>::new(2.0, Bivec2::<f32>::new(6.0)));
        assert_eq!(r / 2.0, Rot2::<f32>::new(0.5, Bivec2::<f32>::new(1.5)));
        assert_eq!(-r, Rot2::<f32>::new(-1.0, Bivec2::<f32>::new(-3.0)));
    }

    #[test]
    fn test_rot3_std_ops() {
        let r = Rot3::<f32>::new(1.0, Bivec3::<f32>::new(1.0, 2.0, 3.0));

        assert_eq!(r + r, Rot3::<f32>::new(2.0, Bivec3::<f32>::new(2.0, 4.0, 6.0)));
        assert_eq!(r - r, Rot3::<f32>::zero());
        assert_eq!(r * 2.0, Rot3::<f32>::new(2.0, Bivec3::<f32>::new(2.0, 4.0, 6.0)));
        assert_eq!(r / 2.0, Rot3::<f32>::new(0.5, Bivec3::<f32>::new(0.5, 1.0, 1.5)));
        assert_eq!(-r, Rot3::<f32>::new(-1.0, Bivec3::<f32>::new(-1.0, -2.0, -3.0)));
    }
}
