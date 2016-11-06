use core::{Object3D, HasObject3D};

pub struct Sprite<'a> {
    scene_object: Object3D<'a>,
}

impl<'a> HasObject3D for Sprite<'a> {
    fn scene_object(&self) -> &Object3D {
        &self.scene_object
    }
}