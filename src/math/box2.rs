use std::f32::{INFINITY, NEG_INFINITY};

use math::*;
// use core::Object3D;

#[derive(Debug,PartialEq,Copy,Clone)]
pub struct Box2 {
    pub min: Vector2,
    pub max: Vector2,
}

impl Box2 {
    pub const EMPTY: Box2 = Box2 {
        min: Vector2 {
            x: INFINITY,
            y: INFINITY,
        },
        max: Vector2 {
            x: NEG_INFINITY,
            y: NEG_INFINITY,
        },
    };

    pub fn new(min: &Vector2, max: &Vector2) -> Box2 {
        Box2 {
            min: *min,
            max: *max,
        }
    }

    pub fn from_points(points: &[&Vector2]) -> Box2 {
        points.into_iter().fold(Box2::EMPTY, |acc, p| acc.expand_by_point(p))
    }

    pub fn from_center_and_size(center: &Vector2, size: &Vector2) -> Box2 {
        let half_size = size.multiply_scalar(0.5);
        Box2::new(&center.subtract(&half_size), &center.add(&half_size))
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
        self.max.x < self.min.x || self.max.y < self.min.y
    }

    pub fn center(&self) -> Vector2 {
        if self.is_empty() {
            Vector2::ZERO
        } else {
            self.min.add(&self.max).multiply_scalar(0.5)
        }
    }

    pub fn size(&self) -> Vector2 {
        if self.is_empty() {
            Vector2::ZERO
        } else {
            self.max.subtract(&self.min)
        }
    }

    pub fn expand_by_point(&self, point: &Vector2) -> Box2 {
        Box2::new(&self.min.min(point), &self.max.max(point))
    }

    pub fn expand_by_vector(&self, vector: &Vector2) -> Box2 {
        Box2::new(&self.min.subtract(vector), &self.max.add(vector))
    }

    pub fn expand_by_scalar(&self, s: f32) -> Box2 {
        Box2::new(&self.min.add_scalar(-s), &self.max.add_scalar(s))
    }

    pub fn contains_point(&self, point: &Vector2) -> bool {
        !(point.x < self.min.x || point.x > self.max.x || point.y < self.min.y ||
          point.y > self.max.y)
    }

    pub fn contains_box(&self, box2: &Box2) -> bool {
        self.min.x <= box2.min.x && box2.max.x <= self.max.x && self.min.y <= box2.min.y &&
        box2.max.y <= self.max.y
    }

    pub fn intersects_box(&self, box2: &Box2) -> bool {
        // using 6 splitting planes to rule out intersections.
        !(box2.max.x < self.min.x || box2.min.x > self.max.x || box2.max.y < self.min.y ||
          box2.min.y > self.max.y)
    }

    pub fn clamp_point(&self, point: &Vector2) -> Vector2 {
        point.clamp(&self.min, &self.max)
    }

    pub fn distance_to_point(&self, point: &Vector2) -> f32 {
        point.clamp(&self.min, &self.max).subtract(point).length()
    }

    pub fn intersect_box(&self, box2: &Box2) -> Option<Box2> {
        let min = self.min.max(&box2.min);
        let max = self.max.min(&box2.max);
        let intersect = Box2::new(&min, &max);

        // ensure that if there is no overlap, the result is fully empty, not slightly empty with non-inf/+inf values that will cause subsequence intersects to erroneously return valid values.
        if intersect.is_empty() {
            None
        } else {
            Some(intersect)
        }
    }

    pub fn union(&self, box2: &Box2) -> Box2 {
        Box2::new(&self.min.min(&box2.min), &self.max.max(&box2.max))
    }

    pub fn translate(&self, offset: &Vector2) -> Box2 {
        Box2::new(&self.min.add(offset), &self.max.add(offset))
    }
}

#[cfg(test)]
mod tests {
    use std::f32::consts::SQRT_2;
    use math::*;

    #[test]
    fn constructor() {
        let a = Box2::EMPTY;
        assert_eq!(a.min, Vector2::INFINITY);
        assert_eq!(a.max, Vector2::NEG_INFINITY);

        let b = Box2::new(&Vector2::ZERO, &Vector2::ZERO);
        assert_eq!(b.min, Vector2::ZERO);
        assert_eq!(b.max, Vector2::ZERO);

        let c = Box2::new(&Vector2::ZERO, &Vector2::ONE);
        assert_eq!(c.min, Vector2::ZERO);
        assert_eq!(c.max, Vector2::ONE);
    }

