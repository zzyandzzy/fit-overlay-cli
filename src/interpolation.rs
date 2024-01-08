pub trait MulF32<RHS = Self> {
    type Output;

    fn mul_f32(self, rhs: f64) -> Self::Output;
}

impl MulF32 for u32 {
    type Output = f64;

    fn mul_f32(self, rhs: f64) -> Self::Output {
        self as f64 * rhs
    }
}

impl MulF32 for u16 {
    type Output = f64;

    fn mul_f32(self, rhs: f64) -> Self::Output {
        self as f64 * rhs
    }
}

impl MulF32 for u8 {
    type Output = f64;

    fn mul_f32(self, rhs: f64) -> Self::Output {
        self as f64 * rhs
    }
}

impl MulF32 for f32 {
    type Output = f64;

    fn mul_f32(self, rhs: f64) -> Self::Output {
        self as f64 * rhs
    }
}

impl MulF32 for f64 {
    type Output = f64;

    fn mul_f32(self, rhs: f64) -> Self::Output {
        self * rhs
    }
}

pub fn lerp<T>(start: T, end: T, t: f64) -> f64
where
    T: MulF32<Output = f64> + Copy,
{
    start.mul_f32(1.0 - t) + end.mul_f32(t)
}
