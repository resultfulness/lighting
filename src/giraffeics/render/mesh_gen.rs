use std::f32::consts::{PI, TAU};

use crate::giraffeics::render::mesh::{Mesh, Vertex};

impl Mesh {
    pub fn cube(side_length: f32) -> Result<Self, String> {
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

        Self::from_vertices_indices(&vertices, &indices)
    }

    pub fn sphere(
        radius: f32,
        stacks: u32,
        slices: u32,
    ) -> Result<Self, String> {
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

        Self::from_vertices_indices(&vertices, &indices)
    }

    pub fn plane(size: f32) -> Result<Self, String> {
        let h = size * 0.5;
        let v = Vertex::new_from_scalars;

        let vertices = vec![
            v(-h, 0.0, -h, 0.0, 1.0, 0.0),
            v(h, 0.0, -h, 0.0, 1.0, 0.0),
            v(h, 0.0, h, 0.0, 1.0, 0.0),
            v(-h, 0.0, h, 0.0, 1.0, 0.0),
        ];

        let indices = vec![0, 1, 2, 2, 3, 0];

        Self::from_vertices_indices(&vertices, &indices)
    }
}
