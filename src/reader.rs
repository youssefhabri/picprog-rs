use std::path::Path;
use std::fs::File;
use std::io::Read;


pub fn hex_file(filename: &str) -> String {

    let ext_address = 0;
    let mut records: Vec<u16> = vec![];

    let path = Path::new(filename);

    println!("{:?}", path.display());

    let mut contents = String::new();
    let mut file = match File::open(&path) {
        Err(err) => panic!("Couldn't open {}: {}", path.display(), err),
        Ok(file) => file
    };
    file.read_to_string(&mut contents);
    return contents
}