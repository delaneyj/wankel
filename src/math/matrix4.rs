use math::Vector3;
use math::Euler;
use math::EulerOrder;
use math::Quaternion;

#[derive(Debug,PartialEq,Copy,Clone)]
pub struct Matrix4 {
    pub elements: [f32; 16],
}

pub struct MatrixBasis {
    pub x_axis: Vector3,
    pub y_axis: Vector3,
    pub z_axis: Vector3,
}

impl Matrix4 {
    pub const IDENTITY: Matrix4 = Matrix4 {
        elements: [1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0],
    };

    pub fn from_columns(column1: (f32, f32, f32, f32),
                        column2: (f32, f32, f32, f32),
                        column3: (f32, f32, f32, f32),
                        column4: (f32, f32, f32, f32))
                        -> Matrix4 {
        Matrix4 {
            elements: [column1.0, column1.1, column1.2, column1.3, column2.0, column2.1,
                       column2.2, column2.3, column3.0, column3.1, column3.2, column3.3,
                       column4.0, column4.1, column4.2, column4.3],
        }
    }

    pub fn from_rows(row1: (f32, f32, f32, f32),
                     row2: (f32, f32, f32, f32),
                     row3: (f32, f32, f32, f32),
                     row4: (f32, f32, f32, f32))
                     -> Matrix4 {
        Matrix4 {
            elements: [row1.0, row2.0, row3.0, row4.0, row1.1, row2.1, row3.1, row4.1, row1.2,
                       row2.2, row3.2, row4.2, row1.3, row2.3, row3.3, row4.3],
        }
    }

    pub fn copy_position(&self, m: &Matrix4) -> Matrix4 {
        let me = &m.elements;
        let mut te = self.elements;
        te[12] = me[12];
        te[13] = me[13];
        te[14] = me[14];

        Matrix4 { elements: te }
    }

    pub fn extract_basis(&self) -> MatrixBasis {
        MatrixBasis {
            x_axis: Vector3::from_matrix_column(self, 0),
            y_axis: Vector3::from_matrix_column(self, 1),
            z_axis: Vector3::from_matrix_column(self, 2),
        }
    }

    pub fn from_basis(x_axis: &Vector3, y_axis: &Vector3, z_axis: &Vector3) -> Matrix4 {
        Matrix4::from_columns((x_axis.x, y_axis.x, z_axis.x, 0.0),
                              (x_axis.y, y_axis.y, z_axis.y, 0.0),
                              (x_axis.z, y_axis.z, z_axis.z, 0.0),
                              (0.0, 0.0, 0.0, 1.0))
    }

    pub fn extract_rotation(&self) -> Matrix4 {
        let scale_x = 1.0 / Vector3::from_matrix_column(self, 0).length();
        let scale_y = 1.0 / Vector3::from_matrix_column(self, 1).length();
        let scale_z = 1.0 / Vector3::from_matrix_column(self, 2).length();

        Matrix4::from_columns((self.elements[0] * scale_x,
                               self.elements[1] * scale_x,
                               self.elements[2] * scale_x,
                               self.elements[3]),
                              (self.elements[4] * scale_y,
                               self.elements[5] * scale_y,
                               self.elements[6] * scale_y,
                               self.elements[7]),
                              (self.elements[8] * scale_z,
                               self.elements[9] * scale_z,
                               self.elements[10] * scale_z,
                               self.elements[11]),
                              (self.elements[12],
                               self.elements[13],
                               self.elements[14],
                               self.elements[15]))
    }


    pub fn rotation_from_euler(euler: &Euler) -> Matrix4 {
        let &Euler { x, y, z, ref order } = euler;
        let a = x.cos();
        let b = x.sin();
        let c = y.cos();
        let d = y.sin();
        let e = z.cos();
        let f = z.sin();

        let mut elements = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                            0.0, 1.0];

