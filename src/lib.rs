mod geo;
mod graph;
mod model;

pub use geo::Vertex;
pub use model::{MergedModel, Model};

use model::ProjectionModel;

#[derive(Debug)]
pub struct Config {
    pub edge_only: bool,
    pub sphere_only: bool,
    pub scale: bool,
}

pub fn merge(model1: Model, model2: Model, config: Config) -> MergedModel {
    let model1 = ProjectionModel::new(model1);
    let model2 = ProjectionModel::new(model2);
    MergedModel::merge(model1, model2, config)
}
