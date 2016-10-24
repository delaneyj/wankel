use math::*;

#[derive(Debug,PartialEq,Clone)]
pub struct Line3 {
    pub start: Vector3,
    pub end: Vector3,
}

impl Line3 {
    pub const ZERO: Line3 = Line3 {
        start: Vector3::ZERO,
        end: Vector3::ZERO,
    };

    pub fn new(start: &Vector3, end: &Vector3) -> Line3 {
        Line3 {
            start: *start,
            end: *end,
        }
    }

    pub fn center(&self) -> Vector3 {
        self.start.add(&self.end).multiply_scalar(0.5)
    }

    pub fn delta(&self) -> Vector3 {
        self.end.subtract(&self.start)
    }

    pub fn distance_squared(&self) -> f32 {
        self.start.distance_to_squared(&self.end)
    }

    pub fn distance(&self) -> f32 {
        self.start.distance_to(&self.end)
    }

    pub fn at(&self, t: f32) -> Vector3 {
        self.delta().multiply_scalar(t).add(&self.start)
    }

    pub fn closest_point_to_point_parameter(&self, point: &Vector3, clamp_to_line: bool) -> f32 {
        let start_p = point.subtract(&self.start);
        let start_end = self.end.subtract(&self.start);
        let start_end2 = start_end.dot(&start_end);
        let start_end_start_p = start_end.dot(&start_p);
        let t = start_end_start_p / start_end2;

        if clamp_to_line {
            t.max(0.0).min(1.0)
        } else {
            t
        }
    }

    pub fn closest_point_to_point(&self, point: &Vector3, clamp_to_line: bool) -> Vector3 {
        let t = self.closest_point_to_point_parameter(point, clamp_to_line);
        self.delta().multiply_scalar(t).add(&self.start)
    }

    pub fn apply_matrix4(&self, matrix: &Matrix4) -> Line3 {
        Line3::new(&self.start.apply_matrix4(matrix),
                   &self.end.apply_matrix4(matrix))
    }
}


#[cfg(test)]
mod tests {
    use math::*;

    #[test]
    fn constructor_equals() {
        let a = Line3::ZERO;
        assert_eq!(a.start, Vector3::ZERO);
        assert_eq!(a.end, Vector3::ZERO);

        let two3 = Vector3::new(2.0, 2.0, 2.0);
        let b = Line3::new(&two3, &Vector3::ONE);
        assert_eq!(b.start, two3);
        assert_eq!(b.end, Vector3::ONE);
    }

    #[test]
    fn at() {
        let a = Line3::new(&Vector3::ONE, &Vector3::new(1.0, 1.0, 2.0));
        let threshold = 0.0001;

        assert!(a.at(-1.0).distance_to(&Vector3::new(1.0, 1.0, 0.0)) < threshold);
        assert!(a.at(0.0).distance_to(&Vector3::ONE) < threshold);
        assert!(a.at(1.0).distance_to(&Vector3::new(1.0, 1.0, 2.0)) < threshold);
        assert!(a.at(2.0).distance_to(&Vector3::new(1.0, 1.0, 3.0)) < threshold);
    }

    #[test]
    fn closest_point_to_point_closest_point_to_point_parameter() {
        let a = Line3::new(&Vector3::ONE, &Vector3::new(1.0, 1.0, 2.0));

        // nearby the ray
        assert_eq!(a.closest_point_to_point_parameter(&Vector3::ZERO, true),
                   0.0);
        let b1 = a.closest_point_to_point(&Vector3::ZERO, true);
        assert!(b1.distance_to(&Vector3::ONE) < 0.0001);

        // nearby the ray
        assert_eq!(a.closest_point_to_point_parameter(&Vector3::ZERO, false),
                   -1.0);
        let b2 = a.closest_point_to_point(&Vector3::ZERO, false);
        assert!(b2.distance_to(&Vector3::new(1.0, 1.0, 0.0)) < 0.0001);

        // nearby the ray
        assert_eq!(a.closest_point_to_point_parameter(&Vector3::new(1.0, 1.0, 5.0), true),
                   1.0);
        let b = a.closest_point_to_point(&Vector3::new(1.0, 1.0, 5.0), true);
        assert!(b.distance_to(&Vector3::new(1.0, 1.0, 2.0)) < 0.0001);

        // exactly on the ray
        assert_eq!(a.closest_point_to_point_parameter(&Vector3::ONE, true), 0.0);
        let c = a.closest_point_to_point(&Vector3::ONE, true);
        assert!(c.distance_to(&Vector3::ONE) < 0.0001);
    }
}