    #[test]
    fn from_points() {
        let twos = Vector2::new(2.0, 2.0);
        let a = Box2::from_points(vec![&Vector2::ZERO, &Vector2::ONE, &twos].as_slice());
        assert_eq!(a.min, Vector2::ZERO);
        assert_eq!(a.max, twos);

        let b = Box2::from_points(&vec![&Vector2::ONE]);
        assert_eq!(b.min, Vector2::ONE);
        assert_eq!(b.max, Vector2::ONE);

        let c = Box2::from_points(&[]);
        assert_eq!(c.is_empty(), true);
    }

    #[test]
    fn empty_make_empty() {
        let a = Box2::EMPTY;

        assert_eq!(a.is_empty(), true);

        let b = Box2::new(&Vector2::ZERO, &Vector2::ONE);
        assert_eq!(b.is_empty(), false);
    }

    #[test]
    fn center() {
        let a = Box2::new(&Vector2::ZERO, &Vector2::ZERO);
        assert_eq!(a.center(), Vector2::ZERO);

        let b = Box2::new(&Vector2::ZERO, &Vector2::ONE);
        let midpoint = Vector2::ONE.multiply_scalar(0.5);
        assert_eq!(b.center(), midpoint);
    }

    #[test]
    fn size() {
        let a = Box2::new(&Vector2::ZERO, &Vector2::ZERO);
        assert_eq!(a.size(), Vector2::ZERO);

        let b = Box2::new(&Vector2::ZERO, &Vector2::ONE);
        assert_eq!(b.size(), Vector2::ONE);
    }


    #[test]
    fn expand_by_point() {
        let a = Box2::new(&Vector2::ZERO, &Vector2::ZERO);

        let b = a.expand_by_point(&Vector2::ZERO);
        assert_eq!(b.size(), Vector2::ZERO);

        let c = b.expand_by_point(&Vector2::ONE);
        assert_eq!(c.size(), Vector2::ONE);

        let d = c.expand_by_point(&Vector2::ONE.negate());
        assert_eq!(d.size(), Vector2::ONE.multiply_scalar(2.0));
        assert_eq!(d.center(), Vector2::ZERO);
    }

    #[test]
    fn expand_by_vector() {
        let a = Box2::new(&Vector2::ZERO, &Vector2::ZERO);

        let b = a.expand_by_vector(&Vector2::ZERO);
        assert_eq!(b.size(), Vector2::ZERO);

        let c = b.expand_by_vector(&Vector2::ONE);
        assert_eq!(c.size(), Vector2::ONE.multiply_scalar(2.0));
        assert_eq!(c.center(), Vector2::ZERO);
    }

    #[test]
    fn expand_by_scalar() {
        let a = Box2::new(&Vector2::ZERO, &Vector2::ZERO);

        let b = a.expand_by_scalar(0.0);
        assert_eq!(b.size(), Vector2::ZERO);

        let c = b.expand_by_scalar(1.0);
        assert_eq!(c.size(), Vector2::ONE.multiply_scalar(2.0));
        assert_eq!(c.center(), Vector2::ZERO);
    }

    #[test]
    fn contains_point() {
        let a = Box2::new(&Vector2::ZERO, &Vector2::ZERO);

        assert_eq!(a.contains_point(&Vector2::ZERO), true);
        assert_eq!(a.contains_point(&Vector2::ONE), false);

        let b = a.expand_by_scalar(1.0);
        assert_eq!(b.contains_point(&Vector2::ZERO), true);
        assert_eq!(b.contains_point(&Vector2::ONE), true);
        assert_eq!(b.contains_point(&Vector2::ONE.negate()), true);
    }

    #[test]
    fn contains_box() {
        let a = Box2::new(&Vector2::ZERO, &Vector2::ZERO);
        let b = Box2::new(&Vector2::ZERO, &Vector2::ONE);
        let c = Box2::new(&Vector2::ONE.negate(), &Vector2::ONE);

        assert_eq!(a.contains_box(&a), true);
        assert_eq!(a.contains_box(&b), false);
        assert_eq!(a.contains_box(&c), false);
        assert_eq!(b.contains_box(&a), true);
        assert_eq!(c.contains_box(&a), true);
        assert_eq!(b.contains_box(&c), false);
    }

