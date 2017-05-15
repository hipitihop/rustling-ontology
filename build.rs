extern crate rmp_serde;
extern crate rustling;
extern crate rustling_ontology_rules;
extern crate rustling_ontology_values;
extern crate rustling_ontology_training as training;
extern crate serde;
#[macro_use]
extern crate serde_derive;

#[path="src/parser.rs"]
mod parser;


macro_rules! lang {
    ($lang:ident) => {
        mod $lang {
            use rustling_ontology_rules as rules;
            use std::{path, env, fs};
            pub fn train() {
                println!("cargo:rerun-if-changed=rules/src/{}.rs", stringify!($lang));
                let out_dir = path::PathBuf::from(env::var("OUT_DIR").unwrap());
                let mut file = fs::File::create(out_dir.join(concat!(stringify!($lang), ".rmp"))).unwrap(); 
                let rules = rules::$lang().unwrap();
                let exs = ::training::$lang();
                let model = ::rustling::train::train(&rules, exs, ::parser::FeatureExtractor()).unwrap();
                ::rmp_serde::encode::write(&mut file, &model).unwrap();
            }
        }
    }
}

lang!(en);
lang!(es);
lang!(fr);

fn main() {
    en::train();
    es::train();
    fr::train();
}
