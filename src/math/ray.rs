use math::*;

#[derive(Debug,PartialEq)]
pub struct Ray {
    pub origin: Vector3,
    pub direction: Vector3,
}

impl Ray {
    pub fn at(&self, t: f32) -> Vector3 {
        self.direction.multiply_scalar(t).add(&self.origin)
    }

    pub fn look_at(&self, v: &Vector3) -> Ray {
        Ray { direction: v.subtract(&self.origin).normalized(), ..*self }
    }

    pub fn recast(&self, t: f32) -> Ray {
        Ray { origin: self.at(t), ..*self }
    }

    pub fn closest_point_to_point(&self, point: &Vector3) -> Vector3 {
        let result = point.subtract(&self.origin);
        let direction_distance = result.dot(&self.direction);

        if direction_distance < 0.0 {
            self.origin
        } else {
            self.direction.multiply_scalar(direction_distance).add(&self.origin)
        }
    }

    pub fn distance_to_point(&self, point: &Vector3) -> f32 {
        self.distance_squared_to_point(point).sqrt()
    }

    pub fn distance_squared_to_point(&self, point: &Vector3) -> f32 {
        let direction_distance = point.subtract(&self.origin).dot(&self.direction);

        if direction_distance < 0.0 {
            // point behind the ray
            self.origin.distance_to_squared(point)
        } else {
            let v = self.direction.multiply_scalar(direction_distance).add(&self.origin);
            v.distance_to_squared(point)
        }
    }

    // (Distance,PointOnRay,PointOnSegment)
    pub fn distance_squared_to_segment(&self,
                                       v0: &Vector3,
                                       v1: &Vector3)
                                       -> (f32, Vector3, Vector3) {
        // from http://www.geometrictools.com/GTEngine/Include/Mathematics/GteDistRaySegment.h
        // It returns the min distance between the ray and the segment
        // defined by v0 and v1
        // It can also set two optional targets :
        // - The closest point on the ray
        // - The closest point on the segment
        let seg_center = v0.add(v1).multiply_scalar(0.5);
        let seg_dir = v1.subtract(v0).normalized();
        let diff = self.origin.subtract(&seg_center);

        let seg_extent = v0.distance_to(v1) * 0.5;
        let a01 = -self.direction.dot(&seg_dir);
        let b0 = diff.dot(&self.direction);
        let b1 = -diff.dot(&seg_dir);
        let c = diff.length_squared();
        let det = (1.0 - a01 * a01).abs();

        let mut s0: f32;
        let mut s1: f32;

        let square_distance = if det > 0.0 {
            // The ray and segment are not parallel.
            s0 = a01 * b1 - b0;
            s1 = a01 * b0 - b1;
            let ext_det = seg_extent * det;

            if s0 >= 0.0 {
                if s1 >= -ext_det {
                    if s1 <= ext_det {
                        // region 0
                        // Minimum at interior points of ray and segment.
                        let inv_det = 1.0 / det;
                        s0 *= inv_det;
                        s1 *= inv_det;
                        0.0 * (s0 + a01 * s1 + 2.0 * b0) + s1 * (a01 * s0 + s1 + 2.0 * b1) + c
                    } else {
                        // region 1
                        s1 = seg_extent;
                        s0 = (-(a01 * s1 + b0)).max(0.0);
                        -s0 * s0 + s1 * (s1 + 2.0 * b1) + c
                    }

                } else {

                    // region 5
                    s1 = -seg_extent;
                    s0 = (-(a01 * s1 + b0)).max(0.0);
                    -s0 * s0 + s1 * (s1 + 2.0 * b1) + c
                }
            } else if s1 <= -ext_det {
                // region 4

                s0 = (-(-a01 * seg_extent + b0)).max(0.0);
                s1 = if s0 > 0.0 {
                    -seg_extent
                } else {
                    (-seg_extent).max(-b1).min(seg_extent)
                };
                -s0 * s0 + s1 * (s1 + 2.0 * b1) + c

            } else if s1 <= ext_det {
                // region 3
                s0 = 0.0;
                s1 = seg_extent.min((-seg_extent).max(-b1));
                s1 * (s1 + 2.0 * b1) + c
            } else {
                // region 2

                s0 = (-(a01 * seg_extent + b0)).max(0.0);
                s1 = if s0 > 0.0 {
                    seg_extent
                } else {
                    seg_extent.min((-seg_extent).max(-b1))
                };
                -s0 * s0 + s1 * (s1 + 2.0 * b1) + c
            }
        } else {
            // Ray and segment are parallel.
            s1 = if a01 > 0.0 { -seg_extent } else { seg_extent };
            s0 = (-(a01 * s1 + b0)).max(0.0);
            -s0 * s0 + s1 * (s1 + 2.0 * b1) + c
        };

        let point_on_ray = self.direction.multiply_scalar(s0).add(&self.origin);
        let point_on_segment = seg_dir.multiply_scalar(s1).add(&seg_center);

        (square_distance, point_on_ray, point_on_segment)
    }

