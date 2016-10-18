// Ref: https://en.wikipedia.org/wiki/Spherical_coordinate_system
// The poles (phi) are at the positive and negative y axis.
// The equator starts at positive z.

#[derive(Debug,PartialEq)]
pub struct Spherical {
    pub radius: f32,
    pub phi: f32,
    pub theta: f32,
}

impl Spherical {
    pub const DEFAULT: Spherical = Spherical {
        radius: 1.0,
        phi: 0.0, // up / down towards top and bottom pole
        theta: 0.0, // around the equator of the sphere
    };

    pub fn new(radius: f32, phi: f32, theta: f32) -> Spherical {
        Spherical {
            radius: radius,
            phi: phi,
            theta: theta,
        }
    }
}