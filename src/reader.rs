use std::path::Path;
use std::fs::File;


pub fn hex_file(filename: &str) {

    let ext_address = 0;
    let mut records: Vec<u16> = vec![];

    let path = Path::new(filename);

    let mut file = match File::open(&path) {
        Err(err) => panic!("Couldn't open {}: {}", path.display(), err.description()),
        Ok(file) => file
    };

    for line in file {
        println!("{:?}", line)
    }
}