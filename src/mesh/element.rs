use super::Mesh;
#[derive(Debug, Clone, Copy)]
pub struct AcousticElement {
    pub triangle_index: usize,
    pub center: Vec3,
    pub area: f64,
    pub normal: Vec3,
}
impl AcousticElement {
    pub fn new(triangle_index: usize, center: Vec3, area: f64, normal: Vec3) -> Self {
        AcousticElement {
            triangle_index,
            center,
            area,
            normal,
        }
    }
    pub fn from_triangle(triangle_index: usize, mesh: &Mesh) -> Self {
        let triangle = &mesh.triangles[triangle_index];
        let v0 = mesh.vertices[triangle.vertices[0]];
        let v1 = mesh.vertices[triangle.vertices[1]];
        let v2 = mesh.vertices[triangle.vertices[2]];
        let center = (v0 + v1 + v2) * (1.0 / 3.0);
        let normal = (v1 - v0).cross(v2 - v0);
        let area = 0.5 * normal.length();
        let normal = normal.normalize();
        AcousticElement::new(triangle_index, center, area, normal)
    }
}
