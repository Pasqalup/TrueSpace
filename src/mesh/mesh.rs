use crate::vec3::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Triangle{
    vertices: [usize; 3],
}
#[derive(Debug)]
pub struct Mesh{
    vertices: Vec<Vec3>,
    triangles: Vec<Triangle>,
}

impl Mesh {
    pub fn new(vertices: Vec<Vec3>, triangles: Vec<Triangle>) -> Self {
        Mesh { vertices, triangles }
    }

    pub fn vertices(&self) -> &Vec<Vec3> {
        &self.vertices
    }

    pub fn triangles(&self) -> &Vec<Triangle> {
        &self.triangles
    }
}