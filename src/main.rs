use std::env;

use morphing::model::Model;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Usage: \n{} <OBJ_FILE_1> <OBJ_FILE_2>", args[0]);
        return;
    }

    let model1 = Model::load(&args[1]).expect(&format!("Cannot open model file {}", args[1]));
    let model2 = Model::load(&args[2]).expect(&format!("Cannot open model file {}", args[2]));

    println!("{:#?} {:#?}", model1.verts.len(), model1.faces.len());
    println!("{:#?} {:#?}", model2.verts.len(), model2.faces.len());
}
