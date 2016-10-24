use std::f32::EPSILON;

use math::Euler;
use math::EulerOrder;
use math::Matrix4;
use math::Vector3;

#[derive(Debug,PartialEq,Clone)]
pub struct Quaternion {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Quaternion {
    pub const DEFAULT: Quaternion = Quaternion {
        x: 0.0,
        y: 0.0,
        z: 0.0,
        w: 1.0,
    };

    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Quaternion {
        Quaternion {
            x: x,
            y: y,
            z: z,
            w: w,
        }
    }

    pub fn from_euler(euler: &Euler) -> Quaternion {
        // http://www.mathworks.com/matlabcentral/fileexchange/
        // 	20696-function-to-convert-between-dcm-euler-angles-quaternions-and-euler-vectors/
        // 	content/SpinCalc.m
        let two = 2.0;
        let c1 = (euler.x / two).cos();
        let c2 = (euler.y / two).cos();
        let c3 = (euler.z / two).cos();
        let s1 = (euler.x / two).sin();
        let s2 = (euler.y / two).sin();
        let s3 = (euler.z / two).sin();

        match euler.order {
            EulerOrder::XYZ => {
                Quaternion {
                    x: s1 * c2 * c3 + c1 * s2 * s3,
                    y: c1 * s2 * c3 - s1 * c2 * s3,
                    z: c1 * c2 * s3 + s1 * s2 * c3,
                    w: c1 * c2 * c3 - s1 * s2 * s3,
                }
            }
            EulerOrder::YXZ => {
                Quaternion {
                    x: s1 * c2 * c3 + c1 * s2 * s3,
                    y: c1 * s2 * c3 - s1 * c2 * s3,
                    z: c1 * c2 * s3 - s1 * s2 * c3,
                    w: c1 * c2 * c3 + s1 * s2 * s3,
                }
            }
            EulerOrder::ZXY => {
                Quaternion {
                    x: s1 * c2 * c3 - c1 * s2 * s3,
                    y: c1 * s2 * c3 + s1 * c2 * s3,
                    z: c1 * c2 * s3 + s1 * s2 * c3,
                    w: c1 * c2 * c3 - s1 * s2 * s3,
                }
            }
            EulerOrder::ZYX => {
                Quaternion {
                    x: s1 * c2 * c3 - c1 * s2 * s3,
                    y: c1 * s2 * c3 + s1 * c2 * s3,
                    z: c1 * c2 * s3 - s1 * s2 * c3,
                    w: c1 * c2 * c3 + s1 * s2 * s3,
                }
            }
            EulerOrder::YZX => {
                Quaternion {
                    x: s1 * c2 * c3 + c1 * s2 * s3,
                    y: c1 * s2 * c3 + s1 * c2 * s3,
                    z: c1 * c2 * s3 - s1 * s2 * c3,
                    w: c1 * c2 * c3 - s1 * s2 * s3,
                }
            }
            EulerOrder::XZY => {
                Quaternion {
                    x: s1 * c2 * c3 - c1 * s2 * s3,
                    y: c1 * s2 * c3 - s1 * c2 * s3,
                    z: c1 * c2 * s3 + s1 * s2 * c3,
                    w: c1 * c2 * c3 + s1 * s2 * s3,
                }
            }
        }
    }

    pub fn from_axis_angle(axis: &Vector3, angle: f32) -> Quaternion {
        // http://www.euclideanspace.com/maths/geometry/rotations/conversions/angleToQuaternion/index.htm
        // assumes axis is normalized
        let half_angle = angle / 2.0;
        let s = half_angle.sin();
        Quaternion::new(axis.x * s, axis.y * s, axis.z * s, half_angle.cos())
    }

