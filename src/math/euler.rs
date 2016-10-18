use math::{Matrix4, Quaternion, Vector3};

#[derive(Debug,PartialEq,Clone,Copy)]
pub enum EulerOrder {
    XYZ,
    YZX,
    ZXY,
    XZY,
    YXZ,
    ZYX,
}

#[derive(Debug,PartialEq)]
pub struct Euler {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub order: EulerOrder,
}

impl Euler {
    pub const DEFAULT_ORDER: EulerOrder = EulerOrder::XYZ;

    pub const DEFAULT: Euler = Euler {
        x: 0.0,
        y: 0.0,
        z: 0.0,
        order: Euler::DEFAULT_ORDER,
    };

    pub fn new(x: f32, y: f32, z: f32, order: &EulerOrder) -> Euler {
        Euler {
            x: x,
            y: y,
            z: z,
            order: *order,
        }
    }

    pub fn from_rotation_matrix(m: &Matrix4, order: &EulerOrder) -> Euler {
        // assumes the upper 3x3 of m is a pure rotation matrix (i.e, unscaled)
        let [m11, m21, m31, _, m12, m22, m32, _, m13, m23, m33, _..] = m.elements;

        let mut x = 0.0;
        let mut y = 0.0;
        let mut z = 0.0;

        let limit = 0.99999;
        let clamped = |x: f32| x.max(-1.0).min(1.0);

        match *order {
            EulerOrder::XYZ => {
                y = clamped(m13).asin();
                if m13.abs() < limit {
                    x = (-m23).atan2(m33);
                    z = (-m12).atan2(m11);
                } else {
                    x = m32.atan2(m22);
                }
            }
            EulerOrder::YXZ => {
                x = clamped(-m23).asin();

                if m23.abs() < limit {
                    y = m13.atan2(m33);
                    z = m21.atan2(m22);
                } else {
                    y = (-m31).atan2(m11);
                }
            }
            EulerOrder::ZXY => {
                x = clamped(m32).asin();

                if m32.abs() < limit {
                    y = (-m31).atan2(m33);
                    z = (-m12).atan2(m22);
                } else {
                    z = m21.atan2(m11);
                }
            }
            EulerOrder::ZYX => {
                y = (-clamped(m31)).asin();

                if m31.abs() < limit {
                    x = m32.atan2(m33);
                    z = m21.atan2(m11);
                } else {
                    z = (-m12).atan2(m22);
                }
            }
            EulerOrder::YZX => {
                z = clamped(m21).asin();

                if m21.abs() < limit {
                    x = (-m23).atan2(m22);
                    y = (-m31).atan2(m11);
                } else {
                    y = m13.atan2(m33);
                }
            }
            EulerOrder::XZY => {
                z = (-clamped(m12)).asin();

                if m12.abs() < limit {
                    x = m32.atan2(m22);
                    y = m13.atan2(m11);
                } else {
                    x = (-m23).atan2(m33);
                }
            }
        }

        Euler::new(x, y, z, order)
    }

    pub fn from_quaternion(q: &Quaternion, order: &EulerOrder) -> Euler {
        Euler::from_rotation_matrix(&Matrix4::rotation_from_quaternion(q), order)
    }

    pub fn from_vector3(v: &Vector3, order: &EulerOrder) -> Euler {
        Euler::new(v.x, v.y, v.z, order)
    }

    pub fn reorder(&self, new_order: &EulerOrder) -> Euler {
        Euler::from_quaternion(&Quaternion::from_euler(self), new_order)
    }

    pub fn to_vector3(&self) -> Vector3 {
        Vector3::new(self.x, self.y, self.z)
    }
}

#[cfg(test)]
mod tests {
    use super::{Euler, EulerOrder};
    use math::{Vector3, Quaternion, Matrix4};

