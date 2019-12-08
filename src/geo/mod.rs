mod arc;
mod triangle;
mod vertex;

pub const EPS: f64 = 1e-6;

pub use arc::{Arc, ArcIntersectionResult};
pub use triangle::Triangle;
pub use vertex::Vertex;
