use std::f32::{INFINITY, NEG_INFINITY};

use math::{Vector3, Sphere, Plane, Matrix4};
// use core::Object3D;

#[derive(Debug,PartialEq,Copy,Clone)]
pub struct Box3 {
    pub min: Vector3,
    pub max: Vector3,
}

impl Box3 {
    pub const EMPTY: Box3 = Box3 {
        min: Vector3 {
            x: INFINITY,
            y: INFINITY,
            z: INFINITY,
        },
        max: Vector3 {
            x: NEG_INFINITY,
            y: NEG_INFINITY,
            z: NEG_INFINITY,
        },
    };

    pub fn new(min: &Vector3, max: &Vector3) -> Box3 {
        Box3 {
            min: *min,
            max: *max,
        }
    }

    pub fn from_points(points: &[&Vector3]) -> Box3 {
        points.into_iter().fold(Box3::EMPTY, |acc, p| acc.expand_by_point(p))
    }

    pub fn from_center_and_size(center: &Vector3, size: &Vector3) -> Box3 {
        let half_size = size.multiply_scalar(0.5);
        Box3::new(&center.subtract(&half_size), &center.add(&half_size))
    }

    // pub fn from_object(object:&Object3D) {
    // 		// Computes the world-axis-aligned bounding box of an object (including its children),
    // 		// accounting for both the object's, and children's, world transforms

    // 		let updated_object_matrix = object.updateMatrixWorld( true );


    // 			self.makeEmpty();

    // 			object.traverse( function ( node ) {

    // 				let geometry = node.geometry;

    // 				if ( geometry !== undefined ) {

    // 					if ( (geometry && geometry.isGeometry) ) {

    // 						let vertices = geometry.vertices;

    // 						for ( let i = 0, il = vertices.length; i < il; i ++ ) {

    // 							v1.copy( vertices[ i ] );
    // 							v1.applyMatrix4( node.matrixWorld );

    // 							scope.expandByPoint( v1 );

    // 						}

    // 					} else if ( (geometry && geometry.isBufferGeometry) ) {

    // 						let attribute = geometry.attributes.position;

    // 						if ( attribute !== undefined ) {

    // 							let array, offset, stride;

    // 							if ( (attribute && attribute.isInterleavedBufferAttribute) ) {

    // 								array = attribute.data.array;
    // 								offset = attribute.offset;
    // 								stride = attribute.data.stride;

    // 							} else {

    // 								array = attribute.array;
    // 								offset = 0;
    // 								stride = 3;

    // 							}

    // 							for ( let i = offset, il = array.length; i < il; i += stride ) {

    // 								v1.fromArray( array, i );
    // 								v1.applyMatrix4( node.matrixWorld );

    // 								scope.expandByPoint( v1 );

    // 							}

    // 						}

    // 					}

    // 				}

    // 			} );

    // 			return this;

    // 		};

    // 	}(),

    pub fn is_empty(&self) -> bool {
        // this is a more robust check for empty than ( volume <= 0 ) because volume can get positive with two negative axes
        self.max.x < self.min.x || self.max.y < self.min.y || self.max.z < self.min.z
    }

    pub fn center(&self) -> Vector3 {
        if self.is_empty() {
            Vector3::ZERO
        } else {
            self.min.add(&self.max).multiply_scalar(0.5)
        }
    }

    pub fn size(&self) -> Vector3 {
        if self.is_empty() {
            Vector3::ZERO
        } else {
            self.max.subtract(&self.min)
        }
    }

    pub fn expand_by_point(&self, point: &Vector3) -> Box3 {
        Box3::new(&self.min.min(point), &self.max.max(point))
    }

    pub fn expand_by_vector(&self, vector: &Vector3) -> Box3 {
        Box3::new(&self.min.subtract(vector), &self.max.add(vector))
    }

    pub fn expand_by_scalar(&self, s: f32) -> Box3 {
        Box3::new(&self.min.add_scalar(-s), &self.max.add_scalar(s))
    }

    pub fn contains_point(&self, point: &Vector3) -> bool {
        !(point.x < self.min.x || point.x > self.max.x || point.y < self.min.y ||
          point.y > self.max.y || point.z < self.min.z || point.z > self.max.z)
    }

    pub fn contains_box(&self, box3: &Box3) -> bool {
        self.min.x <= box3.min.x && box3.max.x <= self.max.x && self.min.y <= box3.min.y &&
        box3.max.y <= self.max.y && self.min.z <= box3.min.z && box3.max.z <= self.max.z
    }

