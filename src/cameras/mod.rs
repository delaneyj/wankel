use math::{Matrix4, Vector3};
use core::{HasObject3D, Object3D};

pub trait Camera: HasObject3D {
    fn projection_matrix(&self) -> Matrix4;
    fn set_projection_matrix(&mut self, val: &Matrix4);

    fn world_direction(&self) -> Vector3 {
        let q = self.scene_object().world_quaternion();
        Vector3::new(0.0, 0.0, -1.0).apply_quaternion(&q)
    }

    // fn look_at(&mut self, vector: &Vector3) {
    fn look_at(&mut self) {
        unimplemented!();
        // should be doing self... learning best way to do traits;
        // let so = self.scene_object();
        // let m = Matrix4::IDENTITY.look_at(so.position, vector, so.up);
        // self.scene_object.quaternion = Quaternion::from_rotation_matrix(&m);
    }
}

pub struct OrthographicCamera {
    scene_object: Object3D,
    projection_matrix: Matrix4,
    pub zoom: f32,
    pub view: Option<OrthographicView>,
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
    pub near: f32,
    pub far: f32,
}

impl Camera for OrthographicCamera {
    fn projection_matrix(&self) -> Matrix4 {
        self.projection_matrix
    }

    fn set_projection_matrix(&mut self, val: &Matrix4) {
        self.projection_matrix = *val;
    }
}

impl HasObject3D for OrthographicCamera {
    fn scene_object(&self) -> &Object3D {
        &self.scene_object
    }
}

pub struct OrthographicView {
    pub full_width: usize,
    pub full_height: usize,
    pub offset_x: f32,
    pub offset_y: f32,
    pub width: usize,
    pub height: usize,
}

impl OrthographicCamera {
    pub fn new(left: f32,
               right: f32,
               top: f32,
               bottom: f32,
               near: Option<f32>,
               far: Option<f32>)
               -> OrthographicCamera {
        let mut ortho = OrthographicCamera {
            scene_object: Object3D::next(),
            projection_matrix: Matrix4::IDENTITY,
            zoom: 1.0,
            view: None,
            left: left,
            right: right,
            top: top,
            bottom: bottom,
            near: near.unwrap_or(0.1),
            far: far.unwrap_or(2000.0),
        };

        ortho.update_projection_matrix();
        ortho
    }

    pub fn set_view_offset(&mut self,
                           full_width: usize,
                           full_height: usize,
                           x: f32,
                           y: f32,
                           width: usize,
                           height: usize) {

        self.view = Some(OrthographicView {
            full_width: full_width,
            full_height: full_height,
            offset_x: x,
            offset_y: y,
            width: width,
            height: height,
        });

        self.update_projection_matrix();
    }

    pub fn clear_view_offset(&mut self) {
        self.view = None;
        self.update_projection_matrix();
    }

    pub fn update_projection_matrix(&mut self) {
        let dx = (self.right - self.left) / (2.0 * self.zoom);
        let dy = (self.top - self.bottom) / (2.0 * self.zoom);
        let cx = (self.right + self.left) / 2.0;
        let cy = (self.top + self.bottom) / 2.0;

        let mut left = cx - dx;
        let mut right = cx + dx;
        let mut top = cy + dy;
        let mut bottom = cy - dy;

        if let Some(ref view) = self.view {
            let zoom_w = self.zoom / (view.width as f32 / view.full_width as f32);
            let zoom_h = self.zoom / (view.height as f32 / view.full_height as f32);
            let scale_w = (self.right - self.left) / view.width as f32;
            let scale_h = (self.top - self.bottom) / view.height as f32;

            left += scale_w * (view.offset_x / zoom_w);
            right = left + scale_w * (view.width as f32 / zoom_w);
            top -= scale_h * (view.offset_y / zoom_h);
            bottom = top - scale_h * (view.height as f32 / zoom_h);
        }

        let m = Matrix4::from_orthographic(left, right, top, bottom, self.near, self.far);
        self.set_projection_matrix(&m);
    }
}