    pub fn from_rotation_matrix(m: &Matrix4) -> Quaternion {

        // http://www.euclideanspace.com/maths/geometry/rotations/conversions/matrixToQuaternion/index.htm

        // assumes the upper 3x3 of m is a pure rotation matrix (i.e, unscaled)
        let m11 = m.elements[0];
        let m12 = m.elements[4];
        let m13 = m.elements[8];
        let m21 = m.elements[1];
        let m22 = m.elements[5];
        let m23 = m.elements[9];
        let m31 = m.elements[2];
        let m32 = m.elements[6];
        let m33 = m.elements[10];
        let trace = m11 + m22 + m33;

        if trace > 0.0 {
            let s = 0.5 / (trace + 1.0).sqrt();
            Quaternion {
                w: 0.25 / s,
                x: (m32 - m23) * s,
                y: (m13 - m31) * s,
                z: (m21 - m12) * s,
            }

        } else if m11 > m22 && m11 > m33 {
            let s = 2.0 * (1.0 + m11 - m22 - m33).sqrt();
            Quaternion {
                w: (m32 - m23) / s,
                x: 0.25 * s,
                y: (m12 + m21) / s,
                z: (m13 + m31) / s,
            }
        } else if m22 > m33 {
            let s = 2.0 * (1.0 + m22 - m11 - m33).sqrt();
            Quaternion {
                w: (m13 - m31) / s,
                x: (m12 + m21) / s,
                y: 0.25 * s,
                z: (m23 + m32) / s,
            }
        } else {
            let s = 2.0 * (1.0 + m33 - m11 - m22).sqrt();
            Quaternion {
                w: (m21 - m12) / s,
                x: (m13 + m31) / s,
                y: (m23 + m32) / s,
                z: 0.25 * s,
            }
        }
    }

    pub fn from_unit_vectors(from: &Vector3, to: &Vector3) -> Quaternion {
        // http://lolengine.net/blog/2014/02/24/quaternion-from-two-vectors-final
        // assumes direction vectors vFrom and vTo are normalized
        let mut x = 0.0;
        let y: f32;
        let mut z = 0.0;
        let mut r = from.dot(to) + 1.0;

        if r < EPSILON {
            r = 0.0;

            if from.x.abs() > from.z.abs() {
                x = -from.y;
                y = from.x;
            } else {
                y = -from.z;
                z = from.y;
            }
        } else {
            let v = from.cross(to);
            x = v.x;
            y = v.y;
            z = v.z;
        }

        Quaternion::new(x, y, z, r).normalized()
    }

    pub fn inverse(&self) -> Quaternion {
        self.conjugate().normalized()
    }

    pub fn conjugate(&self) -> Quaternion {
        Quaternion::new(-self.x, -self.y, -self.z, self.w)
    }

    pub fn dot(&self, q: &Quaternion) -> f32 {
        (self.x * q.x) + (self.y * q.y) + (self.z * q.z) + (self.w * q.w)

    }

    pub fn length_squared(&self) -> f32 {
        (self.x * self.x) + (self.y * self.y) + (self.z * self.z) + (self.w * self.w)
    }

    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn normalized(&self) -> Quaternion {
        match self.length() {
            0.0 => Quaternion::DEFAULT,
            length => {
                let l = 1.0 / length;
                Quaternion {
                    x: self.x * l,
                    y: self.y * l,
                    z: self.z * l,
                    w: self.w * l,
                }
            }
        }
    }

    pub fn multiply(&self, b: &Quaternion) -> Quaternion {
        // from http://www.euclideanspace.com/maths/algebra/realNormedAlgebra/quaternions/code/index.htm
        let &Quaternion { x: qax, y: qay, z: qaz, w: qaw } = self;
        let &Quaternion { x: qbx, y: qby, z: qbz, w: qbw } = b;

        Quaternion {
            x: qax * qbw + qaw * qbx + qay * qbz - qaz * qby,
            y: qay * qbw + qaw * qby + qaz * qbx - qax * qbz,
            z: qaz * qbw + qaw * qbz + qax * qby - qay * qbx,
            w: qaw * qbw - qax * qbx - qay * qby - qaz * qbz,
        }
    }

    pub fn premultiply(&self, q: &Quaternion) -> Quaternion {
        q.multiply(self)
    }

