use math::Matrix4;

#[derive(Debug,PartialEq)]
pub struct Matrix3 {
    pub elements: [f32; 9],
}

impl Matrix3 {
    pub const IDENTITY: Matrix3 =
        Matrix3 { elements: [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0] };

    pub fn from_columns(column1: (f32, f32, f32),
                        column2: (f32, f32, f32),
                        column3: (f32, f32, f32))
                        -> Matrix3 {
        Matrix3 {
            elements: [column1.0, column1.1, column1.2, column2.0, column2.1, column2.2,
                       column3.0, column3.1, column3.2],
        }
    }

    pub fn from_rows(row1: (f32, f32, f32),
                     row2: (f32, f32, f32),
                     row3: (f32, f32, f32))
                     -> Matrix3 {
        Matrix3 {
            elements: [row1.0, row2.0, row3.0, row1.1, row2.1, row3.1, row1.2, row2.2, row3.2],
        }
    }

    pub fn new(column1: (f32, f32, f32),
               column2: (f32, f32, f32),
               column3: (f32, f32, f32))
               -> Matrix3 {
        Matrix3 {
            elements: [column1.0, column1.1, column1.2, column2.0, column2.1, column2.2,
                       column3.0, column3.1, column3.2],
        }
    }

    pub fn from_matrix4(m: &Matrix4) -> Matrix3 {
        let me = m.elements;
        Matrix3 { elements: [me[0], me[4], me[8], me[1], me[5], me[9], me[2], me[6], me[10]] }
    }


    pub fn multiply_scalar(&self, s: f32) -> Matrix3 {
        Matrix3 {
            elements: [self.elements[0] * s,
                       self.elements[1] * s,
                       self.elements[2] * s,
                       self.elements[3] * s,
                       self.elements[4] * s,
                       self.elements[5] * s,
                       self.elements[6] * s,
                       self.elements[7] * s,
                       self.elements[8] * s],
        }
    }

    pub fn determinant(&self) -> f32 {
        let [a, b, c, d, e, f, g, h, i] = self.elements;
        a * e * i - a * f * h - b * d * i + b * f * g + c * d * h - c * e * g
    }

    pub fn inverse(&self) -> Matrix3 {
        let [n11, n21, n31, n12, n22, n32, n13, n23, n33] = self.elements;
        let t11 = n33 * n22 - n32 * n23;
        let t12 = n32 * n13 - n33 * n12;
        let t13 = n23 * n12 - n22 * n13;
        let determinant = n11 * t11 + n21 * t12 + n31 * t13;

        if determinant == 0.0 {
            panic!("Can't invert matrix, determinant is 0");
            // return Matrix3::IDENTITY;
        }

        let determinant_inverse = 1.0 / determinant;

        Matrix3 {
            elements: [t11 * determinant_inverse,
                       (n31 * n23 - n33 * n21) * determinant_inverse,
                       (n32 * n21 - n31 * n22) * determinant_inverse,
                       t12 * determinant_inverse,
                       (n33 * n11 - n31 * n13) * determinant_inverse,
                       (n31 * n12 - n32 * n11) * determinant_inverse,
                       t13 * determinant_inverse,
                       (n21 * n13 - n23 * n11) * determinant_inverse,
                       (n22 * n11 - n21 * n12) * determinant_inverse],
        }
    }

    pub fn transpose(&self) -> Matrix3 {
        Matrix3 {
            elements: [self.elements[0],
                       self.elements[3],
                       self.elements[6],
                       self.elements[1],
                       self.elements[4],
                       self.elements[7],
                       self.elements[2],
                       self.elements[5],
                       self.elements[8]],
        }
    }

    pub fn normal_matrix(m: &Matrix4) -> Matrix3 {
        Matrix3::from_matrix4(m).inverse().transpose()
    }

    pub fn to_matrix4(&self) -> Matrix4 {
        let m = self.elements;
        Matrix4::from_columns((m[0], m[1], m[2], 0.0),
                              (m[3], m[4], m[5], 0.0),
                              (m[6], m[7], m[8], 0.0),
                              (0.0, 0.0, 0.0, 1.0))
    }
}

#[cfg(test)]
mod tests {
    use super::Matrix3;

    const TOLERANCE: f32 = 0.0001;

    fn matrix3_close_enough(a: &Matrix3, b: &Matrix3) -> bool {
        for i in 0..9 {
            let delta = a.elements[i] - b.elements[i];
            if delta > TOLERANCE {
                return false;
            }
        }

        true
    }

    #[test]
    fn constructor() {
        let a = Matrix3::IDENTITY;
        assert_eq!(a.determinant(), 1.0);

        let b = Matrix3::from_rows((0.0, 1.0, 2.0), (3.0, 4.0, 5.0), (6.0, 7.0, 8.0));
        assert_eq!(b.elements[0], 0.0);
        assert_eq!(b.elements[1], 3.0);
        assert_eq!(b.elements[2], 6.0);
        assert_eq!(b.elements[3], 1.0);
        assert_eq!(b.elements[4], 4.0);
        assert_eq!(b.elements[5], 7.0);
        assert_eq!(b.elements[6], 2.0);
        assert_eq!(b.elements[7], 5.0);
        assert_eq!(b.elements[8], 8.0);

        assert!(!matrix3_close_enough(&a, &b));
    }

