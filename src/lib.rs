pub mod model;
pub mod vertex;

pub use model::Model;
pub use vertex::Vertex;

use model::ProjectionModel;

pub fn morphing(model1: Model, model2: Model, _ratio: f32) -> Model {
    let model1 = ProjectionModel::new(model1);
    let model2 = ProjectionModel::new(model2);

    println!(
        "{:#?} {:#?} {:?}",
        model1.model.nr_verts(),
        model1.model.nr_faces(),
        model1.center,
    );
    println!(
        "{:#?} {:#?} {:?}",
        model2.model.nr_verts(),
        model2.model.nr_faces(),
        model1.center,
    );

    Model::new(model1.sphere_verts, model1.model.faces)
}