    pub fn slerp(&self, qb: &Quaternion, t: f32) -> Quaternion {
        match t {
            0.0 => self.clone(),
            1.0 => qb.clone(),
            _ => {
                let mut x = self.x;
                let mut y = self.y;
                let mut z = self.z;
                let mut w = self.w;

                // http://www.euclideanspace.com/maths/algebra/realNormedAlgebra/quaternions/slerp/
                let mut cos_half_theta = w * qb.w + x * qb.x + y * qb.y + z * qb.z;

                if cos_half_theta < 0.0 {
                    w = -qb.w;
                    x = -qb.x;
                    y = -qb.y;
                    z = -qb.z;
                    cos_half_theta *= -1.0;
                } else {
                    w = qb.w;
                    x = qb.x;
                    y = qb.y;
                    z = qb.z;
                }

                if cos_half_theta >= 1.0 {
                    return Quaternion::new(x, y, z, w);
                }

                let sin_half_theta = (1.0 - cos_half_theta * cos_half_theta).sqrt();

                if sin_half_theta.abs() < 0.001 {
                    return Quaternion {
                        w: 0.5 * (w + self.w),
                        x: 0.5 * (x + self.x),
                        y: 0.5 * (y + self.y),
                        z: 0.5 * (z + self.z),
                    };
                }

                let half_theta = sin_half_theta.atan2(cos_half_theta);
                let ratio_a = ((1.0 - t) * half_theta).sin() / sin_half_theta;
                let ratio_b = (t * half_theta).sin() / sin_half_theta;

                Quaternion {
                    w: (w * ratio_a + self.w * ratio_b),
                    x: (x * ratio_a + self.x * ratio_b),
                    y: (y * ratio_a + self.y * ratio_b),
                    z: (z * ratio_a + self.z * ratio_b),
                }
            }
        }
    }

    // pub fn from_array: function ( array, offset ) {

    // 	if ( offset === undefined ) offset = 0;

    // 	self.x = array[ offset ];
    // 	self.y = array[ offset + 1 ];
    // 	self.z = array[ offset + 2 ];
    // 	self.w = array[ offset + 3 ];

    // 	this.onChangeCallback();

    // 	return this;

    // },

    // toArray: function ( array, offset ) {

    // 	if ( array === undefined ) array = [];
    // 	if ( offset === undefined ) offset = 0;

    // 	array[ offset ] = self.x;
    // 	array[ offset + 1 ] = self.y;
    // 	array[ offset + 2 ] = self.z;
    // 	array[ offset + 3 ] = self.w;

    // 	return array;

    // }

    // pub fn slerpFlat(dst:[, dstOffset, src0, srcOffset0, src1, srcOffset1, t ) {

    // 	// fuzz-free, array-based Quaternion SLERP operation

    // 	var x0 = src0[ srcOffset0 + 0 ],
    // 		y0 = src0[ srcOffset0 + 1 ],
    // 		z0 = src0[ srcOffset0 + 2 ],
    // 		w0 = src0[ srcOffset0 + 3 ],

    // 		x1 = src1[ srcOffset1 + 0 ],
    // 		y1 = src1[ srcOffset1 + 1 ],
    // 		z1 = src1[ srcOffset1 + 2 ],
    // 		w1 = src1[ srcOffset1 + 3 ];

    // 	if ( w0 !== w1 || x0 !== x1 || y0 !== y1 || z0 !== z1 ) {

    // 		var s = 1 - t,

    // 			cos = x0 * x1 + y0 * y1 + z0 * z1 + w0 * w1,

    // 			dir = ( cos >= 0 ? 1 : - 1 ),
    // 			sqrSin = 1 - cos * cos;

    // 		// Skip the Slerp for tiny steps to avoid numeric problems:
    // 		if ( sqrSin > Number.EPSILON ) {

    // 			var sin = Math.sqrt( sqrSin ),
    // 				len = Math.atan2( sin, cos * dir );

    // 			s = Math.sin( s * len ) / sin;
    // 			t = Math.sin( t * len ) / sin;

    // 		}

    // 		var tDir = t * dir;

    // 		x0 = x0 * s + x1 * tDir;
    // 		y0 = y0 * s + y1 * tDir;
    // 		z0 = z0 * s + z1 * tDir;
    // 		w0 = w0 * s + w1 * tDir;

    // 		// Normalize in case we just did a lerp:
    // 		if ( s === 1 - t ) {

    // 			var f = 1 / Math.sqrt( x0 * x0 + y0 * y0 + z0 * z0 + w0 * w0 );

