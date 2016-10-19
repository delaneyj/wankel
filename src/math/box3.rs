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

    pub fn from_points(points: &[Vector3]) -> Box3 {
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

    // 				var geometry = node.geometry;

    // 				if ( geometry !== undefined ) {

    // 					if ( (geometry && geometry.isGeometry) ) {

    // 						var vertices = geometry.vertices;

    // 						for ( var i = 0, il = vertices.length; i < il; i ++ ) {

    // 							v1.copy( vertices[ i ] );
    // 							v1.applyMatrix4( node.matrixWorld );

    // 							scope.expandByPoint( v1 );

    // 						}

    // 					} else if ( (geometry && geometry.isBufferGeometry) ) {

    // 						var attribute = geometry.attributes.position;

    // 						if ( attribute !== undefined ) {

    // 							var array, offset, stride;

    // 							if ( (attribute && attribute.isInterleavedBufferAttribute) ) {

    // 								array = attribute.data.array;
    // 								offset = attribute.offset;
    // 								stride = attribute.data.stride;

    // 							} else {

    // 								array = attribute.array;
    // 								offset = 0;
    // 								stride = 3;

    // 							}

    // 							for ( var i = offset, il = array.length; i < il; i += stride ) {

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
        Sphere::new(self.center(), self.size().length() * 0.5)
    }

    pub fn intersect_box(&self, box3: &Box3) -> Box3 {
        let min = self.min.max(&box3.min);
        let max = self.max.min(&box3.max);
        let intersect = Box3::new(&min, &max);

        // ensure that if there is no overlap, the result is fully empty, not slightly empty with non-inf/+inf values that will cause subsequence intersects to erroneously return valid values.
        if intersect.is_empty() {
            Box3::EMPTY
        } else {
            *self
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
            let points = vec![Vector3::new(self.min.x, self.min.y, self.min.z).apply_matrix4(m), /* 000 */
                              Vector3::new(self.min.x, self.min.y, self.max.z).apply_matrix4(m), /* 001 */
                              Vector3::new(self.min.x, self.max.y, self.min.z).apply_matrix4(m), /* 010 */
                              Vector3::new(self.min.x, self.max.y, self.max.z).apply_matrix4(m), /* 011 */
                              Vector3::new(self.max.x, self.min.y, self.min.z).apply_matrix4(m), /* 100 */
                              Vector3::new(self.max.x, self.min.y, self.max.z).apply_matrix4(m), /* 101 */
                              Vector3::new(self.max.x, self.max.y, self.min.z).apply_matrix4(m), /* 110 */
                              Vector3::new(self.max.x, self.max.y, self.max.z).apply_matrix4(m) /* 111 */];

            Box3::from_points(&points)
        }
    }

    pub fn translate(&self, offset: &Vector3) -> Box3 {
        Box3::new(&self.min.add(offset), &self.max.add(offset))
    }
}