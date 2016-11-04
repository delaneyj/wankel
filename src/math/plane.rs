use math::*;

#[derive(Debug,PartialEq,Copy,Clone)]
pub struct Plane {
    pub normal: Vector3,
    pub constant: f32,
}

impl Plane {
    pub const DEFAULT: Plane = Plane {
        normal: Vector3 {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        },
        constant: 0.0,
    };

    pub fn new(normal: &Vector3, constant: f32) -> Plane {
        Plane {
            normal: *normal,
            constant: constant,
        }
    }

    pub fn from_normal_and_coplanar_point(normal: &Vector3, point: &Vector3) -> Plane {
        Plane::new(&normal.normalized(), -point.dot(normal))
    }

    pub fn from_coplanar_points(a: &Vector3, b: &Vector3, c: &Vector3) -> Plane {
        let normal = c.subtract(b).cross(&a.subtract(b)).normalized();

        // Q: should an error be thrown if normal is zero (e.g. degenerate plane)?
        Plane::from_normal_and_coplanar_point(&normal, a)
    }

    pub fn normalized(&self) -> Plane {
        // Note: will lead to a divide by zero if the plane is invalid.
        let inverse_normal_length = 1.0 / self.normal.length();
        Plane::new(&self.normal.multiply_scalar(inverse_normal_length),
                   self.constant * inverse_normal_length)
    }

    pub fn negate(&self) -> Plane {
        Plane::new(&self.normal.negate(), self.constant * -1.0)
    }

    pub fn distance_to_point(&self, point: &Vector3) -> f32 {
        self.normal.dot(point) + self.constant
    }

    pub fn distance_to_sphere(&self, sphere: &Sphere) -> f32 {
        self.distance_to_point(&sphere.center) - sphere.radius
    }

    pub fn project_point(&self, point: &Vector3) -> Vector3 {
        self.ortho_point(point).subtract(point).negate()
    }

    pub fn ortho_point(&self, point: &Vector3) -> Vector3 {
        let perpendicular_magnitude = self.distance_to_point(point);
        self.normal.multiply_scalar(perpendicular_magnitude)
    }

    pub fn intersect_line(&self, line: &Line3) -> Option<Vector3> {
        let direction = line.delta();
        let denominator = self.normal.dot(&direction);

        if denominator == 0.0 {
            // line is coplanar, return origin
            if self.distance_to_point(&line.start) == 0.0 {
                Some(line.start)
            } else {
                // Unsure if this is the correct method to handle this case.
                None
            }
        } else {
            let t = -(line.start.dot(&self.normal) + self.constant) / denominator;

            if t < 0.0 || t > 1.0 {
                None
            } else {
                Some(direction.multiply_scalar(t).add(&line.start))
            }
        }
    }

    pub fn intersects_line(&self, line: &Line3) -> bool {
        // Note: this tests if a line intersects the plane, not whether it (or its end-points) are coplanar with it.
        let start_sign = self.distance_to_point(&line.start);
        let end_sign = self.distance_to_point(&line.end);
        (start_sign < 0.0 && end_sign > 0.0) || (end_sign < 0.0 && start_sign > 0.0)
    }

    pub fn intersects_box(&self, box3: &Box3) -> bool {
        box3.intersects_plane(self)
    }

    pub fn intersects_sphere(&self, sphere: &Sphere) -> bool {
        sphere.intersects_plane(self)
    }

    pub fn coplanar_point(&self) -> Vector3 {
        self.normal.multiply_scalar(-self.constant)
    }

    pub fn apply_matrix4(&self, matrix: &Matrix4) -> Plane {
        let reference_point = self.coplanar_point().apply_matrix4(matrix);

        // transform normal based on theory here:
        // http://www.songho.ca/opengl/gl_normaltransform.html
        let normal_matrix = Matrix3::normal_matrix(matrix);
        let normal = self.normal.apply_matrix3(&normal_matrix).normalized();

        // recalculate constant (like in setFromNormalAndCoplanarPoint)
        let constant = -reference_point.dot(&normal);

        Plane::new(&normal, constant)
    }

    pub fn translate(&self, offset: &Vector3) -> Plane {
        Plane { constant: self.constant - offset.dot(&self.normal), ..*self }
    }
}

#[cfg(test)]
mod tests {
    use std::f32::consts::*;
    use math::*;

    fn compare_planes(a: &Plane, b: &Plane) -> bool {
        let threshold = 0.0001;
        let same = a.normal.distance_to(&b.normal) < threshold &&
                   (a.constant - b.constant).abs() < threshold;

        if !same {
            println!("a:{:?} b:{:?}", a, b);
        }
        same
    }

    #[test]
    fn constructor() {
        let a = Plane::DEFAULT;
        assert_eq!(a.normal.x, 1.0);
        assert_eq!(a.normal.y, 0.0);
        assert_eq!(a.normal.z, 0.0);
        assert_eq!(a.constant, 0.0);

        let b = Plane::new(&Vector3::ONE, 0.0);
        assert_eq!(b.normal.x, 1.0);
        assert_eq!(b.normal.y, 1.0);
        assert_eq!(b.normal.z, 1.0);
        assert_eq!(b.constant, 0.0);

        let c = Plane::new(&Vector3::ONE, 1.0);
        assert_eq!(c.normal.x, 1.0);
        assert_eq!(c.normal.y, 1.0);
        assert_eq!(c.normal.z, 1.0);
        assert_eq!(c.constant, 1.0);
    }


