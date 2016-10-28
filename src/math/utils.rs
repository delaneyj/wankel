pub fn clamp(value: f32, min: f32, max: f32) -> f32 {
    value.max(min).min(max)
}

// compute euclidian modulo of m % n
// https://en.wikipedia.org/wiki/Modulo_operation
pub fn euclidean_modulo(n: f32, m: f32) -> f32 {
    ((n % m) + m) % m
}