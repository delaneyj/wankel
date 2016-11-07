use std::f32::consts::FRAC_1_SQRT_2;

use core::HasObject3D;
use objects::Sprite;
use math::{Vector3, Plane, Sphere, Box3, Matrix4};

#[derive(Debug,PartialEq)]
pub struct Frustum {
    pub planes: [Plane; 6],
}

impl Frustum {
    const DEFAULT: Frustum = Frustum {
        planes: [Plane::DEFAULT,
                 Plane::DEFAULT,
                 Plane::DEFAULT,
                 Plane::DEFAULT,
                 Plane::DEFAULT,
                 Plane::DEFAULT],
    };

    pub fn new(p0: &Plane, p1: &Plane, p2: &Plane, p3: &Plane, p4: &Plane, p5: &Plane) -> Frustum {
        Frustum { planes: [*p0, *p1, *p2, *p3, *p4, *p5] }
    }

    pub fn from_matrix4(m: &Matrix4) -> Frustum {
        let [me0,me1,me2,me3,me4,me5,me6,me7,me8,me9,me10,me11,me12,me13,me14,me15]:[f32;16] = m.elements;
        Frustum {
            planes: [Plane::new(&Vector3::new(me3 - me0, me7 - me4, me11 - me8), me15 - me12)
                         .normalized(),
                     Plane::new(&Vector3::new(me3 + me0, me7 + me4, me11 + me8), me15 + me12)
                         .normalized(),
                     Plane::new(&Vector3::new(me3 + me1, me7 + me5, me11 + me9), me15 + me13)
                         .normalized(),
                     Plane::new(&Vector3::new(me3 - me1, me7 - me5, me11 - me9), me15 - me13)
                         .normalized(),
                     Plane::new(&Vector3::new(me3 - me2, me7 - me6, me11 - me10),
                                me15 - me14)
                         .normalized(),
                     Plane::new(&Vector3::new(me3 + me2, me7 + me6, me11 + me10),
                                me15 + me14)
                         .normalized()],
        }
    }

    pub fn intersects_object<T: HasObject3D>(&self, object: &T) -> bool {
        let scene_object = object.scene_object();
        match scene_object.geometry {
            None => false,
            Some(ref geometry) => {
                let bounding_sphere = match geometry.bounding_sphere {
                    None => geometry.compute_bounding_sphere(),
                    Some(ref bs) => bs.apply_matrix4(&scene_object.matrix_world), 
                };

                self.intersects_sphere(&bounding_sphere)
            }
        }
    }

    pub fn intersects_sprite(&self, sprite: &Sprite) -> bool {
        let sphere = Sphere::new(&Vector3::ZERO, FRAC_1_SQRT_2)
            .apply_matrix4(&sprite.scene_object().matrix_world);
        self.intersects_sphere(&sphere)
    }

    pub fn intersects_sphere(&self, sphere: &Sphere) -> bool {
        !self.planes
            .into_iter()
            .map(|p| p.distance_to_point(&sphere.center))
            .any(|d| d < -sphere.radius)
    }

    pub fn intersects_box(&self, box3: &Box3) -> bool {
        !self.planes.into_iter().any(|plane| {
            let x1 = if plane.normal.x > 0.0 {
                box3.min.x
            } else {
                box3.max.x
            };

            let y1 = if plane.normal.y > 0.0 {
                box3.min.y
            } else {
                box3.max.y
            };

            let z1 = if plane.normal.z > 0.0 {
                box3.min.z
            } else {
                box3.max.z
            };

            let x2 = if plane.normal.x > 0.0 {
                box3.max.x
            } else {
                box3.min.x
            };

            let y2 = if plane.normal.y > 0.0 {
                box3.max.y
            } else {
                box3.min.y
            };

            let z2 = if plane.normal.z > 0.0 {
                box3.max.z
            } else {
                box3.min.z
            };


            let p1 = Vector3::new(x1, y1, z1);
            let p2 = Vector3::new(x2, y2, z2);

            let d1 = plane.distance_to_point(&p1);
            let d2 = plane.distance_to_point(&p2);

            // if both outside plane, no intersection
            d1 < 0.0 && d2 < 0.0
        })
    }

    pub fn contains_point(&self, point: &Vector3) -> bool {
        self.planes.into_iter().all(|plane| plane.distance_to_point(point) < 0.0)
    }
}
