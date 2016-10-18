use std::f32::consts::PI;

#[derive(Debug,PartialEq)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Vector2 {
    const ZERO: Vector2 = Vector2 { x: 0.0, y: 0.0 };
    const ONE: Vector2 = Vector2 { x: 1.0, y: 1.0 };

    pub fn new(x: f32, y: f32) -> Vector2 {
        Vector2 { x: x, y: y }
    }

    pub fn width(&self) -> &f32 {
        &self.x
    }

    pub fn height(&self) -> &f32 {
        &self.y
    }

    pub fn component(&self, index: i32) -> &f32 {
        match index {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("Vector2 only has 2 components, bad index"),
        }
    }

    pub fn copy(&self) -> Vector2 {
        Vector2 {
            x: self.x,
            y: self.y,
        }
    }

    pub fn add(&self, v: &Vector2) -> Vector2 {
        Vector2 {
            x: self.x + v.x,
            y: self.y + v.y,
        }
    }

    pub fn subtract(&self, v: &Vector2) -> Vector2 {
        Vector2 {
            x: self.x - v.x,
            y: self.y - v.y,
        }
    }

    pub fn multiply_scalar(&self, scaler: f32) -> Vector2 {
        Vector2 {
            x: self.x * scaler,
            y: self.y * scaler,
        }
    }

    pub fn divide_scalar(&self, scaler: f32) -> Vector2 {
        Vector2 {
            x: self.x / scaler,
            y: self.y / scaler,
        }
    }

    pub fn min(&self, v: &Vector2) -> Vector2 {
        Vector2 {
            x: self.x.min(v.x),
            y: self.y.min(v.y),
        }
    }

    pub fn max(&self, v: &Vector2) -> Vector2 {
        Vector2 {
            x: self.x.max(v.x),
            y: self.y.max(v.y),
        }
    }

    pub fn clamp(&self, min: &Vector2, max: &Vector2) -> Vector2 {
        Vector2 {
            x: min.x.max(max.x.min(self.x)),
            y: min.y.max(max.y.min(self.y)),
        }
    }

    pub fn clamp_scalar(&self, min: f32, max: f32) -> Vector2 {
        let min_v = Vector2::new(min, min);
        let max_v = Vector2::new(max, max);
        self.clamp(&min_v, &max_v)
    }

    pub fn floor(&self) -> Vector2 {
        Vector2 {
            x: self.x.floor(),
            y: self.y.floor(),
        }
    }

    pub fn ceil(&self) -> Vector2 {
        Vector2 {
            x: self.x.ceil(),
            y: self.y.ceil(),
        }
    }

    pub fn round(&self) -> Vector2 {
        Vector2 {
            x: self.x.round(),
            y: self.y.round(),
        }
    }

    pub fn round_to_zero(&self) -> Vector2 {
        Vector2 {
            x: if self.x < 0.0 {
                self.x.ceil()
            } else {
                self.x.floor()
            },
            y: if self.y < 0.0 {
                self.y.ceil()
            } else {
                self.y.floor()
            },
        }
    }

    pub fn negate(&self) -> Vector2 {
        Vector2 {
            x: -self.x,
            y: -self.y,
        }
    }

    pub fn dot(&self, v: &Vector2) -> f32 {
        self.x * v.x + self.y * v.y
    }

    pub fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y
    }

    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn length_manhattan(&self) -> f32 {
        self.x.abs() + self.y.abs()
    }

    pub fn normalize(&self) -> Vector2 {
        let l = self.length();
        self.divide_scalar(l)
    }

    pub fn angle(&self) -> f32 {
        // computes the angle in radians with respect to the positive x-axis
        let angle = self.y.atan2(self.x);

        if angle < 0.0 {
            angle + (2.0 * PI)
        } else {
            angle
        }
    }

    pub fn distance_to_squared(&self, v: &Vector2) -> f32 {
        let dx = self.x - v.x;
        let dy = self.y - v.y;
        dx * dx + dy * dy
    }

    pub fn distance_to(&self, v: &Vector2) -> f32 {
        self.distance_to_squared(v).sqrt()
    }

    pub fn distance_to_manhattan(&self, v: &Vector2) -> f32 {
        (self.x - v.x).abs() + (self.y - v.y).abs()
    }

    pub fn with_length(&self, length: f32) -> Vector2 {
        let my_length = self.length();
        match my_length {
            0.0 => self.multiply_scalar(0.0),
            _ => self.multiply_scalar(length / my_length),
        }
    }

    pub fn lerp(&self, v: &Vector2, alpha: f32) -> Vector2 {
        Vector2 {
            x: self.x + ((v.x - self.x) * alpha),
            y: self.y + ((v.y - self.y) * alpha),
        }
    }

    pub fn lerp_vectors(v1: &Vector2, v2: &Vector2, alpha: f32) -> Vector2 {
        v2.subtract(v1).multiply_scalar(alpha).add(v1)
    }

    pub fn from_vec(array: &[f32], offset: usize) -> Vector2 {
        Vector2 {
            x: array[offset],
            y: array[offset + 1],
        }
    }

    pub fn to_vec(&self) -> Vec<f32> {
        let mut vec: Vec<f32> = Vec::new();
        vec.push(self.x);
        vec.push(self.y);
        vec
    }

    pub fn rotate_around(&self, center: &Vector2, angle: f32) -> Vector2 {
        let c = angle.cos();
        let s = angle.sin();
        let x = self.x - center.x;
        let y = self.y - center.y;

        Vector2 {
            x: x * c - y * s + center.x,
            y: x * s + y * c + center.y,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::Vector2;

    const X: f32 = 2.0;
    const Y: f32 = 3.0;
    const Z: f32 = 4.0;
    const W: f32 = 5.0;

    const XY: Vector2 = Vector2 { x: X, y: Y };
    const NEG_XY: Vector2 = Vector2 { x: -X, y: -Y };
    #[test]
    fn constructor() {
        let a = Vector2::ZERO;
        assert!(a.x == 0.0);
        assert!(a.y == 0.0);

        let b = XY.copy();
        assert!(b.x == X);
        assert!(b.y == Y);
    }

    #[test]
    fn copy() {
        let mut a = XY.copy();
        let b = a.copy();
        assert!(b.x == X);
        assert!(b.y == Y);

        // ensure that it is a true copy
        a.x = 0.0;
        a.y = -1.0;
        assert!(b.x == X);
        assert!(b.y == Y);
    }

    #[test]
    fn get_component() {
        let a = Vector2::ZERO;
        assert!(a.x == 0.0);
        assert!(a.y == 0.0);

        let b = Vector2::new(1.0, 2.0);
        assert!(*b.component(0) == 1.0);
        assert!(*b.component(1) == 2.0);
    }

    #[test]
    fn add() {
        let a = XY.add(&NEG_XY);

        assert!(a.x == 0.0);
        assert!(a.y == 0.0);

        let b = NEG_XY.add(&NEG_XY);
        assert!(b.x == -2.0 * X);
        assert!(b.y == -2.0 * Y);
    }

    #[test]
    fn sub() {
        let a = XY.subtract(&NEG_XY);
        assert!(a.x == 2.0 * X);
        assert!(a.y == 2.0 * Y);

        let b = XY.subtract(&XY);
        assert_eq!(b.x, 0.0);
        assert_eq!(b.y, 0.0);
    }

    #[test]
    fn multiply_divide() {
        let a = XY.multiply_scalar(-2.0);
        assert_eq!(a.x, X * -2.0);
        assert_eq!(a.y, Y * -2.0);

        let b = NEG_XY.multiply_scalar(-2.0);
        assert_eq!(b.x, 2.0 * X);
        assert_eq!(b.y, 2.0 * Y);

        let c = XY.divide_scalar(-2.0);
        assert_eq!(c.x, -1.0);
        assert_eq!(c.y, -1.5);

        let d = NEG_XY.divide_scalar(-2.0);
        assert_eq!(d.x, 1.0);
        assert_eq!(d.y, 1.5);
    }

    #[test]
    fn min_max_clamp() {
        let a = XY.min(&NEG_XY);
        assert_eq!(a.x, -X);
        assert_eq!(a.y, -Y);

        let b = XY.max(&NEG_XY);
        assert_eq!(b.x, X);
        assert_eq!(b.y, Y);

        let c = Vector2::new(-2.0 * X, 2.0 * Y).clamp(&NEG_XY, &XY);
        assert_eq!(c.x, -X);
        assert_eq!(c.y, Y);

        let d = Vector2::new(-2.0 * X, 2.0 * X).clamp_scalar(-X, X);
        assert_eq!(d.x, -X);
        assert_eq!(d.y, X);
    }

    #[test]
    fn rounding() {
        assert_eq!(Vector2::new(-0.1, 0.1).floor(), Vector2::new(-1.0, 0.0));
        assert_eq!(Vector2::new(-0.5, 0.5).floor(), Vector2::new(-1.0, 0.0));
        assert_eq!(Vector2::new(-0.9, 0.9).floor(), Vector2::new(-1.0, 0.0));

        assert_eq!(Vector2::new(-0.1, 0.1).ceil(), Vector2::new(0.0, 1.0));
        assert_eq!(Vector2::new(-0.5, 0.5).ceil(), Vector2::new(0.0, 1.0));
        assert_eq!(Vector2::new(-0.9, 0.9).ceil(), Vector2::new(0.0, 1.0));

        assert_eq!(Vector2::new(-0.1, 0.1).round(), Vector2::new(0.0, 0.0));
        assert_eq!(Vector2::new(-0.5, 0.5).round(), Vector2::new(-1.0, 1.0));
        assert_eq!(Vector2::new(-0.9, 0.9).round(), Vector2::new(-1.0, 1.0));

        assert_eq!(Vector2::new(-0.1, 0.1).round_to_zero(),
                   Vector2::new(0.0, 0.0));
        assert_eq!(Vector2::new(-0.5, 0.5).round_to_zero(),
                   Vector2::new(0.0, 0.0));
        assert_eq!(Vector2::new(-0.9, 0.9).round_to_zero(),
                   Vector2::new(0.0, 0.0));
        assert_eq!(Vector2::new(-1.1, 1.1).round_to_zero(),
                   Vector2::new(-1.0, 1.0));
        assert_eq!(Vector2::new(-1.5, 1.5).round_to_zero(),
                   Vector2::new(-1.0, 1.0));
        assert_eq!(Vector2::new(-1.9, 1.9).round_to_zero(),
                   Vector2::new(-1.0, 1.0));
    }

    #[test]
    fn negate() {
        let a = XY.negate();
        assert_eq!(a.x, -X);
        assert_eq!(a.y, -Y);
    }

    #[test]
    fn dot() {
        assert_eq!(XY.dot(&NEG_XY), -X * X - Y * Y);
        assert_eq!(XY.dot(&Vector2::ZERO), 0.0);
    }

    #[test]
    fn length_and_length_squared() {
        let a = Vector2::new(X, 0.0);
        let b = Vector2::new(0.0, -Y);
        let c = Vector2::ZERO;

        assert_eq!(a.length(), X);
        assert_eq!(a.length_squared(), X * X);
        assert_eq!(b.length(), Y);
        assert_eq!(b.length_squared(), Y * Y);
        assert_eq!(c.length(), 0.0);
        assert_eq!(c.length_squared(), 0.0);

        assert_eq!(XY.length(), (X * X + Y * Y).sqrt());
        assert_eq!(XY.length_squared(), (X * X + Y * Y));
    }

    #[test]
    fn normalize() {
        let a = Vector2::new(X, 0.0).normalize();
        let b = Vector2::new(0.0, -Y).normalize();
        assert_eq!(a.length(), 1.0);
        assert_eq!(a.x, 1.0);
        assert_eq!(b.length(), 1.0);
        assert_eq!(b.y, -1.0);
    }

    #[test]
    fn distance_to_and_distance_to_squared() {
        let a = Vector2::new(X, 0.0);
        let b = Vector2::new(0.0, -Y);
        assert_eq!(a.distance_to(&Vector2::ZERO), X);
        assert_eq!(a.distance_to_squared(&Vector2::ZERO), X * X);
        assert_eq!(b.distance_to(&Vector2::ZERO), Y);
        assert_eq!(b.distance_to_squared(&Vector2::ZERO), Y * Y);
    }

    #[test]
    fn with_length() {
        let a = Vector2::new(X, 0.0);
        assert_eq!(a.length(), X);
        assert_eq!(a.with_length(Y).length(), Y);
        assert_eq!(Vector2::ZERO.length(), 0.0);
        assert_eq!(Vector2::ZERO.with_length(Y).length(), 0.0);
    }

    #[test]
    fn lerp_and_clone() {
        let a = Vector2::new(X, 0.0);
        let b = Vector2::new(0.0, -Y);

        assert_eq!(a.lerp(&a, 0.0), a.lerp(&a, 0.5));
        assert_eq!(a.lerp(&a, 0.0), a.lerp(&a, 1.0));

        assert_eq!(a.lerp(&b, 0.0), a);

        assert_eq!(a.lerp(&b, 0.5).x, X * 0.5);
        assert_eq!(a.lerp(&b, 0.5).y, -Y * 0.5);

        assert_eq!(a.lerp(&b, 1.0), b);
    }

    #[test]
    fn equals() {
        let a = Vector2::new(X, 0.0);
        let b = Vector2::new(0.0, -Y);

        assert!(a.x != b.x);
        assert!(a.y != b.y);

        assert!(a != b);
        assert!(b != a);

        let c = a.copy();
        assert_eq!(a.x, c.x);
        assert_eq!(a.y, c.y);

        assert_eq!(a, c);
        assert_eq!(c, a);
    }
}
