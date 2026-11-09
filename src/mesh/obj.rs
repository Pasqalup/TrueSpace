use std::fs;
use std::io;
use std::path::Path;
use crate::vec3::Vec3;
use super::{Mesh, Triangle};
// TODO: implement OBJ file parsing

pub fn parse(text: &str) -> Mesh {
    let mut mesh = Mesh::new();
}