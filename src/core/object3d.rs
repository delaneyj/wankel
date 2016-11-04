use math::Matrix4;
use core::Geometry;

pub struct Object3D {
    pub matrix_world: Matrix4,
    pub geometry: Option<Geometry>,
}