    // 			x0 *= f;
    // 			y0 *= f;
    // 			z0 *= f;
    // 			w0 *= f;

    // 		}

    // 	}

    // 	dst[ dstOffset ] = x0;
    // 	dst[ dstOffset + 1 ] = y0;
    // 	dst[ dstOffset + 2 ] = z0;
    // 	dst[ dstOffset + 3 ] = w0;

    // }
}

#[cfg(test)]
mod tests {
    use super::Quaternion;
    use math::{Euler, EulerOrder, Vector3, Matrix4};
    use std::f32::consts::PI;

    const EULER_ANGLES: Euler = Euler {
        x: 0.1,
        y: -0.3,
        z: 0.25,
        order: EulerOrder::XYZ,
    };

    fn quaternions_subtract(a: &Quaternion, b: &Quaternion) -> Quaternion {
        Quaternion::new(a.x - b.x, a.y - b.y, a.z - b.z, a.w - b.w)
    }

    #[test]
    fn constructor() {
        let a = Quaternion::DEFAULT;
        assert_eq!(a.x, 0.0);
        assert_eq!(a.y, 0.0);
        assert_eq!(a.z, 0.0);
        assert_eq!(a.w, 1.0);

        let b = Quaternion::new(1.0, 2.0, 4.0, 8.0);
        assert_eq!(b.x, 1.0);
        assert_eq!(b.y, 2.0);
        assert_eq!(b.z, 4.0);
        assert_eq!(b.w, 8.0);
    }

    #[test]
    fn from_axis_angle() {

        let zero = Quaternion::DEFAULT;

        let a = Quaternion::from_axis_angle(&Vector3::new(1.0, 0.0, 0.0), 0.0);
        assert_eq!(a, zero);
        let b = Quaternion::from_axis_angle(&Vector3::new(0.0, 1.0, 0.0), 0.0);
        assert_eq!(b, zero);
        let c = Quaternion::from_axis_angle(&Vector3::new(0.0, 0.0, 1.0), 0.0);
        assert_eq!(c, zero);

        let b1 = Quaternion::from_axis_angle(&Vector3::new(1.0, 0.0, 0.0), PI);
        assert_eq!(a == b1, false);
        let b2 = Quaternion::from_axis_angle(&Vector3::new(1.0, 0.0, 0.0), -PI);
        assert_eq!(a, b2);

        let b3 = b1.multiply(&b2);
        assert_eq!(a, b3);
    }

    #[test]
    fn from_euler_from_quaternion() {
        let angles = vec![Vector3::new(1.0, 0.0, 0.0),
                          Vector3::new(0.0, 1.0, 0.0),
                          Vector3::new(0.0, 0.0, 1.0)];

        // ensure euler conversion to/from Quaternion matches.
        let orders = vec![EulerOrder::XYZ,
                          EulerOrder::YXZ,
                          EulerOrder::ZXY,
                          EulerOrder::ZYX,
                          EulerOrder::YZX,
                          EulerOrder::XZY];

        for order in orders {
            for angle in &angles {
                let e = Euler::from_vector3(&angle, &order);
                let q = Quaternion::from_euler(&e);
                let euler2 = Euler::from_quaternion(&q, &order);

                let new_angle = Vector3::from_euler(&euler2);
                assert_eq!(new_angle.distance_to(&angle) < 0.001, true);
            }
        }
    }

    #[test]
    fn from_euler_from_rotation_matrix() {
        // ensure euler conversion for Quaternion matches that of Matrix4
        let orders = vec![EulerOrder::XYZ,
                          EulerOrder::YXZ,
                          EulerOrder::ZXY,
                          EulerOrder::ZYX,
                          EulerOrder::YZX,
                          EulerOrder::XZY];

        for order in orders {
            let e = Euler { order: order, ..EULER_ANGLES };
            let q = Quaternion::from_euler(&e);
            let m = Matrix4::rotation_from_euler(&EULER_ANGLES);
            let q2 = Quaternion::from_rotation_matrix(&m);

            let result = quaternions_subtract(&q, &q2).length();
            assert_eq!(result < 0.001, true);
        }
    }