        match *order {
            EulerOrder::XYZ => {
                let ae = a * e;
                let af = a * f;
                let be = b * e;
                let bf = b * f;

                elements[0] = c * e;
                elements[4] = -c * f;
                elements[8] = d;

                elements[1] = af + be * d;
                elements[5] = ae - bf * d;
                elements[9] = -b * c;

                elements[2] = bf - ae * d;
                elements[6] = be + af * d;
                elements[10] = a * c;

            }
            EulerOrder::YXZ => {
                let ce = c * e;
                let cf = c * f;
                let de = d * e;
                let df = d * f;

                elements[0] = ce + df * b;
                elements[4] = de * b - cf;
                elements[8] = a * d;

                elements[1] = a * f;
                elements[5] = a * e;
                elements[9] = -b;

                elements[2] = cf * b - de;
                elements[6] = df + ce * b;
                elements[10] = a * c;

            }
            EulerOrder::ZXY => {
                let ce = c * e;
                let cf = c * f;
                let de = d * e;
                let df = d * f;

                elements[0] = ce - df * b;
                elements[4] = -a * f;
                elements[8] = de + cf * b;

                elements[1] = cf + de * b;
                elements[5] = a * e;
                elements[9] = df - ce * b;

                elements[2] = -a * d;
                elements[6] = b;
                elements[10] = a * c;

            }
            EulerOrder::ZYX => {
                let ae = a * e;
                let af = a * f;
                let be = b * e;
                let bf = b * f;

                elements[0] = c * e;
                elements[4] = be * d - af;
                elements[8] = ae * d + bf;

                elements[1] = c * f;
                elements[5] = bf * d + ae;
                elements[9] = af * d - be;

                elements[2] = -d;
                elements[6] = b * c;
                elements[10] = a * c;

            }
            EulerOrder::YZX => {
                let ac = a * c;
                let ad = a * d;
                let bc = b * c;
                let bd = b * d;

                elements[0] = c * e;
                elements[4] = bd - ac * f;
                elements[8] = bc * f + ad;

                elements[1] = f;
                elements[5] = a * e;
                elements[9] = -b * e;

                elements[2] = -d * e;
                elements[6] = ad * f + bc;
                elements[10] = ac - bd * f;

            }
            EulerOrder::XZY => {
                let ac = a * c;
                let ad = a * d;
                let bc = b * c;
                let bd = b * d;

                elements[0] = c * e;
                elements[4] = -f;
                elements[8] = d * e;

                elements[1] = ac * f + bd;
                elements[5] = a * e;
                elements[9] = ad * f - bc;

                elements[2] = bc * f - ad;
                elements[6] = b * e;
                elements[10] = bd * f + ac;

            }
        }

