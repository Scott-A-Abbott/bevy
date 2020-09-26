use bevy_math::*;
use bevy_render::{
    mesh::{Mesh, VertexAttribute, VertexAttributeValues},
    pipeline::PrimitiveTopology,
};

pub struct ShadedQuad {
    x: f32,
    y: f32,
}

impl ShadedQuad {
    pub fn new(size: Vec2) -> Self {
        Self {
            x: size.x(),
            y: size.y(),
        }
    }
}

impl From<ShadedQuad> for Mesh {
    fn from(quad: ShadedQuad) -> Self {
        let extent_x = quad.x / 2.0;
        let extent_y = quad.y / 2.0;

        let north_west = vec2(-extent_x, extent_y);
        let north_east = vec2(extent_x, extent_y);
        let south_west = vec2(-extent_x, -extent_y);
        let south_east = vec2(extent_x, -extent_y);
        let vertices = [
            (
                [south_west.x(), south_west.y(), 0.0],
                [0.0, 0.0, 1.0],
                [0.0, 1.0],
            ),
            (
                [north_west.x(), north_west.y(), 0.0],
                [0.0, 0.0, 1.0],
                [0.0, 0.0],
            ),
            (
                [north_east.x(), north_east.y(), 0.0],
                [0.0, 0.0, 1.0],
                [1.0, 0.0],
            ),
            (
                [south_east.x(), south_east.y(), 0.0],
                [0.0, 0.0, 1.0],
                [1.0, 1.0],
            ),
        ];

        let indices = vec![0, 2, 1, 0, 3, 2];

        let mut positions = Vec::new();
        let mut normals = Vec::new();
        let mut uvs = Vec::new();
        for (position, normal, uv) in vertices.iter() {
            positions.push(*position);
            normals.push(*normal);
            uvs.push(*uv);
        }
        let tangents = calc_tang(&indices, &positions, &normals, &uvs);

        Mesh {
            primitive_topology: PrimitiveTopology::TriangleList,
            attributes: vec![
                VertexAttribute::position(positions),
                VertexAttribute::normal(normals),
                VertexAttribute::uv(uvs),
                VertexAttribute {
                    name: "Vertex_Tangent".into(),
                    values: VertexAttributeValues::Float4(tangents),
                },
            ],
            indices: Some(indices),
        }
    }
}

fn calc_tang(
    indices: &[u32],
    positions: &[[f32; 3]],
    normals: &[[f32; 3]],
    tex_coords: &[[f32; 2]],
) -> Vec<[f32; 4]> {
    let triangle_count = indices.len() / 3;
    let mut tangents: Vec<[f32; 4]> = Vec::new();
    let mut tan1: Vec<Vec3> = vec![Vec3::new(0., 0., 0.); positions.len()];
    let mut tan2: Vec<Vec3> = vec![Vec3::new(0., 0., 0.); positions.len()];
    let mut tri = 0;

    for _ in 0..triangle_count {
        let (i1, i2, i3) = (
            indices[tri] as usize,
            indices[tri + 1] as usize,
            indices[tri + 2] as usize,
        );
        let (v1, v2, v3) = (positions[i1], positions[i2], positions[i3]);
        let (w1, w2, w3) = (tex_coords[i1], tex_coords[i2], tex_coords[i3]);

        let (x, y, z) = (0, 1, 2);
        let (x1, x2) = (v2[x] - v1[x], v3[x] - v1[x]);
        let (y1, y2) = (v2[y] - v1[y], v3[y] - v1[y]);
        let (z1, z2) = (v2[z] - v1[z], v3[z] - v1[z]);

        let (s1, s2) = (w2[x] - w1[x], w3[x] - w1[x]);
        let (t1, t2) = (w2[y] - w1[y], w3[y] - w1[y]);

        let div = s1 * t2 - s2 * t1;
        let r = {
            if div == 0.0 {
                0.0
            } else {
                1.0 / div
            }
        };
        let s_dir = {
            let x = (t2 * x1 - t1 * x2) * r;
            let y = (t2 * y1 - t1 * y2) * r;
            let z = (t2 * z1 - t1 * z2) * r;
            Vec3::new(x, y, z)
        };
        let t_dir = {
            let x = (s2 * x1 - s1 * x2) * r;
            let y = (s2 * y1 - s1 * y2) * r;
            let z = (s2 * z1 - s1 * z2) * r;
            Vec3::new(x, y, z)
        };

        tan1[i1] += s_dir;
        tan1[i2] += s_dir;
        tan1[i3] += s_dir;

        tan2[i1] += t_dir;
        tan2[i2] += t_dir;
        tan2[i3] += t_dir;

        tri += 3;
    }

    for i in 0..positions.len() {
        let n = normals[i];
        let t = tan1[i].clone();

        let w = {
            let n = Vec3::new(n[0], n[1], n[2]);
            let cross = n.clone().cross(t);
            if cross.clone().dot(tan2[i]) < 0.0 {
                -1.
            } else {
                1.
            }
        };
        tangents.push([t.x(), t.y(), t.z(), w]);
    }

    tangents
}
