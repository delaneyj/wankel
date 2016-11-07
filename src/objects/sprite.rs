use core::{Object3D, HasObject3D};

pub struct Sprite {
    scene_object: Object3D,
}

impl HasObject3D for Sprite {
    fn scene_object(&self) -> &Object3D {
        &self.scene_object
    }
}