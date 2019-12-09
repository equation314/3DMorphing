mod geo;
mod graph;
mod model;

pub use geo::Vertex;
pub use model::{MergedModel, Model};

use model::ProjectionModel;

pub fn merge(model1: Model, model2: Model) -> MergedModel {
    let model1 = ProjectionModel::new(model1);
    let model2 = ProjectionModel::new(model2);
    MergedModel::merge(model1, model2)
}
