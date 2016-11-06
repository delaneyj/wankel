use std::f32::INFINITY;
use math::{Vector3, Plane, Line3};

pub struct Triangle {
    a: Vector3,
    b: Vector3,
    c: Vector3,
}

impl Triangle {
    const DEFAULT: Triangle = Triangle {
        a: Vector3::ZERO,
        b: Vector3::ZERO,
        c: Vector3::ZERO,
    };

    pub fn area(&self) -> f32 {
        let v0 = self.c.subtract(&self.b);
        let v1 = self.a.subtract(&self.b);
        v0.cross(&v1).length() * 0.5
    }

    pub fn midpoint(&self) -> Vector3 {
        self.a.add(&self.b).add(&self.c).multiply_scalar(1.0 / 3.0)
    }

    pub fn normal(&self) -> Vector3 {
        let result_a = self.c.subtract(&self.b);
        let v0 = self.a.subtract(&self.b);
        let result_b = result_a.cross(&v0);
        let result_b_length_squared = result_b.length_squared();

        if result_b_length_squared > 0.0 {
            result_b.multiply_scalar(1.0 / result_b_length_squared.sqrt())
        } else {
            Vector3::ZERO
        }
    }

    pub fn plane(&self) -> Plane {
        Plane::from_coplanar_points(&self.a, &self.b, &self.c)
    }

    pub fn barycentric_coordinates_from_point(&self, point: &Vector3) -> Option<Vector3> {
        let v0 = self.c.subtract(&self.a);
        let v1 = self.b.subtract(&self.a);
        let v2 = point.subtract(&self.a);

        let dot00 = v0.dot(&v0);
        let dot01 = v0.dot(&v1);
        let dot02 = v0.dot(&v2);
        let dot11 = v1.dot(&v1);
        let dot12 = v1.dot(&v2);
        let denom = dot00 * dot11 - dot01 * dot01;

        // collinear or singular triangle
        if denom == 0.0 {
            None
        } else {
            let inv_denom = 1.0 / denom;
            let u = (dot11 * dot02 - dot01 * dot12) * inv_denom;
            let v = (dot00 * dot12 - dot01 * dot02) * inv_denom;

            // barycentric coordinates must always sum to 1
            Some(Vector3::new(1.0 - u - v, v, u))
        }
    }

    pub fn contains_point(&self, point: &Vector3) -> bool {
        self.barycentric_coordinates_from_point(point).is_some()
    }

    pub fn closest_point_to_point(&self, point: &Vector3) -> Vector3 {
        // project the point onto the plane of the triangle
        let projected_point = self.plane().project_point(point);


        // check if the projection lies within the triangle
        if self.contains_point(&projected_point) {
            // if so, this is the closest point
            projected_point
        } else {
            // if not, the point falls outside the triangle. the result is the closest point to the triangle's edges or vertices
            let mut min_distance = INFINITY;
            let mut actual_closest = Vector3::ZERO;

            for line in vec![
                    Line3::new(&self.a,&self.b),
                    Line3::new(&self.b,&self.c),
                    Line3::new(&self.c,&self.a),
                ] {
                let closest = line.closest_point_to_point(&projected_point, true);
                let distance = projected_point.distance_to_squared(&closest);
                if distance < min_distance {
                    min_distance = distance;
                    actual_closest = closest;
                }
            }
            actual_closest
        }

    }
}