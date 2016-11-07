use math::Sphere;

#[derive(Debug,PartialEq)]
pub struct Geometry {
    pub bounding_sphere: Option<Sphere>,
}

impl Geometry {
    pub fn compute_bounding_sphere(&self) -> Sphere {
        unimplemented!();
    }
}