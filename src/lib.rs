mod geo;
mod model;

pub use geo::Vertex;
pub use model::Model;

use model::{MergedModel, ProjectionModel};

pub fn morphing(model1: Model, model2: Model, ratio: f32) -> Model {
    let model1 = ProjectionModel::new(model1);
    let model2 = ProjectionModel::new(model2);
    let merged_model = MergedModel::merge(model1, model2);
    merged_model.interpolation(ratio)
}
