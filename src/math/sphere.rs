use math::{Vector3, Box3, Matrix4, Plane};

#[derive(Debug,PartialEq)]
pub struct Sphere {
    pub center: Vector3,
    pub radius: f32,
}

impl Sphere {
    pub const DEFAULT: Sphere = Sphere {
        center: Vector3::ZERO,
        radius: 0.0,
    };

    pub fn new(center: Vector3, radius: f32) -> Sphere {
        Sphere {
            center: center,
            radius: radius,
        }
    }



    pub fn empty(&self) -> bool {
        self.radius <= 0.0
    }

    pub fn contains_point(&self, point: &Vector3) -> bool {
        point.distance_to_squared(&self.center) < self.radius.powi(2)
    }

    pub fn distance_to_point(&self, point: &Vector3) -> f32 {
        point.distance_to(&self.center) - self.radius
    }

    pub fn intersects_sphere(&self, sphere: &Sphere) -> bool {
        let radius_sum = self.radius + sphere.radius;
        sphere.center.distance_to_squared(&self.center) <= radius_sum.powi(2)
    }

    pub fn intersects_box(&self, box3: &Box3) -> bool {
        box3.intersects_sphere(self)
    }

    pub fn intersects_plane(&self, plane: &Plane) -> bool {
        // We use the following equation to compute the signed distance from
        // the center of the sphere to the plane.
        //
        // distance = q * n - d
        //
        // If this distance is greater than the radius of the sphere,
        // then there is no intersection.
        let &Plane { normal, constant } = plane;
        (self.center.dot(&normal) - constant).abs() <= self.radius
    }

    pub fn clamp_point(&self, point: &Vector3) -> Vector3 {
        let delta_length_squared = self.center.distance_to_squared(point);

        if delta_length_squared > self.radius.powi(2) {
            let n = point.subtract(&self.center).normalize();
            n.multiply_scalar(self.radius).add(&self.center)
        } else {
            *point
        }
    }

    pub fn bounding_box(&self) -> Box3 {
        Box3::new(&self.center, &self.center).expand_by_scalar(self.radius)
    }

    pub fn apply_matrix4(&self, matrix: &Matrix4) -> Sphere {
        Sphere {
            center: self.center.apply_matrix4(matrix),
            radius: self.radius * matrix.max_scale_on_axis(),
        }
    }

    pub fn translate(&self, offset: &Vector3) -> Sphere {
        Sphere { center: self.center.add(offset), ..*self }
    }
}

#[cfg(test)]
mod tests {
    use super::Sphere;
}