    #[test]
    fn copy() {
        let mut a = Matrix3::from_columns((0.0, 1.0, 2.0), (3.0, 4.0, 5.0), (6.0, 7.0, 8.0));
        let b = Matrix3 { ..a };

        assert!(matrix3_close_enough(&a, &b));

        // ensure that it is a true copy
        a.elements[0] = 2.0;
        assert!(!matrix3_close_enough(&a, &b));
    }

    #[test]
    fn multiply_scalar() {
        let a = Matrix3::from_rows((0.0, 1.0, 2.0), (3.0, 4.0, 5.0), (6.0, 7.0, 8.0));
        assert_eq!(a.elements[0], 0.0);
        assert_eq!(a.elements[1], 3.0);
        assert_eq!(a.elements[2], 6.0);
        assert_eq!(a.elements[3], 1.0);
        assert_eq!(a.elements[4], 4.0);
        assert_eq!(a.elements[5], 7.0);
        assert_eq!(a.elements[6], 2.0);
        assert_eq!(a.elements[7], 5.0);
        assert_eq!(a.elements[8], 8.0);

        let b = a.multiply_scalar(2.0);
        assert_eq!(b.elements[0], 0.0 * 2.0);
        assert_eq!(b.elements[1], 3.0 * 2.0);
        assert_eq!(b.elements[2], 6.0 * 2.0);
        assert_eq!(b.elements[3], 1.0 * 2.0);
        assert_eq!(b.elements[4], 4.0 * 2.0);
        assert_eq!(b.elements[5], 7.0 * 2.0);
        assert_eq!(b.elements[6], 2.0 * 2.0);
        assert_eq!(b.elements[7], 5.0 * 2.0);
        assert_eq!(b.elements[8], 8.0 * 2.0);
    }


    // test( "determinant", function() {
    // 	var a = new THREE.Matrix3();
    // 	assert_eq!( a.determinant() == 1);

    // 	a.elements[0] = 2;
    // 	assert_eq!( a.determinant() == 2);

    // 	a.elements[0] = 0;
    // 	assert_eq!( a.determinant() == 0);

    // 	// calculated via http://www.euclideanspace.com/maths/algebra/matrix/functions/determinant/threeD/index.htm
    // 	a.set( 2, 3, 4, 5, 13, 7, 8, 9, 11 );
    // 	assert_eq!( a.determinant() == -73);
    // });


    // test( "getInverse", function() {
    // 	var identity = new THREE.Matrix3();
    // 	var identity4 = new THREE.Matrix4();
    // 	var a = new THREE.Matrix3();
    // 	var b = new THREE.Matrix3().set( 0, 0, 0, 0, 0, 0, 0, 0, 0 );
    // 	var c = new THREE.Matrix3().set( 0, 0, 0, 0, 0, 0, 0, 0, 0 );

    // 	b.getInverse( a, false );
    // 	assert_eq!( matrix3_close_enough( a, identity ));

    // 	try {
    // 		b.getInverse( c, true );
    // 		assert_eq!( false); // should never get here.
    // 	}
    // 	catch( err ) {
    // 		assert_eq!( true);
    // 	}

    // 	var testMatrices = [
    // 		new THREE.Matrix4().makeRotationX( 0.3 ),
    // 		new THREE.Matrix4().makeRotationX( -0.3 ),
    // 		new THREE.Matrix4().makeRotationY( 0.3 ),
    // 		new THREE.Matrix4().makeRotationY( -0.3 ),
    // 		new THREE.Matrix4().makeRotationZ( 0.3 ),
    // 		new THREE.Matrix4().makeRotationZ( -0.3 ),
    // 		new THREE.Matrix4().makeScale( 1, 2, 3 ),
    // 		new THREE.Matrix4().makeScale( 1/8, 1/2, 1/3 )
    // 		];

    // 	for( var i = 0, il = testMatrices.length; i < il; i ++ ) {
    // 		var m = testMatrices[i];

    // 		a.setFromMatrix4( m );
    // 		var mInverse3 = b.getInverse( a );

    // 		var mInverse = toMatrix4( mInverse3 );

    // 		// the determinant of the inverse should be the reciprocal
    // 		assert_eq!( Math.abs( a.determinant() * mInverse3.determinant() - 1 ) < 0.0001);
    // 		assert_eq!( Math.abs( m.determinant() * mInverse.determinant() - 1 ) < 0.0001);

    // 		var mProduct = new THREE.Matrix4().multiplyMatrices( m, mInverse );
    // 		assert_eq!( Math.abs( mProduct.determinant() - 1 ) < 0.0001);
    // 		assert_eq!( matrix3_close_enough( mProduct, identity4 ));
    // 	}
    // });

    // test( "transpose", function() {
    // 	var a = new THREE.Matrix3();
    // 	var b = a.clone().transpose();
    // 	assert_eq!( matrix3_close_enough( a, b ));

    // 	b = new THREE.Matrix3().set( 0, 1, 2, 3, 4, 5, 6, 7, 8 );
    // 	var c = b.clone().transpose();
    // 	assert_eq!( ! matrix3_close_enough( b, c ));
    // 	c.transpose();
    // 	assert_eq!( matrix3_close_enough( b, c ));
    // });

    // test( "clone", function() {
    // 	var a = new THREE.Matrix3().set( 0, 1, 2, 3, 4, 5, 6, 7, 8 );
    // 	var b = a.clone();

    // 	assert_eq!( matrix3_close_enough( a, b ));

    // 	// ensure that it is a true copy
    // 	a.elements[0] = 2;
    // 	assert_eq!( ! matrix3_close_enough( a, b ));
    // });
}