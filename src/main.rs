extern crate yaml_rust;

use std::env;
use std::fs::File;

use std::io::prelude::*;
use yaml_rust::yaml;


fn get_docs() -> Vec<yaml_rust::yaml::Yaml> {
    let args: Vec<_> = env::args().collect();
    let mut gym_yamls = String::new();

    let mut gym_file = match File::open(&args[1]) {
        Ok(f) => f,
        Err(why) => panic!("Cannot open file: {}", why)
    };

    match gym_file.read_to_string(&mut gym_yamls) {
        Ok(s) => s,
        Err(why) => panic!("Cannot read file: {}", why)
    };

    let docs = match yaml::YamlLoader::load_from_str(&gym_yamls) {
        Ok(s) => s,
        Err(why) => panic!("Cannot parse file: {:?}", why)
    };
    docs
}

fn print_gym(gym_days: &Vec<yaml::Yaml>) {
    println!("Gym days found: {}", gym_days.len());
}

fn main() {
    let docs = get_docs();
    let doc = &docs[0];

    let gym: &yaml::Yaml = &doc["gym"];
    let gym_days = match gym.as_vec() {
        Some(days) => days,
        None       => panic!("Gym days is not an array")
    };
    print_gym(gym_days);
}
