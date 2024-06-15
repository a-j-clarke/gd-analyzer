use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub enum DBRKind {
    Item,
    Skill,
}

pub struct DBR {
    kind: DBRKind,
    path: String,
    lines: Vec<String>,
}

impl DBR {
    pub fn new(kind: DBRKind, path: String) -> DBR {
        DBR { kind, path }
    }

    pub fn read_from_file(path: String) {
        let path = Path::new(&path);
        let display = path.display();
        let mut file = match File::open(&path) {
            Err(e) => panic!("Couldn't open {}: {}", display, e),
            Ok(file) => file,
        };

        let mut s = String::new();
        match file.read_to_string(&mut s) {
            Err(e) => panic!("Couldn't read {}: {}", display, e),
            Ok(_) => println!("{}\n{}", display, s),
        }
    }
}
