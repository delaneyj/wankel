use math::{Matrix4, Vector3, Quaternion};
use core::{Object3D, HasObject3D};

pub struct Camera<'a> {
    pub projection_matrix: Matrix4,
    scene_object: Object3D<'a>,
}

impl<'a> HasObject3D for Camera<'a> {
    fn scene_object(&self) -> &Object3D {
        &self.scene_object
    }
}

impl<'a> Camera<'a> {
    pub fn world_direction(&self) -> Vector3 {
        let q = self.scene_object.world_quaternion();
        Vector3::new(0.0, 0.0, -1.0).apply_quaternion(&q)
    }

    pub fn look_at(&mut self, vector: &Vector3) {
        let m =
            Matrix4::IDENTITY.look_at(&self.scene_object.position, vector, &self.scene_object.up);
        self.scene_object.quaternion = Quaternion::from_rotation_matrix(&m);
    }
}