    pub fn intersects_box(&self, box3: &Box3) -> bool {
        // using 6 splitting planes to rule out intersections.
        !(box3.max.x < self.min.x || box3.min.x > self.max.x || box3.max.y < self.min.y ||
          box3.min.y > self.max.y || box3.max.z < self.min.z || box3.min.z > self.max.z)
    }

    pub fn intersects_sphere(&self, sphere: &Sphere) -> bool {
        // Find the point on the AABB closest to the sphere center.
        let closest_point = self.clamp_point(&sphere.center);
        let Sphere { center, radius } = *sphere;


        // If that point is inside the sphere, the AABB and sphere intersect.
        closest_point.distance_to_squared(&center) <= radius.powi(2)
    }

    pub fn intersects_plane(&self, plane: &Plane) -> bool {
        // We compute the minimum and maximum dot product values. If those values
        // are on the same side (back or front) of the plane, then there is no intersection.
        let (x_min, x_max) = if plane.normal.x > 0.0 {
            (plane.normal.x * self.min.x, plane.normal.x * self.max.x)
        } else {
            (plane.normal.x * self.max.x, plane.normal.x * self.min.x)
        };

        let (y_min, y_max) = if plane.normal.y > 0.0 {
            (plane.normal.y * self.min.y, plane.normal.y * self.max.y)

        } else {
            (plane.normal.y * self.max.y, plane.normal.y * self.min.y)

        };

        let (z_min, z_max) = if plane.normal.z > 0.0 {
            (plane.normal.z * self.min.z, plane.normal.z * self.max.z)
        } else {
            (plane.normal.z * self.max.z, plane.normal.z * self.min.z)
        };

        let min = x_min + y_min + z_min;
        let max = x_max + y_max + z_max;
        min <= plane.constant && max >= plane.constant
    }

    pub fn clamp_point(&self, point: &Vector3) -> Vector3 {
        point.clamp(&self.min, &self.max)
    }

    pub fn distance_to_point(&self, point: &Vector3) -> f32 {
        point.clamp(&self.min, &self.max).subtract(point).length()
    }

    pub fn bounding_sphere(&self) -> Sphere {
        Sphere::new(&self.center(), self.size().length() * 0.5)
    }

    pub fn intersect_box(&self, box3: &Box3) -> Option<Box3> {
        let min = self.min.max(&box3.min);
        let max = self.max.min(&box3.max);
        let intersect = Box3::new(&min, &max);

        // ensure that if there is no overlap, the result is fully empty, not slightly empty with non-inf/+inf values that will cause subsequence intersects to erroneously return valid values.
        if intersect.is_empty() {
            None
        } else {
            Some(*self)
        }
    }

    pub fn union(&self, box3: &Box3) -> Box3 {
        Box3::new(&self.min.min(&box3.min), &self.max.max(&box3.max))
    }

    pub fn apply_matrix4(&self, m: &Matrix4) -> Box3 {
        if self.is_empty() {
            Box3::EMPTY
        } else {
            // 	// NOTE: I am using a binary pattern to specify all 2^3 combinations below
            let v000 = Vector3::new(self.min.x, self.min.y, self.min.z).apply_matrix4(m);
            let v001 = Vector3::new(self.min.x, self.min.y, self.max.z).apply_matrix4(m);
            let v010 = Vector3::new(self.min.x, self.max.y, self.min.z).apply_matrix4(m);
            let v011 = Vector3::new(self.min.x, self.max.y, self.max.z).apply_matrix4(m);
            let v100 = Vector3::new(self.max.x, self.min.y, self.min.z).apply_matrix4(m);
            let v101 = Vector3::new(self.max.x, self.min.y, self.max.z).apply_matrix4(m);
            let v110 = Vector3::new(self.max.x, self.max.y, self.min.z).apply_matrix4(m);
            let v111 = Vector3::new(self.max.x, self.max.y, self.max.z).apply_matrix4(m);
            let points = vec![&v000, &v001, &v010, &v011, &v100, &v101, &v110, &v111];
            Box3::from_points(&points)
        }
    }

    pub fn translate(&self, offset: &Vector3) -> Box3 {
        Box3::new(&self.min.add(offset), &self.max.add(offset))
    }
}

#[cfg(test)]
mod tests {
    use math::*;

    const V3_TWO: Vector3 = Vector3 {
        x: 2.0,
        y: 2.0,
        z: 2.0,
    };

    #[test]
    fn constructor() {
        let a = Box3::EMPTY;
        assert_eq!(a.min, Vector3::INFINITY);
        assert_eq!(a.max, Vector3::NEG_INFINITY);

        let b = Box3::new(&Vector3::ZERO, &Vector3::ZERO);
        assert_eq!(b.min, Vector3::ZERO);
        assert_eq!(b.max, Vector3::ZERO);

        let c = Box3::new(&Vector3::ZERO, &Vector3::ONE);
        assert_eq!(c.min, Vector3::ZERO);
        assert_eq!(c.max, Vector3::ONE);
    }