    pub fn intersect_sphere(&self, sphere: &Sphere) -> Option<Vector3> {
        let v1 = sphere.center.subtract(&self.origin);
        let tca = v1.dot(&self.direction);
        let d2 = v1.dot(&v1) - tca * tca;
        let radius2 = sphere.radius.powi(2);

        if d2 > radius2 {
            None
        } else {
            let thc = (radius2 - d2).sqrt();
            let t0 = tca - thc; // first intersect point - entrance on front of sphere
            let t1 = tca + thc; // second intersect point - exit point on back of sphere

            // test to see if both t0 and t1 are behind the ray - if so, return null
            if t0 < 0.0 && t1 < 0.0 {
                None
            } else if t0 < 0.0 {
                // test to see if t0 is behind the ray:
                // if it is, the ray is inside the sphere, so return the second exit point scaled by t1,
                // in order to always return an intersect point that is in front of the ray.
                Some(self.at(t1))
            } else {
                // else t0 is in front of the ray, so return the first collision point scaled by t0
                Some(self.at(t0))
            }
        }
    }

    pub fn intersects_sphere(&self, sphere: &Sphere) -> bool {
        self.distance_to_point(&sphere.center) <= sphere.radius
    }

    pub fn distance_to_plane(&self, plane: &Plane) -> Option<f32> {
        let denominator = plane.normal.dot(&self.direction);

        if denominator == 0.0 {
            // line is coplanar, return origin
            if plane.distance_to_point(&self.origin) == 0.0 {
                Some(0.0)
            } else {
                None
            }
        } else {
            let t = -(self.origin.dot(&plane.normal) + plane.constant) / denominator;

            // Return if the ray never intersects the plane

            if t >= 0.0 { Some(t) } else { None }
        }
    }

    pub fn intersect_plane(&self, plane: &Plane) -> Option<Vector3> {
        match self.distance_to_plane(plane) {
            Some(t) => Some(self.at(t)),
            None => None,
        }
    }

    pub fn intersects_plane(&self, plane: &Plane) -> bool {
        // check if the ray lies on the plane first
        let dist_to_point = plane.distance_to_point(&self.origin);

        if dist_to_point == 0.0 {
            true
        } else {
            let denominator = plane.normal.dot(&self.direction);

            // false means ray origin is behind the plane (and is pointing behind it)
            denominator * dist_to_point < 0.0
        }
    }

