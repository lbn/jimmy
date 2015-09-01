extern crate yaml_rust;
extern crate chrono;
extern crate getopts;

use std::env;
use std::fs::File;

use std::io::prelude::*;
//use yaml_rust::*;
use yaml_rust::*;

use getopts::Options;
use std::io;

// Set
struct Set {
    reps:   u8,
    weight: u16
}

impl Set {
    fn new(set: &Yaml) -> Set {
        let reps = match set["reps"].as_i64() {
            Some(s) => s as u8,
            None    => panic!("No number of reps for an exercise")
        };

        let weight = match set["weight"].as_i64() {
            Some(s) => s as u16,
            None    => panic!("No weight for an exercise")
        };

        Set {reps: reps, weight: weight}
    }

    fn serialise(&self) -> Yaml {
       let mut hash = yaml::Hash::new(); 
       hash.insert(Yaml::String("reps".to_string()), Yaml::Integer(self.reps as i64));
       hash.insert(Yaml::String("weight".to_string()), Yaml::Integer(self.weight as i64));
       return Yaml::Hash(hash);
    }

    fn input() -> Option<Set> {
        println!("{}= New set =", get_pre(5));
        let mut stdin = io::stdin();
        println!("Weight: ");
        let mut weight = String::new();
        stdin.read_line(&mut weight).ok().expect("Cannot get field");
        if weight.trim() == "x" {
            return None
        }
        let weight: u16 = weight.trim().parse().ok().expect("Be a number");

        println!("Reps: ");
        let mut reps = String::new();
        stdin.read_line(&mut reps).ok().expect("Cannot get field");
        if reps.trim() == "x" {
            return None
        }
        let reps: u8 = reps.trim().parse().ok().expect("Be a number");

        println!("{}= End of set =", get_pre(5));
        Some(Set {reps: reps, weight: weight})
    }

    fn print(self) {
        println!("{}{} reps - {}kg", get_pre(3), self.reps, self.weight);
    }
}

// Exercise
struct Exercise {
    name:   String,
    sets:   Vec<Set>
}

impl Exercise {
    fn new(exercise: &Yaml) -> Exercise {
        let name: &str = match exercise["name"].as_str() {
            Some(s) => s,
            None    => panic!("No name for an exercise")
        };
        let sets = match exercise["sets"].as_vec() {
            Some(s) => s.iter().map(|set| Set::new(&set)).collect::<Vec<_>>(),
            None    => panic!("No sets for an exercise")
        };
        Exercise {name: name.to_string(), sets: sets}
    }
    fn input() -> Option<Exercise> {
        println!("{}= New exercise =", get_pre(5));
        let mut stdin = io::stdin();
        println!("Name: ");
        let mut name = String::new();
        stdin.read_line(&mut name).ok().expect("Cannot read value");
        if name.trim() == "x" {
            return None
        }


        let mut sets = Vec::new();
        loop {
            match Set::input() {
                Some(s) => sets.push(s),
                None => break
            }
        }

        println!("{}= End of exercise =", get_pre(5));
        Some(Exercise {name: name, sets: sets})
    }
    fn serialise(&self) -> Yaml {
       let mut hash = yaml::Hash::new(); 
       let mut nsets: Vec<Yaml> = self.sets.iter().map(|set| set.serialise()).collect();

       hash.insert(Yaml::String("name".to_string()), Yaml::String(self.name.to_string()));
       hash.insert(Yaml::String("sets".to_string()), Yaml::Array(nsets));
       Yaml::Hash(hash)
    }

    fn print(self) {
        println!("{}Exercise: {} ({} sets)", get_pre(2), self.name, self.sets.len());

        for set in self.sets {
            set.print();
        }
    }
}

// Day
struct Day {
    date:       chrono::DateTime<chrono::FixedOffset>,
    exercises:  Vec<Exercise>    
}