    #[test]
    fn clamp_point() {
        let a = Box2::new(&Vector2::ZERO, &Vector2::ZERO);
        let b = Box2::new(&Vector2::ONE.negate(), &Vector2::ONE);

        assert_eq!(a.clamp_point(&Vector2::new(0.0, 0.0)),
                   Vector2::new(0.0, 0.0));
        assert_eq!(a.clamp_point(&Vector2::new(1.0, 1.0)),
                   Vector2::new(0.0, 0.0));
        assert_eq!(a.clamp_point(&Vector2::new(-1.0, -1.0)),
                   Vector2::new(0.0, 0.0));
        assert_eq!(b.clamp_point(&Vector2::new(2.0, 2.0)),
                   Vector2::new(1.0, 1.0));
        assert_eq!(b.clamp_point(&Vector2::new(1.0, 1.0)),
                   Vector2::new(1.0, 1.0));
        assert_eq!(b.clamp_point(&Vector2::new(0.0, 0.0)),
                   Vector2::new(0.0, 0.0));
        assert_eq!(b.clamp_point(&Vector2::new(-1.0, -1.0)),
                   Vector2::new(-1.0, -1.0));
        assert_eq!(b.clamp_point(&Vector2::new(-2.0, -2.0)),
                   Vector2::new(-1.0, -1.0));
    }

    #[test]
    fn distance_to_point() {
        let a = Box2::new(&Vector2::ZERO, &Vector2::ZERO);
        let b = Box2::new(&Vector2::ONE.negate(), &Vector2::ONE);

        assert_eq!(a.distance_to_point(&Vector2::new(0.0, 0.0)), 0.0);
        assert_eq!(a.distance_to_point(&Vector2::new(1.0, 1.0)), SQRT_2);
        assert_eq!(a.distance_to_point(&Vector2::new(-1.0, -1.0)), SQRT_2);
        assert_eq!(b.distance_to_point(&Vector2::new(2.0, 2.0)), SQRT_2);
        assert_eq!(b.distance_to_point(&Vector2::new(1.0, 1.0)), 0.0);
        assert_eq!(b.distance_to_point(&Vector2::new(0.0, 0.0)), 0.0);
        assert_eq!(b.distance_to_point(&Vector2::new(-1.0, -1.0)), 0.0);
        assert_eq!(b.distance_to_point(&Vector2::new(-2.0, -2.0)), SQRT_2);
    }

    #[test]
    fn intersects_box() {
        let a = Box2::new(&Vector2::ZERO, &Vector2::ZERO);
        let b = Box2::new(&Vector2::ZERO, &Vector2::ONE);
        let c = Box2::new(&Vector2::ONE.negate(), &Vector2::ONE);

        assert!(a.intersect_box(&a) != None);
        assert!(a.intersect_box(&b) != None);
        assert!(a.intersect_box(&c) != None);
        assert!(b.intersect_box(&a) != None);
        assert!(c.intersect_box(&a) != None);
        assert!(b.intersect_box(&c) != None);

        let d = b.translate(&Vector2::new(2.0, 2.0));
        assert_eq!(a.intersect_box(&d), None);
        assert_eq!(d.intersect_box(&a), None);
        assert_eq!(d.intersect_box(&c), None);
    }

    #[test]
    fn intersect() {
        let a = Box2::new(&Vector2::ZERO, &Vector2::ZERO);
        let b = Box2::new(&Vector2::ZERO, &Vector2::ONE);
        let c = Box2::new(&Vector2::ONE.negate(), &Vector2::ONE);

        assert_eq!(a.intersect_box(&a), Some(a));
        assert_eq!(a.intersect_box(&b), Some(a));
        assert_eq!(b.intersect_box(&b), Some(b));
        assert_eq!(a.intersect_box(&c), Some(a));
        assert_eq!(b.intersect_box(&c), Some(b));
        assert_eq!(c.intersect_box(&c), Some(c));
    }

    #[test]
    fn union() {
        let a = Box2::new(&Vector2::ZERO, &Vector2::ZERO);
        let b = Box2::new(&Vector2::ZERO, &Vector2::ONE);
        let c = Box2::new(&Vector2::ONE.negate(), &Vector2::ONE);

        assert_eq!(a.union(&a), a);
        assert_eq!(a.union(&b), b);
        assert_eq!(a.union(&c), c);
        assert_eq!(b.union(&c), c);
    }

    #[test]
    fn translate() {
        let a = Box2::new(&Vector2::ZERO, &Vector2::ZERO);
        let b = Box2::new(&Vector2::ZERO, &Vector2::ONE);
        let d = Box2::new(&Vector2::ONE.negate(), &Vector2::ZERO);

        assert_eq!(a.translate(&Vector2::ONE),
                   Box2::new(&Vector2::ONE, &Vector2::ONE));
        assert_eq!(a.translate(&Vector2::ONE).translate(&Vector2::ONE.negate()),
                   a);
        assert_eq!(d.translate(&Vector2::ONE), b);
        assert_eq!(b.translate(&Vector2::ONE.negate()), d);
    }

}