    #[test]
    fn from_points() {
        let a = Box3::from_points(&vec![&Vector3::ZERO, &Vector3::ONE, &V3_TWO]);
        assert_eq!(a.min, Vector3::ZERO);
        assert_eq!(a.max, V3_TWO);

        let b = Box3::from_points(&vec![&Vector3::ONE].as_slice());
        assert_eq!(b.min, Vector3::ONE);
        assert_eq!(b.max, Vector3::ONE);

        let c = Box3::from_points(&[]);
        assert_eq!(c.is_empty(), true);
    }

    #[test]
    fn empty() {
        let a = Box3::EMPTY;
        assert_eq!(a.is_empty(), true);

        let b = Box3::new(&Vector3::ZERO, &Vector3::ZERO);
        assert_eq!(b.is_empty(), false);
    }

    #[test]
    fn center() {
        let a = Box3::new(&Vector3::ZERO, &Vector3::ZERO);
        assert_eq!(a.center(), Vector3::ZERO);

        let b = Box3::new(&Vector3::ZERO, &Vector3::ONE);
        let midpoint = Vector3::ONE.multiply_scalar(0.5);
        assert_eq!(b.center(), midpoint);
    }

    #[test]
    fn size() {
        let a = Box3::new(&Vector3::ZERO, &Vector3::ZERO);
        assert_eq!(a.size(), Vector3::ZERO);

        let b = Box3::new(&Vector3::ZERO, &Vector3::ONE);
        assert_eq!(b.size(), Vector3::ONE);
    }

    #[test]
    fn expand_by_point() {
        let a = Box3::new(&Vector3::ZERO, &Vector3::ZERO);

        let b = a.expand_by_point(&Vector3::ZERO);
        assert_eq!(b.size(), Vector3::ZERO);

        let c = b.expand_by_point(&Vector3::ONE);
        assert_eq!(c.size(), Vector3::ONE);

        let d = c.expand_by_point(&Vector3::ONE.negate());
        assert_eq!(d.size(), Vector3::ONE.multiply_scalar(2.0));
        assert_eq!(a.center(), Vector3::ZERO);
    }

    #[test]
    fn expand_by_vector() {
        let a = Box3::new(&Vector3::ZERO, &Vector3::ZERO);
        let b = a.expand_by_vector(&Vector3::ZERO);
        assert_eq!(b.size(), Vector3::ZERO);

        let c = b.expand_by_vector(&Vector3::ONE);
        assert_eq!(c.size(), Vector3::ONE.multiply_scalar(2.0));
        assert_eq!(c.center(), Vector3::ZERO);
    }

    #[test]
    fn expand_by_scalar() {
        let a = Box3::new(&Vector3::ZERO, &Vector3::ZERO);
        let b = a.expand_by_scalar(0.0);
        assert_eq!(a.size(), Vector3::ZERO);

        let c = b.expand_by_scalar(1.0);
        assert_eq!(c.size(), Vector3::ONE.multiply_scalar(2.0));
        assert_eq!(c.center(), Vector3::ZERO);
    }

    #[test]
    fn contains_point() {
        let a = Box3::new(&Vector3::ZERO, &Vector3::ZERO);

        assert_eq!(a.contains_point(&Vector3::ZERO), true);
        assert_eq!(a.contains_point(&Vector3::ONE), false);

        let b = a.expand_by_scalar(1.0);
        assert_eq!(b.contains_point(&Vector3::ZERO), true);
        assert_eq!(b.contains_point(&Vector3::ONE), true);
        assert_eq!(b.contains_point(&Vector3::ONE.negate()), true);
    }

    #[test]
    fn contains_box() {
        let a = Box3::new(&Vector3::ZERO, &Vector3::ZERO);
        let b = Box3::new(&Vector3::ZERO, &Vector3::ONE);
        let c = Box3::new(&Vector3::ONE.negate(), &Vector3::ONE);

        assert_eq!(a.contains_box(&a), true);
        assert_eq!(a.contains_box(&b), false);
        assert_eq!(a.contains_box(&c), false);

        assert_eq!(b.contains_box(&a), true);
        assert_eq!(c.contains_box(&a), true);
        assert_eq!(b.contains_box(&c), false);
    }