impl Day {
    fn new(day: &Yaml) -> Day {
        let date = match day["date"].as_str() {
            Some(s) => s,
            None    => panic!("No date for a gym day")
        };

        let date = match chrono::DateTime::parse_from_rfc3339(date) {
            Ok(s)   => s,
            Err(_)=> panic!("Cannot parse date for a gym day ({})", date)
        };
        
        let exercises = match day["exercises"].as_vec() {
            Some(s) => s.iter().map(|e| Exercise::new(&e)).collect::<Vec<_>>(),
            None    => panic!("No exercises for a gym day ({})", date)
        };
        Day {date: date, exercises: exercises}
    }
    fn input() -> Option<Day> {
        let mut stdin = io::stdin();
        println!("{}= New gym day =", get_pre(5));
        println!("Date: ");
        let mut date = String::new();
        stdin.read_line(&mut date).ok().expect("Cannot read value");
        if date.trim() == "x" {
            return None
        }

        let date = match chrono::DateTime::parse_from_rfc3339(&date.trim()) {
            Ok(s)   => s,
            Err(_)=> panic!("Cannot parse date for a gym day ({})", date)
        };

        let mut exercises = Vec::new();
        loop {
            match Exercise::input() {
                Some(s) => exercises.push(s),
                None => break
            }
        }
        println!("{}= End of gym day =", get_pre(5));
        Some(Day {date: date, exercises: exercises})
    }
    fn serialise(&self) -> Yaml {
       let mut hash = yaml::Hash::new(); 
       let mut nexs: Vec<Yaml> = self.exercises.iter().map(|ex| ex.serialise()).collect();

       hash.insert(Yaml::String("date".to_string()), Yaml::String(self.date.to_rfc3339()));
       hash.insert(Yaml::String("exercises".to_string()), Yaml::Array(nexs));
       Yaml::Hash(hash)
    }
    fn print(self) {
        println!("{}Date: {}", get_pre(1), self.date);
        println!("{}Number of exercises: {}", get_pre(1), self.exercises.len());
        for exercise in self.exercises {
            exercise.print();
        }
    }
}


// Gym
struct Gym {
    days: Vec<Day>,
    file: File
}


impl Gym {
    fn new(filename: &str) -> Gym {
        let mut gym_yamls = String::new();

        let mut gym_file = match File::open(filename) {
            Ok(f) => f,
            Err(why) => panic!("Cannot open file: {}", why)
        };

        match gym_file.read_to_string(&mut gym_yamls) {
            Ok(s) => s,
            Err(why) => panic!("Cannot read file: {}", why)
        };

        let docs = match YamlLoader::load_from_str(&gym_yamls) {
            Ok(s) => s,
            Err(why) => panic!("Cannot parse file: {:?}", why)
        };

        let doc = &docs[0];

        let gym: &Yaml = &doc["gym"];
        let gym_days = match gym.as_vec() {
            Some(days) => days.iter().map(|day| Day::new(&day)).collect::<Vec<_>>(),
            None       => panic!("Gym days is not an array")
        };


        Gym { days: gym_days, file: gym_file }
    }
    fn save(&self, filename: &str) {
        let doc = self.serialise();
        let mut out_str = String::new();
        {
            let mut emitter = YamlEmitter::new(&mut out_str);
            emitter.dump(&doc).unwrap(); // dump the YAML object to a String
        }
        println!("{}",out_str);
    }
    fn serialise(&self) -> Yaml {
       let mut hash = yaml::Hash::new(); 
       let mut days: Vec<Yaml> = self.days.iter().map(|day| day.serialise()).collect();

       hash.insert(Yaml::String("gym".to_string()), Yaml::Array(days));
       Yaml::Hash(hash)
    }
    fn print(self) {
        println!("Gym days found: {}", self.days.len());
        for day in self.days {
            day.print();
        }
    }
}

fn get_pre(level: u8) -> String {
    std::iter::repeat(" ").take((level as usize)*2).collect::<String>()
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}



fn main() {
    let args: Vec<_> = env::args().collect();
    let program = args[0].clone();
    let mut opts = Options::new();
    opts.optopt("i", "in", "gym log file", "NAME");
    opts.optflag("h", "help", "print this help menu");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => { panic!(f.to_string()) }
    };

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    let gym_filename = match matches.opt_str("i") {
        Some(x) => x,
        None => {
            print_usage(&program, opts);
            return;
        }
    };

    let input = if !matches.free.is_empty() {
        matches.free[0].clone()
    } else {
        print_usage(&program, opts);
        return;
    };

    if input == "print" {
        let gym = Gym::new(&gym_filename);
        gym.print();
    } else if input == "edit" {
        let day = Day::input();
    } else if input == "save" {
        let gym = Gym::new(&gym_filename);
        gym.save("test");
    } else {
        print_usage(&program, opts);
        return;
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn get_pre() {
        assert_eq!("", super::get_pre(0));
        assert_eq!("  ", super::get_pre(1));
    }
}

