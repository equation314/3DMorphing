mod arc;
mod face;
mod triangle;
mod vertex;

pub const EPS: f64 = 1e-9;

pub use arc::{Arc, ArcIntersectionResult};
pub use face::{adjust_order, check_order, Face};
pub use triangle::Triangle;
pub use vertex::Vertex;