    #[test]
    fn normalize_length_length_squared() {
        let a = Quaternion::new(2.0, 3.0, 4.0, 5.0);
        assert!(a.length() != 1.0);
        assert!(a.length_squared() != 1.0);

        let b = a.normalized();
        assert!(b.length() == 1.0);
        assert!(b.length_squared() == 1.0);

        let c = Quaternion::new(0.0, 0.0, 0.0, 0.0);
        assert!(c.length_squared() == 0.0);
        assert!(c.length() == 0.0);

        let d = c.normalized();
        assert!(d.length_squared() == 1.0);
        assert!(d.length() == 1.0);
    }

    #[test]
    fn inverse_conjugate() {
        let a = Quaternion::new(2.0, 3.0, 4.0, 5.0);
        let b = a.conjugate();
        assert_eq!(a.x, -b.x);
        assert_eq!(a.y, -b.y);
        assert_eq!(a.z, -b.z);
        assert_eq!(a.w, b.w);
    }

    #[test]
    fn multiply() {
        let angles = vec![Vector3::new(1.0, 0.0, 0.0),
                          Vector3::new(0.0, 1.0, 0.0),
                          Vector3::new(0.0, 0.0, 1.0)];

        let eulers: Vec<Euler> =
            angles.iter().map(|v| Euler::from_vector3(&v, &EulerOrder::XYZ)).collect();
        let ref e1 = eulers[0];
        let ref e2 = eulers[1];
        let ref e3 = eulers[2];

        let m1 = Matrix4::rotation_from_euler(&e1);
        let m2 = Matrix4::rotation_from_euler(&e2);
        let m3 = Matrix4::rotation_from_euler(&e3);
        let m = m1.multiply(&m2).multiply(&m3);
        let q_from_m = Quaternion::from_rotation_matrix(&m);

        let quaternions: Vec<Quaternion> =
            eulers.iter().map(|e| Quaternion::from_euler(&e)).collect();
        let ref q1 = quaternions[0];
        let ref q2 = quaternions[1];
        let ref q3 = quaternions[2];
        let q = q1.multiply(&q2).multiply(&q3);

        let result = quaternions_subtract(&q, &q_from_m).length();
        assert!(result < 0.001);
    }

    #[test]
    fn multiply_vector3() {
        let angles = vec![Vector3::new(1.0, 0.0, 0.0),
                          Vector3::new(0.0, 1.0, 0.0),
                          Vector3::new(0.0, 0.0, 1.0)];

        let orders = vec![EulerOrder::XYZ,
                          EulerOrder::YXZ,
                          EulerOrder::ZXY,
                          EulerOrder::ZYX,
                          EulerOrder::YZX,
                          EulerOrder::XZY];


        // ensure euler conversion for Quaternion matches that of Matrix4
        for angle in &angles {
            for order in &orders {
                let e = Euler::from_vector3(&angle, &order);
                let q = Quaternion::from_euler(&e);
                let m = Matrix4::rotation_from_euler(&e);

                let v0 = Vector3::new(1.0, 0.0, 0.0);
                let qv = v0.apply_quaternion(&q);
                let mv = v0.apply_matrix4(&m);

                assert!(qv.distance_to(&mv) < 0.001);
            }
        }
    }


    #[test]
    fn equals() {
        let x = 2.0;
        let y = 3.0;
        let z = 4.0;
        let w = 5.0;
        let a = Quaternion::new(x, y, z, w);
        let b = Quaternion::new(-x, -y, -z, -w);

        assert!(a.x != b.x);
        assert!(a.y != b.y);

        assert!(a != b);
        assert!(b != a);
    }

