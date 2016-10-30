use std::f32::consts::PI;
use std::f32::EPSILON;
use math::*;

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

    // restrict phi to be betwee EPS and PI-EPS
    pub fn make_safe(&self) -> Spherical {
        Spherical { phi: self.phi.min(PI - EPSILON).max(EPSILON), ..*self }
    }

    pub fn from_vector3(vec3: &Vector3) -> Spherical {
        let radius = vec3.length();

        if radius == 0.0 {
            Spherical::new(radius, 0.0, 0.0)
        } else {
            let phi = clamp(vec3.y / radius, -1.0, 1.0).acos(); // polar angle
            let theta = vec3.x.atan2(vec3.z);// equator angle around y-up axis
            Spherical::new(radius, phi, theta)
        }
    }
}