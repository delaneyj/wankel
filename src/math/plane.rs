use math::Vector3;

#[derive(Debug,PartialEq)]
pub struct Plane {
    pub normal: Vector3,
    pub constant: f32,
}

impl Plane {
    pub const DEFAULT: Plane = Plane {
        normal: Vector3 {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        },
        constant: 0.0,
    };
}