    #[test]
    fn get_parameter() {
        // let a = Box3::new(&Vector3::ZERO, &Vector3::ONE);
        // let b = Box3::new(&Vector3::ONE.negate(), &Vector3::ONE);
        // assert_eq!( a.getParameter( Vector3::new( 0, 0, 0 ) ),Vector3::new( 0, 0, 0 ) ));
        // assert_eq!( a.getParameter( Vector3::new( 1, 1, 1 ) ),Vector3::new( 1, 1, 1 ) ));
        //
        // assert_eq!( b.getParameter( Vector3::new( -1, -1, -1 ) ),Vector3::new( 0, 0, 0 ) ));
        // assert_eq!( b.getParameter( Vector3::new( 0, 0, 0 ) ),Vector3::new( 0.5, 0.5, 0.5 ) ));
        // assert_eq!( b.getParameter( Vector3::new( 1, 1, 1 ) ),Vector3::new( 1, 1, 1 ) ));
        // });
        //
        // #[test] fn clampPoint() {
        // let a = Box3::new( &Vector3::ZERO, &Vector3::ZERO );
        // let b = Box3::new( &Vector3::ONE.negate(), &Vector3::ONE );
        //
        // assert_eq!( a.clampPoint( Vector3::new( 0, 0, 0 ) ),Vector3::new( 0, 0, 0 ) ));
        // assert_eq!( a.clampPoint( Vector3::new( 1, 1, 1 ) ),Vector3::new( 0, 0, 0 ) ));
        // assert_eq!( a.clampPoint( Vector3::new( -1, -1, -1 ) ),Vector3::new( 0, 0, 0 ) ));
        //
        // assert_eq!( b.clampPoint( Vector3::new( 2, 2, 2 ) ),Vector3::new( 1, 1, 1 ) ));
        // assert_eq!( b.clampPoint( Vector3::new( 1, 1, 1 ) ),Vector3::new( 1, 1, 1 ) ));
        // assert_eq!( b.clampPoint( Vector3::new( 0, 0, 0 ) ),Vector3::new( 0, 0, 0 ) ));
        // assert_eq!( b.clampPoint( Vector3::new( -1, -1, -1 ) ),Vector3::new( -1, -1, -1 ) ));
        // assert_eq!( b.clampPoint( Vector3::new( -2, -2, -2 ) ),Vector3::new( -1, -1, -1 ) ));
    }

    #[test]
    fn distance_to_point() {
        let a = Box3::new(&Vector3::ZERO, &Vector3::ZERO);
        let b = Box3::new(&Vector3::ONE.negate(), &Vector3::ONE);
        let sqrt_three = (3.0_f32).sqrt();

        assert_eq!(a.distance_to_point(&Vector3::new(0.0, 0.0, 0.0)), 0.0);
        assert_eq!(a.distance_to_point(&Vector3::new(1.0, 1.0, 1.0)),
                   sqrt_three);
        assert_eq!(a.distance_to_point(&Vector3::new(-1.0, -1.0, -1.0)),
                   sqrt_three);

        assert_eq!(b.distance_to_point(&Vector3::new(2.0, 2.0, 2.0)),
                   sqrt_three);
        assert_eq!(b.distance_to_point(&Vector3::new(1.0, 1.0, 1.0)), 0.0);
        assert_eq!(b.distance_to_point(&Vector3::new(0.0, 0.0, 0.0)), 0.0);
        assert_eq!(b.distance_to_point(&Vector3::new(-1.0, -1.0, -1.0)), 0.0);
        assert_eq!(b.distance_to_point(&Vector3::new(-2.0, -2.0, -2.0)),
                   sqrt_three);
    }

    #[test]
    fn intersects_box() {
        let a = Box3::new(&Vector3::ZERO, &Vector3::ZERO);
        let b = Box3::new(&Vector3::ZERO, &Vector3::ONE);
        let c = Box3::new(&Vector3::ONE.negate(), &Vector3::ONE);

        assert_eq!(a.intersect_box(&a).is_some(), true);
        assert_eq!(a.intersect_box(&b).is_some(), true);
        assert_eq!(a.intersect_box(&c).is_some(), true);

        assert_eq!(b.intersect_box(&a).is_some(), true);
        assert_eq!(c.intersect_box(&a).is_some(), true);
        assert_eq!(b.intersect_box(&c).is_some(), true);

        let d = b.translate(&Vector3::new(2.0, 2.0, 2.0));
        assert_eq!(a.intersect_box(&d), None);
        assert_eq!(d.intersect_box(&a), None);
        assert_eq!(d.intersect_box(&c), None);
    }

