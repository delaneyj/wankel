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
            println!("Matrix3::inverse> Can't invert matrix, determinant is 0");
            Matrix3::IDENTITY
        } else {
            let determinant_inverse = 1.0 / determinant;

            Matrix3::from_columns((t11 * determinant_inverse,
                                   (n31 * n23 - n33 * n21) * determinant_inverse,
                                   (n32 * n21 - n31 * n22) * determinant_inverse),
                                  (t12 * determinant_inverse,
                                   (n33 * n11 - n31 * n13) * determinant_inverse,
                                   (n31 * n12 - n32 * n11) * determinant_inverse),
                                  (t13 * determinant_inverse,
                                   (n21 * n13 - n23 * n11) * determinant_inverse,
                                   (n22 * n11 - n21 * n12) * determinant_inverse))
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
    use math::{Vector3, Matrix4};

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


    #[test]
    fn determinant() {
        let mut a = Matrix3::IDENTITY;
        assert_eq!(a.determinant(), 1.0);

        a.elements[0] = 2.0;
        assert_eq!(a.determinant(), 2.0);

        a.elements[0] = 0.0;
        assert_eq!(a.determinant(), 0.0);

        // calculated via http://www.euclideanspace.com/maths/algebra/matrix/functions/determinant/threeD/index.htm
        a = Matrix3::from_rows((2.0, 3.0, 4.0), (5.0, 13.0, 7.0), (8.0, 9.0, 11.0));
        assert_eq!(a.determinant(), -73.0);
    }


    #[test]
    fn inverse() {
        let identity = Matrix3::IDENTITY;
        let inverse_identity = identity.inverse();
        assert!(matrix3_close_enough(&identity, &inverse_identity));

        let zero = Matrix3::from_columns((0.0, 0.0, 0.0), (0.0, 0.0, 0.0), (0.0, 0.0, 0.0));
        assert!(matrix3_close_enough(&zero.inverse(), &identity));

        let test_matrices = vec![Matrix4::from_rotation_x(0.3),
                                 Matrix4::from_rotation_x(-0.3),
                                 Matrix4::from_rotation_y(0.3),
                                 Matrix4::from_rotation_y(-0.3),
                                 Matrix4::from_rotation_z(0.3),
                                 Matrix4::from_rotation_z(-0.3),
                                 Matrix4::from_scale(&Vector3::new(1.0, 2.0, 3.0)),
                                 Matrix4::from_scale(&Vector3::new(0.125, 0.5, 1.0 / 3.0))];

        for m in test_matrices {
            let a = Matrix3::from_matrix4(&m);
            let m_inverse3 = a.inverse();
            let m_inverse = m_inverse3.to_matrix4();

            // the determinant of the inverse should be the reciprocal
            assert!((a.determinant() * m_inverse3.determinant() - 1.0).abs() < 0.0001);
            assert!((m.determinant() * m_inverse.determinant() - 1.0).abs() < 0.0001);


            let m_product = m.multiply(&m_inverse);
            assert!((m_product.determinant() - 1.0).abs() < 0.0001);
            println!("m:{:?}\nproduct:{:?}", m, m_product);
            assert!(matrix3_close_enough(&Matrix3::from_matrix4(&m_product), &identity));
        }
    }

    #[test]
    fn transpose() {
        let a = Matrix3::IDENTITY;
        let b = a.transpose();
        assert!(matrix3_close_enough(&a, &b));

        let c = Matrix3::from_rows((0.0, 1.0, 2.0), (3.0, 4.0, 5.0), (6.0, 7.0, 8.0));
        let d = c.transpose();
        assert!(!matrix3_close_enough(&c, &d));
        let e = d.transpose();
        assert!(matrix3_close_enough(&c, &e));
    }
}