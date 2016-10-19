use math::Euler;
use math::Matrix3;
use math::Matrix4;
use math::Quaternion;
use math::Spherical;
use cameras::Camera;

#[derive(Debug,PartialEq,Copy,Clone)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
    pub const ZERO: Vector3 = Vector3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    pub const ONE: Vector3 = Vector3 {
        x: 1.0,
        y: 1.0,
        z: 1.0,
    };

    pub fn new(x: f32, y: f32, z: f32) -> Vector3 {
        Vector3 { x: x, y: y, z: z }
    }

    pub fn component(&self, index: i32) -> &f32 {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Vector3 only has 3 components, bad index"),
        }
    }

    pub fn copy(&self) -> Vector3 {
        Vector3 {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }

    pub fn add(&self, v: &Vector3) -> Vector3 {
        Vector3 {
            x: self.x + v.x,
            y: self.y + v.y,
            z: self.z + v.z,
        }
    }

    pub fn add_scalar(&self, s: f32) -> Vector3 {
        Vector3 {
            x: self.x + s,
            y: self.y + s,
            z: self.z + s,
        }
    }

    pub fn add_scaled_vector(&self, v: &Vector3, s: f32) -> Vector3 {
        Vector3 {
            x: self.x + (v.x * s),
            y: self.y + (v.y * s),
            z: self.z + (v.z * s),
        }
    }

    pub fn subtract(&self, v: &Vector3) -> Vector3 {
        Vector3 {
            x: self.x - v.x,
            y: self.y - v.y,
            z: self.z - v.z,
        }
    }

    pub fn subtract_scalar(&self, s: f32) -> Vector3 {
        Vector3 {
            x: self.x - s,
            y: self.y - s,
            z: self.z - s,
        }
    }

    pub fn multiply(&self, v: &Vector3) -> Vector3 {
        Vector3 {
            x: self.x * v.x,
            y: self.y * v.y,
            z: self.z * v.z,
        }
    }

    pub fn multiply_scalar(&self, scalar: f32) -> Vector3 {
        Vector3 {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }

    pub fn apply_euler(&self, euler: &Euler) -> Vector3 {
        self.apply_quaternion(&Quaternion::from_euler(euler))
    }

    pub fn apply_axis_angle(&self, axis: &Vector3, angle: f32) -> Vector3 {
        self.apply_quaternion(&Quaternion::from_axis_angle(axis, angle))
    }

    pub fn apply_matrix3(&self, m: &Matrix3) -> Vector3 {
        Vector3 {
            x: m.elements[0] * self.x + m.elements[3] * self.y + m.elements[6] * self.z,
            y: m.elements[1] * self.x + m.elements[4] * self.y + m.elements[7] * self.z,
            z: m.elements[2] * self.x + m.elements[5] * self.y + m.elements[8] * self.z,
        }
    }

    pub fn apply_matrix4(&self, affine: &Matrix4) -> Vector3 {
        Vector3 {
            x: affine.elements[0] * self.x + affine.elements[4] * self.y +
               affine.elements[8] * self.z + affine.elements[12],
            y: affine.elements[1] * self.x + affine.elements[5] * self.y +
               affine.elements[9] * self.z + affine.elements[13],
            z: affine.elements[2] * self.x + affine.elements[6] * self.y +
               affine.elements[10] * self.z + affine.elements[14],
        }
    }

    pub fn apply_projection(&self, projection: &Matrix4) -> Vector3 {
        let d = 1.0 /
                (projection.elements[3] * self.x + projection.elements[7] * self.y +
                 projection.elements[11] * self.z + projection.elements[15]); // perspective divide

        Vector3 {
            x: (projection.elements[0] * self.x + projection.elements[4] * self.y +
                projection.elements[8] * self.z + projection.elements[12]) * d,
            y: (projection.elements[1] * self.x + projection.elements[5] * self.y +
                projection.elements[9] * self.z + projection.elements[13]) * d,
            z: (projection.elements[2] * self.x + projection.elements[6] * self.y +
                projection.elements[10] * self.z + projection.elements[14]) * d,
        }
    }

    pub fn apply_quaternion(&self, q: &Quaternion) -> Vector3 {
        // calculate quat * vector
        let ix = q.w * self.x + q.y * self.z - q.z * self.y;
        let iy = q.w * self.y + q.z * self.x - q.x * self.z;
        let iz = q.w * self.z + q.x * self.y - q.y * self.x;
        let iw = -q.x * self.x - q.y * self.y - q.z * self.z;

        // calculate result * inverse quaternion
        Vector3 {
            x: ix * q.w + iw * -q.x + iy * -q.z - iz * -q.y,
            y: iy * q.w + iw * -q.y + iz * -q.x - ix * -q.z,
            z: iz * q.w + iw * -q.z + ix * -q.y - iy * -q.x,
        }
    }

    pub fn project(&self, camera: &Camera) -> Vector3 {
        let matrix = camera.projection_matrix.multiply(&camera.transform.matrix_world.inverse());
        self.apply_projection(&matrix)
    }

    pub fn unproject(&self, camera: &Camera) -> Vector3 {
        let matrix = camera.transform.matrix_world.multiply(&camera.projection_matrix.inverse());
        self.apply_projection(&matrix)
    }

    pub fn transform_direction(&self, affine: &Matrix4) -> Vector3 {
        // vector interpreted as a direction
        let transformed = Vector3 {
            x: affine.elements[0] * self.x + affine.elements[4] * self.y +
               affine.elements[8] * self.z,
            y: affine.elements[1] * self.x + affine.elements[5] * self.y +
               affine.elements[9] * self.z,
            z: affine.elements[2] * self.x + affine.elements[6] * self.y +
               affine.elements[10] * self.z,
        };
        transformed.normalize()
    }

    pub fn divide(&self, v: &Vector3) -> Vector3 {
        Vector3 {
            x: self.x / v.x,
            y: self.y / v.y,
            z: self.z / v.z,
        }
    }

    pub fn divide_scalar(&self, scalar: f32) -> Vector3 {
        self.multiply_scalar(1.0 / scalar)
    }

    pub fn min(&self, v: &Vector3) -> Vector3 {
        Vector3 {
            x: self.x.min(v.x),
            y: self.y.min(v.y),
            z: self.z.min(v.z),
        }
    }

    pub fn max(&self, v: &Vector3) -> Vector3 {
        Vector3 {
            x: self.x.max(v.x),
            y: self.y.max(v.y),
            z: self.z.max(v.z),
        }
    }

    pub fn clamp(&self, min: &Vector3, max: &Vector3) -> Vector3 {
        Vector3 {
            x: min.x.max(max.x.min(self.x)),
            y: min.y.max(max.y.min(self.y)),
            z: min.z.max(max.z.min(self.z)),
        }
    }

    pub fn clamp_scalar(&self, min: f32, max: f32) -> Vector3 {
        let min_v = Vector3::new(min, min, min);
        let max_v = Vector3::new(max, max, max);
        self.clamp(&min_v, &max_v)
    }

    pub fn floor(&self) -> Vector3 {
        Vector3 {
            x: self.x.floor(),
            y: self.y.floor(),
            z: self.z.floor(),
        }
    }

    pub fn ceil(&self) -> Vector3 {
        Vector3 {
            x: self.x.ceil(),
            y: self.y.ceil(),
            z: self.z.ceil(),
        }
    }

    pub fn round(&self) -> Vector3 {
        Vector3 {
            x: self.x.round(),
            y: self.y.round(),
            z: self.z.round(),
        }
    }

    pub fn round_to_zero(&self) -> Vector3 {
        Vector3 {
            x: if self.x < 0.0 {
                self.x.ceil()
            } else {
                self.x.floor()
            },
            y: if self.y < 0.0 {
                self.y.ceil()
            } else {
                self.y.floor()
            },
            z: if self.z < 0.0 {
                self.z.ceil()
            } else {
                self.z.floor()
            },
        }
    }

    pub fn negate(&self) -> Vector3 {
        Vector3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }

    pub fn dot(&self, v: &Vector3) -> f32 {
        self.x * v.x + self.y * v.y + self.z * v.z
    }

    pub fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn length_manhattan(&self) -> f32 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }

    pub fn normalize(&self) -> Vector3 {
        self.divide_scalar(self.length())
    }

    pub fn lerp(&self, v2: &Vector3, alpha: f32) -> Vector3 {
        v2.subtract(self).multiply_scalar(alpha).add(self)
    }

    pub fn cross(&self, v: &Vector3) -> Vector3 {
        Vector3 {
            x: self.y * v.z - self.z * v.y,
            y: self.z * v.x - self.x * v.z,
            z: self.x * v.y - self.y * v.x,
        }
    }

    pub fn project_on_vector(&self, vector: &Vector3) -> Vector3 {
        let scalar = vector.dot(self) / vector.length_squared();
        vector.multiply_scalar(scalar)
    }

    pub fn project_on_plane(&self, plane_normal: &Vector3) -> Vector3 {
        let v1 = self.project_on_vector(plane_normal);
        self.subtract(&v1)
    }

    pub fn reflect(&self, normal: &Vector3) -> Vector3 {
        // reflect incident vector off plane orthogonal to normal
        // normal is assumed to have unit length
        self.subtract(&normal.multiply_scalar(2.0 * self.dot(normal)))
    }


    pub fn angle_to(&self, v: &Vector3) -> f32 {
        let theta = self.dot(v) / (self.length_squared() * v.length_squared()).sqrt();

        // clamp, to handle numerical problems
        let clamped = theta.max(-1.0).min(1.0);
        clamped.acos()
    }

    pub fn distance_to(&self, v: &Vector3) -> f32 {
        self.distance_to_squared(v).sqrt()
    }

    pub fn distance_to_squared(&self, v: &Vector3) -> f32 {
        let dx = self.x - v.x;
        let dy = self.y - v.y;
        let dz = self.z - v.z;
        dx * dx + dy * dy + dz * dz
    }

    pub fn distance_to_manhattan(&self, v: &Vector3) -> f32 {
        (self.x - v.x).abs() + (self.y - v.y).abs() + (self.z - v.z).abs()
    }

    pub fn from_spherical(s: &Spherical) -> Vector3 {
        let sin_phi_radius = s.phi.sin() * s.radius;
        Vector3 {
            x: sin_phi_radius * s.theta.sin(),
            y: s.phi.cos() * s.radius,
            z: sin_phi_radius * s.theta.cos(),
        }
    }

    pub fn from_matrix_position(m: &Matrix4) -> Vector3 {
        Vector3::from_matrix_column(m, 3)
    }

    pub fn from_matrix_scale(m: &Matrix4) -> Vector3 {
        let x = Vector3::from_matrix_column(m, 0).length();
        let y = Vector3::from_matrix_column(m, 1).length();
        let z = Vector3::from_matrix_column(m, 2).length();
        Vector3::new(x, y, z)
    }

    pub fn from_matrix_column(m: &Matrix4, index: usize) -> Vector3 {
        Vector3::from_vec(&m.elements, index * 4)
    }

    pub fn from_vec(array: &[f32], offset: usize) -> Vector3 {
        Vector3 {
            x: array[offset],
            y: array[offset + 1],
            z: array[offset + 2],
        }
    }

    pub fn to_vec(&self) -> Vec<f32> {
        let mut vec: Vec<f32> = Vec::new();
        vec.push(self.x);
        vec.push(self.y);
        vec.push(self.z);
        vec
    }
}