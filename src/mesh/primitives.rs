use super::{Mesh, Triangle};
use crate::vec3::Vec3;

pub fn icosahedron() -> Mesh{
    let phi = (1.0 + 5.0_f64.sqrt()) / 2.0;
    let mut vertices = vec![
        Vec3::new(0.0, 1.0, phi),
        Vec3::new(0.0,-1.0, phi),
        Vec3::new(0.0, 1.0,-phi),
        Vec3::new(0.0,-1.0,-phi),

        Vec3::new(1.0, phi, 0.0),
        Vec3::new(-1.0, phi, 0.0),
        Vec3::new(1.0,-phi, 0.0),
        Vec3::new(-1.0,-phi, 0.0),

        Vec3::new(phi, 0.0, 1.0),
        Vec3::new(-phi, 0.0, 1.0),
        Vec3::new(phi, 0.0,-1.0),
        Vec3::new(-phi, 0.0,-1.0),
    ];
    for v in &mut vertices{
        *v = v.normalize();
    }
    let faces: [[usize; 3]; 20] = [
        [0, 1, 8],
        [0, 8, 4],
        [0, 4, 5],
        [0, 5, 9],
        [0, 9, 1],

        [1, 6, 8],
        [1, 7, 6],
        [1, 9, 7],

        [2, 3, 10],
        [2, 10, 4],
        [2, 4, 5],
        [2, 5, 11],
        [2, 11, 3],

        [3, 6, 10],
        [3, 7, 6],
        [3, 11, 7],

        [4, 10, 8],
        [5, 8, 11],
        [6, 9, 10],
        [7, 11, 9],
    ];
    let triangles: Vec<Triangle> = faces.iter().map(|f| Triangle{vertices: *f}).collect();
    Mesh{vertices, triangles}
}