    pub fn intersect_box(&self, box3: &Box3) -> Option<Vector3> {
        let invdirx = 1.0 / self.direction.x;
        let invdiry = 1.0 / self.direction.y;
        let invdirz = 1.0 / self.direction.z;

        let (mut tmin, mut tmax) = if invdirx >= 0.0 {
            ((box3.min.x - self.origin.x) * invdirx, (box3.max.x - self.origin.x) * invdirx)
        } else {
            ((box3.max.x - self.origin.x) * invdirx, (box3.min.x - self.origin.x) * invdirx)
        };

        let (tymin, tymax) = if invdiry >= 0.0 {
            ((box3.min.y - self.origin.y) * invdiry, (box3.max.y - self.origin.y) * invdiry)
        } else {
            ((box3.max.y - self.origin.y) * invdiry, (box3.min.y - self.origin.y) * invdiry)
        };

        if (tmin > tymax) || (tymin > tmax) {
            None
        } else {
            // These lines also handle the case where tmin or tmax is NaN
            // (result of 0 * Infinity). x !== x returns true if x is NaN

            if tymin > tmin {
                tmin = tymin;
            }

            if tymax < tmax {
                tmax = tymax;
            }

            let (tzmin, tzmax) = if invdirz >= 0.0 {
                ((box3.min.z - self.origin.z) * invdirz, (box3.max.z - self.origin.z) * invdirz)
            } else {
                ((box3.max.z - self.origin.z) * invdirz, (box3.min.z - self.origin.z) * invdirz)
            };

            if tmin > tzmax || tzmin > tmax {
                return None;
            }

            if tzmin > tmin {
                tmin = tzmin;
            }

            if tzmax < tmax {
                tmax = tzmax;
            }

            // return point closest to the ray (positive side)

            if tmax < 0.0 {
                None
            } else {
                let x = if tmin >= 0.0 { tmin } else { tmax };
                Some(self.at(x))
            }
        }
    }

    // pub fn intersects_box(&self,box:&Box3)->bool {
    //  self.intersect_box3.is_some()
    // }

    pub fn intersect_triangle(&self,
                              a: &Vector3,
                              b: &Vector3,
                              c: &Vector3,
                              back_face_culling: bool)
                              -> Option<Vector3> {

        // Compute the offset origin, edges, and normal.
        // from http://www.geometrictools.com/GTEngine/Include/Mathematics/GteIntrRay3Triangle3.h
        let edge1 = b.subtract(a);
        let edge2 = c.subtract(a);
        let normal = edge1.cross(&edge2);

        // Solve Q + t*D = b1*E1 + b2*E2 (Q = kDiff, D = ray direction,
        // E1 = kEdge1, E2 = kEdge2, N = Cross(E1,E2)) by
        //   |Dot(D,N)|*b1 = sign(Dot(D,N))*Dot(D,Cross(Q,E2))
        //   |Dot(D,N)|*b2 = sign(Dot(D,N))*Dot(D,Cross(E1,Q))
        //   |Dot(D,N)|*t = -sign(Dot(D,N))*Dot(Q,N)

        let ddn_init = self.direction.dot(&normal);
        let (ddn, sign, should_return_none) = if ddn_init > 0.0 {
            (ddn_init, 1.0, back_face_culling)
        } else if ddn_init < 0.0 {
            (-ddn_init, -1.0, false)
        } else {
            (0.0, 0.0, true)
        };

        if should_return_none {
            None
        } else {
            let diff = self.origin.subtract(a);
            let dd_qx_e2 = sign * self.direction.dot(&edge2.cross(&diff));

            // b1 < 0, no intersection
            if dd_qx_e2 < 0.0 {
                None
            } else {
                let dd_e1_x_q = sign * self.direction.dot(&edge1.cross(&diff));

                // b2 < 0 or b1+b2 > 1, no intersection
                if dd_e1_x_q < 0.0 || dd_qx_e2 + dd_e1_x_q > ddn {
                    None
                } else {
                    // Line intersects triangle, check if ray does.
                    let qdn = -sign * diff.dot(&normal);

                    // t < 0, no intersection
                    if qdn < 0.0 {
                        None
                    } else {
                        // Ray intersects triangle.
                        Some(self.at(qdn / ddn))
                    }
                }
            }
        }
    }


    pub fn apply_matrix_4(&self, matrix4: &Matrix4) -> Ray {
        let o = self.origin.apply_matrix4(matrix4);
        let d = self.direction.add(&self.origin).apply_matrix4(matrix4).subtract(&o).normalized();


        Ray {
            direction: d,
            origin: o,
        }
    }
}

#[cfg(test)]
mod tests {
    // use std::f32::consts::*;
    // use math::*;
}