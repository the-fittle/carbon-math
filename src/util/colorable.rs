pub trait Colorable {
    fn as_rgb(&self) -> [f32; 3];
    fn as_rgba(&self) -> [f32; 4];
}

macro_rules! Colorable {
	($($t:ty),*) => {
		$(
			impl Colorable for $t {
				fn as_rgb(&self) -> [f32; 3] {
					let r = ((*self >> 16) & 0xff) as f32 / 255.0;
					let g = ((*self >> 8) & 0xff) as f32 / 255.0;
					let b = (*self & 0xff) as f32 / 255.0;
					[r.abs(), g.abs(), b.abs()]
				}

				fn as_rgba(&self) -> [f32; 4] {
					let a = ((*self >> 24) & 0xff) as f32 / 255.0;
					let r = ((*self >> 16) & 0xff) as f32 / 255.0;
					let g = ((*self >> 8) & 0xff) as f32 / 255.0;
					let b = (*self & 0xff) as f32 / 255.0;
					[r.abs(), g.abs(), b.abs(), a.abs()]
				}
			}
		)*
	};
}

Colorable!(u32, u64, usize);
