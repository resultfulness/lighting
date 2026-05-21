use std::f32::consts::{PI, TAU};

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    position: [f32; 3],
    normal: [f32; 3],
}

impl Vertex {
    fn new_from_scalars(
        position_x: f32,
        position_y: f32,
        position_z: f32,
        normal_x: f32,
        normal_y: f32,
        normal_z: f32,
    ) -> Self {
        Self {
            position: [position_x, position_y, position_z],
            normal: [normal_x, normal_y, normal_z],
        }
    }
}

pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
}

impl Mesh {
    fn new(vertices: Vec<Vertex>, indices: Vec<u32>) -> Self {
        Self { vertices, indices }
    }

    pub fn cube(side_length: f32) -> Self {
        let h = side_length / 2.;
        let v = Vertex::new_from_scalars;

        let vertices = vec![
            v(-h, -h, h, 0., 0., 1.),
            v(h, -h, h, 0., 0., 1.),
            v(h, h, h, 0., 0., 1.),
            v(-h, h, h, 0., 0., 1.),
            v(h, -h, -h, 0., 0., -1.),
            v(-h, -h, -h, 0., 0., -1.),
            v(-h, h, -h, 0., 0., -1.),
            v(h, h, -h, 0., 0., -1.),
            v(-h, -h, -h, -1., 0., 0.),
            v(-h, -h, h, -1., 0., 0.),
            v(-h, h, h, -1., 0., 0.),
            v(-h, h, -h, -1., 0., 0.),
            v(h, -h, h, 1., 0., 0.),
            v(h, -h, -h, 1., 0., 0.),
            v(h, h, -h, 1., 0., 0.),
            v(h, h, h, 1., 0., 0.),
            v(-h, h, h, 0., 1., 0.),
            v(h, h, h, 0., 1., 0.),
            v(h, h, -h, 0., 1., 0.),
            v(-h, h, -h, 0., 1., 0.),
            v(-h, -h, -h, 0., -1., 0.),
            v(h, -h, -h, 0., -1., 0.),
            v(h, -h, h, 0., -1., 0.),
            v(-h, -h, h, 0., -1., 0.),
        ];

        let indices = vec![
            0, 1, 2, 2, 3, 0, 4, 5, 6, 6, 7, 4, 8, 9, 10, 10, 11, 8, 12, 13,
            14, 14, 15, 12, 16, 17, 18, 18, 19, 16, 20, 21, 22, 22, 23, 20,
        ];

        Mesh::new(vertices, indices)
    }

    pub fn sphere(radius: f32, stacks: u32, slices: u32) -> Self {
        let mut vertices = vec![];
        let mut indices = vec![];

        for i in 0..=stacks {
            let v = i as f32 / stacks as f32;
            let phi = PI * v;

            let y = phi.cos();
            let r = phi.sin();

            for j in 0..=slices {
                let u = j as f32 / slices as f32;
                let theta = TAU * u;

                let x = r * theta.cos();
                let z = r * theta.sin();

                vertices.push(Vertex::new_from_scalars(
                    x * radius,
                    y * radius,
                    z * radius,
                    x,
                    y,
                    z,
                ));

                if i == stacks || j == slices {
                    continue;
                }

                let a = i * (slices + 1) + j;
                let b = a + slices + 1;

                indices.push(a);
                indices.push(b);
                indices.push(a + 1);

                indices.push(b);
                indices.push(b + 1);
                indices.push(a + 1);
            }
        }

        Mesh::new(vertices, indices)
    }

    pub fn plane(size: f32) -> Self {
        let h = size * 0.5;
        let v = Vertex::new_from_scalars;

        let vertices = vec![
            v(-h, 0.0, -h, 0.0, 1.0, 0.0),
            v(h, 0.0, -h, 0.0, 1.0, 0.0),
            v(h, 0.0, h, 0.0, 1.0, 0.0),
            v(-h, 0.0, h, 0.0, 1.0, 0.0),
        ];

        let indices = vec![0, 1, 2, 2, 3, 0];

        Mesh::new(vertices, indices)
    }
}
