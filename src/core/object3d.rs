use math::{Matrix4, Quaternion, Vector3, Euler};
use core::Geometry;

pub struct Object3D<'a> {
    pub parent: Option<&'a Object3D<'a>>,
    pub children: Vec<&'a Object3D<'a>>,
    pub up: Vector3,

    pub position: Vector3,
    pub rotation: Euler,
    pub quaternion: Quaternion,
    pub scale: Vector3,

    pub matrix_world: Matrix4,
    pub geometry: Option<&'a Geometry>,
}

pub trait HasObject3D {
    fn scene_object(&self) -> &Object3D;
}

impl<'a> Object3D<'a> {
    pub fn world_quaternion(&self) -> Quaternion {
        unimplemented!();
    }
}