use core::{Object3D, SceneObject};

pub struct Sprite {
    scene_object: Object3D,
}

impl SceneObject for Sprite {
    fn scene_object(&self) -> &Object3D {
        &self.scene_object
    }
}