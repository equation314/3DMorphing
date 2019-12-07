use std::env;

use morphing::morphing;
use morphing::Model;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Usage: \n{} <OBJ_FILE_1> <OBJ_FILE_2>", args[0]);
        return;
    }

    let model1 = Model::load(&args[1]).expect(&format!("Cannot open model file \"{}\"", args[1]));
    let model2 = Model::load(&args[2]).expect(&format!("Cannot open model file \"{}\"", args[2]));
    let out = morphing(model1, model2, 0.5);

    out.save("output.obj").unwrap();
}
