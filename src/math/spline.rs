use math::Vector3;

pub struct Spline {
    points: Vec<Vector3>,
}

impl Spline {
    pub fn point_at(&self, k: f32) -> Vector3 {
        let point = (self.points.len() - 1) as f32 * k;
        let int_point = point.floor() as usize;
        let weight = point - (int_point as f32);
        let c = vec![
            if int_point == 0 { int_point}else {int_point -1 },
            int_point,
            if int_point > self.points.len() - 2 {self.points.len() -1 }else {int_point + 1},
            if int_point > self.points.len() - 3 {self.points.len() -1 }else {int_point + 2},
        ];


        let point_a = self.points[c[0]];
        let point_b = self.points[c[1]];
        let point_c = self.points[c[2]];
        let point_d = self.points[c[3]];

        let w2 = weight * weight;
        let w3 = weight * w2;

        Vector3 {
            x: Spline::interpolate(point_a.x, point_b.x, point_c.x, point_d.x, weight, w2, w3),
            y: Spline::interpolate(point_a.y, point_b.y, point_c.y, point_d.y, weight, w2, w3),
            z: Spline::interpolate(point_a.z, point_b.z, point_c.z, point_d.z, weight, w2, w3),
        }
    }

    pub fn control_points_array(&self) -> Vec<[f32; 3]> {
        let mut coordinates: Vec<[f32; 3]> = Vec::new();
        for point in &self.points {
            coordinates.push([point.x, point.y, point.z]);
        }
        coordinates
    }

    // approximate length by summing linear segments
    pub fn approximate_length(&self, subdivisions: usize) -> (Vec<f32>, f32) {
        // first point has 0 length
        let mut chunk_lengths = vec![0.0];
        let mut total_length = 0.0;
        let mut old_int_point = 0;
        let mut old_position = self.points[0];
        let samples = self.points.len() * subdivisions;

        for i in 1..samples {
            let index_f = (i as f32) / (samples as f32);
            let position = self.point_at(index_f);
            total_length += position.distance_to(&old_position);
            old_position = position;

            let point = ((self.points.len() as f32) - 1.0) * index_f;
            let int_point = point.floor() as usize;

            if int_point != old_int_point {
                chunk_lengths[int_point] = total_length;
                old_int_point = int_point;
            }
        }

        // last point ends with total length
        chunk_lengths.push(total_length);
        (chunk_lengths, total_length)
    }

    pub fn reparametrize_by_arc_length(&self, sampling_coef: f32) -> Spline {
        let mut new_points = vec![self.points[0]];
        let (chunks, total) = self.approximate_length(100);

        for (i, current) in self.points.iter().enumerate() {
            if i != 0 {
                let real_distance = chunks[i] - chunks[i - 1];
                let sampling = (sampling_coef * real_distance / total).ceil() as usize;

                let index_current = (i - 1) / (self.points.len() - 1);
                let index_next = i / (self.points.len() - 1);

                for j in 1..(sampling - 1) {
                    let sampling_inverse = (j as f32) * (1.0 / sampling as f32);
                    let offset = index_current as f32;
                    let delta = (index_next - index_current) as f32;
                    let index_f = offset + sampling_inverse * delta;
                    let position = self.point_at(index_f);
                    new_points.push(position);
                }

                new_points.push(*current);
            }
        }

        Spline { points: new_points }
    }

    // Catmull-Rom
    pub fn interpolate(p0: f32, p1: f32, p2: f32, p3: f32, t: f32, t2: f32, t3: f32) -> f32 {
        let v0 = (p2 - p0) * 0.5;
        let v1 = (p3 - p1) * 0.5;
        (2.0 * (p1 - p2) + v0 + v1) * t3 + (-3.0 * (p1 - p2) - 2.0 * v0 - v1) * t2 + v0 * t + p1
    }
}