pub const PI: f32 = 3.141592653589793238462643383279502884197169399375105820974944592307816406286208998628034825342117067982148086513282306647093844609550582231725359408128481117450284102701938521105559644622948954930382;
pub const PI_OVER_2: f32 = PI / 2f32;
pub const PI_OVER_3: f32 = PI / 3f32;
pub const PI_OVER_4: f32 = PI / 4f32;
pub const PI_OVER_6: f32 = PI / 6f32;
pub const CSHARP_PI: f32 = 3.14159274;
pub const CSHARP_PI_F64: f64 = 3.1415926535897931;
pub const TWO_PI: f32 = 2f32 * PI;
pub const THREE_PI_OVER_2: f32 = 3f32 * PI / 2f32;
pub const E: f32 = 2.71828182845904523536f32;
pub const LOG_10_E: f32 = 0.434294482f32;
pub const LOG_2_E: f32 = 1.442695041f32;

pub fn next_power_of_two<T>(n: T) -> T
where
    T: Copy + PartialOrd + num_traits::Zero + num_traits::ToPrimitive + num_traits::FromPrimitive,
{
    if n <= T::zero() {
        panic!("n must be positive!");
    }
    T::from_f64(2f64.powf(n.to_f64().expect("Conversion to f64 failed").log2().ceil()))
        .expect("Conversion from f64 failed")
}

pub fn degrees_to_radians_f32(degrees: f32) -> f32 {
    degrees * (PI / 180f32)
}
pub fn radians_to_degrees_f32(radians: f32) -> f32 {
    radians * (180f32 / PI)
}

pub fn degrees_to_radians_f64(degrees: f64) -> f64 {
    degrees * (PI as f64 / 180f64)
}
pub fn radians_to_degrees_f64(radians: f64) -> f64 {
    radians * (180f64 / PI as f64)
}
