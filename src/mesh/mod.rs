pub mod obj;

mod element;
mod mesh;
mod primitives;

pub use element::AcousticElement;
pub use mesh::{Mesh, Triangle};
pub use primitives::icosahedron;

