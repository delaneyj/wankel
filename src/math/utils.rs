pub fn clamp(value: f32, min: f32, max: f32) -> f32 {
    value.max(min).min(max)
}

// compute euclidian modulo of m % n
// https://en.wikipedia.org/wiki/Modulo_operation
pub fn euclidean_modulo(n: f32, m: f32) -> f32 {
    ((n % m) + m) % m
}

// Linear mapping from range <a1, a2> to range <b1, b2>
pub fn map_linear(x: f32, a1: f32, a2: f32, b1: f32, b2: f32) -> f32 {
    b1 + (x - a1) * (b2 - b1) / (a2 - a1)
}

// https://en.wikipedia.org/wiki/Linear_interpolation
pub fn lerp(x: f32, y: f32, t: f32) -> f32 {
    (1.0 - t) * x + t * y
}

// http://en.wikipedia.org/wiki/Smoothstep
pub fn smooth_step(x: f32, min: f32, max: f32) -> f32 {
    if x <= min {
        0.0
    } else if x >= max {
        1.0
    } else {
        let y = (x - min) / (max - min);
        y * y * (3.0 - 2.0 * y)
    }
}

pub fn smoother_step(x: f32, min: f32, max: f32) -> f32 {
    if x <= min {
        0.0
    } else if x >= max {
        1.0
    } else {
        let y = (x - min) / (max - min);
        y * y * y * (y * (y * 6.0 - 15.0) + 10.0)
    }
}