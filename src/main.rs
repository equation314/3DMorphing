use std::path::Path;

use clap::{App, Arg};
use morphing::Model;

fn main() {
    let matches = App::new("morphing")
        .arg(
            Arg::with_name("obj1")
                .required(true)
                .help("Model file 1 (*.obj)"),
        )
        .arg(
            Arg::with_name("obj2")
                .required(true)
                .help("Model file 2 (*.obj)"),
        )
        .arg(
            Arg::with_name("output")
                .long("output")
                .short("o")
                .takes_value(true)
                .help("Result model file (*.obj)"),
        )
        .arg(
            Arg::with_name("ratio")
                .long("ratio")
                .short("r")
                .takes_value(true)
                .default_value("0.5")
                .help("Morphing ratio"),
        )
        .arg(
            Arg::with_name("edge_only")
                .long("edge")
                .short("e")
                .help("Product edges only, no faces"),
        )
        .get_matches();

    let ratio = matches.value_of("ratio").unwrap().parse().unwrap();
    let fname1 = matches.value_of("obj1").unwrap();
    let fname2 = matches.value_of("obj2").unwrap();

    let model1 = Model::load(fname1).expect(&format!("Cannot open model file \"{}\"", fname1));
    let model2 = Model::load(fname2).expect(&format!("Cannot open model file \"{}\"", fname2));

    let merged_model = morphing::merge(model1, model2, matches.occurrences_of("edge_only") > 0);
    let merged_fname = format!(
        "{}_{}.obj",
        Path::new(fname1).file_stem().unwrap().to_string_lossy(),
        Path::new(fname2).file_stem().unwrap().to_string_lossy()
    );
    merged_model.save(&merged_fname).unwrap();

    if let Some(output) = matches.value_of("output") {
        merged_model.interpolation(ratio).save(output).unwrap();
    }
}
