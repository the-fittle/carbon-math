pub(crate) trait Num<T> {
    fn one() -> Self;
    fn zero() -> Self;
    fn splat(num: T) -> Self;
}

macro_rules! Num {
	($($t:ty),*) => {
		$(
			impl Num<$t> for $t {
				fn one() -> Self {
					1.0 as $t
				}

				fn zero() -> Self {
					0.0 as $t
				}

				fn splat(num: $t) -> Self {
					num
				}
			}
		)*
	};
}

Num!(i8, i16, i32, i64, isize);
Num!(u8, u16, u32, u64, usize);
Num!(f32, f64);