        Matrix4 { elements: elements }
    }

    pub fn rotation_from_quaternion(q: &Quaternion) -> Matrix4 {
        let &Quaternion { x, y, z, w } = q;
        let x2 = x + x;
        let y2 = y + y;
        let z2 = z + z;
        let xx = x * x2;
        let xy = x * y2;
        let xz = x * z2;
        let yy = y * y2;
        let yz = y * z2;
        let zz = z * z2;
        let wx = w * x2;
        let wy = w * y2;
        let wz = w * z2;

        Matrix4::from_columns((1.0 - (yy + zz), xy + wz, xz - wy, 0.0),
                              (xy - wz, 1.0 - (xx + zz), yz + wx, 0.0),
                              (xz + wy, yz - wx, 1.0 - (xx + yy), 0.0),
                              (0.0, 0.0, 0.0, 1.0))

    }

    pub fn look_at(&self, eye: &Vector3, target: &Vector3, up: &Vector3) -> Matrix4 {
        let mut z = eye.subtract(target).normalized();
        if z.length_squared() == 0.0 {
            z.z = 1.0;
        }

        let mut x = up.cross(&z).normalized();

        if x.length_squared() == 0.0 {
            z = Vector3 { z: z.z + 0.001, ..z };
            x = up.cross(&z).normalized();
        }

        let y = z.cross(&x);

        Matrix4::from_columns((x.x, x.y, x.z, self.elements[3]),
                              (y.x, y.y, y.z, self.elements[7]),
                              (z.x, z.y, z.z, self.elements[11]),
                              (self.elements[12],
                               self.elements[13],
                               self.elements[14],
                               self.elements[15]))
    }


    pub fn premultiply(&self, m: &Matrix4) -> Matrix4 {
        m.multiply(self)
    }

    pub fn multiply(&self, b: &Matrix4) -> Matrix4 {
        let [a11, a21, a31, a41, a12, a22, a32, a42, a13, a23, a33, a43, a14, a24, a34, a44] =
            self.elements;
        let [b11, b21, b31, b41, b12, b22, b32, b42, b13, b23, b33, b43, b14, b24, b34, b44] =
            b.elements;

        Matrix4::from_columns((a11 * b11 + a12 * b21 + a13 * b31 + a14 * b41,
                               a21 * b11 + a22 * b21 + a23 * b31 + a24 * b41,
                               a31 * b11 + a32 * b21 + a33 * b31 + a34 * b41,
                               a41 * b11 + a42 * b21 + a43 * b31 + a44 * b41),
                              (a11 * b12 + a12 * b22 + a13 * b32 + a14 * b42,
                               a21 * b12 + a22 * b22 + a23 * b32 + a24 * b42,
                               a31 * b12 + a32 * b22 + a33 * b32 + a34 * b42,
                               a41 * b12 + a42 * b22 + a43 * b32 + a44 * b42),
                              (a11 * b13 + a12 * b23 + a13 * b33 + a14 * b43,
                               a21 * b13 + a22 * b23 + a23 * b33 + a24 * b43,
                               a31 * b13 + a32 * b23 + a33 * b33 + a34 * b43,
                               a41 * b13 + a42 * b23 + a43 * b33 + a44 * b43),
                              (a11 * b14 + a12 * b24 + a13 * b34 + a14 * b44,
                               a21 * b14 + a22 * b24 + a23 * b34 + a24 * b44,
                               a31 * b14 + a32 * b24 + a33 * b34 + a34 * b44,
                               a41 * b14 + a42 * b24 + a43 * b34 + a44 * b44))
    }

    pub fn multiply_scalar(&self, s: f32) -> Matrix4 {
        Matrix4::from_columns((self.elements[0] * s,
                               self.elements[1] * s,
                               self.elements[2] * s,
                               self.elements[3] * s),
                              (self.elements[4] * s,
                               self.elements[5] * s,
                               self.elements[6] * s,
                               self.elements[7] * s),
                              (self.elements[8] * s,
                               self.elements[9] * s,
                               self.elements[10] * s,
                               self.elements[11] * s),
                              (self.elements[12] * s,
                               self.elements[13] * s,
                               self.elements[14] * s,
                               self.elements[15] * s))
    }

    pub fn determinant(&self) -> f32 {
        let [n11, n21, n31, n41, n12, n22, n32, n42, n13, n23, n33, n43, n14, n24, n34, n44] =
            self.elements;

        // TODO: make this more efficient
        // ( based on http://www.euclideanspace.com/maths/algebra/matrix/functions/inverse/fourD/index.htm )

        let a = n41 *
                (n14 * n23 * n32 - n13 * n24 * n32 - n14 * n22 * n33 + n12 * n24 * n33 +
                 n13 * n22 * n34 - n12 * n23 * n34);

        let b = n42 *
                (n11 * n23 * n34 - n11 * n24 * n33 + n14 * n21 * n33 - n13 * n21 * n34 +
                 n13 * n24 * n31 - n14 * n23 * n31);

        let c = n43 *
                (n11 * n24 * n32 - n11 * n22 * n34 - n14 * n21 * n32 + n12 * n21 * n34 +
                 n14 * n22 * n31 - n12 * n24 * n31);

        let d = n44 *
                (-n13 * n22 * n31 - n11 * n23 * n32 + n11 * n22 * n33 + n13 * n21 * n32 -
                 n12 * n21 * n33 + n12 * n23 * n31);

        a + b + c + d
    }

    pub fn transpose(&self) -> Matrix4 {
        Matrix4::from_columns((self.elements[0],
                               self.elements[4],
                               self.elements[8],
                               self.elements[12]),
                              (self.elements[1],
                               self.elements[5],
                               self.elements[9],
                               self.elements[13]),
                              (self.elements[2],
                               self.elements[6],
                               self.elements[10],
                               self.elements[14]),
                              (self.elements[3],
                               self.elements[7],
                               self.elements[11],
                               self.elements[15]))
    }

    pub fn with_position(&self, v: &Vector3) -> Matrix4 {
        Matrix4::from_columns((self.elements[0],
                               self.elements[1],
                               self.elements[2],
                               self.elements[3]),
                              (self.elements[4],
                               self.elements[5],
                               self.elements[6],
                               self.elements[7]),
                              (self.elements[8],
                               self.elements[9],
                               self.elements[10],
                               self.elements[11]),
                              (v.x, v.y, v.z, self.elements[15]))
    }

    pub fn inverse(&self) -> Matrix4 {
        // based on http://www.euclideanspace.com/maths/algebra/matrix/functions/inverse/fourD/index.htm
        let [n11, n21, n31, n41, n12, n22, n32, n42, n13, n23, n33, n43, n14, n24, n34, n44] =
            self.elements;

        let t11 = n23 * n34 * n42 - n24 * n33 * n42 + n24 * n32 * n43 - n22 * n34 * n43 -
                  n23 * n32 * n44 + n22 * n33 * n44;
        let t12 = n14 * n33 * n42 - n13 * n34 * n42 - n14 * n32 * n43 + n12 * n34 * n43 +
                  n13 * n32 * n44 - n12 * n33 * n44;
        let t13 = n13 * n24 * n42 - n14 * n23 * n42 + n14 * n22 * n43 - n12 * n24 * n43 -
                  n13 * n22 * n44 + n12 * n23 * n44;
        let t14 = n14 * n23 * n32 - n13 * n24 * n32 - n14 * n22 * n33 + n12 * n24 * n33 +
                  n13 * n22 * n34 - n12 * n23 * n34;

        let determinant = n11 * t11 + n21 * t12 + n31 * t13 + n41 * t14;

        if determinant == 0.0 {
            panic!("Can't invert matrix, determinant is 0");
            // return Matrix4::IDENTITY;
        }

        let determinant_inverse = 1.0 / determinant;
        Matrix4 {
            elements: [t11 * determinant_inverse,
                       (n24 * n33 * n41 - n23 * n34 * n41 - n24 * n31 * n43 + n21 * n34 * n43 +
                        n23 * n31 * n44 -
                        n21 * n33 * n44) * determinant_inverse,
                       (n22 * n34 * n41 - n24 * n32 * n41 + n24 * n31 * n42 - n21 * n34 * n42 -
                        n22 * n31 * n44 +
                        n21 * n32 * n44) * determinant_inverse,
                       (n23 * n32 * n41 - n22 * n33 * n41 - n23 * n31 * n42 + n21 * n33 * n42 +
                        n22 * n31 * n43 -
                        n21 * n32 * n43) * determinant_inverse,
                       //
                       t12 * determinant_inverse,
                       (n13 * n34 * n41 - n14 * n33 * n41 + n14 * n31 * n43 - n11 * n34 * n43 -
                        n13 * n31 * n44 +
                        n11 * n33 * n44) * determinant_inverse,
                       (n14 * n32 * n41 - n12 * n34 * n41 - n14 * n31 * n42 + n11 * n34 * n42 +
                        n12 * n31 * n44 -
                        n11 * n32 * n44) * determinant_inverse,
                       (n12 * n33 * n41 - n13 * n32 * n41 + n13 * n31 * n42 - n11 * n33 * n42 -
                        n12 * n31 * n43 +
                        n11 * n32 * n43) * determinant_inverse,
                       //
                       t13 * determinant_inverse,
                       (n14 * n23 * n41 - n13 * n24 * n41 - n14 * n21 * n43 + n11 * n24 * n43 +
                        n13 * n21 * n44 -
                        n11 * n23 * n44) * determinant_inverse,
                       (n12 * n24 * n41 - n14 * n22 * n41 + n14 * n21 * n42 - n11 * n24 * n42 -
                        n12 * n21 * n44 +
                        n11 * n22 * n44) * determinant_inverse,
                       (n13 * n22 * n41 - n12 * n23 * n41 - n13 * n21 * n42 + n11 * n23 * n42 +
                        n12 * n21 * n43 -
                        n11 * n22 * n43) * determinant_inverse,
                       //
                       t14 * determinant_inverse,
                       (n13 * n24 * n31 - n14 * n23 * n31 + n14 * n21 * n33 - n11 * n24 * n33 -
                        n13 * n21 * n34 +
                        n11 * n23 * n34) * determinant_inverse,
                       (n14 * n22 * n31 - n12 * n24 * n31 - n14 * n21 * n32 + n11 * n24 * n32 +
                        n12 * n21 * n34 -
                        n11 * n22 * n34) * determinant_inverse,
                       (n12 * n23 * n31 - n13 * n22 * n31 + n13 * n21 * n32 - n11 * n23 * n32 -
                        n12 * n21 * n33 +
                        n11 * n22 * n33) * determinant_inverse],
        }
    }

    pub fn scale(&self, v: &Vector3) -> Matrix4 {
        Matrix4::from_columns((self.elements[0] * v.x,
                               self.elements[1] * v.x,
                               self.elements[2] * v.x,
                               self.elements[3] * v.x),
                              (self.elements[4] * v.y,
                               self.elements[5] * v.y,
                               self.elements[6] * v.y,
                               self.elements[7] * v.y),
                              (self.elements[8] * v.z,
                               self.elements[9] * v.z,
                               self.elements[10] * v.z,
                               self.elements[11] * v.z),
                              (self.elements[12],
                               self.elements[13],
                               self.elements[14],
                               self.elements[15]))
    }

    pub fn max_scale_on_axis(&self) -> f32 {
        let te = self.elements;
        let scale_x_squared = te[0] * te[0] + te[1] * te[1] + te[2] * te[2];
        let scale_y_squared = te[4] * te[4] + te[5] * te[5] + te[6] * te[6];
        let scale_z_sqaured = te[8] * te[8] + te[9] * te[9] + te[10] * te[10];
        scale_x_squared.max(scale_y_squared).max(scale_z_sqaured).sqrt()
    }

    pub fn from_translation(v: &Vector3) -> Matrix4 {
        Matrix4::from_rows((1.0, 0.0, 0.0, 0.0),
                           (0.0, 1.0, 0.0, 0.0),
                           (0.0, 0.0, 1.0, 0.0),
                           (v.x, v.y, v.z, 1.0))
    }

    pub fn from_rotation_x(theta: f32) -> Matrix4 {
        let (c, s) = (theta.cos(), theta.sin());
        Matrix4::from_rows((1.0, 0.0, 0.0, 0.0),
                           (0.0, c, s, 0.0),
                           (0.0, -s, c, 0.0),
                           (0.0, 0.0, 0.0, 1.0))
    }

    pub fn from_rotation_y(theta: f32) -> Matrix4 {
        let (c, s) = (theta.cos(), theta.sin());
        Matrix4::from_rows((c, 0.0, -s, 0.0),
                           (0.0, 1.0, 0.0, 0.0),
                           (s, 0.0, c, 0.0),
                           (0.0, 0.0, 0.0, 1.0))
    }


    pub fn from_rotation_z(theta: f32) -> Matrix4 {
        let (c, s) = (theta.cos(), theta.sin());
        Matrix4::from_rows((c, s, 0.0, 0.0),
                           (-s, c, 0.0, 0.0),
                           (0.0, 0.0, 1.0, 0.0),
                           (0.0, 0.0, 0.0, 1.0))
    }

    pub fn from_rotation_axis(axis: &Vector3, angle: f32) -> Matrix4 {
        // Based on http://www.gamedev.net/reference/articles/article1199.aspect
        let (c, s) = (angle.cos(), angle.sin());
        let t = 1.0 - c;
        let &Vector3 { x, y, z } = axis;
        let (tx, ty) = (t * x, t * y);

        Matrix4::from_columns((tx * x + c, tx * y - s * z, tx * z + s * y, 0.0),
                              (tx * y + s * z, ty * y + c, ty * z - s * x, 0.0),
                              (tx * z - s * y, ty * z + s * x, t * z * z + c, 0.0),
                              (0.0, 0.0, 0.0, 1.0))
    }

    pub fn from_scale(v: &Vector3) -> Matrix4 {
        Matrix4::from_columns((v.x, 0.0, 0.0, 0.0),
                              (0.0, v.y, 0.0, 0.0),
                              (0.0, 0.0, v.z, 0.0),
                              (0.0, 0.0, 0.0, 1.0))
    }

    pub fn compose(position: &Vector3, quaternion: &Quaternion, scale: &Vector3) -> Matrix4 {
        Matrix4::rotation_from_quaternion(quaternion).scale(scale).with_position(position)
    }

    pub fn decompose(&self) -> (Vector3, Quaternion, Vector3) {
        let te = self.elements;
        let mut sx = Vector3::new(te[0], te[1], te[2]).length();
        let sy = Vector3::new(te[4], te[5], te[6]).length();
        let sz = Vector3::new(te[8], te[9], te[10]).length();

        // if determine is negative, we need to invert one scale
        if self.determinant() < 0.0 {
            sx = -sx;
        }

        let position = Vector3::new(te[12], te[13], te[14]);

        // scale the rotation part
        let inverse_square_x = 1.0 / sx;
        let inverse_square_y = 1.0 / sy;
        let inverse_square_z = 1.0 / sz;

        let matrix = Matrix4::from_columns((self.elements[0] * inverse_square_x,
                                            self.elements[1] * inverse_square_x,
                                            self.elements[2] * inverse_square_x,
                                            self.elements[3]),
                                           (self.elements[4] * inverse_square_y,
                                            self.elements[5] * inverse_square_y,
                                            self.elements[6] * inverse_square_y,
                                            self.elements[7]),
                                           (self.elements[8] * inverse_square_z,
                                            self.elements[9] * inverse_square_z,
                                            self.elements[10] * inverse_square_z,
                                            self.elements[11]),
                                           (self.elements[12],
                                            self.elements[13],
                                            self.elements[14],
                                            self.elements[15]));
        let quaternion = Quaternion::from_rotation_matrix(&matrix);
        let scale = Vector3::new(sx, sy, sz);

        (position, quaternion, scale)
    }

    pub fn from_frustum(left: f32,
                        right: f32,
                        bottom: f32,
                        top: f32,
                        near: f32,
                        far: f32)
                        -> Matrix4 {
        let x = 2.0 * near / (right - left);
        let y = 2.0 * near / (top - bottom);
        let a = (right + left) / (right - left);
        let b = (top + bottom) / (top - bottom);
        let c = -(far + near) / (far - near);
        let d = -2.0 * far * near / (far - near);

        Matrix4::from_columns((x, 0.0, 0.0, 0.0),
                              (0.0, y, 0.0, 0.0),
                              (a, b, c, -1.0),
                              (0.0, 0.0, d, 0.0))
    }

    pub fn from_perspective(fov: f32, aspect: f32, near: f32, far: f32) -> Matrix4 {
        let ymax = near * (fov.to_degrees() * 0.5).tan();
        let ymin = -ymax;
        let xmin = ymin * aspect;
        let xmax = ymax * aspect;

        Matrix4::from_frustum(xmin, xmax, ymin, ymax, near, far)
    }

    pub fn from_orthographic(left: f32,
                             right: f32,
                             top: f32,
                             bottom: f32,
                             near: f32,
                             far: f32)
                             -> Matrix4 {
        let w = 1.0 / (right - left);
        let h = 1.0 / (top - bottom);
        let p = 1.0 / (far - near);

        let x = (right + left) * w;
        let y = (top + bottom) * h;
        let z = (far + near) * p;

        Matrix4::from_columns((2.0 * w, 0.0, 0.0, 0.0),
                              (0.0, h, 0.0, 0.0),
                              (0.0, 0.0, -2.0 * p, 0.0),
                              (-x, -y, -z, 1.0))
    }
}