    #[test]
    fn from_normal_and_coplanar_point() {
        let normal = Vector3::ONE.normalized();
        let a = Plane::from_normal_and_coplanar_point(&normal, &Vector3::ZERO);

        assert!((a.normal.subtract(&normal)).length().abs() < 0.001);
        assert_eq!(a.constant, 0.0);
    }

    #[test]
    fn normalized() {
        let a = Plane::new(&Vector3::new(2.0, 0.0, 0.0), 2.0);
        let b = a.normalized();
        assert_eq!(b.normal.length(), 1.0);
        assert_eq!(b.normal, Vector3::new(1.0, 0.0, 0.0));
        assert_eq!(b.constant, 1.0);
    }

    #[test]
    fn negate_distance_to_point() {
        let a = Plane::new(&Vector3::new(2.0, 0.0, 0.0), -2.0);

        let b = a.normalized();
        assert_eq!(b.distance_to_point(&Vector3::new(4.0, 0.0, 0.0)), 3.0);
        assert_eq!(b.distance_to_point(&Vector3::new(1.0, 0.0, 0.0)), 0.0);

        let c = b.negate();
        assert_eq!(c.distance_to_point(&Vector3::new(4.0, 0.0, 0.0)), -3.0);
        assert_eq!(c.distance_to_point(&Vector3::new(1.0, 0.0, 0.0)), 0.0);
    }

    #[test]
    fn distance_to_point() {
        let a = Plane::new(&Vector3::new(2.0, 0.0, 0.0), -2.0);
        let b = a.normalized();
        assert_eq!(b.distance_to_point(&b.project_point(&Vector3::ZERO)), 0.0);
        assert_eq!(b.distance_to_point(&Vector3::new(4.0, 0.0, 0.0)), 3.0);
    }

    #[test]
    fn distance_to_sphere() {
        let a = Plane::new(&Vector3::new(1.0, 0.0, 0.0), 0.0);
        let b = Sphere::new(&Vector3::new(2.0, 0.0, 0.0), 1.0);

        assert_eq!(a.distance_to_sphere(&b), 1.0);

        let c = Plane::new(&Vector3::new(1.0, 0.0, 0.0), 2.0);
        assert_eq!(c.distance_to_sphere(&b), 3.0);
        let d = Plane::new(&Vector3::new(1.0, 0.0, 0.0), -2.0);
        assert_eq!(d.distance_to_sphere(&b), -1.0);
    }

    #[test]
    fn is_interestion_line_intersect_line() {
        let line = Line3::new(&Vector3::new(-10.0, 0.0, 0.0),
                              &Vector3::new(10.0, 0.0, 0.0));

        let a = Plane::new(&Vector3::new(1.0, 0.0, 0.0), 0.0);
        assert_eq!(a.intersects_line(&line), true);
        assert_eq!(a.intersect_line(&line), Some(Vector3::ZERO));

        let b = Plane::new(&Vector3::new(1.0, 0.0, 0.0), -3.0);
        assert_eq!(b.intersects_line(&line), true);
        assert_eq!(b.intersect_line(&line), Some(Vector3::new(3.0, 0.0, 0.0)));
    }

    #[test]
    fn project_point() {
        let a = Plane::new(&Vector3::new(1.0, 0.0, 0.0), 0.0);

        assert_eq!(a.project_point(&Vector3::new(10.0, 0.0, 0.0)),
                   Vector3::ZERO);
        assert_eq!(a.project_point(&Vector3::new(-10.0, 0.0, 0.0)),
                   Vector3::ZERO);

        let b = Plane::new(&Vector3::new(0.0, 1.0, 0.0), -1.0);
        assert_eq!(b.project_point(&Vector3::new(0.0, 0.0, 0.0)),
                   Vector3::new(0.0, 1.0, 0.0));
        assert_eq!(b.project_point(&Vector3::new(0.0, 1.0, 0.0)),
                   Vector3::new(0.0, 1.0, 0.0));
    }

    #[test]
    fn ortho_point() {
        let a = Plane::new(&Vector3::new(1.0, 0.0, 0.0), 0.0);

        assert_eq!(a.ortho_point(&Vector3::new(10.0, 0.0, 0.0)),
                   Vector3::new(10.0, 0.0, 0.0));
        assert_eq!(a.ortho_point(&Vector3::new(-10.0, 0.0, 0.0)),
                   Vector3::new(-10.0, 0.0, 0.0));
    }

    #[test]
    fn coplanar_point() {
        let a = Plane::new(&Vector3::new(1.0, 0.0, 0.0), 0.0);
        assert_eq!(a.distance_to_point(&a.coplanar_point()), 0.0);

        let b = Plane::new(&Vector3::new(0.0, 1.0, 0.0), -1.0);
        assert_eq!(b.distance_to_point(&b.coplanar_point()), 0.0);
    }

    #[test]
    fn apply_matrix4_translate() {
        let a = Plane::new(&Vector3::new(1.0, 0.0, 0.0), 0.0);

        let m = Matrix4::from_rotation_z(PI * 0.5);
        assert!(compare_planes(&a.apply_matrix4(&m),
                               &Plane::new(&Vector3::new(0.0, 1.0, 0.0), 0.0)));

        let b = Plane::new(&Vector3::new(0.0, 1.0, 0.0), -1.0);
        assert!(compare_planes(&b.apply_matrix4(&m),
                               &Plane::new(&Vector3::new(-1.0, 0.0, 0.0), -1.0)));

        let mt = Matrix4::from_translation(&Vector3::ONE);
        assert!(compare_planes(&b.apply_matrix4(&mt), &b.translate(&Vector3::ONE)));
    }
}