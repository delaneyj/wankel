use math::Sphere;

pub struct Geometry {
    pub bounding_sphere: Option<Sphere>,
}

impl Geometry {
    pub fn compute_bounding_sphere(&self) -> Sphere {
        unimplemented!();
    }
}