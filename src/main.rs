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

fn get_pre(level: u8) -> String {
    std::iter::repeat(" ").take((level as usize)*2).collect::<String>()
}

fn print_set(set: &yaml::Yaml) {
    let reps = match set["reps"].as_i64() {
        Some(s) => s as u8,
        None    => panic!("No number of reps for an exercise")
    };

    let weight = match set["weight"].as_i64() {
        Some(s) => s as u16,
        None    => panic!("No weight for an exercise")
    };
    println!("{}{} reps - {}kg", get_pre(3), reps, weight);
}

fn print_exercise(exercise: &yaml::Yaml) {
    let name = match exercise["name"].as_str() {
        Some(s) => s,
        None    => panic!("No name for an exercise")
    };
    let sets = match exercise["sets"].as_vec() {
        Some(s) => s,
        None    => panic!("No sets for an exercise")
    };

    println!("{}Exercise: {} ({} sets)", get_pre(2), name, sets.len());

    for set in sets {
        print_set(set);
    }
}

fn print_day(gym_day: &yaml::Yaml) {
    let date = match gym_day["date"].as_str() {
        Some(s) => s,
        None    => panic!("No date for a gym day")
    };
    println!("{}Date: {}", get_pre(1), date);
    
    let exercises = match gym_day["exercises"].as_vec() {
        Some(s) => s,
        None    => panic!("No exercises for a gym day ({})", date)
    };

    println!("{}Number of exercises: {}", get_pre(1), exercises.len());
    for exercise in exercises {
        print_exercise(exercise);
    }
}

fn print_gym(gym_days: &Vec<yaml::Yaml>) {
    println!("Gym days found: {}", gym_days.len());
    for gym_day in gym_days {
        print_day(gym_day);
    }
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
