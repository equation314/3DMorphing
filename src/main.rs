use std::env;
use std::path::Path;

use morphing::Model;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        println!("Usage: \n{} <OBJ_FILE_1> <OBJ_FILE_2> <RATIO>", args[0]);
        return;
    }

    let ratio = args[3].parse().unwrap();
    let fname1 = &args[1];
    let fname2 = &args[2];

    let model1 = Model::load(fname1).expect(&format!("Cannot open model file \"{}\"", fname1));
    let model2 = Model::load(fname2).expect(&format!("Cannot open model file \"{}\"", fname2));

    let merged_model = morphing::merge(model1, model2);
    let out_fname = format!(
        "{:}_{}.obj",
        Path::new(fname1).file_stem().unwrap().to_string_lossy(),
        Path::new(fname2).file_stem().unwrap().to_string_lossy()
    );
    merged_model.save(&out_fname).unwrap();

    let result = merged_model.interpolation(ratio);
    result.save("output.obj").unwrap();
}