    // function doSlerpObject( aArr, bArr, t ) {
    //
    // var a = new THREE.Quaternion().fromArray( aArr ),
    // b = new THREE.Quaternion().fromArray( bArr ),
    // c = new THREE.Quaternion().fromArray( aArr );
    //
    // c.slerp( b, t );
    //
    // return {
    //
    // equals: function( x, y, z, w, maxError ) {
    //
    // if ( maxError === undefined ) maxError = Number.EPSILON;
    //
    // return 	Math.abs( x - c.x ) <= maxError &&
    // Math.abs( y - c.y ) <= maxError &&
    // Math.abs( z - c.z ) <= maxError &&
    // Math.abs( w - c.w ) <= maxError;
    //
    // },
    //
    // length: c.length(),
    //
    // dotA: c.dot( a ),
    // dotB: c.dot( b )
    //
    // };
    //
    // };
    //
    // function doSlerpArray( a, b, t ) {
    //
    // var result = [ 0, 0, 0, 0 ];
    //
    // THREE.Quaternion.slerpFlat( result, 0, a, 0, b, 0, t );
    //
    // function arrDot( a, b ) {
    //
    // return 	a[ 0 ] * b[ 0 ] + a[ 1 ] * b[ 1 ] +
    // a[ 2 ] * b[ 2 ] + a[ 3 ] * b[ 3 ];
    //
    // }
    //
    // return {
    //
    // equals: function( x, y, z, w, maxError ) {
    //
    // if ( maxError === undefined ) maxError = Number.EPSILON;
    //
    // return 	Math.abs( x - result[ 0 ] ) <= maxError &&
    // Math.abs( y - result[ 1 ] ) <= maxError &&
    // Math.abs( z - result[ 2 ] ) <= maxError &&
    // Math.abs( w - result[ 3 ] ) <= maxError;
    //
    // },
    //
    // length: Math.sqrt( arrDot( result, result ) ),
    //
    // dotA: arrDot( result, a ),
    // dotB: arrDot( result, b )
    //
    // };
    //
    // }
    //
    // function slerpTestSkeleton( doSlerp, maxError ) {
    //
    // var a, b, result;
    //
    // a = [
    // 0.6753410084407496,
    // 0.4087830051091744,
    // 0.32856700410659473,
    // 0.5185120064806223,
    // ];
    //
    // b = [
    // 0.6602792107657797,
    // 0.43647413932562285,
    // 0.35119011210236006,
    // 0.5001871596632682
    // ];
    //
    // var maxNormError = 0;
    //
    // function isNormal( result ) {
    //
    // var normError = Math.abs( 1 - result.length );
    // maxNormError = Math.max( maxNormError, normError );
    // return normError <= maxError;
    //
    // }
    //
    // result = doSlerp( a, b, 0 );
    // assert!( result.equals(
    // a[ 0 ], a[ 1 ], a[ 2 ], a[ 3 ], 0 ), "Exactly A @ t = 0" );
    //
    // result = doSlerp( a, b, 1 );
    // assert!( result.equals(
    // b[ 0 ], b[ 1 ], b[ 2 ], b[ 3 ], 0 ), "Exactly B @ t = 1" );
    //
    // result = doSlerp( a, b, 0.5 );
    // assert!( Math.abs( result.dotA - result.dotB ) <= Number.EPSILON, "Symmetry at 0.5" );
    // assert!( isNormal( result ), "Approximately normal (at 0.5)" );
    //
    // result = doSlerp( a, b, 0.25 );
    // assert!( result.dotA > result.dotB, "Interpolating at 0.25" );
    // assert!( isNormal( result ), "Approximately normal (at 0.25)" );
    //
    // result = doSlerp( a, b, 0.75 );
    // assert!( result.dotA < result.dotB, "Interpolating at 0.75" );
    // assert!( isNormal( result ), "Approximately normal (at 0.75)" );
    //
    // var D = Math.SQRT1_2;
    //
    // result = doSlerp( [ 1, 0, 0, 0 ], [ 0, 0, 1, 0 ], 0.5 );
    // assert!( result.equals( D, 0, D, 0 ), "X/Z diagonal from axes" );
    // assert!( isNormal( result ), "Approximately normal (X/Z diagonal)" );
    //
    // result = doSlerp( [ 0, D, 0, D ], [ 0, -D, 0, D ], 0.5 );
    // assert!( result.equals( 0, 0, 0, 1 ), "W-Unit from diagonals" );
    // assert!( isNormal( result ), "Approximately normal (W-Unit)" );
    // }
    //
    //
    // #[test]
    // fn slerp() {
    // slerpTestSkeleton( doSlerpObject, Number.EPSILON );

    // } );

    // test( "slerpFlat", function() {

    // slerpTestSkeleton( doSlerpArray, Number.EPSILON );

    // } );

}