    const EULER_ZERO: Euler = Euler {
        x: 0.0,
        y: 0.0,
        z: 0.0,
        order: EulerOrder::XYZ,
    };
    const EULER_A_XYZ: Euler = Euler {
        x: 1.0,
        y: 0.0,
        z: 0.0,
        order: EulerOrder::XYZ,
    };
    const EULER_A_ZYX: Euler = Euler {
        x: 0.0,
        y: 1.0,
        z: 0.0,
        order: EulerOrder::ZYX,
    };
    const TOLERANCE: f32 = 0.0001;

    fn matrix4s_close_enough(a: &Matrix4, b: &Matrix4) -> bool {
        for i in 0..16 {
            let delta = a.elements[i] - b.elements[i];
            if delta > TOLERANCE {
                return false;
            }
        }
        true
    }

    fn eulers_close_enough(a: &Euler, b: &Euler) -> bool {
        let diff = (a.x - b.x).abs() + (a.y - b.y).abs() + (a.z - b.z).abs();
        diff < TOLERANCE
    }


    fn quaternions_close_enough(a: &Quaternion, b: &Quaternion) -> bool {
        let diff = (a.x - b.x).abs() + (a.y - b.y).abs() + (a.z - b.z).abs() + (a.w - b.w).abs();
        diff < TOLERANCE
    }

    #[test]
    fn set_set_from_vector3_to_vector3() {
        let a = Euler::new(0.0, 1.0, 0.0, &EulerOrder::ZYX);
        assert_eq!(a, EULER_A_ZYX);
        assert!(a != EULER_A_XYZ);
        assert!(a != EULER_ZERO);

        let vec = Vector3::new(0.0, 1.0, 0.0);
        let b = Euler::from_vector3(&vec, &EulerOrder::ZYX);
        assert_eq!(a, b);

        let c = b.to_vector3();
        assert_eq!(c, vec);
    }

    #[test]
    fn from_euler_from_quaternion() {
        for e in vec![EULER_ZERO, EULER_A_XYZ, EULER_A_ZYX] {
            let q = Quaternion::from_euler(&e);
            let v2 = Euler::from_quaternion(&q, &e.order);
            let q2 = Quaternion::from_euler(&v2);
            assert!(quaternions_close_enough(&q, &q2));
        }
    }

    #[test]
    fn from_rotation_matrix() {
        for e in vec![EULER_ZERO, EULER_A_XYZ, EULER_A_ZYX] {
            let m = Matrix4::rotation_from_euler(&e);
            let v2 = Euler::from_rotation_matrix(&m, &e.order);
            let m2 = Matrix4::rotation_from_euler(&v2);
            assert!(matrix4s_close_enough(&m, &m2))
        }
    }

    #[test]
    fn reorder() {
        for e in vec![EULER_ZERO, EULER_A_XYZ, EULER_A_ZYX] {
            let q = Quaternion::from_euler(&e);

            let e2 = e.reorder(&EulerOrder::YZX);
            let q2 = Quaternion::from_euler(&e2);
            assert!(quaternions_close_enough(&q, &q2));

            let e3 = e.reorder(&EulerOrder::ZXY);
            let q3 = Quaternion::from_euler(&e3);
            assert!(quaternions_close_enough(&q, &q3));
        }
    }

    #[test]
    fn gimbal_local_quaternion() {
        // known problematic quaternions
        for q in vec![Quaternion::new(0.5207769385244341,
                                      -0.4783214164122354,
                                      0.520776938524434,
                                      0.47832141641223547),
                      Quaternion::new(0.11284905712620674,
                                      0.6980437630368944,
                                      -0.11284905712620674,
                                      0.6980437630368944)] {
            let order = EulerOrder::ZYX;

            // create Euler directly from a Quaternion
            let e_via_q = Euler::from_quaternion(&q, &order); // there is likely a bug here

            // create Euler from Quaternion via an intermediate Matrix4
            let m_via_q = Matrix4::rotation_from_quaternion(&q);
            let e_via_m_via_q = Euler::from_rotation_matrix(&m_via_q, &order);

            // the results here are different
            assert!(eulers_close_enough(&e_via_q, &e_via_m_via_q));  // this result is correct
        }
    }
}