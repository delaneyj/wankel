use math::Matrix4;

use core::Object3D;

pub struct Camera {
    pub matrix_world_inverse: Matrix4,
    pub projection_matrix: Matrix4,
    pub matrix_world: Matrix4,
    pub transform: Object3D,
}