    #[test]
    fn intersects_sphere() {
        let a = Box3::new(&Vector3::ZERO, &Vector3::ONE);
        let b = Sphere::new(&Vector3::ZERO, 1.0);

        assert_eq!(a.intersects_sphere(&b), true);

        let c = b.translate(&Vector3::new(2.0, 2.0, 2.0));
        assert_eq!(a.intersects_sphere(&c), false);
    }

    #[test]
    fn intersects_plane() {
        let a = Box3::new(&Vector3::ZERO, &Vector3::ONE);
        let b = Plane::new(&Vector3::new(0.0, 1.0, 0.0), 1.0);
        let c = Plane::new(&Vector3::new(0.0, 1.0, 0.0), 1.25);
        let d = Plane::new(&Vector3::new(0.0, -1.0, 0.0), 1.25);

        assert_eq!(a.intersects_plane(&b), true);
        assert_eq!(a.intersects_plane(&c), false);
        assert_eq!(a.intersects_plane(&d), false);
    }

    #[test]
    fn bounding_sphere() {
        let a = Box3::new(&Vector3::ZERO, &Vector3::ZERO);
        let b = Box3::new(&Vector3::ZERO, &Vector3::ONE);
        let c = Box3::new(&Vector3::ONE.negate(), &Vector3::ONE);

        assert_eq!(a.bounding_sphere(), Sphere::new(&Vector3::ZERO, 0.0));
        assert_eq!(b.bounding_sphere(),
                   Sphere::new(&Vector3::ONE.multiply_scalar(0.5), 3.0_f32.sqrt() * 0.5));
        assert_eq!(c.bounding_sphere(),
                   Sphere::new(&Vector3::ZERO, 12.0_f32.sqrt() * 0.5));
    }

    #[test]
    fn intersect() {
        let a = Box3::new(&Vector3::ZERO, &Vector3::ZERO);
        let b = Box3::new(&Vector3::ZERO, &Vector3::ONE);
        let c = Box3::new(&Vector3::ONE.negate(), &Vector3::ONE);

        assert_eq!(a.intersect_box(&a), Some(a));
        assert_eq!(a.intersect_box(&b), Some(a));
        assert_eq!(b.intersect_box(&b), Some(b));
        assert_eq!(a.intersect_box(&c), Some(a));
        assert_eq!(b.intersect_box(&c), Some(b));
        assert_eq!(c.intersect_box(&c), Some(c));
    }

    #[test]
    fn union() {
        let a = Box3::new(&Vector3::ZERO, &Vector3::ZERO);
        let b = Box3::new(&Vector3::ZERO, &Vector3::ONE);
        let c = Box3::new(&Vector3::ONE.negate(), &Vector3::ONE);

        assert_eq!(a.union(&a), a);
        assert_eq!(a.union(&b), b);
        assert_eq!(a.union(&c), c);
        assert_eq!(b.union(&c), c);
    }

    #[test]
    fn apply_matrix4() {
        let a = Box3::new(&Vector3::ZERO, &Vector3::ZERO);
        let b = Box3::new(&Vector3::ZERO, &Vector3::ONE);
        let c = Box3::new(&Vector3::ONE.negate(), &Vector3::ONE);
        let d = Box3::new(&Vector3::ONE.negate(), &Vector3::ZERO);

        let m = Matrix4::from_translation(&Vector3::new(1.0, -2.0, 1.0));
        let t1 = Vector3::new(1.0, -2.0, 1.0);

        let compare_boxes = |a: &Box3, b: &Box3| -> bool {
            let threshold = 0.0001;
            a.min.distance_to(&b.min) < threshold && a.max.distance_to(&b.max) < threshold
        };

        assert_eq!(compare_boxes(&a.apply_matrix4(&m), &a.translate(&t1)), true);
        assert_eq!(compare_boxes(&b.apply_matrix4(&m), &b.translate(&t1)), true);
        assert_eq!(compare_boxes(&c.apply_matrix4(&m), &c.translate(&t1)), true);
        assert_eq!(compare_boxes(&d.apply_matrix4(&m), &d.translate(&t1)), true);
    }

    #[test]
    fn translate() {
        let a = Box3::new(&Vector3::ZERO, &Vector3::ZERO);
        let b = Box3::new(&Vector3::ZERO, &Vector3::ONE);
        let d = Box3::new(&Vector3::ONE.negate(), &Vector3::ZERO);

        assert_eq!(a.translate(&Vector3::ONE),
                   Box3::new(&Vector3::ONE, &Vector3::ONE));
        assert_eq!(a.translate(&Vector3::ONE).translate(&Vector3::ONE.negate()),
                   a);
        assert_eq!(d.translate(&Vector3::ONE), b);
        assert_eq!(b.translate(&Vector3::ONE.negate()), d);
    }
}