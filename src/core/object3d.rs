use std::rc::Rc;
use math::{Matrix4, Quaternion, Vector3, Euler};
use core::Geometry;

#[derive(Debug,PartialEq)]
pub struct Object3D {
    pub parent: Option<Box<Object3D>>,
    pub children: Vec<Box<Object3D>>,
    pub up: Vector3,

    pub position: Vector3,
    pub rotation: Euler,
    pub quaternion: Quaternion,
    pub scale: Vector3,

    pub matrix: Matrix4,
    pub matrix_world: Matrix4,
    pub geometry: Option<Geometry>,

    matrix_world_needs_update: bool,
    matrix_auto_update: bool,
    visible: bool,
    casts_shadows: bool,
    receives_shadows: bool,
    frustum_culled: bool,
    render_order: usize,
}

pub trait HasObject3D {
    fn scene_object(&self) -> &Object3D;
}

impl Object3D {
    pub fn next() -> Object3D {
        Object3D {
            parent: None,
            children: vec![],
            up: Vector3::new(0.0, 1.0, 0.0),
            position: Vector3::ZERO,
            rotation: Euler::DEFAULT,
            quaternion: Quaternion::DEFAULT,
            scale: Vector3::ONE,
            matrix: Matrix4::IDENTITY,
            matrix_world: Matrix4::IDENTITY,
            geometry: None,

            matrix_world_needs_update: false,
            matrix_auto_update: true,
            visible: true,
            casts_shadows: false,
            receives_shadows: false,
            frustum_culled: true,
            render_order: 0,
        }
    }

    pub fn apply_matrix(&mut self, matrix: &Matrix4) {
        self.matrix = matrix.multiply(&self.matrix);
        let (position, quaternion, scale) = self.matrix.decompose();
        self.position = position;
        self.quaternion = quaternion;
        self.scale = scale;
    }

    pub fn set_rotation_from_axis_angle(&mut self, axis: &Vector3, angle: f32) {
        // assumes axis is normalized
        self.quaternion = Quaternion::from_axis_angle(axis, angle);
    }

    pub fn set_rotation_from_euler(&mut self, euler: &Euler) {
        self.quaternion = Quaternion::from_euler(euler);
    }

    pub fn set_rotation_from_matrix(&mut self, m: &Matrix4) {
        // assumes the upper 3x3 of m is a pure rotation matrix (i.e, unscaled)
        self.quaternion = Quaternion::from_rotation_matrix(&m);
    }

    pub fn set_rotation_from_quaternion(&mut self, q: &Quaternion) {
        // assumes q is normalized
        self.quaternion = *q;
    }

    pub fn rotate_on_axis(&mut self, axis: &Vector3, angle: f32) {
        // rotate object on axis in object space
        // axis is assumed to be normalized
        let q1 = Quaternion::from_axis_angle(axis, angle);
        self.quaternion = self.quaternion.multiply(&q1);
    }

    pub fn rotate_x(&mut self, angle: f32) {
        self.rotate_on_axis(&Vector3::X, angle)
    }

    pub fn rotate_y(&mut self, angle: f32) {
        self.rotate_on_axis(&Vector3::Y, angle)
    }

    pub fn rotate_z(&mut self, angle: f32) {
        self.rotate_on_axis(&Vector3::Z, angle)
    }

    pub fn translate_on_axis(&mut self, axis: &Vector3, distance: f32) {

        // translate object by distance along axis in object space
        // axis is assumed to be normalized
        let v1 = axis.apply_quaternion(&self.quaternion);
        self.position = self.position.add(&v1.multiply_scalar(distance));
    }

    pub fn translate_x(&mut self, distance: f32) {
        self.translate_on_axis(&Vector3::X, distance);
    }

    pub fn translate_y(&mut self, distance: f32) {
        self.translate_on_axis(&Vector3::Y, distance);
    }

    pub fn translate_z(&mut self, distance: f32) {
        self.translate_on_axis(&Vector3::Z, distance);
    }

    pub fn local_to_world(&self, vector: &Vector3) -> Vector3 {
        vector.apply_matrix4(&self.matrix_world)
    }

    pub fn world_to_local(&self, vector: &Vector3) -> Vector3 {
        vector.apply_matrix4(&self.matrix_world.inverse());
    }

    pub fn look_at(&mut self, vector: &Vector3) {
        // This routine does not support objects with rotated and/or translated parent(s)
        let m1 = Matrix4::IDENTITY.look_at(vector, &self.position, &self.up);
        self.quaternion = Quaternion::from_rotation_matrix(&m1);
    }

    pub fn add(&mut self, object: &mut Object3D) {

        if object == self {
            panic!("object '{:?}' can't be added as a child of itself.", self);
        }

        match object.parent {
            Some(parent) => parent.remove(object),
            None => {}
        }

        object.parent = Some(Box::new(*self));
        self.children.push(Box::new(*object));
    }

    pub fn remove(&mut self, object: &mut Object3D) {
        self.children = self.children.into_iter().filter(|o| Box::into_raw(*o) == object).collect();
    }

    pub fn world_position(&mut self) -> Vector3 {
        self.update_matrix_world(true);
        Vector3::from_matrix_position(&self.matrix_world)
    }

    pub fn world_quaternion(&mut self) -> Quaternion {
        unimplemented!();

        self.update_matrix_world(true);
        let (_, result, _) = self.matrix_world.decompose();
        result
    }

    pub fn world_rotation(&self) -> Euler {
        let q = self.world_quaternion();
        Euler::from_quaternion(&q, &self.rotation.order)
    }

    pub fn world_scale(&mut self) -> Vector3 {
        self.update_matrix_world(true);
        let (position, quaternion, scale) = self.matrix_world.decompose();
        self.position = position;
        self.quaternion = quaternion;
        self.scale = scale;
        scale
    }

    pub fn world_direction(&mut self) -> Vector3 {
        let q = self.world_quaternion();
        Vector3::Z.apply_quaternion(&q)
    }

    pub fn traverse(&self, callback: &Fn(&Object3D)) {
        callback(self);

        for child in self.children {
            child.traverse(callback);
        }
    }

    pub fn traverse_visible(&self, callback: &Fn(&Object3D)) {
        if self.visible {
            callback(self);

            for child in self.children {
                child.traverse_visible(callback);
            }
        }
    }

    pub fn traverse_ancestors(&self, callback: &Fn(&Object3D)) {
        match self.parent {
            Some(parent_rc) => {
                let parent = Box::into_raw(parent_rc);
                callback(&*parent);
                parent.traverse_ancestors(callback);
            }
            None => {}
        }
    }

    pub fn update_matrix(&mut self) {
        self.matrix = Matrix4::compose(&self.position, &self.quaternion, &self.scale);
        self.matrix_world_needs_update = true;
    }

    pub fn update_matrix_world(&mut self, force: bool) {
        let mut modifiable_forced = force;
        if self.matrix_auto_update {
            self.update_matrix();
        }

        if self.matrix_world_needs_update || force {
            self.matrix_world = match self.parent {
                None => self.matrix,
                Some(parent) => parent.matrix_world.multiply(&self.matrix),
            };

            self.matrix_world_needs_update = false;
            modifiable_forced = true;
        }

        // update children
        for child in self.children {
            child.update_matrix_world(modifiable_forced)
        }
    }
}